local memory, rt, util;
local store_i32, pushStack, popStack, invokeFunction, getPointer, createPointer, allocString;
local signal = {};
function signal.withWasm(localMemory, localRuntime, localUtil)
	memory, rt, util = localMemory, localRuntime, localUtil
	store_i32, pushStack, popStack, invokeFunction, getPointer, createPointer, allocString = rt.store.i32, util.pushStack, util.popStack, util.invokeFunction, util.getPointer, util.createPointer, util.allocString;
end
function signal.connect_instance_ancestry_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).AncestryChanged:Connect(function(child, parent)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, child and 1 or 0);
		store_i32(memory, stackPointer + 4, child and createPointer(child) or 0);
		store_i32(memory, stackPointer + 8, parent and 1 or 0);
		store_i32(memory, stackPointer + 12, parent and createPointer(parent) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.connect_instance_attribute_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).AttributeChanged:Connect(function(attribute)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(attribute));
		store_i32(memory, stackPointer + 4, #attribute);
		store_i32(memory, stackPointer + 8, #attribute);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_instance_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).Changed:Connect(function(property)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(property));
		store_i32(memory, stackPointer + 4, #property);
		store_i32(memory, stackPointer + 8, #property);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_instance_child_added(instance, stack, vtable)
	return createPointer(getPointer(instance).ChildAdded:Connect(function(child)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, child and 1 or 0);
		store_i32(memory, stackPointer + 4, child and createPointer(child) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_instance_child_removed(instance, stack, vtable)
	return createPointer(getPointer(instance).ChildRemoved:Connect(function(child)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, child and 1 or 0);
		store_i32(memory, stackPointer + 4, child and createPointer(child) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_instance_descendant_added(instance, stack, vtable)
	return createPointer(getPointer(instance).DescendantAdded:Connect(function(descendant)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, descendant and 1 or 0);
		store_i32(memory, stackPointer + 4, descendant and createPointer(descendant) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_instance_descendant_removing(instance, stack, vtable)
	return createPointer(getPointer(instance).DescendantRemoving:Connect(function(descendant)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, descendant and 1 or 0);
		store_i32(memory, stackPointer + 4, descendant and createPointer(descendant) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_instance_destroying(instance, stack, vtable)
	return createPointer(getPointer(instance).Destroying:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_animation_controller_animation_played(instance, stack, vtable)
	return createPointer(getPointer(instance).AnimationPlayed:Connect(function(animationTrack)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, animationTrack and 1 or 0);
		store_i32(memory, stackPointer + 4, animationTrack and createPointer(animationTrack) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_animation_track_did_loop(instance, stack, vtable)
	return createPointer(getPointer(instance).DidLoop:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_animation_track_keyframe_reached(instance, stack, vtable)
	return createPointer(getPointer(instance).KeyframeReached:Connect(function(keyframeName)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(keyframeName));
		store_i32(memory, stackPointer + 4, #keyframeName);
		store_i32(memory, stackPointer + 8, #keyframeName);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_animation_track_stopped(instance, stack, vtable)
	return createPointer(getPointer(instance).Stopped:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_animator_animation_played(instance, stack, vtable)
	return createPointer(getPointer(instance).AnimationPlayed:Connect(function(animationTrack)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, animationTrack and 1 or 0);
		store_i32(memory, stackPointer + 4, animationTrack and createPointer(animationTrack) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_tool_activated(instance, stack, vtable)
	return createPointer(getPointer(instance).Activated:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_tool_deactivated(instance, stack, vtable)
	return createPointer(getPointer(instance).Deactivated:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_tool_equipped(instance, stack, vtable)
	return createPointer(getPointer(instance).Equipped:Connect(function(mouse)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, mouse and 1 or 0);
		store_i32(memory, stackPointer + 4, mouse and createPointer(mouse) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_tool_unequipped(instance, stack, vtable)
	return createPointer(getPointer(instance).Unequipped:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_player_gui_topbar_transparency_changed_signal(instance, stack, vtable)
	return createPointer(getPointer(instance).TopbarTransparencyChangedSignal:Connect(function(transparency)
		invokeFunction(stack, vtable, transparency);
	end))
end
function signal.connect_bindable_event_event(instance, stack, vtable)
	return createPointer(getPointer(instance).Event:Connect(function(arguments)
		invokeFunction(stack, vtable, error(arguments));
	end))
end
function signal.connect_body_position_reached_target(instance, stack, vtable)
	return createPointer(getPointer(instance).ReachedTarget:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_rocket_propulsion_reached_target(instance, stack, vtable)
	return createPointer(getPointer(instance).ReachedTarget:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_camera_interpolation_finished(instance, stack, vtable)
	return createPointer(getPointer(instance).InterpolationFinished:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_click_detector_mouse_click(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseClick:Connect(function(playerWhoClicked)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoClicked and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoClicked and createPointer(playerWhoClicked) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_click_detector_mouse_hover_enter(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseHoverEnter:Connect(function(playerWhoHovered)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoHovered and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoHovered and createPointer(playerWhoHovered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_click_detector_mouse_hover_leave(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseHoverLeave:Connect(function(playerWhoHovered)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoHovered and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoHovered and createPointer(playerWhoHovered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_click_detector_right_mouse_click(instance, stack, vtable)
	return createPointer(getPointer(instance).RightMouseClick:Connect(function(playerWhoClicked)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoClicked and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoClicked and createPointer(playerWhoClicked) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_controller_button_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).ButtonChanged:Connect(function(button)
		invokeFunction(stack, vtable, error(button));
	end))
end
function signal.connect_skateboard_controller_axis_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).AxisChanged:Connect(function(axis)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(axis));
		store_i32(memory, stackPointer + 4, #axis);
		store_i32(memory, stackPointer + 8, #axis);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_dialog_dialog_choice_selected(instance, stack, vtable)
	return createPointer(getPointer(instance).DialogChoiceSelected:Connect(function(player, dialogChoice)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		store_i32(memory, stackPointer + 8, dialogChoice and 1 or 0);
		store_i32(memory, stackPointer + 12, dialogChoice and createPointer(dialogChoice) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.connect_explosion_hit(instance, stack, vtable)
	return createPointer(getPointer(instance).Hit:Connect(function(part, distance)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, part and 1 or 0);
		store_i32(memory, stackPointer + 4, part and createPointer(part) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, distance);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_gui_base_2_d_selection_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).SelectionChanged:Connect(function(amISelected, previousSelection, newSelection)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, previousSelection and 1 or 0);
		store_i32(memory, stackPointer + 4, previousSelection and createPointer(previousSelection) or 0);
		store_i32(memory, stackPointer + 8, newSelection and 1 or 0);
		store_i32(memory, stackPointer + 12, newSelection and createPointer(newSelection) or 0);
		invokeFunction(stack, vtable, amISelected and 1 or 0, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.connect_gui_object_drag_begin(instance, stack, vtable)
	return createPointer(getPointer(instance).DragBegin:Connect(function(initialPosition)
		invokeFunction(stack, vtable, createPointer(initialPosition));
	end))
end
function signal.connect_gui_object_drag_stopped(instance, stack, vtable)
	return createPointer(getPointer(instance).DragStopped:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.connect_gui_object_input_began(instance, stack, vtable)
	return createPointer(getPointer(instance).InputBegan:Connect(function(input)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, input and 1 or 0);
		store_i32(memory, stackPointer + 4, input and createPointer(input) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_gui_object_input_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).InputChanged:Connect(function(input)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, input and 1 or 0);
		store_i32(memory, stackPointer + 4, input and createPointer(input) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_gui_object_input_ended(instance, stack, vtable)
	return createPointer(getPointer(instance).InputEnded:Connect(function(input)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, input and 1 or 0);
		store_i32(memory, stackPointer + 4, input and createPointer(input) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_gui_object_mouse_enter(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseEnter:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.connect_gui_object_mouse_leave(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseLeave:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.connect_gui_object_mouse_moved(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseMoved:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.connect_gui_object_mouse_wheel_backward(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseWheelBackward:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.connect_gui_object_mouse_wheel_forward(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseWheelForward:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.connect_gui_object_selection_gained(instance, stack, vtable)
	return createPointer(getPointer(instance).SelectionGained:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_gui_object_selection_lost(instance, stack, vtable)
	return createPointer(getPointer(instance).SelectionLost:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_gui_object_touch_long_press(instance, stack, vtable)
	return createPointer(getPointer(instance).TouchLongPress:Connect(function(touchPositions, state)
		invokeFunction(stack, vtable, error(touchPositions), error(state));
	end))
end
function signal.connect_gui_object_touch_pan(instance, stack, vtable)
	return createPointer(getPointer(instance).TouchPan:Connect(function(touchPositions, totalTranslation, velocity, state)
		invokeFunction(stack, vtable, error(touchPositions), createPointer(totalTranslation), createPointer(velocity), error(state));
	end))
end
function signal.connect_gui_object_touch_pinch(instance, stack, vtable)
	return createPointer(getPointer(instance).TouchPinch:Connect(function(touchPositions, scale, velocity, state)
		invokeFunction(stack, vtable, error(touchPositions), scale, velocity, error(state));
	end))
end
function signal.connect_gui_object_touch_rotate(instance, stack, vtable)
	return createPointer(getPointer(instance).TouchRotate:Connect(function(touchPositions, rotation, velocity, state)
		invokeFunction(stack, vtable, error(touchPositions), rotation, velocity, error(state));
	end))
end
function signal.connect_gui_object_touch_swipe(instance, stack, vtable)
	return createPointer(getPointer(instance).TouchSwipe:Connect(function(swipeDirection, numberOfTouches)
		invokeFunction(stack, vtable, error(swipeDirection), numberOfTouches);
	end))
end
function signal.connect_gui_object_touch_tap(instance, stack, vtable)
	return createPointer(getPointer(instance).TouchTap:Connect(function(touchPositions)
		invokeFunction(stack, vtable, error(touchPositions));
	end))
end
function signal.connect_gui_button_activated(instance, stack, vtable)
	return createPointer(getPointer(instance).Activated:Connect(function(inputObject, clickCount)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, inputObject and 1 or 0);
		store_i32(memory, stackPointer + 4, inputObject and createPointer(inputObject) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, clickCount);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_gui_button_mouse_button_1_click(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseButton1Click:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_gui_button_mouse_button_1_down(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseButton1Down:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.connect_gui_button_mouse_button_1_up(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseButton1Up:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.connect_gui_button_mouse_button_2_click(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseButton2Click:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_gui_button_mouse_button_2_down(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseButton2Down:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.connect_gui_button_mouse_button_2_up(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseButton2Up:Connect(function(x, y)
		invokeFunction(stack, vtable, x, y);
	end))
end
function signal.connect_text_box_focus_lost(instance, stack, vtable)
	return createPointer(getPointer(instance).FocusLost:Connect(function(enterPressed, inputThatCausedFocusLoss)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, inputThatCausedFocusLoss and 1 or 0);
		store_i32(memory, stackPointer + 4, inputThatCausedFocusLoss and createPointer(inputThatCausedFocusLoss) or 0);
		invokeFunction(stack, vtable, enterPressed and 1 or 0, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_text_box_focused(instance, stack, vtable)
	return createPointer(getPointer(instance).Focused:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_text_box_return_pressed_from_on_screen_keyboard(instance, stack, vtable)
	return createPointer(getPointer(instance).ReturnPressedFromOnScreenKeyboard:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_video_frame_did_loop(instance, stack, vtable)
	return createPointer(getPointer(instance).DidLoop:Connect(function(video)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(video));
		store_i32(memory, stackPointer + 4, #video);
		store_i32(memory, stackPointer + 8, #video);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_video_frame_ended(instance, stack, vtable)
	return createPointer(getPointer(instance).Ended:Connect(function(video)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(video));
		store_i32(memory, stackPointer + 4, #video);
		store_i32(memory, stackPointer + 8, #video);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_video_frame_loaded(instance, stack, vtable)
	return createPointer(getPointer(instance).Loaded:Connect(function(video)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(video));
		store_i32(memory, stackPointer + 4, #video);
		store_i32(memory, stackPointer + 8, #video);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_video_frame_paused(instance, stack, vtable)
	return createPointer(getPointer(instance).Paused:Connect(function(video)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(video));
		store_i32(memory, stackPointer + 4, #video);
		store_i32(memory, stackPointer + 8, #video);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_video_frame_played(instance, stack, vtable)
	return createPointer(getPointer(instance).Played:Connect(function(video)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(video));
		store_i32(memory, stackPointer + 4, #video);
		store_i32(memory, stackPointer + 8, #video);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_handle_adornment_mouse_button_1_down(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseButton1Down:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_handle_adornment_mouse_button_1_up(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseButton1Up:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_handle_adornment_mouse_enter(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseEnter:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_handle_adornment_mouse_leave(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseLeave:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_arc_handles_mouse_button_1_down(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseButton1Down:Connect(function(axis)
		invokeFunction(stack, vtable, error(axis));
	end))
end
function signal.connect_arc_handles_mouse_button_1_up(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseButton1Up:Connect(function(axis)
		invokeFunction(stack, vtable, error(axis));
	end))
end
function signal.connect_arc_handles_mouse_drag(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseDrag:Connect(function(axis, relativeAngle, deltaRadius)
		invokeFunction(stack, vtable, error(axis), relativeAngle, deltaRadius);
	end))
end
function signal.connect_arc_handles_mouse_enter(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseEnter:Connect(function(axis)
		invokeFunction(stack, vtable, error(axis));
	end))
end
function signal.connect_arc_handles_mouse_leave(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseLeave:Connect(function(axis)
		invokeFunction(stack, vtable, error(axis));
	end))
end
function signal.connect_handles_mouse_button_1_down(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseButton1Down:Connect(function(face)
		invokeFunction(stack, vtable, error(face));
	end))
end
function signal.connect_handles_mouse_button_1_up(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseButton1Up:Connect(function(face)
		invokeFunction(stack, vtable, error(face));
	end))
end
function signal.connect_handles_mouse_drag(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseDrag:Connect(function(face, distance)
		invokeFunction(stack, vtable, error(face), distance);
	end))
end
function signal.connect_handles_mouse_enter(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseEnter:Connect(function(face)
		invokeFunction(stack, vtable, error(face));
	end))
end
function signal.connect_handles_mouse_leave(instance, stack, vtable)
	return createPointer(getPointer(instance).MouseLeave:Connect(function(face)
		invokeFunction(stack, vtable, error(face));
	end))
end
function signal.connect_humanoid_animation_played(instance, stack, vtable)
	return createPointer(getPointer(instance).AnimationPlayed:Connect(function(animationTrack)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, animationTrack and 1 or 0);
		store_i32(memory, stackPointer + 4, animationTrack and createPointer(animationTrack) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_humanoid_climbing(instance, stack, vtable)
	return createPointer(getPointer(instance).Climbing:Connect(function(speed)
		invokeFunction(stack, vtable, speed);
	end))
end
function signal.connect_humanoid_custom_status_added(instance, stack, vtable)
	return createPointer(getPointer(instance).CustomStatusAdded:Connect(function(status)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(status));
		store_i32(memory, stackPointer + 4, #status);
		store_i32(memory, stackPointer + 8, #status);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_humanoid_custom_status_removed(instance, stack, vtable)
	return createPointer(getPointer(instance).CustomStatusRemoved:Connect(function(status)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(status));
		store_i32(memory, stackPointer + 4, #status);
		store_i32(memory, stackPointer + 8, #status);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_humanoid_died(instance, stack, vtable)
	return createPointer(getPointer(instance).Died:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_humanoid_falling_down(instance, stack, vtable)
	return createPointer(getPointer(instance).FallingDown:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.connect_humanoid_free_falling(instance, stack, vtable)
	return createPointer(getPointer(instance).FreeFalling:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.connect_humanoid_getting_up(instance, stack, vtable)
	return createPointer(getPointer(instance).GettingUp:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.connect_humanoid_health_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).HealthChanged:Connect(function(health)
		invokeFunction(stack, vtable, health);
	end))
end
function signal.connect_humanoid_jumping(instance, stack, vtable)
	return createPointer(getPointer(instance).Jumping:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.connect_humanoid_move_to_finished(instance, stack, vtable)
	return createPointer(getPointer(instance).MoveToFinished:Connect(function(reached)
		invokeFunction(stack, vtable, reached and 1 or 0);
	end))
end
function signal.connect_humanoid_platform_standing(instance, stack, vtable)
	return createPointer(getPointer(instance).PlatformStanding:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.connect_humanoid_ragdoll(instance, stack, vtable)
	return createPointer(getPointer(instance).Ragdoll:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.connect_humanoid_running(instance, stack, vtable)
	return createPointer(getPointer(instance).Running:Connect(function(speed)
		invokeFunction(stack, vtable, speed);
	end))
end
function signal.connect_humanoid_seated(instance, stack, vtable)
	return createPointer(getPointer(instance).Seated:Connect(function(active, currentSeatPart)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, currentSeatPart and 1 or 0);
		store_i32(memory, stackPointer + 4, currentSeatPart and createPointer(currentSeatPart) or 0);
		invokeFunction(stack, vtable, active and 1 or 0, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_humanoid_state_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).StateChanged:Connect(function(old, new)
		invokeFunction(stack, vtable, error(old), error(new));
	end))
end
function signal.connect_humanoid_state_enabled_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).StateEnabledChanged:Connect(function(state, isEnabled)
		invokeFunction(stack, vtable, error(state), isEnabled and 1 or 0);
	end))
end
function signal.connect_humanoid_status_added(instance, stack, vtable)
	return createPointer(getPointer(instance).StatusAdded:Connect(function(status)
		invokeFunction(stack, vtable, error(status));
	end))
end
function signal.connect_humanoid_status_removed(instance, stack, vtable)
	return createPointer(getPointer(instance).StatusRemoved:Connect(function(status)
		invokeFunction(stack, vtable, error(status));
	end))
end
function signal.connect_humanoid_strafing(instance, stack, vtable)
	return createPointer(getPointer(instance).Strafing:Connect(function(active)
		invokeFunction(stack, vtable, active and 1 or 0);
	end))
end
function signal.connect_humanoid_swimming(instance, stack, vtable)
	return createPointer(getPointer(instance).Swimming:Connect(function(speed)
		invokeFunction(stack, vtable, speed);
	end))
end
function signal.connect_humanoid_touched(instance, stack, vtable)
	return createPointer(getPointer(instance).Touched:Connect(function(touchingPart, humanoidPart)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, touchingPart and 1 or 0);
		store_i32(memory, stackPointer + 4, touchingPart and createPointer(touchingPart) or 0);
		store_i32(memory, stackPointer + 8, humanoidPart and 1 or 0);
		store_i32(memory, stackPointer + 12, humanoidPart and createPointer(humanoidPart) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.connect_humanoid_description_emotes_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).EmotesChanged:Connect(function(newEmotes)
		invokeFunction(stack, vtable, error(newEmotes));
	end))
end
function signal.connect_humanoid_description_equipped_emotes_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).EquippedEmotesChanged:Connect(function(newEquippedEmotes)
		invokeFunction(stack, vtable, error(newEquippedEmotes));
	end))
end
function signal.connect_lighting_lighting_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).LightingChanged:Connect(function(skyChanged)
		invokeFunction(stack, vtable, skyChanged and 1 or 0);
	end))
end
function signal.connect_mouse_button_1_down(instance, stack, vtable)
	return createPointer(getPointer(instance).Button1Down:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_mouse_button_1_up(instance, stack, vtable)
	return createPointer(getPointer(instance).Button1Up:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_mouse_button_2_down(instance, stack, vtable)
	return createPointer(getPointer(instance).Button2Down:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_mouse_button_2_up(instance, stack, vtable)
	return createPointer(getPointer(instance).Button2Up:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_mouse_idle(instance, stack, vtable)
	return createPointer(getPointer(instance).Idle:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_mouse_key_down(instance, stack, vtable)
	return createPointer(getPointer(instance).KeyDown:Connect(function(key)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(key));
		store_i32(memory, stackPointer + 4, #key);
		store_i32(memory, stackPointer + 8, #key);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_mouse_key_up(instance, stack, vtable)
	return createPointer(getPointer(instance).KeyUp:Connect(function(key)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(key));
		store_i32(memory, stackPointer + 4, #key);
		store_i32(memory, stackPointer + 8, #key);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_mouse_move(instance, stack, vtable)
	return createPointer(getPointer(instance).Move:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_mouse_wheel_backward(instance, stack, vtable)
	return createPointer(getPointer(instance).WheelBackward:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_mouse_wheel_forward(instance, stack, vtable)
	return createPointer(getPointer(instance).WheelForward:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_network_marker_received(instance, stack, vtable)
	return createPointer(getPointer(instance).Received:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_base_part_local_simulation_touched(instance, stack, vtable)
	return createPointer(getPointer(instance).LocalSimulationTouched:Connect(function(part)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, part and 1 or 0);
		store_i32(memory, stackPointer + 4, part and createPointer(part) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_base_part_outfit_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).OutfitChanged:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_base_part_stopped_touching(instance, stack, vtable)
	return createPointer(getPointer(instance).StoppedTouching:Connect(function(otherPart)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, otherPart and 1 or 0);
		store_i32(memory, stackPointer + 4, otherPart and createPointer(otherPart) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_base_part_touch_ended(instance, stack, vtable)
	return createPointer(getPointer(instance).TouchEnded:Connect(function(otherPart)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, otherPart and 1 or 0);
		store_i32(memory, stackPointer + 4, otherPart and createPointer(otherPart) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_base_part_touched(instance, stack, vtable)
	return createPointer(getPointer(instance).Touched:Connect(function(otherPart)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, otherPart and 1 or 0);
		store_i32(memory, stackPointer + 4, otherPart and createPointer(otherPart) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_skateboard_platform_equipped(instance, stack, vtable)
	return createPointer(getPointer(instance).Equipped:Connect(function(humanoid, skateboardController)
		local stackPointer = pushStack(16);
		store_i32(memory, stackPointer + 0, humanoid and 1 or 0);
		store_i32(memory, stackPointer + 4, humanoid and createPointer(humanoid) or 0);
		store_i32(memory, stackPointer + 8, skateboardController and 1 or 0);
		store_i32(memory, stackPointer + 12, skateboardController and createPointer(skateboardController) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, stackPointer + 8);
		popStack(stackPointer, 16);
	end))
end
function signal.connect_skateboard_platform_move_state_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).MoveStateChanged:Connect(function(newState, oldState)
		invokeFunction(stack, vtable, error(newState), error(oldState));
	end))
end
function signal.connect_skateboard_platform_unequipped(instance, stack, vtable)
	return createPointer(getPointer(instance).Unequipped:Connect(function(humanoid)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, humanoid and 1 or 0);
		store_i32(memory, stackPointer + 4, humanoid and createPointer(humanoid) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_path_blocked(instance, stack, vtable)
	return createPointer(getPointer(instance).Blocked:Connect(function(blockedWaypointIdx)
		invokeFunction(stack, vtable, blockedWaypointIdx);
	end))
end
function signal.connect_path_unblocked(instance, stack, vtable)
	return createPointer(getPointer(instance).Unblocked:Connect(function(unblockedWaypointIdx)
		invokeFunction(stack, vtable, unblockedWaypointIdx);
	end))
end
function signal.connect_player_character_added(instance, stack, vtable)
	return createPointer(getPointer(instance).CharacterAdded:Connect(function(character)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, character and 1 or 0);
		store_i32(memory, stackPointer + 4, character and createPointer(character) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_player_character_appearance_loaded(instance, stack, vtable)
	return createPointer(getPointer(instance).CharacterAppearanceLoaded:Connect(function(character)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, character and 1 or 0);
		store_i32(memory, stackPointer + 4, character and createPointer(character) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_player_character_removing(instance, stack, vtable)
	return createPointer(getPointer(instance).CharacterRemoving:Connect(function(character)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, character and 1 or 0);
		store_i32(memory, stackPointer + 4, character and createPointer(character) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_player_chatted(instance, stack, vtable)
	return createPointer(getPointer(instance).Chatted:Connect(function(message, recipient)
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
function signal.connect_player_idled(instance, stack, vtable)
	return createPointer(getPointer(instance).Idled:Connect(function(time)
		invokeFunction(stack, vtable, time);
	end))
end
function signal.connect_player_on_teleport(instance, stack, vtable)
	return createPointer(getPointer(instance).OnTeleport:Connect(function(teleportState, placeId, spawnName)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(spawnName));
		store_i32(memory, stackPointer + 4, #spawnName);
		store_i32(memory, stackPointer + 8, #spawnName);
		invokeFunction(stack, vtable, error(teleportState), placeId, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_proximity_prompt_prompt_button_hold_began(instance, stack, vtable)
	return createPointer(getPointer(instance).PromptButtonHoldBegan:Connect(function(playerWhoTriggered)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoTriggered and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoTriggered and createPointer(playerWhoTriggered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_proximity_prompt_prompt_button_hold_ended(instance, stack, vtable)
	return createPointer(getPointer(instance).PromptButtonHoldEnded:Connect(function(playerWhoTriggered)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoTriggered and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoTriggered and createPointer(playerWhoTriggered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_proximity_prompt_prompt_hidden(instance, stack, vtable)
	return createPointer(getPointer(instance).PromptHidden:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_proximity_prompt_prompt_shown(instance, stack, vtable)
	return createPointer(getPointer(instance).PromptShown:Connect(function(inputType)
		invokeFunction(stack, vtable, error(inputType));
	end))
end
function signal.connect_proximity_prompt_trigger_ended(instance, stack, vtable)
	return createPointer(getPointer(instance).TriggerEnded:Connect(function(playerWhoTriggered)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoTriggered and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoTriggered and createPointer(playerWhoTriggered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_proximity_prompt_triggered(instance, stack, vtable)
	return createPointer(getPointer(instance).Triggered:Connect(function(playerWhoTriggered)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, playerWhoTriggered and 1 or 0);
		store_i32(memory, stackPointer + 4, playerWhoTriggered and createPointer(playerWhoTriggered) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_remote_event_on_client_event(instance, stack, vtable)
	return createPointer(getPointer(instance).OnClientEvent:Connect(function(arguments)
		invokeFunction(stack, vtable, error(arguments));
	end))
end
function signal.connect_remote_event_on_server_event(instance, stack, vtable)
	return createPointer(getPointer(instance).OnServerEvent:Connect(function(player, arguments)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0, error(arguments));
		popStack(stackPointer, 8);
	end))
end
function signal.connect_service_provider_close(instance, stack, vtable)
	return createPointer(getPointer(instance).Close:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_service_provider_service_added(instance, stack, vtable)
	return createPointer(getPointer(instance).ServiceAdded:Connect(function(service)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, service and 1 or 0);
		store_i32(memory, stackPointer + 4, service and createPointer(service) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_service_provider_service_removing(instance, stack, vtable)
	return createPointer(getPointer(instance).ServiceRemoving:Connect(function(service)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, service and 1 or 0);
		store_i32(memory, stackPointer + 4, service and createPointer(service) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_data_model_allowed_gear_type_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).AllowedGearTypeChanged:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_data_model_graphics_quality_change_request(instance, stack, vtable)
	return createPointer(getPointer(instance).GraphicsQualityChangeRequest:Connect(function(betterQuality)
		invokeFunction(stack, vtable, betterQuality and 1 or 0);
	end))
end
function signal.connect_data_model_item_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).ItemChanged:Connect(function(object, descriptor)
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
function signal.connect_data_model_loaded(instance, stack, vtable)
	return createPointer(getPointer(instance).Loaded:Connect(function()
		invokeFunction(stack, vtable);
	end))
end
function signal.connect_sound_did_loop(instance, stack, vtable)
	return createPointer(getPointer(instance).DidLoop:Connect(function(soundId, numOfTimesLooped)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0, numOfTimesLooped);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_sound_ended(instance, stack, vtable)
	return createPointer(getPointer(instance).Ended:Connect(function(soundId)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_sound_loaded(instance, stack, vtable)
	return createPointer(getPointer(instance).Loaded:Connect(function(soundId)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_sound_paused(instance, stack, vtable)
	return createPointer(getPointer(instance).Paused:Connect(function(soundId)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_sound_played(instance, stack, vtable)
	return createPointer(getPointer(instance).Played:Connect(function(soundId)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_sound_resumed(instance, stack, vtable)
	return createPointer(getPointer(instance).Resumed:Connect(function(soundId)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_sound_stopped(instance, stack, vtable)
	return createPointer(getPointer(instance).Stopped:Connect(function(soundId)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(soundId));
		store_i32(memory, stackPointer + 4, #soundId);
		store_i32(memory, stackPointer + 8, #soundId);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_team_player_added(instance, stack, vtable)
	return createPointer(getPointer(instance).PlayerAdded:Connect(function(player)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_team_player_removed(instance, stack, vtable)
	return createPointer(getPointer(instance).PlayerRemoved:Connect(function(player)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, player and 1 or 0);
		store_i32(memory, stackPointer + 4, player and createPointer(player) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_text_channel_message_received(instance, stack, vtable)
	return createPointer(getPointer(instance).MessageReceived:Connect(function(incomingMessage)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, incomingMessage and 1 or 0);
		store_i32(memory, stackPointer + 4, incomingMessage and createPointer(incomingMessage) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_text_chat_command_triggered(instance, stack, vtable)
	return createPointer(getPointer(instance).Triggered:Connect(function(originTextSource, unfilteredText)
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
function signal.connect_tween_base_completed(instance, stack, vtable)
	return createPointer(getPointer(instance).Completed:Connect(function(playbackState)
		invokeFunction(stack, vtable, error(playbackState));
	end))
end
function signal.connect_ui_page_layout_page_enter(instance, stack, vtable)
	return createPointer(getPointer(instance).PageEnter:Connect(function(page)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, page and 1 or 0);
		store_i32(memory, stackPointer + 4, page and createPointer(page) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_ui_page_layout_page_leave(instance, stack, vtable)
	return createPointer(getPointer(instance).PageLeave:Connect(function(page)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, page and 1 or 0);
		store_i32(memory, stackPointer + 4, page and createPointer(page) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_ui_page_layout_stopped(instance, stack, vtable)
	return createPointer(getPointer(instance).Stopped:Connect(function(currentPage)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, currentPage and 1 or 0);
		store_i32(memory, stackPointer + 4, currentPage and createPointer(currentPage) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_user_game_settings_fullscreen_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).FullscreenChanged:Connect(function(isFullscreen)
		invokeFunction(stack, vtable, isFullscreen and 1 or 0);
	end))
end
function signal.connect_user_game_settings_studio_mode_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).StudioModeChanged:Connect(function(isStudioMode)
		invokeFunction(stack, vtable, isStudioMode and 1 or 0);
	end))
end
function signal.connect_bool_value_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, value and 1 or 0);
	end))
end
function signal.connect_brick_color_value_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, createPointer(value));
	end))
end
function signal.connect_c_frame_value_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, createPointer(value));
	end))
end
function signal.connect_color_3_value_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, createPointer(value));
	end))
end
function signal.connect_double_constrained_value_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, value);
	end))
end
function signal.connect_int_constrained_value_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, value);
	end))
end
function signal.connect_int_value_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, value);
	end))
end
function signal.connect_number_value_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, value);
	end))
end
function signal.connect_object_value_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).Changed:Connect(function(value)
		local stackPointer = pushStack(8);
		store_i32(memory, stackPointer + 0, value and 1 or 0);
		store_i32(memory, stackPointer + 4, value and createPointer(value) or 0);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 8);
	end))
end
function signal.connect_ray_value_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, createPointer(value));
	end))
end
function signal.connect_string_value_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).Changed:Connect(function(value)
		local stackPointer = pushStack(12);
		store_i32(memory, stackPointer + 0, allocString(value));
		store_i32(memory, stackPointer + 4, #value);
		store_i32(memory, stackPointer + 8, #value);
		invokeFunction(stack, vtable, stackPointer + 0);
		popStack(stackPointer, 12);
	end))
end
function signal.connect_vector_3_value_changed(instance, stack, vtable)
	return createPointer(getPointer(instance).Changed:Connect(function(value)
		invokeFunction(stack, vtable, createPointer(value));
	end))
end
return signal;
