return function(rt, wasm, memory)
	local function_table = wasm.table_list.__indirect_function_table;
	local stackPtr = wasm.global_list.__stack_pointer;

	local pointers = { last = 0 };
	local function createPointer(value)
		local id = pointers.last;
		if (id == 0) then
			id = #pointers + 1;
		else
			pointers.last = pointers[id];
		end
		pointers[id] = value;
		return id;
	end

	local function getPointer(id)
		return pointers[id];
	end

	local function dropPointer(id)
		pointers[id] = pointers.last;
		pointers.last = id;
	end

	local function pushStack(bytes)
		local stack_loc = stackPtr.value - bytes;
		stackPtr.value = stack_loc;

		return stack_loc;
	end

	local function popStack(stack_loc, bytes)
		stackPtr.value = stack_loc + bytes;
	end

	local function invokeFunction(data, vtable, ...)
		function_table.data[rt.load.i32(memory, vtable + 12)](data, ...);
	end

	local function dropFunctionRef(data, vtable)
		wasm.func_list.__drop_box_dyn_fn(data, vtable);
	end

	-- Usable on FnOnce()
	-- FnOnce() will clean itself up, so dropFunctionRef is unnecessary
	local function invokeOnce(data, vtable, ...)
		return invokeFunction(data, vtable, ...);
	end

	local function allocString(text)
		local vec = wasm.func_list.__heap_alloc_string(#text);
		for i = 1, #text do
			rt.store.i32_n8(memory, vec + i - 1, string.byte(text, i, i));
		end
		return vec;
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
