local memory, rt, util;
local writeStringReturn, getPointer, createPointer, store_i32, readString;
local functions = { abi = {} };
function functions.withWasm(localMemory, localRuntime, localUtil)
	memory, rt, util = localMemory, localRuntime, localUtil;
	writeStringReturn, getPointer, createPointer, store_i32, readString = util.writeStringReturn, util.getPointer, util.createPointer, rt.store.i32, util.readString;
end
function functions.abi.dyn_instance_clear_all_children(instance)
	getPointer(instance):ClearAllChildren()
end
function functions.abi.dyn_instance_clone(addr, instance)
	local class = getPointer(instance):Clone();
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_instance_destroy(instance)
	getPointer(instance):Destroy()
end
function functions.abi.dyn_instance_find_first_ancestor(addr, instance, nameAddr, nameLength)
	local class = getPointer(instance):FindFirstAncestor(readString(nameAddr, nameLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_instance_find_first_ancestor_of_class(addr, instance, classNameAddr, classNameLength)
	local class = getPointer(instance):FindFirstAncestorOfClass(readString(classNameAddr, classNameLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_instance_find_first_ancestor_which_is_a(addr, instance, classNameAddr, classNameLength)
	local class = getPointer(instance):FindFirstAncestorWhichIsA(readString(classNameAddr, classNameLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_instance_find_first_child(addr, instance, nameAddr, nameLength, recursive)
	local class = getPointer(instance):FindFirstChild(readString(nameAddr, nameLength), recursive == 1);
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_instance_find_first_child_of_class(addr, instance, classNameAddr, classNameLength)
	local class = getPointer(instance):FindFirstChildOfClass(readString(classNameAddr, classNameLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_instance_find_first_child_which_is_a(addr, instance, classNameAddr, classNameLength, recursive)
	local class = getPointer(instance):FindFirstChildWhichIsA(readString(classNameAddr, classNameLength), recursive == 1);
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_instance_find_first_descendant(addr, instance, nameAddr, nameLength)
	local class = getPointer(instance):FindFirstDescendant(readString(nameAddr, nameLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_instance_get_actor(addr, instance)
	local class = getPointer(instance):GetActor();
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_instance_get_attribute(instance, attributeAddr, attributeLength)
	getPointer(instance):GetAttribute(readString(attributeAddr, attributeLength))
end
function functions.abi.dyn_instance_get_attribute_changed_signal(instance, attributeAddr, attributeLength)
	return createPointer(getPointer(instance):GetAttributeChangedSignal(readString(attributeAddr, attributeLength)));
end
function functions.abi.dyn_instance_get_attributes(instance)
	getPointer(instance):GetAttributes()
end
function functions.abi.dyn_instance_get_children(instance)
	return createPointer(getPointer(instance):GetChildren());
end
function functions.abi.dyn_instance_get_descendants(instance)
	getPointer(instance):GetDescendants()
end
function functions.abi.dyn_instance_get_full_name(addr, instance)
	local str = getPointer(instance):GetFullName();
	writeStringReturn(addr, str)
end
function functions.abi.dyn_instance_get_property_changed_signal(instance, propertyAddr, propertyLength)
	return createPointer(getPointer(instance):GetPropertyChangedSignal(readString(propertyAddr, propertyLength)));
end
function functions.abi.dyn_instance_is_a(instance, classNameAddr, classNameLength)
	return getPointer(instance):IsA(readString(classNameAddr, classNameLength)) and 1 or 0;
end
function functions.abi.dyn_instance_is_ancestor_of(instance, descendantSome, descendantValue)
	return getPointer(instance):IsAncestorOf(descendantSome == 1 and getPointer(descendantValue) or nil) and 1 or 0;
end
function functions.abi.dyn_instance_is_descendant_of(instance, ancestorSome, ancestorValue)
	return getPointer(instance):IsDescendantOf(ancestorSome == 1 and getPointer(ancestorValue) or nil) and 1 or 0;
end
function functions.abi.dyn_instance_remove(instance)
	getPointer(instance):Remove()
end
function functions.abi.dyn_instance_set_attribute(instance, attributeAddr, attributeLength)
	getPointer(instance):SetAttribute(readString(attributeAddr, attributeLength))
end
function functions.abi.dyn_instance_wait_for_child(addr, instance, childNameAddr, childNameLength)
	local class = getPointer(instance):WaitForChild(readString(childNameAddr, childNameLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_keyframe_sequence_add_keyframe(instance, keyframeSome, keyframeValue)
	getPointer(instance):AddKeyframe(keyframeSome == 1 and getPointer(keyframeValue) or nil)
end
function functions.abi.dyn_keyframe_sequence_get_keyframes(instance)
	return createPointer(getPointer(instance):GetKeyframes());
end
function functions.abi.dyn_keyframe_sequence_remove_keyframe(instance, keyframeSome, keyframeValue)
	getPointer(instance):RemoveKeyframe(keyframeSome == 1 and getPointer(keyframeValue) or nil)
end
function functions.abi.dyn_animation_controller_get_playing_animation_tracks(instance)
	getPointer(instance):GetPlayingAnimationTracks()
end
function functions.abi.dyn_animation_controller_load_animation(addr, instance, animationSome, animationValue)
	local class = getPointer(instance):LoadAnimation(animationSome == 1 and getPointer(animationValue) or nil);
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_animation_track_adjust_speed(instance)
	getPointer(instance):AdjustSpeed()
end
function functions.abi.dyn_animation_track_adjust_weight(instance)
	getPointer(instance):AdjustWeight()
end
function functions.abi.dyn_animation_track_get_marker_reached_signal(instance, nameAddr, nameLength)
	return createPointer(getPointer(instance):GetMarkerReachedSignal(readString(nameAddr, nameLength)));
end
function functions.abi.dyn_animation_track_get_time_of_keyframe(instance, keyframeNameAddr, keyframeNameLength)
	return getPointer(instance):GetTimeOfKeyframe(readString(keyframeNameAddr, keyframeNameLength));
end
function functions.abi.dyn_animation_track_play(instance)
	getPointer(instance):Play()
end
function functions.abi.dyn_animation_track_stop(instance)
	getPointer(instance):Stop()
end
function functions.abi.dyn_animator_apply_joint_velocities(instance)
	getPointer(instance):ApplyJointVelocities()
end
function functions.abi.dyn_animator_get_playing_animation_tracks(instance)
	getPointer(instance):GetPlayingAnimationTracks()
end
function functions.abi.dyn_animator_load_animation(addr, instance, animationSome, animationValue)
	local class = getPointer(instance):LoadAnimation(animationSome == 1 and getPointer(animationValue) or nil);
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_attachment_get_axis(instance)
	return createPointer(getPointer(instance):GetAxis());
end
function functions.abi.dyn_attachment_get_secondary_axis(instance)
	return createPointer(getPointer(instance):GetSecondaryAxis());
end
function functions.abi.dyn_attachment_set_axis(instance, axis)
	getPointer(instance):SetAxis(getPointer(axis))
end
function functions.abi.dyn_attachment_set_secondary_axis(instance, axis)
	getPointer(instance):SetSecondaryAxis(getPointer(axis))
end
function functions.abi.dyn_tool_activate(instance)
	getPointer(instance):Activate()
end
function functions.abi.dyn_tool_deactivate(instance)
	getPointer(instance):Deactivate()
end
function functions.abi.dyn_base_player_gui_get_gui_objects_at_position(instance)
	return createPointer(getPointer(instance):GetGuiObjectsAtPosition());
end
function functions.abi.dyn_player_gui_get_topbar_transparency(instance)
	return getPointer(instance):GetTopbarTransparency();
end
function functions.abi.dyn_player_gui_set_topbar_transparency(instance)
	getPointer(instance):SetTopbarTransparency()
end
function functions.abi.dyn_starter_gui_get_core_gui_enabled(instance)
	return getPointer(instance):GetCoreGuiEnabled() and 1 or 0;
end
function functions.abi.dyn_starter_gui_set_core(instance, parameterNameAddr, parameterNameLength)
	getPointer(instance):SetCore(readString(parameterNameAddr, parameterNameLength))
end
function functions.abi.dyn_starter_gui_set_core_gui_enabled(instance, enabled)
	getPointer(instance):SetCoreGuiEnabled(enabled == 1)
end
function functions.abi.dyn_starter_gui_get_core(instance, parameterNameAddr, parameterNameLength)
	getPointer(instance):GetCore(readString(parameterNameAddr, parameterNameLength))
end
function functions.abi.dyn_beam_set_texture_offset(instance)
	getPointer(instance):SetTextureOffset()
end
function functions.abi.dyn_bindable_event_fire(instance)
	getPointer(instance):Fire()
end
function functions.abi.dyn_bindable_function_invoke(instance)
	getPointer(instance):Invoke()
end
function functions.abi.dyn_camera_get_largest_cutoff_distance(instance, ignoreList)
	return getPointer(instance):GetLargestCutoffDistance(getPointer(ignoreList));
end
function functions.abi.dyn_camera_get_pan_speed(instance)
	return getPointer(instance):GetPanSpeed();
end
function functions.abi.dyn_camera_get_parts_obscuring_target(instance, ignoreList)
	return createPointer(getPointer(instance):GetPartsObscuringTarget(getPointer(ignoreList)));
end
function functions.abi.dyn_camera_get_render_c_frame(instance)
	return createPointer(getPointer(instance):GetRenderCFrame());
end
function functions.abi.dyn_camera_get_roll(instance)
	return getPointer(instance):GetRoll();
end
function functions.abi.dyn_camera_get_tilt_speed(instance)
	return getPointer(instance):GetTiltSpeed();
end
function functions.abi.dyn_camera_interpolate(instance, endPos, endFocus)
	getPointer(instance):Interpolate(getPointer(endPos), getPointer(endFocus))
end
function functions.abi.dyn_camera_pan_units(instance)
	getPointer(instance):PanUnits()
end
function functions.abi.dyn_camera_screen_point_to_ray(instance)
	return createPointer(getPointer(instance):ScreenPointToRay());
end
function functions.abi.dyn_camera_set_camera_pan_mode(instance)
	getPointer(instance):SetCameraPanMode()
end
function functions.abi.dyn_camera_set_roll(instance)
	getPointer(instance):SetRoll()
end
function functions.abi.dyn_camera_tilt_units(instance)
	return getPointer(instance):TiltUnits() and 1 or 0;
end
function functions.abi.dyn_camera_viewport_point_to_ray(instance)
	return createPointer(getPointer(instance):ViewportPointToRay());
end
function functions.abi.dyn_camera_world_to_screen_point(instance, worldPoint)
	getPointer(instance):WorldToScreenPoint(getPointer(worldPoint))
end
function functions.abi.dyn_camera_world_to_viewport_point(instance, worldPoint)
	getPointer(instance):WorldToViewportPoint(getPointer(worldPoint))
end
function functions.abi.dyn_controller_bind_button(instance, captionAddr, captionLength)
	getPointer(instance):BindButton(readString(captionAddr, captionLength))
end
function functions.abi.dyn_controller_get_button(instance)
	return getPointer(instance):GetButton() and 1 or 0;
end
function functions.abi.dyn_controller_unbind_button(instance)
	getPointer(instance):UnbindButton()
end
function functions.abi.dyn_data_store_increment_options_get_metadata(instance)
	getPointer(instance):GetMetadata()
end
function functions.abi.dyn_data_store_increment_options_set_metadata(instance)
	getPointer(instance):SetMetadata()
end
function functions.abi.dyn_data_store_key_info_get_metadata(instance)
	getPointer(instance):GetMetadata()
end
function functions.abi.dyn_data_store_key_info_get_user_ids(instance)
	getPointer(instance):GetUserIds()
end
function functions.abi.dyn_data_store_options_set_experimental_features(instance)
	getPointer(instance):SetExperimentalFeatures()
end
function functions.abi.dyn_data_store_set_options_get_metadata(instance)
	getPointer(instance):GetMetadata()
end
function functions.abi.dyn_data_store_set_options_set_metadata(instance)
	getPointer(instance):SetMetadata()
end
function functions.abi.dyn_dialog_get_current_players(instance)
	return createPointer(getPointer(instance):GetCurrentPlayers());
end
function functions.abi.dyn_dragger_axis_rotate(instance)
	getPointer(instance):AxisRotate()
end
function functions.abi.dyn_dragger_mouse_down(instance, mousePartSome, mousePartValue, pointOnMousePart, parts)
	getPointer(instance):MouseDown(mousePartSome == 1 and getPointer(mousePartValue) or nil, getPointer(pointOnMousePart), getPointer(parts))
end
function functions.abi.dyn_dragger_mouse_move(instance, mouseRay)
	getPointer(instance):MouseMove(getPointer(mouseRay))
end
function functions.abi.dyn_dragger_mouse_up(instance)
	getPointer(instance):MouseUp()
end
function functions.abi.dyn_euler_rotation_curve_get_angles_at_time(instance)
	getPointer(instance):GetAnglesAtTime()
end
function functions.abi.dyn_euler_rotation_curve_get_rotation_at_time(instance)
	return createPointer(getPointer(instance):GetRotationAtTime());
end
function functions.abi.dyn_euler_rotation_curve_x(addr, instance)
	local class = getPointer(instance):X();
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_euler_rotation_curve_y(addr, instance)
	local class = getPointer(instance):Y();
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_euler_rotation_curve_z(addr, instance)
	local class = getPointer(instance):Z();
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_float_curve_get_key_at_index(instance)
	return createPointer(getPointer(instance):GetKeyAtIndex());
end
function functions.abi.dyn_float_curve_get_key_indices_at_time(instance)
	getPointer(instance):GetKeyIndicesAtTime()
end
function functions.abi.dyn_float_curve_get_keys(instance)
	getPointer(instance):GetKeys()
end
function functions.abi.dyn_float_curve_get_value_at_time(instance)
	getPointer(instance):GetValueAtTime()
end
function functions.abi.dyn_float_curve_insert_key(instance, key)
	getPointer(instance):InsertKey(getPointer(key))
end
function functions.abi.dyn_float_curve_remove_key_at_index(instance)
	return getPointer(instance):RemoveKeyAtIndex();
end
function functions.abi.dyn_float_curve_set_keys(instance)
	return getPointer(instance):SetKeys();
end
function functions.abi.dyn_global_data_store_on_update(instance, keyAddr, keyLength, callback)
	return createPointer(getPointer(instance):OnUpdate(readString(keyAddr, keyLength), getPointer(callback)));
end
function functions.abi.dyn_global_data_store_get_async(instance, keyAddr, keyLength)
	getPointer(instance):GetAsync(readString(keyAddr, keyLength))
end
function functions.abi.dyn_global_data_store_increment_async(instance, keyAddr, keyLength, optionsSome, optionsValue)
	getPointer(instance):IncrementAsync(readString(keyAddr, keyLength), optionsSome == 1 and getPointer(optionsValue) or nil)
end
function functions.abi.dyn_global_data_store_remove_async(instance, keyAddr, keyLength)
	getPointer(instance):RemoveAsync(readString(keyAddr, keyLength))
end
function functions.abi.dyn_global_data_store_set_async(instance, keyAddr, keyLength, optionsSome, optionsValue)
	getPointer(instance):SetAsync(readString(keyAddr, keyLength), optionsSome == 1 and getPointer(optionsValue) or nil)
end
function functions.abi.dyn_global_data_store_update_async(instance, keyAddr, keyLength, transformFunction)
	getPointer(instance):UpdateAsync(readString(keyAddr, keyLength), getPointer(transformFunction))
end
function functions.abi.dyn_data_store_get_version_async(instance, keyAddr, keyLength, versionAddr, versionLength)
	getPointer(instance):GetVersionAsync(readString(keyAddr, keyLength), readString(versionAddr, versionLength))
end
function functions.abi.dyn_data_store_list_keys_async(addr, instance, prefixAddr, prefixLength)
	local class = getPointer(instance):ListKeysAsync(readString(prefixAddr, prefixLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_data_store_list_versions_async(addr, instance, keyAddr, keyLength)
	local class = getPointer(instance):ListVersionsAsync(readString(keyAddr, keyLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_data_store_remove_version_async(instance, keyAddr, keyLength, versionAddr, versionLength)
	getPointer(instance):RemoveVersionAsync(readString(keyAddr, keyLength), readString(versionAddr, versionLength))
end
function functions.abi.dyn_ordered_data_store_get_sorted_async(addr, instance, ascending)
	local class = getPointer(instance):GetSortedAsync(ascending == 1);
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_gui_object_tween_position(instance, endPosition, override, callback)
	return getPointer(instance):TweenPosition(getPointer(endPosition), override == 1, getPointer(callback)) and 1 or 0;
end
function functions.abi.dyn_gui_object_tween_size(instance, endSize, override, callback)
	return getPointer(instance):TweenSize(getPointer(endSize), override == 1, getPointer(callback)) and 1 or 0;
end
function functions.abi.dyn_gui_object_tween_size_and_position(instance, endSize, endPosition, override, callback)
	return getPointer(instance):TweenSizeAndPosition(getPointer(endSize), getPointer(endPosition), override == 1, getPointer(callback)) and 1 or 0;
end
function functions.abi.dyn_text_box_capture_focus(instance)
	getPointer(instance):CaptureFocus()
end
function functions.abi.dyn_text_box_is_focused(instance)
	return getPointer(instance):IsFocused() and 1 or 0;
end
function functions.abi.dyn_text_box_release_focus(instance, submitted)
	getPointer(instance):ReleaseFocus(submitted == 1)
end
function functions.abi.dyn_video_frame_pause(instance)
	getPointer(instance):Pause()
end
function functions.abi.dyn_video_frame_play(instance)
	getPointer(instance):Play()
end
function functions.abi.dyn_layer_collector_get_layout_node_tree(instance)
	getPointer(instance):GetLayoutNodeTree()
end
function functions.abi.dyn_humanoid_add_accessory(instance, accessorySome, accessoryValue)
	getPointer(instance):AddAccessory(accessorySome == 1 and getPointer(accessoryValue) or nil)
end
function functions.abi.dyn_humanoid_add_custom_status(instance, statusAddr, statusLength)
	return getPointer(instance):AddCustomStatus(readString(statusAddr, statusLength)) and 1 or 0;
end
function functions.abi.dyn_humanoid_add_status(instance)
	return getPointer(instance):AddStatus() and 1 or 0;
end
function functions.abi.dyn_humanoid_build_rig_from_attachments(instance)
	getPointer(instance):BuildRigFromAttachments()
end
function functions.abi.dyn_humanoid_change_state(instance)
	getPointer(instance):ChangeState()
end
function functions.abi.dyn_humanoid_equip_tool(instance, toolSome, toolValue)
	getPointer(instance):EquipTool(toolSome == 1 and getPointer(toolValue) or nil)
end
function functions.abi.dyn_humanoid_get_accessories(instance)
	getPointer(instance):GetAccessories()
end
function functions.abi.dyn_humanoid_get_applied_description(addr, instance)
	local class = getPointer(instance):GetAppliedDescription();
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_humanoid_get_body_part_r_15(instance, partSome, partValue)
	getPointer(instance):GetBodyPartR15(partSome == 1 and getPointer(partValue) or nil)
end
function functions.abi.dyn_humanoid_get_limb(instance, partSome, partValue)
	getPointer(instance):GetLimb(partSome == 1 and getPointer(partValue) or nil)
end
function functions.abi.dyn_humanoid_get_playing_animation_tracks(instance)
	getPointer(instance):GetPlayingAnimationTracks()
end
function functions.abi.dyn_humanoid_get_state(instance)
	getPointer(instance):GetState()
end
function functions.abi.dyn_humanoid_get_state_enabled(instance)
	return getPointer(instance):GetStateEnabled() and 1 or 0;
end
function functions.abi.dyn_humanoid_get_statuses(instance)
	getPointer(instance):GetStatuses()
end
function functions.abi.dyn_humanoid_has_custom_status(instance, statusAddr, statusLength)
	return getPointer(instance):HasCustomStatus(readString(statusAddr, statusLength)) and 1 or 0;
end
function functions.abi.dyn_humanoid_has_status(instance)
	return getPointer(instance):HasStatus() and 1 or 0;
end
function functions.abi.dyn_humanoid_load_animation(addr, instance, animationSome, animationValue)
	local class = getPointer(instance):LoadAnimation(animationSome == 1 and getPointer(animationValue) or nil);
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_humanoid_move(instance, moveDirection, relativeToCamera)
	getPointer(instance):Move(getPointer(moveDirection), relativeToCamera == 1)
end
function functions.abi.dyn_humanoid_move_to(instance, location, partSome, partValue)
	getPointer(instance):MoveTo(getPointer(location), partSome == 1 and getPointer(partValue) or nil)
end
function functions.abi.dyn_humanoid_remove_accessories(instance)
	getPointer(instance):RemoveAccessories()
end
function functions.abi.dyn_humanoid_remove_custom_status(instance, statusAddr, statusLength)
	return getPointer(instance):RemoveCustomStatus(readString(statusAddr, statusLength)) and 1 or 0;
end
function functions.abi.dyn_humanoid_remove_status(instance)
	return getPointer(instance):RemoveStatus() and 1 or 0;
end
function functions.abi.dyn_humanoid_replace_body_part_r_15(instance, partSome, partValue)
	return getPointer(instance):ReplaceBodyPartR15(partSome == 1 and getPointer(partValue) or nil) and 1 or 0;
end
function functions.abi.dyn_humanoid_set_state_enabled(instance, enabled)
	getPointer(instance):SetStateEnabled(enabled == 1)
end
function functions.abi.dyn_humanoid_take_damage(instance)
	getPointer(instance):TakeDamage()
end
function functions.abi.dyn_humanoid_unequip_tools(instance)
	getPointer(instance):UnequipTools()
end
function functions.abi.dyn_humanoid_apply_description(instance, humanoidDescriptionSome, humanoidDescriptionValue)
	getPointer(instance):ApplyDescription(humanoidDescriptionSome == 1 and getPointer(humanoidDescriptionValue) or nil)
end
function functions.abi.dyn_humanoid_apply_description_reset(instance, humanoidDescriptionSome, humanoidDescriptionValue)
	getPointer(instance):ApplyDescriptionReset(humanoidDescriptionSome == 1 and getPointer(humanoidDescriptionValue) or nil)
end
function functions.abi.dyn_humanoid_play_emote(instance, emoteNameAddr, emoteNameLength)
	return getPointer(instance):PlayEmote(readString(emoteNameAddr, emoteNameLength)) and 1 or 0;
end
function functions.abi.dyn_humanoid_description_add_emote(instance, nameAddr, nameLength)
	getPointer(instance):AddEmote(readString(nameAddr, nameLength))
end
function functions.abi.dyn_humanoid_description_get_accessories(instance, includeRigidAccessories)
	getPointer(instance):GetAccessories(includeRigidAccessories == 1)
end
function functions.abi.dyn_humanoid_description_get_emotes(instance)
	getPointer(instance):GetEmotes()
end
function functions.abi.dyn_humanoid_description_get_equipped_emotes(instance)
	getPointer(instance):GetEquippedEmotes()
end
function functions.abi.dyn_humanoid_description_remove_emote(instance, nameAddr, nameLength)
	getPointer(instance):RemoveEmote(readString(nameAddr, nameLength))
end
function functions.abi.dyn_humanoid_description_set_accessories(instance, includeRigidAccessories)
	getPointer(instance):SetAccessories(includeRigidAccessories == 1)
end
function functions.abi.dyn_humanoid_description_set_emotes(instance)
	getPointer(instance):SetEmotes()
end
function functions.abi.dyn_humanoid_description_set_equipped_emotes(instance)
	getPointer(instance):SetEquippedEmotes()
end
function functions.abi.dyn_input_object_is_modifier_key_down(instance)
	return getPointer(instance):IsModifierKeyDown() and 1 or 0;
end
function functions.abi.dyn_motor_set_desired_angle(instance)
	getPointer(instance):SetDesiredAngle()
end
function functions.abi.dyn_keyframe_add_marker(instance, markerSome, markerValue)
	getPointer(instance):AddMarker(markerSome == 1 and getPointer(markerValue) or nil)
end
function functions.abi.dyn_keyframe_add_pose(instance, poseSome, poseValue)
	getPointer(instance):AddPose(poseSome == 1 and getPointer(poseValue) or nil)
end
function functions.abi.dyn_keyframe_get_markers(instance)
	return createPointer(getPointer(instance):GetMarkers());
end
function functions.abi.dyn_keyframe_get_poses(instance)
	return createPointer(getPointer(instance):GetPoses());
end
function functions.abi.dyn_keyframe_remove_marker(instance, markerSome, markerValue)
	getPointer(instance):RemoveMarker(markerSome == 1 and getPointer(markerValue) or nil)
end
function functions.abi.dyn_keyframe_remove_pose(instance, poseSome, poseValue)
	getPointer(instance):RemovePose(poseSome == 1 and getPointer(poseValue) or nil)
end
function functions.abi.dyn_lighting_get_minutes_after_midnight(instance)
	return getPointer(instance):GetMinutesAfterMidnight();
end
function functions.abi.dyn_lighting_get_moon_direction(instance)
	return createPointer(getPointer(instance):GetMoonDirection());
end
function functions.abi.dyn_lighting_get_moon_phase(instance)
	return getPointer(instance):GetMoonPhase();
end
function functions.abi.dyn_lighting_get_sun_direction(instance)
	return createPointer(getPointer(instance):GetSunDirection());
end
function functions.abi.dyn_lighting_set_minutes_after_midnight(instance)
	getPointer(instance):SetMinutesAfterMidnight()
end
function functions.abi.dyn_localization_table_get_contents(addr, instance)
	local str = getPointer(instance):GetContents();
	writeStringReturn(addr, str)
end
function functions.abi.dyn_localization_table_get_entries(instance)
	getPointer(instance):GetEntries()
end
function functions.abi.dyn_localization_table_get_string(addr, instance, targetLocaleIdAddr, targetLocaleIdLength, keyAddr, keyLength)
	local str = getPointer(instance):GetString(readString(targetLocaleIdAddr, targetLocaleIdLength), readString(keyAddr, keyLength));
	writeStringReturn(addr, str)
end
function functions.abi.dyn_localization_table_get_translator(addr, instance, localeIdAddr, localeIdLength)
	local class = getPointer(instance):GetTranslator(readString(localeIdAddr, localeIdLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_localization_table_remove_entry(instance, keyAddr, keyLength, sourceAddr, sourceLength, contextAddr, contextLength)
	getPointer(instance):RemoveEntry(readString(keyAddr, keyLength), readString(sourceAddr, sourceLength), readString(contextAddr, contextLength))
end
function functions.abi.dyn_localization_table_remove_entry_value(instance, keyAddr, keyLength, sourceAddr, sourceLength, contextAddr, contextLength, localeIdAddr, localeIdLength)
	getPointer(instance):RemoveEntryValue(readString(keyAddr, keyLength), readString(sourceAddr, sourceLength), readString(contextAddr, contextLength), readString(localeIdAddr, localeIdLength))
end
function functions.abi.dyn_localization_table_remove_key(instance, keyAddr, keyLength)
	getPointer(instance):RemoveKey(readString(keyAddr, keyLength))
end
function functions.abi.dyn_localization_table_remove_target_locale(instance, localeIdAddr, localeIdLength)
	getPointer(instance):RemoveTargetLocale(readString(localeIdAddr, localeIdLength))
end
function functions.abi.dyn_localization_table_set_contents(instance, contentsAddr, contentsLength)
	getPointer(instance):SetContents(readString(contentsAddr, contentsLength))
end
function functions.abi.dyn_localization_table_set_entries(instance)
	getPointer(instance):SetEntries()
end
function functions.abi.dyn_localization_table_set_entry(instance, keyAddr, keyLength, targetLocaleIdAddr, targetLocaleIdLength, textAddr, textLength)
	getPointer(instance):SetEntry(readString(keyAddr, keyLength), readString(targetLocaleIdAddr, targetLocaleIdLength), readString(textAddr, textLength))
end
function functions.abi.dyn_localization_table_set_entry_context(instance, keyAddr, keyLength, sourceAddr, sourceLength, contextAddr, contextLength, newContextAddr, newContextLength)
	getPointer(instance):SetEntryContext(readString(keyAddr, keyLength), readString(sourceAddr, sourceLength), readString(contextAddr, contextLength), readString(newContextAddr, newContextLength))
end
function functions.abi.dyn_localization_table_set_entry_example(instance, keyAddr, keyLength, sourceAddr, sourceLength, contextAddr, contextLength, exampleAddr, exampleLength)
	getPointer(instance):SetEntryExample(readString(keyAddr, keyLength), readString(sourceAddr, sourceLength), readString(contextAddr, contextLength), readString(exampleAddr, exampleLength))
end
function functions.abi.dyn_localization_table_set_entry_key(instance, keyAddr, keyLength, sourceAddr, sourceLength, contextAddr, contextLength, newKeyAddr, newKeyLength)
	getPointer(instance):SetEntryKey(readString(keyAddr, keyLength), readString(sourceAddr, sourceLength), readString(contextAddr, contextLength), readString(newKeyAddr, newKeyLength))
end
function functions.abi.dyn_localization_table_set_entry_source(instance, keyAddr, keyLength, sourceAddr, sourceLength, contextAddr, contextLength, newSourceAddr, newSourceLength)
	getPointer(instance):SetEntrySource(readString(keyAddr, keyLength), readString(sourceAddr, sourceLength), readString(contextAddr, contextLength), readString(newSourceAddr, newSourceLength))
end
function functions.abi.dyn_localization_table_set_entry_value(instance, keyAddr, keyLength, sourceAddr, sourceLength, contextAddr, contextLength, localeIdAddr, localeIdLength, textAddr, textLength)
	getPointer(instance):SetEntryValue(readString(keyAddr, keyLength), readString(sourceAddr, sourceLength), readString(contextAddr, contextLength), readString(localeIdAddr, localeIdLength), readString(textAddr, textLength))
end
function functions.abi.dyn_marker_curve_get_marker_at_index(instance)
	getPointer(instance):GetMarkerAtIndex()
end
function functions.abi.dyn_marker_curve_get_markers(instance)
	getPointer(instance):GetMarkers()
end
function functions.abi.dyn_marker_curve_insert_marker_at_time(instance, markerAddr, markerLength)
	getPointer(instance):InsertMarkerAtTime(readString(markerAddr, markerLength))
end
function functions.abi.dyn_marker_curve_remove_marker_at_index(instance)
	return getPointer(instance):RemoveMarkerAtIndex();
end
function functions.abi.dyn_memory_store_queue_add_async(instance)
	getPointer(instance):AddAsync()
end
function functions.abi.dyn_memory_store_queue_read_async(instance, allOrNothing)
	getPointer(instance):ReadAsync(allOrNothing == 1)
end
function functions.abi.dyn_memory_store_queue_remove_async(instance, idAddr, idLength)
	getPointer(instance):RemoveAsync(readString(idAddr, idLength))
end
function functions.abi.dyn_memory_store_sorted_map_get_async(instance, keyAddr, keyLength)
	getPointer(instance):GetAsync(readString(keyAddr, keyLength))
end
function functions.abi.dyn_memory_store_sorted_map_get_range_async(instance, exclusiveLowerBoundAddr, exclusiveLowerBoundLength, exclusiveUpperBoundAddr, exclusiveUpperBoundLength)
	getPointer(instance):GetRangeAsync(readString(exclusiveLowerBoundAddr, exclusiveLowerBoundLength), readString(exclusiveUpperBoundAddr, exclusiveUpperBoundLength))
end
function functions.abi.dyn_memory_store_sorted_map_remove_async(instance, keyAddr, keyLength)
	getPointer(instance):RemoveAsync(readString(keyAddr, keyLength))
end
function functions.abi.dyn_memory_store_sorted_map_set_async(instance, keyAddr, keyLength)
	return getPointer(instance):SetAsync(readString(keyAddr, keyLength)) and 1 or 0;
end
function functions.abi.dyn_memory_store_sorted_map_update_async(instance, keyAddr, keyLength, transformFunction)
	getPointer(instance):UpdateAsync(readString(keyAddr, keyLength), getPointer(transformFunction))
end
function functions.abi.dyn_pv_instance_get_pivot(instance)
	return createPointer(getPointer(instance):GetPivot());
end
function functions.abi.dyn_pv_instance_pivot_to(instance, targetCFrame)
	getPointer(instance):PivotTo(getPointer(targetCFrame))
end
function functions.abi.dyn_base_part_apply_angular_impulse(instance, impulse)
	getPointer(instance):ApplyAngularImpulse(getPointer(impulse))
end
function functions.abi.dyn_base_part_apply_impulse(instance, impulse)
	getPointer(instance):ApplyImpulse(getPointer(impulse))
end
function functions.abi.dyn_base_part_apply_impulse_at_position(instance, impulse, position)
	getPointer(instance):ApplyImpulseAtPosition(getPointer(impulse), getPointer(position))
end
function functions.abi.dyn_base_part_break_joints(instance)
	getPointer(instance):BreakJoints()
end
function functions.abi.dyn_base_part_can_collide_with(instance, partSome, partValue)
	return getPointer(instance):CanCollideWith(partSome == 1 and getPointer(partValue) or nil) and 1 or 0;
end
function functions.abi.dyn_base_part_can_set_network_ownership(instance)
	getPointer(instance):CanSetNetworkOwnership()
end
function functions.abi.dyn_base_part_get_connected_parts(instance, recursive)
	return createPointer(getPointer(instance):GetConnectedParts(recursive == 1));
end
function functions.abi.dyn_base_part_get_joints(instance)
	return createPointer(getPointer(instance):GetJoints());
end
function functions.abi.dyn_base_part_get_mass(instance)
	return getPointer(instance):GetMass();
end
function functions.abi.dyn_base_part_get_network_owner(addr, instance)
	local class = getPointer(instance):GetNetworkOwner();
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_base_part_get_network_ownership_auto(instance)
	return getPointer(instance):GetNetworkOwnershipAuto() and 1 or 0;
end
function functions.abi.dyn_base_part_get_render_c_frame(instance)
	return createPointer(getPointer(instance):GetRenderCFrame());
end
function functions.abi.dyn_base_part_get_root_part(addr, instance)
	local class = getPointer(instance):GetRootPart();
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_base_part_get_touching_parts(instance)
	return createPointer(getPointer(instance):GetTouchingParts());
end
function functions.abi.dyn_base_part_get_velocity_at_position(instance, position)
	return createPointer(getPointer(instance):GetVelocityAtPosition(getPointer(position)));
end
function functions.abi.dyn_base_part_is_grounded(instance)
	return getPointer(instance):IsGrounded() and 1 or 0;
end
function functions.abi.dyn_base_part_make_joints(instance)
	getPointer(instance):MakeJoints()
end
function functions.abi.dyn_base_part_resize(instance)
	return getPointer(instance):Resize() and 1 or 0;
end
function functions.abi.dyn_base_part_set_network_owner(instance, playerInstanceSome, playerInstanceValue)
	getPointer(instance):SetNetworkOwner(playerInstanceSome == 1 and getPointer(playerInstanceValue) or nil)
end
function functions.abi.dyn_base_part_set_network_ownership_auto(instance)
	getPointer(instance):SetNetworkOwnershipAuto()
end
function functions.abi.dyn_base_part_subtract_async(addr, instance, parts)
	local class = getPointer(instance):SubtractAsync(getPointer(parts));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_base_part_union_async(addr, instance, parts)
	local class = getPointer(instance):UnionAsync(getPointer(parts));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_seat_sit(instance, humanoidSome, humanoidValue)
	getPointer(instance):Sit(humanoidSome == 1 and getPointer(humanoidValue) or nil)
end
function functions.abi.dyn_terrain_autowedge_cell(instance)
	return getPointer(instance):AutowedgeCell() and 1 or 0;
end
function functions.abi.dyn_terrain_autowedge_cells(instance, region)
	getPointer(instance):AutowedgeCells(getPointer(region))
end
function functions.abi.dyn_terrain_cell_center_to_world(instance)
	return createPointer(getPointer(instance):CellCenterToWorld());
end
function functions.abi.dyn_terrain_cell_corner_to_world(instance)
	return createPointer(getPointer(instance):CellCornerToWorld());
end
function functions.abi.dyn_terrain_clear(instance)
	getPointer(instance):Clear()
end
function functions.abi.dyn_terrain_copy_region(addr, instance, region)
	local class = getPointer(instance):CopyRegion(getPointer(region));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_terrain_count_cells(instance)
	return getPointer(instance):CountCells();
end
function functions.abi.dyn_terrain_fill_ball(instance, center)
	getPointer(instance):FillBall(getPointer(center))
end
function functions.abi.dyn_terrain_fill_block(instance, cframe, size)
	getPointer(instance):FillBlock(getPointer(cframe), getPointer(size))
end
function functions.abi.dyn_terrain_fill_cylinder(instance, cframe)
	getPointer(instance):FillCylinder(getPointer(cframe))
end
function functions.abi.dyn_terrain_fill_region(instance, region)
	getPointer(instance):FillRegion(getPointer(region))
end
function functions.abi.dyn_terrain_fill_wedge(instance, cframe, size)
	getPointer(instance):FillWedge(getPointer(cframe), getPointer(size))
end
function functions.abi.dyn_terrain_get_cell(instance)
	getPointer(instance):GetCell()
end
function functions.abi.dyn_terrain_get_material_color(instance)
	return createPointer(getPointer(instance):GetMaterialColor());
end
function functions.abi.dyn_terrain_get_water_cell(instance)
	getPointer(instance):GetWaterCell()
end
function functions.abi.dyn_terrain_paste_region(instance, regionSome, regionValue, corner, pasteEmptyCells)
	getPointer(instance):PasteRegion(regionSome == 1 and getPointer(regionValue) or nil, getPointer(corner), pasteEmptyCells == 1)
end
function functions.abi.dyn_terrain_read_voxels(instance, region)
	getPointer(instance):ReadVoxels(getPointer(region))
end
function functions.abi.dyn_terrain_replace_material(instance, region)
	getPointer(instance):ReplaceMaterial(getPointer(region))
end
function functions.abi.dyn_terrain_set_cell(instance)
	getPointer(instance):SetCell()
end
function functions.abi.dyn_terrain_set_cells(instance, region)
	getPointer(instance):SetCells(getPointer(region))
end
function functions.abi.dyn_terrain_set_material_color(instance, value)
	getPointer(instance):SetMaterialColor(getPointer(value))
end
function functions.abi.dyn_terrain_set_water_cell(instance)
	getPointer(instance):SetWaterCell()
end
function functions.abi.dyn_terrain_world_to_cell(instance, position)
	return createPointer(getPointer(instance):WorldToCell(getPointer(position)));
end
function functions.abi.dyn_terrain_world_to_cell_prefer_empty(instance, position)
	return createPointer(getPointer(instance):WorldToCellPreferEmpty(getPointer(position)));
end
function functions.abi.dyn_terrain_world_to_cell_prefer_solid(instance, position)
	return createPointer(getPointer(instance):WorldToCellPreferSolid(getPointer(position)));
end
function functions.abi.dyn_terrain_write_voxels(instance, region)
	getPointer(instance):WriteVoxels(getPointer(region))
end
function functions.abi.dyn_mesh_part_apply_mesh(instance, meshPartSome, meshPartValue)
	getPointer(instance):ApplyMesh(meshPartSome == 1 and getPointer(meshPartValue) or nil)
end
function functions.abi.dyn_vehicle_seat_sit(instance, humanoidSome, humanoidValue)
	getPointer(instance):Sit(humanoidSome == 1 and getPointer(humanoidValue) or nil)
end
function functions.abi.dyn_model_break_joints(instance)
	getPointer(instance):BreakJoints()
end
function functions.abi.dyn_model_get_bounding_box(instance)
	getPointer(instance):GetBoundingBox()
end
function functions.abi.dyn_model_get_extents_size(instance)
	return createPointer(getPointer(instance):GetExtentsSize());
end
function functions.abi.dyn_model_get_model_c_frame(instance)
	return createPointer(getPointer(instance):GetModelCFrame());
end
function functions.abi.dyn_model_get_model_size(instance)
	return createPointer(getPointer(instance):GetModelSize());
end
function functions.abi.dyn_model_get_primary_part_c_frame(instance)
	return createPointer(getPointer(instance):GetPrimaryPartCFrame());
end
function functions.abi.dyn_model_make_joints(instance)
	getPointer(instance):MakeJoints()
end
function functions.abi.dyn_model_move_to(instance, position)
	getPointer(instance):MoveTo(getPointer(position))
end
function functions.abi.dyn_model_reset_orientation_to_identity(instance)
	getPointer(instance):ResetOrientationToIdentity()
end
function functions.abi.dyn_model_set_identity_orientation(instance)
	getPointer(instance):SetIdentityOrientation()
end
function functions.abi.dyn_model_set_primary_part_c_frame(instance, cframe)
	getPointer(instance):SetPrimaryPartCFrame(getPointer(cframe))
end
function functions.abi.dyn_model_translate_by(instance, delta)
	getPointer(instance):TranslateBy(getPointer(delta))
end
function functions.abi.dyn_world_root_are_parts_touching_others(instance, partList)
	return getPointer(instance):ArePartsTouchingOthers(getPointer(partList)) and 1 or 0;
end
function functions.abi.dyn_world_root_bulk_move_to(instance, partList)
	getPointer(instance):BulkMoveTo(getPointer(partList))
end
function functions.abi.dyn_world_root_find_part_on_ray(instance, ray, ignoreDescendantsInstanceSome, ignoreDescendantsInstanceValue, terrainCellsAreCubes, ignoreWater)
	getPointer(instance):FindPartOnRay(getPointer(ray), ignoreDescendantsInstanceSome == 1 and getPointer(ignoreDescendantsInstanceValue) or nil, terrainCellsAreCubes == 1, ignoreWater == 1)
end
function functions.abi.dyn_world_root_find_part_on_ray_with_ignore_list(instance, ray, ignoreDescendantsTable, terrainCellsAreCubes, ignoreWater)
	getPointer(instance):FindPartOnRayWithIgnoreList(getPointer(ray), getPointer(ignoreDescendantsTable), terrainCellsAreCubes == 1, ignoreWater == 1)
end
function functions.abi.dyn_world_root_find_part_on_ray_with_whitelist(instance, ray, whitelistDescendantsTable, ignoreWater)
	getPointer(instance):FindPartOnRayWithWhitelist(getPointer(ray), getPointer(whitelistDescendantsTable), ignoreWater == 1)
end
function functions.abi.dyn_world_root_find_parts_in_region_3(instance, region, ignoreDescendantsInstanceSome, ignoreDescendantsInstanceValue)
	return createPointer(getPointer(instance):FindPartsInRegion3(getPointer(region), ignoreDescendantsInstanceSome == 1 and getPointer(ignoreDescendantsInstanceValue) or nil));
end
function functions.abi.dyn_world_root_find_parts_in_region_3_with_ignore_list(instance, region, ignoreDescendantsTable)
	return createPointer(getPointer(instance):FindPartsInRegion3WithIgnoreList(getPointer(region), getPointer(ignoreDescendantsTable)));
end
function functions.abi.dyn_world_root_find_parts_in_region_3_with_white_list(instance, region, whitelistDescendantsTable)
	return createPointer(getPointer(instance):FindPartsInRegion3WithWhiteList(getPointer(region), getPointer(whitelistDescendantsTable)));
end
function functions.abi.dyn_world_root_get_part_bounds_in_box(instance, cframe, size, overlapParams)
	return createPointer(getPointer(instance):GetPartBoundsInBox(getPointer(cframe), getPointer(size), getPointer(overlapParams)));
end
function functions.abi.dyn_world_root_get_part_bounds_in_radius(instance, position, overlapParams)
	return createPointer(getPointer(instance):GetPartBoundsInRadius(getPointer(position), getPointer(overlapParams)));
end
function functions.abi.dyn_world_root_get_parts_in_part(instance, partSome, partValue, overlapParams)
	return createPointer(getPointer(instance):GetPartsInPart(partSome == 1 and getPointer(partValue) or nil, getPointer(overlapParams)));
end
function functions.abi.dyn_world_root_is_region_3_empty(instance, region, ignoreDescendentsInstanceSome, ignoreDescendentsInstanceValue)
	return getPointer(instance):IsRegion3Empty(getPointer(region), ignoreDescendentsInstanceSome == 1 and getPointer(ignoreDescendentsInstanceValue) or nil) and 1 or 0;
end
function functions.abi.dyn_world_root_is_region_3_empty_with_ignore_list(instance, region, ignoreDescendentsTable)
	return getPointer(instance):IsRegion3EmptyWithIgnoreList(getPointer(region), getPointer(ignoreDescendentsTable)) and 1 or 0;
end
function functions.abi.dyn_world_root_raycast(instance, origin, direction, raycastParams)
	return createPointer(getPointer(instance):Raycast(getPointer(origin), getPointer(direction), getPointer(raycastParams)));
end
function functions.abi.dyn_workspace_get_num_awake_parts(instance)
	return getPointer(instance):GetNumAwakeParts();
end
function functions.abi.dyn_workspace_get_physics_throttling(instance)
	return getPointer(instance):GetPhysicsThrottling();
end
function functions.abi.dyn_workspace_get_real_physics_fps(instance)
	return getPointer(instance):GetRealPhysicsFPS();
end
function functions.abi.dyn_workspace_get_server_time_now(instance)
	return getPointer(instance):GetServerTimeNow();
end
function functions.abi.dyn_workspace_join_to_outsiders(instance, objects)
	getPointer(instance):JoinToOutsiders(getPointer(objects))
end
function functions.abi.dyn_workspace_pgs_is_enabled(instance)
	return getPointer(instance):PGSIsEnabled() and 1 or 0;
end
function functions.abi.dyn_workspace_unjoin_from_outsiders(instance, objects)
	getPointer(instance):UnjoinFromOutsiders(getPointer(objects))
end
function functions.abi.dyn_pages_get_current_page(instance)
	getPointer(instance):GetCurrentPage()
end
function functions.abi.dyn_pages_advance_to_next_page_async(instance)
	getPointer(instance):AdvanceToNextPageAsync()
end
function functions.abi.dyn_particle_emitter_clear(instance)
	getPointer(instance):Clear()
end
function functions.abi.dyn_particle_emitter_emit(instance)
	getPointer(instance):Emit()
end
function functions.abi.dyn_path_get_point_coordinates(instance)
	getPointer(instance):GetPointCoordinates()
end
function functions.abi.dyn_path_get_waypoints(instance)
	getPointer(instance):GetWaypoints()
end
function functions.abi.dyn_path_check_occlusion_async(instance)
	return getPointer(instance):CheckOcclusionAsync();
end
function functions.abi.dyn_path_compute_async(instance, start, finish)
	getPointer(instance):ComputeAsync(getPointer(start), getPointer(finish))
end
function functions.abi.dyn_player_clear_character_appearance(instance)
	getPointer(instance):ClearCharacterAppearance()
end
function functions.abi.dyn_player_distance_from_character(instance, point)
	return getPointer(instance):DistanceFromCharacter(getPointer(point));
end
function functions.abi.dyn_player_get_join_data(instance)
	getPointer(instance):GetJoinData()
end
function functions.abi.dyn_player_get_mouse(addr, instance)
	local class = getPointer(instance):GetMouse();
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_player_get_network_ping(instance)
	return getPointer(instance):GetNetworkPing();
end
function functions.abi.dyn_player_has_appearance_loaded(instance)
	return getPointer(instance):HasAppearanceLoaded() and 1 or 0;
end
function functions.abi.dyn_player_is_user_available_for_experiment(instance)
	return getPointer(instance):IsUserAvailableForExperiment() and 1 or 0;
end
function functions.abi.dyn_player_kick(instance, messageAddr, messageLength)
	getPointer(instance):Kick(readString(messageAddr, messageLength))
end
function functions.abi.dyn_player_load_boolean(instance, keyAddr, keyLength)
	return getPointer(instance):LoadBoolean(readString(keyAddr, keyLength)) and 1 or 0;
end
function functions.abi.dyn_player_load_character_appearance(instance, assetInstanceSome, assetInstanceValue)
	getPointer(instance):LoadCharacterAppearance(assetInstanceSome == 1 and getPointer(assetInstanceValue) or nil)
end
function functions.abi.dyn_player_load_instance(addr, instance, keyAddr, keyLength)
	local class = getPointer(instance):LoadInstance(readString(keyAddr, keyLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_player_load_number(instance, keyAddr, keyLength)
	return getPointer(instance):LoadNumber(readString(keyAddr, keyLength));
end
function functions.abi.dyn_player_load_string(addr, instance, keyAddr, keyLength)
	local str = getPointer(instance):LoadString(readString(keyAddr, keyLength));
	writeStringReturn(addr, str)
end
function functions.abi.dyn_player_move(instance, walkDirection, relativeToCamera)
	getPointer(instance):Move(getPointer(walkDirection), relativeToCamera == 1)
end
function functions.abi.dyn_player_save_boolean(instance, keyAddr, keyLength, value)
	getPointer(instance):SaveBoolean(readString(keyAddr, keyLength), value == 1)
end
function functions.abi.dyn_player_save_instance(instance, keyAddr, keyLength, valueSome, valueValue)
	getPointer(instance):SaveInstance(readString(keyAddr, keyLength), valueSome == 1 and getPointer(valueValue) or nil)
end
function functions.abi.dyn_player_save_number(instance, keyAddr, keyLength)
	getPointer(instance):SaveNumber(readString(keyAddr, keyLength))
end
function functions.abi.dyn_player_save_string(instance, keyAddr, keyLength, valueAddr, valueLength)
	getPointer(instance):SaveString(readString(keyAddr, keyLength), readString(valueAddr, valueLength))
end
function functions.abi.dyn_player_get_friends_online(instance)
	getPointer(instance):GetFriendsOnline()
end
function functions.abi.dyn_player_get_rank_in_group(instance)
	return getPointer(instance):GetRankInGroup();
end
function functions.abi.dyn_player_get_role_in_group(addr, instance)
	local str = getPointer(instance):GetRoleInGroup();
	writeStringReturn(addr, str)
end
function functions.abi.dyn_player_is_best_friends_with(instance)
	return getPointer(instance):IsBestFriendsWith() and 1 or 0;
end
function functions.abi.dyn_player_is_friends_with(instance)
	return getPointer(instance):IsFriendsWith() and 1 or 0;
end
function functions.abi.dyn_player_is_in_group(instance)
	return getPointer(instance):IsInGroup() and 1 or 0;
end
function functions.abi.dyn_player_load_character(instance)
	getPointer(instance):LoadCharacter()
end
function functions.abi.dyn_player_load_character_with_humanoid_description(instance, humanoidDescriptionSome, humanoidDescriptionValue)
	getPointer(instance):LoadCharacterWithHumanoidDescription(humanoidDescriptionSome == 1 and getPointer(humanoidDescriptionValue) or nil)
end
function functions.abi.dyn_player_request_stream_around_async(instance, position)
	getPointer(instance):RequestStreamAroundAsync(getPointer(position))
end
function functions.abi.dyn_player_wait_for_data_ready(instance)
	return getPointer(instance):WaitForDataReady() and 1 or 0;
end
function functions.abi.dyn_player_scripts_clear_computer_camera_movement_modes(instance)
	getPointer(instance):ClearComputerCameraMovementModes()
end
function functions.abi.dyn_player_scripts_clear_computer_movement_modes(instance)
	getPointer(instance):ClearComputerMovementModes()
end
function functions.abi.dyn_player_scripts_clear_touch_camera_movement_modes(instance)
	getPointer(instance):ClearTouchCameraMovementModes()
end
function functions.abi.dyn_player_scripts_clear_touch_movement_modes(instance)
	getPointer(instance):ClearTouchMovementModes()
end
function functions.abi.dyn_player_scripts_register_computer_camera_movement_mode(instance)
	getPointer(instance):RegisterComputerCameraMovementMode()
end
function functions.abi.dyn_player_scripts_register_computer_movement_mode(instance)
	getPointer(instance):RegisterComputerMovementMode()
end
function functions.abi.dyn_player_scripts_register_touch_camera_movement_mode(instance)
	getPointer(instance):RegisterTouchCameraMovementMode()
end
function functions.abi.dyn_player_scripts_register_touch_movement_mode(instance)
	getPointer(instance):RegisterTouchMovementMode()
end
function functions.abi.dyn_pose_add_sub_pose(instance, poseSome, poseValue)
	getPointer(instance):AddSubPose(poseSome == 1 and getPointer(poseValue) or nil)
end
function functions.abi.dyn_pose_get_sub_poses(instance)
	return createPointer(getPointer(instance):GetSubPoses());
end
function functions.abi.dyn_pose_remove_sub_pose(instance, poseSome, poseValue)
	getPointer(instance):RemoveSubPose(poseSome == 1 and getPointer(poseValue) or nil)
end
function functions.abi.dyn_proximity_prompt_input_hold_begin(instance)
	getPointer(instance):InputHoldBegin()
end
function functions.abi.dyn_proximity_prompt_input_hold_end(instance)
	getPointer(instance):InputHoldEnd()
end
function functions.abi.dyn_remote_event_fire_all_clients(instance)
	getPointer(instance):FireAllClients()
end
function functions.abi.dyn_remote_event_fire_client(instance, playerSome, playerValue)
	getPointer(instance):FireClient(playerSome == 1 and getPointer(playerValue) or nil)
end
function functions.abi.dyn_remote_event_fire_server(instance)
	getPointer(instance):FireServer()
end
function functions.abi.dyn_remote_function_invoke_client(instance, playerSome, playerValue)
	getPointer(instance):InvokeClient(playerSome == 1 and getPointer(playerValue) or nil)
end
function functions.abi.dyn_remote_function_invoke_server(instance)
	getPointer(instance):InvokeServer()
end
function functions.abi.dyn_replicated_first_remove_default_loading_screen(instance)
	getPointer(instance):RemoveDefaultLoadingScreen()
end
function functions.abi.dyn_rotation_curve_get_key_at_index(instance)
	return createPointer(getPointer(instance):GetKeyAtIndex());
end
function functions.abi.dyn_rotation_curve_get_key_indices_at_time(instance)
	getPointer(instance):GetKeyIndicesAtTime()
end
function functions.abi.dyn_rotation_curve_get_keys(instance)
	getPointer(instance):GetKeys()
end
function functions.abi.dyn_rotation_curve_get_value_at_time(instance)
	return createPointer(getPointer(instance):GetValueAtTime());
end
function functions.abi.dyn_rotation_curve_insert_key(instance, key)
	getPointer(instance):InsertKey(getPointer(key))
end
function functions.abi.dyn_rotation_curve_remove_key_at_index(instance)
	return getPointer(instance):RemoveKeyAtIndex();
end
function functions.abi.dyn_rotation_curve_set_keys(instance)
	return getPointer(instance):SetKeys();
end
function functions.abi.dyn_service_provider_find_service(addr, instance, classNameAddr, classNameLength)
	local class = getPointer(instance):FindService(readString(classNameAddr, classNameLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_service_provider_get_service(addr, instance, classNameAddr, classNameLength)
	local class = getPointer(instance):GetService(readString(classNameAddr, classNameLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_data_model_bind_to_close(instance, p_function)
	getPointer(instance):BindToClose(getPointer(p_function))
end
function functions.abi.dyn_data_model_get_message(addr, instance)
	local str = getPointer(instance):GetMessage();
	writeStringReturn(addr, str)
end
function functions.abi.dyn_data_model_get_remote_build_mode(instance)
	return getPointer(instance):GetRemoteBuildMode() and 1 or 0;
end
function functions.abi.dyn_data_model_is_gear_type_allowed(instance)
	return getPointer(instance):IsGearTypeAllowed() and 1 or 0;
end
function functions.abi.dyn_data_model_is_loaded(instance)
	return getPointer(instance):IsLoaded() and 1 or 0;
end
function functions.abi.dyn_data_model_save_place(instance)
	return getPointer(instance):SavePlace() and 1 or 0;
end
function functions.abi.dyn_user_settings_is_user_feature_enabled(instance, nameAddr, nameLength)
	return getPointer(instance):IsUserFeatureEnabled(readString(nameAddr, nameLength)) and 1 or 0;
end
function functions.abi.dyn_user_settings_reset(instance)
	getPointer(instance):Reset()
end
function functions.abi.dyn_sound_pause(instance)
	getPointer(instance):Pause()
end
function functions.abi.dyn_sound_play(instance)
	getPointer(instance):Play()
end
function functions.abi.dyn_sound_resume(instance)
	getPointer(instance):Resume()
end
function functions.abi.dyn_sound_stop(instance)
	getPointer(instance):Stop()
end
function functions.abi.dyn_team_get_players(instance)
	return createPointer(getPointer(instance):GetPlayers());
end
function functions.abi.dyn_teams_get_teams(instance)
	return createPointer(getPointer(instance):GetTeams());
end
function functions.abi.dyn_teams_rebalance_teams(instance)
	getPointer(instance):RebalanceTeams()
end
function functions.abi.dyn_teleport_options_get_teleport_data(instance)
	getPointer(instance):GetTeleportData()
end
function functions.abi.dyn_teleport_options_set_teleport_data(instance)
	getPointer(instance):SetTeleportData()
end
function functions.abi.dyn_text_channel_display_system_message(addr, instance, systemMessageAddr, systemMessageLength, metadataAddr, metadataLength)
	local class = getPointer(instance):DisplaySystemMessage(readString(systemMessageAddr, systemMessageLength), readString(metadataAddr, metadataLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_text_channel_add_user_async(instance)
	getPointer(instance):AddUserAsync()
end
function functions.abi.dyn_text_channel_send_async(addr, instance, messageAddr, messageLength, metadataAddr, metadataLength)
	local class = getPointer(instance):SendAsync(readString(messageAddr, messageLength), readString(metadataAddr, metadataLength));
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_text_filter_result_get_chat_for_user_async(addr, instance)
	local str = getPointer(instance):GetChatForUserAsync();
	writeStringReturn(addr, str)
end
function functions.abi.dyn_text_filter_result_get_non_chat_string_for_broadcast_async(addr, instance)
	local str = getPointer(instance):GetNonChatStringForBroadcastAsync();
	writeStringReturn(addr, str)
end
function functions.abi.dyn_text_filter_result_get_non_chat_string_for_user_async(addr, instance)
	local str = getPointer(instance):GetNonChatStringForUserAsync();
	writeStringReturn(addr, str)
end
function functions.abi.dyn_trail_clear(instance)
	getPointer(instance):Clear()
end
function functions.abi.dyn_translator_format_by_key(addr, instance, keyAddr, keyLength)
	local str = getPointer(instance):FormatByKey(readString(keyAddr, keyLength));
	writeStringReturn(addr, str)
end
function functions.abi.dyn_translator_translate(addr, instance, contextSome, contextValue, textAddr, textLength)
	local str = getPointer(instance):Translate(contextSome == 1 and getPointer(contextValue) or nil, readString(textAddr, textLength));
	writeStringReturn(addr, str)
end
function functions.abi.dyn_tween_base_cancel(instance)
	getPointer(instance):Cancel()
end
function functions.abi.dyn_tween_base_pause(instance)
	getPointer(instance):Pause()
end
function functions.abi.dyn_tween_base_play(instance)
	getPointer(instance):Play()
end
function functions.abi.dyn_ui_grid_style_layout_apply_layout(instance)
	getPointer(instance):ApplyLayout()
end
function functions.abi.dyn_ui_grid_style_layout_set_custom_sort_function(instance, p_function)
	getPointer(instance):SetCustomSortFunction(getPointer(p_function))
end
function functions.abi.dyn_ui_page_layout_jump_to(instance, pageSome, pageValue)
	getPointer(instance):JumpTo(pageSome == 1 and getPointer(pageValue) or nil)
end
function functions.abi.dyn_ui_page_layout_jump_to_index(instance)
	getPointer(instance):JumpToIndex()
end
function functions.abi.dyn_ui_page_layout_next(instance)
	getPointer(instance):Next()
end
function functions.abi.dyn_ui_page_layout_previous(instance)
	getPointer(instance):Previous()
end
function functions.abi.dyn_user_game_settings_get_camera_y_invert_value(instance)
	return getPointer(instance):GetCameraYInvertValue();
end
function functions.abi.dyn_user_game_settings_get_onboarding_completed(instance, onboardingIdAddr, onboardingIdLength)
	return getPointer(instance):GetOnboardingCompleted(readString(onboardingIdAddr, onboardingIdLength)) and 1 or 0;
end
function functions.abi.dyn_user_game_settings_in_full_screen(instance)
	return getPointer(instance):InFullScreen() and 1 or 0;
end
function functions.abi.dyn_user_game_settings_in_studio_mode(instance)
	return getPointer(instance):InStudioMode() and 1 or 0;
end
function functions.abi.dyn_user_game_settings_set_camera_y_invert_visible(instance)
	getPointer(instance):SetCameraYInvertVisible()
end
function functions.abi.dyn_user_game_settings_set_gamepad_camera_sensitivity_visible(instance)
	getPointer(instance):SetGamepadCameraSensitivityVisible()
end
function functions.abi.dyn_user_game_settings_set_onboarding_completed(instance, onboardingIdAddr, onboardingIdLength)
	getPointer(instance):SetOnboardingCompleted(readString(onboardingIdAddr, onboardingIdLength))
end
function functions.abi.dyn_vector_3_curve_get_value_at_time(instance)
	getPointer(instance):GetValueAtTime()
end
function functions.abi.dyn_vector_3_curve_x(addr, instance)
	local class = getPointer(instance):X();
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_vector_3_curve_y(addr, instance)
	local class = getPointer(instance):Y();
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
function functions.abi.dyn_vector_3_curve_z(addr, instance)
	local class = getPointer(instance):Z();
	store_i32(memory, addr, class and 1 or 0);
	store_i32(memory, addr + 4, class and createPointer(class) or 0);
end
return functions;
