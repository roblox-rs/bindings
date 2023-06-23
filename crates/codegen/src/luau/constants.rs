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

pub const LUA_VALUE_NUMBER_TYPES: [&str; 12] = [
    "f32", "f64", "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "usize", "isize",
];
