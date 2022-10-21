use super::MULTI_VALUE_SUPPORT;

pub const LUAU_DISCONNECT_CONNECTION: &str = "\
function abi.ffi.disconnect_connection(connection)
	local func = connections[connection];
	if not func then return end

	getPointer(connection):Disconnect();
	dropFunctionRef(func[1], func[2]);
	connections[connection] = nil;
end";

pub const LUAU_CREATE_CONNECTION: &str = "\
local function createConnection(stack, vtable, connection)
	local id = createPointer(connection);
	connections[id] = { stack, vtable };
	return id;
end";

pub const LUAU_LUA_VALUE_CONVERSION: &str = "\
function abi.ffi.lua_value_string(addr, len)
	return createPointer(loadString(memory, addr, len));
end";

pub const LUAU_LUA_VALUE_CONVERSION_VARIABLE: &str = if MULTI_VALUE_SUPPORT {
    "\
function abi.ffi.string_lua_value(value)
	local text = getPointer(value);
	if type(text) ~= \"string\" then
		return 0, 0, 0;
	end

	local rustString = allocString(#text);
	for i = 1, #text do
		storeU8(memory, rustString + (i - 1), string.byte(text, i, i));
	end
	return 1, rustString, #text;
end"
} else {
    "\
function abi.ffi.string_lua_value(output, value)
	local text = getPointer(value);
	if type(text) ~= \"string\" then
		storeU32(memory, output, 0);
		storeU32(memory, output + 4, 0);
		storeU32(memory, output + 8, 0);
		return;
	end

	local rustString = allocString(#text);
	for i = 1, #text do
		storeU8(memory, rustString + (i - 1), string.byte(text, i, i));
	end
	storeU32(memory, output, 1);
	storeU32(memory, output + 4, rustString);
	storeU32(memory, output + 8, #text);
end"
};

pub const LUA_VALUE_NUMBER_TYPES: [&str; 12] = [
    "f32", "f64", "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "usize", "isize",
];
