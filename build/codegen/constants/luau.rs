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
