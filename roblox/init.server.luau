local rt = require(script.runtime);
local wasm, memory, util;
local createPointer, getPointer, dropPointer, updatePointer, invokeOnce, readString, writeStringReturn;

-- TODO: dropped futures are unsafe
local function createFuture(store, func, ...)
	local future = createPointer(0);

	task.spawn(function(...)
		updatePointer(future, store(func(...)));
		wasm.func_list.rors_wake_future(future);
	end, ...);

	return future;
end

local function createAsyncRuntime()
	-- context, futures and tasks could be represented as a single pointer (Box) for simplicity
	local queue = {};

	local function createContext(awakener)
		return wasm.function_list.create_context(createPointer(awakener));
	end

	local function awaken(context)
		getPointer(context)();
	end

	local function pollFuture(future, context)
		return wasm.function_list.poll_future(future, context);
	end

	local function runTask(task)
		table.insert(queue, task);
	end

	local function waitOn(task)
		local thread = coroutine.running();
		local awake = false;
		local context = createContext(function()
			awake = true;
			coroutine.resume(thread);
		end);

		local data;
		while true do
			data = table.pack(pollFuture(task, context));

			-- Check for Poll::Ready
			if (data[1] == 1) then
				break;
			end

			-- If the task immediately awakens, don't yield
			if (awake) then
				awake = false;
				continue;
			end

			coroutine.yield();
		end

		dropPointer(context);
		return table.unpack(data, 2, data.n);
	end

	return { runTask = runTask, waitOn = waitOn, awaken = awaken };
end

local runtime = createAsyncRuntime();

local env = {
	-- lbiraries
	lib_task_delay = function(delay, stack, fn_ptr)
		task.delay(delay, invokeOnce, stack, fn_ptr);
	end,

	lib_task_defer = function(stack, fn_ptr)
		task.defer(invokeOnce, stack, fn_ptr);
	end,

	lib_task_spawn = function(stack, fn_ptr)
		task.spawn(invokeOnce, stack, fn_ptr);
	end,

	lib_task_wait = function(delay)
		return createFuture(
			function(value)
				local addr = wasm.func_list.__heap_alloc_vec(2);
				rt.store.f64(memory, addr, value);
				return addr;
			end,
			task.wait,
			delay
		);
	end,

	get_poll_state = function(future)
		return getPointer(future);
	end,

	-- pointers
	drop_pointer = function(ptr)
		dropPointer(ptr);
	end,

	clone_pointer = function(ptr)
		return createPointer(getPointer(ptr));
	end,

	-- Instances

	get_game = function()
		return createPointer(game);
	end,

	-- testing/logging
	print = function(addr, len)
		task.defer(print, readString(addr, len));
	end,
	error = function(addr, len)
		error(readString(addr, len));
	end
}

local abi = require(script.abi);
for i, v in pairs(abi.ffi) do
	env[i] = v;
end

wasm = require(script.wasm)({
	env = {
		func_list = env,
	},
});

memory = wasm.memory_list.memory;
util = require(script.util)(rt, wasm, memory);

abi.load(wasm, rt, util);

createPointer, getPointer, dropPointer, updatePointer, invokeOnce, readString =
	util.createPointer, util.getPointer, util.dropPointer, util.updatePointer, util.invokeOnce, util.readString;

task.wait(2);
print("start wasm");
task.wait(0.8);

local start = os.clock();
wasm.func_list.hook();
wasm.func_list.main();
print("Wasm took", (os.clock() - start) * 1000, "ms to execute");
