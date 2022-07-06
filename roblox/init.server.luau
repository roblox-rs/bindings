local rt = require(script.runtime);
local wasm, memory, util;
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

local outputStream = "";
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

	-- datatype constructors
	vector3_new = wrapConstructorNumber(Vector3.new),

	color3_new = wrapConstructorNumber(Color3.new),
	color3_from_rgb = wrapConstructorNumber(Color3.fromRGB),
	color3_from_hsv = wrapConstructorString(Color3.fromHSV),

	instance_new = function(name_addr, name_len)
		return createPointer(Instance.new(readString(name_addr, name_len)));
	end,

	-- RbxScriptConnection
	is_connected = function(id)
		return getPointer(id).Connected and 1 or 0;
	end,

	disconnect_connection = function(id)
		getPointer(id):Disconnect();
	end,

	-- Color3

	get_color3_r = function(id)
		return getPointer(id).R;
	end,

	get_color3_g = function(id)
		return getPointer(id).G;
	end,

	get_color3_b = function(id)
		return getPointer(id).B;
	end,

	color3_to_hex = function(ret, id)
		writeStringReturn(ret, getPointer(id):ToHex());
	end,

	color3_lerp = function(start, target, alpha)
		return createPointer(getPointer(start):Lerp(getPointer(target), alpha));
	end,

	-- pointers
	drop_pointer = function(ptr)
		dropPointer(ptr);
	end,

	clone_pointer = function(ptr)
		return createPointer(getPointer(ptr));
	end,

	-- Vector3

	get_vector3_x = function(id)
		return getPointer(id).X;
	end,

	get_vector3_y = function(id)
		return getPointer(id).Y;
	end,

	get_vector3_z = function(id)
		return getPointer(id).Z;
	end,

	add_datatype = function(lhs, rhs)
		return createPointer(getPointer(lhs) + getPointer(rhs));
	end,

	sub_datatype = function(lhs, rhs)
		return createPointer(getPointer(lhs) - getPointer(rhs));
	end,

	mul_datatype = function(lhs, rhs)
		return createPointer(getPointer(lhs) * getPointer(rhs));
	end,

	div_datatype = function(lhs, rhs)
		return createPointer(getPointer(lhs) / getPointer(rhs));
	end,

	mul_datatype_scalar = function(lhs, rhs)
		return createPointer(getPointer(lhs) * getPointer(rhs));
	end,

	div_datatype_scalar = function(lhs, rhs)
		return createPointer(getPointer(lhs) / getPointer(rhs));
	end,

	get_datatype_scalar = function(vec, name_addr, name_len)
		return getPointer(vec)[readString(name_addr, name_len)];
	end,

	-- Instances

	get_game = function()
		return createPointer(game);
	end,

	-- testing/logging
	stream = function(char)
		outputStream ..= string.char(char);
	end,
	emit = function()
		local stream = outputStream;
		task.defer(print, stream);
		outputStream = "";
	end,
	emit_error = function()
		local s = outputStream;
		outputStream = "";
		error(s);
	end,
}

local withWasm = {}
local function addModule(moduleInstance)
	local module = require(moduleInstance);
	table.insert(withWasm, module);

	for i, v in pairs(module.abi) do
		if env[i] then
			warn(i, "is being overriden");
		end
		env[i] = v;
	end
end

addModule(script.events);
addModule(script.properties);
addModule(script.functions);

wasm = require(script.wasm)({
	env = {
		func_list = env,
	},
});

memory = wasm.memory_list.memory;
util = require(script.util)(rt, wasm, memory);

for _, module in withWasm do
	module.withWasm(memory, rt, util);
end

createPointer, getPointer, dropPointer, invokeOnce, readString, writeStringReturn =
	util.createPointer, util.getPointer, util.dropPointer, util.invokeOnce, util.readString, util.writeStringReturn;

task.wait(2);
print("start wasm");
task.wait(0.8);

local start = os.clock();
wasm.func_list.greet(15);
print("Wasm took", (os.clock() - start) * 1000, "ms to execute");
