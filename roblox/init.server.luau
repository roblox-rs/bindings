local wasm, memory, util, rt;
local createPointer, getPointer, dropPointer, invokeOnce, readString, writeStringReturn;

local function wrapConstructorNumber(constructor)
	return function(...)
		return createPointer(constructor(...));
	end
end

local function wrapConstructorString(constructor)
	return function(addr, len)
		return createPointer(constructor(addr, len));
	end
end

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
		task.wait(delay);
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

rt = wasm.rt;
memory = wasm.memory_list.memory;
util = require(script.util)(rt, wasm, memory);

abi.load(wasm, rt, util);

createPointer, getPointer, dropPointer, invokeOnce, readString =
	util.createPointer, util.getPointer, util.dropPointer, util.invokeOnce, util.readString;

task.wait(2);
print("start wasm");
task.wait(0.8);

local start = os.clock();
wasm.func_list.hook();
wasm.func_list.main();
print("Wasm took", (os.clock() - start) * 1000, "ms to execute");
