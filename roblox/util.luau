return function(rt, wasm, memory)
	local function_table = wasm.table_list.__indirect_function_table;
	local stackPtr = wasm.global_list.__stack_pointer;

	local id = 0;
	local pointers = {};
	local function createPointer(value)
		id += 1;
		pointers[id] = value;
		return id;
	end

	local function getPointer(id)
		return pointers[id];
	end

	local function dropPointer(id)
		pointers[id] = nil;
	end

	local function pushStack(bytes)
		local stack_loc = stackPtr.value - bytes;
		stackPtr.value = stack_loc;

		return stack_loc;
	end

	local function popStack(stack_loc, bytes)
		stackPtr.value = stack_loc + bytes;
	end

	local function invokeFunction(stack, vtable, ...)
		function_table.data[rt.load.i32(memory, vtable + 12)](stack, ...);
	end

	local function dropFunctionRef(stack, vtable)
		wasm.func_list.__drop_box_dyn_fn(stack, vtable);
	end

	local function invokeOnce(stack, vtable, ...)
		invokeFunction(stack, vtable, ...);
		dropFunctionRef(stack, vtable);
	end

	local function allocString(text)
		local vec = wasm.func_list.__heap_alloc_string(#text);
		for i = 1, #text do
			rt.store.i32_n8(memory, vec + i - 1, string.byte(text, i, i));
		end
		return vec;
	end

	local function writeStringReturn(return_addr, text)
		-- *mut u8 contents, usize capacity, usize length
		local contents = allocString(text);
		rt.store.i32(memory, return_addr, contents);
		rt.store.i32(memory, return_addr + 4, #text);
		rt.store.i32(memory, return_addr + 8, #text);
	end

	local function readString(addr, len)
		local s = "";
		for i = 1, len do
			s ..= string.char(rt.load.i32_u8(memory, addr + i - 1));
		end
		return s;
	end

	return {
		writeStringReturn = writeStringReturn,
		dropFunctionRef = dropFunctionRef,
		invokeFunction = invokeFunction,
		createPointer = createPointer,
		dropPointer = dropPointer,
		readString = readString,
		getPointer = getPointer,
		invokeOnce = invokeOnce,
		pushStack = pushStack,
		popStack = popStack,
	}
end
