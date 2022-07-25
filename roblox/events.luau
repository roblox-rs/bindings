local memory, rt, util;
local store_i32, pushStack, popStack, invokeFunction, getPointer, createPointer, allocString, dropFunctionRef;
local signal = { abi = {} };
local connections = {};
local function createConnection(stack, vtable, connection)
	local id = createPointer(connection);
	connections[id] = { stack, vtable };
	return id
end
function signal.withWasm(localMemory, localRuntime, localUtil)
	memory, rt, util = localMemory, localRuntime, localUtil
	store_i32, pushStack, popStack, invokeFunction, getPointer, createPointer, allocString, dropFunctionRef = rt.store.i32, util.pushStack, util.popStack, util.invokeFunction, util.getPointer, util.createPointer, util.allocString, util.dropFunctionRef;
end
function signal.abi.disconnect_connection(connection)
	local func = connections[connection];
	getPointer(connection):Disconnect()
	dropFunctionRef(func[1], func[2]);
	connections[connection] = nil
end
function signal.abi.is_connected(connection)
	return getPointer(connection).Connected and 1 or 0;
end
function signal.abi.connect_instance_ancestry_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).AncestryChanged:Connect(function(child, parent)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, child and 1 or 0);
		store_i32(memory, stackPointer + 4, child and createPointer(child) or 0);
		store_i32(memory, stackPointer + 8, parent and 1 or 0);
		store_i32(memory, stackPointer + 12, parent and createPointer(parent) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.abi.connect_instance_attribute_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).AttributeChanged:Connect(function(attribute)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(attribute));
		store_i32(memory, stackPointer + 4, #attribute);
		store_i32(memory, stackPointer + 8, #attribute);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_instance_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Changed:Connect(function(property)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(property));
		store_i32(memory, stackPointer + 4, #property);
		store_i32(memory, stackPointer + 8, #property);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_instance_child_added(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).ChildAdded:Connect(function(child)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, child and 1 or 0);
		store_i32(memory, stackPointer + 4, child and createPointer(child) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_instance_child_removed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).ChildRemoved:Connect(function(child)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, child and 1 or 0);
		store_i32(memory, stackPointer + 4, child and createPointer(child) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_instance_descendant_added(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).DescendantAdded:Connect(function(descendant)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, descendant and 1 or 0);
		store_i32(memory, stackPointer + 4, descendant and createPointer(descendant) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_instance_descendant_removing(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).DescendantRemoving:Connect(function(descendant)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, descendant and 1 or 0);
		store_i32(memory, stackPointer + 4, descendant and createPointer(descendant) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_instance_destroying(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Destroying:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_animation_controller_animation_played(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).AnimationPlayed:Connect(function(animationTrack)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, animationTrack and 1 or 0);
		store_i32(memory, stackPointer + 4, animationTrack and createPointer(animationTrack) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_animation_track_did_loop(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).DidLoop:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_animation_track_keyframe_reached(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).KeyframeReached:Connect(function(keyframeName)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(keyframeName));
		store_i32(memory, stackPointer + 4, #keyframeName);
		store_i32(memory, stackPointer + 8, #keyframeName);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_animation_track_stopped(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Stopped:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_animator_animation_played(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).AnimationPlayed:Connect(function(animationTrack)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, animationTrack and 1 or 0);
		store_i32(memory, stackPointer + 4, animationTrack and createPointer(animationTrack) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_asset_import_service_progress_update(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).ProgressUpdate:Connect(function(progressRatio)
		invokeFunction(stack, vtable, progressRatio);
	end))
end
function signal.abi.connect_asset_import_service_upload_finished(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).UploadFinished:Connect(function(succeeded, errorMap)
		invokeFunction(stack, vtable, succeeded and 1 or 0, error(errorMap));
	end))
end
function signal.abi.connect_avatar_editor_service_prompt_allow_inventory_read_access_completed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptAllowInventoryReadAccessCompleted:Connect(function(result)
		invokeFunction(stack, vtable, error(result));
	end))
end
function signal.abi.connect_avatar_editor_service_prompt_create_outfit_completed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptCreateOutfitCompleted:Connect(function(result, failureType)
		invokeFunction(stack, vtable, error(result), error(failureType));
	end))
end
function signal.abi.connect_avatar_editor_service_prompt_delete_outfit_completed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptDeleteOutfitCompleted:Connect(function(result)
		invokeFunction(stack, vtable, error(result));
	end))
end
function signal.abi.connect_avatar_editor_service_prompt_rename_outfit_completed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptRenameOutfitCompleted:Connect(function(result)
		invokeFunction(stack, vtable, error(result));
	end))
end
function signal.abi.connect_avatar_editor_service_prompt_save_avatar_completed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptSaveAvatarCompleted:Connect(function(result, humanoidDescription)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, humanoidDescription and 1 or 0);
		store_i32(memory, stackPointer + 4, humanoidDescription and createPointer(humanoidDescription) or 0);
		invokeFunction(stack, vtable, error(result), stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_avatar_editor_service_prompt_set_favorite_completed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptSetFavoriteCompleted:Connect(function(result)
		invokeFunction(stack, vtable, error(result));
	end))
end
function signal.abi.connect_avatar_editor_service_prompt_update_outfit_completed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptUpdateOutfitCompleted:Connect(function(result)
		invokeFunction(stack, vtable, error(result));
	end))
end
function signal.abi.connect_tool_activated(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Activated:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_tool_deactivated(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Deactivated:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_tool_equipped(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Equipped:Connect(function(mouse)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, mouse and 1 or 0);
		store_i32(memory, stackPointer + 4, mouse and createPointer(mouse) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_tool_unequipped(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Unequipped:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_player_gui_topbar_transparency_changed_signal(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TopbarTransparencyChangedSignal:Connect(function(transparency)
		invokeFunction(stack, vtable, transparency);
	end))
end
function signal.abi.connect_bindable_event_event(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Event:Connect(function(arguments)
		invokeFunction(stack, vtable, error(arguments));
	end))
end
function signal.abi.connect_body_position_reached_target(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).ReachedTarget:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_rocket_propulsion_reached_target(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).ReachedTarget:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_camera_interpolation_finished(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).InterpolationFinished:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_chat_chatted(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Chatted:Connect(function(part, message, color)
		local stackPointer = pushStack(20);
		store_i32(memory, stackPointer + 0, part and 1 or 0);
		store_i32(memory, stackPointer + 4, part and createPointer(part) or 0);
		store_i32(memory, stackPointer + 8, allocString(message));
		store_i32(memory, stackPointer + 12, #message);
		store_i32(memory, stackPointer + 16, #message);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8, error(color));
		popStack(stackPointer, 20);
	end))
end
function signal.abi.connect_click_detector_mouse_click(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseClick:Connect(function(playerWhoClicked)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoClicked and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoClicked and createPointer(playerWhoClicked) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_click_detector_mouse_hover_enter(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseHoverEnter:Connect(function(playerWhoHovered)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoHovered and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoHovered and createPointer(playerWhoHovered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_click_detector_mouse_hover_leave(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseHoverLeave:Connect(function(playerWhoHovered)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoHovered and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoHovered and createPointer(playerWhoHovered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_click_detector_right_mouse_click(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).RightMouseClick:Connect(function(playerWhoClicked)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoClicked and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoClicked and createPointer(playerWhoClicked) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_collection_service_item_added(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).ItemAdded:Connect(function(instance)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, instance and 1 or 0);
		store_i32(memory, stackPointer + 4, instance and createPointer(instance) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_collection_service_item_removed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).ItemRemoved:Connect(function(instance)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, instance and 1 or 0);
		store_i32(memory, stackPointer + 4, instance and createPointer(instance) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_collection_service_tag_added(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TagAdded:Connect(function(tag)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(tag));
		store_i32(memory, stackPointer + 4, #tag);
		store_i32(memory, stackPointer + 8, #tag);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_collection_service_tag_removed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TagRemoved:Connect(function(tag)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(tag));
		store_i32(memory, stackPointer + 4, #tag);
		store_i32(memory, stackPointer + 8, #tag);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_content_provider_asset_fetch_failed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).AssetFetchFailed:Connect(function(assetId)
		invokeFunction(stack, vtable, createPointer(assetId));
	end))
end
function signal.abi.connect_context_action_service_local_tool_equipped(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).LocalToolEquipped:Connect(function(toolEquipped)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, toolEquipped and 1 or 0);
		store_i32(memory, stackPointer + 4, toolEquipped and createPointer(toolEquipped) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_context_action_service_local_tool_unequipped(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).LocalToolUnequipped:Connect(function(toolUnequipped)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, toolUnequipped and 1 or 0);
		store_i32(memory, stackPointer + 4, toolUnequipped and createPointer(toolUnequipped) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_controller_button_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).ButtonChanged:Connect(function(button)
		invokeFunction(stack, vtable, error(button));
	end))
end
function signal.abi.connect_skateboard_controller_axis_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).AxisChanged:Connect(function(axis)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(axis));
		store_i32(memory, stackPointer + 4, #axis);
		store_i32(memory, stackPointer + 8, #axis);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_dialog_dialog_choice_selected(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).DialogChoiceSelected:Connect(function(player, dialogChoice)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		store_i32(memory, stackPointer + 8, dialogChoice and 1 or 0);
		store_i32(memory, stackPointer + 12, dialogChoice and createPointer(dialogChoice) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.abi.connect_explosion_hit(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Hit:Connect(function(part, distance)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, part and 1 or 0);
		store_i32(memory, stackPointer + 4, part and createPointer(part) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, distance);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_gui_base_2_d_selection_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).SelectionChanged:Connect(function(amISelected, previousSelection, newSelection)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, previousSelection and 1 or 0);
		store_i32(memory, stackPointer + 4, previousSelection and createPointer(previousSelection) or 0);
		store_i32(memory, stackPointer + 8, newSelection and 1 or 0);
		store_i32(memory, stackPointer + 12, newSelection and createPointer(newSelection) or 0);
		invokeFunction(stack, vtable, amISelected and 1 or 0, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.abi.connect_gui_object_drag_begin(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).DragBegin:Connect(function(initialPosition)
		invokeFunction(stack, vtable, createPointer(initialPosition));
	end))
end
function signal.abi.connect_gui_object_drag_stopped(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).DragStopped:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.abi.connect_gui_object_input_began(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).InputBegan:Connect(function(input)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, input and 1 or 0);
		store_i32(memory, stackPointer + 4, input and createPointer(input) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_gui_object_input_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).InputChanged:Connect(function(input)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, input and 1 or 0);
		store_i32(memory, stackPointer + 4, input and createPointer(input) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_gui_object_input_ended(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).InputEnded:Connect(function(input)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, input and 1 or 0);
		store_i32(memory, stackPointer + 4, input and createPointer(input) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_gui_object_mouse_enter(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseEnter:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.abi.connect_gui_object_mouse_leave(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseLeave:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.abi.connect_gui_object_mouse_moved(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseMoved:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.abi.connect_gui_object_mouse_wheel_backward(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseWheelBackward:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.abi.connect_gui_object_mouse_wheel_forward(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseWheelForward:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.abi.connect_gui_object_selection_gained(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).SelectionGained:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_gui_object_selection_lost(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).SelectionLost:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_gui_object_touch_long_press(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchLongPress:Connect(function(touchPositions, state)
		invokeFunction(stack, vtable, error(touchPositions), error(state));
	end))
end
function signal.abi.connect_gui_object_touch_pan(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchPan:Connect(function(touchPositions, totalTranslation, velocity, state)
		invokeFunction(stack, vtable, error(touchPositions), createPointer(totalTranslation), createPointer(velocity), error(state));
	end))
end
function signal.abi.connect_gui_object_touch_pinch(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchPinch:Connect(function(touchPositions, scale, velocity, state)
		invokeFunction(stack, vtable, error(touchPositions), scale, velocity, error(state));
	end))
end
function signal.abi.connect_gui_object_touch_rotate(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchRotate:Connect(function(touchPositions, rotation, velocity, state)
		invokeFunction(stack, vtable, error(touchPositions), rotation, velocity, error(state));
	end))
end
function signal.abi.connect_gui_object_touch_swipe(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchSwipe:Connect(function(swipeDirection, numberOfTouches)
		invokeFunction(stack, vtable, error(swipeDirection), numberOfTouches);
	end))
end
function signal.abi.connect_gui_object_touch_tap(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchTap:Connect(function(touchPositions)
		invokeFunction(stack, vtable, error(touchPositions));
	end))
end
function signal.abi.connect_gui_button_activated(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Activated:Connect(function(inputObject, clickCount)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, inputObject and 1 or 0);
		store_i32(memory, stackPointer + 4, inputObject and createPointer(inputObject) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, clickCount);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_gui_button_mouse_button_1_click(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseButton1Click:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_gui_button_mouse_button_1_down(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseButton1Down:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.abi.connect_gui_button_mouse_button_1_up(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseButton1Up:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.abi.connect_gui_button_mouse_button_2_click(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseButton2Click:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_gui_button_mouse_button_2_down(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseButton2Down:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.abi.connect_gui_button_mouse_button_2_up(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseButton2Up:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.abi.connect_text_box_focus_lost(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).FocusLost:Connect(function(enterPressed, inputThatCausedFocusLoss)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, inputThatCausedFocusLoss and 1 or 0);
		store_i32(memory, stackPointer + 4, inputThatCausedFocusLoss and createPointer(inputThatCausedFocusLoss) or 0);
		invokeFunction(stack, vtable, enterPressed and 1 or 0, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_text_box_focused(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Focused:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_text_box_return_pressed_from_on_screen_keyboard(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).ReturnPressedFromOnScreenKeyboard:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_video_frame_did_loop(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).DidLoop:Connect(function(video)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(video));
		store_i32(memory, stackPointer + 4, #video);
		store_i32(memory, stackPointer + 8, #video);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_video_frame_ended(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Ended:Connect(function(video)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(video));
		store_i32(memory, stackPointer + 4, #video);
		store_i32(memory, stackPointer + 8, #video);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_video_frame_loaded(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Loaded:Connect(function(video)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(video));
		store_i32(memory, stackPointer + 4, #video);
		store_i32(memory, stackPointer + 8, #video);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_video_frame_paused(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Paused:Connect(function(video)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(video));
		store_i32(memory, stackPointer + 4, #video);
		store_i32(memory, stackPointer + 8, #video);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_video_frame_played(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Played:Connect(function(video)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(video));
		store_i32(memory, stackPointer + 4, #video);
		store_i32(memory, stackPointer + 8, #video);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_handle_adornment_mouse_button_1_down(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseButton1Down:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_handle_adornment_mouse_button_1_up(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseButton1Up:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_handle_adornment_mouse_enter(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseEnter:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_handle_adornment_mouse_leave(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseLeave:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_arc_handles_mouse_button_1_down(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseButton1Down:Connect(function(axis)
		invokeFunction(stack, vtable, error(axis));
	end))
end
function signal.abi.connect_arc_handles_mouse_button_1_up(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseButton1Up:Connect(function(axis)
		invokeFunction(stack, vtable, error(axis));
	end))
end
function signal.abi.connect_arc_handles_mouse_drag(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseDrag:Connect(function(axis, relativeAngle, deltaRadius)
		invokeFunction(stack, vtable, error(axis), relativeAngle, deltaRadius);
	end))
end
function signal.abi.connect_arc_handles_mouse_enter(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseEnter:Connect(function(axis)
		invokeFunction(stack, vtable, error(axis));
	end))
end
function signal.abi.connect_arc_handles_mouse_leave(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseLeave:Connect(function(axis)
		invokeFunction(stack, vtable, error(axis));
	end))
end
function signal.abi.connect_handles_mouse_button_1_down(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseButton1Down:Connect(function(face)
		invokeFunction(stack, vtable, error(face));
	end))
end
function signal.abi.connect_handles_mouse_button_1_up(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseButton1Up:Connect(function(face)
		invokeFunction(stack, vtable, error(face));
	end))
end
function signal.abi.connect_handles_mouse_drag(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseDrag:Connect(function(face, distance)
		invokeFunction(stack, vtable, error(face), distance);
	end))
end
function signal.abi.connect_handles_mouse_enter(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseEnter:Connect(function(face)
		invokeFunction(stack, vtable, error(face));
	end))
end
function signal.abi.connect_handles_mouse_leave(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MouseLeave:Connect(function(face)
		invokeFunction(stack, vtable, error(face));
	end))
end
function signal.abi.connect_gui_service_menu_closed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MenuClosed:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_gui_service_menu_opened(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MenuOpened:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_humanoid_animation_played(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).AnimationPlayed:Connect(function(animationTrack)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, animationTrack and 1 or 0);
		store_i32(memory, stackPointer + 4, animationTrack and createPointer(animationTrack) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_humanoid_climbing(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Climbing:Connect(function(speed)
		invokeFunction(stack, vtable, speed);
	end))
end
function signal.abi.connect_humanoid_custom_status_added(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).CustomStatusAdded:Connect(function(status)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(status));
		store_i32(memory, stackPointer + 4, #status);
		store_i32(memory, stackPointer + 8, #status);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_humanoid_custom_status_removed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).CustomStatusRemoved:Connect(function(status)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(status));
		store_i32(memory, stackPointer + 4, #status);
		store_i32(memory, stackPointer + 8, #status);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_humanoid_died(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Died:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_humanoid_falling_down(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).FallingDown:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.abi.connect_humanoid_free_falling(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).FreeFalling:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.abi.connect_humanoid_getting_up(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).GettingUp:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.abi.connect_humanoid_health_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).HealthChanged:Connect(function(health)
		invokeFunction(stack, vtable, health);
	end))
end
function signal.abi.connect_humanoid_jumping(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Jumping:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.abi.connect_humanoid_move_to_finished(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MoveToFinished:Connect(function(reached)
		invokeFunction(stack, vtable, reached and 1 or 0);
	end))
end
function signal.abi.connect_humanoid_platform_standing(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PlatformStanding:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.abi.connect_humanoid_ragdoll(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Ragdoll:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.abi.connect_humanoid_running(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Running:Connect(function(speed)
		invokeFunction(stack, vtable, speed);
	end))
end
function signal.abi.connect_humanoid_seated(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Seated:Connect(function(active, currentSeatPart)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, currentSeatPart and 1 or 0);
		store_i32(memory, stackPointer + 4, currentSeatPart and createPointer(currentSeatPart) or 0);
		invokeFunction(stack, vtable, active and 1 or 0, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_humanoid_state_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).StateChanged:Connect(function(old, new)
		invokeFunction(stack, vtable, error(old), error(new));
	end))
end
function signal.abi.connect_humanoid_state_enabled_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).StateEnabledChanged:Connect(function(state, isEnabled)
		invokeFunction(stack, vtable, error(state), isEnabled and 1 or 0);
	end))
end
function signal.abi.connect_humanoid_status_added(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).StatusAdded:Connect(function(status)
		invokeFunction(stack, vtable, error(status));
	end))
end
function signal.abi.connect_humanoid_status_removed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).StatusRemoved:Connect(function(status)
		invokeFunction(stack, vtable, error(status));
	end))
end
function signal.abi.connect_humanoid_strafing(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Strafing:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.abi.connect_humanoid_swimming(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Swimming:Connect(function(speed)
		invokeFunction(stack, vtable, speed);
	end))
end
function signal.abi.connect_humanoid_touched(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Touched:Connect(function(touchingPart, humanoidPart)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, touchingPart and 1 or 0);
		store_i32(memory, stackPointer + 4, touchingPart and createPointer(touchingPart) or 0);
		store_i32(memory, stackPointer + 8, humanoidPart and 1 or 0);
		store_i32(memory, stackPointer + 12, humanoidPart and createPointer(humanoidPart) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.abi.connect_humanoid_description_emotes_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).EmotesChanged:Connect(function(newEmotes)
		invokeFunction(stack, vtable, error(newEmotes));
	end))
end
function signal.abi.connect_humanoid_description_equipped_emotes_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).EquippedEmotesChanged:Connect(function(newEquippedEmotes)
		invokeFunction(stack, vtable, error(newEquippedEmotes));
	end))
end
function signal.abi.connect_lighting_lighting_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).LightingChanged:Connect(function(skyChanged)
		invokeFunction(stack, vtable, skyChanged and 1 or 0);
	end))
end
function signal.abi.connect_log_service_message_out(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MessageOut:Connect(function(message, messageType)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(message));
		store_i32(memory, stackPointer + 4, #message);
		store_i32(memory, stackPointer + 8, #message);
		invokeFunction(stack, vtable, stackPointer + 0, error(messageType));
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_marketplace_service_prompt_bundle_purchase_finished(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptBundlePurchaseFinished:Connect(function(player, bundleId, wasPurchased)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, bundleId, wasPurchased and 1 or 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_marketplace_service_prompt_game_pass_purchase_finished(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptGamePassPurchaseFinished:Connect(function(player, gamePassId, wasPurchased)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, gamePassId, wasPurchased and 1 or 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_marketplace_service_prompt_premium_purchase_finished(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptPremiumPurchaseFinished:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_marketplace_service_prompt_product_purchase_finished(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptProductPurchaseFinished:Connect(function(userId, productId, isPurchased)
		invokeFunction(stack, vtable, userId, productId, isPurchased and 1 or 0);
	end))
end
function signal.abi.connect_marketplace_service_prompt_purchase_finished(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptPurchaseFinished:Connect(function(player, assetId, isPurchased)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, assetId, isPurchased and 1 or 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_marketplace_service_prompt_subscription_cancellation_finished(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptSubscriptionCancellationFinished:Connect(function(player, subscriptionId, wasCanceled)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, subscriptionId, wasCanceled and 1 or 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_marketplace_service_prompt_subscription_purchase_finished(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptSubscriptionPurchaseFinished:Connect(function(player, subscriptionId, wasPurchased)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, subscriptionId, wasPurchased and 1 or 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_mouse_button_1_down(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Button1Down:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_mouse_button_1_up(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Button1Up:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_mouse_button_2_down(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Button2Down:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_mouse_button_2_up(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Button2Up:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_mouse_idle(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Idle:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_mouse_key_down(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).KeyDown:Connect(function(key)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(key));
		store_i32(memory, stackPointer + 4, #key);
		store_i32(memory, stackPointer + 8, #key);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_mouse_key_up(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).KeyUp:Connect(function(key)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(key));
		store_i32(memory, stackPointer + 4, #key);
		store_i32(memory, stackPointer + 8, #key);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_mouse_move(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Move:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_mouse_wheel_backward(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).WheelBackward:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_mouse_wheel_forward(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).WheelForward:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_network_marker_received(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Received:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_base_part_local_simulation_touched(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).LocalSimulationTouched:Connect(function(part)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, part and 1 or 0);
		store_i32(memory, stackPointer + 4, part and createPointer(part) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_base_part_outfit_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).OutfitChanged:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_base_part_stopped_touching(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).StoppedTouching:Connect(function(otherPart)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, otherPart and 1 or 0);
		store_i32(memory, stackPointer + 4, otherPart and createPointer(otherPart) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_base_part_touch_ended(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchEnded:Connect(function(otherPart)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, otherPart and 1 or 0);
		store_i32(memory, stackPointer + 4, otherPart and createPointer(otherPart) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_base_part_touched(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Touched:Connect(function(otherPart)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, otherPart and 1 or 0);
		store_i32(memory, stackPointer + 4, otherPart and createPointer(otherPart) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_skateboard_platform_equipped(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Equipped:Connect(function(humanoid, skateboardController)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, humanoid and 1 or 0);
		store_i32(memory, stackPointer + 4, humanoid and createPointer(humanoid) or 0);
		store_i32(memory, stackPointer + 8, skateboardController and 1 or 0);
		store_i32(memory, stackPointer + 12, skateboardController and createPointer(skateboardController) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.abi.connect_skateboard_platform_move_state_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MoveStateChanged:Connect(function(newState, oldState)
		invokeFunction(stack, vtable, error(newState), error(oldState));
	end))
end
function signal.abi.connect_skateboard_platform_unequipped(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Unequipped:Connect(function(humanoid)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, humanoid and 1 or 0);
		store_i32(memory, stackPointer + 4, humanoid and createPointer(humanoid) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_path_blocked(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Blocked:Connect(function(blockedWaypointIdx)
		invokeFunction(stack, vtable, blockedWaypointIdx);
	end))
end
function signal.abi.connect_path_unblocked(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Unblocked:Connect(function(unblockedWaypointIdx)
		invokeFunction(stack, vtable, unblockedWaypointIdx);
	end))
end
function signal.abi.connect_player_character_added(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).CharacterAdded:Connect(function(character)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, character and 1 or 0);
		store_i32(memory, stackPointer + 4, character and createPointer(character) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_player_character_appearance_loaded(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).CharacterAppearanceLoaded:Connect(function(character)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, character and 1 or 0);
		store_i32(memory, stackPointer + 4, character and createPointer(character) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_player_character_removing(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).CharacterRemoving:Connect(function(character)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, character and 1 or 0);
		store_i32(memory, stackPointer + 4, character and createPointer(character) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_player_chatted(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Chatted:Connect(function(message, recipient)
		local stackPointer = pushStack(20);
		store_i32(memory, stackPointer + 0, allocString(message));
		store_i32(memory, stackPointer + 4, #message);
		store_i32(memory, stackPointer + 8, #message);
		store_i32(memory, stackPointer + 12, recipient and 1 or 0);
		store_i32(memory, stackPointer + 16, recipient and createPointer(recipient) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 12);
		popStack(stackPointer, 20);
	end))
end
function signal.abi.connect_player_idled(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Idled:Connect(function(time)
		invokeFunction(stack, vtable, time);
	end))
end
function signal.abi.connect_player_on_teleport(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).OnTeleport:Connect(function(teleportState, placeId, spawnName)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(spawnName));
		store_i32(memory, stackPointer + 4, #spawnName);
		store_i32(memory, stackPointer + 8, #spawnName);
		invokeFunction(stack, vtable, error(teleportState), placeId, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_players_player_added(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PlayerAdded:Connect(function(player)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_players_player_membership_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PlayerMembershipChanged:Connect(function(player)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_players_player_removing(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PlayerRemoving:Connect(function(player)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_proximity_prompt_prompt_button_hold_began(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptButtonHoldBegan:Connect(function(playerWhoTriggered)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoTriggered and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoTriggered and createPointer(playerWhoTriggered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_proximity_prompt_prompt_button_hold_ended(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptButtonHoldEnded:Connect(function(playerWhoTriggered)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoTriggered and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoTriggered and createPointer(playerWhoTriggered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_proximity_prompt_prompt_hidden(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptHidden:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_proximity_prompt_prompt_shown(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptShown:Connect(function(inputType)
		invokeFunction(stack, vtable, error(inputType));
	end))
end
function signal.abi.connect_proximity_prompt_trigger_ended(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TriggerEnded:Connect(function(playerWhoTriggered)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoTriggered and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoTriggered and createPointer(playerWhoTriggered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_proximity_prompt_triggered(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Triggered:Connect(function(playerWhoTriggered)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoTriggered and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoTriggered and createPointer(playerWhoTriggered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_proximity_prompt_service_prompt_button_hold_began(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptButtonHoldBegan:Connect(function(prompt, playerWhoTriggered)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, prompt and 1 or 0);
		store_i32(memory, stackPointer + 4, prompt and createPointer(prompt) or 0);
		store_i32(memory, stackPointer + 8, playerWhoTriggered and 1 or 0);
		store_i32(memory, stackPointer + 12, playerWhoTriggered and createPointer(playerWhoTriggered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.abi.connect_proximity_prompt_service_prompt_button_hold_ended(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptButtonHoldEnded:Connect(function(prompt, playerWhoTriggered)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, prompt and 1 or 0);
		store_i32(memory, stackPointer + 4, prompt and createPointer(prompt) or 0);
		store_i32(memory, stackPointer + 8, playerWhoTriggered and 1 or 0);
		store_i32(memory, stackPointer + 12, playerWhoTriggered and createPointer(playerWhoTriggered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.abi.connect_proximity_prompt_service_prompt_hidden(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptHidden:Connect(function(prompt)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, prompt and 1 or 0);
		store_i32(memory, stackPointer + 4, prompt and createPointer(prompt) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_proximity_prompt_service_prompt_shown(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptShown:Connect(function(prompt, inputType)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, prompt and 1 or 0);
		store_i32(memory, stackPointer + 4, prompt and createPointer(prompt) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, error(inputType));
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_proximity_prompt_service_prompt_trigger_ended(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptTriggerEnded:Connect(function(prompt, playerWhoTriggered)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, prompt and 1 or 0);
		store_i32(memory, stackPointer + 4, prompt and createPointer(prompt) or 0);
		store_i32(memory, stackPointer + 8, playerWhoTriggered and 1 or 0);
		store_i32(memory, stackPointer + 12, playerWhoTriggered and createPointer(playerWhoTriggered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.abi.connect_proximity_prompt_service_prompt_triggered(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PromptTriggered:Connect(function(prompt, playerWhoTriggered)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, prompt and 1 or 0);
		store_i32(memory, stackPointer + 4, prompt and createPointer(prompt) or 0);
		store_i32(memory, stackPointer + 8, playerWhoTriggered and 1 or 0);
		store_i32(memory, stackPointer + 12, playerWhoTriggered and createPointer(playerWhoTriggered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.abi.connect_remote_event_on_client_event(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).OnClientEvent:Connect(function(arguments)
		invokeFunction(stack, vtable, error(arguments));
	end))
end
function signal.abi.connect_remote_event_on_server_event(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).OnServerEvent:Connect(function(player, arguments)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, error(arguments));
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_run_service_heartbeat(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Heartbeat:Connect(function(deltaTime)
		invokeFunction(stack, vtable, deltaTime);
	end))
end
function signal.abi.connect_run_service_post_simulation(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PostSimulation:Connect(function(deltaTime)
		invokeFunction(stack, vtable, deltaTime);
	end))
end
function signal.abi.connect_run_service_pre_animation(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PreAnimation:Connect(function(deltaTime)
		invokeFunction(stack, vtable, deltaTime);
	end))
end
function signal.abi.connect_run_service_pre_render(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PreRender:Connect(function(deltaTime)
		invokeFunction(stack, vtable, deltaTime);
	end))
end
function signal.abi.connect_run_service_pre_simulation(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PreSimulation:Connect(function(deltaTime)
		invokeFunction(stack, vtable, deltaTime);
	end))
end
function signal.abi.connect_run_service_render_stepped(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).RenderStepped:Connect(function(deltaTime)
		invokeFunction(stack, vtable, deltaTime);
	end))
end
function signal.abi.connect_run_service_stepped(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Stepped:Connect(function(time, deltaTime)
		invokeFunction(stack, vtable, time, deltaTime);
	end))
end
function signal.abi.connect_script_context_error(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Error:Connect(function(message, stackTrace, script)
		local stackPointer = pushStack(32);
		store_i32(memory, stackPointer + 0, allocString(message));
		store_i32(memory, stackPointer + 4, #message);
		store_i32(memory, stackPointer + 8, #message);
		store_i32(memory, stackPointer + 12, allocString(stackTrace));
		store_i32(memory, stackPointer + 16, #stackTrace);
		store_i32(memory, stackPointer + 20, #stackTrace);
		store_i32(memory, stackPointer + 24, script and 1 or 0);
		store_i32(memory, stackPointer + 28, script and createPointer(script) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 12, stackPointer + 24);
		popStack(stackPointer, 32);
	end))
end
function signal.abi.connect_service_provider_close(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Close:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_service_provider_service_added(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).ServiceAdded:Connect(function(service)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, service and 1 or 0);
		store_i32(memory, stackPointer + 4, service and createPointer(service) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_service_provider_service_removing(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).ServiceRemoving:Connect(function(service)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, service and 1 or 0);
		store_i32(memory, stackPointer + 4, service and createPointer(service) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_data_model_allowed_gear_type_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).AllowedGearTypeChanged:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_data_model_graphics_quality_change_request(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).GraphicsQualityChangeRequest:Connect(function(betterQuality)
		invokeFunction(stack, vtable, betterQuality and 1 or 0);
	end))
end
function signal.abi.connect_data_model_item_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).ItemChanged:Connect(function(object, descriptor)
		local stackPointer = pushStack(20);
		store_i32(memory, stackPointer + 0, object and 1 or 0);
		store_i32(memory, stackPointer + 4, object and createPointer(object) or 0);
		store_i32(memory, stackPointer + 8, allocString(descriptor));
		store_i32(memory, stackPointer + 12, #descriptor);
		store_i32(memory, stackPointer + 16, #descriptor);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 20);
	end))
end
function signal.abi.connect_data_model_loaded(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Loaded:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_social_service_game_invite_prompt_closed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).GameInvitePromptClosed:Connect(function(player, recipientIds)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, error(recipientIds));
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_sound_did_loop(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).DidLoop:Connect(function(soundId, numOfTimesLooped)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0, numOfTimesLooped);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_sound_ended(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Ended:Connect(function(soundId)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_sound_loaded(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Loaded:Connect(function(soundId)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_sound_paused(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Paused:Connect(function(soundId)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_sound_played(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Played:Connect(function(soundId)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_sound_resumed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Resumed:Connect(function(soundId)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_sound_stopped(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Stopped:Connect(function(soundId)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_team_player_added(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PlayerAdded:Connect(function(player)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_team_player_removed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PlayerRemoved:Connect(function(player)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_teleport_service_local_player_arrived_from_teleport(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).LocalPlayerArrivedFromTeleport:Connect(function(loadingGui, dataTable)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, loadingGui and 1 or 0);
		store_i32(memory, stackPointer + 4, loadingGui and createPointer(loadingGui) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, error(dataTable));
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_teleport_service_teleport_init_failed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TeleportInitFailed:Connect(function(player, teleportResult, errorMessage, placeId, teleportOptions)
		local stackPointer = pushStack(28);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		store_i32(memory, stackPointer + 8, allocString(errorMessage));
		store_i32(memory, stackPointer + 12, #errorMessage);
		store_i32(memory, stackPointer + 16, #errorMessage);
		store_i32(memory, stackPointer + 20, teleportOptions and 1 or 0);
		store_i32(memory, stackPointer + 24, teleportOptions and createPointer(teleportOptions) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, error(teleportResult), stackPointer + 8, placeId, stackPointer + 20);
		popStack(stackPointer, 28);
	end))
end
function signal.abi.connect_text_channel_message_received(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MessageReceived:Connect(function(incomingMessage)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, incomingMessage and 1 or 0);
		store_i32(memory, stackPointer + 4, incomingMessage and createPointer(incomingMessage) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_text_chat_command_triggered(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Triggered:Connect(function(originTextSource, unfilteredText)
		local stackPointer = pushStack(20);
		store_i32(memory, stackPointer + 0, originTextSource and 1 or 0);
		store_i32(memory, stackPointer + 4, originTextSource and createPointer(originTextSource) or 0);
		store_i32(memory, stackPointer + 8, allocString(unfilteredText));
		store_i32(memory, stackPointer + 12, #unfilteredText);
		store_i32(memory, stackPointer + 16, #unfilteredText);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 20);
	end))
end
function signal.abi.connect_text_chat_service_message_received(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).MessageReceived:Connect(function(textChatMessage)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, textChatMessage and 1 or 0);
		store_i32(memory, stackPointer + 4, textChatMessage and createPointer(textChatMessage) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_text_chat_service_sending_message(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).SendingMessage:Connect(function(textChatMessage)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, textChatMessage and 1 or 0);
		store_i32(memory, stackPointer + 4, textChatMessage and createPointer(textChatMessage) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_tween_base_completed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Completed:Connect(function(playbackState)
		invokeFunction(stack, vtable, error(playbackState));
	end))
end
function signal.abi.connect_ui_page_layout_page_enter(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PageEnter:Connect(function(page)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, page and 1 or 0);
		store_i32(memory, stackPointer + 4, page and createPointer(page) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_ui_page_layout_page_leave(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PageLeave:Connect(function(page)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, page and 1 or 0);
		store_i32(memory, stackPointer + 4, page and createPointer(page) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_ui_page_layout_stopped(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Stopped:Connect(function(currentPage)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, currentPage and 1 or 0);
		store_i32(memory, stackPointer + 4, currentPage and createPointer(currentPage) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_user_game_settings_fullscreen_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).FullscreenChanged:Connect(function(isFullscreen)
		invokeFunction(stack, vtable, isFullscreen and 1 or 0);
	end))
end
function signal.abi.connect_user_game_settings_studio_mode_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).StudioModeChanged:Connect(function(isStudioMode)
		invokeFunction(stack, vtable, isStudioMode and 1 or 0);
	end))
end
function signal.abi.connect_user_input_service_device_acceleration_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).DeviceAccelerationChanged:Connect(function(acceleration)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, acceleration and 1 or 0);
		store_i32(memory, stackPointer + 4, acceleration and createPointer(acceleration) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_user_input_service_device_gravity_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).DeviceGravityChanged:Connect(function(gravity)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, gravity and 1 or 0);
		store_i32(memory, stackPointer + 4, gravity and createPointer(gravity) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_user_input_service_device_rotation_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).DeviceRotationChanged:Connect(function(rotation, cframe)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, rotation and 1 or 0);
		store_i32(memory, stackPointer + 4, rotation and createPointer(rotation) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, createPointer(cframe));
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_user_input_service_gamepad_connected(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).GamepadConnected:Connect(function(gamepadNum)
		invokeFunction(stack, vtable, error(gamepadNum));
	end))
end
function signal.abi.connect_user_input_service_gamepad_disconnected(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).GamepadDisconnected:Connect(function(gamepadNum)
		invokeFunction(stack, vtable, error(gamepadNum));
	end))
end
function signal.abi.connect_user_input_service_input_began(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).InputBegan:Connect(function(input, gameProcessedEvent)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, input and 1 or 0);
		store_i32(memory, stackPointer + 4, input and createPointer(input) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, gameProcessedEvent and 1 or 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_user_input_service_input_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).InputChanged:Connect(function(input, gameProcessedEvent)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, input and 1 or 0);
		store_i32(memory, stackPointer + 4, input and createPointer(input) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, gameProcessedEvent and 1 or 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_user_input_service_input_ended(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).InputEnded:Connect(function(input, gameProcessedEvent)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, input and 1 or 0);
		store_i32(memory, stackPointer + 4, input and createPointer(input) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, gameProcessedEvent and 1 or 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_user_input_service_jump_request(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).JumpRequest:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_user_input_service_last_input_type_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).LastInputTypeChanged:Connect(function(lastInputType)
		invokeFunction(stack, vtable, error(lastInputType));
	end))
end
function signal.abi.connect_user_input_service_pointer_action(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).PointerAction:Connect(function(wheel, pan, pinch, gameProcessedEvent)
		invokeFunction(stack, vtable, wheel, createPointer(pan), pinch, gameProcessedEvent and 1 or 0);
	end))
end
function signal.abi.connect_user_input_service_text_box_focus_released(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TextBoxFocusReleased:Connect(function(textboxReleased)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, textboxReleased and 1 or 0);
		store_i32(memory, stackPointer + 4, textboxReleased and createPointer(textboxReleased) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_user_input_service_text_box_focused(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TextBoxFocused:Connect(function(textboxFocused)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, textboxFocused and 1 or 0);
		store_i32(memory, stackPointer + 4, textboxFocused and createPointer(textboxFocused) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_user_input_service_touch_ended(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchEnded:Connect(function(touch, gameProcessedEvent)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, touch and 1 or 0);
		store_i32(memory, stackPointer + 4, touch and createPointer(touch) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, gameProcessedEvent and 1 or 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_user_input_service_touch_long_press(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchLongPress:Connect(function(touchPositions, state, gameProcessedEvent)
		invokeFunction(stack, vtable, error(touchPositions), error(state), gameProcessedEvent and 1 or 0);
	end))
end
function signal.abi.connect_user_input_service_touch_moved(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchMoved:Connect(function(touch, gameProcessedEvent)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, touch and 1 or 0);
		store_i32(memory, stackPointer + 4, touch and createPointer(touch) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, gameProcessedEvent and 1 or 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_user_input_service_touch_pan(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchPan:Connect(function(touchPositions, totalTranslation, velocity, state, gameProcessedEvent)
		invokeFunction(stack, vtable, error(touchPositions), createPointer(totalTranslation), createPointer(velocity), error(state), gameProcessedEvent and 1 or 0);
	end))
end
function signal.abi.connect_user_input_service_touch_pinch(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchPinch:Connect(function(touchPositions, scale, velocity, state, gameProcessedEvent)
		invokeFunction(stack, vtable, error(touchPositions), scale, velocity, error(state), gameProcessedEvent and 1 or 0);
	end))
end
function signal.abi.connect_user_input_service_touch_rotate(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchRotate:Connect(function(touchPositions, rotation, velocity, state, gameProcessedEvent)
		invokeFunction(stack, vtable, error(touchPositions), rotation, velocity, error(state), gameProcessedEvent and 1 or 0);
	end))
end
function signal.abi.connect_user_input_service_touch_started(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchStarted:Connect(function(touch, gameProcessedEvent)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, touch and 1 or 0);
		store_i32(memory, stackPointer + 4, touch and createPointer(touch) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, gameProcessedEvent and 1 or 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_user_input_service_touch_swipe(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchSwipe:Connect(function(swipeDirection, numberOfTouches, gameProcessedEvent)
		invokeFunction(stack, vtable, error(swipeDirection), numberOfTouches, gameProcessedEvent and 1 or 0);
	end))
end
function signal.abi.connect_user_input_service_touch_tap(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchTap:Connect(function(touchPositions, gameProcessedEvent)
		invokeFunction(stack, vtable, error(touchPositions), gameProcessedEvent and 1 or 0);
	end))
end
function signal.abi.connect_user_input_service_touch_tap_in_world(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchTapInWorld:Connect(function(position, processedByUI)
		invokeFunction(stack, vtable, createPointer(position), processedByUI and 1 or 0);
	end))
end
function signal.abi.connect_user_input_service_user_c_frame_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).UserCFrameChanged:Connect(function(type, value)
		invokeFunction(stack, vtable, error(type), createPointer(value));
	end))
end
function signal.abi.connect_user_input_service_window_focus_released(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).WindowFocusReleased:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_user_input_service_window_focused(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).WindowFocused:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.abi.connect_vr_service_navigation_requested(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).NavigationRequested:Connect(function(cframe, inputUserCFrame)
		invokeFunction(stack, vtable, createPointer(cframe), error(inputUserCFrame));
	end))
end
function signal.abi.connect_vr_service_touchpad_mode_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).TouchpadModeChanged:Connect(function(pad, mode)
		invokeFunction(stack, vtable, error(pad), error(mode));
	end))
end
function signal.abi.connect_vr_service_user_c_frame_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).UserCFrameChanged:Connect(function(type, value)
		invokeFunction(stack, vtable, error(type), createPointer(value));
	end))
end
function signal.abi.connect_vr_service_user_c_frame_enabled(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).UserCFrameEnabled:Connect(function(type, enabled)
		invokeFunction(stack, vtable, error(type), enabled and 1 or 0);
	end))
end
function signal.abi.connect_bool_value_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, value and 1 or 0);
	end))
end
function signal.abi.connect_brick_color_value_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, createPointer(value));
	end))
end
function signal.abi.connect_c_frame_value_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, createPointer(value));
	end))
end
function signal.abi.connect_color_3_value_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, createPointer(value));
	end))
end
function signal.abi.connect_double_constrained_value_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, value);
	end))
end
function signal.abi.connect_int_constrained_value_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, value);
	end))
end
function signal.abi.connect_int_value_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, value);
	end))
end
function signal.abi.connect_number_value_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, value);
	end))
end
function signal.abi.connect_object_value_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Changed:Connect(function(value)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, value and 1 or 0);
		store_i32(memory, stackPointer + 4, value and createPointer(value) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.abi.connect_ray_value_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, createPointer(value));
	end))
end
function signal.abi.connect_string_value_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Changed:Connect(function(value)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(value));
		store_i32(memory, stackPointer + 4, #value);
		store_i32(memory, stackPointer + 8, #value);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.abi.connect_vector_3_value_changed(instance, stack, vtable)
	return createConnection(stack, vtable, getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, createPointer(value));
	end))
end
return signal;
