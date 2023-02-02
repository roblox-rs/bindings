local abi = { ffi = {} };
local getPointer, createPointer, memory, storeU8, storeU32, storeFloat, loadU8, loadU32, loadFloat, loadString, allocString, allocVec, functions, invokeFunction, dropFunctionRef;
local connections = {};
local function createConnection(stack, vtable, connection)
	local id = createPointer(connection);
	connections[id] = { stack, vtable };
	return id;
end
function abi.load(wasm, rt, util)
	getPointer, createPointer, memory, storeU8, storeU32, storeFloat, loadU8, loadU32, loadFloat, loadString, allocString, allocVec, functions, invokeFunction, dropFunctionRef = util.getPointer, util.createPointer, wasm.memory_list.memory, rt.store.i32_n8, rt.store.i32, rt.store.f64, rt.load.i32_u8, rt.load.i32, rt.load.f64, rt.load.string, wasm.func_list.__heap_alloc_string, wasm.func_list.__heap_alloc_vec, wasm.table_list.__indirect_function_table, util.invokeFunction, util.dropFunctionRef;
end
function abi.ffi.disconnect_connection(connection)
	local func = connections[connection];
	if not func then return end

	getPointer(connection):Disconnect();
	dropFunctionRef(func[1], func[2]);
	connections[connection] = nil;
end
function abi.ffi.property_getter_instance_archivable(self)
	local result_0 = getPointer(self).Archivable;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_instance_archivable_set_archivable(self, value)
	getPointer(self).Archivable = value == 1;
end
function abi.ffi.property_getter_instance_class_name(self)
	local result_0 = getPointer(self).ClassName;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_instance_name(self)
	local result_0 = getPointer(self).Name;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_instance_name_set_name(self, value_addr, value_len)
	getPointer(self).Name = loadString(memory, value_addr, value_len);
end
function abi.ffi.method_instance_clear_all_children(self)
	getPointer(self):ClearAllChildren()
end
function abi.ffi.method_instance_clone_fn_clone(self)
	local result_0 = getPointer(self):Clone();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_instance_destroy(self)
	getPointer(self):Destroy()
end
function abi.ffi.method_instance_find_first_ancestor(self, name_addr, name_len)
	local result_0 = getPointer(self):FindFirstAncestor(loadString(memory, name_addr, name_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_instance_find_first_ancestor_of_class(self, class_name_addr, class_name_len)
	local result_0 = getPointer(self):FindFirstAncestorOfClass(loadString(memory, class_name_addr, class_name_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_instance_find_first_ancestor_which_is_a(self, class_name_addr, class_name_len)
	local result_0 = getPointer(self):FindFirstAncestorWhichIsA(loadString(memory, class_name_addr, class_name_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_instance_find_first_child(self, name_addr, name_len, recursive)
	local result_0 = getPointer(self):FindFirstChild(loadString(memory, name_addr, name_len), recursive == 1);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_instance_find_first_child_of_class(self, class_name_addr, class_name_len)
	local result_0 = getPointer(self):FindFirstChildOfClass(loadString(memory, class_name_addr, class_name_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_instance_find_first_child_which_is_a(self, class_name_addr, class_name_len, recursive)
	local result_0 = getPointer(self):FindFirstChildWhichIsA(loadString(memory, class_name_addr, class_name_len), recursive == 1);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_instance_find_first_descendant(self, name_addr, name_len)
	local result_0 = getPointer(self):FindFirstDescendant(loadString(memory, name_addr, name_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_instance_get_actor(self)
	local result_0 = getPointer(self):GetActor();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_instance_get_full_name(self)
	local result_0 = getPointer(self):GetFullName();
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_instance_is_a(self, class_name_addr, class_name_len)
	local result_0 = getPointer(self):IsA(loadString(memory, class_name_addr, class_name_len));
	return result_0 and 1 or 0;
end
function abi.ffi.method_instance_is_ancestor_of(self, descendant)
	local result_0 = getPointer(self):IsAncestorOf(getPointer(descendant));
	return result_0 and 1 or 0;
end
function abi.ffi.method_instance_is_descendant_of(self, ancestor)
	local result_0 = getPointer(self):IsDescendantOf(getPointer(ancestor));
	return result_0 and 1 or 0;
end
function abi.ffi.method_instance_wait_for_child(self, child_name_addr, child_name_len, time_out)
	local result_0 = getPointer(self):WaitForChild(loadString(memory, child_name_addr, child_name_len), time_out);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.event_instance_ancestry_changed_on_ancestry_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).AncestryChanged:Connect(function(child, parent)
		local child_optional, child_optional_0 = 0, 0;
		if (child ~= nil) then
			child_optional, child_optional_0 = 1, createPointer(child);
		end
		local parent_optional, parent_optional_0 = 0, 0;
		if (parent ~= nil) then
			parent_optional, parent_optional_0 = 1, createPointer(parent);
		end
		invokeFunction(callback_data, callback_vtable, child_optional, child_optional_0, parent_optional, parent_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_instance_attribute_changed_on_attribute_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).AttributeChanged:Connect(function(attribute)
		local attribute_len = #attribute;
		local attribute_content = allocString(attribute_len);
		for i = 1, attribute_len do
			storeU8(memory, attribute_content + (i - 1), string.byte(attribute:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, attribute_content, attribute_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_instance_child_added_on_child_added(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).ChildAdded:Connect(function(child)
		local child_optional, child_optional_0 = 0, 0;
		if (child ~= nil) then
			child_optional, child_optional_0 = 1, createPointer(child);
		end
		invokeFunction(callback_data, callback_vtable, child_optional, child_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_instance_child_removed_on_child_removed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).ChildRemoved:Connect(function(child)
		local child_optional, child_optional_0 = 0, 0;
		if (child ~= nil) then
			child_optional, child_optional_0 = 1, createPointer(child);
		end
		invokeFunction(callback_data, callback_vtable, child_optional, child_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_instance_descendant_added_on_descendant_added(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).DescendantAdded:Connect(function(descendant)
		local descendant_optional, descendant_optional_0 = 0, 0;
		if (descendant ~= nil) then
			descendant_optional, descendant_optional_0 = 1, createPointer(descendant);
		end
		invokeFunction(callback_data, callback_vtable, descendant_optional, descendant_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_instance_descendant_removing_on_descendant_removing(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).DescendantRemoving:Connect(function(descendant)
		local descendant_optional, descendant_optional_0 = 0, 0;
		if (descendant ~= nil) then
			descendant_optional, descendant_optional_0 = 1, createPointer(descendant);
		end
		invokeFunction(callback_data, callback_vtable, descendant_optional, descendant_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_instance_destroying_on_destroying(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Destroying:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_setter_instance_parent_set_parent(self, value_kind, value)
	local value_optional;
	if (value_kind == 1) then
		value_optional = getPointer(value);
	end
	getPointer(self).Parent = value_optional;
end
function abi.ffi.event_instance_changed_on_instance_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Changed:Connect(function(property)
		local property_len = #property;
		local property_content = allocString(property_len);
		for i = 1, property_len do
			storeU8(memory, property_content + (i - 1), string.byte(property:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, property_content, property_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_accoutrement_attachment_forward(self)
	local result_0 = getPointer(self).AttachmentForward;
	return createPointer(result_0);
end
function abi.ffi.property_setter_accoutrement_attachment_forward_set_attachment_forward(self, value)
	getPointer(self).AttachmentForward = getPointer(value);
end
function abi.ffi.property_getter_accoutrement_attachment_point(self)
	local result_0 = getPointer(self).AttachmentPoint;
	return createPointer(result_0);
end
function abi.ffi.property_setter_accoutrement_attachment_point_set_attachment_point(self, value)
	getPointer(self).AttachmentPoint = getPointer(value);
end
function abi.ffi.property_getter_accoutrement_attachment_pos(self)
	local result_0 = getPointer(self).AttachmentPos;
	return createPointer(result_0);
end
function abi.ffi.property_setter_accoutrement_attachment_pos_set_attachment_pos(self, value)
	getPointer(self).AttachmentPos = getPointer(value);
end
function abi.ffi.property_getter_accoutrement_attachment_right(self)
	local result_0 = getPointer(self).AttachmentRight;
	return createPointer(result_0);
end
function abi.ffi.property_setter_accoutrement_attachment_right_set_attachment_right(self, value)
	getPointer(self).AttachmentRight = getPointer(value);
end
function abi.ffi.property_getter_accoutrement_attachment_up(self)
	local result_0 = getPointer(self).AttachmentUp;
	return createPointer(result_0);
end
function abi.ffi.property_setter_accoutrement_attachment_up_set_attachment_up(self, value)
	getPointer(self).AttachmentUp = getPointer(value);
end
function abi.ffi.property_getter_accessory_accessory_type(self)
	local result_0 = getPointer(self).AccessoryType;
	return result_0;
end
function abi.ffi.property_setter_accessory_accessory_type_set_accessory_type(self, value)
	getPointer(self).AccessoryType = value;
end
function abi.ffi.property_getter_ad_portal_portal_status(self)
	local result_0 = getPointer(self).PortalStatus;
	return result_0;
end
function abi.ffi.property_getter_ad_portal_portal_type(self)
	local result_0 = getPointer(self).PortalType;
	return result_0;
end
function abi.ffi.property_setter_ad_portal_portal_type_set_portal_type(self, value)
	getPointer(self).PortalType = value;
end
function abi.ffi.property_getter_animation_animation_id(self)
	local result_0 = getPointer(self).AnimationId;
	return createPointer(result_0);
end
function abi.ffi.property_setter_animation_animation_id_set_animation_id(self, value)
	getPointer(self).AnimationId = getPointer(value);
end
function abi.ffi.property_getter_animation_clip_loop(self)
	local result_0 = getPointer(self).Loop;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_animation_clip_loop_set_loop(self, value)
	getPointer(self).Loop = value == 1;
end
function abi.ffi.property_getter_animation_clip_priority(self)
	local result_0 = getPointer(self).Priority;
	return result_0;
end
function abi.ffi.property_setter_animation_clip_priority_set_priority(self, value)
	getPointer(self).Priority = value;
end
function abi.ffi.method_keyframe_sequence_add_keyframe(self, keyframe)
	getPointer(self):AddKeyframe(getPointer(keyframe))
end
function abi.ffi.method_keyframe_sequence_remove_keyframe(self, keyframe)
	getPointer(self):RemoveKeyframe(getPointer(keyframe))
end
function abi.ffi.method_animation_clip_provider_register_active_animation_clip(self, animation_clip)
	local result_0 = getPointer(self):RegisterActiveAnimationClip(getPointer(animation_clip));
	return createPointer(result_0);
end
function abi.ffi.method_animation_clip_provider_register_animation_clip(self, animation_clip)
	local result_0 = getPointer(self):RegisterAnimationClip(getPointer(animation_clip));
	return createPointer(result_0);
end
function abi.ffi.method_animation_clip_provider_get_animation_clip_async(self, asset_id)
	local result_0 = getPointer(self):GetAnimationClipAsync(getPointer(asset_id));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_animation_clip_provider_get_animations(self, user_id)
	local result_0 = getPointer(self):GetAnimations(user_id);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_animation_stream_track_is_playing(self)
	local result_0 = getPointer(self).IsPlaying;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_animation_stream_track_priority(self)
	local result_0 = getPointer(self).Priority;
	return result_0;
end
function abi.ffi.property_setter_animation_stream_track_priority_set_priority(self, value)
	getPointer(self).Priority = value;
end
function abi.ffi.property_getter_animation_stream_track_weight_current(self)
	local result_0 = getPointer(self).WeightCurrent;
	return result_0;
end
function abi.ffi.property_getter_animation_stream_track_weight_target(self)
	local result_0 = getPointer(self).WeightTarget;
	return result_0;
end
function abi.ffi.property_getter_animation_track_animation(self)
	local result_0 = getPointer(self).Animation;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_animation_track_is_playing(self)
	local result_0 = getPointer(self).IsPlaying;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_animation_track_length(self)
	local result_0 = getPointer(self).Length;
	return result_0;
end
function abi.ffi.property_getter_animation_track_looped(self)
	local result_0 = getPointer(self).Looped;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_animation_track_looped_set_looped(self, value)
	getPointer(self).Looped = value == 1;
end
function abi.ffi.property_getter_animation_track_priority(self)
	local result_0 = getPointer(self).Priority;
	return result_0;
end
function abi.ffi.property_setter_animation_track_priority_set_priority(self, value)
	getPointer(self).Priority = value;
end
function abi.ffi.property_getter_animation_track_speed(self)
	local result_0 = getPointer(self).Speed;
	return result_0;
end
function abi.ffi.property_getter_animation_track_time_position(self)
	local result_0 = getPointer(self).TimePosition;
	return result_0;
end
function abi.ffi.property_setter_animation_track_time_position_set_time_position(self, value)
	getPointer(self).TimePosition = value;
end
function abi.ffi.property_getter_animation_track_weight_current(self)
	local result_0 = getPointer(self).WeightCurrent;
	return result_0;
end
function abi.ffi.property_getter_animation_track_weight_target(self)
	local result_0 = getPointer(self).WeightTarget;
	return result_0;
end
function abi.ffi.method_animation_track_adjust_speed(self, speed)
	getPointer(self):AdjustSpeed(speed)
end
function abi.ffi.method_animation_track_adjust_weight(self, weight, fade_time)
	getPointer(self):AdjustWeight(weight, fade_time)
end
function abi.ffi.method_animation_track_get_time_of_keyframe(self, keyframe_name_addr, keyframe_name_len)
	local result_0 = getPointer(self):GetTimeOfKeyframe(loadString(memory, keyframe_name_addr, keyframe_name_len));
	return result_0;
end
function abi.ffi.method_animation_track_play(self, fade_time, weight, speed)
	getPointer(self):Play(fade_time, weight, speed)
end
function abi.ffi.method_animation_track_stop(self, fade_time)
	getPointer(self):Stop(fade_time)
end
function abi.ffi.event_animation_track_did_loop_on_did_loop(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).DidLoop:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_animation_track_keyframe_reached_on_keyframe_reached(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).KeyframeReached:Connect(function(keyframe_name)
		local keyframe_name_len = #keyframe_name;
		local keyframe_name_content = allocString(keyframe_name_len);
		for i = 1, keyframe_name_len do
			storeU8(memory, keyframe_name_content + (i - 1), string.byte(keyframe_name:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, keyframe_name_content, keyframe_name_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_animation_track_stopped_on_stopped(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Stopped:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_animator_prefer_lod_enabled(self)
	local result_0 = getPointer(self).PreferLodEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_animator_prefer_lod_enabled_set_prefer_lod_enabled(self, value)
	getPointer(self).PreferLodEnabled = value == 1;
end
function abi.ffi.method_animator_load_animation(self, animation)
	local result_0 = getPointer(self):LoadAnimation(getPointer(animation));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.event_animator_animation_played_on_animation_played(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).AnimationPlayed:Connect(function(animation_track)
		local animation_track_optional, animation_track_optional_0 = 0, 0;
		if (animation_track ~= nil) then
			animation_track_optional, animation_track_optional_0 = 1, createPointer(animation_track);
		end
		invokeFunction(callback_data, callback_vtable, animation_track_optional, animation_track_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_asset_import_session_upload_progress_on_upload_progress(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).UploadProgress:Connect(function(progress_ratio)
		invokeFunction(callback_data, callback_vtable, progress_ratio)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_asset_patch_settings_content_id(self)
	local result_0 = getPointer(self).ContentId;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_asset_patch_settings_content_id_set_content_id(self, value_addr, value_len)
	getPointer(self).ContentId = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_asset_patch_settings_output_path(self)
	local result_0 = getPointer(self).OutputPath;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_asset_patch_settings_output_path_set_output_path(self, value_addr, value_len)
	getPointer(self).OutputPath = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_asset_patch_settings_patch_id(self)
	local result_0 = getPointer(self).PatchId;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_asset_patch_settings_patch_id_set_patch_id(self, value_addr, value_len)
	getPointer(self).PatchId = loadString(memory, value_addr, value_len);
end
function abi.ffi.method_asset_service_create_place_async(self, place_name_addr, place_name_len, template_place_id, description_addr, description_len)
	local result_0 = getPointer(self):CreatePlaceAsync(loadString(memory, place_name_addr, place_name_len), template_place_id, loadString(memory, description_addr, description_len));
	return result_0;
end
function abi.ffi.method_asset_service_create_place_in_player_inventory_async(self, player, place_name_addr, place_name_len, template_place_id, description_addr, description_len)
	local result_0 = getPointer(self):CreatePlaceInPlayerInventoryAsync(getPointer(player), loadString(memory, place_name_addr, place_name_len), template_place_id, loadString(memory, description_addr, description_len));
	return result_0;
end
function abi.ffi.method_asset_service_get_game_places_async(self)
	local result_0 = getPointer(self):GetGamePlacesAsync();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_asset_service_save_place_async(self)
	getPointer(self):SavePlaceAsync()
end
function abi.ffi.property_getter_atmosphere_color(self)
	local result_0 = getPointer(self).Color;
	return createPointer(result_0);
end
function abi.ffi.property_setter_atmosphere_color_set_color(self, value)
	getPointer(self).Color = getPointer(value);
end
function abi.ffi.property_getter_atmosphere_decay(self)
	local result_0 = getPointer(self).Decay;
	return createPointer(result_0);
end
function abi.ffi.property_setter_atmosphere_decay_set_decay(self, value)
	getPointer(self).Decay = getPointer(value);
end
function abi.ffi.property_getter_atmosphere_density(self)
	local result_0 = getPointer(self).Density;
	return result_0;
end
function abi.ffi.property_setter_atmosphere_density_set_density(self, value)
	getPointer(self).Density = value;
end
function abi.ffi.property_getter_atmosphere_glare(self)
	local result_0 = getPointer(self).Glare;
	return result_0;
end
function abi.ffi.property_setter_atmosphere_glare_set_glare(self, value)
	getPointer(self).Glare = value;
end
function abi.ffi.property_getter_atmosphere_haze(self)
	local result_0 = getPointer(self).Haze;
	return result_0;
end
function abi.ffi.property_setter_atmosphere_haze_set_haze(self, value)
	getPointer(self).Haze = value;
end
function abi.ffi.property_getter_atmosphere_offset(self)
	local result_0 = getPointer(self).Offset;
	return result_0;
end
function abi.ffi.property_setter_atmosphere_offset_set_offset(self, value)
	getPointer(self).Offset = value;
end
function abi.ffi.property_getter_attachment_axis(self)
	local result_0 = getPointer(self).Axis;
	return createPointer(result_0);
end
function abi.ffi.property_setter_attachment_axis_set_axis(self, value)
	getPointer(self).Axis = getPointer(value);
end
function abi.ffi.property_getter_attachment_c_frame(self)
	local result_0 = getPointer(self).CFrame;
	return createPointer(result_0);
end
function abi.ffi.property_setter_attachment_c_frame_set_c_frame(self, value)
	getPointer(self).CFrame = getPointer(value);
end
function abi.ffi.property_getter_attachment_orientation(self)
	local result_0 = getPointer(self).Orientation;
	return createPointer(result_0);
end
function abi.ffi.property_setter_attachment_orientation_set_orientation(self, value)
	getPointer(self).Orientation = getPointer(value);
end
function abi.ffi.property_getter_attachment_position(self)
	local result_0 = getPointer(self).Position;
	return createPointer(result_0);
end
function abi.ffi.property_setter_attachment_position_set_position(self, value)
	getPointer(self).Position = getPointer(value);
end
function abi.ffi.property_getter_attachment_secondary_axis(self)
	local result_0 = getPointer(self).SecondaryAxis;
	return createPointer(result_0);
end
function abi.ffi.property_setter_attachment_secondary_axis_set_secondary_axis(self, value)
	getPointer(self).SecondaryAxis = getPointer(value);
end
function abi.ffi.property_getter_attachment_visible(self)
	local result_0 = getPointer(self).Visible;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_attachment_visible_set_visible(self, value)
	getPointer(self).Visible = value == 1;
end
function abi.ffi.property_getter_attachment_world_axis(self)
	local result_0 = getPointer(self).WorldAxis;
	return createPointer(result_0);
end
function abi.ffi.property_setter_attachment_world_axis_set_world_axis(self, value)
	getPointer(self).WorldAxis = getPointer(value);
end
function abi.ffi.property_getter_attachment_world_c_frame(self)
	local result_0 = getPointer(self).WorldCFrame;
	return createPointer(result_0);
end
function abi.ffi.property_setter_attachment_world_c_frame_set_world_c_frame(self, value)
	getPointer(self).WorldCFrame = getPointer(value);
end
function abi.ffi.property_getter_attachment_world_orientation(self)
	local result_0 = getPointer(self).WorldOrientation;
	return createPointer(result_0);
end
function abi.ffi.property_setter_attachment_world_orientation_set_world_orientation(self, value)
	getPointer(self).WorldOrientation = getPointer(value);
end
function abi.ffi.property_getter_attachment_world_position(self)
	local result_0 = getPointer(self).WorldPosition;
	return createPointer(result_0);
end
function abi.ffi.property_setter_attachment_world_position_set_world_position(self, value)
	getPointer(self).WorldPosition = getPointer(value);
end
function abi.ffi.property_getter_attachment_world_secondary_axis(self)
	local result_0 = getPointer(self).WorldSecondaryAxis;
	return createPointer(result_0);
end
function abi.ffi.property_setter_attachment_world_secondary_axis_set_world_secondary_axis(self, value)
	getPointer(self).WorldSecondaryAxis = getPointer(value);
end
function abi.ffi.property_getter_bone_transform(self)
	local result_0 = getPointer(self).Transform;
	return createPointer(result_0);
end
function abi.ffi.property_setter_bone_transform_set_transform(self, value)
	getPointer(self).Transform = getPointer(value);
end
function abi.ffi.property_getter_bone_transformed_c_frame(self)
	local result_0 = getPointer(self).TransformedCFrame;
	return createPointer(result_0);
end
function abi.ffi.property_getter_bone_transformed_world_c_frame(self)
	local result_0 = getPointer(self).TransformedWorldCFrame;
	return createPointer(result_0);
end
function abi.ffi.method_avatar_editor_service_get_accessory_type(self, avatar_asset_type)
	local result_0 = getPointer(self):GetAccessoryType(avatar_asset_type);
	return result_0;
end
function abi.ffi.method_avatar_editor_service_prompt_allow_inventory_read_access(self)
	getPointer(self):PromptAllowInventoryReadAccess()
end
function abi.ffi.method_avatar_editor_service_prompt_create_outfit(self, outfit, rig_type)
	getPointer(self):PromptCreateOutfit(getPointer(outfit), rig_type)
end
function abi.ffi.method_avatar_editor_service_prompt_delete_outfit(self, outfit_id)
	getPointer(self):PromptDeleteOutfit(outfit_id)
end
function abi.ffi.method_avatar_editor_service_prompt_rename_outfit(self, outfit_id)
	getPointer(self):PromptRenameOutfit(outfit_id)
end
function abi.ffi.method_avatar_editor_service_prompt_save_avatar(self, humanoid_description, rig_type)
	getPointer(self):PromptSaveAvatar(getPointer(humanoid_description), rig_type)
end
function abi.ffi.method_avatar_editor_service_prompt_set_favorite(self, item_id, item_type, should_favorite)
	getPointer(self):PromptSetFavorite(item_id, item_type, should_favorite == 1)
end
function abi.ffi.method_avatar_editor_service_prompt_update_outfit(self, outfit_id, updated_outfit, rig_type)
	getPointer(self):PromptUpdateOutfit(outfit_id, getPointer(updated_outfit), rig_type)
end
function abi.ffi.method_avatar_editor_service_check_apply_default_clothing(self, humanoid_description)
	local result_0 = getPointer(self):CheckApplyDefaultClothing(getPointer(humanoid_description));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_avatar_editor_service_conform_to_avatar_rules(self, humanoid_description)
	local result_0 = getPointer(self):ConformToAvatarRules(getPointer(humanoid_description));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_avatar_editor_service_get_favorite(self, item_id, item_type)
	local result_0 = getPointer(self):GetFavorite(item_id, item_type);
	return result_0 and 1 or 0;
end
function abi.ffi.method_avatar_editor_service_get_outfits(self, outfit_source, outfit_type)
	local result_0 = getPointer(self):GetOutfits(outfit_source, outfit_type);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_avatar_editor_service_search_catalog(self, search_parameters)
	local result_0 = getPointer(self):SearchCatalog(getPointer(search_parameters));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.event_avatar_editor_service_prompt_allow_inventory_read_access_completed_on_prompt_allow_inventory_read_access_completed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptAllowInventoryReadAccessCompleted:Connect(function(result)
		invokeFunction(callback_data, callback_vtable, result)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_avatar_editor_service_prompt_delete_outfit_completed_on_prompt_delete_outfit_completed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptDeleteOutfitCompleted:Connect(function(result)
		invokeFunction(callback_data, callback_vtable, result)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_avatar_editor_service_prompt_rename_outfit_completed_on_prompt_rename_outfit_completed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptRenameOutfitCompleted:Connect(function(result)
		invokeFunction(callback_data, callback_vtable, result)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_avatar_editor_service_prompt_save_avatar_completed_on_prompt_save_avatar_completed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptSaveAvatarCompleted:Connect(function(result, humanoid_description)
		local humanoid_description_optional, humanoid_description_optional_0 = 0, 0;
		if (humanoid_description ~= nil) then
			humanoid_description_optional, humanoid_description_optional_0 = 1, createPointer(humanoid_description);
		end
		invokeFunction(callback_data, callback_vtable, result, humanoid_description_optional, humanoid_description_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_avatar_editor_service_prompt_set_favorite_completed_on_prompt_set_favorite_completed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptSetFavoriteCompleted:Connect(function(result)
		invokeFunction(callback_data, callback_vtable, result)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_avatar_editor_service_prompt_update_outfit_completed_on_prompt_update_outfit_completed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptUpdateOutfitCompleted:Connect(function(result)
		invokeFunction(callback_data, callback_vtable, result)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_backpack_item_texture_id(self)
	local result_0 = getPointer(self).TextureId;
	return createPointer(result_0);
end
function abi.ffi.property_setter_backpack_item_texture_id_set_texture_id(self, value)
	getPointer(self).TextureId = getPointer(value);
end
function abi.ffi.property_getter_tool_can_be_dropped(self)
	local result_0 = getPointer(self).CanBeDropped;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_tool_can_be_dropped_set_can_be_dropped(self, value)
	getPointer(self).CanBeDropped = value == 1;
end
function abi.ffi.property_getter_tool_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_tool_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_tool_grip(self)
	local result_0 = getPointer(self).Grip;
	return createPointer(result_0);
end
function abi.ffi.property_setter_tool_grip_set_grip(self, value)
	getPointer(self).Grip = getPointer(value);
end
function abi.ffi.property_getter_tool_grip_forward(self)
	local result_0 = getPointer(self).GripForward;
	return createPointer(result_0);
end
function abi.ffi.property_setter_tool_grip_forward_set_grip_forward(self, value)
	getPointer(self).GripForward = getPointer(value);
end
function abi.ffi.property_getter_tool_grip_pos(self)
	local result_0 = getPointer(self).GripPos;
	return createPointer(result_0);
end
function abi.ffi.property_setter_tool_grip_pos_set_grip_pos(self, value)
	getPointer(self).GripPos = getPointer(value);
end
function abi.ffi.property_getter_tool_grip_right(self)
	local result_0 = getPointer(self).GripRight;
	return createPointer(result_0);
end
function abi.ffi.property_setter_tool_grip_right_set_grip_right(self, value)
	getPointer(self).GripRight = getPointer(value);
end
function abi.ffi.property_getter_tool_grip_up(self)
	local result_0 = getPointer(self).GripUp;
	return createPointer(result_0);
end
function abi.ffi.property_setter_tool_grip_up_set_grip_up(self, value)
	getPointer(self).GripUp = getPointer(value);
end
function abi.ffi.property_getter_tool_manual_activation_only(self)
	local result_0 = getPointer(self).ManualActivationOnly;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_tool_manual_activation_only_set_manual_activation_only(self, value)
	getPointer(self).ManualActivationOnly = value == 1;
end
function abi.ffi.property_getter_tool_requires_handle(self)
	local result_0 = getPointer(self).RequiresHandle;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_tool_requires_handle_set_requires_handle(self, value)
	getPointer(self).RequiresHandle = value == 1;
end
function abi.ffi.property_getter_tool_tool_tip(self)
	local result_0 = getPointer(self).ToolTip;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_tool_tool_tip_set_tool_tip(self, value_addr, value_len)
	getPointer(self).ToolTip = loadString(memory, value_addr, value_len);
end
function abi.ffi.method_tool_activate(self)
	getPointer(self):Activate()
end
function abi.ffi.method_tool_deactivate(self)
	getPointer(self):Deactivate()
end
function abi.ffi.event_tool_activated_on_activated(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Activated:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_tool_deactivated_on_deactivated(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Deactivated:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_tool_equipped_on_equipped(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Equipped:Connect(function(mouse)
		local mouse_optional, mouse_optional_0 = 0, 0;
		if (mouse ~= nil) then
			mouse_optional, mouse_optional_0 = 1, createPointer(mouse);
		end
		invokeFunction(callback_data, callback_vtable, mouse_optional, mouse_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_tool_unequipped_on_unequipped(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Unequipped:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.method_badge_service_award_badge(self, user_id, badge_id)
	local result_0 = getPointer(self):AwardBadge(user_id, badge_id);
	return result_0 and 1 or 0;
end
function abi.ffi.method_badge_service_user_has_badge_async(self, user_id, badge_id)
	local result_0 = getPointer(self):UserHasBadgeAsync(user_id, badge_id);
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_player_gui_current_screen_orientation(self)
	local result_0 = getPointer(self).CurrentScreenOrientation;
	return result_0;
end
function abi.ffi.property_getter_player_gui_screen_orientation(self)
	local result_0 = getPointer(self).ScreenOrientation;
	return result_0;
end
function abi.ffi.property_setter_player_gui_screen_orientation_set_screen_orientation(self, value)
	getPointer(self).ScreenOrientation = value;
end
function abi.ffi.property_getter_player_gui_selection_image_object(self)
	local result_0 = getPointer(self).SelectionImageObject;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_player_gui_selection_image_object_set_selection_image_object(self, value)
	getPointer(self).SelectionImageObject = getPointer(value);
end
function abi.ffi.property_getter_starter_gui_screen_orientation(self)
	local result_0 = getPointer(self).ScreenOrientation;
	return result_0;
end
function abi.ffi.property_setter_starter_gui_screen_orientation_set_screen_orientation(self, value)
	getPointer(self).ScreenOrientation = value;
end
function abi.ffi.property_getter_starter_gui_show_development_gui(self)
	local result_0 = getPointer(self).ShowDevelopmentGui;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_starter_gui_show_development_gui_set_show_development_gui(self, value)
	getPointer(self).ShowDevelopmentGui = value == 1;
end
function abi.ffi.method_starter_gui_get_core_gui_enabled(self, core_gui_type)
	local result_0 = getPointer(self):GetCoreGuiEnabled(core_gui_type);
	return result_0 and 1 or 0;
end
function abi.ffi.method_starter_gui_set_core_gui_enabled(self, core_gui_type, enabled)
	getPointer(self):SetCoreGuiEnabled(core_gui_type, enabled == 1)
end
function abi.ffi.property_getter_base_wrap_cage_mesh_id(self)
	local result_0 = getPointer(self).CageMeshId;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_wrap_cage_mesh_id_set_cage_mesh_id(self, value)
	getPointer(self).CageMeshId = getPointer(value);
end
function abi.ffi.property_getter_base_wrap_cage_origin(self)
	local result_0 = getPointer(self).CageOrigin;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_wrap_cage_origin_set_cage_origin(self, value)
	getPointer(self).CageOrigin = getPointer(value);
end
function abi.ffi.property_getter_base_wrap_cage_origin_world(self)
	local result_0 = getPointer(self).CageOriginWorld;
	return createPointer(result_0);
end
function abi.ffi.property_getter_base_wrap_import_origin(self)
	local result_0 = getPointer(self).ImportOrigin;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_wrap_import_origin_set_import_origin(self, value)
	getPointer(self).ImportOrigin = getPointer(value);
end
function abi.ffi.property_getter_base_wrap_import_origin_world(self)
	local result_0 = getPointer(self).ImportOriginWorld;
	return createPointer(result_0);
end
function abi.ffi.property_getter_wrap_layer_auto_skin(self)
	local result_0 = getPointer(self).AutoSkin;
	return result_0;
end
function abi.ffi.property_setter_wrap_layer_auto_skin_set_auto_skin(self, value)
	getPointer(self).AutoSkin = value;
end
function abi.ffi.property_getter_wrap_layer_bind_offset(self)
	local result_0 = getPointer(self).BindOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_wrap_layer_bind_offset_set_bind_offset(self, value)
	getPointer(self).BindOffset = getPointer(value);
end
function abi.ffi.property_getter_wrap_layer_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_wrap_layer_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_wrap_layer_order(self)
	local result_0 = getPointer(self).Order;
	return result_0;
end
function abi.ffi.property_setter_wrap_layer_order_set_order(self, value)
	getPointer(self).Order = value;
end
function abi.ffi.property_getter_wrap_layer_puffiness(self)
	local result_0 = getPointer(self).Puffiness;
	return result_0;
end
function abi.ffi.property_setter_wrap_layer_puffiness_set_puffiness(self, value)
	getPointer(self).Puffiness = value;
end
function abi.ffi.property_getter_wrap_layer_reference_mesh_id(self)
	local result_0 = getPointer(self).ReferenceMeshId;
	return createPointer(result_0);
end
function abi.ffi.property_setter_wrap_layer_reference_mesh_id_set_reference_mesh_id(self, value)
	getPointer(self).ReferenceMeshId = getPointer(value);
end
function abi.ffi.property_getter_wrap_layer_reference_origin(self)
	local result_0 = getPointer(self).ReferenceOrigin;
	return createPointer(result_0);
end
function abi.ffi.property_setter_wrap_layer_reference_origin_set_reference_origin(self, value)
	getPointer(self).ReferenceOrigin = getPointer(value);
end
function abi.ffi.property_getter_wrap_layer_reference_origin_world(self)
	local result_0 = getPointer(self).ReferenceOriginWorld;
	return createPointer(result_0);
end
function abi.ffi.property_getter_wrap_layer_shrink_factor(self)
	local result_0 = getPointer(self).ShrinkFactor;
	return result_0;
end
function abi.ffi.property_setter_wrap_layer_shrink_factor_set_shrink_factor(self, value)
	getPointer(self).ShrinkFactor = value;
end
function abi.ffi.property_getter_wrap_target_stiffness(self)
	local result_0 = getPointer(self).Stiffness;
	return result_0;
end
function abi.ffi.property_setter_wrap_target_stiffness_set_stiffness(self, value)
	getPointer(self).Stiffness = value;
end
function abi.ffi.property_getter_beam_attachment_0(self)
	local result_0 = getPointer(self).Attachment0;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_beam_attachment_0_set_attachment_0(self, value)
	getPointer(self).Attachment0 = getPointer(value);
end
function abi.ffi.property_getter_beam_attachment_1(self)
	local result_0 = getPointer(self).Attachment1;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_beam_attachment_1_set_attachment_1(self, value)
	getPointer(self).Attachment1 = getPointer(value);
end
function abi.ffi.property_getter_beam_brightness(self)
	local result_0 = getPointer(self).Brightness;
	return result_0;
end
function abi.ffi.property_setter_beam_brightness_set_brightness(self, value)
	getPointer(self).Brightness = value;
end
function abi.ffi.property_getter_beam_color(self)
	local result_0 = getPointer(self).Color;
	return createPointer(result_0);
end
function abi.ffi.property_setter_beam_color_set_color(self, value)
	getPointer(self).Color = getPointer(value);
end
function abi.ffi.property_getter_beam_curve_size_0(self)
	local result_0 = getPointer(self).CurveSize0;
	return result_0;
end
function abi.ffi.property_setter_beam_curve_size_0_set_curve_size_0(self, value)
	getPointer(self).CurveSize0 = value;
end
function abi.ffi.property_getter_beam_curve_size_1(self)
	local result_0 = getPointer(self).CurveSize1;
	return result_0;
end
function abi.ffi.property_setter_beam_curve_size_1_set_curve_size_1(self, value)
	getPointer(self).CurveSize1 = value;
end
function abi.ffi.property_getter_beam_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_beam_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_beam_face_camera(self)
	local result_0 = getPointer(self).FaceCamera;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_beam_face_camera_set_face_camera(self, value)
	getPointer(self).FaceCamera = value == 1;
end
function abi.ffi.property_getter_beam_light_emission(self)
	local result_0 = getPointer(self).LightEmission;
	return result_0;
end
function abi.ffi.property_setter_beam_light_emission_set_light_emission(self, value)
	getPointer(self).LightEmission = value;
end
function abi.ffi.property_getter_beam_light_influence(self)
	local result_0 = getPointer(self).LightInfluence;
	return result_0;
end
function abi.ffi.property_setter_beam_light_influence_set_light_influence(self, value)
	getPointer(self).LightInfluence = value;
end
function abi.ffi.property_getter_beam_segments(self)
	local result_0 = getPointer(self).Segments;
	return result_0;
end
function abi.ffi.property_setter_beam_segments_set_segments(self, value)
	getPointer(self).Segments = value;
end
function abi.ffi.property_getter_beam_texture(self)
	local result_0 = getPointer(self).Texture;
	return createPointer(result_0);
end
function abi.ffi.property_setter_beam_texture_set_texture(self, value)
	getPointer(self).Texture = getPointer(value);
end
function abi.ffi.property_getter_beam_texture_length(self)
	local result_0 = getPointer(self).TextureLength;
	return result_0;
end
function abi.ffi.property_setter_beam_texture_length_set_texture_length(self, value)
	getPointer(self).TextureLength = value;
end
function abi.ffi.property_getter_beam_texture_mode(self)
	local result_0 = getPointer(self).TextureMode;
	return result_0;
end
function abi.ffi.property_setter_beam_texture_mode_set_texture_mode(self, value)
	getPointer(self).TextureMode = value;
end
function abi.ffi.property_getter_beam_texture_speed(self)
	local result_0 = getPointer(self).TextureSpeed;
	return result_0;
end
function abi.ffi.property_setter_beam_texture_speed_set_texture_speed(self, value)
	getPointer(self).TextureSpeed = value;
end
function abi.ffi.property_getter_beam_transparency(self)
	local result_0 = getPointer(self).Transparency;
	return createPointer(result_0);
end
function abi.ffi.property_setter_beam_transparency_set_transparency(self, value)
	getPointer(self).Transparency = getPointer(value);
end
function abi.ffi.property_getter_beam_width_0(self)
	local result_0 = getPointer(self).Width0;
	return result_0;
end
function abi.ffi.property_setter_beam_width_0_set_width_0(self, value)
	getPointer(self).Width0 = value;
end
function abi.ffi.property_getter_beam_width_1(self)
	local result_0 = getPointer(self).Width1;
	return result_0;
end
function abi.ffi.property_setter_beam_width_1_set_width_1(self, value)
	getPointer(self).Width1 = value;
end
function abi.ffi.property_getter_beam_z_offset(self)
	local result_0 = getPointer(self).ZOffset;
	return result_0;
end
function abi.ffi.property_setter_beam_z_offset_set_z_offset(self, value)
	getPointer(self).ZOffset = value;
end
function abi.ffi.method_beam_set_texture_offset(self, offset)
	getPointer(self):SetTextureOffset(offset)
end
function abi.ffi.event_bindable_event_event(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Event:Connect(function(...)
		local values_tuple = { ... };
		local values_len = select("#", ...);
		local values_tuple_vec = allocVec(values_len * 4)
		for i = 1, values_len do
			local values_tuple_vec_value = values_tuple[i];
			storeU32(memory, values_tuple_vec + (i - 1) * 4 + 0, createPointer(values_tuple_vec_value));
		end
		invokeFunction(callback_data, callback_vtable, values_tuple_vec, values_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.method_bindable_event_fire(self, values_addr, values_len)
	local values_vec = table.create(values_len);
	for i = 1, values_len do
		local vecElement_0 = loadU32(memory, values_addr + (i - 1) * 4 + 0)
		values_vec[i] = getPointer(vecElement_0);
	end
	getPointer(self):Fire(unpack(values_vec, 1, values_len))
end
function abi.ffi.property_getter_body_angular_velocity_angular_velocity(self)
	local result_0 = getPointer(self).AngularVelocity;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_angular_velocity_angular_velocity_set_angular_velocity(self, value)
	getPointer(self).AngularVelocity = getPointer(value);
end
function abi.ffi.property_getter_body_angular_velocity_max_torque(self)
	local result_0 = getPointer(self).MaxTorque;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_angular_velocity_max_torque_set_max_torque(self, value)
	getPointer(self).MaxTorque = getPointer(value);
end
function abi.ffi.property_getter_body_angular_velocity_p(self)
	local result_0 = getPointer(self).P;
	return result_0;
end
function abi.ffi.property_setter_body_angular_velocity_p_set_p(self, value)
	getPointer(self).P = value;
end
function abi.ffi.property_getter_body_force_force(self)
	local result_0 = getPointer(self).Force;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_force_force_set_force(self, value)
	getPointer(self).Force = getPointer(value);
end
function abi.ffi.property_getter_body_gyro_c_frame(self)
	local result_0 = getPointer(self).CFrame;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_gyro_c_frame_set_c_frame(self, value)
	getPointer(self).CFrame = getPointer(value);
end
function abi.ffi.property_getter_body_gyro_d(self)
	local result_0 = getPointer(self).D;
	return result_0;
end
function abi.ffi.property_setter_body_gyro_d_set_d(self, value)
	getPointer(self).D = value;
end
function abi.ffi.property_getter_body_gyro_max_torque(self)
	local result_0 = getPointer(self).MaxTorque;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_gyro_max_torque_set_max_torque(self, value)
	getPointer(self).MaxTorque = getPointer(value);
end
function abi.ffi.property_getter_body_gyro_p(self)
	local result_0 = getPointer(self).P;
	return result_0;
end
function abi.ffi.property_setter_body_gyro_p_set_p(self, value)
	getPointer(self).P = value;
end
function abi.ffi.property_getter_body_position_d(self)
	local result_0 = getPointer(self).D;
	return result_0;
end
function abi.ffi.property_setter_body_position_d_set_d(self, value)
	getPointer(self).D = value;
end
function abi.ffi.property_getter_body_position_max_force(self)
	local result_0 = getPointer(self).MaxForce;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_position_max_force_set_max_force(self, value)
	getPointer(self).MaxForce = getPointer(value);
end
function abi.ffi.property_getter_body_position_p(self)
	local result_0 = getPointer(self).P;
	return result_0;
end
function abi.ffi.property_setter_body_position_p_set_p(self, value)
	getPointer(self).P = value;
end
function abi.ffi.property_getter_body_position_position(self)
	local result_0 = getPointer(self).Position;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_position_position_set_position(self, value)
	getPointer(self).Position = getPointer(value);
end
function abi.ffi.method_body_position_get_last_force(self)
	local result_0 = getPointer(self):GetLastForce();
	return createPointer(result_0);
end
function abi.ffi.event_body_position_reached_target_on_reached_target(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).ReachedTarget:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_body_thrust_force(self)
	local result_0 = getPointer(self).Force;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_thrust_force_set_force(self, value)
	getPointer(self).Force = getPointer(value);
end
function abi.ffi.property_getter_body_thrust_location(self)
	local result_0 = getPointer(self).Location;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_thrust_location_set_location(self, value)
	getPointer(self).Location = getPointer(value);
end
function abi.ffi.property_getter_body_velocity_max_force(self)
	local result_0 = getPointer(self).MaxForce;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_velocity_max_force_set_max_force(self, value)
	getPointer(self).MaxForce = getPointer(value);
end
function abi.ffi.property_getter_body_velocity_p(self)
	local result_0 = getPointer(self).P;
	return result_0;
end
function abi.ffi.property_setter_body_velocity_p_set_p(self, value)
	getPointer(self).P = value;
end
function abi.ffi.property_getter_body_velocity_velocity(self)
	local result_0 = getPointer(self).Velocity;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_velocity_velocity_set_velocity(self, value)
	getPointer(self).Velocity = getPointer(value);
end
function abi.ffi.method_body_velocity_get_last_force(self)
	local result_0 = getPointer(self):GetLastForce();
	return createPointer(result_0);
end
function abi.ffi.property_getter_rocket_propulsion_cartoon_factor(self)
	local result_0 = getPointer(self).CartoonFactor;
	return result_0;
end
function abi.ffi.property_setter_rocket_propulsion_cartoon_factor_set_cartoon_factor(self, value)
	getPointer(self).CartoonFactor = value;
end
function abi.ffi.property_getter_rocket_propulsion_max_speed(self)
	local result_0 = getPointer(self).MaxSpeed;
	return result_0;
end
function abi.ffi.property_setter_rocket_propulsion_max_speed_set_max_speed(self, value)
	getPointer(self).MaxSpeed = value;
end
function abi.ffi.property_getter_rocket_propulsion_max_thrust(self)
	local result_0 = getPointer(self).MaxThrust;
	return result_0;
end
function abi.ffi.property_setter_rocket_propulsion_max_thrust_set_max_thrust(self, value)
	getPointer(self).MaxThrust = value;
end
function abi.ffi.property_getter_rocket_propulsion_max_torque(self)
	local result_0 = getPointer(self).MaxTorque;
	return createPointer(result_0);
end
function abi.ffi.property_setter_rocket_propulsion_max_torque_set_max_torque(self, value)
	getPointer(self).MaxTorque = getPointer(value);
end
function abi.ffi.property_getter_rocket_propulsion_target(self)
	local result_0 = getPointer(self).Target;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_rocket_propulsion_target_set_target(self, value)
	getPointer(self).Target = getPointer(value);
end
function abi.ffi.property_getter_rocket_propulsion_target_offset(self)
	local result_0 = getPointer(self).TargetOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_rocket_propulsion_target_offset_set_target_offset(self, value)
	getPointer(self).TargetOffset = getPointer(value);
end
function abi.ffi.property_getter_rocket_propulsion_target_radius(self)
	local result_0 = getPointer(self).TargetRadius;
	return result_0;
end
function abi.ffi.property_setter_rocket_propulsion_target_radius_set_target_radius(self, value)
	getPointer(self).TargetRadius = value;
end
function abi.ffi.property_getter_rocket_propulsion_thrust_d(self)
	local result_0 = getPointer(self).ThrustD;
	return result_0;
end
function abi.ffi.property_setter_rocket_propulsion_thrust_d_set_thrust_d(self, value)
	getPointer(self).ThrustD = value;
end
function abi.ffi.property_getter_rocket_propulsion_thrust_p(self)
	local result_0 = getPointer(self).ThrustP;
	return result_0;
end
function abi.ffi.property_setter_rocket_propulsion_thrust_p_set_thrust_p(self, value)
	getPointer(self).ThrustP = value;
end
function abi.ffi.property_getter_rocket_propulsion_turn_d(self)
	local result_0 = getPointer(self).TurnD;
	return result_0;
end
function abi.ffi.property_setter_rocket_propulsion_turn_d_set_turn_d(self, value)
	getPointer(self).TurnD = value;
end
function abi.ffi.property_getter_rocket_propulsion_turn_p(self)
	local result_0 = getPointer(self).TurnP;
	return result_0;
end
function abi.ffi.property_setter_rocket_propulsion_turn_p_set_turn_p(self, value)
	getPointer(self).TurnP = value;
end
function abi.ffi.method_rocket_propulsion_abort(self)
	getPointer(self):Abort()
end
function abi.ffi.method_rocket_propulsion_fire(self)
	getPointer(self):Fire()
end
function abi.ffi.event_rocket_propulsion_reached_target_on_reached_target(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).ReachedTarget:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_camera_c_frame(self)
	local result_0 = getPointer(self).CFrame;
	return createPointer(result_0);
end
function abi.ffi.property_setter_camera_c_frame_set_c_frame(self, value)
	getPointer(self).CFrame = getPointer(value);
end
function abi.ffi.property_getter_camera_camera_subject(self)
	local result_0 = getPointer(self).CameraSubject;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_camera_camera_subject_set_camera_subject(self, value)
	getPointer(self).CameraSubject = getPointer(value);
end
function abi.ffi.property_getter_camera_camera_type(self)
	local result_0 = getPointer(self).CameraType;
	return result_0;
end
function abi.ffi.property_setter_camera_camera_type_set_camera_type(self, value)
	getPointer(self).CameraType = value;
end
function abi.ffi.property_getter_camera_diagonal_field_of_view(self)
	local result_0 = getPointer(self).DiagonalFieldOfView;
	return result_0;
end
function abi.ffi.property_setter_camera_diagonal_field_of_view_set_diagonal_field_of_view(self, value)
	getPointer(self).DiagonalFieldOfView = value;
end
function abi.ffi.property_getter_camera_field_of_view(self)
	local result_0 = getPointer(self).FieldOfView;
	return result_0;
end
function abi.ffi.property_setter_camera_field_of_view_set_field_of_view(self, value)
	getPointer(self).FieldOfView = value;
end
function abi.ffi.property_getter_camera_field_of_view_mode(self)
	local result_0 = getPointer(self).FieldOfViewMode;
	return result_0;
end
function abi.ffi.property_setter_camera_field_of_view_mode_set_field_of_view_mode(self, value)
	getPointer(self).FieldOfViewMode = value;
end
function abi.ffi.property_getter_camera_focus(self)
	local result_0 = getPointer(self).Focus;
	return createPointer(result_0);
end
function abi.ffi.property_setter_camera_focus_set_focus(self, value)
	getPointer(self).Focus = getPointer(value);
end
function abi.ffi.property_getter_camera_head_locked(self)
	local result_0 = getPointer(self).HeadLocked;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_camera_head_locked_set_head_locked(self, value)
	getPointer(self).HeadLocked = value == 1;
end
function abi.ffi.property_getter_camera_head_scale(self)
	local result_0 = getPointer(self).HeadScale;
	return result_0;
end
function abi.ffi.property_setter_camera_head_scale_set_head_scale(self, value)
	getPointer(self).HeadScale = value;
end
function abi.ffi.property_getter_camera_max_axis_field_of_view(self)
	local result_0 = getPointer(self).MaxAxisFieldOfView;
	return result_0;
end
function abi.ffi.property_setter_camera_max_axis_field_of_view_set_max_axis_field_of_view(self, value)
	getPointer(self).MaxAxisFieldOfView = value;
end
function abi.ffi.property_getter_camera_near_plane_z(self)
	local result_0 = getPointer(self).NearPlaneZ;
	return result_0;
end
function abi.ffi.property_getter_camera_viewport_size(self)
	local result_0 = getPointer(self).ViewportSize;
	return createPointer(result_0);
end
function abi.ffi.method_camera_get_pan_speed(self)
	local result_0 = getPointer(self):GetPanSpeed();
	return result_0;
end
function abi.ffi.method_camera_get_render_c_frame(self)
	local result_0 = getPointer(self):GetRenderCFrame();
	return createPointer(result_0);
end
function abi.ffi.method_camera_get_roll(self)
	local result_0 = getPointer(self):GetRoll();
	return result_0;
end
function abi.ffi.method_camera_get_tilt_speed(self)
	local result_0 = getPointer(self):GetTiltSpeed();
	return result_0;
end
function abi.ffi.method_camera_screen_point_to_ray(self, x, y, depth)
	local result_0 = getPointer(self):ScreenPointToRay(x, y, depth);
	return createPointer(result_0);
end
function abi.ffi.method_camera_set_camera_pan_mode(self, mode)
	getPointer(self):SetCameraPanMode(mode)
end
function abi.ffi.method_camera_set_roll(self, roll_angle)
	getPointer(self):SetRoll(roll_angle)
end
function abi.ffi.method_camera_viewport_point_to_ray(self, x, y, depth)
	local result_0 = getPointer(self):ViewportPointToRay(x, y, depth);
	return createPointer(result_0);
end
function abi.ffi.event_camera_interpolation_finished_on_interpolation_finished(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).InterpolationFinished:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_body_colors_head_color(self)
	local result_0 = getPointer(self).HeadColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_colors_head_color_set_head_color(self, value)
	getPointer(self).HeadColor = getPointer(value);
end
function abi.ffi.property_getter_body_colors_head_color_3(self)
	local result_0 = getPointer(self).HeadColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_colors_head_color_3_set_head_color_3(self, value)
	getPointer(self).HeadColor3 = getPointer(value);
end
function abi.ffi.property_getter_body_colors_left_arm_color(self)
	local result_0 = getPointer(self).LeftArmColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_colors_left_arm_color_set_left_arm_color(self, value)
	getPointer(self).LeftArmColor = getPointer(value);
end
function abi.ffi.property_getter_body_colors_left_arm_color_3(self)
	local result_0 = getPointer(self).LeftArmColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_colors_left_arm_color_3_set_left_arm_color_3(self, value)
	getPointer(self).LeftArmColor3 = getPointer(value);
end
function abi.ffi.property_getter_body_colors_left_leg_color(self)
	local result_0 = getPointer(self).LeftLegColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_colors_left_leg_color_set_left_leg_color(self, value)
	getPointer(self).LeftLegColor = getPointer(value);
end
function abi.ffi.property_getter_body_colors_left_leg_color_3(self)
	local result_0 = getPointer(self).LeftLegColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_colors_left_leg_color_3_set_left_leg_color_3(self, value)
	getPointer(self).LeftLegColor3 = getPointer(value);
end
function abi.ffi.property_getter_body_colors_right_arm_color(self)
	local result_0 = getPointer(self).RightArmColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_colors_right_arm_color_set_right_arm_color(self, value)
	getPointer(self).RightArmColor = getPointer(value);
end
function abi.ffi.property_getter_body_colors_right_arm_color_3(self)
	local result_0 = getPointer(self).RightArmColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_colors_right_arm_color_3_set_right_arm_color_3(self, value)
	getPointer(self).RightArmColor3 = getPointer(value);
end
function abi.ffi.property_getter_body_colors_right_leg_color(self)
	local result_0 = getPointer(self).RightLegColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_colors_right_leg_color_set_right_leg_color(self, value)
	getPointer(self).RightLegColor = getPointer(value);
end
function abi.ffi.property_getter_body_colors_right_leg_color_3(self)
	local result_0 = getPointer(self).RightLegColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_colors_right_leg_color_3_set_right_leg_color_3(self, value)
	getPointer(self).RightLegColor3 = getPointer(value);
end
function abi.ffi.property_getter_body_colors_torso_color(self)
	local result_0 = getPointer(self).TorsoColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_colors_torso_color_set_torso_color(self, value)
	getPointer(self).TorsoColor = getPointer(value);
end
function abi.ffi.property_getter_body_colors_torso_color_3(self)
	local result_0 = getPointer(self).TorsoColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_body_colors_torso_color_3_set_torso_color_3(self, value)
	getPointer(self).TorsoColor3 = getPointer(value);
end
function abi.ffi.property_getter_character_mesh_base_texture_id(self)
	local result_0 = getPointer(self).BaseTextureId;
	return result_0;
end
function abi.ffi.property_setter_character_mesh_base_texture_id_set_base_texture_id(self, value)
	getPointer(self).BaseTextureId = value;
end
function abi.ffi.property_getter_character_mesh_body_part(self)
	local result_0 = getPointer(self).BodyPart;
	return result_0;
end
function abi.ffi.property_setter_character_mesh_body_part_set_body_part(self, value)
	getPointer(self).BodyPart = value;
end
function abi.ffi.property_getter_character_mesh_mesh_id(self)
	local result_0 = getPointer(self).MeshId;
	return result_0;
end
function abi.ffi.property_setter_character_mesh_mesh_id_set_mesh_id(self, value)
	getPointer(self).MeshId = value;
end
function abi.ffi.property_getter_character_mesh_overlay_texture_id(self)
	local result_0 = getPointer(self).OverlayTextureId;
	return result_0;
end
function abi.ffi.property_setter_character_mesh_overlay_texture_id_set_overlay_texture_id(self, value)
	getPointer(self).OverlayTextureId = value;
end
function abi.ffi.property_getter_clothing_color_3(self)
	local result_0 = getPointer(self).Color3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_clothing_color_3_set_color_3(self, value)
	getPointer(self).Color3 = getPointer(value);
end
function abi.ffi.property_getter_pants_pants_template(self)
	local result_0 = getPointer(self).PantsTemplate;
	return createPointer(result_0);
end
function abi.ffi.property_setter_pants_pants_template_set_pants_template(self, value)
	getPointer(self).PantsTemplate = getPointer(value);
end
function abi.ffi.property_getter_shirt_shirt_template(self)
	local result_0 = getPointer(self).ShirtTemplate;
	return createPointer(result_0);
end
function abi.ffi.property_setter_shirt_shirt_template_set_shirt_template(self, value)
	getPointer(self).ShirtTemplate = getPointer(value);
end
function abi.ffi.property_getter_shirt_graphic_color_3(self)
	local result_0 = getPointer(self).Color3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_shirt_graphic_color_3_set_color_3(self, value)
	getPointer(self).Color3 = getPointer(value);
end
function abi.ffi.property_getter_shirt_graphic_graphic(self)
	local result_0 = getPointer(self).Graphic;
	return createPointer(result_0);
end
function abi.ffi.property_setter_shirt_graphic_graphic_set_graphic(self, value)
	getPointer(self).Graphic = getPointer(value);
end
function abi.ffi.property_getter_chat_bubble_chat_enabled(self)
	local result_0 = getPointer(self).BubbleChatEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_chat_bubble_chat_enabled_set_bubble_chat_enabled(self, value)
	getPointer(self).BubbleChatEnabled = value == 1;
end
function abi.ffi.property_getter_chat_load_default_chat(self)
	local result_0 = getPointer(self).LoadDefaultChat;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_chat_load_default_chat_set_load_default_chat(self, value)
	getPointer(self).LoadDefaultChat = value == 1;
end
function abi.ffi.method_chat_chat(self, part_or_character, message_addr, message_len, color)
	getPointer(self):Chat(getPointer(part_or_character), loadString(memory, message_addr, message_len), color)
end
function abi.ffi.method_chat_can_user_chat_async(self, user_id)
	local result_0 = getPointer(self):CanUserChatAsync(user_id);
	return result_0 and 1 or 0;
end
function abi.ffi.method_chat_can_users_chat_async(self, user_id_from, user_id_to)
	local result_0 = getPointer(self):CanUsersChatAsync(user_id_from, user_id_to);
	return result_0 and 1 or 0;
end
function abi.ffi.method_chat_filter_string_async(self, string_to_filter_addr, string_to_filter_len, player_from, player_to)
	local result_0 = getPointer(self):FilterStringAsync(loadString(memory, string_to_filter_addr, string_to_filter_len), getPointer(player_from), getPointer(player_to));
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_chat_filter_string_for_broadcast(self, string_to_filter_addr, string_to_filter_len, player_from)
	local result_0 = getPointer(self):FilterStringForBroadcast(loadString(memory, string_to_filter_addr, string_to_filter_len), getPointer(player_from));
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.event_chat_chatted_on_chatted(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Chatted:Connect(function(part, message, color)
		local part_optional, part_optional_0 = 0, 0;
		if (part ~= nil) then
			part_optional, part_optional_0 = 1, createPointer(part);
		end
		local message_len = #message;
		local message_content = allocString(message_len);
		for i = 1, message_len do
			storeU8(memory, message_content + (i - 1), string.byte(message:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, part_optional, part_optional_0, message_content, message_len, color)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_click_detector_cursor_icon(self)
	local result_0 = getPointer(self).CursorIcon;
	return createPointer(result_0);
end
function abi.ffi.property_setter_click_detector_cursor_icon_set_cursor_icon(self, value)
	getPointer(self).CursorIcon = getPointer(value);
end
function abi.ffi.property_getter_click_detector_max_activation_distance(self)
	local result_0 = getPointer(self).MaxActivationDistance;
	return result_0;
end
function abi.ffi.property_setter_click_detector_max_activation_distance_set_max_activation_distance(self, value)
	getPointer(self).MaxActivationDistance = value;
end
function abi.ffi.event_click_detector_mouse_click_on_mouse_click(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseClick:Connect(function(player_who_clicked)
		local player_who_clicked_optional, player_who_clicked_optional_0 = 0, 0;
		if (player_who_clicked ~= nil) then
			player_who_clicked_optional, player_who_clicked_optional_0 = 1, createPointer(player_who_clicked);
		end
		invokeFunction(callback_data, callback_vtable, player_who_clicked_optional, player_who_clicked_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_click_detector_mouse_hover_enter_on_mouse_hover_enter(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseHoverEnter:Connect(function(player_who_hovered)
		local player_who_hovered_optional, player_who_hovered_optional_0 = 0, 0;
		if (player_who_hovered ~= nil) then
			player_who_hovered_optional, player_who_hovered_optional_0 = 1, createPointer(player_who_hovered);
		end
		invokeFunction(callback_data, callback_vtable, player_who_hovered_optional, player_who_hovered_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_click_detector_mouse_hover_leave_on_mouse_hover_leave(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseHoverLeave:Connect(function(player_who_hovered)
		local player_who_hovered_optional, player_who_hovered_optional_0 = 0, 0;
		if (player_who_hovered ~= nil) then
			player_who_hovered_optional, player_who_hovered_optional_0 = 1, createPointer(player_who_hovered);
		end
		invokeFunction(callback_data, callback_vtable, player_who_hovered_optional, player_who_hovered_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_click_detector_right_mouse_click_on_right_mouse_click(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).RightMouseClick:Connect(function(player_who_clicked)
		local player_who_clicked_optional, player_who_clicked_optional_0 = 0, 0;
		if (player_who_clicked ~= nil) then
			player_who_clicked_optional, player_who_clicked_optional_0 = 1, createPointer(player_who_clicked);
		end
		invokeFunction(callback_data, callback_vtable, player_who_clicked_optional, player_who_clicked_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_clouds_color(self)
	local result_0 = getPointer(self).Color;
	return createPointer(result_0);
end
function abi.ffi.property_setter_clouds_color_set_color(self, value)
	getPointer(self).Color = getPointer(value);
end
function abi.ffi.property_getter_clouds_cover(self)
	local result_0 = getPointer(self).Cover;
	return result_0;
end
function abi.ffi.property_setter_clouds_cover_set_cover(self, value)
	getPointer(self).Cover = value;
end
function abi.ffi.property_getter_clouds_density(self)
	local result_0 = getPointer(self).Density;
	return result_0;
end
function abi.ffi.property_setter_clouds_density_set_density(self, value)
	getPointer(self).Density = value;
end
function abi.ffi.property_getter_clouds_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_clouds_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.method_collection_service_add_tag(self, instance, tag_addr, tag_len)
	getPointer(self):AddTag(getPointer(instance), loadString(memory, tag_addr, tag_len))
end
function abi.ffi.method_collection_service_has_tag(self, instance, tag_addr, tag_len)
	local result_0 = getPointer(self):HasTag(getPointer(instance), loadString(memory, tag_addr, tag_len));
	return result_0 and 1 or 0;
end
function abi.ffi.method_collection_service_remove_tag(self, instance, tag_addr, tag_len)
	getPointer(self):RemoveTag(getPointer(instance), loadString(memory, tag_addr, tag_len))
end
function abi.ffi.event_collection_service_tag_added_on_tag_added(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TagAdded:Connect(function(tag)
		local tag_len = #tag;
		local tag_content = allocString(tag_len);
		for i = 1, tag_len do
			storeU8(memory, tag_content + (i - 1), string.byte(tag:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, tag_content, tag_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_collection_service_tag_removed_on_tag_removed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TagRemoved:Connect(function(tag)
		local tag_len = #tag;
		local tag_content = allocString(tag_len);
		for i = 1, tag_len do
			storeU8(memory, tag_content + (i - 1), string.byte(tag:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, tag_content, tag_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.method_collection_service_get_tags(self, instance)
	local result_0 = getPointer(self):GetTags(getPointer(instance));
	local result_0_len = #result_0;
	local result_0_vec = allocVec(result_0_len * 8)
	for i = 1, result_0_len do
		local result_0_vec_value = result_0[i];
		local result_0_vec_value_len = #result_0_vec_value;
		local result_0_vec_value_content = allocString(result_0_vec_value_len);
		for i = 1, result_0_vec_value_len do
			storeU8(memory, result_0_vec_value_content + (i - 1), string.byte(result_0_vec_value:sub(i, i)));
		end
		storeU32(memory, result_0_vec + (i - 1) * 8 + 0, result_0_vec_value_content);
		storeU32(memory, result_0_vec + (i - 1) * 8 + 4, result_0_vec_value_len);
	end
	return result_0_vec, result_0_len;
end
function abi.ffi.property_getter_command_instance_allow_gui_access_points(self)
	local result_0 = getPointer(self).AllowGUIAccessPoints;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_command_instance_display_name(self)
	local result_0 = getPointer(self).DisplayName;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_command_instance_display_name_set_display_name(self, value_addr, value_len)
	getPointer(self).DisplayName = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_constraint_active(self)
	local result_0 = getPointer(self).Active;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_constraint_attachment_0(self)
	local result_0 = getPointer(self).Attachment0;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_constraint_attachment_0_set_attachment_0(self, value)
	getPointer(self).Attachment0 = getPointer(value);
end
function abi.ffi.property_getter_constraint_attachment_1(self)
	local result_0 = getPointer(self).Attachment1;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_constraint_attachment_1_set_attachment_1(self, value)
	getPointer(self).Attachment1 = getPointer(value);
end
function abi.ffi.property_getter_constraint_color(self)
	local result_0 = getPointer(self).Color;
	return createPointer(result_0);
end
function abi.ffi.property_setter_constraint_color_set_color(self, value)
	getPointer(self).Color = getPointer(value);
end
function abi.ffi.property_getter_constraint_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_constraint_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_constraint_visible(self)
	local result_0 = getPointer(self).Visible;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_constraint_visible_set_visible(self, value)
	getPointer(self).Visible = value == 1;
end
function abi.ffi.property_getter_align_orientation_align_type(self)
	local result_0 = getPointer(self).AlignType;
	return result_0;
end
function abi.ffi.property_setter_align_orientation_align_type_set_align_type(self, value)
	getPointer(self).AlignType = value;
end
function abi.ffi.property_getter_align_orientation_c_frame(self)
	local result_0 = getPointer(self).CFrame;
	return createPointer(result_0);
end
function abi.ffi.property_setter_align_orientation_c_frame_set_c_frame(self, value)
	getPointer(self).CFrame = getPointer(value);
end
function abi.ffi.property_getter_align_orientation_max_angular_velocity(self)
	local result_0 = getPointer(self).MaxAngularVelocity;
	return result_0;
end
function abi.ffi.property_setter_align_orientation_max_angular_velocity_set_max_angular_velocity(self, value)
	getPointer(self).MaxAngularVelocity = value;
end
function abi.ffi.property_getter_align_orientation_max_torque(self)
	local result_0 = getPointer(self).MaxTorque;
	return result_0;
end
function abi.ffi.property_setter_align_orientation_max_torque_set_max_torque(self, value)
	getPointer(self).MaxTorque = value;
end
function abi.ffi.property_getter_align_orientation_mode(self)
	local result_0 = getPointer(self).Mode;
	return result_0;
end
function abi.ffi.property_setter_align_orientation_mode_set_mode(self, value)
	getPointer(self).Mode = value;
end
function abi.ffi.property_getter_align_orientation_primary_axis(self)
	local result_0 = getPointer(self).PrimaryAxis;
	return createPointer(result_0);
end
function abi.ffi.property_setter_align_orientation_primary_axis_set_primary_axis(self, value)
	getPointer(self).PrimaryAxis = getPointer(value);
end
function abi.ffi.property_getter_align_orientation_primary_axis_only(self)
	local result_0 = getPointer(self).PrimaryAxisOnly;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_align_orientation_primary_axis_only_set_primary_axis_only(self, value)
	getPointer(self).PrimaryAxisOnly = value == 1;
end
function abi.ffi.property_getter_align_orientation_reaction_torque_enabled(self)
	local result_0 = getPointer(self).ReactionTorqueEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_align_orientation_reaction_torque_enabled_set_reaction_torque_enabled(self, value)
	getPointer(self).ReactionTorqueEnabled = value == 1;
end
function abi.ffi.property_getter_align_orientation_responsiveness(self)
	local result_0 = getPointer(self).Responsiveness;
	return result_0;
end
function abi.ffi.property_setter_align_orientation_responsiveness_set_responsiveness(self, value)
	getPointer(self).Responsiveness = value;
end
function abi.ffi.property_getter_align_orientation_rigidity_enabled(self)
	local result_0 = getPointer(self).RigidityEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_align_orientation_rigidity_enabled_set_rigidity_enabled(self, value)
	getPointer(self).RigidityEnabled = value == 1;
end
function abi.ffi.property_getter_align_orientation_secondary_axis(self)
	local result_0 = getPointer(self).SecondaryAxis;
	return createPointer(result_0);
end
function abi.ffi.property_setter_align_orientation_secondary_axis_set_secondary_axis(self, value)
	getPointer(self).SecondaryAxis = getPointer(value);
end
function abi.ffi.property_getter_align_position_apply_at_center_of_mass(self)
	local result_0 = getPointer(self).ApplyAtCenterOfMass;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_align_position_apply_at_center_of_mass_set_apply_at_center_of_mass(self, value)
	getPointer(self).ApplyAtCenterOfMass = value == 1;
end
function abi.ffi.property_getter_align_position_max_force(self)
	local result_0 = getPointer(self).MaxForce;
	return result_0;
end
function abi.ffi.property_setter_align_position_max_force_set_max_force(self, value)
	getPointer(self).MaxForce = value;
end
function abi.ffi.property_getter_align_position_max_velocity(self)
	local result_0 = getPointer(self).MaxVelocity;
	return result_0;
end
function abi.ffi.property_setter_align_position_max_velocity_set_max_velocity(self, value)
	getPointer(self).MaxVelocity = value;
end
function abi.ffi.property_getter_align_position_mode(self)
	local result_0 = getPointer(self).Mode;
	return result_0;
end
function abi.ffi.property_setter_align_position_mode_set_mode(self, value)
	getPointer(self).Mode = value;
end
function abi.ffi.property_getter_align_position_position(self)
	local result_0 = getPointer(self).Position;
	return createPointer(result_0);
end
function abi.ffi.property_setter_align_position_position_set_position(self, value)
	getPointer(self).Position = getPointer(value);
end
function abi.ffi.property_getter_align_position_reaction_force_enabled(self)
	local result_0 = getPointer(self).ReactionForceEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_align_position_reaction_force_enabled_set_reaction_force_enabled(self, value)
	getPointer(self).ReactionForceEnabled = value == 1;
end
function abi.ffi.property_getter_align_position_responsiveness(self)
	local result_0 = getPointer(self).Responsiveness;
	return result_0;
end
function abi.ffi.property_setter_align_position_responsiveness_set_responsiveness(self, value)
	getPointer(self).Responsiveness = value;
end
function abi.ffi.property_getter_align_position_rigidity_enabled(self)
	local result_0 = getPointer(self).RigidityEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_align_position_rigidity_enabled_set_rigidity_enabled(self, value)
	getPointer(self).RigidityEnabled = value == 1;
end
function abi.ffi.property_getter_angular_velocity_angular_velocity(self)
	local result_0 = getPointer(self).AngularVelocity;
	return createPointer(result_0);
end
function abi.ffi.property_setter_angular_velocity_angular_velocity_set_angular_velocity(self, value)
	getPointer(self).AngularVelocity = getPointer(value);
end
function abi.ffi.property_getter_angular_velocity_max_torque(self)
	local result_0 = getPointer(self).MaxTorque;
	return result_0;
end
function abi.ffi.property_setter_angular_velocity_max_torque_set_max_torque(self, value)
	getPointer(self).MaxTorque = value;
end
function abi.ffi.property_getter_angular_velocity_reaction_torque_enabled(self)
	local result_0 = getPointer(self).ReactionTorqueEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_angular_velocity_reaction_torque_enabled_set_reaction_torque_enabled(self, value)
	getPointer(self).ReactionTorqueEnabled = value == 1;
end
function abi.ffi.property_getter_angular_velocity_relative_to(self)
	local result_0 = getPointer(self).RelativeTo;
	return result_0;
end
function abi.ffi.property_setter_angular_velocity_relative_to_set_relative_to(self, value)
	getPointer(self).RelativeTo = value;
end
function abi.ffi.property_getter_animation_constraint_max_force(self)
	local result_0 = getPointer(self).MaxForce;
	return result_0;
end
function abi.ffi.property_setter_animation_constraint_max_force_set_max_force(self, value)
	getPointer(self).MaxForce = value;
end
function abi.ffi.property_getter_animation_constraint_max_torque(self)
	local result_0 = getPointer(self).MaxTorque;
	return result_0;
end
function abi.ffi.property_setter_animation_constraint_max_torque_set_max_torque(self, value)
	getPointer(self).MaxTorque = value;
end
function abi.ffi.property_getter_animation_constraint_transform(self)
	local result_0 = getPointer(self).Transform;
	return createPointer(result_0);
end
function abi.ffi.property_setter_animation_constraint_transform_set_transform(self, value)
	getPointer(self).Transform = getPointer(value);
end
function abi.ffi.property_getter_ball_socket_constraint_limits_enabled(self)
	local result_0 = getPointer(self).LimitsEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_ball_socket_constraint_limits_enabled_set_limits_enabled(self, value)
	getPointer(self).LimitsEnabled = value == 1;
end
function abi.ffi.property_getter_ball_socket_constraint_max_friction_torque(self)
	local result_0 = getPointer(self).MaxFrictionTorque;
	return result_0;
end
function abi.ffi.property_setter_ball_socket_constraint_max_friction_torque_set_max_friction_torque(self, value)
	getPointer(self).MaxFrictionTorque = value;
end
function abi.ffi.property_getter_ball_socket_constraint_radius(self)
	local result_0 = getPointer(self).Radius;
	return result_0;
end
function abi.ffi.property_setter_ball_socket_constraint_radius_set_radius(self, value)
	getPointer(self).Radius = value;
end
function abi.ffi.property_getter_ball_socket_constraint_restitution(self)
	local result_0 = getPointer(self).Restitution;
	return result_0;
end
function abi.ffi.property_setter_ball_socket_constraint_restitution_set_restitution(self, value)
	getPointer(self).Restitution = value;
end
function abi.ffi.property_getter_ball_socket_constraint_twist_limits_enabled(self)
	local result_0 = getPointer(self).TwistLimitsEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_ball_socket_constraint_twist_limits_enabled_set_twist_limits_enabled(self, value)
	getPointer(self).TwistLimitsEnabled = value == 1;
end
function abi.ffi.property_getter_ball_socket_constraint_twist_lower_angle(self)
	local result_0 = getPointer(self).TwistLowerAngle;
	return result_0;
end
function abi.ffi.property_setter_ball_socket_constraint_twist_lower_angle_set_twist_lower_angle(self, value)
	getPointer(self).TwistLowerAngle = value;
end
function abi.ffi.property_getter_ball_socket_constraint_twist_upper_angle(self)
	local result_0 = getPointer(self).TwistUpperAngle;
	return result_0;
end
function abi.ffi.property_setter_ball_socket_constraint_twist_upper_angle_set_twist_upper_angle(self, value)
	getPointer(self).TwistUpperAngle = value;
end
function abi.ffi.property_getter_ball_socket_constraint_upper_angle(self)
	local result_0 = getPointer(self).UpperAngle;
	return result_0;
end
function abi.ffi.property_setter_ball_socket_constraint_upper_angle_set_upper_angle(self, value)
	getPointer(self).UpperAngle = value;
end
function abi.ffi.property_getter_hinge_constraint_actuator_type(self)
	local result_0 = getPointer(self).ActuatorType;
	return result_0;
end
function abi.ffi.property_setter_hinge_constraint_actuator_type_set_actuator_type(self, value)
	getPointer(self).ActuatorType = value;
end
function abi.ffi.property_getter_hinge_constraint_angular_responsiveness(self)
	local result_0 = getPointer(self).AngularResponsiveness;
	return result_0;
end
function abi.ffi.property_setter_hinge_constraint_angular_responsiveness_set_angular_responsiveness(self, value)
	getPointer(self).AngularResponsiveness = value;
end
function abi.ffi.property_getter_hinge_constraint_angular_speed(self)
	local result_0 = getPointer(self).AngularSpeed;
	return result_0;
end
function abi.ffi.property_setter_hinge_constraint_angular_speed_set_angular_speed(self, value)
	getPointer(self).AngularSpeed = value;
end
function abi.ffi.property_getter_hinge_constraint_angular_velocity(self)
	local result_0 = getPointer(self).AngularVelocity;
	return result_0;
end
function abi.ffi.property_setter_hinge_constraint_angular_velocity_set_angular_velocity(self, value)
	getPointer(self).AngularVelocity = value;
end
function abi.ffi.property_getter_hinge_constraint_current_angle(self)
	local result_0 = getPointer(self).CurrentAngle;
	return result_0;
end
function abi.ffi.property_getter_hinge_constraint_limits_enabled(self)
	local result_0 = getPointer(self).LimitsEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_hinge_constraint_limits_enabled_set_limits_enabled(self, value)
	getPointer(self).LimitsEnabled = value == 1;
end
function abi.ffi.property_getter_hinge_constraint_lower_angle(self)
	local result_0 = getPointer(self).LowerAngle;
	return result_0;
end
function abi.ffi.property_setter_hinge_constraint_lower_angle_set_lower_angle(self, value)
	getPointer(self).LowerAngle = value;
end
function abi.ffi.property_getter_hinge_constraint_motor_max_acceleration(self)
	local result_0 = getPointer(self).MotorMaxAcceleration;
	return result_0;
end
function abi.ffi.property_setter_hinge_constraint_motor_max_acceleration_set_motor_max_acceleration(self, value)
	getPointer(self).MotorMaxAcceleration = value;
end
function abi.ffi.property_getter_hinge_constraint_motor_max_torque(self)
	local result_0 = getPointer(self).MotorMaxTorque;
	return result_0;
end
function abi.ffi.property_setter_hinge_constraint_motor_max_torque_set_motor_max_torque(self, value)
	getPointer(self).MotorMaxTorque = value;
end
function abi.ffi.property_getter_hinge_constraint_radius(self)
	local result_0 = getPointer(self).Radius;
	return result_0;
end
function abi.ffi.property_setter_hinge_constraint_radius_set_radius(self, value)
	getPointer(self).Radius = value;
end
function abi.ffi.property_getter_hinge_constraint_restitution(self)
	local result_0 = getPointer(self).Restitution;
	return result_0;
end
function abi.ffi.property_setter_hinge_constraint_restitution_set_restitution(self, value)
	getPointer(self).Restitution = value;
end
function abi.ffi.property_getter_hinge_constraint_servo_max_torque(self)
	local result_0 = getPointer(self).ServoMaxTorque;
	return result_0;
end
function abi.ffi.property_setter_hinge_constraint_servo_max_torque_set_servo_max_torque(self, value)
	getPointer(self).ServoMaxTorque = value;
end
function abi.ffi.property_getter_hinge_constraint_target_angle(self)
	local result_0 = getPointer(self).TargetAngle;
	return result_0;
end
function abi.ffi.property_setter_hinge_constraint_target_angle_set_target_angle(self, value)
	getPointer(self).TargetAngle = value;
end
function abi.ffi.property_getter_hinge_constraint_upper_angle(self)
	local result_0 = getPointer(self).UpperAngle;
	return result_0;
end
function abi.ffi.property_setter_hinge_constraint_upper_angle_set_upper_angle(self, value)
	getPointer(self).UpperAngle = value;
end
function abi.ffi.property_getter_line_force_apply_at_center_of_mass(self)
	local result_0 = getPointer(self).ApplyAtCenterOfMass;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_line_force_apply_at_center_of_mass_set_apply_at_center_of_mass(self, value)
	getPointer(self).ApplyAtCenterOfMass = value == 1;
end
function abi.ffi.property_getter_line_force_inverse_square_law(self)
	local result_0 = getPointer(self).InverseSquareLaw;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_line_force_inverse_square_law_set_inverse_square_law(self, value)
	getPointer(self).InverseSquareLaw = value == 1;
end
function abi.ffi.property_getter_line_force_magnitude(self)
	local result_0 = getPointer(self).Magnitude;
	return result_0;
end
function abi.ffi.property_setter_line_force_magnitude_set_magnitude(self, value)
	getPointer(self).Magnitude = value;
end
function abi.ffi.property_getter_line_force_max_force(self)
	local result_0 = getPointer(self).MaxForce;
	return result_0;
end
function abi.ffi.property_setter_line_force_max_force_set_max_force(self, value)
	getPointer(self).MaxForce = value;
end
function abi.ffi.property_getter_line_force_reaction_force_enabled(self)
	local result_0 = getPointer(self).ReactionForceEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_line_force_reaction_force_enabled_set_reaction_force_enabled(self, value)
	getPointer(self).ReactionForceEnabled = value == 1;
end
function abi.ffi.property_getter_linear_velocity_line_direction(self)
	local result_0 = getPointer(self).LineDirection;
	return createPointer(result_0);
end
function abi.ffi.property_setter_linear_velocity_line_direction_set_line_direction(self, value)
	getPointer(self).LineDirection = getPointer(value);
end
function abi.ffi.property_getter_linear_velocity_line_velocity(self)
	local result_0 = getPointer(self).LineVelocity;
	return result_0;
end
function abi.ffi.property_setter_linear_velocity_line_velocity_set_line_velocity(self, value)
	getPointer(self).LineVelocity = value;
end
function abi.ffi.property_getter_linear_velocity_max_force(self)
	local result_0 = getPointer(self).MaxForce;
	return result_0;
end
function abi.ffi.property_setter_linear_velocity_max_force_set_max_force(self, value)
	getPointer(self).MaxForce = value;
end
function abi.ffi.property_getter_linear_velocity_plane_velocity(self)
	local result_0 = getPointer(self).PlaneVelocity;
	return createPointer(result_0);
end
function abi.ffi.property_setter_linear_velocity_plane_velocity_set_plane_velocity(self, value)
	getPointer(self).PlaneVelocity = getPointer(value);
end
function abi.ffi.property_getter_linear_velocity_primary_tangent_axis(self)
	local result_0 = getPointer(self).PrimaryTangentAxis;
	return createPointer(result_0);
end
function abi.ffi.property_setter_linear_velocity_primary_tangent_axis_set_primary_tangent_axis(self, value)
	getPointer(self).PrimaryTangentAxis = getPointer(value);
end
function abi.ffi.property_getter_linear_velocity_relative_to(self)
	local result_0 = getPointer(self).RelativeTo;
	return result_0;
end
function abi.ffi.property_setter_linear_velocity_relative_to_set_relative_to(self, value)
	getPointer(self).RelativeTo = value;
end
function abi.ffi.property_getter_linear_velocity_secondary_tangent_axis(self)
	local result_0 = getPointer(self).SecondaryTangentAxis;
	return createPointer(result_0);
end
function abi.ffi.property_setter_linear_velocity_secondary_tangent_axis_set_secondary_tangent_axis(self, value)
	getPointer(self).SecondaryTangentAxis = getPointer(value);
end
function abi.ffi.property_getter_linear_velocity_vector_velocity(self)
	local result_0 = getPointer(self).VectorVelocity;
	return createPointer(result_0);
end
function abi.ffi.property_setter_linear_velocity_vector_velocity_set_vector_velocity(self, value)
	getPointer(self).VectorVelocity = getPointer(value);
end
function abi.ffi.property_getter_linear_velocity_velocity_constraint_mode(self)
	local result_0 = getPointer(self).VelocityConstraintMode;
	return result_0;
end
function abi.ffi.property_setter_linear_velocity_velocity_constraint_mode_set_velocity_constraint_mode(self, value)
	getPointer(self).VelocityConstraintMode = value;
end
function abi.ffi.property_getter_rod_constraint_current_distance(self)
	local result_0 = getPointer(self).CurrentDistance;
	return result_0;
end
function abi.ffi.property_getter_rod_constraint_length(self)
	local result_0 = getPointer(self).Length;
	return result_0;
end
function abi.ffi.property_setter_rod_constraint_length_set_length(self, value)
	getPointer(self).Length = value;
end
function abi.ffi.property_getter_rod_constraint_limit_angle_0(self)
	local result_0 = getPointer(self).LimitAngle0;
	return result_0;
end
function abi.ffi.property_setter_rod_constraint_limit_angle_0_set_limit_angle_0(self, value)
	getPointer(self).LimitAngle0 = value;
end
function abi.ffi.property_getter_rod_constraint_limit_angle_1(self)
	local result_0 = getPointer(self).LimitAngle1;
	return result_0;
end
function abi.ffi.property_setter_rod_constraint_limit_angle_1_set_limit_angle_1(self, value)
	getPointer(self).LimitAngle1 = value;
end
function abi.ffi.property_getter_rod_constraint_limits_enabled(self)
	local result_0 = getPointer(self).LimitsEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_rod_constraint_limits_enabled_set_limits_enabled(self, value)
	getPointer(self).LimitsEnabled = value == 1;
end
function abi.ffi.property_getter_rod_constraint_thickness(self)
	local result_0 = getPointer(self).Thickness;
	return result_0;
end
function abi.ffi.property_setter_rod_constraint_thickness_set_thickness(self, value)
	getPointer(self).Thickness = value;
end
function abi.ffi.property_getter_rope_constraint_current_distance(self)
	local result_0 = getPointer(self).CurrentDistance;
	return result_0;
end
function abi.ffi.property_getter_rope_constraint_length(self)
	local result_0 = getPointer(self).Length;
	return result_0;
end
function abi.ffi.property_setter_rope_constraint_length_set_length(self, value)
	getPointer(self).Length = value;
end
function abi.ffi.property_getter_rope_constraint_restitution(self)
	local result_0 = getPointer(self).Restitution;
	return result_0;
end
function abi.ffi.property_setter_rope_constraint_restitution_set_restitution(self, value)
	getPointer(self).Restitution = value;
end
function abi.ffi.property_getter_rope_constraint_thickness(self)
	local result_0 = getPointer(self).Thickness;
	return result_0;
end
function abi.ffi.property_setter_rope_constraint_thickness_set_thickness(self, value)
	getPointer(self).Thickness = value;
end
function abi.ffi.property_getter_rope_constraint_winch_enabled(self)
	local result_0 = getPointer(self).WinchEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_rope_constraint_winch_enabled_set_winch_enabled(self, value)
	getPointer(self).WinchEnabled = value == 1;
end
function abi.ffi.property_getter_rope_constraint_winch_force(self)
	local result_0 = getPointer(self).WinchForce;
	return result_0;
end
function abi.ffi.property_setter_rope_constraint_winch_force_set_winch_force(self, value)
	getPointer(self).WinchForce = value;
end
function abi.ffi.property_getter_rope_constraint_winch_responsiveness(self)
	local result_0 = getPointer(self).WinchResponsiveness;
	return result_0;
end
function abi.ffi.property_setter_rope_constraint_winch_responsiveness_set_winch_responsiveness(self, value)
	getPointer(self).WinchResponsiveness = value;
end
function abi.ffi.property_getter_rope_constraint_winch_speed(self)
	local result_0 = getPointer(self).WinchSpeed;
	return result_0;
end
function abi.ffi.property_setter_rope_constraint_winch_speed_set_winch_speed(self, value)
	getPointer(self).WinchSpeed = value;
end
function abi.ffi.property_getter_rope_constraint_winch_target(self)
	local result_0 = getPointer(self).WinchTarget;
	return result_0;
end
function abi.ffi.property_setter_rope_constraint_winch_target_set_winch_target(self, value)
	getPointer(self).WinchTarget = value;
end
function abi.ffi.property_getter_sliding_ball_constraint_actuator_type(self)
	local result_0 = getPointer(self).ActuatorType;
	return result_0;
end
function abi.ffi.property_setter_sliding_ball_constraint_actuator_type_set_actuator_type(self, value)
	getPointer(self).ActuatorType = value;
end
function abi.ffi.property_getter_sliding_ball_constraint_current_position(self)
	local result_0 = getPointer(self).CurrentPosition;
	return result_0;
end
function abi.ffi.property_getter_sliding_ball_constraint_limits_enabled(self)
	local result_0 = getPointer(self).LimitsEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_sliding_ball_constraint_limits_enabled_set_limits_enabled(self, value)
	getPointer(self).LimitsEnabled = value == 1;
end
function abi.ffi.property_getter_sliding_ball_constraint_linear_responsiveness(self)
	local result_0 = getPointer(self).LinearResponsiveness;
	return result_0;
end
function abi.ffi.property_setter_sliding_ball_constraint_linear_responsiveness_set_linear_responsiveness(self, value)
	getPointer(self).LinearResponsiveness = value;
end
function abi.ffi.property_getter_sliding_ball_constraint_lower_limit(self)
	local result_0 = getPointer(self).LowerLimit;
	return result_0;
end
function abi.ffi.property_setter_sliding_ball_constraint_lower_limit_set_lower_limit(self, value)
	getPointer(self).LowerLimit = value;
end
function abi.ffi.property_getter_sliding_ball_constraint_motor_max_acceleration(self)
	local result_0 = getPointer(self).MotorMaxAcceleration;
	return result_0;
end
function abi.ffi.property_setter_sliding_ball_constraint_motor_max_acceleration_set_motor_max_acceleration(self, value)
	getPointer(self).MotorMaxAcceleration = value;
end
function abi.ffi.property_getter_sliding_ball_constraint_motor_max_force(self)
	local result_0 = getPointer(self).MotorMaxForce;
	return result_0;
end
function abi.ffi.property_setter_sliding_ball_constraint_motor_max_force_set_motor_max_force(self, value)
	getPointer(self).MotorMaxForce = value;
end
function abi.ffi.property_getter_sliding_ball_constraint_restitution(self)
	local result_0 = getPointer(self).Restitution;
	return result_0;
end
function abi.ffi.property_setter_sliding_ball_constraint_restitution_set_restitution(self, value)
	getPointer(self).Restitution = value;
end
function abi.ffi.property_getter_sliding_ball_constraint_servo_max_force(self)
	local result_0 = getPointer(self).ServoMaxForce;
	return result_0;
end
function abi.ffi.property_setter_sliding_ball_constraint_servo_max_force_set_servo_max_force(self, value)
	getPointer(self).ServoMaxForce = value;
end
function abi.ffi.property_getter_sliding_ball_constraint_size(self)
	local result_0 = getPointer(self).Size;
	return result_0;
end
function abi.ffi.property_setter_sliding_ball_constraint_size_set_size(self, value)
	getPointer(self).Size = value;
end
function abi.ffi.property_getter_sliding_ball_constraint_speed(self)
	local result_0 = getPointer(self).Speed;
	return result_0;
end
function abi.ffi.property_setter_sliding_ball_constraint_speed_set_speed(self, value)
	getPointer(self).Speed = value;
end
function abi.ffi.property_getter_sliding_ball_constraint_target_position(self)
	local result_0 = getPointer(self).TargetPosition;
	return result_0;
end
function abi.ffi.property_setter_sliding_ball_constraint_target_position_set_target_position(self, value)
	getPointer(self).TargetPosition = value;
end
function abi.ffi.property_getter_sliding_ball_constraint_upper_limit(self)
	local result_0 = getPointer(self).UpperLimit;
	return result_0;
end
function abi.ffi.property_setter_sliding_ball_constraint_upper_limit_set_upper_limit(self, value)
	getPointer(self).UpperLimit = value;
end
function abi.ffi.property_getter_sliding_ball_constraint_velocity(self)
	local result_0 = getPointer(self).Velocity;
	return result_0;
end
function abi.ffi.property_setter_sliding_ball_constraint_velocity_set_velocity(self, value)
	getPointer(self).Velocity = value;
end
function abi.ffi.property_getter_cylindrical_constraint_angular_actuator_type(self)
	local result_0 = getPointer(self).AngularActuatorType;
	return result_0;
end
function abi.ffi.property_setter_cylindrical_constraint_angular_actuator_type_set_angular_actuator_type(self, value)
	getPointer(self).AngularActuatorType = value;
end
function abi.ffi.property_getter_cylindrical_constraint_angular_limits_enabled(self)
	local result_0 = getPointer(self).AngularLimitsEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_cylindrical_constraint_angular_limits_enabled_set_angular_limits_enabled(self, value)
	getPointer(self).AngularLimitsEnabled = value == 1;
end
function abi.ffi.property_getter_cylindrical_constraint_angular_responsiveness(self)
	local result_0 = getPointer(self).AngularResponsiveness;
	return result_0;
end
function abi.ffi.property_setter_cylindrical_constraint_angular_responsiveness_set_angular_responsiveness(self, value)
	getPointer(self).AngularResponsiveness = value;
end
function abi.ffi.property_getter_cylindrical_constraint_angular_restitution(self)
	local result_0 = getPointer(self).AngularRestitution;
	return result_0;
end
function abi.ffi.property_setter_cylindrical_constraint_angular_restitution_set_angular_restitution(self, value)
	getPointer(self).AngularRestitution = value;
end
function abi.ffi.property_getter_cylindrical_constraint_angular_speed(self)
	local result_0 = getPointer(self).AngularSpeed;
	return result_0;
end
function abi.ffi.property_setter_cylindrical_constraint_angular_speed_set_angular_speed(self, value)
	getPointer(self).AngularSpeed = value;
end
function abi.ffi.property_getter_cylindrical_constraint_angular_velocity(self)
	local result_0 = getPointer(self).AngularVelocity;
	return result_0;
end
function abi.ffi.property_setter_cylindrical_constraint_angular_velocity_set_angular_velocity(self, value)
	getPointer(self).AngularVelocity = value;
end
function abi.ffi.property_getter_cylindrical_constraint_current_angle(self)
	local result_0 = getPointer(self).CurrentAngle;
	return result_0;
end
function abi.ffi.property_getter_cylindrical_constraint_inclination_angle(self)
	local result_0 = getPointer(self).InclinationAngle;
	return result_0;
end
function abi.ffi.property_setter_cylindrical_constraint_inclination_angle_set_inclination_angle(self, value)
	getPointer(self).InclinationAngle = value;
end
function abi.ffi.property_getter_cylindrical_constraint_lower_angle(self)
	local result_0 = getPointer(self).LowerAngle;
	return result_0;
end
function abi.ffi.property_setter_cylindrical_constraint_lower_angle_set_lower_angle(self, value)
	getPointer(self).LowerAngle = value;
end
function abi.ffi.property_getter_cylindrical_constraint_motor_max_angular_acceleration(self)
	local result_0 = getPointer(self).MotorMaxAngularAcceleration;
	return result_0;
end
function abi.ffi.property_setter_cylindrical_constraint_motor_max_angular_acceleration_set_motor_max_angular_acceleration(self, value)
	getPointer(self).MotorMaxAngularAcceleration = value;
end
function abi.ffi.property_getter_cylindrical_constraint_motor_max_torque(self)
	local result_0 = getPointer(self).MotorMaxTorque;
	return result_0;
end
function abi.ffi.property_setter_cylindrical_constraint_motor_max_torque_set_motor_max_torque(self, value)
	getPointer(self).MotorMaxTorque = value;
end
function abi.ffi.property_getter_cylindrical_constraint_rotation_axis_visible(self)
	local result_0 = getPointer(self).RotationAxisVisible;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_cylindrical_constraint_rotation_axis_visible_set_rotation_axis_visible(self, value)
	getPointer(self).RotationAxisVisible = value == 1;
end
function abi.ffi.property_getter_cylindrical_constraint_servo_max_torque(self)
	local result_0 = getPointer(self).ServoMaxTorque;
	return result_0;
end
function abi.ffi.property_setter_cylindrical_constraint_servo_max_torque_set_servo_max_torque(self, value)
	getPointer(self).ServoMaxTorque = value;
end
function abi.ffi.property_getter_cylindrical_constraint_target_angle(self)
	local result_0 = getPointer(self).TargetAngle;
	return result_0;
end
function abi.ffi.property_setter_cylindrical_constraint_target_angle_set_target_angle(self, value)
	getPointer(self).TargetAngle = value;
end
function abi.ffi.property_getter_cylindrical_constraint_upper_angle(self)
	local result_0 = getPointer(self).UpperAngle;
	return result_0;
end
function abi.ffi.property_setter_cylindrical_constraint_upper_angle_set_upper_angle(self, value)
	getPointer(self).UpperAngle = value;
end
function abi.ffi.property_getter_cylindrical_constraint_world_rotation_axis(self)
	local result_0 = getPointer(self).WorldRotationAxis;
	return createPointer(result_0);
end
function abi.ffi.property_getter_spring_constraint_coils(self)
	local result_0 = getPointer(self).Coils;
	return result_0;
end
function abi.ffi.property_setter_spring_constraint_coils_set_coils(self, value)
	getPointer(self).Coils = value;
end
function abi.ffi.property_getter_spring_constraint_current_length(self)
	local result_0 = getPointer(self).CurrentLength;
	return result_0;
end
function abi.ffi.property_getter_spring_constraint_damping(self)
	local result_0 = getPointer(self).Damping;
	return result_0;
end
function abi.ffi.property_setter_spring_constraint_damping_set_damping(self, value)
	getPointer(self).Damping = value;
end
function abi.ffi.property_getter_spring_constraint_free_length(self)
	local result_0 = getPointer(self).FreeLength;
	return result_0;
end
function abi.ffi.property_setter_spring_constraint_free_length_set_free_length(self, value)
	getPointer(self).FreeLength = value;
end
function abi.ffi.property_getter_spring_constraint_limits_enabled(self)
	local result_0 = getPointer(self).LimitsEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_spring_constraint_limits_enabled_set_limits_enabled(self, value)
	getPointer(self).LimitsEnabled = value == 1;
end
function abi.ffi.property_getter_spring_constraint_max_force(self)
	local result_0 = getPointer(self).MaxForce;
	return result_0;
end
function abi.ffi.property_setter_spring_constraint_max_force_set_max_force(self, value)
	getPointer(self).MaxForce = value;
end
function abi.ffi.property_getter_spring_constraint_max_length(self)
	local result_0 = getPointer(self).MaxLength;
	return result_0;
end
function abi.ffi.property_setter_spring_constraint_max_length_set_max_length(self, value)
	getPointer(self).MaxLength = value;
end
function abi.ffi.property_getter_spring_constraint_min_length(self)
	local result_0 = getPointer(self).MinLength;
	return result_0;
end
function abi.ffi.property_setter_spring_constraint_min_length_set_min_length(self, value)
	getPointer(self).MinLength = value;
end
function abi.ffi.property_getter_spring_constraint_radius(self)
	local result_0 = getPointer(self).Radius;
	return result_0;
end
function abi.ffi.property_setter_spring_constraint_radius_set_radius(self, value)
	getPointer(self).Radius = value;
end
function abi.ffi.property_getter_spring_constraint_stiffness(self)
	local result_0 = getPointer(self).Stiffness;
	return result_0;
end
function abi.ffi.property_setter_spring_constraint_stiffness_set_stiffness(self, value)
	getPointer(self).Stiffness = value;
end
function abi.ffi.property_getter_spring_constraint_thickness(self)
	local result_0 = getPointer(self).Thickness;
	return result_0;
end
function abi.ffi.property_setter_spring_constraint_thickness_set_thickness(self, value)
	getPointer(self).Thickness = value;
end
function abi.ffi.property_getter_torque_relative_to(self)
	local result_0 = getPointer(self).RelativeTo;
	return result_0;
end
function abi.ffi.property_setter_torque_relative_to_set_relative_to(self, value)
	getPointer(self).RelativeTo = value;
end
function abi.ffi.property_getter_torque_torque(self)
	local result_0 = getPointer(self).Torque;
	return createPointer(result_0);
end
function abi.ffi.property_setter_torque_torque_set_torque(self, value)
	getPointer(self).Torque = getPointer(value);
end
function abi.ffi.property_getter_torsion_spring_constraint_coils(self)
	local result_0 = getPointer(self).Coils;
	return result_0;
end
function abi.ffi.property_setter_torsion_spring_constraint_coils_set_coils(self, value)
	getPointer(self).Coils = value;
end
function abi.ffi.property_getter_torsion_spring_constraint_current_angle(self)
	local result_0 = getPointer(self).CurrentAngle;
	return result_0;
end
function abi.ffi.property_getter_torsion_spring_constraint_damping(self)
	local result_0 = getPointer(self).Damping;
	return result_0;
end
function abi.ffi.property_setter_torsion_spring_constraint_damping_set_damping(self, value)
	getPointer(self).Damping = value;
end
function abi.ffi.property_getter_torsion_spring_constraint_limits_enabled(self)
	local result_0 = getPointer(self).LimitsEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_torsion_spring_constraint_limits_enabled_set_limits_enabled(self, value)
	getPointer(self).LimitsEnabled = value == 1;
end
function abi.ffi.property_getter_torsion_spring_constraint_max_angle(self)
	local result_0 = getPointer(self).MaxAngle;
	return result_0;
end
function abi.ffi.property_setter_torsion_spring_constraint_max_angle_set_max_angle(self, value)
	getPointer(self).MaxAngle = value;
end
function abi.ffi.property_getter_torsion_spring_constraint_max_torque(self)
	local result_0 = getPointer(self).MaxTorque;
	return result_0;
end
function abi.ffi.property_setter_torsion_spring_constraint_max_torque_set_max_torque(self, value)
	getPointer(self).MaxTorque = value;
end
function abi.ffi.property_getter_torsion_spring_constraint_radius(self)
	local result_0 = getPointer(self).Radius;
	return result_0;
end
function abi.ffi.property_setter_torsion_spring_constraint_radius_set_radius(self, value)
	getPointer(self).Radius = value;
end
function abi.ffi.property_getter_torsion_spring_constraint_restitution(self)
	local result_0 = getPointer(self).Restitution;
	return result_0;
end
function abi.ffi.property_setter_torsion_spring_constraint_restitution_set_restitution(self, value)
	getPointer(self).Restitution = value;
end
function abi.ffi.property_getter_torsion_spring_constraint_stiffness(self)
	local result_0 = getPointer(self).Stiffness;
	return result_0;
end
function abi.ffi.property_setter_torsion_spring_constraint_stiffness_set_stiffness(self, value)
	getPointer(self).Stiffness = value;
end
function abi.ffi.property_getter_universal_constraint_limits_enabled(self)
	local result_0 = getPointer(self).LimitsEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_universal_constraint_limits_enabled_set_limits_enabled(self, value)
	getPointer(self).LimitsEnabled = value == 1;
end
function abi.ffi.property_getter_universal_constraint_max_angle(self)
	local result_0 = getPointer(self).MaxAngle;
	return result_0;
end
function abi.ffi.property_setter_universal_constraint_max_angle_set_max_angle(self, value)
	getPointer(self).MaxAngle = value;
end
function abi.ffi.property_getter_universal_constraint_radius(self)
	local result_0 = getPointer(self).Radius;
	return result_0;
end
function abi.ffi.property_setter_universal_constraint_radius_set_radius(self, value)
	getPointer(self).Radius = value;
end
function abi.ffi.property_getter_universal_constraint_restitution(self)
	local result_0 = getPointer(self).Restitution;
	return result_0;
end
function abi.ffi.property_setter_universal_constraint_restitution_set_restitution(self, value)
	getPointer(self).Restitution = value;
end
function abi.ffi.property_getter_vector_force_apply_at_center_of_mass(self)
	local result_0 = getPointer(self).ApplyAtCenterOfMass;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_vector_force_apply_at_center_of_mass_set_apply_at_center_of_mass(self, value)
	getPointer(self).ApplyAtCenterOfMass = value == 1;
end
function abi.ffi.property_getter_vector_force_force(self)
	local result_0 = getPointer(self).Force;
	return createPointer(result_0);
end
function abi.ffi.property_setter_vector_force_force_set_force(self, value)
	getPointer(self).Force = getPointer(value);
end
function abi.ffi.property_getter_vector_force_relative_to(self)
	local result_0 = getPointer(self).RelativeTo;
	return result_0;
end
function abi.ffi.property_setter_vector_force_relative_to_set_relative_to(self, value)
	getPointer(self).RelativeTo = value;
end
function abi.ffi.property_getter_content_provider_base_url(self)
	local result_0 = getPointer(self).BaseUrl;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_content_provider_request_queue_size(self)
	local result_0 = getPointer(self).RequestQueueSize;
	return result_0;
end
function abi.ffi.method_content_provider_register_default_encryption_key(self, encryption_key_addr, encryption_key_len)
	getPointer(self):RegisterDefaultEncryptionKey(loadString(memory, encryption_key_addr, encryption_key_len))
end
function abi.ffi.method_content_provider_register_default_session_key(self, session_key_addr, session_key_len)
	getPointer(self):RegisterDefaultSessionKey(loadString(memory, session_key_addr, session_key_len))
end
function abi.ffi.method_content_provider_register_encrypted_asset(self, asset_id, encryption_key_addr, encryption_key_len)
	getPointer(self):RegisterEncryptedAsset(getPointer(asset_id), loadString(memory, encryption_key_addr, encryption_key_len))
end
function abi.ffi.method_content_provider_register_session_encrypted_asset(self, content_id, session_key_addr, session_key_len)
	getPointer(self):RegisterSessionEncryptedAsset(getPointer(content_id), loadString(memory, session_key_addr, session_key_len))
end
function abi.ffi.method_content_provider_unregister_default_encryption_key(self)
	getPointer(self):UnregisterDefaultEncryptionKey()
end
function abi.ffi.method_content_provider_unregister_encrypted_asset(self, asset_id)
	getPointer(self):UnregisterEncryptedAsset(getPointer(asset_id))
end
function abi.ffi.event_content_provider_asset_fetch_failed_on_asset_fetch_failed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).AssetFetchFailed:Connect(function(asset_id)
		invokeFunction(callback_data, callback_vtable, createPointer(asset_id))
	end));
	return createPointer(result_0);
end
function abi.ffi.method_context_action_service_get_current_local_tool_icon(self)
	local result_0 = getPointer(self):GetCurrentLocalToolIcon();
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_context_action_service_set_description(self, action_name_addr, action_name_len, description_addr, description_len)
	getPointer(self):SetDescription(loadString(memory, action_name_addr, action_name_len), loadString(memory, description_addr, description_len))
end
function abi.ffi.method_context_action_service_set_image(self, action_name_addr, action_name_len, image_addr, image_len)
	getPointer(self):SetImage(loadString(memory, action_name_addr, action_name_len), loadString(memory, image_addr, image_len))
end
function abi.ffi.method_context_action_service_set_position(self, action_name_addr, action_name_len, position)
	getPointer(self):SetPosition(loadString(memory, action_name_addr, action_name_len), getPointer(position))
end
function abi.ffi.method_context_action_service_set_title(self, action_name_addr, action_name_len, title_addr, title_len)
	getPointer(self):SetTitle(loadString(memory, action_name_addr, action_name_len), loadString(memory, title_addr, title_len))
end
function abi.ffi.method_context_action_service_unbind_action(self, action_name_addr, action_name_len)
	getPointer(self):UnbindAction(loadString(memory, action_name_addr, action_name_len))
end
function abi.ffi.method_context_action_service_unbind_activate(self, user_input_type_for_activation, key_code_for_activation)
	getPointer(self):UnbindActivate(user_input_type_for_activation, key_code_for_activation)
end
function abi.ffi.method_context_action_service_unbind_all_actions(self)
	getPointer(self):UnbindAllActions()
end
function abi.ffi.method_context_action_service_get_button(self, action_name_addr, action_name_len)
	local result_0 = getPointer(self):GetButton(loadString(memory, action_name_addr, action_name_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.event_context_action_service_local_tool_equipped_on_local_tool_equipped(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).LocalToolEquipped:Connect(function(tool_equipped)
		local tool_equipped_optional, tool_equipped_optional_0 = 0, 0;
		if (tool_equipped ~= nil) then
			tool_equipped_optional, tool_equipped_optional_0 = 1, createPointer(tool_equipped);
		end
		invokeFunction(callback_data, callback_vtable, tool_equipped_optional, tool_equipped_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_context_action_service_local_tool_unequipped_on_local_tool_unequipped(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).LocalToolUnequipped:Connect(function(tool_unequipped)
		local tool_unequipped_optional, tool_unequipped_optional_0 = 0, 0;
		if (tool_unequipped ~= nil) then
			tool_unequipped_optional, tool_unequipped_optional_0 = 1, createPointer(tool_unequipped);
		end
		invokeFunction(callback_data, callback_vtable, tool_unequipped_optional, tool_unequipped_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.method_controller_bind_button(self, button, caption_addr, caption_len)
	getPointer(self):BindButton(button, loadString(memory, caption_addr, caption_len))
end
function abi.ffi.method_controller_get_button(self, button)
	local result_0 = getPointer(self):GetButton(button);
	return result_0 and 1 or 0;
end
function abi.ffi.method_controller_unbind_button(self, button)
	getPointer(self):UnbindButton(button)
end
function abi.ffi.event_controller_button_changed_on_button_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).ButtonChanged:Connect(function(button)
		invokeFunction(callback_data, callback_vtable, button)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_skateboard_controller_steer(self)
	local result_0 = getPointer(self).Steer;
	return result_0;
end
function abi.ffi.property_getter_skateboard_controller_throttle(self)
	local result_0 = getPointer(self).Throttle;
	return result_0;
end
function abi.ffi.event_skateboard_controller_axis_changed_on_axis_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).AxisChanged:Connect(function(axis)
		local axis_len = #axis;
		local axis_content = allocString(axis_len);
		for i = 1, axis_len do
			storeU8(memory, axis_content + (i - 1), string.byte(axis:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, axis_content, axis_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_controller_base_move_speed_factor(self)
	local result_0 = getPointer(self).MoveSpeedFactor;
	return result_0;
end
function abi.ffi.property_setter_controller_base_move_speed_factor_set_move_speed_factor(self, value)
	getPointer(self).MoveSpeedFactor = value;
end
function abi.ffi.property_getter_controller_base_rigidity_enabled(self)
	local result_0 = getPointer(self).RigidityEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_controller_base_rigidity_enabled_set_rigidity_enabled(self, value)
	getPointer(self).RigidityEnabled = value == 1;
end
function abi.ffi.property_getter_air_controller_cancel_air_momentum(self)
	local result_0 = getPointer(self).CancelAirMomentum;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_air_controller_cancel_air_momentum_set_cancel_air_momentum(self, value)
	getPointer(self).CancelAirMomentum = value == 1;
end
function abi.ffi.property_getter_air_controller_move_max_force(self)
	local result_0 = getPointer(self).MoveMaxForce;
	return result_0;
end
function abi.ffi.property_setter_air_controller_move_max_force_set_move_max_force(self, value)
	getPointer(self).MoveMaxForce = value;
end
function abi.ffi.property_getter_air_controller_orientation_max_torque(self)
	local result_0 = getPointer(self).OrientationMaxTorque;
	return result_0;
end
function abi.ffi.property_setter_air_controller_orientation_max_torque_set_orientation_max_torque(self, value)
	getPointer(self).OrientationMaxTorque = value;
end
function abi.ffi.property_getter_air_controller_orientation_speed_factor(self)
	local result_0 = getPointer(self).OrientationSpeedFactor;
	return result_0;
end
function abi.ffi.property_setter_air_controller_orientation_speed_factor_set_orientation_speed_factor(self, value)
	getPointer(self).OrientationSpeedFactor = value;
end
function abi.ffi.property_getter_air_controller_vector_force(self)
	local result_0 = getPointer(self).VectorForce;
	return createPointer(result_0);
end
function abi.ffi.property_setter_air_controller_vector_force_set_vector_force(self, value)
	getPointer(self).VectorForce = getPointer(value);
end
function abi.ffi.property_getter_climb_controller_acceleration_time(self)
	local result_0 = getPointer(self).AccelerationTime;
	return result_0;
end
function abi.ffi.property_setter_climb_controller_acceleration_time_set_acceleration_time(self, value)
	getPointer(self).AccelerationTime = value;
end
function abi.ffi.property_getter_climb_controller_move_max_force(self)
	local result_0 = getPointer(self).MoveMaxForce;
	return result_0;
end
function abi.ffi.property_setter_climb_controller_move_max_force_set_move_max_force(self, value)
	getPointer(self).MoveMaxForce = value;
end
function abi.ffi.property_getter_climb_controller_orientation_max_torque(self)
	local result_0 = getPointer(self).OrientationMaxTorque;
	return result_0;
end
function abi.ffi.property_setter_climb_controller_orientation_max_torque_set_orientation_max_torque(self, value)
	getPointer(self).OrientationMaxTorque = value;
end
function abi.ffi.property_getter_climb_controller_orientation_speed_factor(self)
	local result_0 = getPointer(self).OrientationSpeedFactor;
	return result_0;
end
function abi.ffi.property_setter_climb_controller_orientation_speed_factor_set_orientation_speed_factor(self, value)
	getPointer(self).OrientationSpeedFactor = value;
end
function abi.ffi.property_getter_ground_controller_acceleration_lean(self)
	local result_0 = getPointer(self).AccelerationLean;
	return result_0;
end
function abi.ffi.property_setter_ground_controller_acceleration_lean_set_acceleration_lean(self, value)
	getPointer(self).AccelerationLean = value;
end
function abi.ffi.property_getter_ground_controller_acceleration_time(self)
	local result_0 = getPointer(self).AccelerationTime;
	return result_0;
end
function abi.ffi.property_setter_ground_controller_acceleration_time_set_acceleration_time(self, value)
	getPointer(self).AccelerationTime = value;
end
function abi.ffi.property_getter_ground_controller_align_speed(self)
	local result_0 = getPointer(self).AlignSpeed;
	return result_0;
end
function abi.ffi.property_setter_ground_controller_align_speed_set_align_speed(self, value)
	getPointer(self).AlignSpeed = value;
end
function abi.ffi.property_getter_ground_controller_align_torque(self)
	local result_0 = getPointer(self).AlignTorque;
	return result_0;
end
function abi.ffi.property_setter_ground_controller_align_torque_set_align_torque(self, value)
	getPointer(self).AlignTorque = value;
end
function abi.ffi.property_getter_ground_controller_deceleration_time(self)
	local result_0 = getPointer(self).DecelerationTime;
	return result_0;
end
function abi.ffi.property_setter_ground_controller_deceleration_time_set_deceleration_time(self, value)
	getPointer(self).DecelerationTime = value;
end
function abi.ffi.property_getter_ground_controller_friction(self)
	local result_0 = getPointer(self).Friction;
	return result_0;
end
function abi.ffi.property_setter_ground_controller_friction_set_friction(self, value)
	getPointer(self).Friction = value;
end
function abi.ffi.property_getter_ground_controller_friction_weight(self)
	local result_0 = getPointer(self).FrictionWeight;
	return result_0;
end
function abi.ffi.property_setter_ground_controller_friction_weight_set_friction_weight(self, value)
	getPointer(self).FrictionWeight = value;
end
function abi.ffi.property_getter_ground_controller_ground_offset(self)
	local result_0 = getPointer(self).GroundOffset;
	return result_0;
end
function abi.ffi.property_setter_ground_controller_ground_offset_set_ground_offset(self, value)
	getPointer(self).GroundOffset = value;
end
function abi.ffi.property_getter_ground_controller_max_slope_angle(self)
	local result_0 = getPointer(self).MaxSlopeAngle;
	return result_0;
end
function abi.ffi.property_setter_ground_controller_max_slope_angle_set_max_slope_angle(self, value)
	getPointer(self).MaxSlopeAngle = value;
end
function abi.ffi.property_getter_ground_controller_stand_force(self)
	local result_0 = getPointer(self).StandForce;
	return result_0;
end
function abi.ffi.property_setter_ground_controller_stand_force_set_stand_force(self, value)
	getPointer(self).StandForce = value;
end
function abi.ffi.property_getter_ground_controller_stand_speed(self)
	local result_0 = getPointer(self).StandSpeed;
	return result_0;
end
function abi.ffi.property_setter_ground_controller_stand_speed_set_stand_speed(self, value)
	getPointer(self).StandSpeed = value;
end
function abi.ffi.property_getter_ground_controller_turning_factor(self)
	local result_0 = getPointer(self).TurningFactor;
	return result_0;
end
function abi.ffi.property_setter_ground_controller_turning_factor_set_turning_factor(self, value)
	getPointer(self).TurningFactor = value;
end
function abi.ffi.property_getter_swim_controller_acceleration_time(self)
	local result_0 = getPointer(self).AccelerationTime;
	return result_0;
end
function abi.ffi.property_setter_swim_controller_acceleration_time_set_acceleration_time(self, value)
	getPointer(self).AccelerationTime = value;
end
function abi.ffi.property_getter_swim_controller_pitch_max_torque(self)
	local result_0 = getPointer(self).PitchMaxTorque;
	return result_0;
end
function abi.ffi.property_setter_swim_controller_pitch_max_torque_set_pitch_max_torque(self, value)
	getPointer(self).PitchMaxTorque = value;
end
function abi.ffi.property_getter_swim_controller_pitch_speed_factor(self)
	local result_0 = getPointer(self).PitchSpeedFactor;
	return result_0;
end
function abi.ffi.property_setter_swim_controller_pitch_speed_factor_set_pitch_speed_factor(self, value)
	getPointer(self).PitchSpeedFactor = value;
end
function abi.ffi.property_getter_swim_controller_roll_max_torque(self)
	local result_0 = getPointer(self).RollMaxTorque;
	return result_0;
end
function abi.ffi.property_setter_swim_controller_roll_max_torque_set_roll_max_torque(self, value)
	getPointer(self).RollMaxTorque = value;
end
function abi.ffi.property_getter_swim_controller_roll_speed_factor(self)
	local result_0 = getPointer(self).RollSpeedFactor;
	return result_0;
end
function abi.ffi.property_setter_swim_controller_roll_speed_factor_set_roll_speed_factor(self, value)
	getPointer(self).RollSpeedFactor = value;
end
function abi.ffi.property_getter_controller_manager_active_controller(self)
	local result_0 = getPointer(self).ActiveController;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_controller_manager_base_move_speed(self)
	local result_0 = getPointer(self).BaseMoveSpeed;
	return result_0;
end
function abi.ffi.property_setter_controller_manager_base_move_speed_set_base_move_speed(self, value)
	getPointer(self).BaseMoveSpeed = value;
end
function abi.ffi.property_getter_controller_manager_base_turn_speed(self)
	local result_0 = getPointer(self).BaseTurnSpeed;
	return result_0;
end
function abi.ffi.property_setter_controller_manager_base_turn_speed_set_base_turn_speed(self, value)
	getPointer(self).BaseTurnSpeed = value;
end
function abi.ffi.property_getter_controller_manager_facing_direction(self)
	local result_0 = getPointer(self).FacingDirection;
	return createPointer(result_0);
end
function abi.ffi.property_setter_controller_manager_facing_direction_set_facing_direction(self, value)
	getPointer(self).FacingDirection = getPointer(value);
end
function abi.ffi.property_getter_controller_manager_hip_height(self)
	local result_0 = getPointer(self).HipHeight;
	return result_0;
end
function abi.ffi.property_setter_controller_manager_hip_height_set_hip_height(self, value)
	getPointer(self).HipHeight = value;
end
function abi.ffi.property_getter_controller_manager_moving_direction(self)
	local result_0 = getPointer(self).MovingDirection;
	return createPointer(result_0);
end
function abi.ffi.property_setter_controller_manager_moving_direction_set_moving_direction(self, value)
	getPointer(self).MovingDirection = getPointer(value);
end
function abi.ffi.property_getter_data_model_mesh_offset(self)
	local result_0 = getPointer(self).Offset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_data_model_mesh_offset_set_offset(self, value)
	getPointer(self).Offset = getPointer(value);
end
function abi.ffi.property_getter_data_model_mesh_scale(self)
	local result_0 = getPointer(self).Scale;
	return createPointer(result_0);
end
function abi.ffi.property_setter_data_model_mesh_scale_set_scale(self, value)
	getPointer(self).Scale = getPointer(value);
end
function abi.ffi.property_getter_data_model_mesh_vertex_color(self)
	local result_0 = getPointer(self).VertexColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_data_model_mesh_vertex_color_set_vertex_color(self, value)
	getPointer(self).VertexColor = getPointer(value);
end
function abi.ffi.property_getter_file_mesh_mesh_id(self)
	local result_0 = getPointer(self).MeshId;
	return createPointer(result_0);
end
function abi.ffi.property_setter_file_mesh_mesh_id_set_mesh_id(self, value)
	getPointer(self).MeshId = getPointer(value);
end
function abi.ffi.property_getter_file_mesh_texture_id(self)
	local result_0 = getPointer(self).TextureId;
	return createPointer(result_0);
end
function abi.ffi.property_setter_file_mesh_texture_id_set_texture_id(self, value)
	getPointer(self).TextureId = getPointer(value);
end
function abi.ffi.property_getter_special_mesh_mesh_type(self)
	local result_0 = getPointer(self).MeshType;
	return result_0;
end
function abi.ffi.property_setter_special_mesh_mesh_type_set_mesh_type(self, value)
	getPointer(self).MeshType = value;
end
function abi.ffi.property_getter_data_store_info_created_time(self)
	local result_0 = getPointer(self).CreatedTime;
	return result_0;
end
function abi.ffi.property_getter_data_store_info_data_store_name(self)
	local result_0 = getPointer(self).DataStoreName;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_data_store_info_updated_time(self)
	local result_0 = getPointer(self).UpdatedTime;
	return result_0;
end
function abi.ffi.property_getter_data_store_key_key_name(self)
	local result_0 = getPointer(self).KeyName;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_data_store_key_info_created_time(self)
	local result_0 = getPointer(self).CreatedTime;
	return result_0;
end
function abi.ffi.property_getter_data_store_key_info_updated_time(self)
	local result_0 = getPointer(self).UpdatedTime;
	return result_0;
end
function abi.ffi.property_getter_data_store_key_info_version(self)
	local result_0 = getPointer(self).Version;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_data_store_object_version_info_created_time(self)
	local result_0 = getPointer(self).CreatedTime;
	return result_0;
end
function abi.ffi.property_getter_data_store_object_version_info_is_deleted(self)
	local result_0 = getPointer(self).IsDeleted;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_data_store_object_version_info_version(self)
	local result_0 = getPointer(self).Version;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_data_store_options_all_scopes(self)
	local result_0 = getPointer(self).AllScopes;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_data_store_options_all_scopes_set_all_scopes(self, value)
	getPointer(self).AllScopes = value == 1;
end
function abi.ffi.method_data_store_service_get_data_store(self, name_addr, name_len, scope_addr, scope_len, options)
	local result_0 = getPointer(self):GetDataStore(loadString(memory, name_addr, name_len), loadString(memory, scope_addr, scope_len), getPointer(options));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_data_store_service_get_global_data_store(self)
	local result_0 = getPointer(self):GetGlobalDataStore();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_data_store_service_get_ordered_data_store(self, name_addr, name_len, scope_addr, scope_len)
	local result_0 = getPointer(self):GetOrderedDataStore(loadString(memory, name_addr, name_len), loadString(memory, scope_addr, scope_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_data_store_service_get_request_budget_for_request_type(self, request_type)
	local result_0 = getPointer(self):GetRequestBudgetForRequestType(request_type);
	return result_0;
end
function abi.ffi.method_data_store_service_list_data_stores_async(self, prefix_addr, prefix_len, page_size, cursor_addr, cursor_len)
	local result_0 = getPointer(self):ListDataStoresAsync(loadString(memory, prefix_addr, prefix_len), page_size, loadString(memory, cursor_addr, cursor_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_debris_add_item(self, item, lifetime)
	getPointer(self):AddItem(getPointer(item), lifetime)
end
function abi.ffi.property_getter_dialog_behavior_type(self)
	local result_0 = getPointer(self).BehaviorType;
	return result_0;
end
function abi.ffi.property_setter_dialog_behavior_type_set_behavior_type(self, value)
	getPointer(self).BehaviorType = value;
end
function abi.ffi.property_getter_dialog_conversation_distance(self)
	local result_0 = getPointer(self).ConversationDistance;
	return result_0;
end
function abi.ffi.property_setter_dialog_conversation_distance_set_conversation_distance(self, value)
	getPointer(self).ConversationDistance = value;
end
function abi.ffi.property_getter_dialog_goodbye_choice_active(self)
	local result_0 = getPointer(self).GoodbyeChoiceActive;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_dialog_goodbye_choice_active_set_goodbye_choice_active(self, value)
	getPointer(self).GoodbyeChoiceActive = value == 1;
end
function abi.ffi.property_getter_dialog_goodbye_dialog(self)
	local result_0 = getPointer(self).GoodbyeDialog;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_dialog_goodbye_dialog_set_goodbye_dialog(self, value_addr, value_len)
	getPointer(self).GoodbyeDialog = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_dialog_in_use(self)
	local result_0 = getPointer(self).InUse;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_dialog_in_use_set_in_use(self, value)
	getPointer(self).InUse = value == 1;
end
function abi.ffi.property_getter_dialog_initial_prompt(self)
	local result_0 = getPointer(self).InitialPrompt;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_dialog_initial_prompt_set_initial_prompt(self, value_addr, value_len)
	getPointer(self).InitialPrompt = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_dialog_purpose(self)
	local result_0 = getPointer(self).Purpose;
	return result_0;
end
function abi.ffi.property_setter_dialog_purpose_set_purpose(self, value)
	getPointer(self).Purpose = value;
end
function abi.ffi.property_getter_dialog_tone(self)
	local result_0 = getPointer(self).Tone;
	return result_0;
end
function abi.ffi.property_setter_dialog_tone_set_tone(self, value)
	getPointer(self).Tone = value;
end
function abi.ffi.property_getter_dialog_trigger_distance(self)
	local result_0 = getPointer(self).TriggerDistance;
	return result_0;
end
function abi.ffi.property_setter_dialog_trigger_distance_set_trigger_distance(self, value)
	getPointer(self).TriggerDistance = value;
end
function abi.ffi.property_getter_dialog_trigger_offset(self)
	local result_0 = getPointer(self).TriggerOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_dialog_trigger_offset_set_trigger_offset(self, value)
	getPointer(self).TriggerOffset = getPointer(value);
end
function abi.ffi.event_dialog_dialog_choice_selected_on_dialog_choice_selected(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).DialogChoiceSelected:Connect(function(player, dialog_choice)
		local player_optional, player_optional_0 = 0, 0;
		if (player ~= nil) then
			player_optional, player_optional_0 = 1, createPointer(player);
		end
		local dialog_choice_optional, dialog_choice_optional_0 = 0, 0;
		if (dialog_choice ~= nil) then
			dialog_choice_optional, dialog_choice_optional_0 = 1, createPointer(dialog_choice);
		end
		invokeFunction(callback_data, callback_vtable, player_optional, player_optional_0, dialog_choice_optional, dialog_choice_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_dialog_choice_goodbye_choice_active(self)
	local result_0 = getPointer(self).GoodbyeChoiceActive;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_dialog_choice_goodbye_choice_active_set_goodbye_choice_active(self, value)
	getPointer(self).GoodbyeChoiceActive = value == 1;
end
function abi.ffi.property_getter_dialog_choice_goodbye_dialog(self)
	local result_0 = getPointer(self).GoodbyeDialog;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_dialog_choice_goodbye_dialog_set_goodbye_dialog(self, value_addr, value_len)
	getPointer(self).GoodbyeDialog = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_dialog_choice_response_dialog(self)
	local result_0 = getPointer(self).ResponseDialog;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_dialog_choice_response_dialog_set_response_dialog(self, value_addr, value_len)
	getPointer(self).ResponseDialog = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_dialog_choice_user_dialog(self)
	local result_0 = getPointer(self).UserDialog;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_dialog_choice_user_dialog_set_user_dialog(self, value_addr, value_len)
	getPointer(self).UserDialog = loadString(memory, value_addr, value_len);
end
function abi.ffi.method_dragger_axis_rotate(self, axis)
	getPointer(self):AxisRotate(axis)
end
function abi.ffi.method_dragger_mouse_move(self, mouse_ray)
	getPointer(self):MouseMove(getPointer(mouse_ray))
end
function abi.ffi.method_dragger_mouse_up(self)
	getPointer(self):MouseUp()
end
function abi.ffi.property_getter_dragger_service_align_dragged_objects(self)
	local result_0 = getPointer(self).AlignDraggedObjects;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_dragger_service_angle_snap_enabled(self)
	local result_0 = getPointer(self).AngleSnapEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_dragger_service_angle_snap_increment(self)
	local result_0 = getPointer(self).AngleSnapIncrement;
	return result_0;
end
function abi.ffi.property_getter_dragger_service_animate_hover(self)
	local result_0 = getPointer(self).AnimateHover;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_dragger_service_collisions_enabled(self)
	local result_0 = getPointer(self).CollisionsEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_dragger_service_dragger_coordinate_space(self)
	local result_0 = getPointer(self).DraggerCoordinateSpace;
	return result_0;
end
function abi.ffi.property_getter_dragger_service_dragger_movement_mode(self)
	local result_0 = getPointer(self).DraggerMovementMode;
	return result_0;
end
function abi.ffi.property_getter_dragger_service_geometry_snap_color(self)
	local result_0 = getPointer(self).GeometrySnapColor;
	return createPointer(result_0);
end
function abi.ffi.property_getter_dragger_service_hover_animate_frequency(self)
	local result_0 = getPointer(self).HoverAnimateFrequency;
	return result_0;
end
function abi.ffi.property_getter_dragger_service_hover_thickness(self)
	local result_0 = getPointer(self).HoverThickness;
	return result_0;
end
function abi.ffi.property_getter_dragger_service_joints_enabled(self)
	local result_0 = getPointer(self).JointsEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_dragger_service_linear_snap_enabled(self)
	local result_0 = getPointer(self).LinearSnapEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_dragger_service_linear_snap_increment(self)
	local result_0 = getPointer(self).LinearSnapIncrement;
	return result_0;
end
function abi.ffi.property_getter_dragger_service_show_hover(self)
	local result_0 = getPointer(self).ShowHover;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_dragger_service_show_pivot_indicator(self)
	local result_0 = getPointer(self).ShowPivotIndicator;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_dragger_service_show_pivot_indicator_set_show_pivot_indicator(self, value)
	getPointer(self).ShowPivotIndicator = value == 1;
end
function abi.ffi.property_getter_euler_rotation_curve_rotation_order(self)
	local result_0 = getPointer(self).RotationOrder;
	return result_0;
end
function abi.ffi.property_setter_euler_rotation_curve_rotation_order_set_rotation_order(self, value)
	getPointer(self).RotationOrder = value;
end
function abi.ffi.method_euler_rotation_curve_get_rotation_at_time(self, time)
	local result_0 = getPointer(self):GetRotationAtTime(time);
	return createPointer(result_0);
end
function abi.ffi.method_euler_rotation_curve_x(self)
	local result_0 = getPointer(self):X();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_euler_rotation_curve_y(self)
	local result_0 = getPointer(self):Y();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_euler_rotation_curve_z(self)
	local result_0 = getPointer(self):Z();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_experience_invite_options_invite_message_id(self)
	local result_0 = getPointer(self).InviteMessageId;
	return result_0;
end
function abi.ffi.property_setter_experience_invite_options_invite_message_id_set_invite_message_id(self, value)
	getPointer(self).InviteMessageId = value;
end
function abi.ffi.property_getter_experience_invite_options_invite_user(self)
	local result_0 = getPointer(self).InviteUser;
	return result_0;
end
function abi.ffi.property_setter_experience_invite_options_invite_user_set_invite_user(self, value)
	getPointer(self).InviteUser = value;
end
function abi.ffi.property_getter_experience_invite_options_launch_data(self)
	local result_0 = getPointer(self).LaunchData;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_experience_invite_options_launch_data_set_launch_data(self, value_addr, value_len)
	getPointer(self).LaunchData = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_experience_invite_options_prompt_message(self)
	local result_0 = getPointer(self).PromptMessage;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_experience_invite_options_prompt_message_set_prompt_message(self, value_addr, value_len)
	getPointer(self).PromptMessage = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_explosion_blast_pressure(self)
	local result_0 = getPointer(self).BlastPressure;
	return result_0;
end
function abi.ffi.property_setter_explosion_blast_pressure_set_blast_pressure(self, value)
	getPointer(self).BlastPressure = value;
end
function abi.ffi.property_getter_explosion_blast_radius(self)
	local result_0 = getPointer(self).BlastRadius;
	return result_0;
end
function abi.ffi.property_setter_explosion_blast_radius_set_blast_radius(self, value)
	getPointer(self).BlastRadius = value;
end
function abi.ffi.property_getter_explosion_destroy_joint_radius_percent(self)
	local result_0 = getPointer(self).DestroyJointRadiusPercent;
	return result_0;
end
function abi.ffi.property_setter_explosion_destroy_joint_radius_percent_set_destroy_joint_radius_percent(self, value)
	getPointer(self).DestroyJointRadiusPercent = value;
end
function abi.ffi.property_getter_explosion_explosion_type(self)
	local result_0 = getPointer(self).ExplosionType;
	return result_0;
end
function abi.ffi.property_setter_explosion_explosion_type_set_explosion_type(self, value)
	getPointer(self).ExplosionType = value;
end
function abi.ffi.property_getter_explosion_position(self)
	local result_0 = getPointer(self).Position;
	return createPointer(result_0);
end
function abi.ffi.property_setter_explosion_position_set_position(self, value)
	getPointer(self).Position = getPointer(value);
end
function abi.ffi.property_getter_explosion_time_scale(self)
	local result_0 = getPointer(self).TimeScale;
	return result_0;
end
function abi.ffi.property_setter_explosion_time_scale_set_time_scale(self, value)
	getPointer(self).TimeScale = value;
end
function abi.ffi.property_getter_explosion_visible(self)
	local result_0 = getPointer(self).Visible;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_explosion_visible_set_visible(self, value)
	getPointer(self).Visible = value == 1;
end
function abi.ffi.event_explosion_hit_on_hit(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Hit:Connect(function(part, distance)
		local part_optional, part_optional_0 = 0, 0;
		if (part ~= nil) then
			part_optional, part_optional_0 = 1, createPointer(part);
		end
		invokeFunction(callback_data, callback_vtable, part_optional, part_optional_0, distance)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_face_instance_face(self)
	local result_0 = getPointer(self).Face;
	return result_0;
end
function abi.ffi.property_setter_face_instance_face_set_face(self, value)
	getPointer(self).Face = value;
end
function abi.ffi.property_getter_decal_color_3(self)
	local result_0 = getPointer(self).Color3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_decal_color_3_set_color_3(self, value)
	getPointer(self).Color3 = getPointer(value);
end
function abi.ffi.property_getter_decal_local_transparency_modifier(self)
	local result_0 = getPointer(self).LocalTransparencyModifier;
	return result_0;
end
function abi.ffi.property_setter_decal_local_transparency_modifier_set_local_transparency_modifier(self, value)
	getPointer(self).LocalTransparencyModifier = value;
end
function abi.ffi.property_getter_decal_texture(self)
	local result_0 = getPointer(self).Texture;
	return createPointer(result_0);
end
function abi.ffi.property_setter_decal_texture_set_texture(self, value)
	getPointer(self).Texture = getPointer(value);
end
function abi.ffi.property_getter_decal_transparency(self)
	local result_0 = getPointer(self).Transparency;
	return result_0;
end
function abi.ffi.property_setter_decal_transparency_set_transparency(self, value)
	getPointer(self).Transparency = value;
end
function abi.ffi.property_getter_decal_z_index(self)
	local result_0 = getPointer(self).ZIndex;
	return result_0;
end
function abi.ffi.property_setter_decal_z_index_set_z_index(self, value)
	getPointer(self).ZIndex = value;
end
function abi.ffi.property_getter_texture_offset_studs_u(self)
	local result_0 = getPointer(self).OffsetStudsU;
	return result_0;
end
function abi.ffi.property_setter_texture_offset_studs_u_set_offset_studs_u(self, value)
	getPointer(self).OffsetStudsU = value;
end
function abi.ffi.property_getter_texture_offset_studs_v(self)
	local result_0 = getPointer(self).OffsetStudsV;
	return result_0;
end
function abi.ffi.property_setter_texture_offset_studs_v_set_offset_studs_v(self, value)
	getPointer(self).OffsetStudsV = value;
end
function abi.ffi.property_getter_texture_studs_per_tile_u(self)
	local result_0 = getPointer(self).StudsPerTileU;
	return result_0;
end
function abi.ffi.property_setter_texture_studs_per_tile_u_set_studs_per_tile_u(self, value)
	getPointer(self).StudsPerTileU = value;
end
function abi.ffi.property_getter_texture_studs_per_tile_v(self)
	local result_0 = getPointer(self).StudsPerTileV;
	return result_0;
end
function abi.ffi.property_setter_texture_studs_per_tile_v_set_studs_per_tile_v(self, value)
	getPointer(self).StudsPerTileV = value;
end
function abi.ffi.property_getter_feature_face_id(self)
	local result_0 = getPointer(self).FaceId;
	return result_0;
end
function abi.ffi.property_setter_feature_face_id_set_face_id(self, value)
	getPointer(self).FaceId = value;
end
function abi.ffi.property_getter_feature_in_out(self)
	local result_0 = getPointer(self).InOut;
	return result_0;
end
function abi.ffi.property_setter_feature_in_out_set_in_out(self, value)
	getPointer(self).InOut = value;
end
function abi.ffi.property_getter_feature_left_right(self)
	local result_0 = getPointer(self).LeftRight;
	return result_0;
end
function abi.ffi.property_setter_feature_left_right_set_left_right(self, value)
	getPointer(self).LeftRight = value;
end
function abi.ffi.property_getter_feature_top_bottom(self)
	local result_0 = getPointer(self).TopBottom;
	return result_0;
end
function abi.ffi.property_setter_feature_top_bottom_set_top_bottom(self, value)
	getPointer(self).TopBottom = value;
end
function abi.ffi.property_getter_fire_color(self)
	local result_0 = getPointer(self).Color;
	return createPointer(result_0);
end
function abi.ffi.property_setter_fire_color_set_color(self, value)
	getPointer(self).Color = getPointer(value);
end
function abi.ffi.property_getter_fire_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_fire_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_fire_heat(self)
	local result_0 = getPointer(self).Heat;
	return result_0;
end
function abi.ffi.property_setter_fire_heat_set_heat(self, value)
	getPointer(self).Heat = value;
end
function abi.ffi.property_getter_fire_secondary_color(self)
	local result_0 = getPointer(self).SecondaryColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_fire_secondary_color_set_secondary_color(self, value)
	getPointer(self).SecondaryColor = getPointer(value);
end
function abi.ffi.property_getter_fire_size(self)
	local result_0 = getPointer(self).Size;
	return result_0;
end
function abi.ffi.property_setter_fire_size_set_size(self, value)
	getPointer(self).Size = value;
end
function abi.ffi.property_getter_fire_time_scale(self)
	local result_0 = getPointer(self).TimeScale;
	return result_0;
end
function abi.ffi.property_setter_fire_time_scale_set_time_scale(self, value)
	getPointer(self).TimeScale = value;
end
function abi.ffi.property_getter_float_curve_length(self)
	local result_0 = getPointer(self).Length;
	return result_0;
end
function abi.ffi.method_float_curve_get_key_at_index(self, index)
	local result_0 = getPointer(self):GetKeyAtIndex(index);
	return createPointer(result_0);
end
function abi.ffi.method_float_curve_get_value_at_time(self, time)
	local result_0 = getPointer(self):GetValueAtTime(time);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, result_0;
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_float_curve_remove_key_at_index(self, starting_index, count)
	local result_0 = getPointer(self):RemoveKeyAtIndex(starting_index, count);
	return result_0;
end
function abi.ffi.property_getter_force_field_visible(self)
	local result_0 = getPointer(self).Visible;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_force_field_visible_set_visible(self, value)
	getPointer(self).Visible = value == 1;
end
function abi.ffi.property_getter_get_text_bounds_params_font(self)
	local result_0 = getPointer(self).Font;
	return createPointer(result_0);
end
function abi.ffi.property_setter_get_text_bounds_params_font_set_font(self, value)
	getPointer(self).Font = getPointer(value);
end
function abi.ffi.property_getter_get_text_bounds_params_size(self)
	local result_0 = getPointer(self).Size;
	return result_0;
end
function abi.ffi.property_setter_get_text_bounds_params_size_set_size(self, value)
	getPointer(self).Size = value;
end
function abi.ffi.property_getter_get_text_bounds_params_text(self)
	local result_0 = getPointer(self).Text;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_get_text_bounds_params_text_set_text(self, value_addr, value_len)
	getPointer(self).Text = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_get_text_bounds_params_width(self)
	local result_0 = getPointer(self).Width;
	return result_0;
end
function abi.ffi.property_setter_get_text_bounds_params_width_set_width(self, value)
	getPointer(self).Width = value;
end
function abi.ffi.method_data_store_list_keys_async(self, prefix_addr, prefix_len, page_size, cursor_addr, cursor_len)
	local result_0 = getPointer(self):ListKeysAsync(loadString(memory, prefix_addr, prefix_len), page_size, loadString(memory, cursor_addr, cursor_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_data_store_list_versions_async(self, key_addr, key_len, sort_direction, min_date, max_date, page_size)
	local result_0 = getPointer(self):ListVersionsAsync(loadString(memory, key_addr, key_len), sort_direction, min_date, max_date, page_size);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_data_store_remove_version_async(self, key_addr, key_len, version_addr, version_len)
	getPointer(self):RemoveVersionAsync(loadString(memory, key_addr, key_len), loadString(memory, version_addr, version_len))
end
function abi.ffi.method_group_service_get_allies_async(self, group_id)
	local result_0 = getPointer(self):GetAlliesAsync(group_id);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_group_service_get_enemies_async(self, group_id)
	local result_0 = getPointer(self):GetEnemiesAsync(group_id);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_gui_base_2_d_absolute_position(self)
	local result_0 = getPointer(self).AbsolutePosition;
	return createPointer(result_0);
end
function abi.ffi.property_getter_gui_base_2_d_absolute_rotation(self)
	local result_0 = getPointer(self).AbsoluteRotation;
	return result_0;
end
function abi.ffi.property_getter_gui_base_2_d_absolute_size(self)
	local result_0 = getPointer(self).AbsoluteSize;
	return createPointer(result_0);
end
function abi.ffi.property_getter_gui_base_2_d_auto_localize(self)
	local result_0 = getPointer(self).AutoLocalize;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_base_2_d_auto_localize_set_auto_localize(self, value)
	getPointer(self).AutoLocalize = value == 1;
end
function abi.ffi.property_getter_gui_base_2_d_root_localization_table(self)
	local result_0 = getPointer(self).RootLocalizationTable;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_gui_base_2_d_root_localization_table_set_root_localization_table(self, value)
	getPointer(self).RootLocalizationTable = getPointer(value);
end
function abi.ffi.property_getter_gui_base_2_d_selection_behavior_down(self)
	local result_0 = getPointer(self).SelectionBehaviorDown;
	return result_0;
end
function abi.ffi.property_setter_gui_base_2_d_selection_behavior_down_set_selection_behavior_down(self, value)
	getPointer(self).SelectionBehaviorDown = value;
end
function abi.ffi.property_getter_gui_base_2_d_selection_behavior_left(self)
	local result_0 = getPointer(self).SelectionBehaviorLeft;
	return result_0;
end
function abi.ffi.property_setter_gui_base_2_d_selection_behavior_left_set_selection_behavior_left(self, value)
	getPointer(self).SelectionBehaviorLeft = value;
end
function abi.ffi.property_getter_gui_base_2_d_selection_behavior_right(self)
	local result_0 = getPointer(self).SelectionBehaviorRight;
	return result_0;
end
function abi.ffi.property_setter_gui_base_2_d_selection_behavior_right_set_selection_behavior_right(self, value)
	getPointer(self).SelectionBehaviorRight = value;
end
function abi.ffi.property_getter_gui_base_2_d_selection_behavior_up(self)
	local result_0 = getPointer(self).SelectionBehaviorUp;
	return result_0;
end
function abi.ffi.property_setter_gui_base_2_d_selection_behavior_up_set_selection_behavior_up(self, value)
	getPointer(self).SelectionBehaviorUp = value;
end
function abi.ffi.property_getter_gui_base_2_d_selection_group(self)
	local result_0 = getPointer(self).SelectionGroup;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_base_2_d_selection_group_set_selection_group(self, value)
	getPointer(self).SelectionGroup = value == 1;
end
function abi.ffi.event_gui_base_2_d_selection_changed_on_selection_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).SelectionChanged:Connect(function(am_i_selected, previous_selection, new_selection)
		local previous_selection_optional, previous_selection_optional_0 = 0, 0;
		if (previous_selection ~= nil) then
			previous_selection_optional, previous_selection_optional_0 = 1, createPointer(previous_selection);
		end
		local new_selection_optional, new_selection_optional_0 = 0, 0;
		if (new_selection ~= nil) then
			new_selection_optional, new_selection_optional_0 = 1, createPointer(new_selection);
		end
		invokeFunction(callback_data, callback_vtable, am_i_selected and 1 or 0, previous_selection_optional, previous_selection_optional_0, new_selection_optional, new_selection_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_gui_object_active(self)
	local result_0 = getPointer(self).Active;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_object_active_set_active(self, value)
	getPointer(self).Active = value == 1;
end
function abi.ffi.property_getter_gui_object_anchor_point(self)
	local result_0 = getPointer(self).AnchorPoint;
	return createPointer(result_0);
end
function abi.ffi.property_setter_gui_object_anchor_point_set_anchor_point(self, value)
	getPointer(self).AnchorPoint = getPointer(value);
end
function abi.ffi.property_getter_gui_object_automatic_size(self)
	local result_0 = getPointer(self).AutomaticSize;
	return result_0;
end
function abi.ffi.property_setter_gui_object_automatic_size_set_automatic_size(self, value)
	getPointer(self).AutomaticSize = value;
end
function abi.ffi.property_getter_gui_object_background_color_3(self)
	local result_0 = getPointer(self).BackgroundColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_gui_object_background_color_3_set_background_color_3(self, value)
	getPointer(self).BackgroundColor3 = getPointer(value);
end
function abi.ffi.property_getter_gui_object_background_transparency(self)
	local result_0 = getPointer(self).BackgroundTransparency;
	return result_0;
end
function abi.ffi.property_setter_gui_object_background_transparency_set_background_transparency(self, value)
	getPointer(self).BackgroundTransparency = value;
end
function abi.ffi.property_getter_gui_object_border_color_3(self)
	local result_0 = getPointer(self).BorderColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_gui_object_border_color_3_set_border_color_3(self, value)
	getPointer(self).BorderColor3 = getPointer(value);
end
function abi.ffi.property_getter_gui_object_border_mode(self)
	local result_0 = getPointer(self).BorderMode;
	return result_0;
end
function abi.ffi.property_setter_gui_object_border_mode_set_border_mode(self, value)
	getPointer(self).BorderMode = value;
end
function abi.ffi.property_getter_gui_object_border_size_pixel(self)
	local result_0 = getPointer(self).BorderSizePixel;
	return result_0;
end
function abi.ffi.property_setter_gui_object_border_size_pixel_set_border_size_pixel(self, value)
	getPointer(self).BorderSizePixel = value;
end
function abi.ffi.property_getter_gui_object_clips_descendants(self)
	local result_0 = getPointer(self).ClipsDescendants;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_object_clips_descendants_set_clips_descendants(self, value)
	getPointer(self).ClipsDescendants = value == 1;
end
function abi.ffi.property_getter_gui_object_layout_order(self)
	local result_0 = getPointer(self).LayoutOrder;
	return result_0;
end
function abi.ffi.property_setter_gui_object_layout_order_set_layout_order(self, value)
	getPointer(self).LayoutOrder = value;
end
function abi.ffi.property_getter_gui_object_next_selection_down(self)
	local result_0 = getPointer(self).NextSelectionDown;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_gui_object_next_selection_down_set_next_selection_down(self, value)
	getPointer(self).NextSelectionDown = getPointer(value);
end
function abi.ffi.property_getter_gui_object_next_selection_left(self)
	local result_0 = getPointer(self).NextSelectionLeft;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_gui_object_next_selection_left_set_next_selection_left(self, value)
	getPointer(self).NextSelectionLeft = getPointer(value);
end
function abi.ffi.property_getter_gui_object_next_selection_right(self)
	local result_0 = getPointer(self).NextSelectionRight;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_gui_object_next_selection_right_set_next_selection_right(self, value)
	getPointer(self).NextSelectionRight = getPointer(value);
end
function abi.ffi.property_getter_gui_object_next_selection_up(self)
	local result_0 = getPointer(self).NextSelectionUp;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_gui_object_next_selection_up_set_next_selection_up(self, value)
	getPointer(self).NextSelectionUp = getPointer(value);
end
function abi.ffi.property_getter_gui_object_position(self)
	local result_0 = getPointer(self).Position;
	return createPointer(result_0);
end
function abi.ffi.property_setter_gui_object_position_set_position(self, value)
	getPointer(self).Position = getPointer(value);
end
function abi.ffi.property_getter_gui_object_rotation(self)
	local result_0 = getPointer(self).Rotation;
	return result_0;
end
function abi.ffi.property_setter_gui_object_rotation_set_rotation(self, value)
	getPointer(self).Rotation = value;
end
function abi.ffi.property_getter_gui_object_selectable(self)
	local result_0 = getPointer(self).Selectable;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_object_selectable_set_selectable(self, value)
	getPointer(self).Selectable = value == 1;
end
function abi.ffi.property_getter_gui_object_selection_image_object(self)
	local result_0 = getPointer(self).SelectionImageObject;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_gui_object_selection_image_object_set_selection_image_object(self, value)
	getPointer(self).SelectionImageObject = getPointer(value);
end
function abi.ffi.property_getter_gui_object_selection_order(self)
	local result_0 = getPointer(self).SelectionOrder;
	return result_0;
end
function abi.ffi.property_setter_gui_object_selection_order_set_selection_order(self, value)
	getPointer(self).SelectionOrder = value;
end
function abi.ffi.property_getter_gui_object_size(self)
	local result_0 = getPointer(self).Size;
	return createPointer(result_0);
end
function abi.ffi.property_setter_gui_object_size_set_size(self, value)
	getPointer(self).Size = getPointer(value);
end
function abi.ffi.property_getter_gui_object_size_constraint(self)
	local result_0 = getPointer(self).SizeConstraint;
	return result_0;
end
function abi.ffi.property_setter_gui_object_size_constraint_set_size_constraint(self, value)
	getPointer(self).SizeConstraint = value;
end
function abi.ffi.property_getter_gui_object_transparency(self)
	local result_0 = getPointer(self).Transparency;
	return result_0;
end
function abi.ffi.property_setter_gui_object_transparency_set_transparency(self, value)
	getPointer(self).Transparency = value;
end
function abi.ffi.property_getter_gui_object_visible(self)
	local result_0 = getPointer(self).Visible;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_object_visible_set_visible(self, value)
	getPointer(self).Visible = value == 1;
end
function abi.ffi.property_getter_gui_object_z_index(self)
	local result_0 = getPointer(self).ZIndex;
	return result_0;
end
function abi.ffi.property_setter_gui_object_z_index_set_z_index(self, value)
	getPointer(self).ZIndex = value;
end
function abi.ffi.event_gui_object_input_began_on_input_began(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).InputBegan:Connect(function(input)
		local input_optional, input_optional_0 = 0, 0;
		if (input ~= nil) then
			input_optional, input_optional_0 = 1, createPointer(input);
		end
		invokeFunction(callback_data, callback_vtable, input_optional, input_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_object_input_changed_on_input_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).InputChanged:Connect(function(input)
		local input_optional, input_optional_0 = 0, 0;
		if (input ~= nil) then
			input_optional, input_optional_0 = 1, createPointer(input);
		end
		invokeFunction(callback_data, callback_vtable, input_optional, input_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_object_input_ended_on_input_ended(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).InputEnded:Connect(function(input)
		local input_optional, input_optional_0 = 0, 0;
		if (input ~= nil) then
			input_optional, input_optional_0 = 1, createPointer(input);
		end
		invokeFunction(callback_data, callback_vtable, input_optional, input_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_object_mouse_enter_on_mouse_enter(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseEnter:Connect(function(x, y)
		invokeFunction(callback_data, callback_vtable, x, y)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_object_mouse_leave_on_mouse_leave(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseLeave:Connect(function(x, y)
		invokeFunction(callback_data, callback_vtable, x, y)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_object_mouse_moved_on_mouse_moved(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseMoved:Connect(function(x, y)
		invokeFunction(callback_data, callback_vtable, x, y)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_object_mouse_wheel_backward_on_mouse_wheel_backward(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseWheelBackward:Connect(function(x, y)
		invokeFunction(callback_data, callback_vtable, x, y)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_object_mouse_wheel_forward_on_mouse_wheel_forward(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseWheelForward:Connect(function(x, y)
		invokeFunction(callback_data, callback_vtable, x, y)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_object_selection_gained_on_selection_gained(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).SelectionGained:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_object_selection_lost_on_selection_lost(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).SelectionLost:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_object_touch_swipe_on_touch_swipe(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TouchSwipe:Connect(function(swipe_direction, number_of_touches)
		invokeFunction(callback_data, callback_vtable, swipe_direction, number_of_touches)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_canvas_group_group_color_3(self)
	local result_0 = getPointer(self).GroupColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_canvas_group_group_color_3_set_group_color_3(self, value)
	getPointer(self).GroupColor3 = getPointer(value);
end
function abi.ffi.property_getter_canvas_group_group_transparency(self)
	local result_0 = getPointer(self).GroupTransparency;
	return result_0;
end
function abi.ffi.property_setter_canvas_group_group_transparency_set_group_transparency(self, value)
	getPointer(self).GroupTransparency = value;
end
function abi.ffi.property_getter_frame_style(self)
	local result_0 = getPointer(self).Style;
	return result_0;
end
function abi.ffi.property_setter_frame_style_set_style(self, value)
	getPointer(self).Style = value;
end
function abi.ffi.property_getter_gui_button_auto_button_color(self)
	local result_0 = getPointer(self).AutoButtonColor;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_button_auto_button_color_set_auto_button_color(self, value)
	getPointer(self).AutoButtonColor = value == 1;
end
function abi.ffi.property_getter_gui_button_modal(self)
	local result_0 = getPointer(self).Modal;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_button_modal_set_modal(self, value)
	getPointer(self).Modal = value == 1;
end
function abi.ffi.property_getter_gui_button_selected(self)
	local result_0 = getPointer(self).Selected;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_button_selected_set_selected(self, value)
	getPointer(self).Selected = value == 1;
end
function abi.ffi.property_getter_gui_button_style(self)
	local result_0 = getPointer(self).Style;
	return result_0;
end
function abi.ffi.property_setter_gui_button_style_set_style(self, value)
	getPointer(self).Style = value;
end
function abi.ffi.event_gui_button_activated_on_activated(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Activated:Connect(function(input_object, click_count)
		local input_object_optional, input_object_optional_0 = 0, 0;
		if (input_object ~= nil) then
			input_object_optional, input_object_optional_0 = 1, createPointer(input_object);
		end
		invokeFunction(callback_data, callback_vtable, input_object_optional, input_object_optional_0, click_count)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_button_mouse_button_1_click_on_mouse_button_1_click(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseButton1Click:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_button_mouse_button_1_down_on_mouse_button_1_down(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseButton1Down:Connect(function(x, y)
		invokeFunction(callback_data, callback_vtable, x, y)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_button_mouse_button_1_up_on_mouse_button_1_up(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseButton1Up:Connect(function(x, y)
		invokeFunction(callback_data, callback_vtable, x, y)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_button_mouse_button_2_click_on_mouse_button_2_click(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseButton2Click:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_button_mouse_button_2_down_on_mouse_button_2_down(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseButton2Down:Connect(function(x, y)
		invokeFunction(callback_data, callback_vtable, x, y)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_button_mouse_button_2_up_on_mouse_button_2_up(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseButton2Up:Connect(function(x, y)
		invokeFunction(callback_data, callback_vtable, x, y)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_image_button_hover_image(self)
	local result_0 = getPointer(self).HoverImage;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_button_hover_image_set_hover_image(self, value)
	getPointer(self).HoverImage = getPointer(value);
end
function abi.ffi.property_getter_image_button_image(self)
	local result_0 = getPointer(self).Image;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_button_image_set_image(self, value)
	getPointer(self).Image = getPointer(value);
end
function abi.ffi.property_getter_image_button_image_color_3(self)
	local result_0 = getPointer(self).ImageColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_button_image_color_3_set_image_color_3(self, value)
	getPointer(self).ImageColor3 = getPointer(value);
end
function abi.ffi.property_getter_image_button_image_rect_offset(self)
	local result_0 = getPointer(self).ImageRectOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_button_image_rect_offset_set_image_rect_offset(self, value)
	getPointer(self).ImageRectOffset = getPointer(value);
end
function abi.ffi.property_getter_image_button_image_rect_size(self)
	local result_0 = getPointer(self).ImageRectSize;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_button_image_rect_size_set_image_rect_size(self, value)
	getPointer(self).ImageRectSize = getPointer(value);
end
function abi.ffi.property_getter_image_button_image_transparency(self)
	local result_0 = getPointer(self).ImageTransparency;
	return result_0;
end
function abi.ffi.property_setter_image_button_image_transparency_set_image_transparency(self, value)
	getPointer(self).ImageTransparency = value;
end
function abi.ffi.property_getter_image_button_is_loaded(self)
	local result_0 = getPointer(self).IsLoaded;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_image_button_pressed_image(self)
	local result_0 = getPointer(self).PressedImage;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_button_pressed_image_set_pressed_image(self, value)
	getPointer(self).PressedImage = getPointer(value);
end
function abi.ffi.property_getter_image_button_resample_mode(self)
	local result_0 = getPointer(self).ResampleMode;
	return result_0;
end
function abi.ffi.property_setter_image_button_resample_mode_set_resample_mode(self, value)
	getPointer(self).ResampleMode = value;
end
function abi.ffi.property_getter_image_button_scale_type(self)
	local result_0 = getPointer(self).ScaleType;
	return result_0;
end
function abi.ffi.property_setter_image_button_scale_type_set_scale_type(self, value)
	getPointer(self).ScaleType = value;
end
function abi.ffi.property_getter_image_button_slice_center(self)
	local result_0 = getPointer(self).SliceCenter;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_button_slice_center_set_slice_center(self, value)
	getPointer(self).SliceCenter = getPointer(value);
end
function abi.ffi.property_getter_image_button_slice_scale(self)
	local result_0 = getPointer(self).SliceScale;
	return result_0;
end
function abi.ffi.property_setter_image_button_slice_scale_set_slice_scale(self, value)
	getPointer(self).SliceScale = value;
end
function abi.ffi.property_getter_image_button_tile_size(self)
	local result_0 = getPointer(self).TileSize;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_button_tile_size_set_tile_size(self, value)
	getPointer(self).TileSize = getPointer(value);
end
function abi.ffi.property_getter_text_button_content_text(self)
	local result_0 = getPointer(self).ContentText;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_text_button_font(self)
	local result_0 = getPointer(self).Font;
	return result_0;
end
function abi.ffi.property_setter_text_button_font_set_font(self, value)
	getPointer(self).Font = value;
end
function abi.ffi.property_getter_text_button_font_face(self)
	local result_0 = getPointer(self).FontFace;
	return createPointer(result_0);
end
function abi.ffi.property_setter_text_button_font_face_set_font_face(self, value)
	getPointer(self).FontFace = getPointer(value);
end
function abi.ffi.property_getter_text_button_line_height(self)
	local result_0 = getPointer(self).LineHeight;
	return result_0;
end
function abi.ffi.property_setter_text_button_line_height_set_line_height(self, value)
	getPointer(self).LineHeight = value;
end
function abi.ffi.property_getter_text_button_localized_text(self)
	local result_0 = getPointer(self).LocalizedText;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_text_button_max_visible_graphemes(self)
	local result_0 = getPointer(self).MaxVisibleGraphemes;
	return result_0;
end
function abi.ffi.property_setter_text_button_max_visible_graphemes_set_max_visible_graphemes(self, value)
	getPointer(self).MaxVisibleGraphemes = value;
end
function abi.ffi.property_getter_text_button_rich_text(self)
	local result_0 = getPointer(self).RichText;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_button_rich_text_set_rich_text(self, value)
	getPointer(self).RichText = value == 1;
end
function abi.ffi.property_getter_text_button_text(self)
	local result_0 = getPointer(self).Text;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_text_button_text_set_text(self, value_addr, value_len)
	getPointer(self).Text = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_text_button_text_bounds(self)
	local result_0 = getPointer(self).TextBounds;
	return createPointer(result_0);
end
function abi.ffi.property_getter_text_button_text_color_3(self)
	local result_0 = getPointer(self).TextColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_text_button_text_color_3_set_text_color_3(self, value)
	getPointer(self).TextColor3 = getPointer(value);
end
function abi.ffi.property_getter_text_button_text_fits(self)
	local result_0 = getPointer(self).TextFits;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_text_button_text_scaled(self)
	local result_0 = getPointer(self).TextScaled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_button_text_scaled_set_text_scaled(self, value)
	getPointer(self).TextScaled = value == 1;
end
function abi.ffi.property_getter_text_button_text_size(self)
	local result_0 = getPointer(self).TextSize;
	return result_0;
end
function abi.ffi.property_setter_text_button_text_size_set_text_size(self, value)
	getPointer(self).TextSize = value;
end
function abi.ffi.property_getter_text_button_text_stroke_color_3(self)
	local result_0 = getPointer(self).TextStrokeColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_text_button_text_stroke_color_3_set_text_stroke_color_3(self, value)
	getPointer(self).TextStrokeColor3 = getPointer(value);
end
function abi.ffi.property_getter_text_button_text_stroke_transparency(self)
	local result_0 = getPointer(self).TextStrokeTransparency;
	return result_0;
end
function abi.ffi.property_setter_text_button_text_stroke_transparency_set_text_stroke_transparency(self, value)
	getPointer(self).TextStrokeTransparency = value;
end
function abi.ffi.property_getter_text_button_text_transparency(self)
	local result_0 = getPointer(self).TextTransparency;
	return result_0;
end
function abi.ffi.property_setter_text_button_text_transparency_set_text_transparency(self, value)
	getPointer(self).TextTransparency = value;
end
function abi.ffi.property_getter_text_button_text_truncate(self)
	local result_0 = getPointer(self).TextTruncate;
	return result_0;
end
function abi.ffi.property_setter_text_button_text_truncate_set_text_truncate(self, value)
	getPointer(self).TextTruncate = value;
end
function abi.ffi.property_getter_text_button_text_wrapped(self)
	local result_0 = getPointer(self).TextWrapped;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_button_text_wrapped_set_text_wrapped(self, value)
	getPointer(self).TextWrapped = value == 1;
end
function abi.ffi.property_getter_text_button_text_x_alignment(self)
	local result_0 = getPointer(self).TextXAlignment;
	return result_0;
end
function abi.ffi.property_setter_text_button_text_x_alignment_set_text_x_alignment(self, value)
	getPointer(self).TextXAlignment = value;
end
function abi.ffi.property_getter_text_button_text_y_alignment(self)
	local result_0 = getPointer(self).TextYAlignment;
	return result_0;
end
function abi.ffi.property_setter_text_button_text_y_alignment_set_text_y_alignment(self, value)
	getPointer(self).TextYAlignment = value;
end
function abi.ffi.property_getter_image_label_image(self)
	local result_0 = getPointer(self).Image;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_label_image_set_image(self, value)
	getPointer(self).Image = getPointer(value);
end
function abi.ffi.property_getter_image_label_image_color_3(self)
	local result_0 = getPointer(self).ImageColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_label_image_color_3_set_image_color_3(self, value)
	getPointer(self).ImageColor3 = getPointer(value);
end
function abi.ffi.property_getter_image_label_image_rect_offset(self)
	local result_0 = getPointer(self).ImageRectOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_label_image_rect_offset_set_image_rect_offset(self, value)
	getPointer(self).ImageRectOffset = getPointer(value);
end
function abi.ffi.property_getter_image_label_image_rect_size(self)
	local result_0 = getPointer(self).ImageRectSize;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_label_image_rect_size_set_image_rect_size(self, value)
	getPointer(self).ImageRectSize = getPointer(value);
end
function abi.ffi.property_getter_image_label_image_transparency(self)
	local result_0 = getPointer(self).ImageTransparency;
	return result_0;
end
function abi.ffi.property_setter_image_label_image_transparency_set_image_transparency(self, value)
	getPointer(self).ImageTransparency = value;
end
function abi.ffi.property_getter_image_label_is_loaded(self)
	local result_0 = getPointer(self).IsLoaded;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_image_label_resample_mode(self)
	local result_0 = getPointer(self).ResampleMode;
	return result_0;
end
function abi.ffi.property_setter_image_label_resample_mode_set_resample_mode(self, value)
	getPointer(self).ResampleMode = value;
end
function abi.ffi.property_getter_image_label_scale_type(self)
	local result_0 = getPointer(self).ScaleType;
	return result_0;
end
function abi.ffi.property_setter_image_label_scale_type_set_scale_type(self, value)
	getPointer(self).ScaleType = value;
end
function abi.ffi.property_getter_image_label_slice_center(self)
	local result_0 = getPointer(self).SliceCenter;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_label_slice_center_set_slice_center(self, value)
	getPointer(self).SliceCenter = getPointer(value);
end
function abi.ffi.property_getter_image_label_slice_scale(self)
	local result_0 = getPointer(self).SliceScale;
	return result_0;
end
function abi.ffi.property_setter_image_label_slice_scale_set_slice_scale(self, value)
	getPointer(self).SliceScale = value;
end
function abi.ffi.property_getter_image_label_tile_size(self)
	local result_0 = getPointer(self).TileSize;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_label_tile_size_set_tile_size(self, value)
	getPointer(self).TileSize = getPointer(value);
end
function abi.ffi.property_getter_text_label_content_text(self)
	local result_0 = getPointer(self).ContentText;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_text_label_font(self)
	local result_0 = getPointer(self).Font;
	return result_0;
end
function abi.ffi.property_setter_text_label_font_set_font(self, value)
	getPointer(self).Font = value;
end
function abi.ffi.property_getter_text_label_font_face(self)
	local result_0 = getPointer(self).FontFace;
	return createPointer(result_0);
end
function abi.ffi.property_setter_text_label_font_face_set_font_face(self, value)
	getPointer(self).FontFace = getPointer(value);
end
function abi.ffi.property_getter_text_label_line_height(self)
	local result_0 = getPointer(self).LineHeight;
	return result_0;
end
function abi.ffi.property_setter_text_label_line_height_set_line_height(self, value)
	getPointer(self).LineHeight = value;
end
function abi.ffi.property_getter_text_label_localized_text(self)
	local result_0 = getPointer(self).LocalizedText;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_text_label_max_visible_graphemes(self)
	local result_0 = getPointer(self).MaxVisibleGraphemes;
	return result_0;
end
function abi.ffi.property_setter_text_label_max_visible_graphemes_set_max_visible_graphemes(self, value)
	getPointer(self).MaxVisibleGraphemes = value;
end
function abi.ffi.property_getter_text_label_rich_text(self)
	local result_0 = getPointer(self).RichText;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_label_rich_text_set_rich_text(self, value)
	getPointer(self).RichText = value == 1;
end
function abi.ffi.property_getter_text_label_text(self)
	local result_0 = getPointer(self).Text;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_text_label_text_set_text(self, value_addr, value_len)
	getPointer(self).Text = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_text_label_text_bounds(self)
	local result_0 = getPointer(self).TextBounds;
	return createPointer(result_0);
end
function abi.ffi.property_getter_text_label_text_color_3(self)
	local result_0 = getPointer(self).TextColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_text_label_text_color_3_set_text_color_3(self, value)
	getPointer(self).TextColor3 = getPointer(value);
end
function abi.ffi.property_getter_text_label_text_fits(self)
	local result_0 = getPointer(self).TextFits;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_text_label_text_scaled(self)
	local result_0 = getPointer(self).TextScaled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_label_text_scaled_set_text_scaled(self, value)
	getPointer(self).TextScaled = value == 1;
end
function abi.ffi.property_getter_text_label_text_size(self)
	local result_0 = getPointer(self).TextSize;
	return result_0;
end
function abi.ffi.property_setter_text_label_text_size_set_text_size(self, value)
	getPointer(self).TextSize = value;
end
function abi.ffi.property_getter_text_label_text_stroke_color_3(self)
	local result_0 = getPointer(self).TextStrokeColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_text_label_text_stroke_color_3_set_text_stroke_color_3(self, value)
	getPointer(self).TextStrokeColor3 = getPointer(value);
end
function abi.ffi.property_getter_text_label_text_stroke_transparency(self)
	local result_0 = getPointer(self).TextStrokeTransparency;
	return result_0;
end
function abi.ffi.property_setter_text_label_text_stroke_transparency_set_text_stroke_transparency(self, value)
	getPointer(self).TextStrokeTransparency = value;
end
function abi.ffi.property_getter_text_label_text_transparency(self)
	local result_0 = getPointer(self).TextTransparency;
	return result_0;
end
function abi.ffi.property_setter_text_label_text_transparency_set_text_transparency(self, value)
	getPointer(self).TextTransparency = value;
end
function abi.ffi.property_getter_text_label_text_truncate(self)
	local result_0 = getPointer(self).TextTruncate;
	return result_0;
end
function abi.ffi.property_setter_text_label_text_truncate_set_text_truncate(self, value)
	getPointer(self).TextTruncate = value;
end
function abi.ffi.property_getter_text_label_text_wrapped(self)
	local result_0 = getPointer(self).TextWrapped;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_label_text_wrapped_set_text_wrapped(self, value)
	getPointer(self).TextWrapped = value == 1;
end
function abi.ffi.property_getter_text_label_text_x_alignment(self)
	local result_0 = getPointer(self).TextXAlignment;
	return result_0;
end
function abi.ffi.property_setter_text_label_text_x_alignment_set_text_x_alignment(self, value)
	getPointer(self).TextXAlignment = value;
end
function abi.ffi.property_getter_text_label_text_y_alignment(self)
	local result_0 = getPointer(self).TextYAlignment;
	return result_0;
end
function abi.ffi.property_setter_text_label_text_y_alignment_set_text_y_alignment(self, value)
	getPointer(self).TextYAlignment = value;
end
function abi.ffi.property_getter_scrolling_frame_absolute_canvas_size(self)
	local result_0 = getPointer(self).AbsoluteCanvasSize;
	return createPointer(result_0);
end
function abi.ffi.property_getter_scrolling_frame_absolute_window_size(self)
	local result_0 = getPointer(self).AbsoluteWindowSize;
	return createPointer(result_0);
end
function abi.ffi.property_getter_scrolling_frame_automatic_canvas_size(self)
	local result_0 = getPointer(self).AutomaticCanvasSize;
	return result_0;
end
function abi.ffi.property_setter_scrolling_frame_automatic_canvas_size_set_automatic_canvas_size(self, value)
	getPointer(self).AutomaticCanvasSize = value;
end
function abi.ffi.property_getter_scrolling_frame_bottom_image(self)
	local result_0 = getPointer(self).BottomImage;
	return createPointer(result_0);
end
function abi.ffi.property_setter_scrolling_frame_bottom_image_set_bottom_image(self, value)
	getPointer(self).BottomImage = getPointer(value);
end
function abi.ffi.property_getter_scrolling_frame_canvas_position(self)
	local result_0 = getPointer(self).CanvasPosition;
	return createPointer(result_0);
end
function abi.ffi.property_setter_scrolling_frame_canvas_position_set_canvas_position(self, value)
	getPointer(self).CanvasPosition = getPointer(value);
end
function abi.ffi.property_getter_scrolling_frame_canvas_size(self)
	local result_0 = getPointer(self).CanvasSize;
	return createPointer(result_0);
end
function abi.ffi.property_setter_scrolling_frame_canvas_size_set_canvas_size(self, value)
	getPointer(self).CanvasSize = getPointer(value);
end
function abi.ffi.property_getter_scrolling_frame_elastic_behavior(self)
	local result_0 = getPointer(self).ElasticBehavior;
	return result_0;
end
function abi.ffi.property_setter_scrolling_frame_elastic_behavior_set_elastic_behavior(self, value)
	getPointer(self).ElasticBehavior = value;
end
function abi.ffi.property_getter_scrolling_frame_horizontal_scroll_bar_inset(self)
	local result_0 = getPointer(self).HorizontalScrollBarInset;
	return result_0;
end
function abi.ffi.property_setter_scrolling_frame_horizontal_scroll_bar_inset_set_horizontal_scroll_bar_inset(self, value)
	getPointer(self).HorizontalScrollBarInset = value;
end
function abi.ffi.property_getter_scrolling_frame_mid_image(self)
	local result_0 = getPointer(self).MidImage;
	return createPointer(result_0);
end
function abi.ffi.property_setter_scrolling_frame_mid_image_set_mid_image(self, value)
	getPointer(self).MidImage = getPointer(value);
end
function abi.ffi.property_getter_scrolling_frame_scroll_bar_image_color_3(self)
	local result_0 = getPointer(self).ScrollBarImageColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_scrolling_frame_scroll_bar_image_color_3_set_scroll_bar_image_color_3(self, value)
	getPointer(self).ScrollBarImageColor3 = getPointer(value);
end
function abi.ffi.property_getter_scrolling_frame_scroll_bar_image_transparency(self)
	local result_0 = getPointer(self).ScrollBarImageTransparency;
	return result_0;
end
function abi.ffi.property_setter_scrolling_frame_scroll_bar_image_transparency_set_scroll_bar_image_transparency(self, value)
	getPointer(self).ScrollBarImageTransparency = value;
end
function abi.ffi.property_getter_scrolling_frame_scroll_bar_thickness(self)
	local result_0 = getPointer(self).ScrollBarThickness;
	return result_0;
end
function abi.ffi.property_setter_scrolling_frame_scroll_bar_thickness_set_scroll_bar_thickness(self, value)
	getPointer(self).ScrollBarThickness = value;
end
function abi.ffi.property_getter_scrolling_frame_scrolling_direction(self)
	local result_0 = getPointer(self).ScrollingDirection;
	return result_0;
end
function abi.ffi.property_setter_scrolling_frame_scrolling_direction_set_scrolling_direction(self, value)
	getPointer(self).ScrollingDirection = value;
end
function abi.ffi.property_getter_scrolling_frame_scrolling_enabled(self)
	local result_0 = getPointer(self).ScrollingEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_scrolling_frame_scrolling_enabled_set_scrolling_enabled(self, value)
	getPointer(self).ScrollingEnabled = value == 1;
end
function abi.ffi.property_getter_scrolling_frame_top_image(self)
	local result_0 = getPointer(self).TopImage;
	return createPointer(result_0);
end
function abi.ffi.property_setter_scrolling_frame_top_image_set_top_image(self, value)
	getPointer(self).TopImage = getPointer(value);
end
function abi.ffi.property_getter_scrolling_frame_vertical_scroll_bar_inset(self)
	local result_0 = getPointer(self).VerticalScrollBarInset;
	return result_0;
end
function abi.ffi.property_setter_scrolling_frame_vertical_scroll_bar_inset_set_vertical_scroll_bar_inset(self, value)
	getPointer(self).VerticalScrollBarInset = value;
end
function abi.ffi.property_getter_scrolling_frame_vertical_scroll_bar_position(self)
	local result_0 = getPointer(self).VerticalScrollBarPosition;
	return result_0;
end
function abi.ffi.property_setter_scrolling_frame_vertical_scroll_bar_position_set_vertical_scroll_bar_position(self, value)
	getPointer(self).VerticalScrollBarPosition = value;
end
function abi.ffi.property_getter_text_box_clear_text_on_focus(self)
	local result_0 = getPointer(self).ClearTextOnFocus;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_box_clear_text_on_focus_set_clear_text_on_focus(self, value)
	getPointer(self).ClearTextOnFocus = value == 1;
end
function abi.ffi.property_getter_text_box_content_text(self)
	local result_0 = getPointer(self).ContentText;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_text_box_cursor_position(self)
	local result_0 = getPointer(self).CursorPosition;
	return result_0;
end
function abi.ffi.property_setter_text_box_cursor_position_set_cursor_position(self, value)
	getPointer(self).CursorPosition = value;
end
function abi.ffi.property_getter_text_box_font(self)
	local result_0 = getPointer(self).Font;
	return result_0;
end
function abi.ffi.property_setter_text_box_font_set_font(self, value)
	getPointer(self).Font = value;
end
function abi.ffi.property_getter_text_box_font_face(self)
	local result_0 = getPointer(self).FontFace;
	return createPointer(result_0);
end
function abi.ffi.property_setter_text_box_font_face_set_font_face(self, value)
	getPointer(self).FontFace = getPointer(value);
end
function abi.ffi.property_getter_text_box_line_height(self)
	local result_0 = getPointer(self).LineHeight;
	return result_0;
end
function abi.ffi.property_setter_text_box_line_height_set_line_height(self, value)
	getPointer(self).LineHeight = value;
end
function abi.ffi.property_getter_text_box_max_visible_graphemes(self)
	local result_0 = getPointer(self).MaxVisibleGraphemes;
	return result_0;
end
function abi.ffi.property_setter_text_box_max_visible_graphemes_set_max_visible_graphemes(self, value)
	getPointer(self).MaxVisibleGraphemes = value;
end
function abi.ffi.property_getter_text_box_multi_line(self)
	local result_0 = getPointer(self).MultiLine;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_box_multi_line_set_multi_line(self, value)
	getPointer(self).MultiLine = value == 1;
end
function abi.ffi.property_getter_text_box_placeholder_color_3(self)
	local result_0 = getPointer(self).PlaceholderColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_text_box_placeholder_color_3_set_placeholder_color_3(self, value)
	getPointer(self).PlaceholderColor3 = getPointer(value);
end
function abi.ffi.property_getter_text_box_placeholder_text(self)
	local result_0 = getPointer(self).PlaceholderText;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_text_box_placeholder_text_set_placeholder_text(self, value_addr, value_len)
	getPointer(self).PlaceholderText = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_text_box_rich_text(self)
	local result_0 = getPointer(self).RichText;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_box_rich_text_set_rich_text(self, value)
	getPointer(self).RichText = value == 1;
end
function abi.ffi.property_getter_text_box_selection_start(self)
	local result_0 = getPointer(self).SelectionStart;
	return result_0;
end
function abi.ffi.property_setter_text_box_selection_start_set_selection_start(self, value)
	getPointer(self).SelectionStart = value;
end
function abi.ffi.property_getter_text_box_show_native_input(self)
	local result_0 = getPointer(self).ShowNativeInput;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_box_show_native_input_set_show_native_input(self, value)
	getPointer(self).ShowNativeInput = value == 1;
end
function abi.ffi.property_getter_text_box_text(self)
	local result_0 = getPointer(self).Text;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_text_box_text_set_text(self, value_addr, value_len)
	getPointer(self).Text = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_text_box_text_bounds(self)
	local result_0 = getPointer(self).TextBounds;
	return createPointer(result_0);
end
function abi.ffi.property_getter_text_box_text_color_3(self)
	local result_0 = getPointer(self).TextColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_text_box_text_color_3_set_text_color_3(self, value)
	getPointer(self).TextColor3 = getPointer(value);
end
function abi.ffi.property_getter_text_box_text_editable(self)
	local result_0 = getPointer(self).TextEditable;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_box_text_editable_set_text_editable(self, value)
	getPointer(self).TextEditable = value == 1;
end
function abi.ffi.property_getter_text_box_text_fits(self)
	local result_0 = getPointer(self).TextFits;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_text_box_text_scaled(self)
	local result_0 = getPointer(self).TextScaled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_box_text_scaled_set_text_scaled(self, value)
	getPointer(self).TextScaled = value == 1;
end
function abi.ffi.property_getter_text_box_text_size(self)
	local result_0 = getPointer(self).TextSize;
	return result_0;
end
function abi.ffi.property_setter_text_box_text_size_set_text_size(self, value)
	getPointer(self).TextSize = value;
end
function abi.ffi.property_getter_text_box_text_stroke_color_3(self)
	local result_0 = getPointer(self).TextStrokeColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_text_box_text_stroke_color_3_set_text_stroke_color_3(self, value)
	getPointer(self).TextStrokeColor3 = getPointer(value);
end
function abi.ffi.property_getter_text_box_text_stroke_transparency(self)
	local result_0 = getPointer(self).TextStrokeTransparency;
	return result_0;
end
function abi.ffi.property_setter_text_box_text_stroke_transparency_set_text_stroke_transparency(self, value)
	getPointer(self).TextStrokeTransparency = value;
end
function abi.ffi.property_getter_text_box_text_transparency(self)
	local result_0 = getPointer(self).TextTransparency;
	return result_0;
end
function abi.ffi.property_setter_text_box_text_transparency_set_text_transparency(self, value)
	getPointer(self).TextTransparency = value;
end
function abi.ffi.property_getter_text_box_text_truncate(self)
	local result_0 = getPointer(self).TextTruncate;
	return result_0;
end
function abi.ffi.property_setter_text_box_text_truncate_set_text_truncate(self, value)
	getPointer(self).TextTruncate = value;
end
function abi.ffi.property_getter_text_box_text_wrapped(self)
	local result_0 = getPointer(self).TextWrapped;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_box_text_wrapped_set_text_wrapped(self, value)
	getPointer(self).TextWrapped = value == 1;
end
function abi.ffi.property_getter_text_box_text_x_alignment(self)
	local result_0 = getPointer(self).TextXAlignment;
	return result_0;
end
function abi.ffi.property_setter_text_box_text_x_alignment_set_text_x_alignment(self, value)
	getPointer(self).TextXAlignment = value;
end
function abi.ffi.property_getter_text_box_text_y_alignment(self)
	local result_0 = getPointer(self).TextYAlignment;
	return result_0;
end
function abi.ffi.property_setter_text_box_text_y_alignment_set_text_y_alignment(self, value)
	getPointer(self).TextYAlignment = value;
end
function abi.ffi.method_text_box_capture_focus(self)
	getPointer(self):CaptureFocus()
end
function abi.ffi.method_text_box_is_focused(self)
	local result_0 = getPointer(self):IsFocused();
	return result_0 and 1 or 0;
end
function abi.ffi.method_text_box_release_focus(self, submitted)
	getPointer(self):ReleaseFocus(submitted == 1)
end
function abi.ffi.event_text_box_focus_lost_on_focus_lost(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).FocusLost:Connect(function(enter_pressed, input_that_caused_focus_loss)
		local input_that_caused_focus_loss_optional, input_that_caused_focus_loss_optional_0 = 0, 0;
		if (input_that_caused_focus_loss ~= nil) then
			input_that_caused_focus_loss_optional, input_that_caused_focus_loss_optional_0 = 1, createPointer(input_that_caused_focus_loss);
		end
		invokeFunction(callback_data, callback_vtable, enter_pressed and 1 or 0, input_that_caused_focus_loss_optional, input_that_caused_focus_loss_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_text_box_focused_on_focused(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Focused:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_text_box_return_pressed_from_on_screen_keyboard_on_return_pressed_from_on_screen_keyboard(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).ReturnPressedFromOnScreenKeyboard:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_video_frame_is_loaded(self)
	local result_0 = getPointer(self).IsLoaded;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_video_frame_looped(self)
	local result_0 = getPointer(self).Looped;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_video_frame_looped_set_looped(self, value)
	getPointer(self).Looped = value == 1;
end
function abi.ffi.property_getter_video_frame_playing(self)
	local result_0 = getPointer(self).Playing;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_video_frame_playing_set_playing(self, value)
	getPointer(self).Playing = value == 1;
end
function abi.ffi.property_getter_video_frame_resolution(self)
	local result_0 = getPointer(self).Resolution;
	return createPointer(result_0);
end
function abi.ffi.property_getter_video_frame_time_length(self)
	local result_0 = getPointer(self).TimeLength;
	return result_0;
end
function abi.ffi.property_getter_video_frame_time_position(self)
	local result_0 = getPointer(self).TimePosition;
	return result_0;
end
function abi.ffi.property_setter_video_frame_time_position_set_time_position(self, value)
	getPointer(self).TimePosition = value;
end
function abi.ffi.property_getter_video_frame_video(self)
	local result_0 = getPointer(self).Video;
	return createPointer(result_0);
end
function abi.ffi.property_setter_video_frame_video_set_video(self, value)
	getPointer(self).Video = getPointer(value);
end
function abi.ffi.property_getter_video_frame_volume(self)
	local result_0 = getPointer(self).Volume;
	return result_0;
end
function abi.ffi.property_setter_video_frame_volume_set_volume(self, value)
	getPointer(self).Volume = value;
end
function abi.ffi.method_video_frame_pause(self)
	getPointer(self):Pause()
end
function abi.ffi.method_video_frame_play(self)
	getPointer(self):Play()
end
function abi.ffi.event_video_frame_did_loop_on_did_loop(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).DidLoop:Connect(function(video)
		local video_len = #video;
		local video_content = allocString(video_len);
		for i = 1, video_len do
			storeU8(memory, video_content + (i - 1), string.byte(video:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, video_content, video_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_video_frame_ended_on_ended(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Ended:Connect(function(video)
		local video_len = #video;
		local video_content = allocString(video_len);
		for i = 1, video_len do
			storeU8(memory, video_content + (i - 1), string.byte(video:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, video_content, video_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_video_frame_loaded_on_loaded(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Loaded:Connect(function(video)
		local video_len = #video;
		local video_content = allocString(video_len);
		for i = 1, video_len do
			storeU8(memory, video_content + (i - 1), string.byte(video:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, video_content, video_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_video_frame_paused_on_paused(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Paused:Connect(function(video)
		local video_len = #video;
		local video_content = allocString(video_len);
		for i = 1, video_len do
			storeU8(memory, video_content + (i - 1), string.byte(video:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, video_content, video_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_video_frame_played_on_played(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Played:Connect(function(video)
		local video_len = #video;
		local video_content = allocString(video_len);
		for i = 1, video_len do
			storeU8(memory, video_content + (i - 1), string.byte(video:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, video_content, video_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_viewport_frame_ambient(self)
	local result_0 = getPointer(self).Ambient;
	return createPointer(result_0);
end
function abi.ffi.property_setter_viewport_frame_ambient_set_ambient(self, value)
	getPointer(self).Ambient = getPointer(value);
end
function abi.ffi.property_getter_viewport_frame_current_camera(self)
	local result_0 = getPointer(self).CurrentCamera;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_viewport_frame_current_camera_set_current_camera(self, value)
	getPointer(self).CurrentCamera = getPointer(value);
end
function abi.ffi.property_getter_viewport_frame_image_color_3(self)
	local result_0 = getPointer(self).ImageColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_viewport_frame_image_color_3_set_image_color_3(self, value)
	getPointer(self).ImageColor3 = getPointer(value);
end
function abi.ffi.property_getter_viewport_frame_image_transparency(self)
	local result_0 = getPointer(self).ImageTransparency;
	return result_0;
end
function abi.ffi.property_setter_viewport_frame_image_transparency_set_image_transparency(self, value)
	getPointer(self).ImageTransparency = value;
end
function abi.ffi.property_getter_viewport_frame_light_color(self)
	local result_0 = getPointer(self).LightColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_viewport_frame_light_color_set_light_color(self, value)
	getPointer(self).LightColor = getPointer(value);
end
function abi.ffi.property_getter_viewport_frame_light_direction(self)
	local result_0 = getPointer(self).LightDirection;
	return createPointer(result_0);
end
function abi.ffi.property_setter_viewport_frame_light_direction_set_light_direction(self, value)
	getPointer(self).LightDirection = getPointer(value);
end
function abi.ffi.property_getter_layer_collector_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_layer_collector_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_layer_collector_reset_on_spawn(self)
	local result_0 = getPointer(self).ResetOnSpawn;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_layer_collector_reset_on_spawn_set_reset_on_spawn(self, value)
	getPointer(self).ResetOnSpawn = value == 1;
end
function abi.ffi.property_getter_layer_collector_z_index_behavior(self)
	local result_0 = getPointer(self).ZIndexBehavior;
	return result_0;
end
function abi.ffi.property_setter_layer_collector_z_index_behavior_set_z_index_behavior(self, value)
	getPointer(self).ZIndexBehavior = value;
end
function abi.ffi.property_getter_billboard_gui_active(self)
	local result_0 = getPointer(self).Active;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_billboard_gui_active_set_active(self, value)
	getPointer(self).Active = value == 1;
end
function abi.ffi.property_getter_billboard_gui_adornee(self)
	local result_0 = getPointer(self).Adornee;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_billboard_gui_adornee_set_adornee(self, value)
	getPointer(self).Adornee = getPointer(value);
end
function abi.ffi.property_getter_billboard_gui_always_on_top(self)
	local result_0 = getPointer(self).AlwaysOnTop;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_billboard_gui_always_on_top_set_always_on_top(self, value)
	getPointer(self).AlwaysOnTop = value == 1;
end
function abi.ffi.property_getter_billboard_gui_brightness(self)
	local result_0 = getPointer(self).Brightness;
	return result_0;
end
function abi.ffi.property_setter_billboard_gui_brightness_set_brightness(self, value)
	getPointer(self).Brightness = value;
end
function abi.ffi.property_getter_billboard_gui_clips_descendants(self)
	local result_0 = getPointer(self).ClipsDescendants;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_billboard_gui_clips_descendants_set_clips_descendants(self, value)
	getPointer(self).ClipsDescendants = value == 1;
end
function abi.ffi.property_getter_billboard_gui_current_distance(self)
	local result_0 = getPointer(self).CurrentDistance;
	return result_0;
end
function abi.ffi.property_getter_billboard_gui_distance_lower_limit(self)
	local result_0 = getPointer(self).DistanceLowerLimit;
	return result_0;
end
function abi.ffi.property_setter_billboard_gui_distance_lower_limit_set_distance_lower_limit(self, value)
	getPointer(self).DistanceLowerLimit = value;
end
function abi.ffi.property_getter_billboard_gui_distance_step(self)
	local result_0 = getPointer(self).DistanceStep;
	return result_0;
end
function abi.ffi.property_setter_billboard_gui_distance_step_set_distance_step(self, value)
	getPointer(self).DistanceStep = value;
end
function abi.ffi.property_getter_billboard_gui_distance_upper_limit(self)
	local result_0 = getPointer(self).DistanceUpperLimit;
	return result_0;
end
function abi.ffi.property_setter_billboard_gui_distance_upper_limit_set_distance_upper_limit(self, value)
	getPointer(self).DistanceUpperLimit = value;
end
function abi.ffi.property_getter_billboard_gui_extents_offset(self)
	local result_0 = getPointer(self).ExtentsOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_billboard_gui_extents_offset_set_extents_offset(self, value)
	getPointer(self).ExtentsOffset = getPointer(value);
end
function abi.ffi.property_getter_billboard_gui_extents_offset_world_space(self)
	local result_0 = getPointer(self).ExtentsOffsetWorldSpace;
	return createPointer(result_0);
end
function abi.ffi.property_setter_billboard_gui_extents_offset_world_space_set_extents_offset_world_space(self, value)
	getPointer(self).ExtentsOffsetWorldSpace = getPointer(value);
end
function abi.ffi.property_getter_billboard_gui_light_influence(self)
	local result_0 = getPointer(self).LightInfluence;
	return result_0;
end
function abi.ffi.property_setter_billboard_gui_light_influence_set_light_influence(self, value)
	getPointer(self).LightInfluence = value;
end
function abi.ffi.property_getter_billboard_gui_max_distance(self)
	local result_0 = getPointer(self).MaxDistance;
	return result_0;
end
function abi.ffi.property_setter_billboard_gui_max_distance_set_max_distance(self, value)
	getPointer(self).MaxDistance = value;
end
function abi.ffi.property_getter_billboard_gui_player_to_hide_from(self)
	local result_0 = getPointer(self).PlayerToHideFrom;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_billboard_gui_player_to_hide_from_set_player_to_hide_from(self, value)
	getPointer(self).PlayerToHideFrom = getPointer(value);
end
function abi.ffi.property_getter_billboard_gui_size(self)
	local result_0 = getPointer(self).Size;
	return createPointer(result_0);
end
function abi.ffi.property_setter_billboard_gui_size_set_size(self, value)
	getPointer(self).Size = getPointer(value);
end
function abi.ffi.property_getter_billboard_gui_size_offset(self)
	local result_0 = getPointer(self).SizeOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_billboard_gui_size_offset_set_size_offset(self, value)
	getPointer(self).SizeOffset = getPointer(value);
end
function abi.ffi.property_getter_billboard_gui_studs_offset(self)
	local result_0 = getPointer(self).StudsOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_billboard_gui_studs_offset_set_studs_offset(self, value)
	getPointer(self).StudsOffset = getPointer(value);
end
function abi.ffi.property_getter_billboard_gui_studs_offset_world_space(self)
	local result_0 = getPointer(self).StudsOffsetWorldSpace;
	return createPointer(result_0);
end
function abi.ffi.property_setter_billboard_gui_studs_offset_world_space_set_studs_offset_world_space(self, value)
	getPointer(self).StudsOffsetWorldSpace = getPointer(value);
end
function abi.ffi.property_getter_screen_gui_clip_to_device_safe_area(self)
	local result_0 = getPointer(self).ClipToDeviceSafeArea;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_screen_gui_clip_to_device_safe_area_set_clip_to_device_safe_area(self, value)
	getPointer(self).ClipToDeviceSafeArea = value == 1;
end
function abi.ffi.property_getter_screen_gui_display_order(self)
	local result_0 = getPointer(self).DisplayOrder;
	return result_0;
end
function abi.ffi.property_setter_screen_gui_display_order_set_display_order(self, value)
	getPointer(self).DisplayOrder = value;
end
function abi.ffi.property_getter_screen_gui_ignore_gui_inset(self)
	local result_0 = getPointer(self).IgnoreGuiInset;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_screen_gui_ignore_gui_inset_set_ignore_gui_inset(self, value)
	getPointer(self).IgnoreGuiInset = value == 1;
end
function abi.ffi.property_getter_screen_gui_safe_area_compatibility(self)
	local result_0 = getPointer(self).SafeAreaCompatibility;
	return result_0;
end
function abi.ffi.property_setter_screen_gui_safe_area_compatibility_set_safe_area_compatibility(self, value)
	getPointer(self).SafeAreaCompatibility = value;
end
function abi.ffi.property_getter_surface_gui_base_active(self)
	local result_0 = getPointer(self).Active;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_surface_gui_base_active_set_active(self, value)
	getPointer(self).Active = value == 1;
end
function abi.ffi.property_getter_surface_gui_base_adornee(self)
	local result_0 = getPointer(self).Adornee;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_surface_gui_base_adornee_set_adornee(self, value)
	getPointer(self).Adornee = getPointer(value);
end
function abi.ffi.property_getter_surface_gui_base_face(self)
	local result_0 = getPointer(self).Face;
	return result_0;
end
function abi.ffi.property_setter_surface_gui_base_face_set_face(self, value)
	getPointer(self).Face = value;
end
function abi.ffi.property_getter_ad_gui_ad_shape(self)
	local result_0 = getPointer(self).AdShape;
	return result_0;
end
function abi.ffi.property_setter_ad_gui_ad_shape_set_ad_shape(self, value)
	getPointer(self).AdShape = value;
end
function abi.ffi.property_getter_surface_gui_always_on_top(self)
	local result_0 = getPointer(self).AlwaysOnTop;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_surface_gui_always_on_top_set_always_on_top(self, value)
	getPointer(self).AlwaysOnTop = value == 1;
end
function abi.ffi.property_getter_surface_gui_brightness(self)
	local result_0 = getPointer(self).Brightness;
	return result_0;
end
function abi.ffi.property_setter_surface_gui_brightness_set_brightness(self, value)
	getPointer(self).Brightness = value;
end
function abi.ffi.property_getter_surface_gui_canvas_size(self)
	local result_0 = getPointer(self).CanvasSize;
	return createPointer(result_0);
end
function abi.ffi.property_setter_surface_gui_canvas_size_set_canvas_size(self, value)
	getPointer(self).CanvasSize = getPointer(value);
end
function abi.ffi.property_getter_surface_gui_clips_descendants(self)
	local result_0 = getPointer(self).ClipsDescendants;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_surface_gui_clips_descendants_set_clips_descendants(self, value)
	getPointer(self).ClipsDescendants = value == 1;
end
function abi.ffi.property_getter_surface_gui_light_influence(self)
	local result_0 = getPointer(self).LightInfluence;
	return result_0;
end
function abi.ffi.property_setter_surface_gui_light_influence_set_light_influence(self, value)
	getPointer(self).LightInfluence = value;
end
function abi.ffi.property_getter_surface_gui_pixels_per_stud(self)
	local result_0 = getPointer(self).PixelsPerStud;
	return result_0;
end
function abi.ffi.property_setter_surface_gui_pixels_per_stud_set_pixels_per_stud(self, value)
	getPointer(self).PixelsPerStud = value;
end
function abi.ffi.property_getter_surface_gui_sizing_mode(self)
	local result_0 = getPointer(self).SizingMode;
	return result_0;
end
function abi.ffi.property_setter_surface_gui_sizing_mode_set_sizing_mode(self, value)
	getPointer(self).SizingMode = value;
end
function abi.ffi.property_getter_surface_gui_tool_punch_through_distance(self)
	local result_0 = getPointer(self).ToolPunchThroughDistance;
	return result_0;
end
function abi.ffi.property_setter_surface_gui_tool_punch_through_distance_set_tool_punch_through_distance(self, value)
	getPointer(self).ToolPunchThroughDistance = value;
end
function abi.ffi.property_getter_surface_gui_z_offset(self)
	local result_0 = getPointer(self).ZOffset;
	return result_0;
end
function abi.ffi.property_setter_surface_gui_z_offset_set_z_offset(self, value)
	getPointer(self).ZOffset = value;
end
function abi.ffi.property_getter_gui_base_3_d_color_3(self)
	local result_0 = getPointer(self).Color3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_gui_base_3_d_color_3_set_color_3(self, value)
	getPointer(self).Color3 = getPointer(value);
end
function abi.ffi.property_getter_gui_base_3_d_transparency(self)
	local result_0 = getPointer(self).Transparency;
	return result_0;
end
function abi.ffi.property_setter_gui_base_3_d_transparency_set_transparency(self, value)
	getPointer(self).Transparency = value;
end
function abi.ffi.property_getter_gui_base_3_d_visible(self)
	local result_0 = getPointer(self).Visible;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_base_3_d_visible_set_visible(self, value)
	getPointer(self).Visible = value == 1;
end
function abi.ffi.property_getter_instance_adornment_adornee(self)
	local result_0 = getPointer(self).Adornee;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_instance_adornment_adornee_set_adornee(self, value)
	getPointer(self).Adornee = getPointer(value);
end
function abi.ffi.property_getter_selection_box_line_thickness(self)
	local result_0 = getPointer(self).LineThickness;
	return result_0;
end
function abi.ffi.property_setter_selection_box_line_thickness_set_line_thickness(self, value)
	getPointer(self).LineThickness = value;
end
function abi.ffi.property_getter_selection_box_surface_color_3(self)
	local result_0 = getPointer(self).SurfaceColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_selection_box_surface_color_3_set_surface_color_3(self, value)
	getPointer(self).SurfaceColor3 = getPointer(value);
end
function abi.ffi.property_getter_selection_box_surface_transparency(self)
	local result_0 = getPointer(self).SurfaceTransparency;
	return result_0;
end
function abi.ffi.property_setter_selection_box_surface_transparency_set_surface_transparency(self, value)
	getPointer(self).SurfaceTransparency = value;
end
function abi.ffi.property_getter_pv_adornment_adornee(self)
	local result_0 = getPointer(self).Adornee;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_pv_adornment_adornee_set_adornee(self, value)
	getPointer(self).Adornee = getPointer(value);
end
function abi.ffi.property_getter_handle_adornment_adorn_culling_mode(self)
	local result_0 = getPointer(self).AdornCullingMode;
	return result_0;
end
function abi.ffi.property_setter_handle_adornment_adorn_culling_mode_set_adorn_culling_mode(self, value)
	getPointer(self).AdornCullingMode = value;
end
function abi.ffi.property_getter_handle_adornment_always_on_top(self)
	local result_0 = getPointer(self).AlwaysOnTop;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_handle_adornment_always_on_top_set_always_on_top(self, value)
	getPointer(self).AlwaysOnTop = value == 1;
end
function abi.ffi.property_getter_handle_adornment_c_frame(self)
	local result_0 = getPointer(self).CFrame;
	return createPointer(result_0);
end
function abi.ffi.property_setter_handle_adornment_c_frame_set_c_frame(self, value)
	getPointer(self).CFrame = getPointer(value);
end
function abi.ffi.property_getter_handle_adornment_size_relative_offset(self)
	local result_0 = getPointer(self).SizeRelativeOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_handle_adornment_size_relative_offset_set_size_relative_offset(self, value)
	getPointer(self).SizeRelativeOffset = getPointer(value);
end
function abi.ffi.property_getter_handle_adornment_z_index(self)
	local result_0 = getPointer(self).ZIndex;
	return result_0;
end
function abi.ffi.property_setter_handle_adornment_z_index_set_z_index(self, value)
	getPointer(self).ZIndex = value;
end
function abi.ffi.event_handle_adornment_mouse_button_1_down_on_mouse_button_1_down(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseButton1Down:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_handle_adornment_mouse_button_1_up_on_mouse_button_1_up(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseButton1Up:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_handle_adornment_mouse_enter_on_mouse_enter(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseEnter:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_handle_adornment_mouse_leave_on_mouse_leave(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseLeave:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_box_handle_adornment_size(self)
	local result_0 = getPointer(self).Size;
	return createPointer(result_0);
end
function abi.ffi.property_setter_box_handle_adornment_size_set_size(self, value)
	getPointer(self).Size = getPointer(value);
end
function abi.ffi.property_getter_cone_handle_adornment_height(self)
	local result_0 = getPointer(self).Height;
	return result_0;
end
function abi.ffi.property_setter_cone_handle_adornment_height_set_height(self, value)
	getPointer(self).Height = value;
end
function abi.ffi.property_getter_cone_handle_adornment_radius(self)
	local result_0 = getPointer(self).Radius;
	return result_0;
end
function abi.ffi.property_setter_cone_handle_adornment_radius_set_radius(self, value)
	getPointer(self).Radius = value;
end
function abi.ffi.property_getter_cylinder_handle_adornment_angle(self)
	local result_0 = getPointer(self).Angle;
	return result_0;
end
function abi.ffi.property_setter_cylinder_handle_adornment_angle_set_angle(self, value)
	getPointer(self).Angle = value;
end
function abi.ffi.property_getter_cylinder_handle_adornment_height(self)
	local result_0 = getPointer(self).Height;
	return result_0;
end
function abi.ffi.property_setter_cylinder_handle_adornment_height_set_height(self, value)
	getPointer(self).Height = value;
end
function abi.ffi.property_getter_cylinder_handle_adornment_inner_radius(self)
	local result_0 = getPointer(self).InnerRadius;
	return result_0;
end
function abi.ffi.property_setter_cylinder_handle_adornment_inner_radius_set_inner_radius(self, value)
	getPointer(self).InnerRadius = value;
end
function abi.ffi.property_getter_cylinder_handle_adornment_radius(self)
	local result_0 = getPointer(self).Radius;
	return result_0;
end
function abi.ffi.property_setter_cylinder_handle_adornment_radius_set_radius(self, value)
	getPointer(self).Radius = value;
end
function abi.ffi.property_getter_image_handle_adornment_image(self)
	local result_0 = getPointer(self).Image;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_handle_adornment_image_set_image(self, value)
	getPointer(self).Image = getPointer(value);
end
function abi.ffi.property_getter_image_handle_adornment_size(self)
	local result_0 = getPointer(self).Size;
	return createPointer(result_0);
end
function abi.ffi.property_setter_image_handle_adornment_size_set_size(self, value)
	getPointer(self).Size = getPointer(value);
end
function abi.ffi.property_getter_line_handle_adornment_length(self)
	local result_0 = getPointer(self).Length;
	return result_0;
end
function abi.ffi.property_setter_line_handle_adornment_length_set_length(self, value)
	getPointer(self).Length = value;
end
function abi.ffi.property_getter_line_handle_adornment_thickness(self)
	local result_0 = getPointer(self).Thickness;
	return result_0;
end
function abi.ffi.property_setter_line_handle_adornment_thickness_set_thickness(self, value)
	getPointer(self).Thickness = value;
end
function abi.ffi.property_getter_sphere_handle_adornment_radius(self)
	local result_0 = getPointer(self).Radius;
	return result_0;
end
function abi.ffi.property_setter_sphere_handle_adornment_radius_set_radius(self, value)
	getPointer(self).Radius = value;
end
function abi.ffi.method_wireframe_handle_adornment_add_line(self, from, to)
	getPointer(self):AddLine(getPointer(from), getPointer(to))
end
function abi.ffi.method_wireframe_handle_adornment_clear(self)
	getPointer(self):Clear()
end
function abi.ffi.property_getter_selection_sphere_surface_color_3(self)
	local result_0 = getPointer(self).SurfaceColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_selection_sphere_surface_color_3_set_surface_color_3(self, value)
	getPointer(self).SurfaceColor3 = getPointer(value);
end
function abi.ffi.property_getter_selection_sphere_surface_transparency(self)
	local result_0 = getPointer(self).SurfaceTransparency;
	return result_0;
end
function abi.ffi.property_setter_selection_sphere_surface_transparency_set_surface_transparency(self, value)
	getPointer(self).SurfaceTransparency = value;
end
function abi.ffi.property_getter_part_adornment_adornee(self)
	local result_0 = getPointer(self).Adornee;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_part_adornment_adornee_set_adornee(self, value)
	getPointer(self).Adornee = getPointer(value);
end
function abi.ffi.property_getter_arc_handles_axes(self)
	local result_0 = getPointer(self).Axes;
	return createPointer(result_0);
end
function abi.ffi.property_setter_arc_handles_axes_set_axes(self, value)
	getPointer(self).Axes = getPointer(value);
end
function abi.ffi.event_arc_handles_mouse_button_1_down_on_mouse_button_1_down(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseButton1Down:Connect(function(axis)
		invokeFunction(callback_data, callback_vtable, axis)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_arc_handles_mouse_button_1_up_on_mouse_button_1_up(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseButton1Up:Connect(function(axis)
		invokeFunction(callback_data, callback_vtable, axis)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_arc_handles_mouse_drag_on_mouse_drag(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseDrag:Connect(function(axis, relative_angle, delta_radius)
		invokeFunction(callback_data, callback_vtable, axis, relative_angle, delta_radius)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_arc_handles_mouse_enter_on_mouse_enter(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseEnter:Connect(function(axis)
		invokeFunction(callback_data, callback_vtable, axis)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_arc_handles_mouse_leave_on_mouse_leave(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseLeave:Connect(function(axis)
		invokeFunction(callback_data, callback_vtable, axis)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_handles_faces(self)
	local result_0 = getPointer(self).Faces;
	return createPointer(result_0);
end
function abi.ffi.property_setter_handles_faces_set_faces(self, value)
	getPointer(self).Faces = getPointer(value);
end
function abi.ffi.property_getter_handles_style(self)
	local result_0 = getPointer(self).Style;
	return result_0;
end
function abi.ffi.property_setter_handles_style_set_style(self, value)
	getPointer(self).Style = value;
end
function abi.ffi.event_handles_mouse_button_1_down_on_mouse_button_1_down(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseButton1Down:Connect(function(face)
		invokeFunction(callback_data, callback_vtable, face)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_handles_mouse_button_1_up_on_mouse_button_1_up(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseButton1Up:Connect(function(face)
		invokeFunction(callback_data, callback_vtable, face)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_handles_mouse_drag_on_mouse_drag(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseDrag:Connect(function(face, distance)
		invokeFunction(callback_data, callback_vtable, face, distance)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_handles_mouse_enter_on_mouse_enter(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseEnter:Connect(function(face)
		invokeFunction(callback_data, callback_vtable, face)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_handles_mouse_leave_on_mouse_leave(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MouseLeave:Connect(function(face)
		invokeFunction(callback_data, callback_vtable, face)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_surface_selection_target_surface(self)
	local result_0 = getPointer(self).TargetSurface;
	return result_0;
end
function abi.ffi.property_setter_surface_selection_target_surface_set_target_surface(self, value)
	getPointer(self).TargetSurface = value;
end
function abi.ffi.property_getter_selection_lasso_humanoid(self)
	local result_0 = getPointer(self).Humanoid;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_selection_lasso_humanoid_set_humanoid(self, value)
	getPointer(self).Humanoid = getPointer(value);
end
function abi.ffi.property_getter_gui_service_auto_select_gui_enabled(self)
	local result_0 = getPointer(self).AutoSelectGuiEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_service_auto_select_gui_enabled_set_auto_select_gui_enabled(self, value)
	getPointer(self).AutoSelectGuiEnabled = value == 1;
end
function abi.ffi.property_getter_gui_service_core_gui_navigation_enabled(self)
	local result_0 = getPointer(self).CoreGuiNavigationEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_service_core_gui_navigation_enabled_set_core_gui_navigation_enabled(self, value)
	getPointer(self).CoreGuiNavigationEnabled = value == 1;
end
function abi.ffi.property_getter_gui_service_gui_navigation_enabled(self)
	local result_0 = getPointer(self).GuiNavigationEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_service_gui_navigation_enabled_set_gui_navigation_enabled(self, value)
	getPointer(self).GuiNavigationEnabled = value == 1;
end
function abi.ffi.property_getter_gui_service_menu_is_open(self)
	local result_0 = getPointer(self).MenuIsOpen;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_gui_service_selected_object(self)
	local result_0 = getPointer(self).SelectedObject;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_gui_service_selected_object_set_selected_object(self, value)
	getPointer(self).SelectedObject = getPointer(value);
end
function abi.ffi.property_getter_gui_service_touch_controls_enabled(self)
	local result_0 = getPointer(self).TouchControlsEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_gui_service_touch_controls_enabled_set_touch_controls_enabled(self, value)
	getPointer(self).TouchControlsEnabled = value == 1;
end
function abi.ffi.method_gui_service_add_selection_parent(self, selection_name_addr, selection_name_len, selection_parent)
	getPointer(self):AddSelectionParent(loadString(memory, selection_name_addr, selection_name_len), getPointer(selection_parent))
end
function abi.ffi.method_gui_service_close_inspect_menu(self)
	getPointer(self):CloseInspectMenu()
end
function abi.ffi.method_gui_service_get_emotes_menu_open(self)
	local result_0 = getPointer(self):GetEmotesMenuOpen();
	return result_0 and 1 or 0;
end
function abi.ffi.method_gui_service_get_gameplay_paused_notification_enabled(self)
	local result_0 = getPointer(self):GetGameplayPausedNotificationEnabled();
	return result_0 and 1 or 0;
end
function abi.ffi.method_gui_service_get_inspect_menu_enabled(self)
	local result_0 = getPointer(self):GetInspectMenuEnabled();
	return result_0 and 1 or 0;
end
function abi.ffi.method_gui_service_inspect_player_from_humanoid_description(self, humanoid_description, name_addr, name_len)
	getPointer(self):InspectPlayerFromHumanoidDescription(getPointer(humanoid_description), loadString(memory, name_addr, name_len))
end
function abi.ffi.method_gui_service_inspect_player_from_user_id(self, user_id)
	getPointer(self):InspectPlayerFromUserId(user_id)
end
function abi.ffi.method_gui_service_is_ten_foot_interface(self)
	local result_0 = getPointer(self):IsTenFootInterface();
	return result_0 and 1 or 0;
end
function abi.ffi.method_gui_service_remove_selection_group(self, selection_name_addr, selection_name_len)
	getPointer(self):RemoveSelectionGroup(loadString(memory, selection_name_addr, selection_name_len))
end
function abi.ffi.method_gui_service_select(self, selection_parent)
	getPointer(self):Select(getPointer(selection_parent))
end
function abi.ffi.method_gui_service_set_emotes_menu_open(self, is_open)
	getPointer(self):SetEmotesMenuOpen(is_open == 1)
end
function abi.ffi.method_gui_service_set_gameplay_paused_notification_enabled(self, enabled)
	getPointer(self):SetGameplayPausedNotificationEnabled(enabled == 1)
end
function abi.ffi.method_gui_service_set_inspect_menu_enabled(self, enabled)
	getPointer(self):SetInspectMenuEnabled(enabled == 1)
end
function abi.ffi.event_gui_service_menu_closed_on_menu_closed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MenuClosed:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_gui_service_menu_opened_on_menu_opened(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MenuOpened:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.method_haptic_service_is_motor_supported(self, input_type, vibration_motor)
	local result_0 = getPointer(self):IsMotorSupported(input_type, vibration_motor);
	return result_0 and 1 or 0;
end
function abi.ffi.method_haptic_service_is_vibration_supported(self, input_type)
	local result_0 = getPointer(self):IsVibrationSupported(input_type);
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_highlight_adornee(self)
	local result_0 = getPointer(self).Adornee;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_highlight_adornee_set_adornee(self, value)
	getPointer(self).Adornee = getPointer(value);
end
function abi.ffi.property_getter_highlight_depth_mode(self)
	local result_0 = getPointer(self).DepthMode;
	return result_0;
end
function abi.ffi.property_setter_highlight_depth_mode_set_depth_mode(self, value)
	getPointer(self).DepthMode = value;
end
function abi.ffi.property_getter_highlight_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_highlight_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_highlight_fill_color(self)
	local result_0 = getPointer(self).FillColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_highlight_fill_color_set_fill_color(self, value)
	getPointer(self).FillColor = getPointer(value);
end
function abi.ffi.property_getter_highlight_fill_transparency(self)
	local result_0 = getPointer(self).FillTransparency;
	return result_0;
end
function abi.ffi.property_setter_highlight_fill_transparency_set_fill_transparency(self, value)
	getPointer(self).FillTransparency = value;
end
function abi.ffi.property_getter_highlight_outline_color(self)
	local result_0 = getPointer(self).OutlineColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_highlight_outline_color_set_outline_color(self, value)
	getPointer(self).OutlineColor = getPointer(value);
end
function abi.ffi.property_getter_highlight_outline_transparency(self)
	local result_0 = getPointer(self).OutlineTransparency;
	return result_0;
end
function abi.ffi.property_setter_highlight_outline_transparency_set_outline_transparency(self, value)
	getPointer(self).OutlineTransparency = value;
end
function abi.ffi.method_http_service_generate_guid(self, wrap_in_curly_braces)
	local result_0 = getPointer(self):GenerateGUID(wrap_in_curly_braces == 1);
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_http_service_url_encode(self, input_addr, input_len)
	local result_0 = getPointer(self):UrlEncode(loadString(memory, input_addr, input_len));
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_humanoid_auto_jump_enabled(self)
	local result_0 = getPointer(self).AutoJumpEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_humanoid_auto_jump_enabled_set_auto_jump_enabled(self, value)
	getPointer(self).AutoJumpEnabled = value == 1;
end
function abi.ffi.property_getter_humanoid_auto_rotate(self)
	local result_0 = getPointer(self).AutoRotate;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_humanoid_auto_rotate_set_auto_rotate(self, value)
	getPointer(self).AutoRotate = value == 1;
end
function abi.ffi.property_getter_humanoid_automatic_scaling_enabled(self)
	local result_0 = getPointer(self).AutomaticScalingEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_humanoid_automatic_scaling_enabled_set_automatic_scaling_enabled(self, value)
	getPointer(self).AutomaticScalingEnabled = value == 1;
end
function abi.ffi.property_getter_humanoid_break_joints_on_death(self)
	local result_0 = getPointer(self).BreakJointsOnDeath;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_humanoid_break_joints_on_death_set_break_joints_on_death(self, value)
	getPointer(self).BreakJointsOnDeath = value == 1;
end
function abi.ffi.property_getter_humanoid_camera_offset(self)
	local result_0 = getPointer(self).CameraOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_humanoid_camera_offset_set_camera_offset(self, value)
	getPointer(self).CameraOffset = getPointer(value);
end
function abi.ffi.property_getter_humanoid_display_distance_type(self)
	local result_0 = getPointer(self).DisplayDistanceType;
	return result_0;
end
function abi.ffi.property_setter_humanoid_display_distance_type_set_display_distance_type(self, value)
	getPointer(self).DisplayDistanceType = value;
end
function abi.ffi.property_getter_humanoid_display_name(self)
	local result_0 = getPointer(self).DisplayName;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_humanoid_display_name_set_display_name(self, value_addr, value_len)
	getPointer(self).DisplayName = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_humanoid_evaluate_state_machine(self)
	local result_0 = getPointer(self).EvaluateStateMachine;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_humanoid_evaluate_state_machine_set_evaluate_state_machine(self, value)
	getPointer(self).EvaluateStateMachine = value == 1;
end
function abi.ffi.property_getter_humanoid_floor_material(self)
	local result_0 = getPointer(self).FloorMaterial;
	return result_0;
end
function abi.ffi.property_getter_humanoid_health(self)
	local result_0 = getPointer(self).Health;
	return result_0;
end
function abi.ffi.property_setter_humanoid_health_set_health(self, value)
	getPointer(self).Health = value;
end
function abi.ffi.property_getter_humanoid_health_display_distance(self)
	local result_0 = getPointer(self).HealthDisplayDistance;
	return result_0;
end
function abi.ffi.property_setter_humanoid_health_display_distance_set_health_display_distance(self, value)
	getPointer(self).HealthDisplayDistance = value;
end
function abi.ffi.property_getter_humanoid_health_display_type(self)
	local result_0 = getPointer(self).HealthDisplayType;
	return result_0;
end
function abi.ffi.property_setter_humanoid_health_display_type_set_health_display_type(self, value)
	getPointer(self).HealthDisplayType = value;
end
function abi.ffi.property_getter_humanoid_hip_height(self)
	local result_0 = getPointer(self).HipHeight;
	return result_0;
end
function abi.ffi.property_setter_humanoid_hip_height_set_hip_height(self, value)
	getPointer(self).HipHeight = value;
end
function abi.ffi.property_getter_humanoid_jump(self)
	local result_0 = getPointer(self).Jump;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_humanoid_jump_set_jump(self, value)
	getPointer(self).Jump = value == 1;
end
function abi.ffi.property_getter_humanoid_jump_height(self)
	local result_0 = getPointer(self).JumpHeight;
	return result_0;
end
function abi.ffi.property_setter_humanoid_jump_height_set_jump_height(self, value)
	getPointer(self).JumpHeight = value;
end
function abi.ffi.property_getter_humanoid_jump_power(self)
	local result_0 = getPointer(self).JumpPower;
	return result_0;
end
function abi.ffi.property_setter_humanoid_jump_power_set_jump_power(self, value)
	getPointer(self).JumpPower = value;
end
function abi.ffi.property_getter_humanoid_max_health(self)
	local result_0 = getPointer(self).MaxHealth;
	return result_0;
end
function abi.ffi.property_setter_humanoid_max_health_set_max_health(self, value)
	getPointer(self).MaxHealth = value;
end
function abi.ffi.property_getter_humanoid_max_slope_angle(self)
	local result_0 = getPointer(self).MaxSlopeAngle;
	return result_0;
end
function abi.ffi.property_setter_humanoid_max_slope_angle_set_max_slope_angle(self, value)
	getPointer(self).MaxSlopeAngle = value;
end
function abi.ffi.property_getter_humanoid_move_direction(self)
	local result_0 = getPointer(self).MoveDirection;
	return createPointer(result_0);
end
function abi.ffi.property_getter_humanoid_name_display_distance(self)
	local result_0 = getPointer(self).NameDisplayDistance;
	return result_0;
end
function abi.ffi.property_setter_humanoid_name_display_distance_set_name_display_distance(self, value)
	getPointer(self).NameDisplayDistance = value;
end
function abi.ffi.property_getter_humanoid_name_occlusion(self)
	local result_0 = getPointer(self).NameOcclusion;
	return result_0;
end
function abi.ffi.property_setter_humanoid_name_occlusion_set_name_occlusion(self, value)
	getPointer(self).NameOcclusion = value;
end
function abi.ffi.property_getter_humanoid_platform_stand(self)
	local result_0 = getPointer(self).PlatformStand;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_humanoid_platform_stand_set_platform_stand(self, value)
	getPointer(self).PlatformStand = value == 1;
end
function abi.ffi.property_getter_humanoid_requires_neck(self)
	local result_0 = getPointer(self).RequiresNeck;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_humanoid_requires_neck_set_requires_neck(self, value)
	getPointer(self).RequiresNeck = value == 1;
end
function abi.ffi.property_getter_humanoid_rig_type(self)
	local result_0 = getPointer(self).RigType;
	return result_0;
end
function abi.ffi.property_setter_humanoid_rig_type_set_rig_type(self, value)
	getPointer(self).RigType = value;
end
function abi.ffi.property_getter_humanoid_root_part(self)
	local result_0 = getPointer(self).RootPart;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_humanoid_seat_part(self)
	local result_0 = getPointer(self).SeatPart;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_humanoid_sit(self)
	local result_0 = getPointer(self).Sit;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_humanoid_sit_set_sit(self, value)
	getPointer(self).Sit = value == 1;
end
function abi.ffi.property_getter_humanoid_target_point(self)
	local result_0 = getPointer(self).TargetPoint;
	return createPointer(result_0);
end
function abi.ffi.property_setter_humanoid_target_point_set_target_point(self, value)
	getPointer(self).TargetPoint = getPointer(value);
end
function abi.ffi.property_getter_humanoid_use_jump_power(self)
	local result_0 = getPointer(self).UseJumpPower;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_humanoid_use_jump_power_set_use_jump_power(self, value)
	getPointer(self).UseJumpPower = value == 1;
end
function abi.ffi.property_getter_humanoid_walk_speed(self)
	local result_0 = getPointer(self).WalkSpeed;
	return result_0;
end
function abi.ffi.property_setter_humanoid_walk_speed_set_walk_speed(self, value)
	getPointer(self).WalkSpeed = value;
end
function abi.ffi.property_getter_humanoid_walk_to_part(self)
	local result_0 = getPointer(self).WalkToPart;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_humanoid_walk_to_part_set_walk_to_part(self, value)
	getPointer(self).WalkToPart = getPointer(value);
end
function abi.ffi.property_getter_humanoid_walk_to_point(self)
	local result_0 = getPointer(self).WalkToPoint;
	return createPointer(result_0);
end
function abi.ffi.property_setter_humanoid_walk_to_point_set_walk_to_point(self, value)
	getPointer(self).WalkToPoint = getPointer(value);
end
function abi.ffi.method_humanoid_add_accessory(self, accessory)
	getPointer(self):AddAccessory(getPointer(accessory))
end
function abi.ffi.method_humanoid_build_rig_from_attachments(self)
	getPointer(self):BuildRigFromAttachments()
end
function abi.ffi.method_humanoid_change_state(self, state)
	getPointer(self):ChangeState(state)
end
function abi.ffi.method_humanoid_equip_tool(self, tool)
	getPointer(self):EquipTool(getPointer(tool))
end
function abi.ffi.method_humanoid_get_applied_description(self)
	local result_0 = getPointer(self):GetAppliedDescription();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_humanoid_get_body_part_r_15(self, part)
	local result_0 = getPointer(self):GetBodyPartR15(getPointer(part));
	return result_0;
end
function abi.ffi.method_humanoid_get_limb(self, part)
	local result_0 = getPointer(self):GetLimb(getPointer(part));
	return result_0;
end
function abi.ffi.method_humanoid_get_state(self)
	local result_0 = getPointer(self):GetState();
	return result_0;
end
function abi.ffi.method_humanoid_get_state_enabled(self, state)
	local result_0 = getPointer(self):GetStateEnabled(state);
	return result_0 and 1 or 0;
end
function abi.ffi.method_humanoid_move(self, move_direction, relative_to_camera)
	getPointer(self):Move(getPointer(move_direction), relative_to_camera == 1)
end
function abi.ffi.method_humanoid_move_to(self, location, part)
	getPointer(self):MoveTo(getPointer(location), getPointer(part))
end
function abi.ffi.method_humanoid_remove_accessories(self)
	getPointer(self):RemoveAccessories()
end
function abi.ffi.method_humanoid_replace_body_part_r_15(self, body_part, part)
	local result_0 = getPointer(self):ReplaceBodyPartR15(body_part, getPointer(part));
	return result_0 and 1 or 0;
end
function abi.ffi.method_humanoid_set_state_enabled(self, state, enabled)
	getPointer(self):SetStateEnabled(state, enabled == 1)
end
function abi.ffi.method_humanoid_take_damage(self, amount)
	getPointer(self):TakeDamage(amount)
end
function abi.ffi.method_humanoid_unequip_tools(self)
	getPointer(self):UnequipTools()
end
function abi.ffi.method_humanoid_apply_description(self, humanoid_description, asset_type_verification)
	getPointer(self):ApplyDescription(getPointer(humanoid_description), asset_type_verification)
end
function abi.ffi.method_humanoid_apply_description_reset(self, humanoid_description, asset_type_verification)
	getPointer(self):ApplyDescriptionReset(getPointer(humanoid_description), asset_type_verification)
end
function abi.ffi.method_humanoid_play_emote(self, emote_name_addr, emote_name_len)
	local result_0 = getPointer(self):PlayEmote(loadString(memory, emote_name_addr, emote_name_len));
	return result_0 and 1 or 0;
end
function abi.ffi.event_humanoid_climbing_on_climbing(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Climbing:Connect(function(speed)
		invokeFunction(callback_data, callback_vtable, speed)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_died_on_died(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Died:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_falling_down_on_falling_down(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).FallingDown:Connect(function(active)
		invokeFunction(callback_data, callback_vtable, active and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_free_falling_on_free_falling(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).FreeFalling:Connect(function(active)
		invokeFunction(callback_data, callback_vtable, active and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_getting_up_on_getting_up(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).GettingUp:Connect(function(active)
		invokeFunction(callback_data, callback_vtable, active and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_health_changed_on_health_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).HealthChanged:Connect(function(health)
		invokeFunction(callback_data, callback_vtable, health)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_jumping_on_jumping(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Jumping:Connect(function(active)
		invokeFunction(callback_data, callback_vtable, active and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_move_to_finished_on_move_to_finished(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MoveToFinished:Connect(function(reached)
		invokeFunction(callback_data, callback_vtable, reached and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_platform_standing_on_platform_standing(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PlatformStanding:Connect(function(active)
		invokeFunction(callback_data, callback_vtable, active and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_ragdoll_on_ragdoll(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Ragdoll:Connect(function(active)
		invokeFunction(callback_data, callback_vtable, active and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_running_on_running(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Running:Connect(function(speed)
		invokeFunction(callback_data, callback_vtable, speed)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_seated_on_seated(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Seated:Connect(function(active, current_seat_part)
		local current_seat_part_optional, current_seat_part_optional_0 = 0, 0;
		if (current_seat_part ~= nil) then
			current_seat_part_optional, current_seat_part_optional_0 = 1, createPointer(current_seat_part);
		end
		invokeFunction(callback_data, callback_vtable, active and 1 or 0, current_seat_part_optional, current_seat_part_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_state_changed_on_state_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).StateChanged:Connect(function(old, new)
		invokeFunction(callback_data, callback_vtable, old, new)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_state_enabled_changed_on_state_enabled_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).StateEnabledChanged:Connect(function(state, is_enabled)
		invokeFunction(callback_data, callback_vtable, state, is_enabled and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_strafing_on_strafing(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Strafing:Connect(function(active)
		invokeFunction(callback_data, callback_vtable, active and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_swimming_on_swimming(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Swimming:Connect(function(speed)
		invokeFunction(callback_data, callback_vtable, speed)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_humanoid_touched_on_humanoid_touched(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Touched:Connect(function(part, limb)
		invokeFunction(callback_data, callback_vtable, createPointer(part), createPointer(limb))
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_humanoid_description_back_accessory(self)
	local result_0 = getPointer(self).BackAccessory;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_humanoid_description_back_accessory_set_back_accessory(self, value_addr, value_len)
	getPointer(self).BackAccessory = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_humanoid_description_body_type_scale(self)
	local result_0 = getPointer(self).BodyTypeScale;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_body_type_scale_set_body_type_scale(self, value)
	getPointer(self).BodyTypeScale = value;
end
function abi.ffi.property_getter_humanoid_description_climb_animation(self)
	local result_0 = getPointer(self).ClimbAnimation;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_climb_animation_set_climb_animation(self, value)
	getPointer(self).ClimbAnimation = value;
end
function abi.ffi.property_getter_humanoid_description_depth_scale(self)
	local result_0 = getPointer(self).DepthScale;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_depth_scale_set_depth_scale(self, value)
	getPointer(self).DepthScale = value;
end
function abi.ffi.property_getter_humanoid_description_face(self)
	local result_0 = getPointer(self).Face;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_face_set_face(self, value)
	getPointer(self).Face = value;
end
function abi.ffi.property_getter_humanoid_description_face_accessory(self)
	local result_0 = getPointer(self).FaceAccessory;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_humanoid_description_face_accessory_set_face_accessory(self, value_addr, value_len)
	getPointer(self).FaceAccessory = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_humanoid_description_fall_animation(self)
	local result_0 = getPointer(self).FallAnimation;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_fall_animation_set_fall_animation(self, value)
	getPointer(self).FallAnimation = value;
end
function abi.ffi.property_getter_humanoid_description_front_accessory(self)
	local result_0 = getPointer(self).FrontAccessory;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_humanoid_description_front_accessory_set_front_accessory(self, value_addr, value_len)
	getPointer(self).FrontAccessory = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_humanoid_description_graphic_t_shirt(self)
	local result_0 = getPointer(self).GraphicTShirt;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_graphic_t_shirt_set_graphic_t_shirt(self, value)
	getPointer(self).GraphicTShirt = value;
end
function abi.ffi.property_getter_humanoid_description_hair_accessory(self)
	local result_0 = getPointer(self).HairAccessory;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_humanoid_description_hair_accessory_set_hair_accessory(self, value_addr, value_len)
	getPointer(self).HairAccessory = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_humanoid_description_hat_accessory(self)
	local result_0 = getPointer(self).HatAccessory;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_humanoid_description_hat_accessory_set_hat_accessory(self, value_addr, value_len)
	getPointer(self).HatAccessory = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_humanoid_description_head(self)
	local result_0 = getPointer(self).Head;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_head_set_head(self, value)
	getPointer(self).Head = value;
end
function abi.ffi.property_getter_humanoid_description_head_color(self)
	local result_0 = getPointer(self).HeadColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_humanoid_description_head_color_set_head_color(self, value)
	getPointer(self).HeadColor = getPointer(value);
end
function abi.ffi.property_getter_humanoid_description_head_scale(self)
	local result_0 = getPointer(self).HeadScale;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_head_scale_set_head_scale(self, value)
	getPointer(self).HeadScale = value;
end
function abi.ffi.property_getter_humanoid_description_height_scale(self)
	local result_0 = getPointer(self).HeightScale;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_height_scale_set_height_scale(self, value)
	getPointer(self).HeightScale = value;
end
function abi.ffi.property_getter_humanoid_description_idle_animation(self)
	local result_0 = getPointer(self).IdleAnimation;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_idle_animation_set_idle_animation(self, value)
	getPointer(self).IdleAnimation = value;
end
function abi.ffi.property_getter_humanoid_description_jump_animation(self)
	local result_0 = getPointer(self).JumpAnimation;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_jump_animation_set_jump_animation(self, value)
	getPointer(self).JumpAnimation = value;
end
function abi.ffi.property_getter_humanoid_description_left_arm(self)
	local result_0 = getPointer(self).LeftArm;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_left_arm_set_left_arm(self, value)
	getPointer(self).LeftArm = value;
end
function abi.ffi.property_getter_humanoid_description_left_arm_color(self)
	local result_0 = getPointer(self).LeftArmColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_humanoid_description_left_arm_color_set_left_arm_color(self, value)
	getPointer(self).LeftArmColor = getPointer(value);
end
function abi.ffi.property_getter_humanoid_description_left_leg(self)
	local result_0 = getPointer(self).LeftLeg;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_left_leg_set_left_leg(self, value)
	getPointer(self).LeftLeg = value;
end
function abi.ffi.property_getter_humanoid_description_left_leg_color(self)
	local result_0 = getPointer(self).LeftLegColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_humanoid_description_left_leg_color_set_left_leg_color(self, value)
	getPointer(self).LeftLegColor = getPointer(value);
end
function abi.ffi.property_getter_humanoid_description_mood_animation(self)
	local result_0 = getPointer(self).MoodAnimation;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_mood_animation_set_mood_animation(self, value)
	getPointer(self).MoodAnimation = value;
end
function abi.ffi.property_getter_humanoid_description_neck_accessory(self)
	local result_0 = getPointer(self).NeckAccessory;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_humanoid_description_neck_accessory_set_neck_accessory(self, value_addr, value_len)
	getPointer(self).NeckAccessory = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_humanoid_description_pants(self)
	local result_0 = getPointer(self).Pants;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_pants_set_pants(self, value)
	getPointer(self).Pants = value;
end
function abi.ffi.property_getter_humanoid_description_proportion_scale(self)
	local result_0 = getPointer(self).ProportionScale;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_proportion_scale_set_proportion_scale(self, value)
	getPointer(self).ProportionScale = value;
end
function abi.ffi.property_getter_humanoid_description_right_arm(self)
	local result_0 = getPointer(self).RightArm;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_right_arm_set_right_arm(self, value)
	getPointer(self).RightArm = value;
end
function abi.ffi.property_getter_humanoid_description_right_arm_color(self)
	local result_0 = getPointer(self).RightArmColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_humanoid_description_right_arm_color_set_right_arm_color(self, value)
	getPointer(self).RightArmColor = getPointer(value);
end
function abi.ffi.property_getter_humanoid_description_right_leg(self)
	local result_0 = getPointer(self).RightLeg;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_right_leg_set_right_leg(self, value)
	getPointer(self).RightLeg = value;
end
function abi.ffi.property_getter_humanoid_description_right_leg_color(self)
	local result_0 = getPointer(self).RightLegColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_humanoid_description_right_leg_color_set_right_leg_color(self, value)
	getPointer(self).RightLegColor = getPointer(value);
end
function abi.ffi.property_getter_humanoid_description_run_animation(self)
	local result_0 = getPointer(self).RunAnimation;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_run_animation_set_run_animation(self, value)
	getPointer(self).RunAnimation = value;
end
function abi.ffi.property_getter_humanoid_description_shirt(self)
	local result_0 = getPointer(self).Shirt;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_shirt_set_shirt(self, value)
	getPointer(self).Shirt = value;
end
function abi.ffi.property_getter_humanoid_description_shoulders_accessory(self)
	local result_0 = getPointer(self).ShouldersAccessory;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_humanoid_description_shoulders_accessory_set_shoulders_accessory(self, value_addr, value_len)
	getPointer(self).ShouldersAccessory = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_humanoid_description_swim_animation(self)
	local result_0 = getPointer(self).SwimAnimation;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_swim_animation_set_swim_animation(self, value)
	getPointer(self).SwimAnimation = value;
end
function abi.ffi.property_getter_humanoid_description_torso(self)
	local result_0 = getPointer(self).Torso;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_torso_set_torso(self, value)
	getPointer(self).Torso = value;
end
function abi.ffi.property_getter_humanoid_description_torso_color(self)
	local result_0 = getPointer(self).TorsoColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_humanoid_description_torso_color_set_torso_color(self, value)
	getPointer(self).TorsoColor = getPointer(value);
end
function abi.ffi.property_getter_humanoid_description_waist_accessory(self)
	local result_0 = getPointer(self).WaistAccessory;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_humanoid_description_waist_accessory_set_waist_accessory(self, value_addr, value_len)
	getPointer(self).WaistAccessory = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_humanoid_description_walk_animation(self)
	local result_0 = getPointer(self).WalkAnimation;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_walk_animation_set_walk_animation(self, value)
	getPointer(self).WalkAnimation = value;
end
function abi.ffi.property_getter_humanoid_description_width_scale(self)
	local result_0 = getPointer(self).WidthScale;
	return result_0;
end
function abi.ffi.property_setter_humanoid_description_width_scale_set_width_scale(self, value)
	getPointer(self).WidthScale = value;
end
function abi.ffi.method_humanoid_description_add_emote(self, name_addr, name_len, asset_id)
	getPointer(self):AddEmote(loadString(memory, name_addr, name_len), asset_id)
end
function abi.ffi.method_humanoid_description_remove_emote(self, name_addr, name_len)
	getPointer(self):RemoveEmote(loadString(memory, name_addr, name_len))
end
function abi.ffi.property_getter_ik_control_alignment_offset(self)
	local result_0 = getPointer(self).AlignmentOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ik_control_alignment_offset_set_alignment_offset(self, value)
	getPointer(self).AlignmentOffset = getPointer(value);
end
function abi.ffi.property_getter_ik_control_chain_root(self)
	local result_0 = getPointer(self).ChainRoot;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_ik_control_chain_root_set_chain_root(self, value)
	getPointer(self).ChainRoot = getPointer(value);
end
function abi.ffi.property_getter_ik_control_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_ik_control_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_ik_control_end_effector(self)
	local result_0 = getPointer(self).EndEffector;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_ik_control_end_effector_set_end_effector(self, value)
	getPointer(self).EndEffector = getPointer(value);
end
function abi.ffi.property_getter_ik_control_offset(self)
	local result_0 = getPointer(self).Offset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ik_control_offset_set_offset(self, value)
	getPointer(self).Offset = getPointer(value);
end
function abi.ffi.property_getter_ik_control_pole(self)
	local result_0 = getPointer(self).Pole;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_ik_control_pole_set_pole(self, value)
	getPointer(self).Pole = getPointer(value);
end
function abi.ffi.property_getter_ik_control_priority(self)
	local result_0 = getPointer(self).Priority;
	return result_0;
end
function abi.ffi.property_setter_ik_control_priority_set_priority(self, value)
	getPointer(self).Priority = value;
end
function abi.ffi.property_getter_ik_control_target(self)
	local result_0 = getPointer(self).Target;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_ik_control_target_set_target(self, value)
	getPointer(self).Target = getPointer(value);
end
function abi.ffi.property_getter_ik_control_type(self)
	local result_0 = getPointer(self).Type;
	return result_0;
end
function abi.ffi.property_setter_ik_control_type_set_type(self, value)
	getPointer(self).Type = value;
end
function abi.ffi.property_getter_ik_control_weight(self)
	local result_0 = getPointer(self).Weight;
	return result_0;
end
function abi.ffi.property_setter_ik_control_weight_set_weight(self, value)
	getPointer(self).Weight = value;
end
function abi.ffi.property_getter_importer_base_settings_id(self)
	local result_0 = getPointer(self).Id;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_importer_base_settings_import_name(self)
	local result_0 = getPointer(self).ImportName;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_importer_base_settings_import_name_set_import_name(self, value_addr, value_len)
	getPointer(self).ImportName = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_importer_base_settings_should_import(self)
	local result_0 = getPointer(self).ShouldImport;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_base_settings_should_import_set_should_import(self, value)
	getPointer(self).ShouldImport = value == 1;
end
function abi.ffi.property_getter_importer_group_settings_anchored(self)
	local result_0 = getPointer(self).Anchored;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_group_settings_anchored_set_anchored(self, value)
	getPointer(self).Anchored = value == 1;
end
function abi.ffi.property_getter_importer_group_settings_import_as_model_asset(self)
	local result_0 = getPointer(self).ImportAsModelAsset;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_group_settings_import_as_model_asset_set_import_as_model_asset(self, value)
	getPointer(self).ImportAsModelAsset = value == 1;
end
function abi.ffi.property_getter_importer_group_settings_insert_in_workspace(self)
	local result_0 = getPointer(self).InsertInWorkspace;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_group_settings_insert_in_workspace_set_insert_in_workspace(self, value)
	getPointer(self).InsertInWorkspace = value == 1;
end
function abi.ffi.property_getter_importer_material_settings_diffuse_file_path(self)
	local result_0 = getPointer(self).DiffuseFilePath;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_importer_material_settings_diffuse_file_path_set_diffuse_file_path(self, value_addr, value_len)
	getPointer(self).DiffuseFilePath = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_importer_material_settings_is_pbr(self)
	local result_0 = getPointer(self).IsPbr;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_importer_material_settings_metalness_file_path(self)
	local result_0 = getPointer(self).MetalnessFilePath;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_importer_material_settings_metalness_file_path_set_metalness_file_path(self, value_addr, value_len)
	getPointer(self).MetalnessFilePath = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_importer_material_settings_normal_file_path(self)
	local result_0 = getPointer(self).NormalFilePath;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_importer_material_settings_normal_file_path_set_normal_file_path(self, value_addr, value_len)
	getPointer(self).NormalFilePath = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_importer_material_settings_roughness_file_path(self)
	local result_0 = getPointer(self).RoughnessFilePath;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_importer_material_settings_roughness_file_path_set_roughness_file_path(self, value_addr, value_len)
	getPointer(self).RoughnessFilePath = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_importer_mesh_settings_anchored(self)
	local result_0 = getPointer(self).Anchored;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_mesh_settings_anchored_set_anchored(self, value)
	getPointer(self).Anchored = value == 1;
end
function abi.ffi.property_getter_importer_mesh_settings_cage_manifold(self)
	local result_0 = getPointer(self).CageManifold;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_importer_mesh_settings_cage_mesh_intersected_preview(self)
	local result_0 = getPointer(self).CageMeshIntersectedPreview;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_mesh_settings_cage_mesh_intersected_preview_set_cage_mesh_intersected_preview(self, value)
	getPointer(self).CageMeshIntersectedPreview = value == 1;
end
function abi.ffi.property_getter_importer_mesh_settings_cage_mesh_not_intersected(self)
	local result_0 = getPointer(self).CageMeshNotIntersected;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_importer_mesh_settings_cage_no_overlapping_vertices(self)
	local result_0 = getPointer(self).CageNoOverlappingVertices;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_importer_mesh_settings_cage_non_manifold_preview(self)
	local result_0 = getPointer(self).CageNonManifoldPreview;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_mesh_settings_cage_non_manifold_preview_set_cage_non_manifold_preview(self, value)
	getPointer(self).CageNonManifoldPreview = value == 1;
end
function abi.ffi.property_getter_importer_mesh_settings_cage_overlapping_vertices_preview(self)
	local result_0 = getPointer(self).CageOverlappingVerticesPreview;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_mesh_settings_cage_overlapping_vertices_preview_set_cage_overlapping_vertices_preview(self, value)
	getPointer(self).CageOverlappingVerticesPreview = value == 1;
end
function abi.ffi.property_getter_importer_mesh_settings_cage_uv_matched(self)
	local result_0 = getPointer(self).CageUVMatched;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_importer_mesh_settings_cage_uv_mis_matched_preview(self)
	local result_0 = getPointer(self).CageUVMisMatchedPreview;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_mesh_settings_cage_uv_mis_matched_preview_set_cage_uv_mis_matched_preview(self, value)
	getPointer(self).CageUVMisMatchedPreview = value == 1;
end
function abi.ffi.property_getter_importer_mesh_settings_dimensions(self)
	local result_0 = getPointer(self).Dimensions;
	return createPointer(result_0);
end
function abi.ffi.property_getter_importer_mesh_settings_double_sided(self)
	local result_0 = getPointer(self).DoubleSided;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_mesh_settings_double_sided_set_double_sided(self, value)
	getPointer(self).DoubleSided = value == 1;
end
function abi.ffi.property_getter_importer_mesh_settings_ignore_vertex_colors(self)
	local result_0 = getPointer(self).IgnoreVertexColors;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_mesh_settings_ignore_vertex_colors_set_ignore_vertex_colors(self, value)
	getPointer(self).IgnoreVertexColors = value == 1;
end
function abi.ffi.property_getter_importer_mesh_settings_irrelevant_cage_modified_preview(self)
	local result_0 = getPointer(self).IrrelevantCageModifiedPreview;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_mesh_settings_irrelevant_cage_modified_preview_set_irrelevant_cage_modified_preview(self, value)
	getPointer(self).IrrelevantCageModifiedPreview = value == 1;
end
function abi.ffi.property_getter_importer_mesh_settings_mesh_hole_detected_preview(self)
	local result_0 = getPointer(self).MeshHoleDetectedPreview;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_mesh_settings_mesh_hole_detected_preview_set_mesh_hole_detected_preview(self, value)
	getPointer(self).MeshHoleDetectedPreview = value == 1;
end
function abi.ffi.property_getter_importer_mesh_settings_mesh_no_hole_detected(self)
	local result_0 = getPointer(self).MeshNoHoleDetected;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_importer_mesh_settings_no_irrelevant_cage_modified(self)
	local result_0 = getPointer(self).NoIrrelevantCageModified;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_importer_mesh_settings_no_outer_cage_far_extended_from_mesh(self)
	local result_0 = getPointer(self).NoOuterCageFarExtendedFromMesh;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_importer_mesh_settings_outer_cage_far_extended_from_mesh_preview(self)
	local result_0 = getPointer(self).OuterCageFarExtendedFromMeshPreview;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_mesh_settings_outer_cage_far_extended_from_mesh_preview_set_outer_cage_far_extended_from_mesh_preview(self, value)
	getPointer(self).OuterCageFarExtendedFromMeshPreview = value == 1;
end
function abi.ffi.property_getter_importer_mesh_settings_polygon_count(self)
	local result_0 = getPointer(self).PolygonCount;
	return result_0;
end
function abi.ffi.property_getter_importer_mesh_settings_use_imported_pivot(self)
	local result_0 = getPointer(self).UseImportedPivot;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_mesh_settings_use_imported_pivot_set_use_imported_pivot(self, value)
	getPointer(self).UseImportedPivot = value == 1;
end
function abi.ffi.property_getter_importer_root_settings_anchored(self)
	local result_0 = getPointer(self).Anchored;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_root_settings_anchored_set_anchored(self, value)
	getPointer(self).Anchored = value == 1;
end
function abi.ffi.property_getter_importer_root_settings_file_dimensions(self)
	local result_0 = getPointer(self).FileDimensions;
	return createPointer(result_0);
end
function abi.ffi.property_getter_importer_root_settings_import_as_model_asset(self)
	local result_0 = getPointer(self).ImportAsModelAsset;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_root_settings_import_as_model_asset_set_import_as_model_asset(self, value)
	getPointer(self).ImportAsModelAsset = value == 1;
end
function abi.ffi.property_getter_importer_root_settings_insert_in_workspace(self)
	local result_0 = getPointer(self).InsertInWorkspace;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_root_settings_insert_in_workspace_set_insert_in_workspace(self, value)
	getPointer(self).InsertInWorkspace = value == 1;
end
function abi.ffi.property_getter_importer_root_settings_insert_with_scene_position(self)
	local result_0 = getPointer(self).InsertWithScenePosition;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_root_settings_insert_with_scene_position_set_insert_with_scene_position(self, value)
	getPointer(self).InsertWithScenePosition = value == 1;
end
function abi.ffi.property_getter_importer_root_settings_invert_negative_faces(self)
	local result_0 = getPointer(self).InvertNegativeFaces;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_root_settings_invert_negative_faces_set_invert_negative_faces(self, value)
	getPointer(self).InvertNegativeFaces = value == 1;
end
function abi.ffi.property_getter_importer_root_settings_merge_meshes(self)
	local result_0 = getPointer(self).MergeMeshes;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_root_settings_merge_meshes_set_merge_meshes(self, value)
	getPointer(self).MergeMeshes = value == 1;
end
function abi.ffi.property_getter_importer_root_settings_polygon_count(self)
	local result_0 = getPointer(self).PolygonCount;
	return result_0;
end
function abi.ffi.property_getter_importer_root_settings_rig_type(self)
	local result_0 = getPointer(self).RigType;
	return result_0;
end
function abi.ffi.property_setter_importer_root_settings_rig_type_set_rig_type(self, value)
	getPointer(self).RigType = value;
end
function abi.ffi.property_getter_importer_root_settings_scale_unit(self)
	local result_0 = getPointer(self).ScaleUnit;
	return result_0;
end
function abi.ffi.property_setter_importer_root_settings_scale_unit_set_scale_unit(self, value)
	getPointer(self).ScaleUnit = value;
end
function abi.ffi.property_getter_importer_root_settings_use_scene_origin_as_pivot(self)
	local result_0 = getPointer(self).UseSceneOriginAsPivot;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_importer_root_settings_use_scene_origin_as_pivot_set_use_scene_origin_as_pivot(self, value)
	getPointer(self).UseSceneOriginAsPivot = value == 1;
end
function abi.ffi.property_getter_importer_root_settings_world_forward(self)
	local result_0 = getPointer(self).WorldForward;
	return result_0;
end
function abi.ffi.property_setter_importer_root_settings_world_forward_set_world_forward(self, value)
	getPointer(self).WorldForward = value;
end
function abi.ffi.property_getter_importer_root_settings_world_up(self)
	local result_0 = getPointer(self).WorldUp;
	return result_0;
end
function abi.ffi.property_setter_importer_root_settings_world_up_set_world_up(self, value)
	getPointer(self).WorldUp = value;
end
function abi.ffi.property_getter_input_object_delta(self)
	local result_0 = getPointer(self).Delta;
	return createPointer(result_0);
end
function abi.ffi.property_setter_input_object_delta_set_delta(self, value)
	getPointer(self).Delta = getPointer(value);
end
function abi.ffi.property_getter_input_object_key_code(self)
	local result_0 = getPointer(self).KeyCode;
	return result_0;
end
function abi.ffi.property_setter_input_object_key_code_set_key_code(self, value)
	getPointer(self).KeyCode = value;
end
function abi.ffi.property_getter_input_object_position(self)
	local result_0 = getPointer(self).Position;
	return createPointer(result_0);
end
function abi.ffi.property_setter_input_object_position_set_position(self, value)
	getPointer(self).Position = getPointer(value);
end
function abi.ffi.property_getter_input_object_user_input_state(self)
	local result_0 = getPointer(self).UserInputState;
	return result_0;
end
function abi.ffi.property_setter_input_object_user_input_state_set_user_input_state(self, value)
	getPointer(self).UserInputState = value;
end
function abi.ffi.property_getter_input_object_user_input_type(self)
	local result_0 = getPointer(self).UserInputType;
	return result_0;
end
function abi.ffi.property_setter_input_object_user_input_type_set_user_input_type(self, value)
	getPointer(self).UserInputType = value;
end
function abi.ffi.method_input_object_is_modifier_key_down(self, modifier_key)
	local result_0 = getPointer(self):IsModifierKeyDown(modifier_key);
	return result_0 and 1 or 0;
end
function abi.ffi.method_insert_service_get_latest_asset_version_async(self, asset_id)
	local result_0 = getPointer(self):GetLatestAssetVersionAsync(asset_id);
	return result_0;
end
function abi.ffi.method_insert_service_load_asset(self, asset_id)
	local result_0 = getPointer(self):LoadAsset(asset_id);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_insert_service_load_asset_version(self, asset_version_id)
	local result_0 = getPointer(self):LoadAssetVersion(asset_version_id);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_joint_instance_active(self)
	local result_0 = getPointer(self).Active;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_joint_instance_c_0(self)
	local result_0 = getPointer(self).C0;
	return createPointer(result_0);
end
function abi.ffi.property_setter_joint_instance_c_0_set_c_0(self, value)
	getPointer(self).C0 = getPointer(value);
end
function abi.ffi.property_getter_joint_instance_c_1(self)
	local result_0 = getPointer(self).C1;
	return createPointer(result_0);
end
function abi.ffi.property_setter_joint_instance_c_1_set_c_1(self, value)
	getPointer(self).C1 = getPointer(value);
end
function abi.ffi.property_getter_joint_instance_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_joint_instance_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_joint_instance_part_0(self)
	local result_0 = getPointer(self).Part0;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_joint_instance_part_0_set_part_0(self, value)
	getPointer(self).Part0 = getPointer(value);
end
function abi.ffi.property_getter_joint_instance_part_1(self)
	local result_0 = getPointer(self).Part1;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_joint_instance_part_1_set_part_1(self, value)
	getPointer(self).Part1 = getPointer(value);
end
function abi.ffi.property_getter_dynamic_rotate_base_angle(self)
	local result_0 = getPointer(self).BaseAngle;
	return result_0;
end
function abi.ffi.property_setter_dynamic_rotate_base_angle_set_base_angle(self, value)
	getPointer(self).BaseAngle = value;
end
function abi.ffi.property_getter_motor_current_angle(self)
	local result_0 = getPointer(self).CurrentAngle;
	return result_0;
end
function abi.ffi.property_setter_motor_current_angle_set_current_angle(self, value)
	getPointer(self).CurrentAngle = value;
end
function abi.ffi.property_getter_motor_desired_angle(self)
	local result_0 = getPointer(self).DesiredAngle;
	return result_0;
end
function abi.ffi.property_setter_motor_desired_angle_set_desired_angle(self, value)
	getPointer(self).DesiredAngle = value;
end
function abi.ffi.property_getter_motor_max_velocity(self)
	local result_0 = getPointer(self).MaxVelocity;
	return result_0;
end
function abi.ffi.property_setter_motor_max_velocity_set_max_velocity(self, value)
	getPointer(self).MaxVelocity = value;
end
function abi.ffi.method_motor_set_desired_angle_fn_set_desired_angle(self, value)
	getPointer(self):SetDesiredAngle(value)
end
function abi.ffi.property_getter_motor_6_d_transform(self)
	local result_0 = getPointer(self).Transform;
	return createPointer(result_0);
end
function abi.ffi.property_setter_motor_6_d_transform_set_transform(self, value)
	getPointer(self).Transform = getPointer(value);
end
function abi.ffi.property_getter_velocity_motor_current_angle(self)
	local result_0 = getPointer(self).CurrentAngle;
	return result_0;
end
function abi.ffi.property_setter_velocity_motor_current_angle_set_current_angle(self, value)
	getPointer(self).CurrentAngle = value;
end
function abi.ffi.property_getter_velocity_motor_desired_angle(self)
	local result_0 = getPointer(self).DesiredAngle;
	return result_0;
end
function abi.ffi.property_setter_velocity_motor_desired_angle_set_desired_angle(self, value)
	getPointer(self).DesiredAngle = value;
end
function abi.ffi.property_getter_velocity_motor_max_velocity(self)
	local result_0 = getPointer(self).MaxVelocity;
	return result_0;
end
function abi.ffi.property_setter_velocity_motor_max_velocity_set_max_velocity(self, value)
	getPointer(self).MaxVelocity = value;
end
function abi.ffi.property_getter_keyframe_time(self)
	local result_0 = getPointer(self).Time;
	return result_0;
end
function abi.ffi.property_setter_keyframe_time_set_time(self, value)
	getPointer(self).Time = value;
end
function abi.ffi.method_keyframe_add_marker(self, marker)
	getPointer(self):AddMarker(getPointer(marker))
end
function abi.ffi.method_keyframe_add_pose(self, pose)
	getPointer(self):AddPose(getPointer(pose))
end
function abi.ffi.method_keyframe_remove_marker(self, marker)
	getPointer(self):RemoveMarker(getPointer(marker))
end
function abi.ffi.method_keyframe_remove_pose(self, pose)
	getPointer(self):RemovePose(getPointer(pose))
end
function abi.ffi.property_getter_keyframe_marker_value(self)
	local result_0 = getPointer(self).Value;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_keyframe_marker_value_set_value(self, value_addr, value_len)
	getPointer(self).Value = loadString(memory, value_addr, value_len);
end
function abi.ffi.method_keyframe_sequence_provider_register_active_keyframe_sequence(self, keyframe_sequence)
	local result_0 = getPointer(self):RegisterActiveKeyframeSequence(getPointer(keyframe_sequence));
	return createPointer(result_0);
end
function abi.ffi.method_keyframe_sequence_provider_register_keyframe_sequence(self, keyframe_sequence)
	local result_0 = getPointer(self):RegisterKeyframeSequence(getPointer(keyframe_sequence));
	return createPointer(result_0);
end
function abi.ffi.method_keyframe_sequence_provider_get_animations(self, user_id)
	local result_0 = getPointer(self):GetAnimations(user_id);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_keyframe_sequence_provider_get_keyframe_sequence_async(self, asset_id)
	local result_0 = getPointer(self):GetKeyframeSequenceAsync(getPointer(asset_id));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_light_brightness(self)
	local result_0 = getPointer(self).Brightness;
	return result_0;
end
function abi.ffi.property_setter_light_brightness_set_brightness(self, value)
	getPointer(self).Brightness = value;
end
function abi.ffi.property_getter_light_color(self)
	local result_0 = getPointer(self).Color;
	return createPointer(result_0);
end
function abi.ffi.property_setter_light_color_set_color(self, value)
	getPointer(self).Color = getPointer(value);
end
function abi.ffi.property_getter_light_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_light_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_light_shadows(self)
	local result_0 = getPointer(self).Shadows;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_light_shadows_set_shadows(self, value)
	getPointer(self).Shadows = value == 1;
end
function abi.ffi.property_getter_point_light_range(self)
	local result_0 = getPointer(self).Range;
	return result_0;
end
function abi.ffi.property_setter_point_light_range_set_range(self, value)
	getPointer(self).Range = value;
end
function abi.ffi.property_getter_spot_light_angle(self)
	local result_0 = getPointer(self).Angle;
	return result_0;
end
function abi.ffi.property_setter_spot_light_angle_set_angle(self, value)
	getPointer(self).Angle = value;
end
function abi.ffi.property_getter_spot_light_face(self)
	local result_0 = getPointer(self).Face;
	return result_0;
end
function abi.ffi.property_setter_spot_light_face_set_face(self, value)
	getPointer(self).Face = value;
end
function abi.ffi.property_getter_spot_light_range(self)
	local result_0 = getPointer(self).Range;
	return result_0;
end
function abi.ffi.property_setter_spot_light_range_set_range(self, value)
	getPointer(self).Range = value;
end
function abi.ffi.property_getter_surface_light_angle(self)
	local result_0 = getPointer(self).Angle;
	return result_0;
end
function abi.ffi.property_setter_surface_light_angle_set_angle(self, value)
	getPointer(self).Angle = value;
end
function abi.ffi.property_getter_surface_light_face(self)
	local result_0 = getPointer(self).Face;
	return result_0;
end
function abi.ffi.property_setter_surface_light_face_set_face(self, value)
	getPointer(self).Face = value;
end
function abi.ffi.property_getter_surface_light_range(self)
	local result_0 = getPointer(self).Range;
	return result_0;
end
function abi.ffi.property_setter_surface_light_range_set_range(self, value)
	getPointer(self).Range = value;
end
function abi.ffi.property_getter_lighting_ambient(self)
	local result_0 = getPointer(self).Ambient;
	return createPointer(result_0);
end
function abi.ffi.property_setter_lighting_ambient_set_ambient(self, value)
	getPointer(self).Ambient = getPointer(value);
end
function abi.ffi.property_getter_lighting_brightness(self)
	local result_0 = getPointer(self).Brightness;
	return result_0;
end
function abi.ffi.property_setter_lighting_brightness_set_brightness(self, value)
	getPointer(self).Brightness = value;
end
function abi.ffi.property_getter_lighting_clock_time(self)
	local result_0 = getPointer(self).ClockTime;
	return result_0;
end
function abi.ffi.property_setter_lighting_clock_time_set_clock_time(self, value)
	getPointer(self).ClockTime = value;
end
function abi.ffi.property_getter_lighting_color_shift_bottom(self)
	local result_0 = getPointer(self).ColorShift_Bottom;
	return createPointer(result_0);
end
function abi.ffi.property_setter_lighting_color_shift_bottom_set_color_shift_bottom(self, value)
	getPointer(self).ColorShift_Bottom = getPointer(value);
end
function abi.ffi.property_getter_lighting_color_shift_top(self)
	local result_0 = getPointer(self).ColorShift_Top;
	return createPointer(result_0);
end
function abi.ffi.property_setter_lighting_color_shift_top_set_color_shift_top(self, value)
	getPointer(self).ColorShift_Top = getPointer(value);
end
function abi.ffi.property_getter_lighting_environment_diffuse_scale(self)
	local result_0 = getPointer(self).EnvironmentDiffuseScale;
	return result_0;
end
function abi.ffi.property_setter_lighting_environment_diffuse_scale_set_environment_diffuse_scale(self, value)
	getPointer(self).EnvironmentDiffuseScale = value;
end
function abi.ffi.property_getter_lighting_environment_specular_scale(self)
	local result_0 = getPointer(self).EnvironmentSpecularScale;
	return result_0;
end
function abi.ffi.property_setter_lighting_environment_specular_scale_set_environment_specular_scale(self, value)
	getPointer(self).EnvironmentSpecularScale = value;
end
function abi.ffi.property_getter_lighting_exposure_compensation(self)
	local result_0 = getPointer(self).ExposureCompensation;
	return result_0;
end
function abi.ffi.property_setter_lighting_exposure_compensation_set_exposure_compensation(self, value)
	getPointer(self).ExposureCompensation = value;
end
function abi.ffi.property_getter_lighting_fog_color(self)
	local result_0 = getPointer(self).FogColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_lighting_fog_color_set_fog_color(self, value)
	getPointer(self).FogColor = getPointer(value);
end
function abi.ffi.property_getter_lighting_fog_end(self)
	local result_0 = getPointer(self).FogEnd;
	return result_0;
end
function abi.ffi.property_setter_lighting_fog_end_set_fog_end(self, value)
	getPointer(self).FogEnd = value;
end
function abi.ffi.property_getter_lighting_fog_start(self)
	local result_0 = getPointer(self).FogStart;
	return result_0;
end
function abi.ffi.property_setter_lighting_fog_start_set_fog_start(self, value)
	getPointer(self).FogStart = value;
end
function abi.ffi.property_getter_lighting_geographic_latitude(self)
	local result_0 = getPointer(self).GeographicLatitude;
	return result_0;
end
function abi.ffi.property_setter_lighting_geographic_latitude_set_geographic_latitude(self, value)
	getPointer(self).GeographicLatitude = value;
end
function abi.ffi.property_getter_lighting_global_shadows(self)
	local result_0 = getPointer(self).GlobalShadows;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_lighting_global_shadows_set_global_shadows(self, value)
	getPointer(self).GlobalShadows = value == 1;
end
function abi.ffi.property_getter_lighting_outdoor_ambient(self)
	local result_0 = getPointer(self).OutdoorAmbient;
	return createPointer(result_0);
end
function abi.ffi.property_setter_lighting_outdoor_ambient_set_outdoor_ambient(self, value)
	getPointer(self).OutdoorAmbient = getPointer(value);
end
function abi.ffi.property_getter_lighting_shadow_softness(self)
	local result_0 = getPointer(self).ShadowSoftness;
	return result_0;
end
function abi.ffi.property_setter_lighting_shadow_softness_set_shadow_softness(self, value)
	getPointer(self).ShadowSoftness = value;
end
function abi.ffi.property_getter_lighting_time_of_day(self)
	local result_0 = getPointer(self).TimeOfDay;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_lighting_time_of_day_set_time_of_day(self, value_addr, value_len)
	getPointer(self).TimeOfDay = loadString(memory, value_addr, value_len);
end
function abi.ffi.method_lighting_get_minutes_after_midnight(self)
	local result_0 = getPointer(self):GetMinutesAfterMidnight();
	return result_0;
end
function abi.ffi.method_lighting_get_moon_direction(self)
	local result_0 = getPointer(self):GetMoonDirection();
	return createPointer(result_0);
end
function abi.ffi.method_lighting_get_moon_phase(self)
	local result_0 = getPointer(self):GetMoonPhase();
	return result_0;
end
function abi.ffi.method_lighting_get_sun_direction(self)
	local result_0 = getPointer(self):GetSunDirection();
	return createPointer(result_0);
end
function abi.ffi.method_lighting_set_minutes_after_midnight(self, minutes)
	getPointer(self):SetMinutesAfterMidnight(minutes)
end
function abi.ffi.event_lighting_lighting_changed_on_lighting_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).LightingChanged:Connect(function(sky_changed)
		invokeFunction(callback_data, callback_vtable, sky_changed and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_localization_service_roblox_locale_id(self)
	local result_0 = getPointer(self).RobloxLocaleId;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_localization_service_system_locale_id(self)
	local result_0 = getPointer(self).SystemLocaleId;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_localization_service_get_translator_for_player(self, player)
	local result_0 = getPointer(self):GetTranslatorForPlayer(getPointer(player));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_localization_service_get_country_region_for_player_async(self, player)
	local result_0 = getPointer(self):GetCountryRegionForPlayerAsync(getPointer(player));
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_localization_service_get_translator_for_locale_async(self, locale_addr, locale_len)
	local result_0 = getPointer(self):GetTranslatorForLocaleAsync(loadString(memory, locale_addr, locale_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_localization_service_get_translator_for_player_async(self, player)
	local result_0 = getPointer(self):GetTranslatorForPlayerAsync(getPointer(player));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_localization_table_source_locale_id(self)
	local result_0 = getPointer(self).SourceLocaleId;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_localization_table_source_locale_id_set_source_locale_id(self, value_addr, value_len)
	getPointer(self).SourceLocaleId = loadString(memory, value_addr, value_len);
end
function abi.ffi.method_localization_table_get_translator(self, locale_id_addr, locale_id_len)
	local result_0 = getPointer(self):GetTranslator(loadString(memory, locale_id_addr, locale_id_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_localization_table_remove_entry(self, key_addr, key_len, source_addr, source_len, context_addr, context_len)
	getPointer(self):RemoveEntry(loadString(memory, key_addr, key_len), loadString(memory, source_addr, source_len), loadString(memory, context_addr, context_len))
end
function abi.ffi.method_localization_table_remove_entry_value(self, key_addr, key_len, source_addr, source_len, context_addr, context_len, locale_id_addr, locale_id_len)
	getPointer(self):RemoveEntryValue(loadString(memory, key_addr, key_len), loadString(memory, source_addr, source_len), loadString(memory, context_addr, context_len), loadString(memory, locale_id_addr, locale_id_len))
end
function abi.ffi.method_localization_table_remove_target_locale(self, locale_id_addr, locale_id_len)
	getPointer(self):RemoveTargetLocale(loadString(memory, locale_id_addr, locale_id_len))
end
function abi.ffi.method_localization_table_set_entry_context(self, key_addr, key_len, source_addr, source_len, context_addr, context_len, new_context_addr, new_context_len)
	getPointer(self):SetEntryContext(loadString(memory, key_addr, key_len), loadString(memory, source_addr, source_len), loadString(memory, context_addr, context_len), loadString(memory, new_context_addr, new_context_len))
end
function abi.ffi.method_localization_table_set_entry_example(self, key_addr, key_len, source_addr, source_len, context_addr, context_len, example_addr, example_len)
	getPointer(self):SetEntryExample(loadString(memory, key_addr, key_len), loadString(memory, source_addr, source_len), loadString(memory, context_addr, context_len), loadString(memory, example_addr, example_len))
end
function abi.ffi.method_localization_table_set_entry_key(self, key_addr, key_len, source_addr, source_len, context_addr, context_len, new_key_addr, new_key_len)
	getPointer(self):SetEntryKey(loadString(memory, key_addr, key_len), loadString(memory, source_addr, source_len), loadString(memory, context_addr, context_len), loadString(memory, new_key_addr, new_key_len))
end
function abi.ffi.method_localization_table_set_entry_source(self, key_addr, key_len, source_addr, source_len, context_addr, context_len, new_source_addr, new_source_len)
	getPointer(self):SetEntrySource(loadString(memory, key_addr, key_len), loadString(memory, source_addr, source_len), loadString(memory, context_addr, context_len), loadString(memory, new_source_addr, new_source_len))
end
function abi.ffi.method_localization_table_set_entry_value(self, key_addr, key_len, source_addr, source_len, context_addr, context_len, locale_id_addr, locale_id_len, text_addr, text_len)
	getPointer(self):SetEntryValue(loadString(memory, key_addr, key_len), loadString(memory, source_addr, source_len), loadString(memory, context_addr, context_len), loadString(memory, locale_id_addr, locale_id_len), loadString(memory, text_addr, text_len))
end
function abi.ffi.property_getter_lod_data_entity_entity_lod_enabled(self)
	local result_0 = getPointer(self).EntityLodEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_lod_data_entity_entity_lod_enabled_set_entity_lod_enabled(self, value)
	getPointer(self).EntityLodEnabled = value == 1;
end
function abi.ffi.event_log_service_message_out_on_message_out(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MessageOut:Connect(function(message, message_type)
		local message_len = #message;
		local message_content = allocString(message_len);
		for i = 1, message_len do
			storeU8(memory, message_content + (i - 1), string.byte(message:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, message_content, message_len, message_type)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_base_script_disabled(self)
	local result_0 = getPointer(self).Disabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_base_script_disabled_set_disabled(self, value)
	getPointer(self).Disabled = value == 1;
end
function abi.ffi.property_getter_base_script_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_base_script_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_base_script_linked_source(self)
	local result_0 = getPointer(self).LinkedSource;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_script_linked_source_set_linked_source(self, value)
	getPointer(self).LinkedSource = getPointer(value);
end
function abi.ffi.property_getter_module_script_linked_source(self)
	local result_0 = getPointer(self).LinkedSource;
	return createPointer(result_0);
end
function abi.ffi.property_setter_module_script_linked_source_set_linked_source(self, value)
	getPointer(self).LinkedSource = getPointer(value);
end
function abi.ffi.property_getter_marker_curve_length(self)
	local result_0 = getPointer(self).Length;
	return result_0;
end
function abi.ffi.method_marker_curve_remove_marker_at_index(self, starting_index, count)
	local result_0 = getPointer(self):RemoveMarkerAtIndex(starting_index, count);
	return result_0;
end
function abi.ffi.method_marketplace_service_prompt_bundle_purchase(self, player, bundle_id)
	getPointer(self):PromptBundlePurchase(getPointer(player), bundle_id)
end
function abi.ffi.method_marketplace_service_prompt_game_pass_purchase(self, player, game_pass_id)
	getPointer(self):PromptGamePassPurchase(getPointer(player), game_pass_id)
end
function abi.ffi.method_marketplace_service_prompt_premium_purchase(self, player)
	getPointer(self):PromptPremiumPurchase(getPointer(player))
end
function abi.ffi.method_marketplace_service_prompt_product_purchase(self, player, product_id, equip_if_purchased, currency_type)
	getPointer(self):PromptProductPurchase(getPointer(player), product_id, equip_if_purchased == 1, currency_type)
end
function abi.ffi.method_marketplace_service_prompt_purchase(self, player, asset_id, equip_if_purchased, currency_type)
	getPointer(self):PromptPurchase(getPointer(player), asset_id, equip_if_purchased == 1, currency_type)
end
function abi.ffi.method_marketplace_service_prompt_subscription_cancellation(self, player, subscription_id)
	getPointer(self):PromptSubscriptionCancellation(getPointer(player), subscription_id)
end
function abi.ffi.method_marketplace_service_prompt_subscription_purchase(self, player, subscription_id)
	getPointer(self):PromptSubscriptionPurchase(getPointer(player), subscription_id)
end
function abi.ffi.method_marketplace_service_get_developer_products_async(self)
	local result_0 = getPointer(self):GetDeveloperProductsAsync();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_marketplace_service_is_player_subscribed(self, player, subscription_id)
	local result_0 = getPointer(self):IsPlayerSubscribed(getPointer(player), subscription_id);
	return result_0 and 1 or 0;
end
function abi.ffi.method_marketplace_service_player_owns_asset(self, player, asset_id)
	local result_0 = getPointer(self):PlayerOwnsAsset(getPointer(player), asset_id);
	return result_0 and 1 or 0;
end
function abi.ffi.method_marketplace_service_player_owns_bundle(self, player, bundle_id)
	local result_0 = getPointer(self):PlayerOwnsBundle(getPointer(player), bundle_id);
	return result_0 and 1 or 0;
end
function abi.ffi.method_marketplace_service_user_owns_game_pass_async(self, user_id, game_pass_id)
	local result_0 = getPointer(self):UserOwnsGamePassAsync(user_id, game_pass_id);
	return result_0 and 1 or 0;
end
function abi.ffi.event_marketplace_service_prompt_bundle_purchase_finished_on_prompt_bundle_purchase_finished(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptBundlePurchaseFinished:Connect(function(player, bundle_id, was_purchased)
		local player_optional, player_optional_0 = 0, 0;
		if (player ~= nil) then
			player_optional, player_optional_0 = 1, createPointer(player);
		end
		invokeFunction(callback_data, callback_vtable, player_optional, player_optional_0, bundle_id, was_purchased and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_marketplace_service_prompt_game_pass_purchase_finished_on_prompt_game_pass_purchase_finished(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptGamePassPurchaseFinished:Connect(function(player, game_pass_id, was_purchased)
		local player_optional, player_optional_0 = 0, 0;
		if (player ~= nil) then
			player_optional, player_optional_0 = 1, createPointer(player);
		end
		invokeFunction(callback_data, callback_vtable, player_optional, player_optional_0, game_pass_id, was_purchased and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_marketplace_service_prompt_premium_purchase_finished_on_prompt_premium_purchase_finished(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptPremiumPurchaseFinished:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_marketplace_service_prompt_purchase_finished_on_prompt_purchase_finished(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptPurchaseFinished:Connect(function(player, asset_id, is_purchased)
		local player_optional, player_optional_0 = 0, 0;
		if (player ~= nil) then
			player_optional, player_optional_0 = 1, createPointer(player);
		end
		invokeFunction(callback_data, callback_vtable, player_optional, player_optional_0, asset_id, is_purchased and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_marketplace_service_prompt_subscription_cancellation_finished_on_prompt_subscription_cancellation_finished(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptSubscriptionCancellationFinished:Connect(function(player, subscription_id, was_canceled)
		local player_optional, player_optional_0 = 0, 0;
		if (player ~= nil) then
			player_optional, player_optional_0 = 1, createPointer(player);
		end
		invokeFunction(callback_data, callback_vtable, player_optional, player_optional_0, subscription_id, was_canceled and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_marketplace_service_prompt_subscription_purchase_finished_on_prompt_subscription_purchase_finished(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptSubscriptionPurchaseFinished:Connect(function(player, subscription_id, was_purchased)
		local player_optional, player_optional_0 = 0, 0;
		if (player ~= nil) then
			player_optional, player_optional_0 = 1, createPointer(player);
		end
		invokeFunction(callback_data, callback_vtable, player_optional, player_optional_0, subscription_id, was_purchased and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.method_material_service_get_base_material_override(self, material)
	local result_0 = getPointer(self):GetBaseMaterialOverride(material);
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_material_service_get_material_variant(self, material, name_addr, name_len)
	local result_0 = getPointer(self):GetMaterialVariant(material, loadString(memory, name_addr, name_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_material_service_set_base_material_override(self, material, name_addr, name_len)
	getPointer(self):SetBaseMaterialOverride(material, loadString(memory, name_addr, name_len))
end
function abi.ffi.property_getter_material_variant_custom_physical_properties(self)
	local result_0 = getPointer(self).CustomPhysicalProperties;
	return createPointer(result_0);
end
function abi.ffi.property_setter_material_variant_custom_physical_properties_set_custom_physical_properties(self, value)
	getPointer(self).CustomPhysicalProperties = getPointer(value);
end
function abi.ffi.property_getter_material_variant_material_pattern(self)
	local result_0 = getPointer(self).MaterialPattern;
	return result_0;
end
function abi.ffi.property_setter_material_variant_material_pattern_set_material_pattern(self, value)
	getPointer(self).MaterialPattern = value;
end
function abi.ffi.property_getter_material_variant_studs_per_tile(self)
	local result_0 = getPointer(self).StudsPerTile;
	return result_0;
end
function abi.ffi.property_setter_material_variant_studs_per_tile_set_studs_per_tile(self, value)
	getPointer(self).StudsPerTile = value;
end
function abi.ffi.method_memory_store_queue_remove_async(self, id_addr, id_len)
	getPointer(self):RemoveAsync(loadString(memory, id_addr, id_len))
end
function abi.ffi.method_memory_store_service_get_queue(self, name_addr, name_len, invisibility_timeout)
	local result_0 = getPointer(self):GetQueue(loadString(memory, name_addr, name_len), invisibility_timeout);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_memory_store_service_get_sorted_map(self, name_addr, name_len)
	local result_0 = getPointer(self):GetSortedMap(loadString(memory, name_addr, name_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_memory_store_sorted_map_remove_async(self, key_addr, key_len)
	getPointer(self):RemoveAsync(loadString(memory, key_addr, key_len))
end
function abi.ffi.property_getter_mouse_hit(self)
	local result_0 = getPointer(self).Hit;
	return createPointer(result_0);
end
function abi.ffi.property_getter_mouse_icon(self)
	local result_0 = getPointer(self).Icon;
	return createPointer(result_0);
end
function abi.ffi.property_setter_mouse_icon_set_icon(self, value)
	getPointer(self).Icon = getPointer(value);
end
function abi.ffi.property_getter_mouse_origin(self)
	local result_0 = getPointer(self).Origin;
	return createPointer(result_0);
end
function abi.ffi.property_getter_mouse_target(self)
	local result_0 = getPointer(self).Target;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_mouse_target_filter(self)
	local result_0 = getPointer(self).TargetFilter;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_mouse_target_filter_set_target_filter(self, value)
	getPointer(self).TargetFilter = getPointer(value);
end
function abi.ffi.property_getter_mouse_target_surface(self)
	local result_0 = getPointer(self).TargetSurface;
	return result_0;
end
function abi.ffi.property_getter_mouse_unit_ray(self)
	local result_0 = getPointer(self).UnitRay;
	return createPointer(result_0);
end
function abi.ffi.property_getter_mouse_view_size_x(self)
	local result_0 = getPointer(self).ViewSizeX;
	return result_0;
end
function abi.ffi.property_getter_mouse_view_size_y(self)
	local result_0 = getPointer(self).ViewSizeY;
	return result_0;
end
function abi.ffi.property_getter_mouse_x(self)
	local result_0 = getPointer(self).X;
	return result_0;
end
function abi.ffi.property_getter_mouse_y(self)
	local result_0 = getPointer(self).Y;
	return result_0;
end
function abi.ffi.event_mouse_button_1_down_on_button_1_down(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Button1Down:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_mouse_button_1_up_on_button_1_up(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Button1Up:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_mouse_button_2_down_on_button_2_down(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Button2Down:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_mouse_button_2_up_on_button_2_up(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Button2Up:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_mouse_idle_on_idle(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Idle:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_mouse_move_on_move(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Move:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_mouse_wheel_backward_on_wheel_backward(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).WheelBackward:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_mouse_wheel_forward_on_wheel_forward(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).WheelForward:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_network_marker_received_on_received(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Received:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_no_collision_constraint_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_no_collision_constraint_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_no_collision_constraint_part_0(self)
	local result_0 = getPointer(self).Part0;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_no_collision_constraint_part_0_set_part_0(self, value)
	getPointer(self).Part0 = getPointer(value);
end
function abi.ffi.property_getter_no_collision_constraint_part_1(self)
	local result_0 = getPointer(self).Part1;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_no_collision_constraint_part_1_set_part_1(self, value)
	getPointer(self).Part1 = getPointer(value);
end
function abi.ffi.method_pv_instance_get_pivot(self)
	local result_0 = getPointer(self):GetPivot();
	return createPointer(result_0);
end
function abi.ffi.method_pv_instance_pivot_to(self, target_c_frame)
	getPointer(self):PivotTo(getPointer(target_c_frame))
end
function abi.ffi.property_getter_base_part_anchored(self)
	local result_0 = getPointer(self).Anchored;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_base_part_anchored_set_anchored(self, value)
	getPointer(self).Anchored = value == 1;
end
function abi.ffi.property_getter_base_part_assembly_angular_velocity(self)
	local result_0 = getPointer(self).AssemblyAngularVelocity;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_part_assembly_angular_velocity_set_assembly_angular_velocity(self, value)
	getPointer(self).AssemblyAngularVelocity = getPointer(value);
end
function abi.ffi.property_getter_base_part_assembly_center_of_mass(self)
	local result_0 = getPointer(self).AssemblyCenterOfMass;
	return createPointer(result_0);
end
function abi.ffi.property_getter_base_part_assembly_linear_velocity(self)
	local result_0 = getPointer(self).AssemblyLinearVelocity;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_part_assembly_linear_velocity_set_assembly_linear_velocity(self, value)
	getPointer(self).AssemblyLinearVelocity = getPointer(value);
end
function abi.ffi.property_getter_base_part_assembly_mass(self)
	local result_0 = getPointer(self).AssemblyMass;
	return result_0;
end
function abi.ffi.property_getter_base_part_assembly_root_part(self)
	local result_0 = getPointer(self).AssemblyRootPart;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_base_part_back_surface(self)
	local result_0 = getPointer(self).BackSurface;
	return result_0;
end
function abi.ffi.property_setter_base_part_back_surface_set_back_surface(self, value)
	getPointer(self).BackSurface = value;
end
function abi.ffi.property_getter_base_part_bottom_surface(self)
	local result_0 = getPointer(self).BottomSurface;
	return result_0;
end
function abi.ffi.property_setter_base_part_bottom_surface_set_bottom_surface(self, value)
	getPointer(self).BottomSurface = value;
end
function abi.ffi.property_getter_base_part_brick_color(self)
	local result_0 = getPointer(self).BrickColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_part_brick_color_set_brick_color(self, value)
	getPointer(self).BrickColor = getPointer(value);
end
function abi.ffi.property_getter_base_part_c_frame(self)
	local result_0 = getPointer(self).CFrame;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_part_c_frame_set_c_frame(self, value)
	getPointer(self).CFrame = getPointer(value);
end
function abi.ffi.property_getter_base_part_can_collide(self)
	local result_0 = getPointer(self).CanCollide;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_base_part_can_collide_set_can_collide(self, value)
	getPointer(self).CanCollide = value == 1;
end
function abi.ffi.property_getter_base_part_can_query(self)
	local result_0 = getPointer(self).CanQuery;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_base_part_can_query_set_can_query(self, value)
	getPointer(self).CanQuery = value == 1;
end
function abi.ffi.property_getter_base_part_can_touch(self)
	local result_0 = getPointer(self).CanTouch;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_base_part_can_touch_set_can_touch(self, value)
	getPointer(self).CanTouch = value == 1;
end
function abi.ffi.property_getter_base_part_cast_shadow(self)
	local result_0 = getPointer(self).CastShadow;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_base_part_cast_shadow_set_cast_shadow(self, value)
	getPointer(self).CastShadow = value == 1;
end
function abi.ffi.property_getter_base_part_center_of_mass(self)
	local result_0 = getPointer(self).CenterOfMass;
	return createPointer(result_0);
end
function abi.ffi.property_getter_base_part_collision_group(self)
	local result_0 = getPointer(self).CollisionGroup;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_base_part_collision_group_set_collision_group(self, value_addr, value_len)
	getPointer(self).CollisionGroup = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_base_part_collision_group_id(self)
	local result_0 = getPointer(self).CollisionGroupId;
	return result_0;
end
function abi.ffi.property_setter_base_part_collision_group_id_set_collision_group_id(self, value)
	getPointer(self).CollisionGroupId = value;
end
function abi.ffi.property_getter_base_part_color(self)
	local result_0 = getPointer(self).Color;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_part_color_set_color(self, value)
	getPointer(self).Color = getPointer(value);
end
function abi.ffi.property_getter_base_part_current_physical_properties(self)
	local result_0 = getPointer(self).CurrentPhysicalProperties;
	return createPointer(result_0);
end
function abi.ffi.property_getter_base_part_custom_physical_properties(self)
	local result_0 = getPointer(self).CustomPhysicalProperties;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_part_custom_physical_properties_set_custom_physical_properties(self, value)
	getPointer(self).CustomPhysicalProperties = getPointer(value);
end
function abi.ffi.property_getter_base_part_front_surface(self)
	local result_0 = getPointer(self).FrontSurface;
	return result_0;
end
function abi.ffi.property_setter_base_part_front_surface_set_front_surface(self, value)
	getPointer(self).FrontSurface = value;
end
function abi.ffi.property_getter_base_part_left_surface(self)
	local result_0 = getPointer(self).LeftSurface;
	return result_0;
end
function abi.ffi.property_setter_base_part_left_surface_set_left_surface(self, value)
	getPointer(self).LeftSurface = value;
end
function abi.ffi.property_getter_base_part_local_transparency_modifier(self)
	local result_0 = getPointer(self).LocalTransparencyModifier;
	return result_0;
end
function abi.ffi.property_setter_base_part_local_transparency_modifier_set_local_transparency_modifier(self, value)
	getPointer(self).LocalTransparencyModifier = value;
end
function abi.ffi.property_getter_base_part_locked(self)
	local result_0 = getPointer(self).Locked;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_base_part_locked_set_locked(self, value)
	getPointer(self).Locked = value == 1;
end
function abi.ffi.property_getter_base_part_mass(self)
	local result_0 = getPointer(self).Mass;
	return result_0;
end
function abi.ffi.property_getter_base_part_massless(self)
	local result_0 = getPointer(self).Massless;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_base_part_massless_set_massless(self, value)
	getPointer(self).Massless = value == 1;
end
function abi.ffi.property_getter_base_part_material(self)
	local result_0 = getPointer(self).Material;
	return result_0;
end
function abi.ffi.property_setter_base_part_material_set_material(self, value)
	getPointer(self).Material = value;
end
function abi.ffi.property_getter_base_part_material_variant(self)
	local result_0 = getPointer(self).MaterialVariant;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_base_part_material_variant_set_material_variant(self, value_addr, value_len)
	getPointer(self).MaterialVariant = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_base_part_orientation(self)
	local result_0 = getPointer(self).Orientation;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_part_orientation_set_orientation(self, value)
	getPointer(self).Orientation = getPointer(value);
end
function abi.ffi.property_getter_base_part_pivot_offset(self)
	local result_0 = getPointer(self).PivotOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_part_pivot_offset_set_pivot_offset(self, value)
	getPointer(self).PivotOffset = getPointer(value);
end
function abi.ffi.property_getter_base_part_position(self)
	local result_0 = getPointer(self).Position;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_part_position_set_position(self, value)
	getPointer(self).Position = getPointer(value);
end
function abi.ffi.property_getter_base_part_receive_age(self)
	local result_0 = getPointer(self).ReceiveAge;
	return result_0;
end
function abi.ffi.property_getter_base_part_reflectance(self)
	local result_0 = getPointer(self).Reflectance;
	return result_0;
end
function abi.ffi.property_setter_base_part_reflectance_set_reflectance(self, value)
	getPointer(self).Reflectance = value;
end
function abi.ffi.property_getter_base_part_resize_increment(self)
	local result_0 = getPointer(self).ResizeIncrement;
	return result_0;
end
function abi.ffi.property_getter_base_part_resizeable_faces(self)
	local result_0 = getPointer(self).ResizeableFaces;
	return createPointer(result_0);
end
function abi.ffi.property_getter_base_part_right_surface(self)
	local result_0 = getPointer(self).RightSurface;
	return result_0;
end
function abi.ffi.property_setter_base_part_right_surface_set_right_surface(self, value)
	getPointer(self).RightSurface = value;
end
function abi.ffi.property_getter_base_part_root_priority(self)
	local result_0 = getPointer(self).RootPriority;
	return result_0;
end
function abi.ffi.property_setter_base_part_root_priority_set_root_priority(self, value)
	getPointer(self).RootPriority = value;
end
function abi.ffi.property_getter_base_part_rotation(self)
	local result_0 = getPointer(self).Rotation;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_part_rotation_set_rotation(self, value)
	getPointer(self).Rotation = getPointer(value);
end
function abi.ffi.property_getter_base_part_size(self)
	local result_0 = getPointer(self).Size;
	return createPointer(result_0);
end
function abi.ffi.property_setter_base_part_size_set_size(self, value)
	getPointer(self).Size = getPointer(value);
end
function abi.ffi.property_getter_base_part_top_surface(self)
	local result_0 = getPointer(self).TopSurface;
	return result_0;
end
function abi.ffi.property_setter_base_part_top_surface_set_top_surface(self, value)
	getPointer(self).TopSurface = value;
end
function abi.ffi.property_getter_base_part_transparency(self)
	local result_0 = getPointer(self).Transparency;
	return result_0;
end
function abi.ffi.property_setter_base_part_transparency_set_transparency(self, value)
	getPointer(self).Transparency = value;
end
function abi.ffi.method_base_part_apply_angular_impulse(self, impulse)
	getPointer(self):ApplyAngularImpulse(getPointer(impulse))
end
function abi.ffi.method_base_part_apply_impulse(self, impulse)
	getPointer(self):ApplyImpulse(getPointer(impulse))
end
function abi.ffi.method_base_part_apply_impulse_at_position(self, impulse, position)
	getPointer(self):ApplyImpulseAtPosition(getPointer(impulse), getPointer(position))
end
function abi.ffi.method_base_part_break_joints(self)
	getPointer(self):BreakJoints()
end
function abi.ffi.method_base_part_can_collide_with(self, part)
	local result_0 = getPointer(self):CanCollideWith(getPointer(part));
	return result_0 and 1 or 0;
end
function abi.ffi.method_base_part_get_mass(self)
	local result_0 = getPointer(self):GetMass();
	return result_0;
end
function abi.ffi.method_base_part_get_network_owner(self)
	local result_0 = getPointer(self):GetNetworkOwner();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_base_part_get_network_ownership_auto(self)
	local result_0 = getPointer(self):GetNetworkOwnershipAuto();
	return result_0 and 1 or 0;
end
function abi.ffi.method_base_part_get_root_part(self)
	local result_0 = getPointer(self):GetRootPart();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_base_part_get_velocity_at_position(self, position)
	local result_0 = getPointer(self):GetVelocityAtPosition(getPointer(position));
	return createPointer(result_0);
end
function abi.ffi.method_base_part_is_grounded(self)
	local result_0 = getPointer(self):IsGrounded();
	return result_0 and 1 or 0;
end
function abi.ffi.method_base_part_make_joints(self)
	getPointer(self):MakeJoints()
end
function abi.ffi.method_base_part_resize(self, normal_id, delta_amount)
	local result_0 = getPointer(self):Resize(normal_id, delta_amount);
	return result_0 and 1 or 0;
end
function abi.ffi.method_base_part_set_network_owner(self, player_instance)
	getPointer(self):SetNetworkOwner(getPointer(player_instance))
end
function abi.ffi.method_base_part_set_network_ownership_auto(self)
	getPointer(self):SetNetworkOwnershipAuto()
end
function abi.ffi.event_base_part_touch_ended_on_touch_ended(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TouchEnded:Connect(function(other_part)
		local other_part_optional, other_part_optional_0 = 0, 0;
		if (other_part ~= nil) then
			other_part_optional, other_part_optional_0 = 1, createPointer(other_part);
		end
		invokeFunction(callback_data, callback_vtable, other_part_optional, other_part_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_base_part_touched_on_touched(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Touched:Connect(function(part)
		invokeFunction(callback_data, callback_vtable, createPointer(part))
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_part_shape(self)
	local result_0 = getPointer(self).Shape;
	return result_0;
end
function abi.ffi.property_setter_part_shape_set_shape(self, value)
	getPointer(self).Shape = value;
end
function abi.ffi.property_getter_seat_disabled(self)
	local result_0 = getPointer(self).Disabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_seat_disabled_set_disabled(self, value)
	getPointer(self).Disabled = value == 1;
end
function abi.ffi.property_getter_seat_occupant(self)
	local result_0 = getPointer(self).Occupant;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_seat_sit(self, humanoid)
	getPointer(self):Sit(getPointer(humanoid))
end
function abi.ffi.property_getter_spawn_location_allow_team_change_on_touch(self)
	local result_0 = getPointer(self).AllowTeamChangeOnTouch;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_spawn_location_allow_team_change_on_touch_set_allow_team_change_on_touch(self, value)
	getPointer(self).AllowTeamChangeOnTouch = value == 1;
end
function abi.ffi.property_getter_spawn_location_duration(self)
	local result_0 = getPointer(self).Duration;
	return result_0;
end
function abi.ffi.property_setter_spawn_location_duration_set_duration(self, value)
	getPointer(self).Duration = value;
end
function abi.ffi.property_getter_spawn_location_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_spawn_location_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_spawn_location_neutral(self)
	local result_0 = getPointer(self).Neutral;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_spawn_location_neutral_set_neutral(self, value)
	getPointer(self).Neutral = value == 1;
end
function abi.ffi.property_getter_spawn_location_team_color(self)
	local result_0 = getPointer(self).TeamColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_spawn_location_team_color_set_team_color(self, value)
	getPointer(self).TeamColor = getPointer(value);
end
function abi.ffi.property_getter_terrain_max_extents(self)
	local result_0 = getPointer(self).MaxExtents;
	return createPointer(result_0);
end
function abi.ffi.property_getter_terrain_water_color(self)
	local result_0 = getPointer(self).WaterColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_terrain_water_color_set_water_color(self, value)
	getPointer(self).WaterColor = getPointer(value);
end
function abi.ffi.property_getter_terrain_water_reflectance(self)
	local result_0 = getPointer(self).WaterReflectance;
	return result_0;
end
function abi.ffi.property_setter_terrain_water_reflectance_set_water_reflectance(self, value)
	getPointer(self).WaterReflectance = value;
end
function abi.ffi.property_getter_terrain_water_transparency(self)
	local result_0 = getPointer(self).WaterTransparency;
	return result_0;
end
function abi.ffi.property_setter_terrain_water_transparency_set_water_transparency(self, value)
	getPointer(self).WaterTransparency = value;
end
function abi.ffi.property_getter_terrain_water_wave_size(self)
	local result_0 = getPointer(self).WaterWaveSize;
	return result_0;
end
function abi.ffi.property_setter_terrain_water_wave_size_set_water_wave_size(self, value)
	getPointer(self).WaterWaveSize = value;
end
function abi.ffi.property_getter_terrain_water_wave_speed(self)
	local result_0 = getPointer(self).WaterWaveSpeed;
	return result_0;
end
function abi.ffi.property_setter_terrain_water_wave_speed_set_water_wave_speed(self, value)
	getPointer(self).WaterWaveSpeed = value;
end
function abi.ffi.method_terrain_cell_center_to_world(self, x, y, z)
	local result_0 = getPointer(self):CellCenterToWorld(x, y, z);
	return createPointer(result_0);
end
function abi.ffi.method_terrain_cell_corner_to_world(self, x, y, z)
	local result_0 = getPointer(self):CellCornerToWorld(x, y, z);
	return createPointer(result_0);
end
function abi.ffi.method_terrain_clear(self)
	getPointer(self):Clear()
end
function abi.ffi.method_terrain_copy_region(self, region)
	local result_0 = getPointer(self):CopyRegion(getPointer(region));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_terrain_count_cells(self)
	local result_0 = getPointer(self):CountCells();
	return result_0;
end
function abi.ffi.method_terrain_fill_ball(self, center, radius, material)
	getPointer(self):FillBall(getPointer(center), radius, material)
end
function abi.ffi.method_terrain_fill_block(self, cframe, size, material)
	getPointer(self):FillBlock(getPointer(cframe), getPointer(size), material)
end
function abi.ffi.method_terrain_fill_cylinder(self, cframe, height, radius, material)
	getPointer(self):FillCylinder(getPointer(cframe), height, radius, material)
end
function abi.ffi.method_terrain_fill_region(self, region, resolution, material)
	getPointer(self):FillRegion(getPointer(region), resolution, material)
end
function abi.ffi.method_terrain_fill_wedge(self, cframe, size, material)
	getPointer(self):FillWedge(getPointer(cframe), getPointer(size), material)
end
function abi.ffi.method_terrain_get_material_color(self, material)
	local result_0 = getPointer(self):GetMaterialColor(material);
	return createPointer(result_0);
end
function abi.ffi.method_terrain_paste_region(self, region, corner, paste_empty_cells)
	getPointer(self):PasteRegion(getPointer(region), getPointer(corner), paste_empty_cells == 1)
end
function abi.ffi.method_terrain_replace_material(self, region, resolution, source_material, target_material)
	getPointer(self):ReplaceMaterial(getPointer(region), resolution, source_material, target_material)
end
function abi.ffi.method_terrain_set_material_color(self, material, value)
	getPointer(self):SetMaterialColor(material, getPointer(value))
end
function abi.ffi.method_terrain_world_to_cell(self, position)
	local result_0 = getPointer(self):WorldToCell(getPointer(position));
	return createPointer(result_0);
end
function abi.ffi.method_terrain_world_to_cell_prefer_empty(self, position)
	local result_0 = getPointer(self):WorldToCellPreferEmpty(getPointer(position));
	return createPointer(result_0);
end
function abi.ffi.method_terrain_world_to_cell_prefer_solid(self, position)
	local result_0 = getPointer(self):WorldToCellPreferSolid(getPointer(position));
	return createPointer(result_0);
end
function abi.ffi.property_getter_triangle_mesh_part_collision_fidelity(self)
	local result_0 = getPointer(self).CollisionFidelity;
	return result_0;
end
function abi.ffi.property_setter_triangle_mesh_part_collision_fidelity_set_collision_fidelity(self, value)
	getPointer(self).CollisionFidelity = value;
end
function abi.ffi.property_getter_triangle_mesh_part_mesh_size(self)
	local result_0 = getPointer(self).MeshSize;
	return createPointer(result_0);
end
function abi.ffi.property_getter_mesh_part_double_sided(self)
	local result_0 = getPointer(self).DoubleSided;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_mesh_part_double_sided_set_double_sided(self, value)
	getPointer(self).DoubleSided = value == 1;
end
function abi.ffi.property_getter_mesh_part_has_joint_offset(self)
	local result_0 = getPointer(self).HasJointOffset;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_mesh_part_has_joint_offset_set_has_joint_offset(self, value)
	getPointer(self).HasJointOffset = value == 1;
end
function abi.ffi.property_getter_mesh_part_has_skinned_mesh(self)
	local result_0 = getPointer(self).HasSkinnedMesh;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_mesh_part_has_skinned_mesh_set_has_skinned_mesh(self, value)
	getPointer(self).HasSkinnedMesh = value == 1;
end
function abi.ffi.property_getter_mesh_part_joint_offset(self)
	local result_0 = getPointer(self).JointOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_mesh_part_joint_offset_set_joint_offset(self, value)
	getPointer(self).JointOffset = getPointer(value);
end
function abi.ffi.property_getter_mesh_part_mesh_id(self)
	local result_0 = getPointer(self).MeshId;
	return createPointer(result_0);
end
function abi.ffi.property_setter_mesh_part_mesh_id_set_mesh_id(self, value)
	getPointer(self).MeshId = getPointer(value);
end
function abi.ffi.property_getter_mesh_part_render_fidelity(self)
	local result_0 = getPointer(self).RenderFidelity;
	return result_0;
end
function abi.ffi.property_setter_mesh_part_render_fidelity_set_render_fidelity(self, value)
	getPointer(self).RenderFidelity = value;
end
function abi.ffi.property_getter_mesh_part_texture_id(self)
	local result_0 = getPointer(self).TextureID;
	return createPointer(result_0);
end
function abi.ffi.property_setter_mesh_part_texture_id_set_texture_id(self, value)
	getPointer(self).TextureID = getPointer(value);
end
function abi.ffi.method_mesh_part_apply_mesh(self, mesh_part)
	getPointer(self):ApplyMesh(getPointer(mesh_part))
end
function abi.ffi.property_getter_part_operation_render_fidelity(self)
	local result_0 = getPointer(self).RenderFidelity;
	return result_0;
end
function abi.ffi.property_setter_part_operation_render_fidelity_set_render_fidelity(self, value)
	getPointer(self).RenderFidelity = value;
end
function abi.ffi.property_getter_part_operation_smoothing_angle(self)
	local result_0 = getPointer(self).SmoothingAngle;
	return result_0;
end
function abi.ffi.property_setter_part_operation_smoothing_angle_set_smoothing_angle(self, value)
	getPointer(self).SmoothingAngle = value;
end
function abi.ffi.property_getter_part_operation_triangle_count(self)
	local result_0 = getPointer(self).TriangleCount;
	return result_0;
end
function abi.ffi.property_getter_part_operation_use_part_color(self)
	local result_0 = getPointer(self).UsePartColor;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_part_operation_use_part_color_set_use_part_color(self, value)
	getPointer(self).UsePartColor = value == 1;
end
function abi.ffi.property_getter_truss_part_style(self)
	local result_0 = getPointer(self).Style;
	return result_0;
end
function abi.ffi.property_setter_truss_part_style_set_style(self, value)
	getPointer(self).Style = value;
end
function abi.ffi.property_getter_vehicle_seat_are_hinges_detected(self)
	local result_0 = getPointer(self).AreHingesDetected;
	return result_0;
end
function abi.ffi.property_getter_vehicle_seat_disabled(self)
	local result_0 = getPointer(self).Disabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_vehicle_seat_disabled_set_disabled(self, value)
	getPointer(self).Disabled = value == 1;
end
function abi.ffi.property_getter_vehicle_seat_heads_up_display(self)
	local result_0 = getPointer(self).HeadsUpDisplay;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_vehicle_seat_heads_up_display_set_heads_up_display(self, value)
	getPointer(self).HeadsUpDisplay = value == 1;
end
function abi.ffi.property_getter_vehicle_seat_max_speed(self)
	local result_0 = getPointer(self).MaxSpeed;
	return result_0;
end
function abi.ffi.property_setter_vehicle_seat_max_speed_set_max_speed(self, value)
	getPointer(self).MaxSpeed = value;
end
function abi.ffi.property_getter_vehicle_seat_occupant(self)
	local result_0 = getPointer(self).Occupant;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_vehicle_seat_steer(self)
	local result_0 = getPointer(self).Steer;
	return result_0;
end
function abi.ffi.property_setter_vehicle_seat_steer_set_steer(self, value)
	getPointer(self).Steer = value;
end
function abi.ffi.property_getter_vehicle_seat_steer_float(self)
	local result_0 = getPointer(self).SteerFloat;
	return result_0;
end
function abi.ffi.property_setter_vehicle_seat_steer_float_set_steer_float(self, value)
	getPointer(self).SteerFloat = value;
end
function abi.ffi.property_getter_vehicle_seat_throttle(self)
	local result_0 = getPointer(self).Throttle;
	return result_0;
end
function abi.ffi.property_setter_vehicle_seat_throttle_set_throttle(self, value)
	getPointer(self).Throttle = value;
end
function abi.ffi.property_getter_vehicle_seat_throttle_float(self)
	local result_0 = getPointer(self).ThrottleFloat;
	return result_0;
end
function abi.ffi.property_setter_vehicle_seat_throttle_float_set_throttle_float(self, value)
	getPointer(self).ThrottleFloat = value;
end
function abi.ffi.property_getter_vehicle_seat_torque(self)
	local result_0 = getPointer(self).Torque;
	return result_0;
end
function abi.ffi.property_setter_vehicle_seat_torque_set_torque(self, value)
	getPointer(self).Torque = value;
end
function abi.ffi.property_getter_vehicle_seat_turn_speed(self)
	local result_0 = getPointer(self).TurnSpeed;
	return result_0;
end
function abi.ffi.property_setter_vehicle_seat_turn_speed_set_turn_speed(self, value)
	getPointer(self).TurnSpeed = value;
end
function abi.ffi.method_vehicle_seat_sit(self, humanoid)
	getPointer(self):Sit(getPointer(humanoid))
end
function abi.ffi.property_getter_model_model_streaming_mode(self)
	local result_0 = getPointer(self).ModelStreamingMode;
	return result_0;
end
function abi.ffi.property_setter_model_model_streaming_mode_set_model_streaming_mode(self, value)
	getPointer(self).ModelStreamingMode = value;
end
function abi.ffi.property_getter_model_primary_part(self)
	local result_0 = getPointer(self).PrimaryPart;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_model_primary_part_set_primary_part(self, value)
	getPointer(self).PrimaryPart = getPointer(value);
end
function abi.ffi.property_getter_model_world_pivot(self)
	local result_0 = getPointer(self).WorldPivot;
	return createPointer(result_0);
end
function abi.ffi.property_setter_model_world_pivot_set_world_pivot(self, value)
	getPointer(self).WorldPivot = getPointer(value);
end
function abi.ffi.method_model_break_joints(self)
	getPointer(self):BreakJoints()
end
function abi.ffi.method_model_get_extents_size(self)
	local result_0 = getPointer(self):GetExtentsSize();
	return createPointer(result_0);
end
function abi.ffi.method_model_make_joints(self)
	getPointer(self):MakeJoints()
end
function abi.ffi.method_model_move_to(self, position)
	getPointer(self):MoveTo(getPointer(position))
end
function abi.ffi.method_model_translate_by(self, delta)
	getPointer(self):TranslateBy(getPointer(delta))
end
function abi.ffi.method_world_root_raycast(self, origin, direction, raycast_params)
	local result_0 = getPointer(self):Raycast(getPointer(origin), getPointer(direction), getPointer(raycast_params));
	return createPointer(result_0);
end
function abi.ffi.property_getter_workspace_allow_third_party_sales(self)
	local result_0 = getPointer(self).AllowThirdPartySales;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_workspace_allow_third_party_sales_set_allow_third_party_sales(self, value)
	getPointer(self).AllowThirdPartySales = value == 1;
end
function abi.ffi.property_getter_workspace_client_animator_throttling(self)
	local result_0 = getPointer(self).ClientAnimatorThrottling;
	return result_0;
end
function abi.ffi.property_setter_workspace_client_animator_throttling_set_client_animator_throttling(self, value)
	getPointer(self).ClientAnimatorThrottling = value;
end
function abi.ffi.property_getter_workspace_current_camera(self)
	local result_0 = getPointer(self).CurrentCamera;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_workspace_current_camera_set_current_camera(self, value)
	getPointer(self).CurrentCamera = getPointer(value);
end
function abi.ffi.property_getter_workspace_distributed_game_time(self)
	local result_0 = getPointer(self).DistributedGameTime;
	return result_0;
end
function abi.ffi.property_setter_workspace_distributed_game_time_set_distributed_game_time(self, value)
	getPointer(self).DistributedGameTime = value;
end
function abi.ffi.property_getter_workspace_fallen_parts_destroy_height(self)
	local result_0 = getPointer(self).FallenPartsDestroyHeight;
	return result_0;
end
function abi.ffi.property_setter_workspace_fallen_parts_destroy_height_set_fallen_parts_destroy_height(self, value)
	getPointer(self).FallenPartsDestroyHeight = value;
end
function abi.ffi.property_getter_workspace_global_wind(self)
	local result_0 = getPointer(self).GlobalWind;
	return createPointer(result_0);
end
function abi.ffi.property_setter_workspace_global_wind_set_global_wind(self, value)
	getPointer(self).GlobalWind = getPointer(value);
end
function abi.ffi.property_getter_workspace_gravity(self)
	local result_0 = getPointer(self).Gravity;
	return result_0;
end
function abi.ffi.property_setter_workspace_gravity_set_gravity(self, value)
	getPointer(self).Gravity = value;
end
function abi.ffi.property_getter_workspace_interpolation_throttling(self)
	local result_0 = getPointer(self).InterpolationThrottling;
	return result_0;
end
function abi.ffi.property_setter_workspace_interpolation_throttling_set_interpolation_throttling(self, value)
	getPointer(self).InterpolationThrottling = value;
end
function abi.ffi.property_getter_workspace_retargeting(self)
	local result_0 = getPointer(self).Retargeting;
	return result_0;
end
function abi.ffi.property_setter_workspace_retargeting_set_retargeting(self, value)
	getPointer(self).Retargeting = value;
end
function abi.ffi.property_getter_workspace_streaming_enabled(self)
	local result_0 = getPointer(self).StreamingEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_workspace_streaming_enabled_set_streaming_enabled(self, value)
	getPointer(self).StreamingEnabled = value == 1;
end
function abi.ffi.property_getter_workspace_terrain(self)
	local result_0 = getPointer(self).Terrain;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_workspace_get_num_awake_parts(self)
	local result_0 = getPointer(self):GetNumAwakeParts();
	return result_0;
end
function abi.ffi.method_workspace_get_physics_throttling(self)
	local result_0 = getPointer(self):GetPhysicsThrottling();
	return result_0;
end
function abi.ffi.method_workspace_get_real_physics_fps(self)
	local result_0 = getPointer(self):GetRealPhysicsFPS();
	return result_0;
end
function abi.ffi.method_workspace_get_server_time_now(self)
	local result_0 = getPointer(self):GetServerTimeNow();
	return result_0;
end
function abi.ffi.method_workspace_pgs_is_enabled(self)
	local result_0 = getPointer(self):PGSIsEnabled();
	return result_0 and 1 or 0;
end
function abi.ffi.event_workspace_persistent_loaded_on_persistent_loaded(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PersistentLoaded:Connect(function(player)
		local player_optional, player_optional_0 = 0, 0;
		if (player ~= nil) then
			player_optional, player_optional_0 = 1, createPointer(player);
		end
		invokeFunction(callback_data, callback_vtable, player_optional, player_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_package_link_package_id(self)
	local result_0 = getPointer(self).PackageId;
	return createPointer(result_0);
end
function abi.ffi.property_getter_package_link_version_number(self)
	local result_0 = getPointer(self).VersionNumber;
	return result_0;
end
function abi.ffi.property_setter_package_link_version_number_set_version_number(self, value)
	getPointer(self).VersionNumber = value;
end
function abi.ffi.property_getter_pages_is_finished(self)
	local result_0 = getPointer(self).IsFinished;
	return result_0 and 1 or 0;
end
function abi.ffi.method_pages_advance_to_next_page_async(self)
	getPointer(self):AdvanceToNextPageAsync()
end
function abi.ffi.property_getter_data_store_key_pages_cursor(self)
	local result_0 = getPointer(self).Cursor;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_data_store_listing_pages_cursor(self)
	local result_0 = getPointer(self).Cursor;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_particle_emitter_acceleration(self)
	local result_0 = getPointer(self).Acceleration;
	return createPointer(result_0);
end
function abi.ffi.property_setter_particle_emitter_acceleration_set_acceleration(self, value)
	getPointer(self).Acceleration = getPointer(value);
end
function abi.ffi.property_getter_particle_emitter_brightness(self)
	local result_0 = getPointer(self).Brightness;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_brightness_set_brightness(self, value)
	getPointer(self).Brightness = value;
end
function abi.ffi.property_getter_particle_emitter_color(self)
	local result_0 = getPointer(self).Color;
	return createPointer(result_0);
end
function abi.ffi.property_setter_particle_emitter_color_set_color(self, value)
	getPointer(self).Color = getPointer(value);
end
function abi.ffi.property_getter_particle_emitter_drag(self)
	local result_0 = getPointer(self).Drag;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_drag_set_drag(self, value)
	getPointer(self).Drag = value;
end
function abi.ffi.property_getter_particle_emitter_emission_direction(self)
	local result_0 = getPointer(self).EmissionDirection;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_emission_direction_set_emission_direction(self, value)
	getPointer(self).EmissionDirection = value;
end
function abi.ffi.property_getter_particle_emitter_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_particle_emitter_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_particle_emitter_flipbook_framerate(self)
	local result_0 = getPointer(self).FlipbookFramerate;
	return createPointer(result_0);
end
function abi.ffi.property_setter_particle_emitter_flipbook_framerate_set_flipbook_framerate(self, value)
	getPointer(self).FlipbookFramerate = getPointer(value);
end
function abi.ffi.property_getter_particle_emitter_flipbook_incompatible(self)
	local result_0 = getPointer(self).FlipbookIncompatible;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_particle_emitter_flipbook_incompatible_set_flipbook_incompatible(self, value_addr, value_len)
	getPointer(self).FlipbookIncompatible = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_particle_emitter_flipbook_layout(self)
	local result_0 = getPointer(self).FlipbookLayout;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_flipbook_layout_set_flipbook_layout(self, value)
	getPointer(self).FlipbookLayout = value;
end
function abi.ffi.property_getter_particle_emitter_flipbook_mode(self)
	local result_0 = getPointer(self).FlipbookMode;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_flipbook_mode_set_flipbook_mode(self, value)
	getPointer(self).FlipbookMode = value;
end
function abi.ffi.property_getter_particle_emitter_flipbook_start_random(self)
	local result_0 = getPointer(self).FlipbookStartRandom;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_particle_emitter_flipbook_start_random_set_flipbook_start_random(self, value)
	getPointer(self).FlipbookStartRandom = value == 1;
end
function abi.ffi.property_getter_particle_emitter_lifetime(self)
	local result_0 = getPointer(self).Lifetime;
	return createPointer(result_0);
end
function abi.ffi.property_setter_particle_emitter_lifetime_set_lifetime(self, value)
	getPointer(self).Lifetime = getPointer(value);
end
function abi.ffi.property_getter_particle_emitter_light_emission(self)
	local result_0 = getPointer(self).LightEmission;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_light_emission_set_light_emission(self, value)
	getPointer(self).LightEmission = value;
end
function abi.ffi.property_getter_particle_emitter_light_influence(self)
	local result_0 = getPointer(self).LightInfluence;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_light_influence_set_light_influence(self, value)
	getPointer(self).LightInfluence = value;
end
function abi.ffi.property_getter_particle_emitter_locked_to_part(self)
	local result_0 = getPointer(self).LockedToPart;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_particle_emitter_locked_to_part_set_locked_to_part(self, value)
	getPointer(self).LockedToPart = value == 1;
end
function abi.ffi.property_getter_particle_emitter_orientation(self)
	local result_0 = getPointer(self).Orientation;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_orientation_set_orientation(self, value)
	getPointer(self).Orientation = value;
end
function abi.ffi.property_getter_particle_emitter_rate(self)
	local result_0 = getPointer(self).Rate;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_rate_set_rate(self, value)
	getPointer(self).Rate = value;
end
function abi.ffi.property_getter_particle_emitter_rot_speed(self)
	local result_0 = getPointer(self).RotSpeed;
	return createPointer(result_0);
end
function abi.ffi.property_setter_particle_emitter_rot_speed_set_rot_speed(self, value)
	getPointer(self).RotSpeed = getPointer(value);
end
function abi.ffi.property_getter_particle_emitter_rotation(self)
	local result_0 = getPointer(self).Rotation;
	return createPointer(result_0);
end
function abi.ffi.property_setter_particle_emitter_rotation_set_rotation(self, value)
	getPointer(self).Rotation = getPointer(value);
end
function abi.ffi.property_getter_particle_emitter_shape(self)
	local result_0 = getPointer(self).Shape;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_shape_set_shape(self, value)
	getPointer(self).Shape = value;
end
function abi.ffi.property_getter_particle_emitter_shape_in_out(self)
	local result_0 = getPointer(self).ShapeInOut;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_shape_in_out_set_shape_in_out(self, value)
	getPointer(self).ShapeInOut = value;
end
function abi.ffi.property_getter_particle_emitter_shape_partial(self)
	local result_0 = getPointer(self).ShapePartial;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_shape_partial_set_shape_partial(self, value)
	getPointer(self).ShapePartial = value;
end
function abi.ffi.property_getter_particle_emitter_shape_style(self)
	local result_0 = getPointer(self).ShapeStyle;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_shape_style_set_shape_style(self, value)
	getPointer(self).ShapeStyle = value;
end
function abi.ffi.property_getter_particle_emitter_size(self)
	local result_0 = getPointer(self).Size;
	return createPointer(result_0);
end
function abi.ffi.property_setter_particle_emitter_size_set_size(self, value)
	getPointer(self).Size = getPointer(value);
end
function abi.ffi.property_getter_particle_emitter_speed(self)
	local result_0 = getPointer(self).Speed;
	return createPointer(result_0);
end
function abi.ffi.property_setter_particle_emitter_speed_set_speed(self, value)
	getPointer(self).Speed = getPointer(value);
end
function abi.ffi.property_getter_particle_emitter_spread_angle(self)
	local result_0 = getPointer(self).SpreadAngle;
	return createPointer(result_0);
end
function abi.ffi.property_setter_particle_emitter_spread_angle_set_spread_angle(self, value)
	getPointer(self).SpreadAngle = getPointer(value);
end
function abi.ffi.property_getter_particle_emitter_squash(self)
	local result_0 = getPointer(self).Squash;
	return createPointer(result_0);
end
function abi.ffi.property_setter_particle_emitter_squash_set_squash(self, value)
	getPointer(self).Squash = getPointer(value);
end
function abi.ffi.property_getter_particle_emitter_texture(self)
	local result_0 = getPointer(self).Texture;
	return createPointer(result_0);
end
function abi.ffi.property_setter_particle_emitter_texture_set_texture(self, value)
	getPointer(self).Texture = getPointer(value);
end
function abi.ffi.property_getter_particle_emitter_time_scale(self)
	local result_0 = getPointer(self).TimeScale;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_time_scale_set_time_scale(self, value)
	getPointer(self).TimeScale = value;
end
function abi.ffi.property_getter_particle_emitter_transparency(self)
	local result_0 = getPointer(self).Transparency;
	return createPointer(result_0);
end
function abi.ffi.property_setter_particle_emitter_transparency_set_transparency(self, value)
	getPointer(self).Transparency = getPointer(value);
end
function abi.ffi.property_getter_particle_emitter_velocity_inheritance(self)
	local result_0 = getPointer(self).VelocityInheritance;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_velocity_inheritance_set_velocity_inheritance(self, value)
	getPointer(self).VelocityInheritance = value;
end
function abi.ffi.property_getter_particle_emitter_z_offset(self)
	local result_0 = getPointer(self).ZOffset;
	return result_0;
end
function abi.ffi.property_setter_particle_emitter_z_offset_set_z_offset(self, value)
	getPointer(self).ZOffset = value;
end
function abi.ffi.method_particle_emitter_clear(self)
	getPointer(self):Clear()
end
function abi.ffi.method_particle_emitter_emit(self, particle_count)
	getPointer(self):Emit(particle_count)
end
function abi.ffi.property_getter_path_status(self)
	local result_0 = getPointer(self).Status;
	return result_0;
end
function abi.ffi.method_path_check_occlusion_async(self, start)
	local result_0 = getPointer(self):CheckOcclusionAsync(start);
	return result_0;
end
function abi.ffi.method_path_compute_async(self, start, finish)
	getPointer(self):ComputeAsync(getPointer(start), getPointer(finish))
end
function abi.ffi.event_path_blocked_on_blocked(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Blocked:Connect(function(blocked_waypoint_idx)
		invokeFunction(callback_data, callback_vtable, blocked_waypoint_idx)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_path_unblocked_on_unblocked(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Unblocked:Connect(function(unblocked_waypoint_idx)
		invokeFunction(callback_data, callback_vtable, unblocked_waypoint_idx)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_pathfinding_link_attachment_0(self)
	local result_0 = getPointer(self).Attachment0;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_pathfinding_link_attachment_0_set_attachment_0(self, value)
	getPointer(self).Attachment0 = getPointer(value);
end
function abi.ffi.property_getter_pathfinding_link_attachment_1(self)
	local result_0 = getPointer(self).Attachment1;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_pathfinding_link_attachment_1_set_attachment_1(self, value)
	getPointer(self).Attachment1 = getPointer(value);
end
function abi.ffi.property_getter_pathfinding_link_is_bidirectional(self)
	local result_0 = getPointer(self).IsBidirectional;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_pathfinding_link_is_bidirectional_set_is_bidirectional(self, value)
	getPointer(self).IsBidirectional = value == 1;
end
function abi.ffi.property_getter_pathfinding_link_label(self)
	local result_0 = getPointer(self).Label;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_pathfinding_link_label_set_label(self, value_addr, value_len)
	getPointer(self).Label = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_pathfinding_modifier_label(self)
	local result_0 = getPointer(self).Label;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_pathfinding_modifier_label_set_label(self, value_addr, value_len)
	getPointer(self).Label = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_pathfinding_modifier_pass_through(self)
	local result_0 = getPointer(self).PassThrough;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_pathfinding_modifier_pass_through_set_pass_through(self, value)
	getPointer(self).PassThrough = value == 1;
end
function abi.ffi.method_pathfinding_service_find_path_async(self, start, finish)
	local result_0 = getPointer(self):FindPathAsync(getPointer(start), getPointer(finish));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_physics_service_collision_group_contains_part(self, name_addr, name_len, part)
	local result_0 = getPointer(self):CollisionGroupContainsPart(loadString(memory, name_addr, name_len), getPointer(part));
	return result_0 and 1 or 0;
end
function abi.ffi.method_physics_service_collision_group_set_collidable(self, name_1_addr, name_1_len, name_2_addr, name_2_len, collidable)
	getPointer(self):CollisionGroupSetCollidable(loadString(memory, name_1_addr, name_1_len), loadString(memory, name_2_addr, name_2_len), collidable == 1)
end
function abi.ffi.method_physics_service_collision_groups_are_collidable(self, name_1_addr, name_1_len, name_2_addr, name_2_len)
	local result_0 = getPointer(self):CollisionGroupsAreCollidable(loadString(memory, name_1_addr, name_1_len), loadString(memory, name_2_addr, name_2_len));
	return result_0 and 1 or 0;
end
function abi.ffi.method_physics_service_create_collision_group(self, name_addr, name_len)
	local result_0 = getPointer(self):CreateCollisionGroup(loadString(memory, name_addr, name_len));
	return result_0;
end
function abi.ffi.method_physics_service_get_collision_group_id(self, name_addr, name_len)
	local result_0 = getPointer(self):GetCollisionGroupId(loadString(memory, name_addr, name_len));
	return result_0;
end
function abi.ffi.method_physics_service_get_collision_group_name(self, name)
	local result_0 = getPointer(self):GetCollisionGroupName(name);
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_physics_service_get_max_collision_groups(self)
	local result_0 = getPointer(self):GetMaxCollisionGroups();
	return result_0;
end
function abi.ffi.method_physics_service_is_collision_group_registered(self, name_addr, name_len)
	local result_0 = getPointer(self):IsCollisionGroupRegistered(loadString(memory, name_addr, name_len));
	return result_0 and 1 or 0;
end
function abi.ffi.method_physics_service_register_collision_group(self, name_addr, name_len)
	getPointer(self):RegisterCollisionGroup(loadString(memory, name_addr, name_len))
end
function abi.ffi.method_physics_service_remove_collision_group(self, name_addr, name_len)
	getPointer(self):RemoveCollisionGroup(loadString(memory, name_addr, name_len))
end
function abi.ffi.method_physics_service_rename_collision_group(self, from_addr, from_len, to_addr, to_len)
	getPointer(self):RenameCollisionGroup(loadString(memory, from_addr, from_len), loadString(memory, to_addr, to_len))
end
function abi.ffi.method_physics_service_set_part_collision_group(self, part, name_addr, name_len)
	getPointer(self):SetPartCollisionGroup(getPointer(part), loadString(memory, name_addr, name_len))
end
function abi.ffi.method_physics_service_unregister_collision_group(self, name_addr, name_len)
	getPointer(self):UnregisterCollisionGroup(loadString(memory, name_addr, name_len))
end
function abi.ffi.property_getter_player_account_age(self)
	local result_0 = getPointer(self).AccountAge;
	return result_0;
end
function abi.ffi.property_getter_player_auto_jump_enabled(self)
	local result_0 = getPointer(self).AutoJumpEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_player_auto_jump_enabled_set_auto_jump_enabled(self, value)
	getPointer(self).AutoJumpEnabled = value == 1;
end
function abi.ffi.property_getter_player_camera_max_zoom_distance(self)
	local result_0 = getPointer(self).CameraMaxZoomDistance;
	return result_0;
end
function abi.ffi.property_setter_player_camera_max_zoom_distance_set_camera_max_zoom_distance(self, value)
	getPointer(self).CameraMaxZoomDistance = value;
end
function abi.ffi.property_getter_player_camera_min_zoom_distance(self)
	local result_0 = getPointer(self).CameraMinZoomDistance;
	return result_0;
end
function abi.ffi.property_setter_player_camera_min_zoom_distance_set_camera_min_zoom_distance(self, value)
	getPointer(self).CameraMinZoomDistance = value;
end
function abi.ffi.property_getter_player_camera_mode(self)
	local result_0 = getPointer(self).CameraMode;
	return result_0;
end
function abi.ffi.property_setter_player_camera_mode_set_camera_mode(self, value)
	getPointer(self).CameraMode = value;
end
function abi.ffi.property_getter_player_can_load_character_appearance(self)
	local result_0 = getPointer(self).CanLoadCharacterAppearance;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_player_can_load_character_appearance_set_can_load_character_appearance(self, value)
	getPointer(self).CanLoadCharacterAppearance = value == 1;
end
function abi.ffi.property_getter_player_character(self)
	local result_0 = getPointer(self).Character;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_player_character_set_character(self, value)
	getPointer(self).Character = getPointer(value);
end
function abi.ffi.property_getter_player_character_appearance_id(self)
	local result_0 = getPointer(self).CharacterAppearanceId;
	return result_0;
end
function abi.ffi.property_setter_player_character_appearance_id_set_character_appearance_id(self, value)
	getPointer(self).CharacterAppearanceId = value;
end
function abi.ffi.property_getter_player_dev_camera_occlusion_mode(self)
	local result_0 = getPointer(self).DevCameraOcclusionMode;
	return result_0;
end
function abi.ffi.property_setter_player_dev_camera_occlusion_mode_set_dev_camera_occlusion_mode(self, value)
	getPointer(self).DevCameraOcclusionMode = value;
end
function abi.ffi.property_getter_player_dev_computer_camera_mode(self)
	local result_0 = getPointer(self).DevComputerCameraMode;
	return result_0;
end
function abi.ffi.property_setter_player_dev_computer_camera_mode_set_dev_computer_camera_mode(self, value)
	getPointer(self).DevComputerCameraMode = value;
end
function abi.ffi.property_getter_player_dev_computer_movement_mode(self)
	local result_0 = getPointer(self).DevComputerMovementMode;
	return result_0;
end
function abi.ffi.property_setter_player_dev_computer_movement_mode_set_dev_computer_movement_mode(self, value)
	getPointer(self).DevComputerMovementMode = value;
end
function abi.ffi.property_getter_player_dev_enable_mouse_lock(self)
	local result_0 = getPointer(self).DevEnableMouseLock;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_player_dev_enable_mouse_lock_set_dev_enable_mouse_lock(self, value)
	getPointer(self).DevEnableMouseLock = value == 1;
end
function abi.ffi.property_getter_player_dev_touch_camera_mode(self)
	local result_0 = getPointer(self).DevTouchCameraMode;
	return result_0;
end
function abi.ffi.property_setter_player_dev_touch_camera_mode_set_dev_touch_camera_mode(self, value)
	getPointer(self).DevTouchCameraMode = value;
end
function abi.ffi.property_getter_player_dev_touch_movement_mode(self)
	local result_0 = getPointer(self).DevTouchMovementMode;
	return result_0;
end
function abi.ffi.property_setter_player_dev_touch_movement_mode_set_dev_touch_movement_mode(self, value)
	getPointer(self).DevTouchMovementMode = value;
end
function abi.ffi.property_getter_player_display_name(self)
	local result_0 = getPointer(self).DisplayName;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_player_display_name_set_display_name(self, value_addr, value_len)
	getPointer(self).DisplayName = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_player_follow_user_id(self)
	local result_0 = getPointer(self).FollowUserId;
	return result_0;
end
function abi.ffi.property_getter_player_gameplay_paused(self)
	local result_0 = getPointer(self).GameplayPaused;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_player_gameplay_paused_set_gameplay_paused(self, value)
	getPointer(self).GameplayPaused = value == 1;
end
function abi.ffi.property_getter_player_health_display_distance(self)
	local result_0 = getPointer(self).HealthDisplayDistance;
	return result_0;
end
function abi.ffi.property_setter_player_health_display_distance_set_health_display_distance(self, value)
	getPointer(self).HealthDisplayDistance = value;
end
function abi.ffi.property_getter_player_locale_id(self)
	local result_0 = getPointer(self).LocaleId;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_player_membership_type(self)
	local result_0 = getPointer(self).MembershipType;
	return result_0;
end
function abi.ffi.property_getter_player_name_display_distance(self)
	local result_0 = getPointer(self).NameDisplayDistance;
	return result_0;
end
function abi.ffi.property_setter_player_name_display_distance_set_name_display_distance(self, value)
	getPointer(self).NameDisplayDistance = value;
end
function abi.ffi.property_getter_player_neutral(self)
	local result_0 = getPointer(self).Neutral;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_player_neutral_set_neutral(self, value)
	getPointer(self).Neutral = value == 1;
end
function abi.ffi.property_getter_player_replication_focus(self)
	local result_0 = getPointer(self).ReplicationFocus;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_player_replication_focus_set_replication_focus(self, value)
	getPointer(self).ReplicationFocus = getPointer(value);
end
function abi.ffi.property_getter_player_respawn_location(self)
	local result_0 = getPointer(self).RespawnLocation;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_player_respawn_location_set_respawn_location(self, value)
	getPointer(self).RespawnLocation = getPointer(value);
end
function abi.ffi.property_getter_player_team(self)
	local result_0 = getPointer(self).Team;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_player_team_set_team(self, value)
	getPointer(self).Team = getPointer(value);
end
function abi.ffi.property_getter_player_team_color(self)
	local result_0 = getPointer(self).TeamColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_player_team_color_set_team_color(self, value)
	getPointer(self).TeamColor = getPointer(value);
end
function abi.ffi.property_getter_player_user_id(self)
	local result_0 = getPointer(self).UserId;
	return result_0;
end
function abi.ffi.property_setter_player_user_id_set_user_id(self, value)
	getPointer(self).UserId = value;
end
function abi.ffi.method_player_clear_character_appearance(self)
	getPointer(self):ClearCharacterAppearance()
end
function abi.ffi.method_player_distance_from_character(self, point)
	local result_0 = getPointer(self):DistanceFromCharacter(getPointer(point));
	return result_0;
end
function abi.ffi.method_player_get_mouse(self)
	local result_0 = getPointer(self):GetMouse();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_player_get_network_ping(self)
	local result_0 = getPointer(self):GetNetworkPing();
	return result_0;
end
function abi.ffi.method_player_has_appearance_loaded(self)
	local result_0 = getPointer(self):HasAppearanceLoaded();
	return result_0 and 1 or 0;
end
function abi.ffi.method_player_kick(self, message_addr, message_len)
	getPointer(self):Kick(loadString(memory, message_addr, message_len))
end
function abi.ffi.method_player_move(self, walk_direction, relative_to_camera)
	getPointer(self):Move(getPointer(walk_direction), relative_to_camera == 1)
end
function abi.ffi.method_player_get_rank_in_group(self, group_id)
	local result_0 = getPointer(self):GetRankInGroup(group_id);
	return result_0;
end
function abi.ffi.method_player_get_role_in_group(self, group_id)
	local result_0 = getPointer(self):GetRoleInGroup(group_id);
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_player_is_friends_with(self, user_id)
	local result_0 = getPointer(self):IsFriendsWith(user_id);
	return result_0 and 1 or 0;
end
function abi.ffi.method_player_is_in_group(self, group_id)
	local result_0 = getPointer(self):IsInGroup(group_id);
	return result_0 and 1 or 0;
end
function abi.ffi.method_player_load_character(self)
	getPointer(self):LoadCharacter()
end
function abi.ffi.method_player_load_character_with_humanoid_description(self, humanoid_description)
	getPointer(self):LoadCharacterWithHumanoidDescription(getPointer(humanoid_description))
end
function abi.ffi.method_player_request_stream_around_async(self, position, time_out)
	getPointer(self):RequestStreamAroundAsync(getPointer(position), time_out)
end
function abi.ffi.event_player_character_added_on_character_added(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).CharacterAdded:Connect(function(character)
		local character_optional, character_optional_0 = 0, 0;
		if (character ~= nil) then
			character_optional, character_optional_0 = 1, createPointer(character);
		end
		invokeFunction(callback_data, callback_vtable, character_optional, character_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_player_character_appearance_loaded_on_character_appearance_loaded(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).CharacterAppearanceLoaded:Connect(function(character)
		local character_optional, character_optional_0 = 0, 0;
		if (character ~= nil) then
			character_optional, character_optional_0 = 1, createPointer(character);
		end
		invokeFunction(callback_data, callback_vtable, character_optional, character_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_player_character_removing_on_character_removing(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).CharacterRemoving:Connect(function(character)
		local character_optional, character_optional_0 = 0, 0;
		if (character ~= nil) then
			character_optional, character_optional_0 = 1, createPointer(character);
		end
		invokeFunction(callback_data, callback_vtable, character_optional, character_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_player_chatted_on_chatted(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Chatted:Connect(function(message, recipient)
		local message_len = #message;
		local message_content = allocString(message_len);
		for i = 1, message_len do
			storeU8(memory, message_content + (i - 1), string.byte(message:sub(i, i)));
		end
		local recipient_optional, recipient_optional_0 = 0, 0;
		if (recipient ~= nil) then
			recipient_optional, recipient_optional_0 = 1, createPointer(recipient);
		end
		invokeFunction(callback_data, callback_vtable, message_content, message_len, recipient_optional, recipient_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_player_idled_on_idled(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Idled:Connect(function(time)
		invokeFunction(callback_data, callback_vtable, time)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_player_on_teleport_on_on_teleport(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).OnTeleport:Connect(function(teleport_state, place_id, spawn_name)
		local spawn_name_len = #spawn_name;
		local spawn_name_content = allocString(spawn_name_len);
		for i = 1, spawn_name_len do
			storeU8(memory, spawn_name_content + (i - 1), string.byte(spawn_name:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, teleport_state, place_id, spawn_name_content, spawn_name_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.method_player_scripts_clear_computer_camera_movement_modes(self)
	getPointer(self):ClearComputerCameraMovementModes()
end
function abi.ffi.method_player_scripts_clear_computer_movement_modes(self)
	getPointer(self):ClearComputerMovementModes()
end
function abi.ffi.method_player_scripts_clear_touch_camera_movement_modes(self)
	getPointer(self):ClearTouchCameraMovementModes()
end
function abi.ffi.method_player_scripts_clear_touch_movement_modes(self)
	getPointer(self):ClearTouchMovementModes()
end
function abi.ffi.method_player_scripts_register_computer_camera_movement_mode(self, camera_movement_mode)
	getPointer(self):RegisterComputerCameraMovementMode(camera_movement_mode)
end
function abi.ffi.method_player_scripts_register_computer_movement_mode(self, movement_mode)
	getPointer(self):RegisterComputerMovementMode(movement_mode)
end
function abi.ffi.method_player_scripts_register_touch_camera_movement_mode(self, camera_movement_mode)
	getPointer(self):RegisterTouchCameraMovementMode(camera_movement_mode)
end
function abi.ffi.method_player_scripts_register_touch_movement_mode(self, movement_mode)
	getPointer(self):RegisterTouchMovementMode(movement_mode)
end
function abi.ffi.property_getter_players_bubble_chat(self)
	local result_0 = getPointer(self).BubbleChat;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_players_character_auto_loads(self)
	local result_0 = getPointer(self).CharacterAutoLoads;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_players_character_auto_loads_set_character_auto_loads(self, value)
	getPointer(self).CharacterAutoLoads = value == 1;
end
function abi.ffi.property_getter_players_classic_chat(self)
	local result_0 = getPointer(self).ClassicChat;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_players_local_player(self)
	local result_0 = getPointer(self).LocalPlayer;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_players_max_players(self)
	local result_0 = getPointer(self).MaxPlayers;
	return result_0;
end
function abi.ffi.property_getter_players_preferred_players(self)
	local result_0 = getPointer(self).PreferredPlayers;
	return result_0;
end
function abi.ffi.property_getter_players_respawn_time(self)
	local result_0 = getPointer(self).RespawnTime;
	return result_0;
end
function abi.ffi.property_setter_players_respawn_time_set_respawn_time(self, value)
	getPointer(self).RespawnTime = value;
end
function abi.ffi.method_players_get_player_by_user_id(self, user_id)
	local result_0 = getPointer(self):GetPlayerByUserId(user_id);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_players_get_player_from_character(self, character)
	local result_0 = getPointer(self):GetPlayerFromCharacter(getPointer(character));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_players_create_humanoid_model_from_description(self, description, rig_type, asset_type_verification)
	local result_0 = getPointer(self):CreateHumanoidModelFromDescription(getPointer(description), rig_type, asset_type_verification);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_players_create_humanoid_model_from_user_id(self, user_id)
	local result_0 = getPointer(self):CreateHumanoidModelFromUserId(user_id);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_players_get_friends_async(self, user_id)
	local result_0 = getPointer(self):GetFriendsAsync(user_id);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_players_get_humanoid_description_from_outfit_id(self, outfit_id)
	local result_0 = getPointer(self):GetHumanoidDescriptionFromOutfitId(outfit_id);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_players_get_humanoid_description_from_user_id(self, user_id)
	local result_0 = getPointer(self):GetHumanoidDescriptionFromUserId(user_id);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_players_get_name_from_user_id_async(self, user_id)
	local result_0 = getPointer(self):GetNameFromUserIdAsync(user_id);
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_players_get_user_id_from_name_async(self, user_name_addr, user_name_len)
	local result_0 = getPointer(self):GetUserIdFromNameAsync(loadString(memory, user_name_addr, user_name_len));
	return result_0;
end
function abi.ffi.event_players_player_added_on_player_added(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PlayerAdded:Connect(function(player)
		local player_optional, player_optional_0 = 0, 0;
		if (player ~= nil) then
			player_optional, player_optional_0 = 1, createPointer(player);
		end
		invokeFunction(callback_data, callback_vtable, player_optional, player_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_players_player_membership_changed_on_player_membership_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PlayerMembershipChanged:Connect(function(player)
		local player_optional, player_optional_0 = 0, 0;
		if (player ~= nil) then
			player_optional, player_optional_0 = 1, createPointer(player);
		end
		invokeFunction(callback_data, callback_vtable, player_optional, player_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_players_player_removing_on_player_removing(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PlayerRemoving:Connect(function(player)
		local player_optional, player_optional_0 = 0, 0;
		if (player ~= nil) then
			player_optional, player_optional_0 = 1, createPointer(player);
		end
		invokeFunction(callback_data, callback_vtable, player_optional, player_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_pose_base_easing_direction(self)
	local result_0 = getPointer(self).EasingDirection;
	return result_0;
end
function abi.ffi.property_setter_pose_base_easing_direction_set_easing_direction(self, value)
	getPointer(self).EasingDirection = value;
end
function abi.ffi.property_getter_pose_base_easing_style(self)
	local result_0 = getPointer(self).EasingStyle;
	return result_0;
end
function abi.ffi.property_setter_pose_base_easing_style_set_easing_style(self, value)
	getPointer(self).EasingStyle = value;
end
function abi.ffi.property_getter_pose_base_weight(self)
	local result_0 = getPointer(self).Weight;
	return result_0;
end
function abi.ffi.property_setter_pose_base_weight_set_weight(self, value)
	getPointer(self).Weight = value;
end
function abi.ffi.property_getter_number_pose_value(self)
	local result_0 = getPointer(self).Value;
	return result_0;
end
function abi.ffi.property_setter_number_pose_value_set_value(self, value)
	getPointer(self).Value = value;
end
function abi.ffi.property_getter_pose_c_frame(self)
	local result_0 = getPointer(self).CFrame;
	return createPointer(result_0);
end
function abi.ffi.property_setter_pose_c_frame_set_c_frame(self, value)
	getPointer(self).CFrame = getPointer(value);
end
function abi.ffi.method_pose_add_sub_pose(self, pose)
	getPointer(self):AddSubPose(getPointer(pose))
end
function abi.ffi.method_pose_remove_sub_pose(self, pose)
	getPointer(self):RemoveSubPose(getPointer(pose))
end
function abi.ffi.property_getter_post_effect_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_post_effect_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_bloom_effect_intensity(self)
	local result_0 = getPointer(self).Intensity;
	return result_0;
end
function abi.ffi.property_setter_bloom_effect_intensity_set_intensity(self, value)
	getPointer(self).Intensity = value;
end
function abi.ffi.property_getter_bloom_effect_size(self)
	local result_0 = getPointer(self).Size;
	return result_0;
end
function abi.ffi.property_setter_bloom_effect_size_set_size(self, value)
	getPointer(self).Size = value;
end
function abi.ffi.property_getter_bloom_effect_threshold(self)
	local result_0 = getPointer(self).Threshold;
	return result_0;
end
function abi.ffi.property_setter_bloom_effect_threshold_set_threshold(self, value)
	getPointer(self).Threshold = value;
end
function abi.ffi.property_getter_blur_effect_size(self)
	local result_0 = getPointer(self).Size;
	return result_0;
end
function abi.ffi.property_setter_blur_effect_size_set_size(self, value)
	getPointer(self).Size = value;
end
function abi.ffi.property_getter_color_correction_effect_brightness(self)
	local result_0 = getPointer(self).Brightness;
	return result_0;
end
function abi.ffi.property_setter_color_correction_effect_brightness_set_brightness(self, value)
	getPointer(self).Brightness = value;
end
function abi.ffi.property_getter_color_correction_effect_contrast(self)
	local result_0 = getPointer(self).Contrast;
	return result_0;
end
function abi.ffi.property_setter_color_correction_effect_contrast_set_contrast(self, value)
	getPointer(self).Contrast = value;
end
function abi.ffi.property_getter_color_correction_effect_saturation(self)
	local result_0 = getPointer(self).Saturation;
	return result_0;
end
function abi.ffi.property_setter_color_correction_effect_saturation_set_saturation(self, value)
	getPointer(self).Saturation = value;
end
function abi.ffi.property_getter_color_correction_effect_tint_color(self)
	local result_0 = getPointer(self).TintColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_color_correction_effect_tint_color_set_tint_color(self, value)
	getPointer(self).TintColor = getPointer(value);
end
function abi.ffi.property_getter_depth_of_field_effect_far_intensity(self)
	local result_0 = getPointer(self).FarIntensity;
	return result_0;
end
function abi.ffi.property_setter_depth_of_field_effect_far_intensity_set_far_intensity(self, value)
	getPointer(self).FarIntensity = value;
end
function abi.ffi.property_getter_depth_of_field_effect_focus_distance(self)
	local result_0 = getPointer(self).FocusDistance;
	return result_0;
end
function abi.ffi.property_setter_depth_of_field_effect_focus_distance_set_focus_distance(self, value)
	getPointer(self).FocusDistance = value;
end
function abi.ffi.property_getter_depth_of_field_effect_in_focus_radius(self)
	local result_0 = getPointer(self).InFocusRadius;
	return result_0;
end
function abi.ffi.property_setter_depth_of_field_effect_in_focus_radius_set_in_focus_radius(self, value)
	getPointer(self).InFocusRadius = value;
end
function abi.ffi.property_getter_depth_of_field_effect_near_intensity(self)
	local result_0 = getPointer(self).NearIntensity;
	return result_0;
end
function abi.ffi.property_setter_depth_of_field_effect_near_intensity_set_near_intensity(self, value)
	getPointer(self).NearIntensity = value;
end
function abi.ffi.property_getter_sun_rays_effect_intensity(self)
	local result_0 = getPointer(self).Intensity;
	return result_0;
end
function abi.ffi.property_setter_sun_rays_effect_intensity_set_intensity(self, value)
	getPointer(self).Intensity = value;
end
function abi.ffi.property_getter_sun_rays_effect_spread(self)
	local result_0 = getPointer(self).Spread;
	return result_0;
end
function abi.ffi.property_setter_sun_rays_effect_spread_set_spread(self, value)
	getPointer(self).Spread = value;
end
function abi.ffi.property_getter_proximity_prompt_action_text(self)
	local result_0 = getPointer(self).ActionText;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_proximity_prompt_action_text_set_action_text(self, value_addr, value_len)
	getPointer(self).ActionText = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_proximity_prompt_auto_localize(self)
	local result_0 = getPointer(self).AutoLocalize;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_proximity_prompt_auto_localize_set_auto_localize(self, value)
	getPointer(self).AutoLocalize = value == 1;
end
function abi.ffi.property_getter_proximity_prompt_clickable_prompt(self)
	local result_0 = getPointer(self).ClickablePrompt;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_proximity_prompt_clickable_prompt_set_clickable_prompt(self, value)
	getPointer(self).ClickablePrompt = value == 1;
end
function abi.ffi.property_getter_proximity_prompt_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_proximity_prompt_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_proximity_prompt_exclusivity(self)
	local result_0 = getPointer(self).Exclusivity;
	return result_0;
end
function abi.ffi.property_setter_proximity_prompt_exclusivity_set_exclusivity(self, value)
	getPointer(self).Exclusivity = value;
end
function abi.ffi.property_getter_proximity_prompt_gamepad_key_code(self)
	local result_0 = getPointer(self).GamepadKeyCode;
	return result_0;
end
function abi.ffi.property_setter_proximity_prompt_gamepad_key_code_set_gamepad_key_code(self, value)
	getPointer(self).GamepadKeyCode = value;
end
function abi.ffi.property_getter_proximity_prompt_hold_duration(self)
	local result_0 = getPointer(self).HoldDuration;
	return result_0;
end
function abi.ffi.property_setter_proximity_prompt_hold_duration_set_hold_duration(self, value)
	getPointer(self).HoldDuration = value;
end
function abi.ffi.property_getter_proximity_prompt_keyboard_key_code(self)
	local result_0 = getPointer(self).KeyboardKeyCode;
	return result_0;
end
function abi.ffi.property_setter_proximity_prompt_keyboard_key_code_set_keyboard_key_code(self, value)
	getPointer(self).KeyboardKeyCode = value;
end
function abi.ffi.property_getter_proximity_prompt_max_activation_distance(self)
	local result_0 = getPointer(self).MaxActivationDistance;
	return result_0;
end
function abi.ffi.property_setter_proximity_prompt_max_activation_distance_set_max_activation_distance(self, value)
	getPointer(self).MaxActivationDistance = value;
end
function abi.ffi.property_getter_proximity_prompt_object_text(self)
	local result_0 = getPointer(self).ObjectText;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_proximity_prompt_object_text_set_object_text(self, value_addr, value_len)
	getPointer(self).ObjectText = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_proximity_prompt_requires_line_of_sight(self)
	local result_0 = getPointer(self).RequiresLineOfSight;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_proximity_prompt_requires_line_of_sight_set_requires_line_of_sight(self, value)
	getPointer(self).RequiresLineOfSight = value == 1;
end
function abi.ffi.property_getter_proximity_prompt_root_localization_table(self)
	local result_0 = getPointer(self).RootLocalizationTable;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_proximity_prompt_root_localization_table_set_root_localization_table(self, value)
	getPointer(self).RootLocalizationTable = getPointer(value);
end
function abi.ffi.property_getter_proximity_prompt_style(self)
	local result_0 = getPointer(self).Style;
	return result_0;
end
function abi.ffi.property_setter_proximity_prompt_style_set_style(self, value)
	getPointer(self).Style = value;
end
function abi.ffi.property_getter_proximity_prompt_ui_offset(self)
	local result_0 = getPointer(self).UIOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_proximity_prompt_ui_offset_set_ui_offset(self, value)
	getPointer(self).UIOffset = getPointer(value);
end
function abi.ffi.method_proximity_prompt_input_hold_begin(self)
	getPointer(self):InputHoldBegin()
end
function abi.ffi.method_proximity_prompt_input_hold_end(self)
	getPointer(self):InputHoldEnd()
end
function abi.ffi.event_proximity_prompt_prompt_button_hold_began_on_prompt_button_hold_began(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptButtonHoldBegan:Connect(function(player_who_triggered)
		local player_who_triggered_optional, player_who_triggered_optional_0 = 0, 0;
		if (player_who_triggered ~= nil) then
			player_who_triggered_optional, player_who_triggered_optional_0 = 1, createPointer(player_who_triggered);
		end
		invokeFunction(callback_data, callback_vtable, player_who_triggered_optional, player_who_triggered_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_proximity_prompt_prompt_button_hold_ended_on_prompt_button_hold_ended(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptButtonHoldEnded:Connect(function(player_who_triggered)
		local player_who_triggered_optional, player_who_triggered_optional_0 = 0, 0;
		if (player_who_triggered ~= nil) then
			player_who_triggered_optional, player_who_triggered_optional_0 = 1, createPointer(player_who_triggered);
		end
		invokeFunction(callback_data, callback_vtable, player_who_triggered_optional, player_who_triggered_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_proximity_prompt_prompt_hidden_on_prompt_hidden(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptHidden:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_proximity_prompt_prompt_shown_on_prompt_shown(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptShown:Connect(function(input_type)
		invokeFunction(callback_data, callback_vtable, input_type)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_proximity_prompt_trigger_ended_on_trigger_ended(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TriggerEnded:Connect(function(player_who_triggered)
		local player_who_triggered_optional, player_who_triggered_optional_0 = 0, 0;
		if (player_who_triggered ~= nil) then
			player_who_triggered_optional, player_who_triggered_optional_0 = 1, createPointer(player_who_triggered);
		end
		invokeFunction(callback_data, callback_vtable, player_who_triggered_optional, player_who_triggered_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_proximity_prompt_triggered_on_triggered(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Triggered:Connect(function(player_who_triggered)
		local player_who_triggered_optional, player_who_triggered_optional_0 = 0, 0;
		if (player_who_triggered ~= nil) then
			player_who_triggered_optional, player_who_triggered_optional_0 = 1, createPointer(player_who_triggered);
		end
		invokeFunction(callback_data, callback_vtable, player_who_triggered_optional, player_who_triggered_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_proximity_prompt_service_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_proximity_prompt_service_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_proximity_prompt_service_max_prompts_visible(self)
	local result_0 = getPointer(self).MaxPromptsVisible;
	return result_0;
end
function abi.ffi.property_setter_proximity_prompt_service_max_prompts_visible_set_max_prompts_visible(self, value)
	getPointer(self).MaxPromptsVisible = value;
end
function abi.ffi.event_proximity_prompt_service_prompt_button_hold_began_on_prompt_button_hold_began(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptButtonHoldBegan:Connect(function(prompt, player_who_triggered)
		local prompt_optional, prompt_optional_0 = 0, 0;
		if (prompt ~= nil) then
			prompt_optional, prompt_optional_0 = 1, createPointer(prompt);
		end
		local player_who_triggered_optional, player_who_triggered_optional_0 = 0, 0;
		if (player_who_triggered ~= nil) then
			player_who_triggered_optional, player_who_triggered_optional_0 = 1, createPointer(player_who_triggered);
		end
		invokeFunction(callback_data, callback_vtable, prompt_optional, prompt_optional_0, player_who_triggered_optional, player_who_triggered_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_proximity_prompt_service_prompt_button_hold_ended_on_prompt_button_hold_ended(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptButtonHoldEnded:Connect(function(prompt, player_who_triggered)
		local prompt_optional, prompt_optional_0 = 0, 0;
		if (prompt ~= nil) then
			prompt_optional, prompt_optional_0 = 1, createPointer(prompt);
		end
		local player_who_triggered_optional, player_who_triggered_optional_0 = 0, 0;
		if (player_who_triggered ~= nil) then
			player_who_triggered_optional, player_who_triggered_optional_0 = 1, createPointer(player_who_triggered);
		end
		invokeFunction(callback_data, callback_vtable, prompt_optional, prompt_optional_0, player_who_triggered_optional, player_who_triggered_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_proximity_prompt_service_prompt_hidden_on_prompt_hidden(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptHidden:Connect(function(prompt)
		local prompt_optional, prompt_optional_0 = 0, 0;
		if (prompt ~= nil) then
			prompt_optional, prompt_optional_0 = 1, createPointer(prompt);
		end
		invokeFunction(callback_data, callback_vtable, prompt_optional, prompt_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_proximity_prompt_service_prompt_shown_on_prompt_shown(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptShown:Connect(function(prompt, input_type)
		local prompt_optional, prompt_optional_0 = 0, 0;
		if (prompt ~= nil) then
			prompt_optional, prompt_optional_0 = 1, createPointer(prompt);
		end
		invokeFunction(callback_data, callback_vtable, prompt_optional, prompt_optional_0, input_type)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_proximity_prompt_service_prompt_trigger_ended_on_prompt_trigger_ended(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptTriggerEnded:Connect(function(prompt, player_who_triggered)
		local prompt_optional, prompt_optional_0 = 0, 0;
		if (prompt ~= nil) then
			prompt_optional, prompt_optional_0 = 1, createPointer(prompt);
		end
		local player_who_triggered_optional, player_who_triggered_optional_0 = 0, 0;
		if (player_who_triggered ~= nil) then
			player_who_triggered_optional, player_who_triggered_optional_0 = 1, createPointer(player_who_triggered);
		end
		invokeFunction(callback_data, callback_vtable, prompt_optional, prompt_optional_0, player_who_triggered_optional, player_who_triggered_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_proximity_prompt_service_prompt_triggered_on_prompt_triggered(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PromptTriggered:Connect(function(prompt, player_who_triggered)
		local prompt_optional, prompt_optional_0 = 0, 0;
		if (prompt ~= nil) then
			prompt_optional, prompt_optional_0 = 1, createPointer(prompt);
		end
		local player_who_triggered_optional, player_who_triggered_optional_0 = 0, 0;
		if (player_who_triggered ~= nil) then
			player_who_triggered_optional, player_who_triggered_optional_0 = 1, createPointer(player_who_triggered);
		end
		invokeFunction(callback_data, callback_vtable, prompt_optional, prompt_optional_0, player_who_triggered_optional, player_who_triggered_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_remote_event_on_server_event(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).OnServerEvent:Connect(function(player, ...)
		local values_tuple = { ... };
		local values_len = select("#", ...);
		local values_tuple_vec = allocVec(values_len * 4)
		for i = 1, values_len do
			local values_tuple_vec_value = values_tuple[i];
			storeU32(memory, values_tuple_vec + (i - 1) * 4 + 0, createPointer(values_tuple_vec_value));
		end
		invokeFunction(callback_data, callback_vtable, createPointer(player), values_tuple_vec, values_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_remote_event_on_client_event(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).OnClientEvent:Connect(function(...)
		local values_tuple = { ... };
		local values_len = select("#", ...);
		local values_tuple_vec = allocVec(values_len * 4)
		for i = 1, values_len do
			local values_tuple_vec_value = values_tuple[i];
			storeU32(memory, values_tuple_vec + (i - 1) * 4 + 0, createPointer(values_tuple_vec_value));
		end
		invokeFunction(callback_data, callback_vtable, values_tuple_vec, values_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.method_remote_event_fire_all_clients(self, values_addr, values_len)
	local values_vec = table.create(values_len);
	for i = 1, values_len do
		local vecElement_0 = loadU32(memory, values_addr + (i - 1) * 4 + 0)
		values_vec[i] = getPointer(vecElement_0);
	end
	getPointer(self):FireAllClients(unpack(values_vec, 1, values_len))
end
function abi.ffi.method_remote_event_fire_client(self, player, values_addr, values_len)
	local values_vec = table.create(values_len);
	for i = 1, values_len do
		local vecElement_0 = loadU32(memory, values_addr + (i - 1) * 4 + 0)
		values_vec[i] = getPointer(vecElement_0);
	end
	getPointer(self):FireClient(getPointer(player), unpack(values_vec, 1, values_len))
end
function abi.ffi.method_remote_event_fire_server(self, values_addr, values_len)
	local values_vec = table.create(values_len);
	for i = 1, values_len do
		local vecElement_0 = loadU32(memory, values_addr + (i - 1) * 4 + 0)
		values_vec[i] = getPointer(vecElement_0);
	end
	getPointer(self):FireServer(unpack(values_vec, 1, values_len))
end
function abi.ffi.callback_remote_function_on_server_invoke(self, callback_data, callback_vtable)
	getPointer(self).OnServerInvoke = function(player, ...)
		local values_tuple = { ... };
		local values_len = select("#", ...);
		local values_tuple_vec = allocVec(values_len * 4)
		for i = 1, values_len do
			local values_tuple_vec_value = values_tuple[i];
			storeU32(memory, values_tuple_vec + (i - 1) * 4 + 0, createPointer(values_tuple_vec_value));
		end
		local result_0, result_1 = invokeFunction(callback_data, callback_vtable, createPointer(player), values_tuple_vec, values_len);
		local callback_vec = table.create(result_1);
		for i = 1, result_1 do
			local vecElement_0 = loadU32(memory, result_0 + (i - 1) * 4 + 0)
			callback_vec[i] = getPointer(vecElement_0);
		end
		return unpack(callback_vec, 1, result_1);
	end;
end
function abi.ffi.callback_remote_function_on_client_invoke(self, callback_data, callback_vtable)
	getPointer(self).OnClientInvoke = function(...)
		local values_tuple = { ... };
		local values_len = select("#", ...);
		local values_tuple_vec = allocVec(values_len * 4)
		for i = 1, values_len do
			local values_tuple_vec_value = values_tuple[i];
			storeU32(memory, values_tuple_vec + (i - 1) * 4 + 0, createPointer(values_tuple_vec_value));
		end
		local result_0, result_1 = invokeFunction(callback_data, callback_vtable, values_tuple_vec, values_len);
		local callback_vec = table.create(result_1);
		for i = 1, result_1 do
			local vecElement_0 = loadU32(memory, result_0 + (i - 1) * 4 + 0)
			callback_vec[i] = getPointer(vecElement_0);
		end
		return unpack(callback_vec, 1, result_1);
	end;
end
function abi.ffi.method_remote_function_invoke_server(self, values_addr, values_len)
	local values_vec = table.create(values_len);
	for i = 1, values_len do
		local vecElement_0 = loadU32(memory, values_addr + (i - 1) * 4 + 0)
		values_vec[i] = getPointer(vecElement_0);
	end
	local result_0 = { getPointer(self):InvokeServer(unpack(values_vec, 1, values_len)) }
	local result_0_len = #result_0;
	local result_0_vec = allocVec(result_0_len * 4)
	for i = 1, result_0_len do
		local result_0_vec_value = result_0[i];
		storeU32(memory, result_0_vec + (i - 1) * 4 + 0, createPointer(result_0_vec_value));
	end
	return result_0_vec, result_0_len;
end
function abi.ffi.method_remote_function_invoke_client(self, player, values_addr, values_len)
	local values_vec = table.create(values_len);
	for i = 1, values_len do
		local vecElement_0 = loadU32(memory, values_addr + (i - 1) * 4 + 0)
		values_vec[i] = getPointer(vecElement_0);
	end
	local result_0 = { getPointer(self):InvokeClient(getPointer(player), unpack(values_vec, 1, values_len)) }
	local result_0_len = #result_0;
	local result_0_vec = allocVec(result_0_len * 4)
	for i = 1, result_0_len do
		local result_0_vec_value = result_0[i];
		storeU32(memory, result_0_vec + (i - 1) * 4 + 0, createPointer(result_0_vec_value));
	end
	return result_0_vec, result_0_len;
end
function abi.ffi.method_replicated_first_remove_default_loading_screen(self)
	getPointer(self):RemoveDefaultLoadingScreen()
end
function abi.ffi.property_getter_rotation_curve_length(self)
	local result_0 = getPointer(self).Length;
	return result_0;
end
function abi.ffi.method_rotation_curve_get_key_at_index(self, index)
	local result_0 = getPointer(self):GetKeyAtIndex(index);
	return createPointer(result_0);
end
function abi.ffi.method_rotation_curve_get_value_at_time(self, time)
	local result_0 = getPointer(self):GetValueAtTime(time);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_rotation_curve_remove_key_at_index(self, starting_index, count)
	local result_0 = getPointer(self):RemoveKeyAtIndex(starting_index, count);
	return result_0;
end
function abi.ffi.method_run_service_is_client(self)
	local result_0 = getPointer(self):IsClient();
	return result_0 and 1 or 0;
end
function abi.ffi.method_run_service_is_run_mode(self)
	local result_0 = getPointer(self):IsRunMode();
	return result_0 and 1 or 0;
end
function abi.ffi.method_run_service_is_running(self)
	local result_0 = getPointer(self):IsRunning();
	return result_0 and 1 or 0;
end
function abi.ffi.method_run_service_is_server(self)
	local result_0 = getPointer(self):IsServer();
	return result_0 and 1 or 0;
end
function abi.ffi.method_run_service_is_studio(self)
	local result_0 = getPointer(self):IsStudio();
	return result_0 and 1 or 0;
end
function abi.ffi.method_run_service_unbind_from_render_step(self, name_addr, name_len)
	getPointer(self):UnbindFromRenderStep(loadString(memory, name_addr, name_len))
end
function abi.ffi.event_run_service_heartbeat_on_heartbeat(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Heartbeat:Connect(function(delta_time)
		invokeFunction(callback_data, callback_vtable, delta_time)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_run_service_post_simulation_on_post_simulation(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PostSimulation:Connect(function(delta_time_sim)
		invokeFunction(callback_data, callback_vtable, delta_time_sim)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_run_service_pre_animation_on_pre_animation(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PreAnimation:Connect(function(delta_time_sim)
		invokeFunction(callback_data, callback_vtable, delta_time_sim)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_run_service_pre_render_on_pre_render(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PreRender:Connect(function(delta_time_render)
		invokeFunction(callback_data, callback_vtable, delta_time_render)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_run_service_pre_simulation_on_pre_simulation(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PreSimulation:Connect(function(delta_time_sim)
		invokeFunction(callback_data, callback_vtable, delta_time_sim)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_run_service_render_stepped_on_render_stepped(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).RenderStepped:Connect(function(delta_time)
		invokeFunction(callback_data, callback_vtable, delta_time)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_run_service_stepped_on_stepped(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Stepped:Connect(function(time, delta_time)
		invokeFunction(callback_data, callback_vtable, time, delta_time)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_screenshot_hud_camera_button_icon(self)
	local result_0 = getPointer(self).CameraButtonIcon;
	return createPointer(result_0);
end
function abi.ffi.property_setter_screenshot_hud_camera_button_icon_set_camera_button_icon(self, value)
	getPointer(self).CameraButtonIcon = getPointer(value);
end
function abi.ffi.property_getter_screenshot_hud_camera_button_position(self)
	local result_0 = getPointer(self).CameraButtonPosition;
	return createPointer(result_0);
end
function abi.ffi.property_setter_screenshot_hud_camera_button_position_set_camera_button_position(self, value)
	getPointer(self).CameraButtonPosition = getPointer(value);
end
function abi.ffi.property_getter_screenshot_hud_close_button_position(self)
	local result_0 = getPointer(self).CloseButtonPosition;
	return createPointer(result_0);
end
function abi.ffi.property_setter_screenshot_hud_close_button_position_set_close_button_position(self, value)
	getPointer(self).CloseButtonPosition = getPointer(value);
end
function abi.ffi.property_getter_screenshot_hud_close_when_screenshot_taken(self)
	local result_0 = getPointer(self).CloseWhenScreenshotTaken;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_screenshot_hud_close_when_screenshot_taken_set_close_when_screenshot_taken(self, value)
	getPointer(self).CloseWhenScreenshotTaken = value == 1;
end
function abi.ffi.property_getter_screenshot_hud_experience_name_overlay_enabled(self)
	local result_0 = getPointer(self).ExperienceNameOverlayEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_screenshot_hud_experience_name_overlay_enabled_set_experience_name_overlay_enabled(self, value)
	getPointer(self).ExperienceNameOverlayEnabled = value == 1;
end
function abi.ffi.property_getter_screenshot_hud_overlay_font(self)
	local result_0 = getPointer(self).OverlayFont;
	return result_0;
end
function abi.ffi.property_setter_screenshot_hud_overlay_font_set_overlay_font(self, value)
	getPointer(self).OverlayFont = value;
end
function abi.ffi.property_getter_screenshot_hud_username_overlay_enabled(self)
	local result_0 = getPointer(self).UsernameOverlayEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_screenshot_hud_username_overlay_enabled_set_username_overlay_enabled(self, value)
	getPointer(self).UsernameOverlayEnabled = value == 1;
end
function abi.ffi.property_getter_screenshot_hud_visible(self)
	local result_0 = getPointer(self).Visible;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_screenshot_hud_visible_set_visible(self, value)
	getPointer(self).Visible = value == 1;
end
function abi.ffi.event_script_context_error_on_error(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Error:Connect(function(message, stack_trace, script)
		local message_len = #message;
		local message_content = allocString(message_len);
		for i = 1, message_len do
			storeU8(memory, message_content + (i - 1), string.byte(message:sub(i, i)));
		end
		local stack_trace_len = #stack_trace;
		local stack_trace_content = allocString(stack_trace_len);
		for i = 1, stack_trace_len do
			storeU8(memory, stack_trace_content + (i - 1), string.byte(stack_trace:sub(i, i)));
		end
		local script_optional, script_optional_0 = 0, 0;
		if (script ~= nil) then
			script_optional, script_optional_0 = 1, createPointer(script);
		end
		invokeFunction(callback_data, callback_vtable, message_content, message_len, stack_trace_content, stack_trace_len, script_optional, script_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.method_service_provider_find_service(self, class_name_addr, class_name_len)
	local result_0 = getPointer(self):FindService(loadString(memory, class_name_addr, class_name_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_service_provider_get_service(self, class_name_addr, class_name_len)
	local result_0 = getPointer(self):GetService(loadString(memory, class_name_addr, class_name_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.event_service_provider_close_on_close(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Close:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_service_provider_service_added_on_service_added(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).ServiceAdded:Connect(function(service)
		local service_optional, service_optional_0 = 0, 0;
		if (service ~= nil) then
			service_optional, service_optional_0 = 1, createPointer(service);
		end
		invokeFunction(callback_data, callback_vtable, service_optional, service_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_service_provider_service_removing_on_service_removing(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).ServiceRemoving:Connect(function(service)
		local service_optional, service_optional_0 = 0, 0;
		if (service ~= nil) then
			service_optional, service_optional_0 = 1, createPointer(service);
		end
		invokeFunction(callback_data, callback_vtable, service_optional, service_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_data_model_creator_id(self)
	local result_0 = getPointer(self).CreatorId;
	return result_0;
end
function abi.ffi.property_getter_data_model_creator_type(self)
	local result_0 = getPointer(self).CreatorType;
	return result_0;
end
function abi.ffi.property_getter_data_model_game_id(self)
	local result_0 = getPointer(self).GameId;
	return result_0;
end
function abi.ffi.property_getter_data_model_genre(self)
	local result_0 = getPointer(self).Genre;
	return result_0;
end
function abi.ffi.property_getter_data_model_job_id(self)
	local result_0 = getPointer(self).JobId;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_data_model_place_id(self)
	local result_0 = getPointer(self).PlaceId;
	return result_0;
end
function abi.ffi.property_getter_data_model_place_version(self)
	local result_0 = getPointer(self).PlaceVersion;
	return result_0;
end
function abi.ffi.property_getter_data_model_private_server_id(self)
	local result_0 = getPointer(self).PrivateServerId;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_data_model_private_server_owner_id(self)
	local result_0 = getPointer(self).PrivateServerOwnerId;
	return result_0;
end
function abi.ffi.property_getter_data_model_workspace(self)
	local result_0 = getPointer(self).Workspace;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_data_model_is_loaded(self)
	local result_0 = getPointer(self):IsLoaded();
	return result_0 and 1 or 0;
end
function abi.ffi.event_data_model_graphics_quality_change_request_on_graphics_quality_change_request(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).GraphicsQualityChangeRequest:Connect(function(better_quality)
		invokeFunction(callback_data, callback_vtable, better_quality and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_data_model_loaded_on_loaded(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Loaded:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.method_user_settings_is_user_feature_enabled(self, name_addr, name_len)
	local result_0 = getPointer(self):IsUserFeatureEnabled(loadString(memory, name_addr, name_len));
	return result_0 and 1 or 0;
end
function abi.ffi.method_user_settings_reset(self)
	getPointer(self):Reset()
end
function abi.ffi.property_getter_sky_celestial_bodies_shown(self)
	local result_0 = getPointer(self).CelestialBodiesShown;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_sky_celestial_bodies_shown_set_celestial_bodies_shown(self, value)
	getPointer(self).CelestialBodiesShown = value == 1;
end
function abi.ffi.property_getter_sky_moon_angular_size(self)
	local result_0 = getPointer(self).MoonAngularSize;
	return result_0;
end
function abi.ffi.property_setter_sky_moon_angular_size_set_moon_angular_size(self, value)
	getPointer(self).MoonAngularSize = value;
end
function abi.ffi.property_getter_sky_moon_texture_id(self)
	local result_0 = getPointer(self).MoonTextureId;
	return createPointer(result_0);
end
function abi.ffi.property_setter_sky_moon_texture_id_set_moon_texture_id(self, value)
	getPointer(self).MoonTextureId = getPointer(value);
end
function abi.ffi.property_getter_sky_skybox_bk(self)
	local result_0 = getPointer(self).SkyboxBk;
	return createPointer(result_0);
end
function abi.ffi.property_setter_sky_skybox_bk_set_skybox_bk(self, value)
	getPointer(self).SkyboxBk = getPointer(value);
end
function abi.ffi.property_getter_sky_skybox_dn(self)
	local result_0 = getPointer(self).SkyboxDn;
	return createPointer(result_0);
end
function abi.ffi.property_setter_sky_skybox_dn_set_skybox_dn(self, value)
	getPointer(self).SkyboxDn = getPointer(value);
end
function abi.ffi.property_getter_sky_skybox_ft(self)
	local result_0 = getPointer(self).SkyboxFt;
	return createPointer(result_0);
end
function abi.ffi.property_setter_sky_skybox_ft_set_skybox_ft(self, value)
	getPointer(self).SkyboxFt = getPointer(value);
end
function abi.ffi.property_getter_sky_skybox_lf(self)
	local result_0 = getPointer(self).SkyboxLf;
	return createPointer(result_0);
end
function abi.ffi.property_setter_sky_skybox_lf_set_skybox_lf(self, value)
	getPointer(self).SkyboxLf = getPointer(value);
end
function abi.ffi.property_getter_sky_skybox_rt(self)
	local result_0 = getPointer(self).SkyboxRt;
	return createPointer(result_0);
end
function abi.ffi.property_setter_sky_skybox_rt_set_skybox_rt(self, value)
	getPointer(self).SkyboxRt = getPointer(value);
end
function abi.ffi.property_getter_sky_skybox_up(self)
	local result_0 = getPointer(self).SkyboxUp;
	return createPointer(result_0);
end
function abi.ffi.property_setter_sky_skybox_up_set_skybox_up(self, value)
	getPointer(self).SkyboxUp = getPointer(value);
end
function abi.ffi.property_getter_sky_star_count(self)
	local result_0 = getPointer(self).StarCount;
	return result_0;
end
function abi.ffi.property_setter_sky_star_count_set_star_count(self, value)
	getPointer(self).StarCount = value;
end
function abi.ffi.property_getter_sky_sun_angular_size(self)
	local result_0 = getPointer(self).SunAngularSize;
	return result_0;
end
function abi.ffi.property_setter_sky_sun_angular_size_set_sun_angular_size(self, value)
	getPointer(self).SunAngularSize = value;
end
function abi.ffi.property_getter_sky_sun_texture_id(self)
	local result_0 = getPointer(self).SunTextureId;
	return createPointer(result_0);
end
function abi.ffi.property_setter_sky_sun_texture_id_set_sun_texture_id(self, value)
	getPointer(self).SunTextureId = getPointer(value);
end
function abi.ffi.property_getter_smoke_color(self)
	local result_0 = getPointer(self).Color;
	return createPointer(result_0);
end
function abi.ffi.property_setter_smoke_color_set_color(self, value)
	getPointer(self).Color = getPointer(value);
end
function abi.ffi.property_getter_smoke_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_smoke_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_smoke_opacity(self)
	local result_0 = getPointer(self).Opacity;
	return result_0;
end
function abi.ffi.property_setter_smoke_opacity_set_opacity(self, value)
	getPointer(self).Opacity = value;
end
function abi.ffi.property_getter_smoke_rise_velocity(self)
	local result_0 = getPointer(self).RiseVelocity;
	return result_0;
end
function abi.ffi.property_setter_smoke_rise_velocity_set_rise_velocity(self, value)
	getPointer(self).RiseVelocity = value;
end
function abi.ffi.property_getter_smoke_size(self)
	local result_0 = getPointer(self).Size;
	return result_0;
end
function abi.ffi.property_setter_smoke_size_set_size(self, value)
	getPointer(self).Size = value;
end
function abi.ffi.property_getter_smoke_time_scale(self)
	local result_0 = getPointer(self).TimeScale;
	return result_0;
end
function abi.ffi.property_setter_smoke_time_scale_set_time_scale(self, value)
	getPointer(self).TimeScale = value;
end
function abi.ffi.method_social_service_prompt_game_invite(self, player)
	getPointer(self):PromptGameInvite(getPointer(player))
end
function abi.ffi.method_social_service_can_send_game_invite_async(self, player, recipient_id)
	local result_0 = getPointer(self):CanSendGameInviteAsync(getPointer(player), recipient_id);
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_sound_is_loaded(self)
	local result_0 = getPointer(self).IsLoaded;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_sound_is_paused(self)
	local result_0 = getPointer(self).IsPaused;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_sound_is_playing(self)
	local result_0 = getPointer(self).IsPlaying;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_sound_loop_region(self)
	local result_0 = getPointer(self).LoopRegion;
	return createPointer(result_0);
end
function abi.ffi.property_setter_sound_loop_region_set_loop_region(self, value)
	getPointer(self).LoopRegion = getPointer(value);
end
function abi.ffi.property_getter_sound_looped(self)
	local result_0 = getPointer(self).Looped;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_sound_looped_set_looped(self, value)
	getPointer(self).Looped = value == 1;
end
function abi.ffi.property_getter_sound_play_on_remove(self)
	local result_0 = getPointer(self).PlayOnRemove;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_sound_play_on_remove_set_play_on_remove(self, value)
	getPointer(self).PlayOnRemove = value == 1;
end
function abi.ffi.property_getter_sound_playback_loudness(self)
	local result_0 = getPointer(self).PlaybackLoudness;
	return result_0;
end
function abi.ffi.property_getter_sound_playback_region(self)
	local result_0 = getPointer(self).PlaybackRegion;
	return createPointer(result_0);
end
function abi.ffi.property_setter_sound_playback_region_set_playback_region(self, value)
	getPointer(self).PlaybackRegion = getPointer(value);
end
function abi.ffi.property_getter_sound_playback_regions_enabled(self)
	local result_0 = getPointer(self).PlaybackRegionsEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_sound_playback_regions_enabled_set_playback_regions_enabled(self, value)
	getPointer(self).PlaybackRegionsEnabled = value == 1;
end
function abi.ffi.property_getter_sound_playback_speed(self)
	local result_0 = getPointer(self).PlaybackSpeed;
	return result_0;
end
function abi.ffi.property_setter_sound_playback_speed_set_playback_speed(self, value)
	getPointer(self).PlaybackSpeed = value;
end
function abi.ffi.property_getter_sound_playing(self)
	local result_0 = getPointer(self).Playing;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_sound_playing_set_playing(self, value)
	getPointer(self).Playing = value == 1;
end
function abi.ffi.property_getter_sound_roll_off_max_distance(self)
	local result_0 = getPointer(self).RollOffMaxDistance;
	return result_0;
end
function abi.ffi.property_setter_sound_roll_off_max_distance_set_roll_off_max_distance(self, value)
	getPointer(self).RollOffMaxDistance = value;
end
function abi.ffi.property_getter_sound_roll_off_min_distance(self)
	local result_0 = getPointer(self).RollOffMinDistance;
	return result_0;
end
function abi.ffi.property_setter_sound_roll_off_min_distance_set_roll_off_min_distance(self, value)
	getPointer(self).RollOffMinDistance = value;
end
function abi.ffi.property_getter_sound_roll_off_mode(self)
	local result_0 = getPointer(self).RollOffMode;
	return result_0;
end
function abi.ffi.property_setter_sound_roll_off_mode_set_roll_off_mode(self, value)
	getPointer(self).RollOffMode = value;
end
function abi.ffi.property_getter_sound_sound_group(self)
	local result_0 = getPointer(self).SoundGroup;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_sound_sound_group_set_sound_group(self, value)
	getPointer(self).SoundGroup = getPointer(value);
end
function abi.ffi.property_getter_sound_sound_id(self)
	local result_0 = getPointer(self).SoundId;
	return createPointer(result_0);
end
function abi.ffi.property_setter_sound_sound_id_set_sound_id(self, value)
	getPointer(self).SoundId = getPointer(value);
end
function abi.ffi.property_getter_sound_time_length(self)
	local result_0 = getPointer(self).TimeLength;
	return result_0;
end
function abi.ffi.property_getter_sound_time_position(self)
	local result_0 = getPointer(self).TimePosition;
	return result_0;
end
function abi.ffi.property_setter_sound_time_position_set_time_position(self, value)
	getPointer(self).TimePosition = value;
end
function abi.ffi.property_getter_sound_volume(self)
	local result_0 = getPointer(self).Volume;
	return result_0;
end
function abi.ffi.property_setter_sound_volume_set_volume(self, value)
	getPointer(self).Volume = value;
end
function abi.ffi.method_sound_pause(self)
	getPointer(self):Pause()
end
function abi.ffi.method_sound_play(self)
	getPointer(self):Play()
end
function abi.ffi.method_sound_resume(self)
	getPointer(self):Resume()
end
function abi.ffi.method_sound_stop(self)
	getPointer(self):Stop()
end
function abi.ffi.event_sound_did_loop_on_did_loop(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).DidLoop:Connect(function(sound_id, num_of_times_looped)
		local sound_id_len = #sound_id;
		local sound_id_content = allocString(sound_id_len);
		for i = 1, sound_id_len do
			storeU8(memory, sound_id_content + (i - 1), string.byte(sound_id:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, sound_id_content, sound_id_len, num_of_times_looped)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_sound_ended_on_ended(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Ended:Connect(function(sound_id)
		local sound_id_len = #sound_id;
		local sound_id_content = allocString(sound_id_len);
		for i = 1, sound_id_len do
			storeU8(memory, sound_id_content + (i - 1), string.byte(sound_id:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, sound_id_content, sound_id_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_sound_loaded_on_loaded(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Loaded:Connect(function(sound_id)
		local sound_id_len = #sound_id;
		local sound_id_content = allocString(sound_id_len);
		for i = 1, sound_id_len do
			storeU8(memory, sound_id_content + (i - 1), string.byte(sound_id:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, sound_id_content, sound_id_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_sound_paused_on_paused(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Paused:Connect(function(sound_id)
		local sound_id_len = #sound_id;
		local sound_id_content = allocString(sound_id_len);
		for i = 1, sound_id_len do
			storeU8(memory, sound_id_content + (i - 1), string.byte(sound_id:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, sound_id_content, sound_id_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_sound_played_on_played(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Played:Connect(function(sound_id)
		local sound_id_len = #sound_id;
		local sound_id_content = allocString(sound_id_len);
		for i = 1, sound_id_len do
			storeU8(memory, sound_id_content + (i - 1), string.byte(sound_id:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, sound_id_content, sound_id_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_sound_resumed_on_resumed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Resumed:Connect(function(sound_id)
		local sound_id_len = #sound_id;
		local sound_id_content = allocString(sound_id_len);
		for i = 1, sound_id_len do
			storeU8(memory, sound_id_content + (i - 1), string.byte(sound_id:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, sound_id_content, sound_id_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_sound_stopped_on_stopped(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Stopped:Connect(function(sound_id)
		local sound_id_len = #sound_id;
		local sound_id_content = allocString(sound_id_len);
		for i = 1, sound_id_len do
			storeU8(memory, sound_id_content + (i - 1), string.byte(sound_id:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, sound_id_content, sound_id_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_sound_effect_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_sound_effect_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_sound_effect_priority(self)
	local result_0 = getPointer(self).Priority;
	return result_0;
end
function abi.ffi.property_setter_sound_effect_priority_set_priority(self, value)
	getPointer(self).Priority = value;
end
function abi.ffi.property_getter_chorus_sound_effect_depth(self)
	local result_0 = getPointer(self).Depth;
	return result_0;
end
function abi.ffi.property_setter_chorus_sound_effect_depth_set_depth(self, value)
	getPointer(self).Depth = value;
end
function abi.ffi.property_getter_chorus_sound_effect_mix(self)
	local result_0 = getPointer(self).Mix;
	return result_0;
end
function abi.ffi.property_setter_chorus_sound_effect_mix_set_mix(self, value)
	getPointer(self).Mix = value;
end
function abi.ffi.property_getter_chorus_sound_effect_rate(self)
	local result_0 = getPointer(self).Rate;
	return result_0;
end
function abi.ffi.property_setter_chorus_sound_effect_rate_set_rate(self, value)
	getPointer(self).Rate = value;
end
function abi.ffi.property_getter_compressor_sound_effect_attack(self)
	local result_0 = getPointer(self).Attack;
	return result_0;
end
function abi.ffi.property_setter_compressor_sound_effect_attack_set_attack(self, value)
	getPointer(self).Attack = value;
end
function abi.ffi.property_getter_compressor_sound_effect_gain_makeup(self)
	local result_0 = getPointer(self).GainMakeup;
	return result_0;
end
function abi.ffi.property_setter_compressor_sound_effect_gain_makeup_set_gain_makeup(self, value)
	getPointer(self).GainMakeup = value;
end
function abi.ffi.property_getter_compressor_sound_effect_ratio(self)
	local result_0 = getPointer(self).Ratio;
	return result_0;
end
function abi.ffi.property_setter_compressor_sound_effect_ratio_set_ratio(self, value)
	getPointer(self).Ratio = value;
end
function abi.ffi.property_getter_compressor_sound_effect_release(self)
	local result_0 = getPointer(self).Release;
	return result_0;
end
function abi.ffi.property_setter_compressor_sound_effect_release_set_release(self, value)
	getPointer(self).Release = value;
end
function abi.ffi.property_getter_compressor_sound_effect_side_chain(self)
	local result_0 = getPointer(self).SideChain;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_compressor_sound_effect_side_chain_set_side_chain(self, value)
	getPointer(self).SideChain = getPointer(value);
end
function abi.ffi.property_getter_compressor_sound_effect_threshold(self)
	local result_0 = getPointer(self).Threshold;
	return result_0;
end
function abi.ffi.property_setter_compressor_sound_effect_threshold_set_threshold(self, value)
	getPointer(self).Threshold = value;
end
function abi.ffi.property_getter_channel_selector_sound_effect_channel(self)
	local result_0 = getPointer(self).Channel;
	return result_0;
end
function abi.ffi.property_setter_channel_selector_sound_effect_channel_set_channel(self, value)
	getPointer(self).Channel = value;
end
function abi.ffi.property_getter_distortion_sound_effect_level(self)
	local result_0 = getPointer(self).Level;
	return result_0;
end
function abi.ffi.property_setter_distortion_sound_effect_level_set_level(self, value)
	getPointer(self).Level = value;
end
function abi.ffi.property_getter_echo_sound_effect_delay(self)
	local result_0 = getPointer(self).Delay;
	return result_0;
end
function abi.ffi.property_setter_echo_sound_effect_delay_set_delay(self, value)
	getPointer(self).Delay = value;
end
function abi.ffi.property_getter_echo_sound_effect_dry_level(self)
	local result_0 = getPointer(self).DryLevel;
	return result_0;
end
function abi.ffi.property_setter_echo_sound_effect_dry_level_set_dry_level(self, value)
	getPointer(self).DryLevel = value;
end
function abi.ffi.property_getter_echo_sound_effect_feedback(self)
	local result_0 = getPointer(self).Feedback;
	return result_0;
end
function abi.ffi.property_setter_echo_sound_effect_feedback_set_feedback(self, value)
	getPointer(self).Feedback = value;
end
function abi.ffi.property_getter_echo_sound_effect_wet_level(self)
	local result_0 = getPointer(self).WetLevel;
	return result_0;
end
function abi.ffi.property_setter_echo_sound_effect_wet_level_set_wet_level(self, value)
	getPointer(self).WetLevel = value;
end
function abi.ffi.property_getter_equalizer_sound_effect_high_gain(self)
	local result_0 = getPointer(self).HighGain;
	return result_0;
end
function abi.ffi.property_setter_equalizer_sound_effect_high_gain_set_high_gain(self, value)
	getPointer(self).HighGain = value;
end
function abi.ffi.property_getter_equalizer_sound_effect_low_gain(self)
	local result_0 = getPointer(self).LowGain;
	return result_0;
end
function abi.ffi.property_setter_equalizer_sound_effect_low_gain_set_low_gain(self, value)
	getPointer(self).LowGain = value;
end
function abi.ffi.property_getter_equalizer_sound_effect_mid_gain(self)
	local result_0 = getPointer(self).MidGain;
	return result_0;
end
function abi.ffi.property_setter_equalizer_sound_effect_mid_gain_set_mid_gain(self, value)
	getPointer(self).MidGain = value;
end
function abi.ffi.property_getter_flange_sound_effect_depth(self)
	local result_0 = getPointer(self).Depth;
	return result_0;
end
function abi.ffi.property_setter_flange_sound_effect_depth_set_depth(self, value)
	getPointer(self).Depth = value;
end
function abi.ffi.property_getter_flange_sound_effect_mix(self)
	local result_0 = getPointer(self).Mix;
	return result_0;
end
function abi.ffi.property_setter_flange_sound_effect_mix_set_mix(self, value)
	getPointer(self).Mix = value;
end
function abi.ffi.property_getter_flange_sound_effect_rate(self)
	local result_0 = getPointer(self).Rate;
	return result_0;
end
function abi.ffi.property_setter_flange_sound_effect_rate_set_rate(self, value)
	getPointer(self).Rate = value;
end
function abi.ffi.property_getter_pitch_shift_sound_effect_octave(self)
	local result_0 = getPointer(self).Octave;
	return result_0;
end
function abi.ffi.property_setter_pitch_shift_sound_effect_octave_set_octave(self, value)
	getPointer(self).Octave = value;
end
function abi.ffi.property_getter_reverb_sound_effect_decay_time(self)
	local result_0 = getPointer(self).DecayTime;
	return result_0;
end
function abi.ffi.property_setter_reverb_sound_effect_decay_time_set_decay_time(self, value)
	getPointer(self).DecayTime = value;
end
function abi.ffi.property_getter_reverb_sound_effect_density(self)
	local result_0 = getPointer(self).Density;
	return result_0;
end
function abi.ffi.property_setter_reverb_sound_effect_density_set_density(self, value)
	getPointer(self).Density = value;
end
function abi.ffi.property_getter_reverb_sound_effect_diffusion(self)
	local result_0 = getPointer(self).Diffusion;
	return result_0;
end
function abi.ffi.property_setter_reverb_sound_effect_diffusion_set_diffusion(self, value)
	getPointer(self).Diffusion = value;
end
function abi.ffi.property_getter_reverb_sound_effect_dry_level(self)
	local result_0 = getPointer(self).DryLevel;
	return result_0;
end
function abi.ffi.property_setter_reverb_sound_effect_dry_level_set_dry_level(self, value)
	getPointer(self).DryLevel = value;
end
function abi.ffi.property_getter_reverb_sound_effect_wet_level(self)
	local result_0 = getPointer(self).WetLevel;
	return result_0;
end
function abi.ffi.property_setter_reverb_sound_effect_wet_level_set_wet_level(self, value)
	getPointer(self).WetLevel = value;
end
function abi.ffi.property_getter_tremolo_sound_effect_depth(self)
	local result_0 = getPointer(self).Depth;
	return result_0;
end
function abi.ffi.property_setter_tremolo_sound_effect_depth_set_depth(self, value)
	getPointer(self).Depth = value;
end
function abi.ffi.property_getter_tremolo_sound_effect_duty(self)
	local result_0 = getPointer(self).Duty;
	return result_0;
end
function abi.ffi.property_setter_tremolo_sound_effect_duty_set_duty(self, value)
	getPointer(self).Duty = value;
end
function abi.ffi.property_getter_tremolo_sound_effect_frequency(self)
	local result_0 = getPointer(self).Frequency;
	return result_0;
end
function abi.ffi.property_setter_tremolo_sound_effect_frequency_set_frequency(self, value)
	getPointer(self).Frequency = value;
end
function abi.ffi.property_getter_sound_group_volume(self)
	local result_0 = getPointer(self).Volume;
	return result_0;
end
function abi.ffi.property_setter_sound_group_volume_set_volume(self, value)
	getPointer(self).Volume = value;
end
function abi.ffi.property_getter_sound_service_ambient_reverb(self)
	local result_0 = getPointer(self).AmbientReverb;
	return result_0;
end
function abi.ffi.property_setter_sound_service_ambient_reverb_set_ambient_reverb(self, value)
	getPointer(self).AmbientReverb = value;
end
function abi.ffi.property_getter_sound_service_distance_factor(self)
	local result_0 = getPointer(self).DistanceFactor;
	return result_0;
end
function abi.ffi.property_setter_sound_service_distance_factor_set_distance_factor(self, value)
	getPointer(self).DistanceFactor = value;
end
function abi.ffi.property_getter_sound_service_doppler_scale(self)
	local result_0 = getPointer(self).DopplerScale;
	return result_0;
end
function abi.ffi.property_setter_sound_service_doppler_scale_set_doppler_scale(self, value)
	getPointer(self).DopplerScale = value;
end
function abi.ffi.property_getter_sound_service_respect_filtering_enabled(self)
	local result_0 = getPointer(self).RespectFilteringEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_sound_service_respect_filtering_enabled_set_respect_filtering_enabled(self, value)
	getPointer(self).RespectFilteringEnabled = value == 1;
end
function abi.ffi.property_getter_sound_service_rolloff_scale(self)
	local result_0 = getPointer(self).RolloffScale;
	return result_0;
end
function abi.ffi.property_setter_sound_service_rolloff_scale_set_rolloff_scale(self, value)
	getPointer(self).RolloffScale = value;
end
function abi.ffi.method_sound_service_play_local_sound(self, sound)
	getPointer(self):PlayLocalSound(getPointer(sound))
end
function abi.ffi.property_getter_sparkles_color(self)
	local result_0 = getPointer(self).Color;
	return createPointer(result_0);
end
function abi.ffi.property_setter_sparkles_color_set_color(self, value)
	getPointer(self).Color = getPointer(value);
end
function abi.ffi.property_getter_sparkles_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_sparkles_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_sparkles_sparkle_color(self)
	local result_0 = getPointer(self).SparkleColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_sparkles_sparkle_color_set_sparkle_color(self, value)
	getPointer(self).SparkleColor = getPointer(value);
end
function abi.ffi.property_getter_sparkles_time_scale(self)
	local result_0 = getPointer(self).TimeScale;
	return result_0;
end
function abi.ffi.property_setter_sparkles_time_scale_set_time_scale(self, value)
	getPointer(self).TimeScale = value;
end
function abi.ffi.property_getter_speaker_channel_count(self)
	local result_0 = getPointer(self).ChannelCount;
	return result_0;
end
function abi.ffi.property_getter_speaker_playback_loudness(self)
	local result_0 = getPointer(self).PlaybackLoudness;
	return result_0;
end
function abi.ffi.property_getter_speaker_roll_off_max_distance(self)
	local result_0 = getPointer(self).RollOffMaxDistance;
	return result_0;
end
function abi.ffi.property_setter_speaker_roll_off_max_distance_set_roll_off_max_distance(self, value)
	getPointer(self).RollOffMaxDistance = value;
end
function abi.ffi.property_getter_speaker_roll_off_min_distance(self)
	local result_0 = getPointer(self).RollOffMinDistance;
	return result_0;
end
function abi.ffi.property_setter_speaker_roll_off_min_distance_set_roll_off_min_distance(self, value)
	getPointer(self).RollOffMinDistance = value;
end
function abi.ffi.property_getter_speaker_roll_off_mode(self)
	local result_0 = getPointer(self).RollOffMode;
	return result_0;
end
function abi.ffi.property_setter_speaker_roll_off_mode_set_roll_off_mode(self, value)
	getPointer(self).RollOffMode = value;
end
function abi.ffi.property_getter_speaker_sound_group(self)
	local result_0 = getPointer(self).SoundGroup;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_speaker_sound_group_set_sound_group(self, value)
	getPointer(self).SoundGroup = getPointer(value);
end
function abi.ffi.property_getter_speaker_source(self)
	local result_0 = getPointer(self).Source;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_speaker_source_set_source(self, value)
	getPointer(self).Source = getPointer(value);
end
function abi.ffi.property_getter_speaker_volume(self)
	local result_0 = getPointer(self).Volume;
	return result_0;
end
function abi.ffi.property_setter_speaker_volume_set_volume(self, value)
	getPointer(self).Volume = value;
end
function abi.ffi.property_getter_starter_player_allow_custom_animations(self)
	local result_0 = getPointer(self).AllowCustomAnimations;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_starter_player_allow_custom_animations_set_allow_custom_animations(self, value)
	getPointer(self).AllowCustomAnimations = value == 1;
end
function abi.ffi.property_getter_starter_player_auto_jump_enabled(self)
	local result_0 = getPointer(self).AutoJumpEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_starter_player_auto_jump_enabled_set_auto_jump_enabled(self, value)
	getPointer(self).AutoJumpEnabled = value == 1;
end
function abi.ffi.property_getter_starter_player_camera_max_zoom_distance(self)
	local result_0 = getPointer(self).CameraMaxZoomDistance;
	return result_0;
end
function abi.ffi.property_setter_starter_player_camera_max_zoom_distance_set_camera_max_zoom_distance(self, value)
	getPointer(self).CameraMaxZoomDistance = value;
end
function abi.ffi.property_getter_starter_player_camera_min_zoom_distance(self)
	local result_0 = getPointer(self).CameraMinZoomDistance;
	return result_0;
end
function abi.ffi.property_setter_starter_player_camera_min_zoom_distance_set_camera_min_zoom_distance(self, value)
	getPointer(self).CameraMinZoomDistance = value;
end
function abi.ffi.property_getter_starter_player_camera_mode(self)
	local result_0 = getPointer(self).CameraMode;
	return result_0;
end
function abi.ffi.property_setter_starter_player_camera_mode_set_camera_mode(self, value)
	getPointer(self).CameraMode = value;
end
function abi.ffi.property_getter_starter_player_character_jump_height(self)
	local result_0 = getPointer(self).CharacterJumpHeight;
	return result_0;
end
function abi.ffi.property_setter_starter_player_character_jump_height_set_character_jump_height(self, value)
	getPointer(self).CharacterJumpHeight = value;
end
function abi.ffi.property_getter_starter_player_character_jump_power(self)
	local result_0 = getPointer(self).CharacterJumpPower;
	return result_0;
end
function abi.ffi.property_setter_starter_player_character_jump_power_set_character_jump_power(self, value)
	getPointer(self).CharacterJumpPower = value;
end
function abi.ffi.property_getter_starter_player_character_max_slope_angle(self)
	local result_0 = getPointer(self).CharacterMaxSlopeAngle;
	return result_0;
end
function abi.ffi.property_setter_starter_player_character_max_slope_angle_set_character_max_slope_angle(self, value)
	getPointer(self).CharacterMaxSlopeAngle = value;
end
function abi.ffi.property_getter_starter_player_character_use_jump_power(self)
	local result_0 = getPointer(self).CharacterUseJumpPower;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_starter_player_character_use_jump_power_set_character_use_jump_power(self, value)
	getPointer(self).CharacterUseJumpPower = value == 1;
end
function abi.ffi.property_getter_starter_player_character_walk_speed(self)
	local result_0 = getPointer(self).CharacterWalkSpeed;
	return result_0;
end
function abi.ffi.property_setter_starter_player_character_walk_speed_set_character_walk_speed(self, value)
	getPointer(self).CharacterWalkSpeed = value;
end
function abi.ffi.property_getter_starter_player_dev_camera_occlusion_mode(self)
	local result_0 = getPointer(self).DevCameraOcclusionMode;
	return result_0;
end
function abi.ffi.property_setter_starter_player_dev_camera_occlusion_mode_set_dev_camera_occlusion_mode(self, value)
	getPointer(self).DevCameraOcclusionMode = value;
end
function abi.ffi.property_getter_starter_player_dev_computer_camera_movement_mode(self)
	local result_0 = getPointer(self).DevComputerCameraMovementMode;
	return result_0;
end
function abi.ffi.property_setter_starter_player_dev_computer_camera_movement_mode_set_dev_computer_camera_movement_mode(self, value)
	getPointer(self).DevComputerCameraMovementMode = value;
end
function abi.ffi.property_getter_starter_player_dev_computer_movement_mode(self)
	local result_0 = getPointer(self).DevComputerMovementMode;
	return result_0;
end
function abi.ffi.property_setter_starter_player_dev_computer_movement_mode_set_dev_computer_movement_mode(self, value)
	getPointer(self).DevComputerMovementMode = value;
end
function abi.ffi.property_getter_starter_player_dev_touch_camera_movement_mode(self)
	local result_0 = getPointer(self).DevTouchCameraMovementMode;
	return result_0;
end
function abi.ffi.property_setter_starter_player_dev_touch_camera_movement_mode_set_dev_touch_camera_movement_mode(self, value)
	getPointer(self).DevTouchCameraMovementMode = value;
end
function abi.ffi.property_getter_starter_player_dev_touch_movement_mode(self)
	local result_0 = getPointer(self).DevTouchMovementMode;
	return result_0;
end
function abi.ffi.property_setter_starter_player_dev_touch_movement_mode_set_dev_touch_movement_mode(self, value)
	getPointer(self).DevTouchMovementMode = value;
end
function abi.ffi.property_getter_starter_player_enable_mouse_lock_option(self)
	local result_0 = getPointer(self).EnableMouseLockOption;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_starter_player_enable_mouse_lock_option_set_enable_mouse_lock_option(self, value)
	getPointer(self).EnableMouseLockOption = value == 1;
end
function abi.ffi.property_getter_starter_player_health_display_distance(self)
	local result_0 = getPointer(self).HealthDisplayDistance;
	return result_0;
end
function abi.ffi.property_setter_starter_player_health_display_distance_set_health_display_distance(self, value)
	getPointer(self).HealthDisplayDistance = value;
end
function abi.ffi.property_getter_starter_player_humanoid_state_machine_mode(self)
	local result_0 = getPointer(self).HumanoidStateMachineMode;
	return result_0;
end
function abi.ffi.property_setter_starter_player_humanoid_state_machine_mode_set_humanoid_state_machine_mode(self, value)
	getPointer(self).HumanoidStateMachineMode = value;
end
function abi.ffi.property_getter_starter_player_load_character_appearance(self)
	local result_0 = getPointer(self).LoadCharacterAppearance;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_starter_player_load_character_appearance_set_load_character_appearance(self, value)
	getPointer(self).LoadCharacterAppearance = value == 1;
end
function abi.ffi.property_getter_starter_player_name_display_distance(self)
	local result_0 = getPointer(self).NameDisplayDistance;
	return result_0;
end
function abi.ffi.property_setter_starter_player_name_display_distance_set_name_display_distance(self, value)
	getPointer(self).NameDisplayDistance = value;
end
function abi.ffi.property_getter_starter_player_user_emotes_enabled(self)
	local result_0 = getPointer(self).UserEmotesEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_starter_player_user_emotes_enabled_set_user_emotes_enabled(self, value)
	getPointer(self).UserEmotesEnabled = value == 1;
end
function abi.ffi.property_getter_stats_contacts_count(self)
	local result_0 = getPointer(self).ContactsCount;
	return result_0;
end
function abi.ffi.property_getter_stats_data_receive_kbps(self)
	local result_0 = getPointer(self).DataReceiveKbps;
	return result_0;
end
function abi.ffi.property_getter_stats_data_send_kbps(self)
	local result_0 = getPointer(self).DataSendKbps;
	return result_0;
end
function abi.ffi.property_getter_stats_heartbeat_time_ms(self)
	local result_0 = getPointer(self).HeartbeatTimeMs;
	return result_0;
end
function abi.ffi.property_getter_stats_instance_count(self)
	local result_0 = getPointer(self).InstanceCount;
	return result_0;
end
function abi.ffi.property_getter_stats_moving_primitives_count(self)
	local result_0 = getPointer(self).MovingPrimitivesCount;
	return result_0;
end
function abi.ffi.property_getter_stats_physics_receive_kbps(self)
	local result_0 = getPointer(self).PhysicsReceiveKbps;
	return result_0;
end
function abi.ffi.property_getter_stats_physics_send_kbps(self)
	local result_0 = getPointer(self).PhysicsSendKbps;
	return result_0;
end
function abi.ffi.property_getter_stats_physics_step_time_ms(self)
	local result_0 = getPointer(self).PhysicsStepTimeMs;
	return result_0;
end
function abi.ffi.property_getter_stats_primitives_count(self)
	local result_0 = getPointer(self).PrimitivesCount;
	return result_0;
end
function abi.ffi.method_stats_get_memory_usage_mb_for_tag(self, tag)
	local result_0 = getPointer(self):GetMemoryUsageMbForTag(tag);
	return result_0;
end
function abi.ffi.method_stats_get_total_memory_usage_mb(self)
	local result_0 = getPointer(self):GetTotalMemoryUsageMb();
	return result_0;
end
function abi.ffi.property_getter_team_auto_assignable(self)
	local result_0 = getPointer(self).AutoAssignable;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_team_auto_assignable_set_auto_assignable(self, value)
	getPointer(self).AutoAssignable = value == 1;
end
function abi.ffi.property_getter_team_team_color(self)
	local result_0 = getPointer(self).TeamColor;
	return createPointer(result_0);
end
function abi.ffi.property_setter_team_team_color_set_team_color(self, value)
	getPointer(self).TeamColor = getPointer(value);
end
function abi.ffi.event_team_player_added_on_player_added(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PlayerAdded:Connect(function(player)
		local player_optional, player_optional_0 = 0, 0;
		if (player ~= nil) then
			player_optional, player_optional_0 = 1, createPointer(player);
		end
		invokeFunction(callback_data, callback_vtable, player_optional, player_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_team_player_removed_on_player_removed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PlayerRemoved:Connect(function(player)
		local player_optional, player_optional_0 = 0, 0;
		if (player ~= nil) then
			player_optional, player_optional_0 = 1, createPointer(player);
		end
		invokeFunction(callback_data, callback_vtable, player_optional, player_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_teleport_async_result_private_server_id(self)
	local result_0 = getPointer(self).PrivateServerId;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_teleport_async_result_reserved_server_access_code(self)
	local result_0 = getPointer(self).ReservedServerAccessCode;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_teleport_options_reserved_server_access_code(self)
	local result_0 = getPointer(self).ReservedServerAccessCode;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_teleport_options_reserved_server_access_code_set_reserved_server_access_code(self, value_addr, value_len)
	getPointer(self).ReservedServerAccessCode = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_teleport_options_server_instance_id(self)
	local result_0 = getPointer(self).ServerInstanceId;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_teleport_options_server_instance_id_set_server_instance_id(self, value_addr, value_len)
	getPointer(self).ServerInstanceId = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_teleport_options_should_reserve_server(self)
	local result_0 = getPointer(self).ShouldReserveServer;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_teleport_options_should_reserve_server_set_should_reserve_server(self, value)
	getPointer(self).ShouldReserveServer = value == 1;
end
function abi.ffi.method_teleport_service_get_arriving_teleport_gui(self)
	local result_0 = getPointer(self):GetArrivingTeleportGui();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_teleport_service_set_teleport_gui(self, gui)
	getPointer(self):SetTeleportGui(getPointer(gui))
end
function abi.ffi.event_teleport_service_teleport_init_failed_on_teleport_init_failed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TeleportInitFailed:Connect(function(player, teleport_result, error_message, place_id, teleport_options)
		local player_optional, player_optional_0 = 0, 0;
		if (player ~= nil) then
			player_optional, player_optional_0 = 1, createPointer(player);
		end
		local error_message_len = #error_message;
		local error_message_content = allocString(error_message_len);
		for i = 1, error_message_len do
			storeU8(memory, error_message_content + (i - 1), string.byte(error_message:sub(i, i)));
		end
		local teleport_options_optional, teleport_options_optional_0 = 0, 0;
		if (teleport_options ~= nil) then
			teleport_options_optional, teleport_options_optional_0 = 1, createPointer(teleport_options);
		end
		invokeFunction(callback_data, callback_vtable, player_optional, player_optional_0, teleport_result, error_message_content, error_message_len, place_id, teleport_options_optional, teleport_options_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_terrain_detail_face(self)
	local result_0 = getPointer(self).Face;
	return result_0;
end
function abi.ffi.property_setter_terrain_detail_face_set_face(self, value)
	getPointer(self).Face = value;
end
function abi.ffi.property_getter_terrain_detail_material_pattern(self)
	local result_0 = getPointer(self).MaterialPattern;
	return result_0;
end
function abi.ffi.property_setter_terrain_detail_material_pattern_set_material_pattern(self, value)
	getPointer(self).MaterialPattern = value;
end
function abi.ffi.property_getter_terrain_detail_studs_per_tile(self)
	local result_0 = getPointer(self).StudsPerTile;
	return result_0;
end
function abi.ffi.property_setter_terrain_detail_studs_per_tile_set_studs_per_tile(self, value)
	getPointer(self).StudsPerTile = value;
end
function abi.ffi.property_getter_terrain_region_size_in_cells(self)
	local result_0 = getPointer(self).SizeInCells;
	return createPointer(result_0);
end
function abi.ffi.method_text_channel_display_system_message(self, system_message_addr, system_message_len, metadata_addr, metadata_len)
	local result_0 = getPointer(self):DisplaySystemMessage(loadString(memory, system_message_addr, system_message_len), loadString(memory, metadata_addr, metadata_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_text_channel_send_async(self, message_addr, message_len, metadata_addr, metadata_len)
	local result_0 = getPointer(self):SendAsync(loadString(memory, message_addr, message_len), loadString(memory, metadata_addr, metadata_len));
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.event_text_channel_message_received_on_message_received(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MessageReceived:Connect(function(incoming_message)
		local incoming_message_optional, incoming_message_optional_0 = 0, 0;
		if (incoming_message ~= nil) then
			incoming_message_optional, incoming_message_optional_0 = 1, createPointer(incoming_message);
		end
		invokeFunction(callback_data, callback_vtable, incoming_message_optional, incoming_message_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_text_chat_command_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_chat_command_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_text_chat_command_primary_alias(self)
	local result_0 = getPointer(self).PrimaryAlias;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_text_chat_command_primary_alias_set_primary_alias(self, value_addr, value_len)
	getPointer(self).PrimaryAlias = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_text_chat_command_secondary_alias(self)
	local result_0 = getPointer(self).SecondaryAlias;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_text_chat_command_secondary_alias_set_secondary_alias(self, value_addr, value_len)
	getPointer(self).SecondaryAlias = loadString(memory, value_addr, value_len);
end
function abi.ffi.event_text_chat_command_triggered_on_triggered(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Triggered:Connect(function(origin_text_source, unfiltered_text)
		local origin_text_source_optional, origin_text_source_optional_0 = 0, 0;
		if (origin_text_source ~= nil) then
			origin_text_source_optional, origin_text_source_optional_0 = 1, createPointer(origin_text_source);
		end
		local unfiltered_text_len = #unfiltered_text;
		local unfiltered_text_content = allocString(unfiltered_text_len);
		for i = 1, unfiltered_text_len do
			storeU8(memory, unfiltered_text_content + (i - 1), string.byte(unfiltered_text:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, origin_text_source_optional, origin_text_source_optional_0, unfiltered_text_content, unfiltered_text_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_bubble_chat_configuration_adornee_name(self)
	local result_0 = getPointer(self).AdorneeName;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_bubble_chat_configuration_adornee_name_set_adornee_name(self, value_addr, value_len)
	getPointer(self).AdorneeName = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_bubble_chat_configuration_background_color_3(self)
	local result_0 = getPointer(self).BackgroundColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_bubble_chat_configuration_background_color_3_set_background_color_3(self, value)
	getPointer(self).BackgroundColor3 = getPointer(value);
end
function abi.ffi.property_getter_bubble_chat_configuration_bubble_duration(self)
	local result_0 = getPointer(self).BubbleDuration;
	return result_0;
end
function abi.ffi.property_setter_bubble_chat_configuration_bubble_duration_set_bubble_duration(self, value)
	getPointer(self).BubbleDuration = value;
end
function abi.ffi.property_getter_bubble_chat_configuration_bubbles_spacing(self)
	local result_0 = getPointer(self).BubblesSpacing;
	return result_0;
end
function abi.ffi.property_setter_bubble_chat_configuration_bubbles_spacing_set_bubbles_spacing(self, value)
	getPointer(self).BubblesSpacing = value;
end
function abi.ffi.property_getter_bubble_chat_configuration_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_bubble_chat_configuration_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_bubble_chat_configuration_font(self)
	local result_0 = getPointer(self).Font;
	return result_0;
end
function abi.ffi.property_setter_bubble_chat_configuration_font_set_font(self, value)
	getPointer(self).Font = value;
end
function abi.ffi.property_getter_bubble_chat_configuration_local_player_studs_offset(self)
	local result_0 = getPointer(self).LocalPlayerStudsOffset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_bubble_chat_configuration_local_player_studs_offset_set_local_player_studs_offset(self, value)
	getPointer(self).LocalPlayerStudsOffset = getPointer(value);
end
function abi.ffi.property_getter_bubble_chat_configuration_max_distance(self)
	local result_0 = getPointer(self).MaxDistance;
	return result_0;
end
function abi.ffi.property_setter_bubble_chat_configuration_max_distance_set_max_distance(self, value)
	getPointer(self).MaxDistance = value;
end
function abi.ffi.property_getter_bubble_chat_configuration_minimize_distance(self)
	local result_0 = getPointer(self).MinimizeDistance;
	return result_0;
end
function abi.ffi.property_setter_bubble_chat_configuration_minimize_distance_set_minimize_distance(self, value)
	getPointer(self).MinimizeDistance = value;
end
function abi.ffi.property_getter_bubble_chat_configuration_text_color_3(self)
	local result_0 = getPointer(self).TextColor3;
	return createPointer(result_0);
end
function abi.ffi.property_setter_bubble_chat_configuration_text_color_3_set_text_color_3(self, value)
	getPointer(self).TextColor3 = getPointer(value);
end
function abi.ffi.property_getter_bubble_chat_configuration_text_size(self)
	local result_0 = getPointer(self).TextSize;
	return result_0;
end
function abi.ffi.property_setter_bubble_chat_configuration_text_size_set_text_size(self, value)
	getPointer(self).TextSize = value;
end
function abi.ffi.property_getter_bubble_chat_configuration_vertical_studs_offset(self)
	local result_0 = getPointer(self).VerticalStudsOffset;
	return result_0;
end
function abi.ffi.property_setter_bubble_chat_configuration_vertical_studs_offset_set_vertical_studs_offset(self, value)
	getPointer(self).VerticalStudsOffset = value;
end
function abi.ffi.property_getter_chat_input_bar_configuration_absolute_position(self)
	local result_0 = getPointer(self).AbsolutePosition;
	return createPointer(result_0);
end
function abi.ffi.property_getter_chat_input_bar_configuration_absolute_size(self)
	local result_0 = getPointer(self).AbsoluteSize;
	return createPointer(result_0);
end
function abi.ffi.property_getter_chat_input_bar_configuration_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_chat_input_bar_configuration_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_chat_input_bar_configuration_target_text_channel(self)
	local result_0 = getPointer(self).TargetTextChannel;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_chat_input_bar_configuration_target_text_channel_set_target_text_channel(self, value)
	getPointer(self).TargetTextChannel = getPointer(value);
end
function abi.ffi.property_getter_chat_window_configuration_absolute_position(self)
	local result_0 = getPointer(self).AbsolutePosition;
	return createPointer(result_0);
end
function abi.ffi.property_getter_chat_window_configuration_absolute_size(self)
	local result_0 = getPointer(self).AbsoluteSize;
	return createPointer(result_0);
end
function abi.ffi.property_getter_chat_window_configuration_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_chat_window_configuration_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_chat_window_configuration_horizontal_alignment(self)
	local result_0 = getPointer(self).HorizontalAlignment;
	return result_0;
end
function abi.ffi.property_setter_chat_window_configuration_horizontal_alignment_set_horizontal_alignment(self, value)
	getPointer(self).HorizontalAlignment = value;
end
function abi.ffi.property_getter_chat_window_configuration_vertical_alignment(self)
	local result_0 = getPointer(self).VerticalAlignment;
	return result_0;
end
function abi.ffi.property_setter_chat_window_configuration_vertical_alignment_set_vertical_alignment(self, value)
	getPointer(self).VerticalAlignment = value;
end
function abi.ffi.property_getter_text_chat_message_message_id(self)
	local result_0 = getPointer(self).MessageId;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_text_chat_message_message_id_set_message_id(self, value_addr, value_len)
	getPointer(self).MessageId = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_text_chat_message_metadata(self)
	local result_0 = getPointer(self).Metadata;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_text_chat_message_metadata_set_metadata(self, value_addr, value_len)
	getPointer(self).Metadata = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_text_chat_message_prefix_text(self)
	local result_0 = getPointer(self).PrefixText;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_text_chat_message_prefix_text_set_prefix_text(self, value_addr, value_len)
	getPointer(self).PrefixText = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_text_chat_message_status(self)
	local result_0 = getPointer(self).Status;
	return result_0;
end
function abi.ffi.property_setter_text_chat_message_status_set_status(self, value)
	getPointer(self).Status = value;
end
function abi.ffi.property_getter_text_chat_message_text(self)
	local result_0 = getPointer(self).Text;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_text_chat_message_text_set_text(self, value_addr, value_len)
	getPointer(self).Text = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_text_chat_message_text_channel(self)
	local result_0 = getPointer(self).TextChannel;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_text_chat_message_text_channel_set_text_channel(self, value)
	getPointer(self).TextChannel = getPointer(value);
end
function abi.ffi.property_getter_text_chat_message_text_source(self)
	local result_0 = getPointer(self).TextSource;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_text_chat_message_text_source_set_text_source(self, value)
	getPointer(self).TextSource = getPointer(value);
end
function abi.ffi.property_getter_text_chat_message_timestamp(self)
	local result_0 = getPointer(self).Timestamp;
	return createPointer(result_0);
end
function abi.ffi.property_setter_text_chat_message_timestamp_set_timestamp(self, value)
	getPointer(self).Timestamp = getPointer(value);
end
function abi.ffi.property_getter_text_chat_message_properties_prefix_text(self)
	local result_0 = getPointer(self).PrefixText;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_text_chat_message_properties_prefix_text_set_prefix_text(self, value_addr, value_len)
	getPointer(self).PrefixText = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_text_chat_message_properties_text(self)
	local result_0 = getPointer(self).Text;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_text_chat_message_properties_text_set_text(self, value_addr, value_len)
	getPointer(self).Text = loadString(memory, value_addr, value_len);
end
function abi.ffi.property_getter_text_chat_service_chat_version(self)
	local result_0 = getPointer(self).ChatVersion;
	return result_0;
end
function abi.ffi.property_setter_text_chat_service_chat_version_set_chat_version(self, value)
	getPointer(self).ChatVersion = value;
end
function abi.ffi.property_getter_text_chat_service_create_default_commands(self)
	local result_0 = getPointer(self).CreateDefaultCommands;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_chat_service_create_default_commands_set_create_default_commands(self, value)
	getPointer(self).CreateDefaultCommands = value == 1;
end
function abi.ffi.property_getter_text_chat_service_create_default_text_channels(self)
	local result_0 = getPointer(self).CreateDefaultTextChannels;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_chat_service_create_default_text_channels_set_create_default_text_channels(self, value)
	getPointer(self).CreateDefaultTextChannels = value == 1;
end
function abi.ffi.event_text_chat_service_message_received_on_message_received(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).MessageReceived:Connect(function(text_chat_message)
		local text_chat_message_optional, text_chat_message_optional_0 = 0, 0;
		if (text_chat_message ~= nil) then
			text_chat_message_optional, text_chat_message_optional_0 = 1, createPointer(text_chat_message);
		end
		invokeFunction(callback_data, callback_vtable, text_chat_message_optional, text_chat_message_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_text_chat_service_sending_message_on_sending_message(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).SendingMessage:Connect(function(text_chat_message)
		local text_chat_message_optional, text_chat_message_optional_0 = 0, 0;
		if (text_chat_message ~= nil) then
			text_chat_message_optional, text_chat_message_optional_0 = 1, createPointer(text_chat_message);
		end
		invokeFunction(callback_data, callback_vtable, text_chat_message_optional, text_chat_message_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.method_text_filter_result_get_chat_for_user_async(self, to_user_id)
	local result_0 = getPointer(self):GetChatForUserAsync(to_user_id);
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_text_filter_result_get_non_chat_string_for_broadcast_async(self)
	local result_0 = getPointer(self):GetNonChatStringForBroadcastAsync();
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_text_filter_result_get_non_chat_string_for_user_async(self, to_user_id)
	local result_0 = getPointer(self):GetNonChatStringForUserAsync(to_user_id);
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_text_service_get_text_size(self, string_addr, string_len, font_size, font, frame_size)
	local result_0 = getPointer(self):GetTextSize(loadString(memory, string_addr, string_len), font_size, font, getPointer(frame_size));
	return createPointer(result_0);
end
function abi.ffi.method_text_service_filter_string_async(self, string_to_filter_addr, string_to_filter_len, from_user_id, text_context)
	local result_0 = getPointer(self):FilterStringAsync(loadString(memory, string_to_filter_addr, string_to_filter_len), from_user_id, text_context);
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_text_service_get_text_bounds_async(self, params)
	local result_0 = getPointer(self):GetTextBoundsAsync(getPointer(params));
	return createPointer(result_0);
end
function abi.ffi.property_getter_text_source_can_send(self)
	local result_0 = getPointer(self).CanSend;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_text_source_can_send_set_can_send(self, value)
	getPointer(self).CanSend = value == 1;
end
function abi.ffi.property_getter_text_source_user_id(self)
	local result_0 = getPointer(self).UserId;
	return result_0;
end
function abi.ffi.property_getter_tracker_lod_controller_audio_mode(self)
	local result_0 = getPointer(self).AudioMode;
	return result_0;
end
function abi.ffi.property_setter_tracker_lod_controller_audio_mode_set_audio_mode(self, value)
	getPointer(self).AudioMode = value;
end
function abi.ffi.property_getter_tracker_lod_controller_video_extrapolation_mode(self)
	local result_0 = getPointer(self).VideoExtrapolationMode;
	return result_0;
end
function abi.ffi.property_setter_tracker_lod_controller_video_extrapolation_mode_set_video_extrapolation_mode(self, value)
	getPointer(self).VideoExtrapolationMode = value;
end
function abi.ffi.property_getter_tracker_lod_controller_video_lod_mode(self)
	local result_0 = getPointer(self).VideoLodMode;
	return result_0;
end
function abi.ffi.property_setter_tracker_lod_controller_video_lod_mode_set_video_lod_mode(self, value)
	getPointer(self).VideoLodMode = value;
end
function abi.ffi.property_getter_tracker_lod_controller_video_mode(self)
	local result_0 = getPointer(self).VideoMode;
	return result_0;
end
function abi.ffi.property_setter_tracker_lod_controller_video_mode_set_video_mode(self, value)
	getPointer(self).VideoMode = value;
end
function abi.ffi.property_getter_trail_attachment_0(self)
	local result_0 = getPointer(self).Attachment0;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_trail_attachment_0_set_attachment_0(self, value)
	getPointer(self).Attachment0 = getPointer(value);
end
function abi.ffi.property_getter_trail_attachment_1(self)
	local result_0 = getPointer(self).Attachment1;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_trail_attachment_1_set_attachment_1(self, value)
	getPointer(self).Attachment1 = getPointer(value);
end
function abi.ffi.property_getter_trail_brightness(self)
	local result_0 = getPointer(self).Brightness;
	return result_0;
end
function abi.ffi.property_setter_trail_brightness_set_brightness(self, value)
	getPointer(self).Brightness = value;
end
function abi.ffi.property_getter_trail_color(self)
	local result_0 = getPointer(self).Color;
	return createPointer(result_0);
end
function abi.ffi.property_setter_trail_color_set_color(self, value)
	getPointer(self).Color = getPointer(value);
end
function abi.ffi.property_getter_trail_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_trail_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_trail_face_camera(self)
	local result_0 = getPointer(self).FaceCamera;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_trail_face_camera_set_face_camera(self, value)
	getPointer(self).FaceCamera = value == 1;
end
function abi.ffi.property_getter_trail_lifetime(self)
	local result_0 = getPointer(self).Lifetime;
	return result_0;
end
function abi.ffi.property_setter_trail_lifetime_set_lifetime(self, value)
	getPointer(self).Lifetime = value;
end
function abi.ffi.property_getter_trail_light_emission(self)
	local result_0 = getPointer(self).LightEmission;
	return result_0;
end
function abi.ffi.property_setter_trail_light_emission_set_light_emission(self, value)
	getPointer(self).LightEmission = value;
end
function abi.ffi.property_getter_trail_light_influence(self)
	local result_0 = getPointer(self).LightInfluence;
	return result_0;
end
function abi.ffi.property_setter_trail_light_influence_set_light_influence(self, value)
	getPointer(self).LightInfluence = value;
end
function abi.ffi.property_getter_trail_max_length(self)
	local result_0 = getPointer(self).MaxLength;
	return result_0;
end
function abi.ffi.property_setter_trail_max_length_set_max_length(self, value)
	getPointer(self).MaxLength = value;
end
function abi.ffi.property_getter_trail_min_length(self)
	local result_0 = getPointer(self).MinLength;
	return result_0;
end
function abi.ffi.property_setter_trail_min_length_set_min_length(self, value)
	getPointer(self).MinLength = value;
end
function abi.ffi.property_getter_trail_texture(self)
	local result_0 = getPointer(self).Texture;
	return createPointer(result_0);
end
function abi.ffi.property_setter_trail_texture_set_texture(self, value)
	getPointer(self).Texture = getPointer(value);
end
function abi.ffi.property_getter_trail_texture_length(self)
	local result_0 = getPointer(self).TextureLength;
	return result_0;
end
function abi.ffi.property_setter_trail_texture_length_set_texture_length(self, value)
	getPointer(self).TextureLength = value;
end
function abi.ffi.property_getter_trail_texture_mode(self)
	local result_0 = getPointer(self).TextureMode;
	return result_0;
end
function abi.ffi.property_setter_trail_texture_mode_set_texture_mode(self, value)
	getPointer(self).TextureMode = value;
end
function abi.ffi.property_getter_trail_transparency(self)
	local result_0 = getPointer(self).Transparency;
	return createPointer(result_0);
end
function abi.ffi.property_setter_trail_transparency_set_transparency(self, value)
	getPointer(self).Transparency = getPointer(value);
end
function abi.ffi.property_getter_trail_width_scale(self)
	local result_0 = getPointer(self).WidthScale;
	return createPointer(result_0);
end
function abi.ffi.property_setter_trail_width_scale_set_width_scale(self, value)
	getPointer(self).WidthScale = getPointer(value);
end
function abi.ffi.method_trail_clear(self)
	getPointer(self):Clear()
end
function abi.ffi.property_getter_translator_locale_id(self)
	local result_0 = getPointer(self).LocaleId;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_translator_translate(self, context, text_addr, text_len)
	local result_0 = getPointer(self):Translate(getPointer(context), loadString(memory, text_addr, text_len));
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_getter_tween_base_playback_state(self)
	local result_0 = getPointer(self).PlaybackState;
	return result_0;
end
function abi.ffi.method_tween_base_cancel(self)
	getPointer(self):Cancel()
end
function abi.ffi.method_tween_base_pause(self)
	getPointer(self):Pause()
end
function abi.ffi.method_tween_base_play(self)
	getPointer(self):Play()
end
function abi.ffi.event_tween_base_completed_on_completed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Completed:Connect(function(playback_state)
		invokeFunction(callback_data, callback_vtable, playback_state)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_tween_instance(self)
	local result_0 = getPointer(self).Instance;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_tween_tween_info(self)
	local result_0 = getPointer(self).TweenInfo;
	return createPointer(result_0);
end
function abi.ffi.method_tween_service_get_value(self, alpha, easing_style, easing_direction)
	local result_0 = getPointer(self):GetValue(alpha, easing_style, easing_direction);
	return result_0;
end
function abi.ffi.property_getter_ui_aspect_ratio_constraint_aspect_ratio(self)
	local result_0 = getPointer(self).AspectRatio;
	return result_0;
end
function abi.ffi.property_setter_ui_aspect_ratio_constraint_aspect_ratio_set_aspect_ratio(self, value)
	getPointer(self).AspectRatio = value;
end
function abi.ffi.property_getter_ui_aspect_ratio_constraint_aspect_type(self)
	local result_0 = getPointer(self).AspectType;
	return result_0;
end
function abi.ffi.property_setter_ui_aspect_ratio_constraint_aspect_type_set_aspect_type(self, value)
	getPointer(self).AspectType = value;
end
function abi.ffi.property_getter_ui_aspect_ratio_constraint_dominant_axis(self)
	local result_0 = getPointer(self).DominantAxis;
	return result_0;
end
function abi.ffi.property_setter_ui_aspect_ratio_constraint_dominant_axis_set_dominant_axis(self, value)
	getPointer(self).DominantAxis = value;
end
function abi.ffi.property_getter_ui_size_constraint_max_size(self)
	local result_0 = getPointer(self).MaxSize;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_size_constraint_max_size_set_max_size(self, value)
	getPointer(self).MaxSize = getPointer(value);
end
function abi.ffi.property_getter_ui_size_constraint_min_size(self)
	local result_0 = getPointer(self).MinSize;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_size_constraint_min_size_set_min_size(self, value)
	getPointer(self).MinSize = getPointer(value);
end
function abi.ffi.property_getter_ui_text_size_constraint_max_text_size(self)
	local result_0 = getPointer(self).MaxTextSize;
	return result_0;
end
function abi.ffi.property_setter_ui_text_size_constraint_max_text_size_set_max_text_size(self, value)
	getPointer(self).MaxTextSize = value;
end
function abi.ffi.property_getter_ui_text_size_constraint_min_text_size(self)
	local result_0 = getPointer(self).MinTextSize;
	return result_0;
end
function abi.ffi.property_setter_ui_text_size_constraint_min_text_size_set_min_text_size(self, value)
	getPointer(self).MinTextSize = value;
end
function abi.ffi.property_getter_ui_corner_corner_radius(self)
	local result_0 = getPointer(self).CornerRadius;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_corner_corner_radius_set_corner_radius(self, value)
	getPointer(self).CornerRadius = getPointer(value);
end
function abi.ffi.property_getter_ui_gradient_color(self)
	local result_0 = getPointer(self).Color;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_gradient_color_set_color(self, value)
	getPointer(self).Color = getPointer(value);
end
function abi.ffi.property_getter_ui_gradient_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_ui_gradient_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_ui_gradient_offset(self)
	local result_0 = getPointer(self).Offset;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_gradient_offset_set_offset(self, value)
	getPointer(self).Offset = getPointer(value);
end
function abi.ffi.property_getter_ui_gradient_rotation(self)
	local result_0 = getPointer(self).Rotation;
	return result_0;
end
function abi.ffi.property_setter_ui_gradient_rotation_set_rotation(self, value)
	getPointer(self).Rotation = value;
end
function abi.ffi.property_getter_ui_gradient_transparency(self)
	local result_0 = getPointer(self).Transparency;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_gradient_transparency_set_transparency(self, value)
	getPointer(self).Transparency = getPointer(value);
end
function abi.ffi.property_getter_ui_grid_style_layout_absolute_content_size(self)
	local result_0 = getPointer(self).AbsoluteContentSize;
	return createPointer(result_0);
end
function abi.ffi.property_getter_ui_grid_style_layout_fill_direction(self)
	local result_0 = getPointer(self).FillDirection;
	return result_0;
end
function abi.ffi.property_setter_ui_grid_style_layout_fill_direction_set_fill_direction(self, value)
	getPointer(self).FillDirection = value;
end
function abi.ffi.property_getter_ui_grid_style_layout_horizontal_alignment(self)
	local result_0 = getPointer(self).HorizontalAlignment;
	return result_0;
end
function abi.ffi.property_setter_ui_grid_style_layout_horizontal_alignment_set_horizontal_alignment(self, value)
	getPointer(self).HorizontalAlignment = value;
end
function abi.ffi.property_getter_ui_grid_style_layout_sort_order(self)
	local result_0 = getPointer(self).SortOrder;
	return result_0;
end
function abi.ffi.property_setter_ui_grid_style_layout_sort_order_set_sort_order(self, value)
	getPointer(self).SortOrder = value;
end
function abi.ffi.property_getter_ui_grid_style_layout_vertical_alignment(self)
	local result_0 = getPointer(self).VerticalAlignment;
	return result_0;
end
function abi.ffi.property_setter_ui_grid_style_layout_vertical_alignment_set_vertical_alignment(self, value)
	getPointer(self).VerticalAlignment = value;
end
function abi.ffi.property_getter_ui_grid_layout_absolute_cell_count(self)
	local result_0 = getPointer(self).AbsoluteCellCount;
	return createPointer(result_0);
end
function abi.ffi.property_getter_ui_grid_layout_absolute_cell_size(self)
	local result_0 = getPointer(self).AbsoluteCellSize;
	return createPointer(result_0);
end
function abi.ffi.property_getter_ui_grid_layout_cell_padding(self)
	local result_0 = getPointer(self).CellPadding;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_grid_layout_cell_padding_set_cell_padding(self, value)
	getPointer(self).CellPadding = getPointer(value);
end
function abi.ffi.property_getter_ui_grid_layout_cell_size(self)
	local result_0 = getPointer(self).CellSize;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_grid_layout_cell_size_set_cell_size(self, value)
	getPointer(self).CellSize = getPointer(value);
end
function abi.ffi.property_getter_ui_grid_layout_fill_direction_max_cells(self)
	local result_0 = getPointer(self).FillDirectionMaxCells;
	return result_0;
end
function abi.ffi.property_setter_ui_grid_layout_fill_direction_max_cells_set_fill_direction_max_cells(self, value)
	getPointer(self).FillDirectionMaxCells = value;
end
function abi.ffi.property_getter_ui_grid_layout_start_corner(self)
	local result_0 = getPointer(self).StartCorner;
	return result_0;
end
function abi.ffi.property_setter_ui_grid_layout_start_corner_set_start_corner(self, value)
	getPointer(self).StartCorner = value;
end
function abi.ffi.property_getter_ui_list_layout_padding(self)
	local result_0 = getPointer(self).Padding;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_list_layout_padding_set_padding(self, value)
	getPointer(self).Padding = getPointer(value);
end
function abi.ffi.property_getter_ui_page_layout_animated(self)
	local result_0 = getPointer(self).Animated;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_ui_page_layout_animated_set_animated(self, value)
	getPointer(self).Animated = value == 1;
end
function abi.ffi.property_getter_ui_page_layout_circular(self)
	local result_0 = getPointer(self).Circular;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_ui_page_layout_circular_set_circular(self, value)
	getPointer(self).Circular = value == 1;
end
function abi.ffi.property_getter_ui_page_layout_current_page(self)
	local result_0 = getPointer(self).CurrentPage;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_getter_ui_page_layout_easing_direction(self)
	local result_0 = getPointer(self).EasingDirection;
	return result_0;
end
function abi.ffi.property_setter_ui_page_layout_easing_direction_set_easing_direction(self, value)
	getPointer(self).EasingDirection = value;
end
function abi.ffi.property_getter_ui_page_layout_easing_style(self)
	local result_0 = getPointer(self).EasingStyle;
	return result_0;
end
function abi.ffi.property_setter_ui_page_layout_easing_style_set_easing_style(self, value)
	getPointer(self).EasingStyle = value;
end
function abi.ffi.property_getter_ui_page_layout_gamepad_input_enabled(self)
	local result_0 = getPointer(self).GamepadInputEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_ui_page_layout_gamepad_input_enabled_set_gamepad_input_enabled(self, value)
	getPointer(self).GamepadInputEnabled = value == 1;
end
function abi.ffi.property_getter_ui_page_layout_padding(self)
	local result_0 = getPointer(self).Padding;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_page_layout_padding_set_padding(self, value)
	getPointer(self).Padding = getPointer(value);
end
function abi.ffi.property_getter_ui_page_layout_scroll_wheel_input_enabled(self)
	local result_0 = getPointer(self).ScrollWheelInputEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_ui_page_layout_scroll_wheel_input_enabled_set_scroll_wheel_input_enabled(self, value)
	getPointer(self).ScrollWheelInputEnabled = value == 1;
end
function abi.ffi.property_getter_ui_page_layout_touch_input_enabled(self)
	local result_0 = getPointer(self).TouchInputEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_ui_page_layout_touch_input_enabled_set_touch_input_enabled(self, value)
	getPointer(self).TouchInputEnabled = value == 1;
end
function abi.ffi.property_getter_ui_page_layout_tween_time(self)
	local result_0 = getPointer(self).TweenTime;
	return result_0;
end
function abi.ffi.property_setter_ui_page_layout_tween_time_set_tween_time(self, value)
	getPointer(self).TweenTime = value;
end
function abi.ffi.method_ui_page_layout_jump_to(self, page)
	getPointer(self):JumpTo(getPointer(page))
end
function abi.ffi.method_ui_page_layout_jump_to_index(self, index)
	getPointer(self):JumpToIndex(index)
end
function abi.ffi.method_ui_page_layout_next(self)
	getPointer(self):Next()
end
function abi.ffi.method_ui_page_layout_previous(self)
	getPointer(self):Previous()
end
function abi.ffi.event_ui_page_layout_page_enter_on_page_enter(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PageEnter:Connect(function(page)
		local page_optional, page_optional_0 = 0, 0;
		if (page ~= nil) then
			page_optional, page_optional_0 = 1, createPointer(page);
		end
		invokeFunction(callback_data, callback_vtable, page_optional, page_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_ui_page_layout_page_leave_on_page_leave(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PageLeave:Connect(function(page)
		local page_optional, page_optional_0 = 0, 0;
		if (page ~= nil) then
			page_optional, page_optional_0 = 1, createPointer(page);
		end
		invokeFunction(callback_data, callback_vtable, page_optional, page_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_ui_page_layout_stopped_on_stopped(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Stopped:Connect(function(current_page)
		local current_page_optional, current_page_optional_0 = 0, 0;
		if (current_page ~= nil) then
			current_page_optional, current_page_optional_0 = 1, createPointer(current_page);
		end
		invokeFunction(callback_data, callback_vtable, current_page_optional, current_page_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_ui_table_layout_fill_empty_space_columns(self)
	local result_0 = getPointer(self).FillEmptySpaceColumns;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_ui_table_layout_fill_empty_space_columns_set_fill_empty_space_columns(self, value)
	getPointer(self).FillEmptySpaceColumns = value == 1;
end
function abi.ffi.property_getter_ui_table_layout_fill_empty_space_rows(self)
	local result_0 = getPointer(self).FillEmptySpaceRows;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_ui_table_layout_fill_empty_space_rows_set_fill_empty_space_rows(self, value)
	getPointer(self).FillEmptySpaceRows = value == 1;
end
function abi.ffi.property_getter_ui_table_layout_major_axis(self)
	local result_0 = getPointer(self).MajorAxis;
	return result_0;
end
function abi.ffi.property_setter_ui_table_layout_major_axis_set_major_axis(self, value)
	getPointer(self).MajorAxis = value;
end
function abi.ffi.property_getter_ui_table_layout_padding(self)
	local result_0 = getPointer(self).Padding;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_table_layout_padding_set_padding(self, value)
	getPointer(self).Padding = getPointer(value);
end
function abi.ffi.property_getter_ui_padding_padding_bottom(self)
	local result_0 = getPointer(self).PaddingBottom;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_padding_padding_bottom_set_padding_bottom(self, value)
	getPointer(self).PaddingBottom = getPointer(value);
end
function abi.ffi.property_getter_ui_padding_padding_left(self)
	local result_0 = getPointer(self).PaddingLeft;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_padding_padding_left_set_padding_left(self, value)
	getPointer(self).PaddingLeft = getPointer(value);
end
function abi.ffi.property_getter_ui_padding_padding_right(self)
	local result_0 = getPointer(self).PaddingRight;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_padding_padding_right_set_padding_right(self, value)
	getPointer(self).PaddingRight = getPointer(value);
end
function abi.ffi.property_getter_ui_padding_padding_top(self)
	local result_0 = getPointer(self).PaddingTop;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_padding_padding_top_set_padding_top(self, value)
	getPointer(self).PaddingTop = getPointer(value);
end
function abi.ffi.property_getter_ui_scale_scale(self)
	local result_0 = getPointer(self).Scale;
	return result_0;
end
function abi.ffi.property_setter_ui_scale_scale_set_scale(self, value)
	getPointer(self).Scale = value;
end
function abi.ffi.property_getter_ui_stroke_apply_stroke_mode(self)
	local result_0 = getPointer(self).ApplyStrokeMode;
	return result_0;
end
function abi.ffi.property_setter_ui_stroke_apply_stroke_mode_set_apply_stroke_mode(self, value)
	getPointer(self).ApplyStrokeMode = value;
end
function abi.ffi.property_getter_ui_stroke_color(self)
	local result_0 = getPointer(self).Color;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ui_stroke_color_set_color(self, value)
	getPointer(self).Color = getPointer(value);
end
function abi.ffi.property_getter_ui_stroke_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_ui_stroke_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_ui_stroke_line_join_mode(self)
	local result_0 = getPointer(self).LineJoinMode;
	return result_0;
end
function abi.ffi.property_setter_ui_stroke_line_join_mode_set_line_join_mode(self, value)
	getPointer(self).LineJoinMode = value;
end
function abi.ffi.property_getter_ui_stroke_thickness(self)
	local result_0 = getPointer(self).Thickness;
	return result_0;
end
function abi.ffi.property_setter_ui_stroke_thickness_set_thickness(self, value)
	getPointer(self).Thickness = value;
end
function abi.ffi.property_getter_ui_stroke_transparency(self)
	local result_0 = getPointer(self).Transparency;
	return result_0;
end
function abi.ffi.property_setter_ui_stroke_transparency_set_transparency(self, value)
	getPointer(self).Transparency = value;
end
function abi.ffi.property_getter_user_game_settings_computer_camera_movement_mode(self)
	local result_0 = getPointer(self).ComputerCameraMovementMode;
	return result_0;
end
function abi.ffi.property_setter_user_game_settings_computer_camera_movement_mode_set_computer_camera_movement_mode(self, value)
	getPointer(self).ComputerCameraMovementMode = value;
end
function abi.ffi.property_getter_user_game_settings_computer_movement_mode(self)
	local result_0 = getPointer(self).ComputerMovementMode;
	return result_0;
end
function abi.ffi.property_setter_user_game_settings_computer_movement_mode_set_computer_movement_mode(self, value)
	getPointer(self).ComputerMovementMode = value;
end
function abi.ffi.property_getter_user_game_settings_control_mode(self)
	local result_0 = getPointer(self).ControlMode;
	return result_0;
end
function abi.ffi.property_setter_user_game_settings_control_mode_set_control_mode(self, value)
	getPointer(self).ControlMode = value;
end
function abi.ffi.property_getter_user_game_settings_gamepad_camera_sensitivity(self)
	local result_0 = getPointer(self).GamepadCameraSensitivity;
	return result_0;
end
function abi.ffi.property_setter_user_game_settings_gamepad_camera_sensitivity_set_gamepad_camera_sensitivity(self, value)
	getPointer(self).GamepadCameraSensitivity = value;
end
function abi.ffi.property_getter_user_game_settings_mouse_sensitivity(self)
	local result_0 = getPointer(self).MouseSensitivity;
	return result_0;
end
function abi.ffi.property_setter_user_game_settings_mouse_sensitivity_set_mouse_sensitivity(self, value)
	getPointer(self).MouseSensitivity = value;
end
function abi.ffi.property_getter_user_game_settings_rcc_profiler_record_frame_rate(self)
	local result_0 = getPointer(self).RCCProfilerRecordFrameRate;
	return result_0;
end
function abi.ffi.property_setter_user_game_settings_rcc_profiler_record_frame_rate_set_rcc_profiler_record_frame_rate(self, value)
	getPointer(self).RCCProfilerRecordFrameRate = value;
end
function abi.ffi.property_getter_user_game_settings_rcc_profiler_record_time_frame(self)
	local result_0 = getPointer(self).RCCProfilerRecordTimeFrame;
	return result_0;
end
function abi.ffi.property_setter_user_game_settings_rcc_profiler_record_time_frame_set_rcc_profiler_record_time_frame(self, value)
	getPointer(self).RCCProfilerRecordTimeFrame = value;
end
function abi.ffi.property_getter_user_game_settings_rotation_type(self)
	local result_0 = getPointer(self).RotationType;
	return result_0;
end
function abi.ffi.property_setter_user_game_settings_rotation_type_set_rotation_type(self, value)
	getPointer(self).RotationType = value;
end
function abi.ffi.property_getter_user_game_settings_saved_quality_level(self)
	local result_0 = getPointer(self).SavedQualityLevel;
	return result_0;
end
function abi.ffi.property_setter_user_game_settings_saved_quality_level_set_saved_quality_level(self, value)
	getPointer(self).SavedQualityLevel = value;
end
function abi.ffi.property_getter_user_game_settings_touch_camera_movement_mode(self)
	local result_0 = getPointer(self).TouchCameraMovementMode;
	return result_0;
end
function abi.ffi.property_setter_user_game_settings_touch_camera_movement_mode_set_touch_camera_movement_mode(self, value)
	getPointer(self).TouchCameraMovementMode = value;
end
function abi.ffi.property_getter_user_game_settings_touch_movement_mode(self)
	local result_0 = getPointer(self).TouchMovementMode;
	return result_0;
end
function abi.ffi.property_setter_user_game_settings_touch_movement_mode_set_touch_movement_mode(self, value)
	getPointer(self).TouchMovementMode = value;
end
function abi.ffi.property_getter_user_game_settings_vr_smooth_rotation_enabled(self)
	local result_0 = getPointer(self).VRSmoothRotationEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_user_game_settings_vr_smooth_rotation_enabled_set_vr_smooth_rotation_enabled(self, value)
	getPointer(self).VRSmoothRotationEnabled = value == 1;
end
function abi.ffi.property_getter_user_game_settings_vignette_enabled(self)
	local result_0 = getPointer(self).VignetteEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_user_game_settings_vignette_enabled_set_vignette_enabled(self, value)
	getPointer(self).VignetteEnabled = value == 1;
end
function abi.ffi.method_user_game_settings_get_camera_y_invert_value(self)
	local result_0 = getPointer(self):GetCameraYInvertValue();
	return result_0;
end
function abi.ffi.method_user_game_settings_get_onboarding_completed(self, onboarding_id_addr, onboarding_id_len)
	local result_0 = getPointer(self):GetOnboardingCompleted(loadString(memory, onboarding_id_addr, onboarding_id_len));
	return result_0 and 1 or 0;
end
function abi.ffi.method_user_game_settings_in_full_screen(self)
	local result_0 = getPointer(self):InFullScreen();
	return result_0 and 1 or 0;
end
function abi.ffi.method_user_game_settings_in_studio_mode(self)
	local result_0 = getPointer(self):InStudioMode();
	return result_0 and 1 or 0;
end
function abi.ffi.method_user_game_settings_set_camera_y_invert_visible(self)
	getPointer(self):SetCameraYInvertVisible()
end
function abi.ffi.method_user_game_settings_set_gamepad_camera_sensitivity_visible(self)
	getPointer(self):SetGamepadCameraSensitivityVisible()
end
function abi.ffi.method_user_game_settings_set_onboarding_completed(self, onboarding_id_addr, onboarding_id_len)
	getPointer(self):SetOnboardingCompleted(loadString(memory, onboarding_id_addr, onboarding_id_len))
end
function abi.ffi.event_user_game_settings_fullscreen_changed_on_fullscreen_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).FullscreenChanged:Connect(function(is_fullscreen)
		invokeFunction(callback_data, callback_vtable, is_fullscreen and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_game_settings_studio_mode_changed_on_studio_mode_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).StudioModeChanged:Connect(function(is_studio_mode)
		invokeFunction(callback_data, callback_vtable, is_studio_mode and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_user_input_service_accelerometer_enabled(self)
	local result_0 = getPointer(self).AccelerometerEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_user_input_service_gamepad_enabled(self)
	local result_0 = getPointer(self).GamepadEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_user_input_service_gyroscope_enabled(self)
	local result_0 = getPointer(self).GyroscopeEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_user_input_service_keyboard_enabled(self)
	local result_0 = getPointer(self).KeyboardEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_user_input_service_mouse_behavior(self)
	local result_0 = getPointer(self).MouseBehavior;
	return result_0;
end
function abi.ffi.property_setter_user_input_service_mouse_behavior_set_mouse_behavior(self, value)
	getPointer(self).MouseBehavior = value;
end
function abi.ffi.property_getter_user_input_service_mouse_delta_sensitivity(self)
	local result_0 = getPointer(self).MouseDeltaSensitivity;
	return result_0;
end
function abi.ffi.property_setter_user_input_service_mouse_delta_sensitivity_set_mouse_delta_sensitivity(self, value)
	getPointer(self).MouseDeltaSensitivity = value;
end
function abi.ffi.property_getter_user_input_service_mouse_enabled(self)
	local result_0 = getPointer(self).MouseEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_user_input_service_mouse_icon_enabled(self)
	local result_0 = getPointer(self).MouseIconEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_user_input_service_mouse_icon_enabled_set_mouse_icon_enabled(self, value)
	getPointer(self).MouseIconEnabled = value == 1;
end
function abi.ffi.property_getter_user_input_service_on_screen_keyboard_position(self)
	local result_0 = getPointer(self).OnScreenKeyboardPosition;
	return createPointer(result_0);
end
function abi.ffi.property_getter_user_input_service_on_screen_keyboard_size(self)
	local result_0 = getPointer(self).OnScreenKeyboardSize;
	return createPointer(result_0);
end
function abi.ffi.property_getter_user_input_service_on_screen_keyboard_visible(self)
	local result_0 = getPointer(self).OnScreenKeyboardVisible;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_user_input_service_touch_enabled(self)
	local result_0 = getPointer(self).TouchEnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_user_input_service_vr_enabled(self)
	local result_0 = getPointer(self).VREnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.method_user_input_service_gamepad_supports(self, gamepad_num, gamepad_key_code)
	local result_0 = getPointer(self):GamepadSupports(gamepad_num, gamepad_key_code);
	return result_0 and 1 or 0;
end
function abi.ffi.method_user_input_service_get_device_acceleration(self)
	local result_0 = getPointer(self):GetDeviceAcceleration();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_user_input_service_get_device_gravity(self)
	local result_0 = getPointer(self):GetDeviceGravity();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_user_input_service_get_focused_text_box(self)
	local result_0 = getPointer(self):GetFocusedTextBox();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_user_input_service_get_gamepad_connected(self, gamepad_num)
	local result_0 = getPointer(self):GetGamepadConnected(gamepad_num);
	return result_0 and 1 or 0;
end
function abi.ffi.method_user_input_service_get_last_input_type(self)
	local result_0 = getPointer(self):GetLastInputType();
	return result_0;
end
function abi.ffi.method_user_input_service_get_mouse_delta(self)
	local result_0 = getPointer(self):GetMouseDelta();
	return createPointer(result_0);
end
function abi.ffi.method_user_input_service_get_mouse_location(self)
	local result_0 = getPointer(self):GetMouseLocation();
	return createPointer(result_0);
end
function abi.ffi.method_user_input_service_get_string_for_key_code(self, key_code)
	local result_0 = getPointer(self):GetStringForKeyCode(key_code);
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.method_user_input_service_get_user_c_frame(self, type)
	local result_0 = getPointer(self):GetUserCFrame(type);
	return createPointer(result_0);
end
function abi.ffi.method_user_input_service_is_gamepad_button_down(self, gamepad_num, gamepad_key_code)
	local result_0 = getPointer(self):IsGamepadButtonDown(gamepad_num, gamepad_key_code);
	return result_0 and 1 or 0;
end
function abi.ffi.method_user_input_service_is_key_down(self, key_code)
	local result_0 = getPointer(self):IsKeyDown(key_code);
	return result_0 and 1 or 0;
end
function abi.ffi.method_user_input_service_is_mouse_button_pressed(self, mouse_button)
	local result_0 = getPointer(self):IsMouseButtonPressed(mouse_button);
	return result_0 and 1 or 0;
end
function abi.ffi.method_user_input_service_is_navigation_gamepad(self, gamepad_enum)
	local result_0 = getPointer(self):IsNavigationGamepad(gamepad_enum);
	return result_0 and 1 or 0;
end
function abi.ffi.method_user_input_service_recenter_user_head_c_frame(self)
	getPointer(self):RecenterUserHeadCFrame()
end
function abi.ffi.method_user_input_service_set_navigation_gamepad(self, gamepad_enum, enabled)
	getPointer(self):SetNavigationGamepad(gamepad_enum, enabled == 1)
end
function abi.ffi.event_user_input_service_device_acceleration_changed_on_device_acceleration_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).DeviceAccelerationChanged:Connect(function(acceleration)
		local acceleration_optional, acceleration_optional_0 = 0, 0;
		if (acceleration ~= nil) then
			acceleration_optional, acceleration_optional_0 = 1, createPointer(acceleration);
		end
		invokeFunction(callback_data, callback_vtable, acceleration_optional, acceleration_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_device_gravity_changed_on_device_gravity_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).DeviceGravityChanged:Connect(function(gravity)
		local gravity_optional, gravity_optional_0 = 0, 0;
		if (gravity ~= nil) then
			gravity_optional, gravity_optional_0 = 1, createPointer(gravity);
		end
		invokeFunction(callback_data, callback_vtable, gravity_optional, gravity_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_device_rotation_changed_on_device_rotation_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).DeviceRotationChanged:Connect(function(rotation, cframe)
		local rotation_optional, rotation_optional_0 = 0, 0;
		if (rotation ~= nil) then
			rotation_optional, rotation_optional_0 = 1, createPointer(rotation);
		end
		invokeFunction(callback_data, callback_vtable, rotation_optional, rotation_optional_0, createPointer(cframe))
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_gamepad_connected_on_gamepad_connected(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).GamepadConnected:Connect(function(gamepad_num)
		invokeFunction(callback_data, callback_vtable, gamepad_num)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_gamepad_disconnected_on_gamepad_disconnected(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).GamepadDisconnected:Connect(function(gamepad_num)
		invokeFunction(callback_data, callback_vtable, gamepad_num)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_input_began_on_input_began(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).InputBegan:Connect(function(input, game_processed_event)
		local input_optional, input_optional_0 = 0, 0;
		if (input ~= nil) then
			input_optional, input_optional_0 = 1, createPointer(input);
		end
		invokeFunction(callback_data, callback_vtable, input_optional, input_optional_0, game_processed_event and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_input_changed_on_input_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).InputChanged:Connect(function(input, game_processed_event)
		local input_optional, input_optional_0 = 0, 0;
		if (input ~= nil) then
			input_optional, input_optional_0 = 1, createPointer(input);
		end
		invokeFunction(callback_data, callback_vtable, input_optional, input_optional_0, game_processed_event and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_input_ended_on_input_ended(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).InputEnded:Connect(function(input, game_processed_event)
		local input_optional, input_optional_0 = 0, 0;
		if (input ~= nil) then
			input_optional, input_optional_0 = 1, createPointer(input);
		end
		invokeFunction(callback_data, callback_vtable, input_optional, input_optional_0, game_processed_event and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_jump_request_on_jump_request(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).JumpRequest:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_last_input_type_changed_on_last_input_type_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).LastInputTypeChanged:Connect(function(last_input_type)
		invokeFunction(callback_data, callback_vtable, last_input_type)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_pointer_action_on_pointer_action(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).PointerAction:Connect(function(wheel, pan, pinch, game_processed_event)
		invokeFunction(callback_data, callback_vtable, wheel, createPointer(pan), pinch, game_processed_event and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_text_box_focus_released_on_text_box_focus_released(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TextBoxFocusReleased:Connect(function(textbox_released)
		local textbox_released_optional, textbox_released_optional_0 = 0, 0;
		if (textbox_released ~= nil) then
			textbox_released_optional, textbox_released_optional_0 = 1, createPointer(textbox_released);
		end
		invokeFunction(callback_data, callback_vtable, textbox_released_optional, textbox_released_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_text_box_focused_on_text_box_focused(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TextBoxFocused:Connect(function(textbox_focused)
		local textbox_focused_optional, textbox_focused_optional_0 = 0, 0;
		if (textbox_focused ~= nil) then
			textbox_focused_optional, textbox_focused_optional_0 = 1, createPointer(textbox_focused);
		end
		invokeFunction(callback_data, callback_vtable, textbox_focused_optional, textbox_focused_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_touch_ended_on_touch_ended(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TouchEnded:Connect(function(touch, game_processed_event)
		local touch_optional, touch_optional_0 = 0, 0;
		if (touch ~= nil) then
			touch_optional, touch_optional_0 = 1, createPointer(touch);
		end
		invokeFunction(callback_data, callback_vtable, touch_optional, touch_optional_0, game_processed_event and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_touch_moved_on_touch_moved(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TouchMoved:Connect(function(touch, game_processed_event)
		local touch_optional, touch_optional_0 = 0, 0;
		if (touch ~= nil) then
			touch_optional, touch_optional_0 = 1, createPointer(touch);
		end
		invokeFunction(callback_data, callback_vtable, touch_optional, touch_optional_0, game_processed_event and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_touch_started_on_touch_started(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TouchStarted:Connect(function(touch, game_processed_event)
		local touch_optional, touch_optional_0 = 0, 0;
		if (touch ~= nil) then
			touch_optional, touch_optional_0 = 1, createPointer(touch);
		end
		invokeFunction(callback_data, callback_vtable, touch_optional, touch_optional_0, game_processed_event and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_touch_swipe_on_touch_swipe(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TouchSwipe:Connect(function(swipe_direction, number_of_touches, game_processed_event)
		invokeFunction(callback_data, callback_vtable, swipe_direction, number_of_touches, game_processed_event and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_touch_tap_in_world_on_touch_tap_in_world(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TouchTapInWorld:Connect(function(position, processed_by_ui)
		invokeFunction(callback_data, callback_vtable, createPointer(position), processed_by_ui and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_user_c_frame_changed_on_user_c_frame_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).UserCFrameChanged:Connect(function(type, value)
		invokeFunction(callback_data, callback_vtable, type, createPointer(value))
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_window_focus_released_on_window_focus_released(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).WindowFocusReleased:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_user_input_service_window_focused_on_window_focused(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).WindowFocused:Connect(function()
		invokeFunction(callback_data, callback_vtable)
	end));
	return createPointer(result_0);
end
function abi.ffi.method_user_input_service_get_keys_pressed(self)
	local result_0 = getPointer(self):GetKeysPressed();
	local result_0_len = #result_0;
	local result_0_vec = allocVec(result_0_len * 4)
	for i = 1, result_0_len do
		local result_0_vec_value = result_0[i];
		storeU32(memory, result_0_vec + (i - 1) * 4 + 0, result_0_vec_value);
	end
	return result_0_vec, result_0_len;
end
function abi.ffi.property_getter_vr_service_gui_input_user_c_frame(self)
	local result_0 = getPointer(self).GuiInputUserCFrame;
	return result_0;
end
function abi.ffi.property_setter_vr_service_gui_input_user_c_frame_set_gui_input_user_c_frame(self, value)
	getPointer(self).GuiInputUserCFrame = value;
end
function abi.ffi.property_getter_vr_service_vr_enabled(self)
	local result_0 = getPointer(self).VREnabled;
	return result_0 and 1 or 0;
end
function abi.ffi.method_vr_service_get_touchpad_mode(self, pad)
	local result_0 = getPointer(self):GetTouchpadMode(pad);
	return result_0;
end
function abi.ffi.method_vr_service_get_user_c_frame(self, type)
	local result_0 = getPointer(self):GetUserCFrame(type);
	return createPointer(result_0);
end
function abi.ffi.method_vr_service_get_user_c_frame_enabled(self, type)
	local result_0 = getPointer(self):GetUserCFrameEnabled(type);
	return result_0 and 1 or 0;
end
function abi.ffi.method_vr_service_recenter_user_head_c_frame(self)
	getPointer(self):RecenterUserHeadCFrame()
end
function abi.ffi.method_vr_service_request_navigation(self, cframe, input_user_c_frame)
	getPointer(self):RequestNavigation(getPointer(cframe), input_user_c_frame)
end
function abi.ffi.method_vr_service_set_touchpad_mode(self, pad, mode)
	getPointer(self):SetTouchpadMode(pad, mode)
end
function abi.ffi.event_vr_service_navigation_requested_on_navigation_requested(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).NavigationRequested:Connect(function(cframe, input_user_c_frame)
		invokeFunction(callback_data, callback_vtable, createPointer(cframe), input_user_c_frame)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_vr_service_touchpad_mode_changed_on_touchpad_mode_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).TouchpadModeChanged:Connect(function(pad, mode)
		invokeFunction(callback_data, callback_vtable, pad, mode)
	end));
	return createPointer(result_0);
end
function abi.ffi.event_vr_service_user_c_frame_changed_on_user_c_frame_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).UserCFrameChanged:Connect(function(type, value)
		invokeFunction(callback_data, callback_vtable, type, createPointer(value))
	end));
	return createPointer(result_0);
end
function abi.ffi.event_vr_service_user_c_frame_enabled_on_user_c_frame_enabled(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).UserCFrameEnabled:Connect(function(type, enabled)
		invokeFunction(callback_data, callback_vtable, type, enabled and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_bool_value_value(self)
	local result_0 = getPointer(self).Value;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_bool_value_value_set_value(self, value)
	getPointer(self).Value = value == 1;
end
function abi.ffi.event_bool_value_changed_on_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Changed:Connect(function(value)
		invokeFunction(callback_data, callback_vtable, value and 1 or 0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_brick_color_value_value(self)
	local result_0 = getPointer(self).Value;
	return createPointer(result_0);
end
function abi.ffi.property_setter_brick_color_value_value_set_value(self, value)
	getPointer(self).Value = getPointer(value);
end
function abi.ffi.event_brick_color_value_changed_on_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Changed:Connect(function(value)
		invokeFunction(callback_data, callback_vtable, createPointer(value))
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_c_frame_value_value(self)
	local result_0 = getPointer(self).Value;
	return createPointer(result_0);
end
function abi.ffi.property_setter_c_frame_value_value_set_value(self, value)
	getPointer(self).Value = getPointer(value);
end
function abi.ffi.event_c_frame_value_changed_on_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Changed:Connect(function(value)
		invokeFunction(callback_data, callback_vtable, createPointer(value))
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_color_3_value_value(self)
	local result_0 = getPointer(self).Value;
	return createPointer(result_0);
end
function abi.ffi.property_setter_color_3_value_value_set_value(self, value)
	getPointer(self).Value = getPointer(value);
end
function abi.ffi.event_color_3_value_changed_on_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Changed:Connect(function(value)
		invokeFunction(callback_data, callback_vtable, createPointer(value))
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_int_value_value(self)
	local result_0 = getPointer(self).Value;
	return result_0;
end
function abi.ffi.property_setter_int_value_value_set_value(self, value)
	getPointer(self).Value = value;
end
function abi.ffi.event_int_value_changed_on_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Changed:Connect(function(value)
		invokeFunction(callback_data, callback_vtable, value)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_number_value_value(self)
	local result_0 = getPointer(self).Value;
	return result_0;
end
function abi.ffi.property_setter_number_value_value_set_value(self, value)
	getPointer(self).Value = value;
end
function abi.ffi.event_number_value_changed_on_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Changed:Connect(function(value)
		invokeFunction(callback_data, callback_vtable, value)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_object_value_value(self)
	local result_0 = getPointer(self).Value;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_object_value_value_set_value(self, value)
	getPointer(self).Value = getPointer(value);
end
function abi.ffi.event_object_value_changed_on_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Changed:Connect(function(value)
		local value_optional, value_optional_0 = 0, 0;
		if (value ~= nil) then
			value_optional, value_optional_0 = 1, createPointer(value);
		end
		invokeFunction(callback_data, callback_vtable, value_optional, value_optional_0)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_ray_value_value(self)
	local result_0 = getPointer(self).Value;
	return createPointer(result_0);
end
function abi.ffi.property_setter_ray_value_value_set_value(self, value)
	getPointer(self).Value = getPointer(value);
end
function abi.ffi.event_ray_value_changed_on_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Changed:Connect(function(value)
		invokeFunction(callback_data, callback_vtable, createPointer(value))
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_string_value_value(self)
	local result_0 = getPointer(self).Value;
	local result_0_len = #result_0;
	local result_0_content = allocString(result_0_len);
	for i = 1, result_0_len do
		storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
	end
	return result_0_content, result_0_len;
end
function abi.ffi.property_setter_string_value_value_set_value(self, value_addr, value_len)
	getPointer(self).Value = loadString(memory, value_addr, value_len);
end
function abi.ffi.event_string_value_changed_on_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Changed:Connect(function(value)
		local value_len = #value;
		local value_content = allocString(value_len);
		for i = 1, value_len do
			storeU8(memory, value_content + (i - 1), string.byte(value:sub(i, i)));
		end
		invokeFunction(callback_data, callback_vtable, value_content, value_len)
	end));
	return createPointer(result_0);
end
function abi.ffi.property_getter_vector_3_value_value(self)
	local result_0 = getPointer(self).Value;
	return createPointer(result_0);
end
function abi.ffi.property_setter_vector_3_value_value_set_value(self, value)
	getPointer(self).Value = getPointer(value);
end
function abi.ffi.event_vector_3_value_changed_on_changed(self, callback_data, callback_vtable)
	local result_0 = createConnection(callback_data, callback_vtable, getPointer(self).Changed:Connect(function(value)
		invokeFunction(callback_data, callback_vtable, createPointer(value))
	end));
	return createPointer(result_0);
end
function abi.ffi.method_vector_3_curve_x(self)
	local result_0 = getPointer(self):X();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_vector_3_curve_y(self)
	local result_0 = getPointer(self):Y();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_vector_3_curve_z(self)
	local result_0 = getPointer(self):Z();
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.method_voice_chat_internal_is_voice_enabled_for_user_id_async(self, user_id)
	local result_0 = getPointer(self):IsVoiceEnabledForUserIdAsync(user_id);
	return result_0 and 1 or 0;
end
function abi.ffi.method_voice_chat_service_is_voice_enabled_for_user_id_async(self, user_id)
	local result_0 = getPointer(self):IsVoiceEnabledForUserIdAsync(user_id);
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_voice_source_user_id(self)
	local result_0 = getPointer(self).UserId;
	return result_0;
end
function abi.ffi.property_getter_weld_constraint_active(self)
	local result_0 = getPointer(self).Active;
	return result_0 and 1 or 0;
end
function abi.ffi.property_getter_weld_constraint_enabled(self)
	local result_0 = getPointer(self).Enabled;
	return result_0 and 1 or 0;
end
function abi.ffi.property_setter_weld_constraint_enabled_set_enabled(self, value)
	getPointer(self).Enabled = value == 1;
end
function abi.ffi.property_getter_weld_constraint_part_0(self)
	local result_0 = getPointer(self).Part0;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_weld_constraint_part_0_set_part_0(self, value)
	getPointer(self).Part0 = getPointer(value);
end
function abi.ffi.property_getter_weld_constraint_part_1(self)
	local result_0 = getPointer(self).Part1;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, createPointer(result_0);
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.property_setter_weld_constraint_part_1_set_part_1(self, value)
	getPointer(self).Part1 = getPointer(value);
end
function abi.ffi.pointer_conversion_internal_string_to_lua_value(string_addr, string_len)
	local result_0 = loadString(memory, string_addr, string_len);
	return createPointer(result_0);
end
function abi.ffi.pointer_conversion_internal_float_to_lua_value(float)
	local result_0 = float;
	return createPointer(result_0);
end
function abi.ffi.primitive_conversion_internal_lua_value_to_string(value)
	local text = getPointer(value);
	local result_0 = if type(text) == "string" then text else nil;
	local result_0_optional, result_0_optional_0, result_0_optional_1 = 0, 0, 0;
	if (result_0 ~= nil) then
		local result_0_len = #result_0;
		local result_0_content = allocString(result_0_len);
		for i = 1, result_0_len do
			storeU8(memory, result_0_content + (i - 1), string.byte(result_0:sub(i, i)));
		end
		result_0_optional, result_0_optional_0, result_0_optional_1 = 1, result_0_content, result_0_len;
	end
	return result_0_optional, result_0_optional_0, result_0_optional_1;
end
function abi.ffi.primitive_conversion_internal_lua_value_to_float(value)
	local text = getPointer(value);
	local result_0 = if type(text) == "number" then text else nil;
	local result_0_optional, result_0_optional_0 = 0, 0;
	if (result_0 ~= nil) then
		result_0_optional, result_0_optional_0 = 1, result_0;
	end
	return result_0_optional, result_0_optional_0;
end
function abi.ffi.static_function_internal_new_instance_new(class_name_addr, class_name_len)
	local result_0 = Instance.new(loadString(memory, class_name_addr, class_name_len));
	return createPointer(result_0);
end
function abi.ffi.static_function_vector_3_new()
	local result_0 = Vector3.new();
	return createPointer(result_0);
end
function abi.ffi.static_function_vector_3_new_new_with_position(x, y, z)
	local result_0 = Vector3.new(x, y, z);
	return createPointer(result_0);
end
function abi.ffi.static_function_vector_3_new_new_with_vector_3(v3)
	local result_0 = Vector3.new(getPointer(v3));
	return createPointer(result_0);
end
function abi.ffi.static_function_vector_3_new_new_with_normal(normal)
	local result_0 = Vector3.new(normal);
	return createPointer(result_0);
end
function abi.ffi.static_function_vector_3_new_new_with_axis(axis)
	local result_0 = Vector3.new(axis);
	return createPointer(result_0);
end
function abi.ffi.static_property_vector_3_zero()
	local result_0 = Vector3.zero;
	return createPointer(result_0);
end
function abi.ffi.static_property_vector_3_one()
	local result_0 = Vector3.one;
	return createPointer(result_0);
end
function abi.ffi.static_property_vector_3_x_axis()
	local result_0 = Vector3.xAxis;
	return createPointer(result_0);
end
function abi.ffi.static_property_vector_3_y_axis()
	local result_0 = Vector3.yAxis;
	return createPointer(result_0);
end
function abi.ffi.static_property_vector_3_z_axis()
	local result_0 = Vector3.zAxis;
	return createPointer(result_0);
end
function abi.ffi.property_getter_vector_3_x(self)
	local result_0 = getPointer(self).X;
	return result_0;
end
function abi.ffi.property_getter_vector_3_y(self)
	local result_0 = getPointer(self).Y;
	return result_0;
end
function abi.ffi.property_getter_vector_3_z(self)
	local result_0 = getPointer(self).Z;
	return result_0;
end
function abi.ffi.property_getter_vector_3_magnitude(self)
	local result_0 = getPointer(self).Magnitude;
	return result_0;
end
function abi.ffi.property_getter_vector_3_unit(self)
	local result_0 = getPointer(self).Unit;
	return createPointer(result_0);
end
function abi.ffi.method_vector_3_lerp(self, goal, alpha)
	local result_0 = getPointer(self):Lerp(getPointer(goal), alpha);
	return createPointer(result_0);
end
function abi.ffi.method_vector_3_cross(self, other)
	local result_0 = getPointer(self):Cross(getPointer(other));
	return createPointer(result_0);
end
function abi.ffi.method_vector_3_angle(self, other, axis)
	local result_0 = getPointer(self):Angle(getPointer(other), getPointer(axis));
	return result_0;
end
function abi.ffi.method_vector_3_dot(self, other)
	local result_0 = getPointer(self):Dot(getPointer(other));
	return result_0;
end
function abi.ffi.method_vector_3_fuzzy_eq(self, other, epsilon)
	local result_0 = getPointer(self):FuzzyEq(getPointer(other), epsilon);
	return result_0 and 1 or 0;
end
function abi.ffi.method_vector_3_max(self, others_addr, others_len)
	local others_vec = table.create(others_len);
	for i = 1, others_len do
		local vecElement_0 = loadU32(memory, others_addr + (i - 1) * 4 + 0)
		others_vec[i] = getPointer(vecElement_0);
	end
	local result_0 = getPointer(self):Max(unpack(others_vec, 1, others_len));
	return createPointer(result_0);
end
function abi.ffi.method_vector_3_min(self, others_addr, others_len)
	local others_vec = table.create(others_len);
	for i = 1, others_len do
		local vecElement_0 = loadU32(memory, others_addr + (i - 1) * 4 + 0)
		others_vec[i] = getPointer(vecElement_0);
	end
	local result_0 = getPointer(self):Min(unpack(others_vec, 1, others_len));
	return createPointer(result_0);
end
function abi.ffi.bin_op_vector_3_add_vector_3_add(self, value)
	local result_0 = getPointer(self) + getPointer(value);
	return createPointer(result_0);
end
function abi.ffi.bin_op_vector_3_sub_vector_3_sub(self, value)
	local result_0 = getPointer(self) - getPointer(value);
	return createPointer(result_0);
end
function abi.ffi.bin_op_vector_3_div_vector_3_div(self, value)
	local result_0 = getPointer(self) / getPointer(value);
	return createPointer(result_0);
end
function abi.ffi.bin_op_vector_3_div_number_div(self, value)
	local result_0 = getPointer(self) / value;
	return createPointer(result_0);
end
function abi.ffi.bin_op_vector_3_mul_vector_3_mul(self, value)
	local result_0 = getPointer(self) * getPointer(value);
	return createPointer(result_0);
end
function abi.ffi.bin_op_vector_3_mul_number_mul(self, value)
	local result_0 = getPointer(self) * value;
	return createPointer(result_0);
end
function abi.ffi.static_function_c_frame_new()
	local result_0 = CFrame.new();
	return createPointer(result_0);
end
function abi.ffi.static_function_c_frame_new_new_with_position(x, y, z)
	local result_0 = CFrame.new(x, y, z);
	return createPointer(result_0);
end
function abi.ffi.property_getter_c_frame_position(self)
	local result_0 = getPointer(self).Position;
	return createPointer(result_0);
end
function abi.ffi.property_getter_c_frame_rotation(self)
	local result_0 = getPointer(self).Rotation;
	return createPointer(result_0);
end
function abi.ffi.property_getter_c_frame_look_vector(self)
	local result_0 = getPointer(self).LookVector;
	return createPointer(result_0);
end
function abi.ffi.property_getter_c_frame_right_vector(self)
	local result_0 = getPointer(self).RightVector;
	return createPointer(result_0);
end
function abi.ffi.property_getter_c_frame_up_vector(self)
	local result_0 = getPointer(self).UpVector;
	return createPointer(result_0);
end
function abi.ffi.property_getter_c_frame_x_vector(self)
	local result_0 = getPointer(self).XVector;
	return createPointer(result_0);
end
function abi.ffi.property_getter_c_frame_y_vector(self)
	local result_0 = getPointer(self).YVector;
	return createPointer(result_0);
end
function abi.ffi.property_getter_c_frame_z_vector(self)
	local result_0 = getPointer(self).ZVector;
	return createPointer(result_0);
end
function abi.ffi.property_getter_c_frame_x(self)
	local result_0 = getPointer(self).X;
	return result_0;
end
function abi.ffi.property_getter_c_frame_y(self)
	local result_0 = getPointer(self).Y;
	return result_0;
end
function abi.ffi.property_getter_c_frame_z(self)
	local result_0 = getPointer(self).Z;
	return result_0;
end
function abi.ffi.method_c_frame_inverse(self)
	local result_0 = getPointer(self):Inverse();
	return createPointer(result_0);
end
function abi.ffi.method_c_frame_lerp(self, goal, alpha)
	local result_0 = getPointer(self):Lerp(getPointer(goal), alpha);
	return createPointer(result_0);
end
function abi.ffi.method_c_frame_orthonormalize(self)
	local result_0 = getPointer(self):Orthonormalize();
	return createPointer(result_0);
end
function abi.ffi.method_c_frame_to_world_space(self, cf)
	local result_0 = getPointer(self):ToWorldSpace(getPointer(cf));
	return createPointer(result_0);
end
function abi.ffi.method_c_frame_to_object_space(self, cf)
	local result_0 = getPointer(self):ToObjectSpace(getPointer(cf));
	return createPointer(result_0);
end
function abi.ffi.method_c_frame_point_to_world_space(self, v3)
	local result_0 = getPointer(self):PointToWorldSpace(getPointer(v3));
	return createPointer(result_0);
end
function abi.ffi.method_c_frame_point_to_object_space(self, v3)
	local result_0 = getPointer(self):PointToObjectSpace(getPointer(v3));
	return createPointer(result_0);
end
function abi.ffi.method_c_frame_vector_to_world_space(self, v3)
	local result_0 = getPointer(self):VectorToWorldSpace(getPointer(v3));
	return createPointer(result_0);
end
function abi.ffi.method_c_frame_vector_to_object_space(self, v3)
	local result_0 = getPointer(self):VectorToObjectSpace(getPointer(v3));
	return createPointer(result_0);
end
function abi.ffi.method_c_frame_get_components(self)
	local result_0 = { getPointer(self):GetComponents() }
	local result_0_len = #result_0;
	local result_0_vec = allocVec(result_0_len * 8)
	for i = 1, result_0_len do
		local result_0_vec_value = result_0[i];
		storeFloat(memory, result_0_vec + (i - 1) * 8 + 0, result_0_vec_value);
	end
	return result_0_vec, result_0_len;
end
function abi.ffi.bin_op_c_frame_add_vector_3_add(self, value)
	local result_0 = getPointer(self) + getPointer(value);
	return createPointer(result_0);
end
function abi.ffi.bin_op_c_frame_sub_vector_3_sub(self, value)
	local result_0 = getPointer(self) - getPointer(value);
	return createPointer(result_0);
end
function abi.ffi.bin_op_c_frame_mul_cframe_mul(self, value)
	local result_0 = getPointer(self) * getPointer(value);
	return createPointer(result_0);
end
function abi.ffi.bin_op_c_frame_mul_vector_3_mul(self, value)
	local result_0 = getPointer(self) * getPointer(value);
	return createPointer(result_0);
end
function abi.ffi.static_function_color_3_from_rgb(r, g, b)
	local result_0 = Color3.fromRGB(r, g, b);
	return createPointer(result_0);
end
local function lua_value_number(value)
	return createPointer(value);
end
abi.ffi.lua_value_f32 = lua_value_number;
abi.ffi.lua_value_f64 = lua_value_number;
abi.ffi.lua_value_i8 = lua_value_number;
abi.ffi.lua_value_i16 = lua_value_number;
abi.ffi.lua_value_i32 = lua_value_number;
abi.ffi.lua_value_i64 = lua_value_number;
abi.ffi.lua_value_u8 = lua_value_number;
abi.ffi.lua_value_u16 = lua_value_number;
abi.ffi.lua_value_u32 = lua_value_number;
abi.ffi.lua_value_u64 = lua_value_number;
abi.ffi.lua_value_usize = lua_value_number;
abi.ffi.lua_value_isize = lua_value_number;
return abi;