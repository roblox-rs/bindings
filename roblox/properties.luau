local memory, rt, util;
local writeStringReturn, getPointer, createPointer, store_i32, readString;
local property = { abi = {} };
function property.withWasm(localMemory, localRuntime, localUtil)
	memory, rt, util = localMemory, localRuntime, localUtil;
	writeStringReturn, getPointer, createPointer, store_i32, readString = util.writeStringReturn, util.getPointer, util.createPointer, rt.store.i32, util.readString;
end
function property.abi.prop_instance_archivable(instance)
	return getPointer(instance).Archivable and 1 or 0;
end
function property.abi.prop_set_instance_archivable(instance, bool)
	getPointer(instance).Archivable = bool and 1 or 0;
end
function property.abi.prop_instance_class_name(addr, instance)
	writeStringReturn(addr, getPointer(instance).ClassName);
end
function property.abi.prop_instance_name(addr, instance)
	writeStringReturn(addr, getPointer(instance).Name);
end
function property.abi.prop_set_instance_name(instance, addr, length)
	getPointer(instance).Name = readString(addr, length);
end
function property.abi.prop_instance_parent(addr, instance)
	local value = getPointer(instance).Parent;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_instance_parent(instance, isSome, value)
	getPointer(instance).Parent = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_accoutrement_attachment_forward(instance)
	return createPointer(getPointer(instance).AttachmentForward);
end
function property.abi.prop_set_accoutrement_attachment_forward(instance, vector_3)
	getPointer(instance).AttachmentForward = getPointer(vector_3);
end
function property.abi.prop_accoutrement_attachment_point(instance)
	return createPointer(getPointer(instance).AttachmentPoint);
end
function property.abi.prop_set_accoutrement_attachment_point(instance, c_frame)
	getPointer(instance).AttachmentPoint = getPointer(c_frame);
end
function property.abi.prop_accoutrement_attachment_pos(instance)
	return createPointer(getPointer(instance).AttachmentPos);
end
function property.abi.prop_set_accoutrement_attachment_pos(instance, vector_3)
	getPointer(instance).AttachmentPos = getPointer(vector_3);
end
function property.abi.prop_accoutrement_attachment_right(instance)
	return createPointer(getPointer(instance).AttachmentRight);
end
function property.abi.prop_set_accoutrement_attachment_right(instance, vector_3)
	getPointer(instance).AttachmentRight = getPointer(vector_3);
end
function property.abi.prop_accoutrement_attachment_up(instance)
	return createPointer(getPointer(instance).AttachmentUp);
end
function property.abi.prop_set_accoutrement_attachment_up(instance, vector_3)
	getPointer(instance).AttachmentUp = getPointer(vector_3);
end
function property.abi.prop_accessory_accessory_type(instance)
end
function property.abi.prop_animation_animation_id(instance)
	return createPointer(getPointer(instance).AnimationId);
end
function property.abi.prop_set_animation_animation_id(instance, content)
	getPointer(instance).AnimationId = getPointer(content);
end
function property.abi.prop_animation_clip_loop(instance)
	return getPointer(instance).Loop and 1 or 0;
end
function property.abi.prop_set_animation_clip_loop(instance, bool)
	getPointer(instance).Loop = bool and 1 or 0;
end
function property.abi.prop_animation_clip_priority(instance)
end
function property.abi.prop_animation_stream_track_is_playing(instance)
	return getPointer(instance).IsPlaying and 1 or 0;
end
function property.abi.prop_animation_stream_track_priority(instance)
end
function property.abi.prop_animation_stream_track_weight_current(instance)
	return getPointer(instance).WeightCurrent;
end
function property.abi.prop_animation_stream_track_weight_target(instance)
	return getPointer(instance).WeightTarget;
end
function property.abi.prop_animation_track_animation(addr, instance)
	local value = getPointer(instance).Animation;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_animation_track_is_playing(instance)
	return getPointer(instance).IsPlaying and 1 or 0;
end
function property.abi.prop_animation_track_length(instance)
	return getPointer(instance).Length;
end
function property.abi.prop_animation_track_looped(instance)
	return getPointer(instance).Looped and 1 or 0;
end
function property.abi.prop_set_animation_track_looped(instance, bool)
	getPointer(instance).Looped = bool and 1 or 0;
end
function property.abi.prop_animation_track_priority(instance)
end
function property.abi.prop_animation_track_speed(instance)
	return getPointer(instance).Speed;
end
function property.abi.prop_animation_track_time_position(instance)
	return getPointer(instance).TimePosition;
end
function property.abi.prop_set_animation_track_time_position(instance, number)
	getPointer(instance).TimePosition = number;
end
function property.abi.prop_animation_track_weight_current(instance)
	return getPointer(instance).WeightCurrent;
end
function property.abi.prop_animation_track_weight_target(instance)
	return getPointer(instance).WeightTarget;
end
function property.abi.prop_atmosphere_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_atmosphere_color(instance, color_3)
	getPointer(instance).Color = getPointer(color_3);
end
function property.abi.prop_atmosphere_decay(instance)
	return createPointer(getPointer(instance).Decay);
end
function property.abi.prop_set_atmosphere_decay(instance, color_3)
	getPointer(instance).Decay = getPointer(color_3);
end
function property.abi.prop_atmosphere_density(instance)
	return getPointer(instance).Density;
end
function property.abi.prop_set_atmosphere_density(instance, number)
	getPointer(instance).Density = number;
end
function property.abi.prop_atmosphere_glare(instance)
	return getPointer(instance).Glare;
end
function property.abi.prop_set_atmosphere_glare(instance, number)
	getPointer(instance).Glare = number;
end
function property.abi.prop_atmosphere_haze(instance)
	return getPointer(instance).Haze;
end
function property.abi.prop_set_atmosphere_haze(instance, number)
	getPointer(instance).Haze = number;
end
function property.abi.prop_atmosphere_offset(instance)
	return getPointer(instance).Offset;
end
function property.abi.prop_set_atmosphere_offset(instance, number)
	getPointer(instance).Offset = number;
end
function property.abi.prop_attachment_axis(instance)
	return createPointer(getPointer(instance).Axis);
end
function property.abi.prop_set_attachment_axis(instance, vector_3)
	getPointer(instance).Axis = getPointer(vector_3);
end
function property.abi.prop_attachment_c_frame(instance)
	return createPointer(getPointer(instance).CFrame);
end
function property.abi.prop_set_attachment_c_frame(instance, c_frame)
	getPointer(instance).CFrame = getPointer(c_frame);
end
function property.abi.prop_attachment_orientation(instance)
	return createPointer(getPointer(instance).Orientation);
end
function property.abi.prop_set_attachment_orientation(instance, vector_3)
	getPointer(instance).Orientation = getPointer(vector_3);
end
function property.abi.prop_attachment_position(instance)
	return createPointer(getPointer(instance).Position);
end
function property.abi.prop_set_attachment_position(instance, vector_3)
	getPointer(instance).Position = getPointer(vector_3);
end
function property.abi.prop_attachment_rotation(instance)
	return createPointer(getPointer(instance).Rotation);
end
function property.abi.prop_set_attachment_rotation(instance, vector_3)
	getPointer(instance).Rotation = getPointer(vector_3);
end
function property.abi.prop_attachment_secondary_axis(instance)
	return createPointer(getPointer(instance).SecondaryAxis);
end
function property.abi.prop_set_attachment_secondary_axis(instance, vector_3)
	getPointer(instance).SecondaryAxis = getPointer(vector_3);
end
function property.abi.prop_attachment_visible(instance)
	return getPointer(instance).Visible and 1 or 0;
end
function property.abi.prop_set_attachment_visible(instance, bool)
	getPointer(instance).Visible = bool and 1 or 0;
end
function property.abi.prop_attachment_world_axis(instance)
	return createPointer(getPointer(instance).WorldAxis);
end
function property.abi.prop_set_attachment_world_axis(instance, vector_3)
	getPointer(instance).WorldAxis = getPointer(vector_3);
end
function property.abi.prop_attachment_world_c_frame(instance)
	return createPointer(getPointer(instance).WorldCFrame);
end
function property.abi.prop_set_attachment_world_c_frame(instance, c_frame)
	getPointer(instance).WorldCFrame = getPointer(c_frame);
end
function property.abi.prop_attachment_world_orientation(instance)
	return createPointer(getPointer(instance).WorldOrientation);
end
function property.abi.prop_set_attachment_world_orientation(instance, vector_3)
	getPointer(instance).WorldOrientation = getPointer(vector_3);
end
function property.abi.prop_attachment_world_position(instance)
	return createPointer(getPointer(instance).WorldPosition);
end
function property.abi.prop_set_attachment_world_position(instance, vector_3)
	getPointer(instance).WorldPosition = getPointer(vector_3);
end
function property.abi.prop_attachment_world_rotation(instance)
	return createPointer(getPointer(instance).WorldRotation);
end
function property.abi.prop_attachment_world_secondary_axis(instance)
	return createPointer(getPointer(instance).WorldSecondaryAxis);
end
function property.abi.prop_set_attachment_world_secondary_axis(instance, vector_3)
	getPointer(instance).WorldSecondaryAxis = getPointer(vector_3);
end
function property.abi.prop_bone_transform(instance)
	return createPointer(getPointer(instance).Transform);
end
function property.abi.prop_set_bone_transform(instance, c_frame)
	getPointer(instance).Transform = getPointer(c_frame);
end
function property.abi.prop_bone_transformed_c_frame(instance)
	return createPointer(getPointer(instance).TransformedCFrame);
end
function property.abi.prop_bone_transformed_world_c_frame(instance)
	return createPointer(getPointer(instance).TransformedWorldCFrame);
end
function property.abi.prop_backpack_item_texture_id(instance)
	return createPointer(getPointer(instance).TextureId);
end
function property.abi.prop_set_backpack_item_texture_id(instance, content)
	getPointer(instance).TextureId = getPointer(content);
end
function property.abi.prop_tool_can_be_dropped(instance)
	return getPointer(instance).CanBeDropped and 1 or 0;
end
function property.abi.prop_set_tool_can_be_dropped(instance, bool)
	getPointer(instance).CanBeDropped = bool and 1 or 0;
end
function property.abi.prop_tool_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_tool_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_tool_grip(instance)
	return createPointer(getPointer(instance).Grip);
end
function property.abi.prop_set_tool_grip(instance, c_frame)
	getPointer(instance).Grip = getPointer(c_frame);
end
function property.abi.prop_tool_grip_forward(instance)
	return createPointer(getPointer(instance).GripForward);
end
function property.abi.prop_set_tool_grip_forward(instance, vector_3)
	getPointer(instance).GripForward = getPointer(vector_3);
end
function property.abi.prop_tool_grip_pos(instance)
	return createPointer(getPointer(instance).GripPos);
end
function property.abi.prop_set_tool_grip_pos(instance, vector_3)
	getPointer(instance).GripPos = getPointer(vector_3);
end
function property.abi.prop_tool_grip_right(instance)
	return createPointer(getPointer(instance).GripRight);
end
function property.abi.prop_set_tool_grip_right(instance, vector_3)
	getPointer(instance).GripRight = getPointer(vector_3);
end
function property.abi.prop_tool_grip_up(instance)
	return createPointer(getPointer(instance).GripUp);
end
function property.abi.prop_set_tool_grip_up(instance, vector_3)
	getPointer(instance).GripUp = getPointer(vector_3);
end
function property.abi.prop_tool_manual_activation_only(instance)
	return getPointer(instance).ManualActivationOnly and 1 or 0;
end
function property.abi.prop_set_tool_manual_activation_only(instance, bool)
	getPointer(instance).ManualActivationOnly = bool and 1 or 0;
end
function property.abi.prop_tool_requires_handle(instance)
	return getPointer(instance).RequiresHandle and 1 or 0;
end
function property.abi.prop_set_tool_requires_handle(instance, bool)
	getPointer(instance).RequiresHandle = bool and 1 or 0;
end
function property.abi.prop_tool_tool_tip(addr, instance)
	writeStringReturn(addr, getPointer(instance).ToolTip);
end
function property.abi.prop_set_tool_tool_tip(instance, addr, length)
	getPointer(instance).ToolTip = readString(addr, length);
end
function property.abi.prop_player_gui_current_screen_orientation(instance)
end
function property.abi.prop_player_gui_screen_orientation(instance)
end
function property.abi.prop_player_gui_selection_image_object(addr, instance)
	local value = getPointer(instance).SelectionImageObject;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_player_gui_selection_image_object(instance, isSome, value)
	getPointer(instance).SelectionImageObject = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_starter_gui_reset_player_gui_on_spawn(instance)
	return getPointer(instance).ResetPlayerGuiOnSpawn and 1 or 0;
end
function property.abi.prop_set_starter_gui_reset_player_gui_on_spawn(instance, bool)
	getPointer(instance).ResetPlayerGuiOnSpawn = bool and 1 or 0;
end
function property.abi.prop_starter_gui_screen_orientation(instance)
end
function property.abi.prop_starter_gui_show_development_gui(instance)
	return getPointer(instance).ShowDevelopmentGui and 1 or 0;
end
function property.abi.prop_set_starter_gui_show_development_gui(instance, bool)
	getPointer(instance).ShowDevelopmentGui = bool and 1 or 0;
end
function property.abi.prop_starter_gui_virtual_cursor_mode(instance)
end
function property.abi.prop_base_wrap_cage_mesh_id(instance)
	return createPointer(getPointer(instance).CageMeshId);
end
function property.abi.prop_base_wrap_cage_origin(instance)
	return createPointer(getPointer(instance).CageOrigin);
end
function property.abi.prop_base_wrap_cage_origin_world(instance)
	return createPointer(getPointer(instance).CageOriginWorld);
end
function property.abi.prop_base_wrap_import_origin(instance)
	return createPointer(getPointer(instance).ImportOrigin);
end
function property.abi.prop_base_wrap_import_origin_world(instance)
	return createPointer(getPointer(instance).ImportOriginWorld);
end
function property.abi.prop_wrap_layer_auto_skin(instance)
end
function property.abi.prop_wrap_layer_bind_offset(instance)
	return createPointer(getPointer(instance).BindOffset);
end
function property.abi.prop_wrap_layer_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_wrap_layer_color(instance, color_3)
	getPointer(instance).Color = getPointer(color_3);
end
function property.abi.prop_wrap_layer_debug_mode(instance)
end
function property.abi.prop_wrap_layer_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_wrap_layer_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_wrap_layer_order(instance)
	return getPointer(instance).Order;
end
function property.abi.prop_set_wrap_layer_order(instance, number)
	getPointer(instance).Order = number;
end
function property.abi.prop_wrap_layer_puffiness(instance)
	return getPointer(instance).Puffiness;
end
function property.abi.prop_set_wrap_layer_puffiness(instance, number)
	getPointer(instance).Puffiness = number;
end
function property.abi.prop_wrap_layer_reference_mesh_id(instance)
	return createPointer(getPointer(instance).ReferenceMeshId);
end
function property.abi.prop_wrap_layer_reference_origin(instance)
	return createPointer(getPointer(instance).ReferenceOrigin);
end
function property.abi.prop_wrap_layer_reference_origin_world(instance)
	return createPointer(getPointer(instance).ReferenceOriginWorld);
end
function property.abi.prop_wrap_layer_shrink_factor(instance)
	return getPointer(instance).ShrinkFactor;
end
function property.abi.prop_wrap_target_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_wrap_target_color(instance, color_3)
	getPointer(instance).Color = getPointer(color_3);
end
function property.abi.prop_wrap_target_debug_mode(instance)
end
function property.abi.prop_wrap_target_stiffness(instance)
	return getPointer(instance).Stiffness;
end
function property.abi.prop_beam_attachment_0(addr, instance)
	local value = getPointer(instance).Attachment0;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_beam_attachment_0(instance, isSome, value)
	getPointer(instance).Attachment0 = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_beam_attachment_1(addr, instance)
	local value = getPointer(instance).Attachment1;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_beam_attachment_1(instance, isSome, value)
	getPointer(instance).Attachment1 = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_beam_brightness(instance)
	return getPointer(instance).Brightness;
end
function property.abi.prop_set_beam_brightness(instance, number)
	getPointer(instance).Brightness = number;
end
function property.abi.prop_beam_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_beam_color(instance, color_sequence)
	getPointer(instance).Color = getPointer(color_sequence);
end
function property.abi.prop_beam_curve_size_0(instance)
	return getPointer(instance).CurveSize0;
end
function property.abi.prop_set_beam_curve_size_0(instance, number)
	getPointer(instance).CurveSize0 = number;
end
function property.abi.prop_beam_curve_size_1(instance)
	return getPointer(instance).CurveSize1;
end
function property.abi.prop_set_beam_curve_size_1(instance, number)
	getPointer(instance).CurveSize1 = number;
end
function property.abi.prop_beam_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_beam_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_beam_face_camera(instance)
	return getPointer(instance).FaceCamera and 1 or 0;
end
function property.abi.prop_set_beam_face_camera(instance, bool)
	getPointer(instance).FaceCamera = bool and 1 or 0;
end
function property.abi.prop_beam_light_emission(instance)
	return getPointer(instance).LightEmission;
end
function property.abi.prop_set_beam_light_emission(instance, number)
	getPointer(instance).LightEmission = number;
end
function property.abi.prop_beam_light_influence(instance)
	return getPointer(instance).LightInfluence;
end
function property.abi.prop_set_beam_light_influence(instance, number)
	getPointer(instance).LightInfluence = number;
end
function property.abi.prop_beam_segments(instance)
	return getPointer(instance).Segments;
end
function property.abi.prop_set_beam_segments(instance, number)
	getPointer(instance).Segments = number;
end
function property.abi.prop_beam_texture(instance)
	return createPointer(getPointer(instance).Texture);
end
function property.abi.prop_set_beam_texture(instance, content)
	getPointer(instance).Texture = getPointer(content);
end
function property.abi.prop_beam_texture_length(instance)
	return getPointer(instance).TextureLength;
end
function property.abi.prop_set_beam_texture_length(instance, number)
	getPointer(instance).TextureLength = number;
end
function property.abi.prop_beam_texture_mode(instance)
end
function property.abi.prop_beam_texture_speed(instance)
	return getPointer(instance).TextureSpeed;
end
function property.abi.prop_set_beam_texture_speed(instance, number)
	getPointer(instance).TextureSpeed = number;
end
function property.abi.prop_beam_transparency(instance)
	return createPointer(getPointer(instance).Transparency);
end
function property.abi.prop_set_beam_transparency(instance, number_sequence)
	getPointer(instance).Transparency = getPointer(number_sequence);
end
function property.abi.prop_beam_width_0(instance)
	return getPointer(instance).Width0;
end
function property.abi.prop_set_beam_width_0(instance, number)
	getPointer(instance).Width0 = number;
end
function property.abi.prop_beam_width_1(instance)
	return getPointer(instance).Width1;
end
function property.abi.prop_set_beam_width_1(instance, number)
	getPointer(instance).Width1 = number;
end
function property.abi.prop_beam_z_offset(instance)
	return getPointer(instance).ZOffset;
end
function property.abi.prop_set_beam_z_offset(instance, number)
	getPointer(instance).ZOffset = number;
end
function property.abi.prop_body_angular_velocity_angular_velocity(instance)
	return createPointer(getPointer(instance).AngularVelocity);
end
function property.abi.prop_set_body_angular_velocity_angular_velocity(instance, vector_3)
	getPointer(instance).AngularVelocity = getPointer(vector_3);
end
function property.abi.prop_body_angular_velocity_max_torque(instance)
	return createPointer(getPointer(instance).MaxTorque);
end
function property.abi.prop_set_body_angular_velocity_max_torque(instance, vector_3)
	getPointer(instance).MaxTorque = getPointer(vector_3);
end
function property.abi.prop_body_angular_velocity_p(instance)
	return getPointer(instance).P;
end
function property.abi.prop_set_body_angular_velocity_p(instance, number)
	getPointer(instance).P = number;
end
function property.abi.prop_body_force_force(instance)
	return createPointer(getPointer(instance).Force);
end
function property.abi.prop_set_body_force_force(instance, vector_3)
	getPointer(instance).Force = getPointer(vector_3);
end
function property.abi.prop_body_gyro_c_frame(instance)
	return createPointer(getPointer(instance).CFrame);
end
function property.abi.prop_set_body_gyro_c_frame(instance, c_frame)
	getPointer(instance).CFrame = getPointer(c_frame);
end
function property.abi.prop_body_gyro_d(instance)
	return getPointer(instance).D;
end
function property.abi.prop_set_body_gyro_d(instance, number)
	getPointer(instance).D = number;
end
function property.abi.prop_body_gyro_max_torque(instance)
	return createPointer(getPointer(instance).MaxTorque);
end
function property.abi.prop_set_body_gyro_max_torque(instance, vector_3)
	getPointer(instance).MaxTorque = getPointer(vector_3);
end
function property.abi.prop_body_gyro_p(instance)
	return getPointer(instance).P;
end
function property.abi.prop_set_body_gyro_p(instance, number)
	getPointer(instance).P = number;
end
function property.abi.prop_body_position_d(instance)
	return getPointer(instance).D;
end
function property.abi.prop_set_body_position_d(instance, number)
	getPointer(instance).D = number;
end
function property.abi.prop_body_position_max_force(instance)
	return createPointer(getPointer(instance).MaxForce);
end
function property.abi.prop_set_body_position_max_force(instance, vector_3)
	getPointer(instance).MaxForce = getPointer(vector_3);
end
function property.abi.prop_body_position_p(instance)
	return getPointer(instance).P;
end
function property.abi.prop_set_body_position_p(instance, number)
	getPointer(instance).P = number;
end
function property.abi.prop_body_position_position(instance)
	return createPointer(getPointer(instance).Position);
end
function property.abi.prop_set_body_position_position(instance, vector_3)
	getPointer(instance).Position = getPointer(vector_3);
end
function property.abi.prop_body_thrust_force(instance)
	return createPointer(getPointer(instance).Force);
end
function property.abi.prop_set_body_thrust_force(instance, vector_3)
	getPointer(instance).Force = getPointer(vector_3);
end
function property.abi.prop_body_thrust_location(instance)
	return createPointer(getPointer(instance).Location);
end
function property.abi.prop_set_body_thrust_location(instance, vector_3)
	getPointer(instance).Location = getPointer(vector_3);
end
function property.abi.prop_body_velocity_max_force(instance)
	return createPointer(getPointer(instance).MaxForce);
end
function property.abi.prop_set_body_velocity_max_force(instance, vector_3)
	getPointer(instance).MaxForce = getPointer(vector_3);
end
function property.abi.prop_body_velocity_p(instance)
	return getPointer(instance).P;
end
function property.abi.prop_set_body_velocity_p(instance, number)
	getPointer(instance).P = number;
end
function property.abi.prop_body_velocity_velocity(instance)
	return createPointer(getPointer(instance).Velocity);
end
function property.abi.prop_set_body_velocity_velocity(instance, vector_3)
	getPointer(instance).Velocity = getPointer(vector_3);
end
function property.abi.prop_rocket_propulsion_cartoon_factor(instance)
	return getPointer(instance).CartoonFactor;
end
function property.abi.prop_set_rocket_propulsion_cartoon_factor(instance, number)
	getPointer(instance).CartoonFactor = number;
end
function property.abi.prop_rocket_propulsion_max_speed(instance)
	return getPointer(instance).MaxSpeed;
end
function property.abi.prop_set_rocket_propulsion_max_speed(instance, number)
	getPointer(instance).MaxSpeed = number;
end
function property.abi.prop_rocket_propulsion_max_thrust(instance)
	return getPointer(instance).MaxThrust;
end
function property.abi.prop_set_rocket_propulsion_max_thrust(instance, number)
	getPointer(instance).MaxThrust = number;
end
function property.abi.prop_rocket_propulsion_max_torque(instance)
	return createPointer(getPointer(instance).MaxTorque);
end
function property.abi.prop_set_rocket_propulsion_max_torque(instance, vector_3)
	getPointer(instance).MaxTorque = getPointer(vector_3);
end
function property.abi.prop_rocket_propulsion_target(addr, instance)
	local value = getPointer(instance).Target;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_rocket_propulsion_target(instance, isSome, value)
	getPointer(instance).Target = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_rocket_propulsion_target_offset(instance)
	return createPointer(getPointer(instance).TargetOffset);
end
function property.abi.prop_set_rocket_propulsion_target_offset(instance, vector_3)
	getPointer(instance).TargetOffset = getPointer(vector_3);
end
function property.abi.prop_rocket_propulsion_target_radius(instance)
	return getPointer(instance).TargetRadius;
end
function property.abi.prop_set_rocket_propulsion_target_radius(instance, number)
	getPointer(instance).TargetRadius = number;
end
function property.abi.prop_rocket_propulsion_thrust_d(instance)
	return getPointer(instance).ThrustD;
end
function property.abi.prop_set_rocket_propulsion_thrust_d(instance, number)
	getPointer(instance).ThrustD = number;
end
function property.abi.prop_rocket_propulsion_thrust_p(instance)
	return getPointer(instance).ThrustP;
end
function property.abi.prop_set_rocket_propulsion_thrust_p(instance, number)
	getPointer(instance).ThrustP = number;
end
function property.abi.prop_rocket_propulsion_turn_d(instance)
	return getPointer(instance).TurnD;
end
function property.abi.prop_set_rocket_propulsion_turn_d(instance, number)
	getPointer(instance).TurnD = number;
end
function property.abi.prop_rocket_propulsion_turn_p(instance)
	return getPointer(instance).TurnP;
end
function property.abi.prop_set_rocket_propulsion_turn_p(instance, number)
	getPointer(instance).TurnP = number;
end
function property.abi.prop_camera_c_frame(instance)
	return createPointer(getPointer(instance).CFrame);
end
function property.abi.prop_set_camera_c_frame(instance, c_frame)
	getPointer(instance).CFrame = getPointer(c_frame);
end
function property.abi.prop_camera_camera_subject(addr, instance)
	local value = getPointer(instance).CameraSubject;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_camera_camera_subject(instance, isSome, value)
	getPointer(instance).CameraSubject = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_camera_camera_type(instance)
end
function property.abi.prop_camera_coordinate_frame(instance)
	return createPointer(getPointer(instance).CoordinateFrame);
end
function property.abi.prop_set_camera_coordinate_frame(instance, c_frame)
	getPointer(instance).CoordinateFrame = getPointer(c_frame);
end
function property.abi.prop_camera_diagonal_field_of_view(instance)
	return getPointer(instance).DiagonalFieldOfView;
end
function property.abi.prop_set_camera_diagonal_field_of_view(instance, number)
	getPointer(instance).DiagonalFieldOfView = number;
end
function property.abi.prop_camera_field_of_view(instance)
	return getPointer(instance).FieldOfView;
end
function property.abi.prop_set_camera_field_of_view(instance, number)
	getPointer(instance).FieldOfView = number;
end
function property.abi.prop_camera_field_of_view_mode(instance)
end
function property.abi.prop_camera_focus(instance)
	return createPointer(getPointer(instance).Focus);
end
function property.abi.prop_set_camera_focus(instance, c_frame)
	getPointer(instance).Focus = getPointer(c_frame);
end
function property.abi.prop_camera_head_locked(instance)
	return getPointer(instance).HeadLocked and 1 or 0;
end
function property.abi.prop_set_camera_head_locked(instance, bool)
	getPointer(instance).HeadLocked = bool and 1 or 0;
end
function property.abi.prop_camera_head_scale(instance)
	return getPointer(instance).HeadScale;
end
function property.abi.prop_set_camera_head_scale(instance, number)
	getPointer(instance).HeadScale = number;
end
function property.abi.prop_camera_max_axis_field_of_view(instance)
	return getPointer(instance).MaxAxisFieldOfView;
end
function property.abi.prop_set_camera_max_axis_field_of_view(instance, number)
	getPointer(instance).MaxAxisFieldOfView = number;
end
function property.abi.prop_camera_near_plane_z(instance)
	return getPointer(instance).NearPlaneZ;
end
function property.abi.prop_camera_viewport_size(instance)
	return createPointer(getPointer(instance).ViewportSize);
end
function property.abi.prop_body_colors_head_color(instance)
	return createPointer(getPointer(instance).HeadColor);
end
function property.abi.prop_set_body_colors_head_color(instance, brick_color)
	getPointer(instance).HeadColor = getPointer(brick_color);
end
function property.abi.prop_body_colors_head_color_3(instance)
	return createPointer(getPointer(instance).HeadColor3);
end
function property.abi.prop_set_body_colors_head_color_3(instance, color_3)
	getPointer(instance).HeadColor3 = getPointer(color_3);
end
function property.abi.prop_body_colors_left_arm_color(instance)
	return createPointer(getPointer(instance).LeftArmColor);
end
function property.abi.prop_set_body_colors_left_arm_color(instance, brick_color)
	getPointer(instance).LeftArmColor = getPointer(brick_color);
end
function property.abi.prop_body_colors_left_arm_color_3(instance)
	return createPointer(getPointer(instance).LeftArmColor3);
end
function property.abi.prop_set_body_colors_left_arm_color_3(instance, color_3)
	getPointer(instance).LeftArmColor3 = getPointer(color_3);
end
function property.abi.prop_body_colors_left_leg_color(instance)
	return createPointer(getPointer(instance).LeftLegColor);
end
function property.abi.prop_set_body_colors_left_leg_color(instance, brick_color)
	getPointer(instance).LeftLegColor = getPointer(brick_color);
end
function property.abi.prop_body_colors_left_leg_color_3(instance)
	return createPointer(getPointer(instance).LeftLegColor3);
end
function property.abi.prop_set_body_colors_left_leg_color_3(instance, color_3)
	getPointer(instance).LeftLegColor3 = getPointer(color_3);
end
function property.abi.prop_body_colors_right_arm_color(instance)
	return createPointer(getPointer(instance).RightArmColor);
end
function property.abi.prop_set_body_colors_right_arm_color(instance, brick_color)
	getPointer(instance).RightArmColor = getPointer(brick_color);
end
function property.abi.prop_body_colors_right_arm_color_3(instance)
	return createPointer(getPointer(instance).RightArmColor3);
end
function property.abi.prop_set_body_colors_right_arm_color_3(instance, color_3)
	getPointer(instance).RightArmColor3 = getPointer(color_3);
end
function property.abi.prop_body_colors_right_leg_color(instance)
	return createPointer(getPointer(instance).RightLegColor);
end
function property.abi.prop_set_body_colors_right_leg_color(instance, brick_color)
	getPointer(instance).RightLegColor = getPointer(brick_color);
end
function property.abi.prop_body_colors_right_leg_color_3(instance)
	return createPointer(getPointer(instance).RightLegColor3);
end
function property.abi.prop_set_body_colors_right_leg_color_3(instance, color_3)
	getPointer(instance).RightLegColor3 = getPointer(color_3);
end
function property.abi.prop_body_colors_torso_color(instance)
	return createPointer(getPointer(instance).TorsoColor);
end
function property.abi.prop_set_body_colors_torso_color(instance, brick_color)
	getPointer(instance).TorsoColor = getPointer(brick_color);
end
function property.abi.prop_body_colors_torso_color_3(instance)
	return createPointer(getPointer(instance).TorsoColor3);
end
function property.abi.prop_set_body_colors_torso_color_3(instance, color_3)
	getPointer(instance).TorsoColor3 = getPointer(color_3);
end
function property.abi.prop_character_mesh_base_texture_id(instance)
	return getPointer(instance).BaseTextureId;
end
function property.abi.prop_set_character_mesh_base_texture_id(instance, number)
	getPointer(instance).BaseTextureId = number;
end
function property.abi.prop_character_mesh_body_part(instance)
end
function property.abi.prop_character_mesh_mesh_id(instance)
	return getPointer(instance).MeshId;
end
function property.abi.prop_set_character_mesh_mesh_id(instance, number)
	getPointer(instance).MeshId = number;
end
function property.abi.prop_character_mesh_overlay_texture_id(instance)
	return getPointer(instance).OverlayTextureId;
end
function property.abi.prop_set_character_mesh_overlay_texture_id(instance, number)
	getPointer(instance).OverlayTextureId = number;
end
function property.abi.prop_clothing_color_3(instance)
	return createPointer(getPointer(instance).Color3);
end
function property.abi.prop_set_clothing_color_3(instance, color_3)
	getPointer(instance).Color3 = getPointer(color_3);
end
function property.abi.prop_pants_pants_template(instance)
	return createPointer(getPointer(instance).PantsTemplate);
end
function property.abi.prop_set_pants_pants_template(instance, content)
	getPointer(instance).PantsTemplate = getPointer(content);
end
function property.abi.prop_shirt_shirt_template(instance)
	return createPointer(getPointer(instance).ShirtTemplate);
end
function property.abi.prop_set_shirt_shirt_template(instance, content)
	getPointer(instance).ShirtTemplate = getPointer(content);
end
function property.abi.prop_shirt_graphic_color_3(instance)
	return createPointer(getPointer(instance).Color3);
end
function property.abi.prop_set_shirt_graphic_color_3(instance, color_3)
	getPointer(instance).Color3 = getPointer(color_3);
end
function property.abi.prop_shirt_graphic_graphic(instance)
	return createPointer(getPointer(instance).Graphic);
end
function property.abi.prop_set_shirt_graphic_graphic(instance, content)
	getPointer(instance).Graphic = getPointer(content);
end
function property.abi.prop_chat_bubble_chat_enabled(instance)
	return getPointer(instance).BubbleChatEnabled and 1 or 0;
end
function property.abi.prop_set_chat_bubble_chat_enabled(instance, bool)
	getPointer(instance).BubbleChatEnabled = bool and 1 or 0;
end
function property.abi.prop_chat_load_default_chat(instance)
	return getPointer(instance).LoadDefaultChat and 1 or 0;
end
function property.abi.prop_click_detector_cursor_icon(instance)
	return createPointer(getPointer(instance).CursorIcon);
end
function property.abi.prop_set_click_detector_cursor_icon(instance, content)
	getPointer(instance).CursorIcon = getPointer(content);
end
function property.abi.prop_click_detector_max_activation_distance(instance)
	return getPointer(instance).MaxActivationDistance;
end
function property.abi.prop_set_click_detector_max_activation_distance(instance, number)
	getPointer(instance).MaxActivationDistance = number;
end
function property.abi.prop_clouds_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_clouds_color(instance, color_3)
	getPointer(instance).Color = getPointer(color_3);
end
function property.abi.prop_clouds_cover(instance)
	return getPointer(instance).Cover;
end
function property.abi.prop_set_clouds_cover(instance, number)
	getPointer(instance).Cover = number;
end
function property.abi.prop_clouds_density(instance)
	return getPointer(instance).Density;
end
function property.abi.prop_set_clouds_density(instance, number)
	getPointer(instance).Density = number;
end
function property.abi.prop_clouds_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_clouds_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_command_instance_allow_gui_access_points(instance)
	return getPointer(instance).AllowGUIAccessPoints and 1 or 0;
end
function property.abi.prop_command_instance_display_name(addr, instance)
	writeStringReturn(addr, getPointer(instance).DisplayName);
end
function property.abi.prop_set_command_instance_display_name(instance, addr, length)
	getPointer(instance).DisplayName = readString(addr, length);
end
function property.abi.prop_constraint_active(instance)
	return getPointer(instance).Active and 1 or 0;
end
function property.abi.prop_constraint_attachment_0(addr, instance)
	local value = getPointer(instance).Attachment0;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_constraint_attachment_0(instance, isSome, value)
	getPointer(instance).Attachment0 = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_constraint_attachment_1(addr, instance)
	local value = getPointer(instance).Attachment1;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_constraint_attachment_1(instance, isSome, value)
	getPointer(instance).Attachment1 = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_constraint_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_constraint_color(instance, brick_color)
	getPointer(instance).Color = getPointer(brick_color);
end
function property.abi.prop_constraint_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_constraint_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_constraint_visible(instance)
	return getPointer(instance).Visible and 1 or 0;
end
function property.abi.prop_set_constraint_visible(instance, bool)
	getPointer(instance).Visible = bool and 1 or 0;
end
function property.abi.prop_align_orientation_align_type(instance)
end
function property.abi.prop_align_orientation_c_frame(instance)
	return createPointer(getPointer(instance).CFrame);
end
function property.abi.prop_set_align_orientation_c_frame(instance, c_frame)
	getPointer(instance).CFrame = getPointer(c_frame);
end
function property.abi.prop_align_orientation_max_angular_velocity(instance)
	return getPointer(instance).MaxAngularVelocity;
end
function property.abi.prop_set_align_orientation_max_angular_velocity(instance, number)
	getPointer(instance).MaxAngularVelocity = number;
end
function property.abi.prop_align_orientation_max_torque(instance)
	return getPointer(instance).MaxTorque;
end
function property.abi.prop_set_align_orientation_max_torque(instance, number)
	getPointer(instance).MaxTorque = number;
end
function property.abi.prop_align_orientation_mode(instance)
end
function property.abi.prop_align_orientation_primary_axis(instance)
	return createPointer(getPointer(instance).PrimaryAxis);
end
function property.abi.prop_set_align_orientation_primary_axis(instance, vector_3)
	getPointer(instance).PrimaryAxis = getPointer(vector_3);
end
function property.abi.prop_align_orientation_primary_axis_only(instance)
	return getPointer(instance).PrimaryAxisOnly and 1 or 0;
end
function property.abi.prop_set_align_orientation_primary_axis_only(instance, bool)
	getPointer(instance).PrimaryAxisOnly = bool and 1 or 0;
end
function property.abi.prop_align_orientation_reaction_torque_enabled(instance)
	return getPointer(instance).ReactionTorqueEnabled and 1 or 0;
end
function property.abi.prop_set_align_orientation_reaction_torque_enabled(instance, bool)
	getPointer(instance).ReactionTorqueEnabled = bool and 1 or 0;
end
function property.abi.prop_align_orientation_responsiveness(instance)
	return getPointer(instance).Responsiveness;
end
function property.abi.prop_set_align_orientation_responsiveness(instance, number)
	getPointer(instance).Responsiveness = number;
end
function property.abi.prop_align_orientation_rigidity_enabled(instance)
	return getPointer(instance).RigidityEnabled and 1 or 0;
end
function property.abi.prop_set_align_orientation_rigidity_enabled(instance, bool)
	getPointer(instance).RigidityEnabled = bool and 1 or 0;
end
function property.abi.prop_align_orientation_secondary_axis(instance)
	return createPointer(getPointer(instance).SecondaryAxis);
end
function property.abi.prop_set_align_orientation_secondary_axis(instance, vector_3)
	getPointer(instance).SecondaryAxis = getPointer(vector_3);
end
function property.abi.prop_align_position_apply_at_center_of_mass(instance)
	return getPointer(instance).ApplyAtCenterOfMass and 1 or 0;
end
function property.abi.prop_set_align_position_apply_at_center_of_mass(instance, bool)
	getPointer(instance).ApplyAtCenterOfMass = bool and 1 or 0;
end
function property.abi.prop_align_position_max_force(instance)
	return getPointer(instance).MaxForce;
end
function property.abi.prop_set_align_position_max_force(instance, number)
	getPointer(instance).MaxForce = number;
end
function property.abi.prop_align_position_max_velocity(instance)
	return getPointer(instance).MaxVelocity;
end
function property.abi.prop_set_align_position_max_velocity(instance, number)
	getPointer(instance).MaxVelocity = number;
end
function property.abi.prop_align_position_mode(instance)
end
function property.abi.prop_align_position_position(instance)
	return createPointer(getPointer(instance).Position);
end
function property.abi.prop_set_align_position_position(instance, vector_3)
	getPointer(instance).Position = getPointer(vector_3);
end
function property.abi.prop_align_position_reaction_force_enabled(instance)
	return getPointer(instance).ReactionForceEnabled and 1 or 0;
end
function property.abi.prop_set_align_position_reaction_force_enabled(instance, bool)
	getPointer(instance).ReactionForceEnabled = bool and 1 or 0;
end
function property.abi.prop_align_position_responsiveness(instance)
	return getPointer(instance).Responsiveness;
end
function property.abi.prop_set_align_position_responsiveness(instance, number)
	getPointer(instance).Responsiveness = number;
end
function property.abi.prop_align_position_rigidity_enabled(instance)
	return getPointer(instance).RigidityEnabled and 1 or 0;
end
function property.abi.prop_set_align_position_rigidity_enabled(instance, bool)
	getPointer(instance).RigidityEnabled = bool and 1 or 0;
end
function property.abi.prop_angular_velocity_angular_velocity(instance)
	return createPointer(getPointer(instance).AngularVelocity);
end
function property.abi.prop_set_angular_velocity_angular_velocity(instance, vector_3)
	getPointer(instance).AngularVelocity = getPointer(vector_3);
end
function property.abi.prop_angular_velocity_max_torque(instance)
	return getPointer(instance).MaxTorque;
end
function property.abi.prop_set_angular_velocity_max_torque(instance, number)
	getPointer(instance).MaxTorque = number;
end
function property.abi.prop_angular_velocity_reaction_torque_enabled(instance)
	return getPointer(instance).ReactionTorqueEnabled and 1 or 0;
end
function property.abi.prop_set_angular_velocity_reaction_torque_enabled(instance, bool)
	getPointer(instance).ReactionTorqueEnabled = bool and 1 or 0;
end
function property.abi.prop_angular_velocity_relative_to(instance)
end
function property.abi.prop_ball_socket_constraint_limits_enabled(instance)
	return getPointer(instance).LimitsEnabled and 1 or 0;
end
function property.abi.prop_set_ball_socket_constraint_limits_enabled(instance, bool)
	getPointer(instance).LimitsEnabled = bool and 1 or 0;
end
function property.abi.prop_ball_socket_constraint_max_friction_torque(instance)
	return getPointer(instance).MaxFrictionTorque;
end
function property.abi.prop_set_ball_socket_constraint_max_friction_torque(instance, number)
	getPointer(instance).MaxFrictionTorque = number;
end
function property.abi.prop_ball_socket_constraint_radius(instance)
	return getPointer(instance).Radius;
end
function property.abi.prop_set_ball_socket_constraint_radius(instance, number)
	getPointer(instance).Radius = number;
end
function property.abi.prop_ball_socket_constraint_restitution(instance)
	return getPointer(instance).Restitution;
end
function property.abi.prop_set_ball_socket_constraint_restitution(instance, number)
	getPointer(instance).Restitution = number;
end
function property.abi.prop_ball_socket_constraint_twist_limits_enabled(instance)
	return getPointer(instance).TwistLimitsEnabled and 1 or 0;
end
function property.abi.prop_set_ball_socket_constraint_twist_limits_enabled(instance, bool)
	getPointer(instance).TwistLimitsEnabled = bool and 1 or 0;
end
function property.abi.prop_ball_socket_constraint_twist_lower_angle(instance)
	return getPointer(instance).TwistLowerAngle;
end
function property.abi.prop_set_ball_socket_constraint_twist_lower_angle(instance, number)
	getPointer(instance).TwistLowerAngle = number;
end
function property.abi.prop_ball_socket_constraint_twist_upper_angle(instance)
	return getPointer(instance).TwistUpperAngle;
end
function property.abi.prop_set_ball_socket_constraint_twist_upper_angle(instance, number)
	getPointer(instance).TwistUpperAngle = number;
end
function property.abi.prop_ball_socket_constraint_upper_angle(instance)
	return getPointer(instance).UpperAngle;
end
function property.abi.prop_set_ball_socket_constraint_upper_angle(instance, number)
	getPointer(instance).UpperAngle = number;
end
function property.abi.prop_hinge_constraint_actuator_type(instance)
end
function property.abi.prop_hinge_constraint_angular_responsiveness(instance)
	return getPointer(instance).AngularResponsiveness;
end
function property.abi.prop_set_hinge_constraint_angular_responsiveness(instance, number)
	getPointer(instance).AngularResponsiveness = number;
end
function property.abi.prop_hinge_constraint_angular_speed(instance)
	return getPointer(instance).AngularSpeed;
end
function property.abi.prop_set_hinge_constraint_angular_speed(instance, number)
	getPointer(instance).AngularSpeed = number;
end
function property.abi.prop_hinge_constraint_angular_velocity(instance)
	return getPointer(instance).AngularVelocity;
end
function property.abi.prop_set_hinge_constraint_angular_velocity(instance, number)
	getPointer(instance).AngularVelocity = number;
end
function property.abi.prop_hinge_constraint_current_angle(instance)
	return getPointer(instance).CurrentAngle;
end
function property.abi.prop_hinge_constraint_limits_enabled(instance)
	return getPointer(instance).LimitsEnabled and 1 or 0;
end
function property.abi.prop_set_hinge_constraint_limits_enabled(instance, bool)
	getPointer(instance).LimitsEnabled = bool and 1 or 0;
end
function property.abi.prop_hinge_constraint_lower_angle(instance)
	return getPointer(instance).LowerAngle;
end
function property.abi.prop_set_hinge_constraint_lower_angle(instance, number)
	getPointer(instance).LowerAngle = number;
end
function property.abi.prop_hinge_constraint_motor_max_acceleration(instance)
	return getPointer(instance).MotorMaxAcceleration;
end
function property.abi.prop_set_hinge_constraint_motor_max_acceleration(instance, number)
	getPointer(instance).MotorMaxAcceleration = number;
end
function property.abi.prop_hinge_constraint_motor_max_torque(instance)
	return getPointer(instance).MotorMaxTorque;
end
function property.abi.prop_set_hinge_constraint_motor_max_torque(instance, number)
	getPointer(instance).MotorMaxTorque = number;
end
function property.abi.prop_hinge_constraint_radius(instance)
	return getPointer(instance).Radius;
end
function property.abi.prop_set_hinge_constraint_radius(instance, number)
	getPointer(instance).Radius = number;
end
function property.abi.prop_hinge_constraint_restitution(instance)
	return getPointer(instance).Restitution;
end
function property.abi.prop_set_hinge_constraint_restitution(instance, number)
	getPointer(instance).Restitution = number;
end
function property.abi.prop_hinge_constraint_servo_max_torque(instance)
	return getPointer(instance).ServoMaxTorque;
end
function property.abi.prop_set_hinge_constraint_servo_max_torque(instance, number)
	getPointer(instance).ServoMaxTorque = number;
end
function property.abi.prop_hinge_constraint_target_angle(instance)
	return getPointer(instance).TargetAngle;
end
function property.abi.prop_set_hinge_constraint_target_angle(instance, number)
	getPointer(instance).TargetAngle = number;
end
function property.abi.prop_hinge_constraint_upper_angle(instance)
	return getPointer(instance).UpperAngle;
end
function property.abi.prop_set_hinge_constraint_upper_angle(instance, number)
	getPointer(instance).UpperAngle = number;
end
function property.abi.prop_line_force_apply_at_center_of_mass(instance)
	return getPointer(instance).ApplyAtCenterOfMass and 1 or 0;
end
function property.abi.prop_set_line_force_apply_at_center_of_mass(instance, bool)
	getPointer(instance).ApplyAtCenterOfMass = bool and 1 or 0;
end
function property.abi.prop_line_force_inverse_square_law(instance)
	return getPointer(instance).InverseSquareLaw and 1 or 0;
end
function property.abi.prop_set_line_force_inverse_square_law(instance, bool)
	getPointer(instance).InverseSquareLaw = bool and 1 or 0;
end
function property.abi.prop_line_force_magnitude(instance)
	return getPointer(instance).Magnitude;
end
function property.abi.prop_set_line_force_magnitude(instance, number)
	getPointer(instance).Magnitude = number;
end
function property.abi.prop_line_force_max_force(instance)
	return getPointer(instance).MaxForce;
end
function property.abi.prop_set_line_force_max_force(instance, number)
	getPointer(instance).MaxForce = number;
end
function property.abi.prop_line_force_reaction_force_enabled(instance)
	return getPointer(instance).ReactionForceEnabled and 1 or 0;
end
function property.abi.prop_set_line_force_reaction_force_enabled(instance, bool)
	getPointer(instance).ReactionForceEnabled = bool and 1 or 0;
end
function property.abi.prop_linear_velocity_line_direction(instance)
	return createPointer(getPointer(instance).LineDirection);
end
function property.abi.prop_set_linear_velocity_line_direction(instance, vector_3)
	getPointer(instance).LineDirection = getPointer(vector_3);
end
function property.abi.prop_linear_velocity_line_velocity(instance)
	return getPointer(instance).LineVelocity;
end
function property.abi.prop_set_linear_velocity_line_velocity(instance, number)
	getPointer(instance).LineVelocity = number;
end
function property.abi.prop_linear_velocity_max_force(instance)
	return getPointer(instance).MaxForce;
end
function property.abi.prop_set_linear_velocity_max_force(instance, number)
	getPointer(instance).MaxForce = number;
end
function property.abi.prop_linear_velocity_plane_velocity(instance)
	return createPointer(getPointer(instance).PlaneVelocity);
end
function property.abi.prop_set_linear_velocity_plane_velocity(instance, vector_2)
	getPointer(instance).PlaneVelocity = getPointer(vector_2);
end
function property.abi.prop_linear_velocity_primary_tangent_axis(instance)
	return createPointer(getPointer(instance).PrimaryTangentAxis);
end
function property.abi.prop_set_linear_velocity_primary_tangent_axis(instance, vector_3)
	getPointer(instance).PrimaryTangentAxis = getPointer(vector_3);
end
function property.abi.prop_linear_velocity_relative_to(instance)
end
function property.abi.prop_linear_velocity_secondary_tangent_axis(instance)
	return createPointer(getPointer(instance).SecondaryTangentAxis);
end
function property.abi.prop_set_linear_velocity_secondary_tangent_axis(instance, vector_3)
	getPointer(instance).SecondaryTangentAxis = getPointer(vector_3);
end
function property.abi.prop_linear_velocity_vector_velocity(instance)
	return createPointer(getPointer(instance).VectorVelocity);
end
function property.abi.prop_set_linear_velocity_vector_velocity(instance, vector_3)
	getPointer(instance).VectorVelocity = getPointer(vector_3);
end
function property.abi.prop_linear_velocity_velocity_constraint_mode(instance)
end
function property.abi.prop_rigid_constraint_broken(instance)
	return getPointer(instance).Broken and 1 or 0;
end
function property.abi.prop_rigid_constraint_destruction_enabled(instance)
	return getPointer(instance).DestructionEnabled and 1 or 0;
end
function property.abi.prop_set_rigid_constraint_destruction_enabled(instance, bool)
	getPointer(instance).DestructionEnabled = bool and 1 or 0;
end
function property.abi.prop_rigid_constraint_destruction_force(instance)
	return getPointer(instance).DestructionForce;
end
function property.abi.prop_set_rigid_constraint_destruction_force(instance, number)
	getPointer(instance).DestructionForce = number;
end
function property.abi.prop_rigid_constraint_destruction_torque(instance)
	return getPointer(instance).DestructionTorque;
end
function property.abi.prop_set_rigid_constraint_destruction_torque(instance, number)
	getPointer(instance).DestructionTorque = number;
end
function property.abi.prop_rod_constraint_current_distance(instance)
	return getPointer(instance).CurrentDistance;
end
function property.abi.prop_rod_constraint_length(instance)
	return getPointer(instance).Length;
end
function property.abi.prop_set_rod_constraint_length(instance, number)
	getPointer(instance).Length = number;
end
function property.abi.prop_rod_constraint_limit_angle_0(instance)
	return getPointer(instance).LimitAngle0;
end
function property.abi.prop_set_rod_constraint_limit_angle_0(instance, number)
	getPointer(instance).LimitAngle0 = number;
end
function property.abi.prop_rod_constraint_limit_angle_1(instance)
	return getPointer(instance).LimitAngle1;
end
function property.abi.prop_set_rod_constraint_limit_angle_1(instance, number)
	getPointer(instance).LimitAngle1 = number;
end
function property.abi.prop_rod_constraint_limits_enabled(instance)
	return getPointer(instance).LimitsEnabled and 1 or 0;
end
function property.abi.prop_set_rod_constraint_limits_enabled(instance, bool)
	getPointer(instance).LimitsEnabled = bool and 1 or 0;
end
function property.abi.prop_rod_constraint_thickness(instance)
	return getPointer(instance).Thickness;
end
function property.abi.prop_set_rod_constraint_thickness(instance, number)
	getPointer(instance).Thickness = number;
end
function property.abi.prop_rope_constraint_current_distance(instance)
	return getPointer(instance).CurrentDistance;
end
function property.abi.prop_rope_constraint_length(instance)
	return getPointer(instance).Length;
end
function property.abi.prop_set_rope_constraint_length(instance, number)
	getPointer(instance).Length = number;
end
function property.abi.prop_rope_constraint_restitution(instance)
	return getPointer(instance).Restitution;
end
function property.abi.prop_set_rope_constraint_restitution(instance, number)
	getPointer(instance).Restitution = number;
end
function property.abi.prop_rope_constraint_thickness(instance)
	return getPointer(instance).Thickness;
end
function property.abi.prop_set_rope_constraint_thickness(instance, number)
	getPointer(instance).Thickness = number;
end
function property.abi.prop_rope_constraint_winch_enabled(instance)
	return getPointer(instance).WinchEnabled and 1 or 0;
end
function property.abi.prop_set_rope_constraint_winch_enabled(instance, bool)
	getPointer(instance).WinchEnabled = bool and 1 or 0;
end
function property.abi.prop_rope_constraint_winch_force(instance)
	return getPointer(instance).WinchForce;
end
function property.abi.prop_set_rope_constraint_winch_force(instance, number)
	getPointer(instance).WinchForce = number;
end
function property.abi.prop_rope_constraint_winch_responsiveness(instance)
	return getPointer(instance).WinchResponsiveness;
end
function property.abi.prop_set_rope_constraint_winch_responsiveness(instance, number)
	getPointer(instance).WinchResponsiveness = number;
end
function property.abi.prop_rope_constraint_winch_speed(instance)
	return getPointer(instance).WinchSpeed;
end
function property.abi.prop_set_rope_constraint_winch_speed(instance, number)
	getPointer(instance).WinchSpeed = number;
end
function property.abi.prop_rope_constraint_winch_target(instance)
	return getPointer(instance).WinchTarget;
end
function property.abi.prop_set_rope_constraint_winch_target(instance, number)
	getPointer(instance).WinchTarget = number;
end
function property.abi.prop_sliding_ball_constraint_actuator_type(instance)
end
function property.abi.prop_sliding_ball_constraint_current_position(instance)
	return getPointer(instance).CurrentPosition;
end
function property.abi.prop_sliding_ball_constraint_limits_enabled(instance)
	return getPointer(instance).LimitsEnabled and 1 or 0;
end
function property.abi.prop_set_sliding_ball_constraint_limits_enabled(instance, bool)
	getPointer(instance).LimitsEnabled = bool and 1 or 0;
end
function property.abi.prop_sliding_ball_constraint_linear_responsiveness(instance)
	return getPointer(instance).LinearResponsiveness;
end
function property.abi.prop_set_sliding_ball_constraint_linear_responsiveness(instance, number)
	getPointer(instance).LinearResponsiveness = number;
end
function property.abi.prop_sliding_ball_constraint_lower_limit(instance)
	return getPointer(instance).LowerLimit;
end
function property.abi.prop_set_sliding_ball_constraint_lower_limit(instance, number)
	getPointer(instance).LowerLimit = number;
end
function property.abi.prop_sliding_ball_constraint_motor_max_acceleration(instance)
	return getPointer(instance).MotorMaxAcceleration;
end
function property.abi.prop_set_sliding_ball_constraint_motor_max_acceleration(instance, number)
	getPointer(instance).MotorMaxAcceleration = number;
end
function property.abi.prop_sliding_ball_constraint_motor_max_force(instance)
	return getPointer(instance).MotorMaxForce;
end
function property.abi.prop_set_sliding_ball_constraint_motor_max_force(instance, number)
	getPointer(instance).MotorMaxForce = number;
end
function property.abi.prop_sliding_ball_constraint_restitution(instance)
	return getPointer(instance).Restitution;
end
function property.abi.prop_set_sliding_ball_constraint_restitution(instance, number)
	getPointer(instance).Restitution = number;
end
function property.abi.prop_sliding_ball_constraint_servo_max_force(instance)
	return getPointer(instance).ServoMaxForce;
end
function property.abi.prop_set_sliding_ball_constraint_servo_max_force(instance, number)
	getPointer(instance).ServoMaxForce = number;
end
function property.abi.prop_sliding_ball_constraint_size(instance)
	return getPointer(instance).Size;
end
function property.abi.prop_set_sliding_ball_constraint_size(instance, number)
	getPointer(instance).Size = number;
end
function property.abi.prop_sliding_ball_constraint_speed(instance)
	return getPointer(instance).Speed;
end
function property.abi.prop_set_sliding_ball_constraint_speed(instance, number)
	getPointer(instance).Speed = number;
end
function property.abi.prop_sliding_ball_constraint_target_position(instance)
	return getPointer(instance).TargetPosition;
end
function property.abi.prop_set_sliding_ball_constraint_target_position(instance, number)
	getPointer(instance).TargetPosition = number;
end
function property.abi.prop_sliding_ball_constraint_upper_limit(instance)
	return getPointer(instance).UpperLimit;
end
function property.abi.prop_set_sliding_ball_constraint_upper_limit(instance, number)
	getPointer(instance).UpperLimit = number;
end
function property.abi.prop_sliding_ball_constraint_velocity(instance)
	return getPointer(instance).Velocity;
end
function property.abi.prop_set_sliding_ball_constraint_velocity(instance, number)
	getPointer(instance).Velocity = number;
end
function property.abi.prop_cylindrical_constraint_angular_actuator_type(instance)
end
function property.abi.prop_cylindrical_constraint_angular_limits_enabled(instance)
	return getPointer(instance).AngularLimitsEnabled and 1 or 0;
end
function property.abi.prop_set_cylindrical_constraint_angular_limits_enabled(instance, bool)
	getPointer(instance).AngularLimitsEnabled = bool and 1 or 0;
end
function property.abi.prop_cylindrical_constraint_angular_responsiveness(instance)
	return getPointer(instance).AngularResponsiveness;
end
function property.abi.prop_set_cylindrical_constraint_angular_responsiveness(instance, number)
	getPointer(instance).AngularResponsiveness = number;
end
function property.abi.prop_cylindrical_constraint_angular_restitution(instance)
	return getPointer(instance).AngularRestitution;
end
function property.abi.prop_set_cylindrical_constraint_angular_restitution(instance, number)
	getPointer(instance).AngularRestitution = number;
end
function property.abi.prop_cylindrical_constraint_angular_speed(instance)
	return getPointer(instance).AngularSpeed;
end
function property.abi.prop_set_cylindrical_constraint_angular_speed(instance, number)
	getPointer(instance).AngularSpeed = number;
end
function property.abi.prop_cylindrical_constraint_angular_velocity(instance)
	return getPointer(instance).AngularVelocity;
end
function property.abi.prop_set_cylindrical_constraint_angular_velocity(instance, number)
	getPointer(instance).AngularVelocity = number;
end
function property.abi.prop_cylindrical_constraint_current_angle(instance)
	return getPointer(instance).CurrentAngle;
end
function property.abi.prop_cylindrical_constraint_inclination_angle(instance)
	return getPointer(instance).InclinationAngle;
end
function property.abi.prop_set_cylindrical_constraint_inclination_angle(instance, number)
	getPointer(instance).InclinationAngle = number;
end
function property.abi.prop_cylindrical_constraint_lower_angle(instance)
	return getPointer(instance).LowerAngle;
end
function property.abi.prop_set_cylindrical_constraint_lower_angle(instance, number)
	getPointer(instance).LowerAngle = number;
end
function property.abi.prop_cylindrical_constraint_motor_max_angular_acceleration(instance)
	return getPointer(instance).MotorMaxAngularAcceleration;
end
function property.abi.prop_set_cylindrical_constraint_motor_max_angular_acceleration(instance, number)
	getPointer(instance).MotorMaxAngularAcceleration = number;
end
function property.abi.prop_cylindrical_constraint_motor_max_torque(instance)
	return getPointer(instance).MotorMaxTorque;
end
function property.abi.prop_set_cylindrical_constraint_motor_max_torque(instance, number)
	getPointer(instance).MotorMaxTorque = number;
end
function property.abi.prop_cylindrical_constraint_rotation_axis_visible(instance)
	return getPointer(instance).RotationAxisVisible and 1 or 0;
end
function property.abi.prop_set_cylindrical_constraint_rotation_axis_visible(instance, bool)
	getPointer(instance).RotationAxisVisible = bool and 1 or 0;
end
function property.abi.prop_cylindrical_constraint_servo_max_torque(instance)
	return getPointer(instance).ServoMaxTorque;
end
function property.abi.prop_set_cylindrical_constraint_servo_max_torque(instance, number)
	getPointer(instance).ServoMaxTorque = number;
end
function property.abi.prop_cylindrical_constraint_target_angle(instance)
	return getPointer(instance).TargetAngle;
end
function property.abi.prop_set_cylindrical_constraint_target_angle(instance, number)
	getPointer(instance).TargetAngle = number;
end
function property.abi.prop_cylindrical_constraint_upper_angle(instance)
	return getPointer(instance).UpperAngle;
end
function property.abi.prop_set_cylindrical_constraint_upper_angle(instance, number)
	getPointer(instance).UpperAngle = number;
end
function property.abi.prop_cylindrical_constraint_world_rotation_axis(instance)
	return createPointer(getPointer(instance).WorldRotationAxis);
end
function property.abi.prop_spring_constraint_coils(instance)
	return getPointer(instance).Coils;
end
function property.abi.prop_set_spring_constraint_coils(instance, number)
	getPointer(instance).Coils = number;
end
function property.abi.prop_spring_constraint_current_length(instance)
	return getPointer(instance).CurrentLength;
end
function property.abi.prop_spring_constraint_damping(instance)
	return getPointer(instance).Damping;
end
function property.abi.prop_set_spring_constraint_damping(instance, number)
	getPointer(instance).Damping = number;
end
function property.abi.prop_spring_constraint_free_length(instance)
	return getPointer(instance).FreeLength;
end
function property.abi.prop_set_spring_constraint_free_length(instance, number)
	getPointer(instance).FreeLength = number;
end
function property.abi.prop_spring_constraint_limits_enabled(instance)
	return getPointer(instance).LimitsEnabled and 1 or 0;
end
function property.abi.prop_set_spring_constraint_limits_enabled(instance, bool)
	getPointer(instance).LimitsEnabled = bool and 1 or 0;
end
function property.abi.prop_spring_constraint_max_force(instance)
	return getPointer(instance).MaxForce;
end
function property.abi.prop_set_spring_constraint_max_force(instance, number)
	getPointer(instance).MaxForce = number;
end
function property.abi.prop_spring_constraint_max_length(instance)
	return getPointer(instance).MaxLength;
end
function property.abi.prop_set_spring_constraint_max_length(instance, number)
	getPointer(instance).MaxLength = number;
end
function property.abi.prop_spring_constraint_min_length(instance)
	return getPointer(instance).MinLength;
end
function property.abi.prop_set_spring_constraint_min_length(instance, number)
	getPointer(instance).MinLength = number;
end
function property.abi.prop_spring_constraint_radius(instance)
	return getPointer(instance).Radius;
end
function property.abi.prop_set_spring_constraint_radius(instance, number)
	getPointer(instance).Radius = number;
end
function property.abi.prop_spring_constraint_stiffness(instance)
	return getPointer(instance).Stiffness;
end
function property.abi.prop_set_spring_constraint_stiffness(instance, number)
	getPointer(instance).Stiffness = number;
end
function property.abi.prop_spring_constraint_thickness(instance)
	return getPointer(instance).Thickness;
end
function property.abi.prop_set_spring_constraint_thickness(instance, number)
	getPointer(instance).Thickness = number;
end
function property.abi.prop_torque_relative_to(instance)
end
function property.abi.prop_torque_torque(instance)
	return createPointer(getPointer(instance).Torque);
end
function property.abi.prop_set_torque_torque(instance, vector_3)
	getPointer(instance).Torque = getPointer(vector_3);
end
function property.abi.prop_torsion_spring_constraint_coils(instance)
	return getPointer(instance).Coils;
end
function property.abi.prop_set_torsion_spring_constraint_coils(instance, number)
	getPointer(instance).Coils = number;
end
function property.abi.prop_torsion_spring_constraint_current_angle(instance)
	return getPointer(instance).CurrentAngle;
end
function property.abi.prop_torsion_spring_constraint_damping(instance)
	return getPointer(instance).Damping;
end
function property.abi.prop_set_torsion_spring_constraint_damping(instance, number)
	getPointer(instance).Damping = number;
end
function property.abi.prop_torsion_spring_constraint_limit_enabled(instance)
	return getPointer(instance).LimitEnabled and 1 or 0;
end
function property.abi.prop_set_torsion_spring_constraint_limit_enabled(instance, bool)
	getPointer(instance).LimitEnabled = bool and 1 or 0;
end
function property.abi.prop_torsion_spring_constraint_limits_enabled(instance)
	return getPointer(instance).LimitsEnabled and 1 or 0;
end
function property.abi.prop_set_torsion_spring_constraint_limits_enabled(instance, bool)
	getPointer(instance).LimitsEnabled = bool and 1 or 0;
end
function property.abi.prop_torsion_spring_constraint_max_angle(instance)
	return getPointer(instance).MaxAngle;
end
function property.abi.prop_set_torsion_spring_constraint_max_angle(instance, number)
	getPointer(instance).MaxAngle = number;
end
function property.abi.prop_torsion_spring_constraint_max_torque(instance)
	return getPointer(instance).MaxTorque;
end
function property.abi.prop_set_torsion_spring_constraint_max_torque(instance, number)
	getPointer(instance).MaxTorque = number;
end
function property.abi.prop_torsion_spring_constraint_radius(instance)
	return getPointer(instance).Radius;
end
function property.abi.prop_set_torsion_spring_constraint_radius(instance, number)
	getPointer(instance).Radius = number;
end
function property.abi.prop_torsion_spring_constraint_restitution(instance)
	return getPointer(instance).Restitution;
end
function property.abi.prop_set_torsion_spring_constraint_restitution(instance, number)
	getPointer(instance).Restitution = number;
end
function property.abi.prop_torsion_spring_constraint_stiffness(instance)
	return getPointer(instance).Stiffness;
end
function property.abi.prop_set_torsion_spring_constraint_stiffness(instance, number)
	getPointer(instance).Stiffness = number;
end
function property.abi.prop_universal_constraint_limits_enabled(instance)
	return getPointer(instance).LimitsEnabled and 1 or 0;
end
function property.abi.prop_set_universal_constraint_limits_enabled(instance, bool)
	getPointer(instance).LimitsEnabled = bool and 1 or 0;
end
function property.abi.prop_universal_constraint_max_angle(instance)
	return getPointer(instance).MaxAngle;
end
function property.abi.prop_set_universal_constraint_max_angle(instance, number)
	getPointer(instance).MaxAngle = number;
end
function property.abi.prop_universal_constraint_radius(instance)
	return getPointer(instance).Radius;
end
function property.abi.prop_set_universal_constraint_radius(instance, number)
	getPointer(instance).Radius = number;
end
function property.abi.prop_universal_constraint_restitution(instance)
	return getPointer(instance).Restitution;
end
function property.abi.prop_set_universal_constraint_restitution(instance, number)
	getPointer(instance).Restitution = number;
end
function property.abi.prop_vector_force_apply_at_center_of_mass(instance)
	return getPointer(instance).ApplyAtCenterOfMass and 1 or 0;
end
function property.abi.prop_set_vector_force_apply_at_center_of_mass(instance, bool)
	getPointer(instance).ApplyAtCenterOfMass = bool and 1 or 0;
end
function property.abi.prop_vector_force_force(instance)
	return createPointer(getPointer(instance).Force);
end
function property.abi.prop_set_vector_force_force(instance, vector_3)
	getPointer(instance).Force = getPointer(vector_3);
end
function property.abi.prop_vector_force_relative_to(instance)
end
function property.abi.prop_content_provider_base_url(addr, instance)
	writeStringReturn(addr, getPointer(instance).BaseUrl);
end
function property.abi.prop_content_provider_request_queue_size(instance)
	return getPointer(instance).RequestQueueSize;
end
function property.abi.prop_skateboard_controller_steer(instance)
	return getPointer(instance).Steer;
end
function property.abi.prop_skateboard_controller_throttle(instance)
	return getPointer(instance).Throttle;
end
function property.abi.prop_data_model_mesh_offset(instance)
	return createPointer(getPointer(instance).Offset);
end
function property.abi.prop_set_data_model_mesh_offset(instance, vector_3)
	getPointer(instance).Offset = getPointer(vector_3);
end
function property.abi.prop_data_model_mesh_scale(instance)
	return createPointer(getPointer(instance).Scale);
end
function property.abi.prop_set_data_model_mesh_scale(instance, vector_3)
	getPointer(instance).Scale = getPointer(vector_3);
end
function property.abi.prop_data_model_mesh_vertex_color(instance)
	return createPointer(getPointer(instance).VertexColor);
end
function property.abi.prop_set_data_model_mesh_vertex_color(instance, vector_3)
	getPointer(instance).VertexColor = getPointer(vector_3);
end
function property.abi.prop_file_mesh_mesh_id(instance)
	return createPointer(getPointer(instance).MeshId);
end
function property.abi.prop_set_file_mesh_mesh_id(instance, content)
	getPointer(instance).MeshId = getPointer(content);
end
function property.abi.prop_file_mesh_texture_id(instance)
	return createPointer(getPointer(instance).TextureId);
end
function property.abi.prop_set_file_mesh_texture_id(instance, content)
	getPointer(instance).TextureId = getPointer(content);
end
function property.abi.prop_special_mesh_mesh_type(instance)
end
function property.abi.prop_data_store_info_created_time(instance)
	return getPointer(instance).CreatedTime;
end
function property.abi.prop_data_store_info_data_store_name(addr, instance)
	writeStringReturn(addr, getPointer(instance).DataStoreName);
end
function property.abi.prop_data_store_info_updated_time(instance)
	return getPointer(instance).UpdatedTime;
end
function property.abi.prop_data_store_key_key_name(addr, instance)
	writeStringReturn(addr, getPointer(instance).KeyName);
end
function property.abi.prop_data_store_key_info_created_time(instance)
	return getPointer(instance).CreatedTime;
end
function property.abi.prop_data_store_key_info_updated_time(instance)
	return getPointer(instance).UpdatedTime;
end
function property.abi.prop_data_store_key_info_version(addr, instance)
	writeStringReturn(addr, getPointer(instance).Version);
end
function property.abi.prop_data_store_object_version_info_created_time(instance)
	return getPointer(instance).CreatedTime;
end
function property.abi.prop_data_store_object_version_info_is_deleted(instance)
	return getPointer(instance).IsDeleted and 1 or 0;
end
function property.abi.prop_data_store_object_version_info_version(addr, instance)
	writeStringReturn(addr, getPointer(instance).Version);
end
function property.abi.prop_data_store_options_all_scopes(instance)
	return getPointer(instance).AllScopes and 1 or 0;
end
function property.abi.prop_set_data_store_options_all_scopes(instance, bool)
	getPointer(instance).AllScopes = bool and 1 or 0;
end
function property.abi.prop_debris_max_items(instance)
	return getPointer(instance).MaxItems;
end
function property.abi.prop_set_debris_max_items(instance, number)
	getPointer(instance).MaxItems = number;
end
function property.abi.prop_dialog_behavior_type(instance)
end
function property.abi.prop_dialog_conversation_distance(instance)
	return getPointer(instance).ConversationDistance;
end
function property.abi.prop_set_dialog_conversation_distance(instance, number)
	getPointer(instance).ConversationDistance = number;
end
function property.abi.prop_dialog_goodbye_choice_active(instance)
	return getPointer(instance).GoodbyeChoiceActive and 1 or 0;
end
function property.abi.prop_set_dialog_goodbye_choice_active(instance, bool)
	getPointer(instance).GoodbyeChoiceActive = bool and 1 or 0;
end
function property.abi.prop_dialog_goodbye_dialog(addr, instance)
	writeStringReturn(addr, getPointer(instance).GoodbyeDialog);
end
function property.abi.prop_set_dialog_goodbye_dialog(instance, addr, length)
	getPointer(instance).GoodbyeDialog = readString(addr, length);
end
function property.abi.prop_dialog_in_use(instance)
	return getPointer(instance).InUse and 1 or 0;
end
function property.abi.prop_set_dialog_in_use(instance, bool)
	getPointer(instance).InUse = bool and 1 or 0;
end
function property.abi.prop_dialog_initial_prompt(addr, instance)
	writeStringReturn(addr, getPointer(instance).InitialPrompt);
end
function property.abi.prop_set_dialog_initial_prompt(instance, addr, length)
	getPointer(instance).InitialPrompt = readString(addr, length);
end
function property.abi.prop_dialog_purpose(instance)
end
function property.abi.prop_dialog_tone(instance)
end
function property.abi.prop_dialog_trigger_distance(instance)
	return getPointer(instance).TriggerDistance;
end
function property.abi.prop_set_dialog_trigger_distance(instance, number)
	getPointer(instance).TriggerDistance = number;
end
function property.abi.prop_dialog_trigger_offset(instance)
	return createPointer(getPointer(instance).TriggerOffset);
end
function property.abi.prop_set_dialog_trigger_offset(instance, vector_3)
	getPointer(instance).TriggerOffset = getPointer(vector_3);
end
function property.abi.prop_dialog_choice_goodbye_choice_active(instance)
	return getPointer(instance).GoodbyeChoiceActive and 1 or 0;
end
function property.abi.prop_set_dialog_choice_goodbye_choice_active(instance, bool)
	getPointer(instance).GoodbyeChoiceActive = bool and 1 or 0;
end
function property.abi.prop_dialog_choice_goodbye_dialog(addr, instance)
	writeStringReturn(addr, getPointer(instance).GoodbyeDialog);
end
function property.abi.prop_set_dialog_choice_goodbye_dialog(instance, addr, length)
	getPointer(instance).GoodbyeDialog = readString(addr, length);
end
function property.abi.prop_dialog_choice_response_dialog(addr, instance)
	writeStringReturn(addr, getPointer(instance).ResponseDialog);
end
function property.abi.prop_set_dialog_choice_response_dialog(instance, addr, length)
	getPointer(instance).ResponseDialog = readString(addr, length);
end
function property.abi.prop_dialog_choice_user_dialog(addr, instance)
	writeStringReturn(addr, getPointer(instance).UserDialog);
end
function property.abi.prop_set_dialog_choice_user_dialog(instance, addr, length)
	getPointer(instance).UserDialog = readString(addr, length);
end
function property.abi.prop_dragger_service_align_dragged_objects(instance)
	return getPointer(instance).AlignDraggedObjects and 1 or 0;
end
function property.abi.prop_dragger_service_angle_snap_enabled(instance)
	return getPointer(instance).AngleSnapEnabled and 1 or 0;
end
function property.abi.prop_dragger_service_angle_snap_increment(instance)
	return getPointer(instance).AngleSnapIncrement;
end
function property.abi.prop_dragger_service_animate_hover(instance)
	return getPointer(instance).AnimateHover and 1 or 0;
end
function property.abi.prop_dragger_service_collisions_enabled(instance)
	return getPointer(instance).CollisionsEnabled and 1 or 0;
end
function property.abi.prop_dragger_service_dragger_coordinate_space(instance)
end
function property.abi.prop_dragger_service_dragger_movement_mode(instance)
end
function property.abi.prop_dragger_service_geometry_snap_color(instance)
	return createPointer(getPointer(instance).GeometrySnapColor);
end
function property.abi.prop_dragger_service_hover_animate_frequency(instance)
	return getPointer(instance).HoverAnimateFrequency;
end
function property.abi.prop_dragger_service_hover_thickness(instance)
	return getPointer(instance).HoverThickness;
end
function property.abi.prop_dragger_service_joints_enabled(instance)
	return getPointer(instance).JointsEnabled and 1 or 0;
end
function property.abi.prop_dragger_service_linear_snap_enabled(instance)
	return getPointer(instance).LinearSnapEnabled and 1 or 0;
end
function property.abi.prop_dragger_service_linear_snap_increment(instance)
	return getPointer(instance).LinearSnapIncrement;
end
function property.abi.prop_dragger_service_show_hover(instance)
	return getPointer(instance).ShowHover and 1 or 0;
end
function property.abi.prop_dragger_service_show_pivot_indicator(instance)
	return getPointer(instance).ShowPivotIndicator and 1 or 0;
end
function property.abi.prop_set_dragger_service_show_pivot_indicator(instance, bool)
	getPointer(instance).ShowPivotIndicator = bool and 1 or 0;
end
function property.abi.prop_euler_rotation_curve_rotation_order(instance)
end
function property.abi.prop_explosion_blast_pressure(instance)
	return getPointer(instance).BlastPressure;
end
function property.abi.prop_set_explosion_blast_pressure(instance, number)
	getPointer(instance).BlastPressure = number;
end
function property.abi.prop_explosion_blast_radius(instance)
	return getPointer(instance).BlastRadius;
end
function property.abi.prop_set_explosion_blast_radius(instance, number)
	getPointer(instance).BlastRadius = number;
end
function property.abi.prop_explosion_destroy_joint_radius_percent(instance)
	return getPointer(instance).DestroyJointRadiusPercent;
end
function property.abi.prop_set_explosion_destroy_joint_radius_percent(instance, number)
	getPointer(instance).DestroyJointRadiusPercent = number;
end
function property.abi.prop_explosion_explosion_type(instance)
end
function property.abi.prop_explosion_position(instance)
	return createPointer(getPointer(instance).Position);
end
function property.abi.prop_set_explosion_position(instance, vector_3)
	getPointer(instance).Position = getPointer(vector_3);
end
function property.abi.prop_explosion_time_scale(instance)
	return getPointer(instance).TimeScale;
end
function property.abi.prop_set_explosion_time_scale(instance, number)
	getPointer(instance).TimeScale = number;
end
function property.abi.prop_explosion_visible(instance)
	return getPointer(instance).Visible and 1 or 0;
end
function property.abi.prop_set_explosion_visible(instance, bool)
	getPointer(instance).Visible = bool and 1 or 0;
end
function property.abi.prop_face_instance_face(instance)
end
function property.abi.prop_decal_color_3(instance)
	return createPointer(getPointer(instance).Color3);
end
function property.abi.prop_set_decal_color_3(instance, color_3)
	getPointer(instance).Color3 = getPointer(color_3);
end
function property.abi.prop_decal_local_transparency_modifier(instance)
	return getPointer(instance).LocalTransparencyModifier;
end
function property.abi.prop_set_decal_local_transparency_modifier(instance, number)
	getPointer(instance).LocalTransparencyModifier = number;
end
function property.abi.prop_decal_shiny(instance)
	return getPointer(instance).Shiny;
end
function property.abi.prop_set_decal_shiny(instance, number)
	getPointer(instance).Shiny = number;
end
function property.abi.prop_decal_specular(instance)
	return getPointer(instance).Specular;
end
function property.abi.prop_set_decal_specular(instance, number)
	getPointer(instance).Specular = number;
end
function property.abi.prop_decal_texture(instance)
	return createPointer(getPointer(instance).Texture);
end
function property.abi.prop_set_decal_texture(instance, content)
	getPointer(instance).Texture = getPointer(content);
end
function property.abi.prop_decal_transparency(instance)
	return getPointer(instance).Transparency;
end
function property.abi.prop_set_decal_transparency(instance, number)
	getPointer(instance).Transparency = number;
end
function property.abi.prop_decal_z_index(instance)
	return getPointer(instance).ZIndex;
end
function property.abi.prop_set_decal_z_index(instance, number)
	getPointer(instance).ZIndex = number;
end
function property.abi.prop_texture_offset_studs_u(instance)
	return getPointer(instance).OffsetStudsU;
end
function property.abi.prop_set_texture_offset_studs_u(instance, number)
	getPointer(instance).OffsetStudsU = number;
end
function property.abi.prop_texture_offset_studs_v(instance)
	return getPointer(instance).OffsetStudsV;
end
function property.abi.prop_set_texture_offset_studs_v(instance, number)
	getPointer(instance).OffsetStudsV = number;
end
function property.abi.prop_texture_studs_per_tile_u(instance)
	return getPointer(instance).StudsPerTileU;
end
function property.abi.prop_set_texture_studs_per_tile_u(instance, number)
	getPointer(instance).StudsPerTileU = number;
end
function property.abi.prop_texture_studs_per_tile_v(instance)
	return getPointer(instance).StudsPerTileV;
end
function property.abi.prop_set_texture_studs_per_tile_v(instance, number)
	getPointer(instance).StudsPerTileV = number;
end
function property.abi.prop_feature_face_id(instance)
end
function property.abi.prop_feature_in_out(instance)
end
function property.abi.prop_feature_left_right(instance)
end
function property.abi.prop_feature_top_bottom(instance)
end
function property.abi.prop_fire_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_fire_color(instance, color_3)
	getPointer(instance).Color = getPointer(color_3);
end
function property.abi.prop_fire_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_fire_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_fire_heat(instance)
	return getPointer(instance).Heat;
end
function property.abi.prop_set_fire_heat(instance, number)
	getPointer(instance).Heat = number;
end
function property.abi.prop_fire_secondary_color(instance)
	return createPointer(getPointer(instance).SecondaryColor);
end
function property.abi.prop_set_fire_secondary_color(instance, color_3)
	getPointer(instance).SecondaryColor = getPointer(color_3);
end
function property.abi.prop_fire_size(instance)
	return getPointer(instance).Size;
end
function property.abi.prop_set_fire_size(instance, number)
	getPointer(instance).Size = number;
end
function property.abi.prop_fire_time_scale(instance)
	return getPointer(instance).TimeScale;
end
function property.abi.prop_set_fire_time_scale(instance, number)
	getPointer(instance).TimeScale = number;
end
function property.abi.prop_float_curve_length(instance)
	return getPointer(instance).Length;
end
function property.abi.prop_force_field_visible(instance)
	return getPointer(instance).Visible and 1 or 0;
end
function property.abi.prop_set_force_field_visible(instance, bool)
	getPointer(instance).Visible = bool and 1 or 0;
end
function property.abi.prop_get_text_bounds_params_font(instance)
	return createPointer(getPointer(instance).Font);
end
function property.abi.prop_set_get_text_bounds_params_font(instance, font)
	getPointer(instance).Font = getPointer(font);
end
function property.abi.prop_get_text_bounds_params_size(instance)
	return getPointer(instance).Size;
end
function property.abi.prop_set_get_text_bounds_params_size(instance, number)
	getPointer(instance).Size = number;
end
function property.abi.prop_get_text_bounds_params_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).Text);
end
function property.abi.prop_set_get_text_bounds_params_text(instance, addr, length)
	getPointer(instance).Text = readString(addr, length);
end
function property.abi.prop_get_text_bounds_params_width(instance)
	return getPointer(instance).Width;
end
function property.abi.prop_set_get_text_bounds_params_width(instance, number)
	getPointer(instance).Width = number;
end
function property.abi.prop_gui_base_2_d_absolute_position(instance)
	return createPointer(getPointer(instance).AbsolutePosition);
end
function property.abi.prop_gui_base_2_d_absolute_rotation(instance)
	return getPointer(instance).AbsoluteRotation;
end
function property.abi.prop_gui_base_2_d_absolute_size(instance)
	return createPointer(getPointer(instance).AbsoluteSize);
end
function property.abi.prop_gui_base_2_d_auto_localize(instance)
	return getPointer(instance).AutoLocalize and 1 or 0;
end
function property.abi.prop_set_gui_base_2_d_auto_localize(instance, bool)
	getPointer(instance).AutoLocalize = bool and 1 or 0;
end
function property.abi.prop_gui_base_2_d_localize(instance)
	return getPointer(instance).Localize and 1 or 0;
end
function property.abi.prop_set_gui_base_2_d_localize(instance, bool)
	getPointer(instance).Localize = bool and 1 or 0;
end
function property.abi.prop_gui_base_2_d_root_localization_table(addr, instance)
	local value = getPointer(instance).RootLocalizationTable;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_gui_base_2_d_root_localization_table(instance, isSome, value)
	getPointer(instance).RootLocalizationTable = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_gui_base_2_d_selection_behavior_down(instance)
end
function property.abi.prop_gui_base_2_d_selection_behavior_left(instance)
end
function property.abi.prop_gui_base_2_d_selection_behavior_right(instance)
end
function property.abi.prop_gui_base_2_d_selection_behavior_up(instance)
end
function property.abi.prop_gui_base_2_d_selection_group(instance)
	return getPointer(instance).SelectionGroup and 1 or 0;
end
function property.abi.prop_set_gui_base_2_d_selection_group(instance, bool)
	getPointer(instance).SelectionGroup = bool and 1 or 0;
end
function property.abi.prop_gui_object_active(instance)
	return getPointer(instance).Active and 1 or 0;
end
function property.abi.prop_set_gui_object_active(instance, bool)
	getPointer(instance).Active = bool and 1 or 0;
end
function property.abi.prop_gui_object_anchor_point(instance)
	return createPointer(getPointer(instance).AnchorPoint);
end
function property.abi.prop_set_gui_object_anchor_point(instance, vector_2)
	getPointer(instance).AnchorPoint = getPointer(vector_2);
end
function property.abi.prop_gui_object_automatic_size(instance)
end
function property.abi.prop_gui_object_background_color(instance)
	return createPointer(getPointer(instance).BackgroundColor);
end
function property.abi.prop_set_gui_object_background_color(instance, brick_color)
	getPointer(instance).BackgroundColor = getPointer(brick_color);
end
function property.abi.prop_gui_object_background_color_3(instance)
	return createPointer(getPointer(instance).BackgroundColor3);
end
function property.abi.prop_set_gui_object_background_color_3(instance, color_3)
	getPointer(instance).BackgroundColor3 = getPointer(color_3);
end
function property.abi.prop_gui_object_background_transparency(instance)
	return getPointer(instance).BackgroundTransparency;
end
function property.abi.prop_set_gui_object_background_transparency(instance, number)
	getPointer(instance).BackgroundTransparency = number;
end
function property.abi.prop_gui_object_border_color(instance)
	return createPointer(getPointer(instance).BorderColor);
end
function property.abi.prop_set_gui_object_border_color(instance, brick_color)
	getPointer(instance).BorderColor = getPointer(brick_color);
end
function property.abi.prop_gui_object_border_color_3(instance)
	return createPointer(getPointer(instance).BorderColor3);
end
function property.abi.prop_set_gui_object_border_color_3(instance, color_3)
	getPointer(instance).BorderColor3 = getPointer(color_3);
end
function property.abi.prop_gui_object_border_mode(instance)
end
function property.abi.prop_gui_object_border_size_pixel(instance)
	return getPointer(instance).BorderSizePixel;
end
function property.abi.prop_set_gui_object_border_size_pixel(instance, number)
	getPointer(instance).BorderSizePixel = number;
end
function property.abi.prop_gui_object_clips_descendants(instance)
	return getPointer(instance).ClipsDescendants and 1 or 0;
end
function property.abi.prop_set_gui_object_clips_descendants(instance, bool)
	getPointer(instance).ClipsDescendants = bool and 1 or 0;
end
function property.abi.prop_gui_object_draggable(instance)
	return getPointer(instance).Draggable and 1 or 0;
end
function property.abi.prop_set_gui_object_draggable(instance, bool)
	getPointer(instance).Draggable = bool and 1 or 0;
end
function property.abi.prop_gui_object_layout_order(instance)
	return getPointer(instance).LayoutOrder;
end
function property.abi.prop_set_gui_object_layout_order(instance, number)
	getPointer(instance).LayoutOrder = number;
end
function property.abi.prop_gui_object_next_selection_down(addr, instance)
	local value = getPointer(instance).NextSelectionDown;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_gui_object_next_selection_down(instance, isSome, value)
	getPointer(instance).NextSelectionDown = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_gui_object_next_selection_left(addr, instance)
	local value = getPointer(instance).NextSelectionLeft;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_gui_object_next_selection_left(instance, isSome, value)
	getPointer(instance).NextSelectionLeft = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_gui_object_next_selection_right(addr, instance)
	local value = getPointer(instance).NextSelectionRight;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_gui_object_next_selection_right(instance, isSome, value)
	getPointer(instance).NextSelectionRight = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_gui_object_next_selection_up(addr, instance)
	local value = getPointer(instance).NextSelectionUp;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_gui_object_next_selection_up(instance, isSome, value)
	getPointer(instance).NextSelectionUp = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_gui_object_position(instance)
	return createPointer(getPointer(instance).Position);
end
function property.abi.prop_set_gui_object_position(instance, u_dim_2)
	getPointer(instance).Position = getPointer(u_dim_2);
end
function property.abi.prop_gui_object_rotation(instance)
	return getPointer(instance).Rotation;
end
function property.abi.prop_set_gui_object_rotation(instance, number)
	getPointer(instance).Rotation = number;
end
function property.abi.prop_gui_object_selectable(instance)
	return getPointer(instance).Selectable and 1 or 0;
end
function property.abi.prop_set_gui_object_selectable(instance, bool)
	getPointer(instance).Selectable = bool and 1 or 0;
end
function property.abi.prop_gui_object_selection_image_object(addr, instance)
	local value = getPointer(instance).SelectionImageObject;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_gui_object_selection_image_object(instance, isSome, value)
	getPointer(instance).SelectionImageObject = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_gui_object_selection_order(instance)
	return getPointer(instance).SelectionOrder;
end
function property.abi.prop_set_gui_object_selection_order(instance, number)
	getPointer(instance).SelectionOrder = number;
end
function property.abi.prop_gui_object_size(instance)
	return createPointer(getPointer(instance).Size);
end
function property.abi.prop_set_gui_object_size(instance, u_dim_2)
	getPointer(instance).Size = getPointer(u_dim_2);
end
function property.abi.prop_gui_object_size_constraint(instance)
end
function property.abi.prop_gui_object_transparency(instance)
	return getPointer(instance).Transparency;
end
function property.abi.prop_set_gui_object_transparency(instance, number)
	getPointer(instance).Transparency = number;
end
function property.abi.prop_gui_object_visible(instance)
	return getPointer(instance).Visible and 1 or 0;
end
function property.abi.prop_set_gui_object_visible(instance, bool)
	getPointer(instance).Visible = bool and 1 or 0;
end
function property.abi.prop_gui_object_z_index(instance)
	return getPointer(instance).ZIndex;
end
function property.abi.prop_set_gui_object_z_index(instance, number)
	getPointer(instance).ZIndex = number;
end
function property.abi.prop_canvas_group_group_color_3(instance)
	return createPointer(getPointer(instance).GroupColor3);
end
function property.abi.prop_set_canvas_group_group_color_3(instance, color_3)
	getPointer(instance).GroupColor3 = getPointer(color_3);
end
function property.abi.prop_canvas_group_group_transparency(instance)
	return getPointer(instance).GroupTransparency;
end
function property.abi.prop_set_canvas_group_group_transparency(instance, number)
	getPointer(instance).GroupTransparency = number;
end
function property.abi.prop_frame_style(instance)
end
function property.abi.prop_gui_button_auto_button_color(instance)
	return getPointer(instance).AutoButtonColor and 1 or 0;
end
function property.abi.prop_set_gui_button_auto_button_color(instance, bool)
	getPointer(instance).AutoButtonColor = bool and 1 or 0;
end
function property.abi.prop_gui_button_modal(instance)
	return getPointer(instance).Modal and 1 or 0;
end
function property.abi.prop_set_gui_button_modal(instance, bool)
	getPointer(instance).Modal = bool and 1 or 0;
end
function property.abi.prop_gui_button_selected(instance)
	return getPointer(instance).Selected and 1 or 0;
end
function property.abi.prop_set_gui_button_selected(instance, bool)
	getPointer(instance).Selected = bool and 1 or 0;
end
function property.abi.prop_gui_button_style(instance)
end
function property.abi.prop_image_button_hover_image(instance)
	return createPointer(getPointer(instance).HoverImage);
end
function property.abi.prop_set_image_button_hover_image(instance, content)
	getPointer(instance).HoverImage = getPointer(content);
end
function property.abi.prop_image_button_image(instance)
	return createPointer(getPointer(instance).Image);
end
function property.abi.prop_set_image_button_image(instance, content)
	getPointer(instance).Image = getPointer(content);
end
function property.abi.prop_image_button_image_color_3(instance)
	return createPointer(getPointer(instance).ImageColor3);
end
function property.abi.prop_set_image_button_image_color_3(instance, color_3)
	getPointer(instance).ImageColor3 = getPointer(color_3);
end
function property.abi.prop_image_button_image_rect_offset(instance)
	return createPointer(getPointer(instance).ImageRectOffset);
end
function property.abi.prop_set_image_button_image_rect_offset(instance, vector_2)
	getPointer(instance).ImageRectOffset = getPointer(vector_2);
end
function property.abi.prop_image_button_image_rect_size(instance)
	return createPointer(getPointer(instance).ImageRectSize);
end
function property.abi.prop_set_image_button_image_rect_size(instance, vector_2)
	getPointer(instance).ImageRectSize = getPointer(vector_2);
end
function property.abi.prop_image_button_image_transparency(instance)
	return getPointer(instance).ImageTransparency;
end
function property.abi.prop_set_image_button_image_transparency(instance, number)
	getPointer(instance).ImageTransparency = number;
end
function property.abi.prop_image_button_is_loaded(instance)
	return getPointer(instance).IsLoaded and 1 or 0;
end
function property.abi.prop_image_button_pressed_image(instance)
	return createPointer(getPointer(instance).PressedImage);
end
function property.abi.prop_set_image_button_pressed_image(instance, content)
	getPointer(instance).PressedImage = getPointer(content);
end
function property.abi.prop_image_button_resample_mode(instance)
end
function property.abi.prop_image_button_scale_type(instance)
end
function property.abi.prop_image_button_slice_center(instance)
	return createPointer(getPointer(instance).SliceCenter);
end
function property.abi.prop_set_image_button_slice_center(instance, rect)
	getPointer(instance).SliceCenter = getPointer(rect);
end
function property.abi.prop_image_button_slice_scale(instance)
	return getPointer(instance).SliceScale;
end
function property.abi.prop_set_image_button_slice_scale(instance, number)
	getPointer(instance).SliceScale = number;
end
function property.abi.prop_image_button_tile_size(instance)
	return createPointer(getPointer(instance).TileSize);
end
function property.abi.prop_set_image_button_tile_size(instance, u_dim_2)
	getPointer(instance).TileSize = getPointer(u_dim_2);
end
function property.abi.prop_text_button_content_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).ContentText);
end
function property.abi.prop_text_button_font(instance)
end
function property.abi.prop_text_button_font_face(instance)
	return createPointer(getPointer(instance).FontFace);
end
function property.abi.prop_set_text_button_font_face(instance, font)
	getPointer(instance).FontFace = getPointer(font);
end
function property.abi.prop_text_button_font_size(instance)
end
function property.abi.prop_text_button_line_height(instance)
	return getPointer(instance).LineHeight;
end
function property.abi.prop_set_text_button_line_height(instance, number)
	getPointer(instance).LineHeight = number;
end
function property.abi.prop_text_button_localized_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).LocalizedText);
end
function property.abi.prop_text_button_max_visible_graphemes(instance)
	return getPointer(instance).MaxVisibleGraphemes;
end
function property.abi.prop_set_text_button_max_visible_graphemes(instance, number)
	getPointer(instance).MaxVisibleGraphemes = number;
end
function property.abi.prop_text_button_rich_text(instance)
	return getPointer(instance).RichText and 1 or 0;
end
function property.abi.prop_set_text_button_rich_text(instance, bool)
	getPointer(instance).RichText = bool and 1 or 0;
end
function property.abi.prop_text_button_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).Text);
end
function property.abi.prop_set_text_button_text(instance, addr, length)
	getPointer(instance).Text = readString(addr, length);
end
function property.abi.prop_text_button_text_bounds(instance)
	return createPointer(getPointer(instance).TextBounds);
end
function property.abi.prop_text_button_text_color(instance)
	return createPointer(getPointer(instance).TextColor);
end
function property.abi.prop_set_text_button_text_color(instance, brick_color)
	getPointer(instance).TextColor = getPointer(brick_color);
end
function property.abi.prop_text_button_text_color_3(instance)
	return createPointer(getPointer(instance).TextColor3);
end
function property.abi.prop_set_text_button_text_color_3(instance, color_3)
	getPointer(instance).TextColor3 = getPointer(color_3);
end
function property.abi.prop_text_button_text_fits(instance)
	return getPointer(instance).TextFits and 1 or 0;
end
function property.abi.prop_text_button_text_scaled(instance)
	return getPointer(instance).TextScaled and 1 or 0;
end
function property.abi.prop_set_text_button_text_scaled(instance, bool)
	getPointer(instance).TextScaled = bool and 1 or 0;
end
function property.abi.prop_text_button_text_size(instance)
	return getPointer(instance).TextSize;
end
function property.abi.prop_set_text_button_text_size(instance, number)
	getPointer(instance).TextSize = number;
end
function property.abi.prop_text_button_text_stroke_color_3(instance)
	return createPointer(getPointer(instance).TextStrokeColor3);
end
function property.abi.prop_set_text_button_text_stroke_color_3(instance, color_3)
	getPointer(instance).TextStrokeColor3 = getPointer(color_3);
end
function property.abi.prop_text_button_text_stroke_transparency(instance)
	return getPointer(instance).TextStrokeTransparency;
end
function property.abi.prop_set_text_button_text_stroke_transparency(instance, number)
	getPointer(instance).TextStrokeTransparency = number;
end
function property.abi.prop_text_button_text_transparency(instance)
	return getPointer(instance).TextTransparency;
end
function property.abi.prop_set_text_button_text_transparency(instance, number)
	getPointer(instance).TextTransparency = number;
end
function property.abi.prop_text_button_text_truncate(instance)
end
function property.abi.prop_text_button_text_wrap(instance)
	return getPointer(instance).TextWrap and 1 or 0;
end
function property.abi.prop_set_text_button_text_wrap(instance, bool)
	getPointer(instance).TextWrap = bool and 1 or 0;
end
function property.abi.prop_text_button_text_wrapped(instance)
	return getPointer(instance).TextWrapped and 1 or 0;
end
function property.abi.prop_set_text_button_text_wrapped(instance, bool)
	getPointer(instance).TextWrapped = bool and 1 or 0;
end
function property.abi.prop_text_button_text_x_alignment(instance)
end
function property.abi.prop_text_button_text_y_alignment(instance)
end
function property.abi.prop_image_label_image(instance)
	return createPointer(getPointer(instance).Image);
end
function property.abi.prop_set_image_label_image(instance, content)
	getPointer(instance).Image = getPointer(content);
end
function property.abi.prop_image_label_image_color_3(instance)
	return createPointer(getPointer(instance).ImageColor3);
end
function property.abi.prop_set_image_label_image_color_3(instance, color_3)
	getPointer(instance).ImageColor3 = getPointer(color_3);
end
function property.abi.prop_image_label_image_rect_offset(instance)
	return createPointer(getPointer(instance).ImageRectOffset);
end
function property.abi.prop_set_image_label_image_rect_offset(instance, vector_2)
	getPointer(instance).ImageRectOffset = getPointer(vector_2);
end
function property.abi.prop_image_label_image_rect_size(instance)
	return createPointer(getPointer(instance).ImageRectSize);
end
function property.abi.prop_set_image_label_image_rect_size(instance, vector_2)
	getPointer(instance).ImageRectSize = getPointer(vector_2);
end
function property.abi.prop_image_label_image_transparency(instance)
	return getPointer(instance).ImageTransparency;
end
function property.abi.prop_set_image_label_image_transparency(instance, number)
	getPointer(instance).ImageTransparency = number;
end
function property.abi.prop_image_label_is_loaded(instance)
	return getPointer(instance).IsLoaded and 1 or 0;
end
function property.abi.prop_image_label_resample_mode(instance)
end
function property.abi.prop_image_label_scale_type(instance)
end
function property.abi.prop_image_label_slice_center(instance)
	return createPointer(getPointer(instance).SliceCenter);
end
function property.abi.prop_set_image_label_slice_center(instance, rect)
	getPointer(instance).SliceCenter = getPointer(rect);
end
function property.abi.prop_image_label_slice_scale(instance)
	return getPointer(instance).SliceScale;
end
function property.abi.prop_set_image_label_slice_scale(instance, number)
	getPointer(instance).SliceScale = number;
end
function property.abi.prop_image_label_tile_size(instance)
	return createPointer(getPointer(instance).TileSize);
end
function property.abi.prop_set_image_label_tile_size(instance, u_dim_2)
	getPointer(instance).TileSize = getPointer(u_dim_2);
end
function property.abi.prop_text_label_content_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).ContentText);
end
function property.abi.prop_text_label_font(instance)
end
function property.abi.prop_text_label_font_face(instance)
	return createPointer(getPointer(instance).FontFace);
end
function property.abi.prop_set_text_label_font_face(instance, font)
	getPointer(instance).FontFace = getPointer(font);
end
function property.abi.prop_text_label_font_size(instance)
end
function property.abi.prop_text_label_line_height(instance)
	return getPointer(instance).LineHeight;
end
function property.abi.prop_set_text_label_line_height(instance, number)
	getPointer(instance).LineHeight = number;
end
function property.abi.prop_text_label_localized_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).LocalizedText);
end
function property.abi.prop_text_label_max_visible_graphemes(instance)
	return getPointer(instance).MaxVisibleGraphemes;
end
function property.abi.prop_set_text_label_max_visible_graphemes(instance, number)
	getPointer(instance).MaxVisibleGraphemes = number;
end
function property.abi.prop_text_label_rich_text(instance)
	return getPointer(instance).RichText and 1 or 0;
end
function property.abi.prop_set_text_label_rich_text(instance, bool)
	getPointer(instance).RichText = bool and 1 or 0;
end
function property.abi.prop_text_label_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).Text);
end
function property.abi.prop_set_text_label_text(instance, addr, length)
	getPointer(instance).Text = readString(addr, length);
end
function property.abi.prop_text_label_text_bounds(instance)
	return createPointer(getPointer(instance).TextBounds);
end
function property.abi.prop_text_label_text_color(instance)
	return createPointer(getPointer(instance).TextColor);
end
function property.abi.prop_set_text_label_text_color(instance, brick_color)
	getPointer(instance).TextColor = getPointer(brick_color);
end
function property.abi.prop_text_label_text_color_3(instance)
	return createPointer(getPointer(instance).TextColor3);
end
function property.abi.prop_set_text_label_text_color_3(instance, color_3)
	getPointer(instance).TextColor3 = getPointer(color_3);
end
function property.abi.prop_text_label_text_fits(instance)
	return getPointer(instance).TextFits and 1 or 0;
end
function property.abi.prop_text_label_text_scaled(instance)
	return getPointer(instance).TextScaled and 1 or 0;
end
function property.abi.prop_set_text_label_text_scaled(instance, bool)
	getPointer(instance).TextScaled = bool and 1 or 0;
end
function property.abi.prop_text_label_text_size(instance)
	return getPointer(instance).TextSize;
end
function property.abi.prop_set_text_label_text_size(instance, number)
	getPointer(instance).TextSize = number;
end
function property.abi.prop_text_label_text_stroke_color_3(instance)
	return createPointer(getPointer(instance).TextStrokeColor3);
end
function property.abi.prop_set_text_label_text_stroke_color_3(instance, color_3)
	getPointer(instance).TextStrokeColor3 = getPointer(color_3);
end
function property.abi.prop_text_label_text_stroke_transparency(instance)
	return getPointer(instance).TextStrokeTransparency;
end
function property.abi.prop_set_text_label_text_stroke_transparency(instance, number)
	getPointer(instance).TextStrokeTransparency = number;
end
function property.abi.prop_text_label_text_transparency(instance)
	return getPointer(instance).TextTransparency;
end
function property.abi.prop_set_text_label_text_transparency(instance, number)
	getPointer(instance).TextTransparency = number;
end
function property.abi.prop_text_label_text_truncate(instance)
end
function property.abi.prop_text_label_text_wrap(instance)
	return getPointer(instance).TextWrap and 1 or 0;
end
function property.abi.prop_set_text_label_text_wrap(instance, bool)
	getPointer(instance).TextWrap = bool and 1 or 0;
end
function property.abi.prop_text_label_text_wrapped(instance)
	return getPointer(instance).TextWrapped and 1 or 0;
end
function property.abi.prop_set_text_label_text_wrapped(instance, bool)
	getPointer(instance).TextWrapped = bool and 1 or 0;
end
function property.abi.prop_text_label_text_x_alignment(instance)
end
function property.abi.prop_text_label_text_y_alignment(instance)
end
function property.abi.prop_scrolling_frame_absolute_canvas_size(instance)
	return createPointer(getPointer(instance).AbsoluteCanvasSize);
end
function property.abi.prop_scrolling_frame_absolute_window_size(instance)
	return createPointer(getPointer(instance).AbsoluteWindowSize);
end
function property.abi.prop_scrolling_frame_automatic_canvas_size(instance)
end
function property.abi.prop_scrolling_frame_bottom_image(instance)
	return createPointer(getPointer(instance).BottomImage);
end
function property.abi.prop_set_scrolling_frame_bottom_image(instance, content)
	getPointer(instance).BottomImage = getPointer(content);
end
function property.abi.prop_scrolling_frame_canvas_position(instance)
	return createPointer(getPointer(instance).CanvasPosition);
end
function property.abi.prop_set_scrolling_frame_canvas_position(instance, vector_2)
	getPointer(instance).CanvasPosition = getPointer(vector_2);
end
function property.abi.prop_scrolling_frame_canvas_size(instance)
	return createPointer(getPointer(instance).CanvasSize);
end
function property.abi.prop_set_scrolling_frame_canvas_size(instance, u_dim_2)
	getPointer(instance).CanvasSize = getPointer(u_dim_2);
end
function property.abi.prop_scrolling_frame_elastic_behavior(instance)
end
function property.abi.prop_scrolling_frame_horizontal_scroll_bar_inset(instance)
end
function property.abi.prop_scrolling_frame_mid_image(instance)
	return createPointer(getPointer(instance).MidImage);
end
function property.abi.prop_set_scrolling_frame_mid_image(instance, content)
	getPointer(instance).MidImage = getPointer(content);
end
function property.abi.prop_scrolling_frame_scroll_bar_image_color_3(instance)
	return createPointer(getPointer(instance).ScrollBarImageColor3);
end
function property.abi.prop_set_scrolling_frame_scroll_bar_image_color_3(instance, color_3)
	getPointer(instance).ScrollBarImageColor3 = getPointer(color_3);
end
function property.abi.prop_scrolling_frame_scroll_bar_image_transparency(instance)
	return getPointer(instance).ScrollBarImageTransparency;
end
function property.abi.prop_set_scrolling_frame_scroll_bar_image_transparency(instance, number)
	getPointer(instance).ScrollBarImageTransparency = number;
end
function property.abi.prop_scrolling_frame_scroll_bar_thickness(instance)
	return getPointer(instance).ScrollBarThickness;
end
function property.abi.prop_set_scrolling_frame_scroll_bar_thickness(instance, number)
	getPointer(instance).ScrollBarThickness = number;
end
function property.abi.prop_scrolling_frame_scrolling_direction(instance)
end
function property.abi.prop_scrolling_frame_scrolling_enabled(instance)
	return getPointer(instance).ScrollingEnabled and 1 or 0;
end
function property.abi.prop_set_scrolling_frame_scrolling_enabled(instance, bool)
	getPointer(instance).ScrollingEnabled = bool and 1 or 0;
end
function property.abi.prop_scrolling_frame_top_image(instance)
	return createPointer(getPointer(instance).TopImage);
end
function property.abi.prop_set_scrolling_frame_top_image(instance, content)
	getPointer(instance).TopImage = getPointer(content);
end
function property.abi.prop_scrolling_frame_vertical_scroll_bar_inset(instance)
end
function property.abi.prop_scrolling_frame_vertical_scroll_bar_position(instance)
end
function property.abi.prop_text_box_clear_text_on_focus(instance)
	return getPointer(instance).ClearTextOnFocus and 1 or 0;
end
function property.abi.prop_set_text_box_clear_text_on_focus(instance, bool)
	getPointer(instance).ClearTextOnFocus = bool and 1 or 0;
end
function property.abi.prop_text_box_content_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).ContentText);
end
function property.abi.prop_text_box_cursor_position(instance)
	return getPointer(instance).CursorPosition;
end
function property.abi.prop_set_text_box_cursor_position(instance, number)
	getPointer(instance).CursorPosition = number;
end
function property.abi.prop_text_box_font(instance)
end
function property.abi.prop_text_box_font_face(instance)
	return createPointer(getPointer(instance).FontFace);
end
function property.abi.prop_set_text_box_font_face(instance, font)
	getPointer(instance).FontFace = getPointer(font);
end
function property.abi.prop_text_box_font_size(instance)
end
function property.abi.prop_text_box_line_height(instance)
	return getPointer(instance).LineHeight;
end
function property.abi.prop_set_text_box_line_height(instance, number)
	getPointer(instance).LineHeight = number;
end
function property.abi.prop_text_box_max_visible_graphemes(instance)
	return getPointer(instance).MaxVisibleGraphemes;
end
function property.abi.prop_set_text_box_max_visible_graphemes(instance, number)
	getPointer(instance).MaxVisibleGraphemes = number;
end
function property.abi.prop_text_box_multi_line(instance)
	return getPointer(instance).MultiLine and 1 or 0;
end
function property.abi.prop_set_text_box_multi_line(instance, bool)
	getPointer(instance).MultiLine = bool and 1 or 0;
end
function property.abi.prop_text_box_placeholder_color_3(instance)
	return createPointer(getPointer(instance).PlaceholderColor3);
end
function property.abi.prop_set_text_box_placeholder_color_3(instance, color_3)
	getPointer(instance).PlaceholderColor3 = getPointer(color_3);
end
function property.abi.prop_text_box_placeholder_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).PlaceholderText);
end
function property.abi.prop_set_text_box_placeholder_text(instance, addr, length)
	getPointer(instance).PlaceholderText = readString(addr, length);
end
function property.abi.prop_text_box_rich_text(instance)
	return getPointer(instance).RichText and 1 or 0;
end
function property.abi.prop_set_text_box_rich_text(instance, bool)
	getPointer(instance).RichText = bool and 1 or 0;
end
function property.abi.prop_text_box_selection_start(instance)
	return getPointer(instance).SelectionStart;
end
function property.abi.prop_set_text_box_selection_start(instance, number)
	getPointer(instance).SelectionStart = number;
end
function property.abi.prop_text_box_show_native_input(instance)
	return getPointer(instance).ShowNativeInput and 1 or 0;
end
function property.abi.prop_set_text_box_show_native_input(instance, bool)
	getPointer(instance).ShowNativeInput = bool and 1 or 0;
end
function property.abi.prop_text_box_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).Text);
end
function property.abi.prop_set_text_box_text(instance, addr, length)
	getPointer(instance).Text = readString(addr, length);
end
function property.abi.prop_text_box_text_bounds(instance)
	return createPointer(getPointer(instance).TextBounds);
end
function property.abi.prop_text_box_text_color(instance)
	return createPointer(getPointer(instance).TextColor);
end
function property.abi.prop_set_text_box_text_color(instance, brick_color)
	getPointer(instance).TextColor = getPointer(brick_color);
end
function property.abi.prop_text_box_text_color_3(instance)
	return createPointer(getPointer(instance).TextColor3);
end
function property.abi.prop_set_text_box_text_color_3(instance, color_3)
	getPointer(instance).TextColor3 = getPointer(color_3);
end
function property.abi.prop_text_box_text_editable(instance)
	return getPointer(instance).TextEditable and 1 or 0;
end
function property.abi.prop_set_text_box_text_editable(instance, bool)
	getPointer(instance).TextEditable = bool and 1 or 0;
end
function property.abi.prop_text_box_text_fits(instance)
	return getPointer(instance).TextFits and 1 or 0;
end
function property.abi.prop_text_box_text_scaled(instance)
	return getPointer(instance).TextScaled and 1 or 0;
end
function property.abi.prop_set_text_box_text_scaled(instance, bool)
	getPointer(instance).TextScaled = bool and 1 or 0;
end
function property.abi.prop_text_box_text_size(instance)
	return getPointer(instance).TextSize;
end
function property.abi.prop_set_text_box_text_size(instance, number)
	getPointer(instance).TextSize = number;
end
function property.abi.prop_text_box_text_stroke_color_3(instance)
	return createPointer(getPointer(instance).TextStrokeColor3);
end
function property.abi.prop_set_text_box_text_stroke_color_3(instance, color_3)
	getPointer(instance).TextStrokeColor3 = getPointer(color_3);
end
function property.abi.prop_text_box_text_stroke_transparency(instance)
	return getPointer(instance).TextStrokeTransparency;
end
function property.abi.prop_set_text_box_text_stroke_transparency(instance, number)
	getPointer(instance).TextStrokeTransparency = number;
end
function property.abi.prop_text_box_text_transparency(instance)
	return getPointer(instance).TextTransparency;
end
function property.abi.prop_set_text_box_text_transparency(instance, number)
	getPointer(instance).TextTransparency = number;
end
function property.abi.prop_text_box_text_truncate(instance)
end
function property.abi.prop_text_box_text_wrap(instance)
	return getPointer(instance).TextWrap and 1 or 0;
end
function property.abi.prop_set_text_box_text_wrap(instance, bool)
	getPointer(instance).TextWrap = bool and 1 or 0;
end
function property.abi.prop_text_box_text_wrapped(instance)
	return getPointer(instance).TextWrapped and 1 or 0;
end
function property.abi.prop_set_text_box_text_wrapped(instance, bool)
	getPointer(instance).TextWrapped = bool and 1 or 0;
end
function property.abi.prop_text_box_text_x_alignment(instance)
end
function property.abi.prop_text_box_text_y_alignment(instance)
end
function property.abi.prop_video_frame_is_loaded(instance)
	return getPointer(instance).IsLoaded and 1 or 0;
end
function property.abi.prop_video_frame_looped(instance)
	return getPointer(instance).Looped and 1 or 0;
end
function property.abi.prop_set_video_frame_looped(instance, bool)
	getPointer(instance).Looped = bool and 1 or 0;
end
function property.abi.prop_video_frame_playing(instance)
	return getPointer(instance).Playing and 1 or 0;
end
function property.abi.prop_set_video_frame_playing(instance, bool)
	getPointer(instance).Playing = bool and 1 or 0;
end
function property.abi.prop_video_frame_resolution(instance)
	return createPointer(getPointer(instance).Resolution);
end
function property.abi.prop_video_frame_time_length(instance)
	return getPointer(instance).TimeLength;
end
function property.abi.prop_video_frame_time_position(instance)
	return getPointer(instance).TimePosition;
end
function property.abi.prop_set_video_frame_time_position(instance, number)
	getPointer(instance).TimePosition = number;
end
function property.abi.prop_video_frame_video(instance)
	return createPointer(getPointer(instance).Video);
end
function property.abi.prop_set_video_frame_video(instance, content)
	getPointer(instance).Video = getPointer(content);
end
function property.abi.prop_video_frame_volume(instance)
	return getPointer(instance).Volume;
end
function property.abi.prop_set_video_frame_volume(instance, number)
	getPointer(instance).Volume = number;
end
function property.abi.prop_viewport_frame_ambient(instance)
	return createPointer(getPointer(instance).Ambient);
end
function property.abi.prop_set_viewport_frame_ambient(instance, color_3)
	getPointer(instance).Ambient = getPointer(color_3);
end
function property.abi.prop_viewport_frame_current_camera(addr, instance)
	local value = getPointer(instance).CurrentCamera;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_viewport_frame_current_camera(instance, isSome, value)
	getPointer(instance).CurrentCamera = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_viewport_frame_image_color_3(instance)
	return createPointer(getPointer(instance).ImageColor3);
end
function property.abi.prop_set_viewport_frame_image_color_3(instance, color_3)
	getPointer(instance).ImageColor3 = getPointer(color_3);
end
function property.abi.prop_viewport_frame_image_transparency(instance)
	return getPointer(instance).ImageTransparency;
end
function property.abi.prop_set_viewport_frame_image_transparency(instance, number)
	getPointer(instance).ImageTransparency = number;
end
function property.abi.prop_viewport_frame_light_color(instance)
	return createPointer(getPointer(instance).LightColor);
end
function property.abi.prop_set_viewport_frame_light_color(instance, color_3)
	getPointer(instance).LightColor = getPointer(color_3);
end
function property.abi.prop_viewport_frame_light_direction(instance)
	return createPointer(getPointer(instance).LightDirection);
end
function property.abi.prop_set_viewport_frame_light_direction(instance, vector_3)
	getPointer(instance).LightDirection = getPointer(vector_3);
end
function property.abi.prop_layer_collector_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_layer_collector_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_layer_collector_reset_on_spawn(instance)
	return getPointer(instance).ResetOnSpawn and 1 or 0;
end
function property.abi.prop_set_layer_collector_reset_on_spawn(instance, bool)
	getPointer(instance).ResetOnSpawn = bool and 1 or 0;
end
function property.abi.prop_layer_collector_z_index_behavior(instance)
end
function property.abi.prop_billboard_gui_active(instance)
	return getPointer(instance).Active and 1 or 0;
end
function property.abi.prop_set_billboard_gui_active(instance, bool)
	getPointer(instance).Active = bool and 1 or 0;
end
function property.abi.prop_billboard_gui_adornee(addr, instance)
	local value = getPointer(instance).Adornee;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_billboard_gui_adornee(instance, isSome, value)
	getPointer(instance).Adornee = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_billboard_gui_always_on_top(instance)
	return getPointer(instance).AlwaysOnTop and 1 or 0;
end
function property.abi.prop_set_billboard_gui_always_on_top(instance, bool)
	getPointer(instance).AlwaysOnTop = bool and 1 or 0;
end
function property.abi.prop_billboard_gui_brightness(instance)
	return getPointer(instance).Brightness;
end
function property.abi.prop_set_billboard_gui_brightness(instance, number)
	getPointer(instance).Brightness = number;
end
function property.abi.prop_billboard_gui_clips_descendants(instance)
	return getPointer(instance).ClipsDescendants and 1 or 0;
end
function property.abi.prop_set_billboard_gui_clips_descendants(instance, bool)
	getPointer(instance).ClipsDescendants = bool and 1 or 0;
end
function property.abi.prop_billboard_gui_current_distance(instance)
	return getPointer(instance).CurrentDistance;
end
function property.abi.prop_billboard_gui_distance_lower_limit(instance)
	return getPointer(instance).DistanceLowerLimit;
end
function property.abi.prop_set_billboard_gui_distance_lower_limit(instance, number)
	getPointer(instance).DistanceLowerLimit = number;
end
function property.abi.prop_billboard_gui_distance_step(instance)
	return getPointer(instance).DistanceStep;
end
function property.abi.prop_set_billboard_gui_distance_step(instance, number)
	getPointer(instance).DistanceStep = number;
end
function property.abi.prop_billboard_gui_distance_upper_limit(instance)
	return getPointer(instance).DistanceUpperLimit;
end
function property.abi.prop_set_billboard_gui_distance_upper_limit(instance, number)
	getPointer(instance).DistanceUpperLimit = number;
end
function property.abi.prop_billboard_gui_extents_offset(instance)
	return createPointer(getPointer(instance).ExtentsOffset);
end
function property.abi.prop_set_billboard_gui_extents_offset(instance, vector_3)
	getPointer(instance).ExtentsOffset = getPointer(vector_3);
end
function property.abi.prop_billboard_gui_extents_offset_world_space(instance)
	return createPointer(getPointer(instance).ExtentsOffsetWorldSpace);
end
function property.abi.prop_set_billboard_gui_extents_offset_world_space(instance, vector_3)
	getPointer(instance).ExtentsOffsetWorldSpace = getPointer(vector_3);
end
function property.abi.prop_billboard_gui_light_influence(instance)
	return getPointer(instance).LightInfluence;
end
function property.abi.prop_set_billboard_gui_light_influence(instance, number)
	getPointer(instance).LightInfluence = number;
end
function property.abi.prop_billboard_gui_max_distance(instance)
	return getPointer(instance).MaxDistance;
end
function property.abi.prop_set_billboard_gui_max_distance(instance, number)
	getPointer(instance).MaxDistance = number;
end
function property.abi.prop_billboard_gui_player_to_hide_from(addr, instance)
	local value = getPointer(instance).PlayerToHideFrom;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_billboard_gui_player_to_hide_from(instance, isSome, value)
	getPointer(instance).PlayerToHideFrom = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_billboard_gui_size(instance)
	return createPointer(getPointer(instance).Size);
end
function property.abi.prop_set_billboard_gui_size(instance, u_dim_2)
	getPointer(instance).Size = getPointer(u_dim_2);
end
function property.abi.prop_billboard_gui_size_offset(instance)
	return createPointer(getPointer(instance).SizeOffset);
end
function property.abi.prop_set_billboard_gui_size_offset(instance, vector_2)
	getPointer(instance).SizeOffset = getPointer(vector_2);
end
function property.abi.prop_billboard_gui_studs_offset(instance)
	return createPointer(getPointer(instance).StudsOffset);
end
function property.abi.prop_set_billboard_gui_studs_offset(instance, vector_3)
	getPointer(instance).StudsOffset = getPointer(vector_3);
end
function property.abi.prop_billboard_gui_studs_offset_world_space(instance)
	return createPointer(getPointer(instance).StudsOffsetWorldSpace);
end
function property.abi.prop_set_billboard_gui_studs_offset_world_space(instance, vector_3)
	getPointer(instance).StudsOffsetWorldSpace = getPointer(vector_3);
end
function property.abi.prop_screen_gui_display_order(instance)
	return getPointer(instance).DisplayOrder;
end
function property.abi.prop_set_screen_gui_display_order(instance, number)
	getPointer(instance).DisplayOrder = number;
end
function property.abi.prop_screen_gui_ignore_gui_inset(instance)
	return getPointer(instance).IgnoreGuiInset and 1 or 0;
end
function property.abi.prop_set_screen_gui_ignore_gui_inset(instance, bool)
	getPointer(instance).IgnoreGuiInset = bool and 1 or 0;
end
function property.abi.prop_surface_gui_active(instance)
	return getPointer(instance).Active and 1 or 0;
end
function property.abi.prop_set_surface_gui_active(instance, bool)
	getPointer(instance).Active = bool and 1 or 0;
end
function property.abi.prop_surface_gui_adornee(addr, instance)
	local value = getPointer(instance).Adornee;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_surface_gui_adornee(instance, isSome, value)
	getPointer(instance).Adornee = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_surface_gui_always_on_top(instance)
	return getPointer(instance).AlwaysOnTop and 1 or 0;
end
function property.abi.prop_set_surface_gui_always_on_top(instance, bool)
	getPointer(instance).AlwaysOnTop = bool and 1 or 0;
end
function property.abi.prop_surface_gui_brightness(instance)
	return getPointer(instance).Brightness;
end
function property.abi.prop_set_surface_gui_brightness(instance, number)
	getPointer(instance).Brightness = number;
end
function property.abi.prop_surface_gui_canvas_size(instance)
	return createPointer(getPointer(instance).CanvasSize);
end
function property.abi.prop_set_surface_gui_canvas_size(instance, vector_2)
	getPointer(instance).CanvasSize = getPointer(vector_2);
end
function property.abi.prop_surface_gui_clips_descendants(instance)
	return getPointer(instance).ClipsDescendants and 1 or 0;
end
function property.abi.prop_set_surface_gui_clips_descendants(instance, bool)
	getPointer(instance).ClipsDescendants = bool and 1 or 0;
end
function property.abi.prop_surface_gui_face(instance)
end
function property.abi.prop_surface_gui_light_influence(instance)
	return getPointer(instance).LightInfluence;
end
function property.abi.prop_set_surface_gui_light_influence(instance, number)
	getPointer(instance).LightInfluence = number;
end
function property.abi.prop_surface_gui_pixels_per_stud(instance)
	return getPointer(instance).PixelsPerStud;
end
function property.abi.prop_set_surface_gui_pixels_per_stud(instance, number)
	getPointer(instance).PixelsPerStud = number;
end
function property.abi.prop_surface_gui_sizing_mode(instance)
end
function property.abi.prop_surface_gui_tool_punch_through_distance(instance)
	return getPointer(instance).ToolPunchThroughDistance;
end
function property.abi.prop_set_surface_gui_tool_punch_through_distance(instance, number)
	getPointer(instance).ToolPunchThroughDistance = number;
end
function property.abi.prop_surface_gui_z_offset(instance)
	return getPointer(instance).ZOffset;
end
function property.abi.prop_set_surface_gui_z_offset(instance, number)
	getPointer(instance).ZOffset = number;
end
function property.abi.prop_gui_base_3_d_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_gui_base_3_d_color(instance, brick_color)
	getPointer(instance).Color = getPointer(brick_color);
end
function property.abi.prop_gui_base_3_d_color_3(instance)
	return createPointer(getPointer(instance).Color3);
end
function property.abi.prop_set_gui_base_3_d_color_3(instance, color_3)
	getPointer(instance).Color3 = getPointer(color_3);
end
function property.abi.prop_gui_base_3_d_transparency(instance)
	return getPointer(instance).Transparency;
end
function property.abi.prop_set_gui_base_3_d_transparency(instance, number)
	getPointer(instance).Transparency = number;
end
function property.abi.prop_gui_base_3_d_visible(instance)
	return getPointer(instance).Visible and 1 or 0;
end
function property.abi.prop_set_gui_base_3_d_visible(instance, bool)
	getPointer(instance).Visible = bool and 1 or 0;
end
function property.abi.prop_floor_wire_cycle_offset(instance)
	return getPointer(instance).CycleOffset;
end
function property.abi.prop_set_floor_wire_cycle_offset(instance, number)
	getPointer(instance).CycleOffset = number;
end
function property.abi.prop_floor_wire_from(addr, instance)
	local value = getPointer(instance).From;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_floor_wire_from(instance, isSome, value)
	getPointer(instance).From = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_floor_wire_studs_between_textures(instance)
	return getPointer(instance).StudsBetweenTextures;
end
function property.abi.prop_set_floor_wire_studs_between_textures(instance, number)
	getPointer(instance).StudsBetweenTextures = number;
end
function property.abi.prop_floor_wire_texture(instance)
	return createPointer(getPointer(instance).Texture);
end
function property.abi.prop_set_floor_wire_texture(instance, content)
	getPointer(instance).Texture = getPointer(content);
end
function property.abi.prop_floor_wire_texture_size(instance)
	return createPointer(getPointer(instance).TextureSize);
end
function property.abi.prop_set_floor_wire_texture_size(instance, vector_2)
	getPointer(instance).TextureSize = getPointer(vector_2);
end
function property.abi.prop_floor_wire_to(addr, instance)
	local value = getPointer(instance).To;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_floor_wire_to(instance, isSome, value)
	getPointer(instance).To = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_floor_wire_velocity(instance)
	return getPointer(instance).Velocity;
end
function property.abi.prop_set_floor_wire_velocity(instance, number)
	getPointer(instance).Velocity = number;
end
function property.abi.prop_floor_wire_wire_radius(instance)
	return getPointer(instance).WireRadius;
end
function property.abi.prop_set_floor_wire_wire_radius(instance, number)
	getPointer(instance).WireRadius = number;
end
function property.abi.prop_instance_adornment_adornee(addr, instance)
	local value = getPointer(instance).Adornee;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_instance_adornment_adornee(instance, isSome, value)
	getPointer(instance).Adornee = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_selection_box_line_thickness(instance)
	return getPointer(instance).LineThickness;
end
function property.abi.prop_set_selection_box_line_thickness(instance, number)
	getPointer(instance).LineThickness = number;
end
function property.abi.prop_selection_box_surface_color(instance)
	return createPointer(getPointer(instance).SurfaceColor);
end
function property.abi.prop_set_selection_box_surface_color(instance, brick_color)
	getPointer(instance).SurfaceColor = getPointer(brick_color);
end
function property.abi.prop_selection_box_surface_color_3(instance)
	return createPointer(getPointer(instance).SurfaceColor3);
end
function property.abi.prop_set_selection_box_surface_color_3(instance, color_3)
	getPointer(instance).SurfaceColor3 = getPointer(color_3);
end
function property.abi.prop_selection_box_surface_transparency(instance)
	return getPointer(instance).SurfaceTransparency;
end
function property.abi.prop_set_selection_box_surface_transparency(instance, number)
	getPointer(instance).SurfaceTransparency = number;
end
function property.abi.prop_pv_adornment_adornee(addr, instance)
	local value = getPointer(instance).Adornee;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_pv_adornment_adornee(instance, isSome, value)
	getPointer(instance).Adornee = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_handle_adornment_adorn_culling_mode(instance)
end
function property.abi.prop_handle_adornment_always_on_top(instance)
	return getPointer(instance).AlwaysOnTop and 1 or 0;
end
function property.abi.prop_set_handle_adornment_always_on_top(instance, bool)
	getPointer(instance).AlwaysOnTop = bool and 1 or 0;
end
function property.abi.prop_handle_adornment_c_frame(instance)
	return createPointer(getPointer(instance).CFrame);
end
function property.abi.prop_set_handle_adornment_c_frame(instance, c_frame)
	getPointer(instance).CFrame = getPointer(c_frame);
end
function property.abi.prop_handle_adornment_size_relative_offset(instance)
	return createPointer(getPointer(instance).SizeRelativeOffset);
end
function property.abi.prop_set_handle_adornment_size_relative_offset(instance, vector_3)
	getPointer(instance).SizeRelativeOffset = getPointer(vector_3);
end
function property.abi.prop_handle_adornment_z_index(instance)
	return getPointer(instance).ZIndex;
end
function property.abi.prop_set_handle_adornment_z_index(instance, number)
	getPointer(instance).ZIndex = number;
end
function property.abi.prop_box_handle_adornment_size(instance)
	return createPointer(getPointer(instance).Size);
end
function property.abi.prop_set_box_handle_adornment_size(instance, vector_3)
	getPointer(instance).Size = getPointer(vector_3);
end
function property.abi.prop_cone_handle_adornment_height(instance)
	return getPointer(instance).Height;
end
function property.abi.prop_set_cone_handle_adornment_height(instance, number)
	getPointer(instance).Height = number;
end
function property.abi.prop_cone_handle_adornment_radius(instance)
	return getPointer(instance).Radius;
end
function property.abi.prop_set_cone_handle_adornment_radius(instance, number)
	getPointer(instance).Radius = number;
end
function property.abi.prop_cylinder_handle_adornment_angle(instance)
	return getPointer(instance).Angle;
end
function property.abi.prop_set_cylinder_handle_adornment_angle(instance, number)
	getPointer(instance).Angle = number;
end
function property.abi.prop_cylinder_handle_adornment_height(instance)
	return getPointer(instance).Height;
end
function property.abi.prop_set_cylinder_handle_adornment_height(instance, number)
	getPointer(instance).Height = number;
end
function property.abi.prop_cylinder_handle_adornment_inner_radius(instance)
	return getPointer(instance).InnerRadius;
end
function property.abi.prop_set_cylinder_handle_adornment_inner_radius(instance, number)
	getPointer(instance).InnerRadius = number;
end
function property.abi.prop_cylinder_handle_adornment_radius(instance)
	return getPointer(instance).Radius;
end
function property.abi.prop_set_cylinder_handle_adornment_radius(instance, number)
	getPointer(instance).Radius = number;
end
function property.abi.prop_image_handle_adornment_image(instance)
	return createPointer(getPointer(instance).Image);
end
function property.abi.prop_set_image_handle_adornment_image(instance, content)
	getPointer(instance).Image = getPointer(content);
end
function property.abi.prop_image_handle_adornment_size(instance)
	return createPointer(getPointer(instance).Size);
end
function property.abi.prop_set_image_handle_adornment_size(instance, vector_2)
	getPointer(instance).Size = getPointer(vector_2);
end
function property.abi.prop_line_handle_adornment_length(instance)
	return getPointer(instance).Length;
end
function property.abi.prop_set_line_handle_adornment_length(instance, number)
	getPointer(instance).Length = number;
end
function property.abi.prop_line_handle_adornment_thickness(instance)
	return getPointer(instance).Thickness;
end
function property.abi.prop_set_line_handle_adornment_thickness(instance, number)
	getPointer(instance).Thickness = number;
end
function property.abi.prop_sphere_handle_adornment_radius(instance)
	return getPointer(instance).Radius;
end
function property.abi.prop_set_sphere_handle_adornment_radius(instance, number)
	getPointer(instance).Radius = number;
end
function property.abi.prop_selection_sphere_surface_color(instance)
	return createPointer(getPointer(instance).SurfaceColor);
end
function property.abi.prop_set_selection_sphere_surface_color(instance, brick_color)
	getPointer(instance).SurfaceColor = getPointer(brick_color);
end
function property.abi.prop_selection_sphere_surface_color_3(instance)
	return createPointer(getPointer(instance).SurfaceColor3);
end
function property.abi.prop_set_selection_sphere_surface_color_3(instance, color_3)
	getPointer(instance).SurfaceColor3 = getPointer(color_3);
end
function property.abi.prop_selection_sphere_surface_transparency(instance)
	return getPointer(instance).SurfaceTransparency;
end
function property.abi.prop_set_selection_sphere_surface_transparency(instance, number)
	getPointer(instance).SurfaceTransparency = number;
end
function property.abi.prop_part_adornment_adornee(addr, instance)
	local value = getPointer(instance).Adornee;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_part_adornment_adornee(instance, isSome, value)
	getPointer(instance).Adornee = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_arc_handles_axes(instance)
	return createPointer(getPointer(instance).Axes);
end
function property.abi.prop_set_arc_handles_axes(instance, axes)
	getPointer(instance).Axes = getPointer(axes);
end
function property.abi.prop_handles_faces(instance)
	return createPointer(getPointer(instance).Faces);
end
function property.abi.prop_set_handles_faces(instance, faces)
	getPointer(instance).Faces = getPointer(faces);
end
function property.abi.prop_handles_style(instance)
end
function property.abi.prop_surface_selection_target_surface(instance)
end
function property.abi.prop_selection_lasso_humanoid(addr, instance)
	local value = getPointer(instance).Humanoid;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_selection_lasso_humanoid(instance, isSome, value)
	getPointer(instance).Humanoid = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_selection_part_lasso_part(addr, instance)
	local value = getPointer(instance).Part;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_selection_part_lasso_part(instance, isSome, value)
	getPointer(instance).Part = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_selection_point_lasso_point(instance)
	return createPointer(getPointer(instance).Point);
end
function property.abi.prop_set_selection_point_lasso_point(instance, vector_3)
	getPointer(instance).Point = getPointer(vector_3);
end
function property.abi.prop_gui_service_auto_select_gui_enabled(instance)
	return getPointer(instance).AutoSelectGuiEnabled and 1 or 0;
end
function property.abi.prop_set_gui_service_auto_select_gui_enabled(instance, bool)
	getPointer(instance).AutoSelectGuiEnabled = bool and 1 or 0;
end
function property.abi.prop_gui_service_core_gui_navigation_enabled(instance)
	return getPointer(instance).CoreGuiNavigationEnabled and 1 or 0;
end
function property.abi.prop_set_gui_service_core_gui_navigation_enabled(instance, bool)
	getPointer(instance).CoreGuiNavigationEnabled = bool and 1 or 0;
end
function property.abi.prop_gui_service_gui_navigation_enabled(instance)
	return getPointer(instance).GuiNavigationEnabled and 1 or 0;
end
function property.abi.prop_set_gui_service_gui_navigation_enabled(instance, bool)
	getPointer(instance).GuiNavigationEnabled = bool and 1 or 0;
end
function property.abi.prop_gui_service_is_modal_dialog(instance)
	return getPointer(instance).IsModalDialog and 1 or 0;
end
function property.abi.prop_gui_service_is_windows(instance)
	return getPointer(instance).IsWindows and 1 or 0;
end
function property.abi.prop_gui_service_menu_is_open(instance)
	return getPointer(instance).MenuIsOpen and 1 or 0;
end
function property.abi.prop_gui_service_selected_object(addr, instance)
	local value = getPointer(instance).SelectedObject;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_gui_service_selected_object(instance, isSome, value)
	getPointer(instance).SelectedObject = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_gui_service_touch_controls_enabled(instance)
	return getPointer(instance).TouchControlsEnabled and 1 or 0;
end
function property.abi.prop_set_gui_service_touch_controls_enabled(instance, bool)
	getPointer(instance).TouchControlsEnabled = bool and 1 or 0;
end
function property.abi.prop_highlight_adornee(addr, instance)
	local value = getPointer(instance).Adornee;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_highlight_adornee(instance, isSome, value)
	getPointer(instance).Adornee = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_highlight_depth_mode(instance)
end
function property.abi.prop_highlight_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_highlight_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_highlight_fill_color(instance)
	return createPointer(getPointer(instance).FillColor);
end
function property.abi.prop_set_highlight_fill_color(instance, color_3)
	getPointer(instance).FillColor = getPointer(color_3);
end
function property.abi.prop_highlight_fill_transparency(instance)
	return getPointer(instance).FillTransparency;
end
function property.abi.prop_set_highlight_fill_transparency(instance, number)
	getPointer(instance).FillTransparency = number;
end
function property.abi.prop_highlight_outline_color(instance)
	return createPointer(getPointer(instance).OutlineColor);
end
function property.abi.prop_set_highlight_outline_color(instance, color_3)
	getPointer(instance).OutlineColor = getPointer(color_3);
end
function property.abi.prop_highlight_outline_transparency(instance)
	return getPointer(instance).OutlineTransparency;
end
function property.abi.prop_set_highlight_outline_transparency(instance, number)
	getPointer(instance).OutlineTransparency = number;
end
function property.abi.prop_humanoid_auto_jump_enabled(instance)
	return getPointer(instance).AutoJumpEnabled and 1 or 0;
end
function property.abi.prop_set_humanoid_auto_jump_enabled(instance, bool)
	getPointer(instance).AutoJumpEnabled = bool and 1 or 0;
end
function property.abi.prop_humanoid_auto_rotate(instance)
	return getPointer(instance).AutoRotate and 1 or 0;
end
function property.abi.prop_set_humanoid_auto_rotate(instance, bool)
	getPointer(instance).AutoRotate = bool and 1 or 0;
end
function property.abi.prop_humanoid_automatic_scaling_enabled(instance)
	return getPointer(instance).AutomaticScalingEnabled and 1 or 0;
end
function property.abi.prop_set_humanoid_automatic_scaling_enabled(instance, bool)
	getPointer(instance).AutomaticScalingEnabled = bool and 1 or 0;
end
function property.abi.prop_humanoid_break_joints_on_death(instance)
	return getPointer(instance).BreakJointsOnDeath and 1 or 0;
end
function property.abi.prop_set_humanoid_break_joints_on_death(instance, bool)
	getPointer(instance).BreakJointsOnDeath = bool and 1 or 0;
end
function property.abi.prop_humanoid_camera_offset(instance)
	return createPointer(getPointer(instance).CameraOffset);
end
function property.abi.prop_set_humanoid_camera_offset(instance, vector_3)
	getPointer(instance).CameraOffset = getPointer(vector_3);
end
function property.abi.prop_humanoid_collision_type(instance)
end
function property.abi.prop_humanoid_display_distance_type(instance)
end
function property.abi.prop_humanoid_display_name(addr, instance)
	writeStringReturn(addr, getPointer(instance).DisplayName);
end
function property.abi.prop_set_humanoid_display_name(instance, addr, length)
	getPointer(instance).DisplayName = readString(addr, length);
end
function property.abi.prop_humanoid_floor_material(instance)
end
function property.abi.prop_humanoid_health(instance)
	return getPointer(instance).Health;
end
function property.abi.prop_set_humanoid_health(instance, number)
	getPointer(instance).Health = number;
end
function property.abi.prop_humanoid_health_display_distance(instance)
	return getPointer(instance).HealthDisplayDistance;
end
function property.abi.prop_set_humanoid_health_display_distance(instance, number)
	getPointer(instance).HealthDisplayDistance = number;
end
function property.abi.prop_humanoid_health_display_type(instance)
end
function property.abi.prop_humanoid_hip_height(instance)
	return getPointer(instance).HipHeight;
end
function property.abi.prop_set_humanoid_hip_height(instance, number)
	getPointer(instance).HipHeight = number;
end
function property.abi.prop_humanoid_jump(instance)
	return getPointer(instance).Jump and 1 or 0;
end
function property.abi.prop_set_humanoid_jump(instance, bool)
	getPointer(instance).Jump = bool and 1 or 0;
end
function property.abi.prop_humanoid_jump_height(instance)
	return getPointer(instance).JumpHeight;
end
function property.abi.prop_set_humanoid_jump_height(instance, number)
	getPointer(instance).JumpHeight = number;
end
function property.abi.prop_humanoid_jump_power(instance)
	return getPointer(instance).JumpPower;
end
function property.abi.prop_set_humanoid_jump_power(instance, number)
	getPointer(instance).JumpPower = number;
end
function property.abi.prop_humanoid_left_leg(addr, instance)
	local value = getPointer(instance).LeftLeg;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_humanoid_left_leg(instance, isSome, value)
	getPointer(instance).LeftLeg = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_humanoid_max_health(instance)
	return getPointer(instance).MaxHealth;
end
function property.abi.prop_set_humanoid_max_health(instance, number)
	getPointer(instance).MaxHealth = number;
end
function property.abi.prop_humanoid_max_slope_angle(instance)
	return getPointer(instance).MaxSlopeAngle;
end
function property.abi.prop_set_humanoid_max_slope_angle(instance, number)
	getPointer(instance).MaxSlopeAngle = number;
end
function property.abi.prop_humanoid_move_direction(instance)
	return createPointer(getPointer(instance).MoveDirection);
end
function property.abi.prop_humanoid_name_display_distance(instance)
	return getPointer(instance).NameDisplayDistance;
end
function property.abi.prop_set_humanoid_name_display_distance(instance, number)
	getPointer(instance).NameDisplayDistance = number;
end
function property.abi.prop_humanoid_name_occlusion(instance)
end
function property.abi.prop_humanoid_platform_stand(instance)
	return getPointer(instance).PlatformStand and 1 or 0;
end
function property.abi.prop_set_humanoid_platform_stand(instance, bool)
	getPointer(instance).PlatformStand = bool and 1 or 0;
end
function property.abi.prop_humanoid_requires_neck(instance)
	return getPointer(instance).RequiresNeck and 1 or 0;
end
function property.abi.prop_set_humanoid_requires_neck(instance, bool)
	getPointer(instance).RequiresNeck = bool and 1 or 0;
end
function property.abi.prop_humanoid_rig_type(instance)
end
function property.abi.prop_humanoid_right_leg(addr, instance)
	local value = getPointer(instance).RightLeg;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_humanoid_right_leg(instance, isSome, value)
	getPointer(instance).RightLeg = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_humanoid_root_part(addr, instance)
	local value = getPointer(instance).RootPart;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_humanoid_seat_part(addr, instance)
	local value = getPointer(instance).SeatPart;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_humanoid_sit(instance)
	return getPointer(instance).Sit and 1 or 0;
end
function property.abi.prop_set_humanoid_sit(instance, bool)
	getPointer(instance).Sit = bool and 1 or 0;
end
function property.abi.prop_humanoid_target_point(instance)
	return createPointer(getPointer(instance).TargetPoint);
end
function property.abi.prop_set_humanoid_target_point(instance, vector_3)
	getPointer(instance).TargetPoint = getPointer(vector_3);
end
function property.abi.prop_humanoid_torso(addr, instance)
	local value = getPointer(instance).Torso;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_humanoid_torso(instance, isSome, value)
	getPointer(instance).Torso = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_humanoid_use_jump_power(instance)
	return getPointer(instance).UseJumpPower and 1 or 0;
end
function property.abi.prop_set_humanoid_use_jump_power(instance, bool)
	getPointer(instance).UseJumpPower = bool and 1 or 0;
end
function property.abi.prop_humanoid_walk_speed(instance)
	return getPointer(instance).WalkSpeed;
end
function property.abi.prop_set_humanoid_walk_speed(instance, number)
	getPointer(instance).WalkSpeed = number;
end
function property.abi.prop_humanoid_walk_to_part(addr, instance)
	local value = getPointer(instance).WalkToPart;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_humanoid_walk_to_part(instance, isSome, value)
	getPointer(instance).WalkToPart = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_humanoid_walk_to_point(instance)
	return createPointer(getPointer(instance).WalkToPoint);
end
function property.abi.prop_set_humanoid_walk_to_point(instance, vector_3)
	getPointer(instance).WalkToPoint = getPointer(vector_3);
end
function property.abi.prop_humanoid_description_accessory_blob(addr, instance)
	writeStringReturn(addr, getPointer(instance).AccessoryBlob);
end
function property.abi.prop_set_humanoid_description_accessory_blob(instance, addr, length)
	getPointer(instance).AccessoryBlob = readString(addr, length);
end
function property.abi.prop_humanoid_description_back_accessory(addr, instance)
	writeStringReturn(addr, getPointer(instance).BackAccessory);
end
function property.abi.prop_set_humanoid_description_back_accessory(instance, addr, length)
	getPointer(instance).BackAccessory = readString(addr, length);
end
function property.abi.prop_humanoid_description_body_type_scale(instance)
	return getPointer(instance).BodyTypeScale;
end
function property.abi.prop_set_humanoid_description_body_type_scale(instance, number)
	getPointer(instance).BodyTypeScale = number;
end
function property.abi.prop_humanoid_description_climb_animation(instance)
	return getPointer(instance).ClimbAnimation;
end
function property.abi.prop_set_humanoid_description_climb_animation(instance, number)
	getPointer(instance).ClimbAnimation = number;
end
function property.abi.prop_humanoid_description_depth_scale(instance)
	return getPointer(instance).DepthScale;
end
function property.abi.prop_set_humanoid_description_depth_scale(instance, number)
	getPointer(instance).DepthScale = number;
end
function property.abi.prop_humanoid_description_face(instance)
	return getPointer(instance).Face;
end
function property.abi.prop_set_humanoid_description_face(instance, number)
	getPointer(instance).Face = number;
end
function property.abi.prop_humanoid_description_face_accessory(addr, instance)
	writeStringReturn(addr, getPointer(instance).FaceAccessory);
end
function property.abi.prop_set_humanoid_description_face_accessory(instance, addr, length)
	getPointer(instance).FaceAccessory = readString(addr, length);
end
function property.abi.prop_humanoid_description_fall_animation(instance)
	return getPointer(instance).FallAnimation;
end
function property.abi.prop_set_humanoid_description_fall_animation(instance, number)
	getPointer(instance).FallAnimation = number;
end
function property.abi.prop_humanoid_description_front_accessory(addr, instance)
	writeStringReturn(addr, getPointer(instance).FrontAccessory);
end
function property.abi.prop_set_humanoid_description_front_accessory(instance, addr, length)
	getPointer(instance).FrontAccessory = readString(addr, length);
end
function property.abi.prop_humanoid_description_graphic_t_shirt(instance)
	return getPointer(instance).GraphicTShirt;
end
function property.abi.prop_set_humanoid_description_graphic_t_shirt(instance, number)
	getPointer(instance).GraphicTShirt = number;
end
function property.abi.prop_humanoid_description_hair_accessory(addr, instance)
	writeStringReturn(addr, getPointer(instance).HairAccessory);
end
function property.abi.prop_set_humanoid_description_hair_accessory(instance, addr, length)
	getPointer(instance).HairAccessory = readString(addr, length);
end
function property.abi.prop_humanoid_description_hat_accessory(addr, instance)
	writeStringReturn(addr, getPointer(instance).HatAccessory);
end
function property.abi.prop_set_humanoid_description_hat_accessory(instance, addr, length)
	getPointer(instance).HatAccessory = readString(addr, length);
end
function property.abi.prop_humanoid_description_head(instance)
	return getPointer(instance).Head;
end
function property.abi.prop_set_humanoid_description_head(instance, number)
	getPointer(instance).Head = number;
end
function property.abi.prop_humanoid_description_head_color(instance)
	return createPointer(getPointer(instance).HeadColor);
end
function property.abi.prop_set_humanoid_description_head_color(instance, color_3)
	getPointer(instance).HeadColor = getPointer(color_3);
end
function property.abi.prop_humanoid_description_head_scale(instance)
	return getPointer(instance).HeadScale;
end
function property.abi.prop_set_humanoid_description_head_scale(instance, number)
	getPointer(instance).HeadScale = number;
end
function property.abi.prop_humanoid_description_height_scale(instance)
	return getPointer(instance).HeightScale;
end
function property.abi.prop_set_humanoid_description_height_scale(instance, number)
	getPointer(instance).HeightScale = number;
end
function property.abi.prop_humanoid_description_idle_animation(instance)
	return getPointer(instance).IdleAnimation;
end
function property.abi.prop_set_humanoid_description_idle_animation(instance, number)
	getPointer(instance).IdleAnimation = number;
end
function property.abi.prop_humanoid_description_jump_animation(instance)
	return getPointer(instance).JumpAnimation;
end
function property.abi.prop_set_humanoid_description_jump_animation(instance, number)
	getPointer(instance).JumpAnimation = number;
end
function property.abi.prop_humanoid_description_left_arm(instance)
	return getPointer(instance).LeftArm;
end
function property.abi.prop_set_humanoid_description_left_arm(instance, number)
	getPointer(instance).LeftArm = number;
end
function property.abi.prop_humanoid_description_left_arm_color(instance)
	return createPointer(getPointer(instance).LeftArmColor);
end
function property.abi.prop_set_humanoid_description_left_arm_color(instance, color_3)
	getPointer(instance).LeftArmColor = getPointer(color_3);
end
function property.abi.prop_humanoid_description_left_leg(instance)
	return getPointer(instance).LeftLeg;
end
function property.abi.prop_set_humanoid_description_left_leg(instance, number)
	getPointer(instance).LeftLeg = number;
end
function property.abi.prop_humanoid_description_left_leg_color(instance)
	return createPointer(getPointer(instance).LeftLegColor);
end
function property.abi.prop_set_humanoid_description_left_leg_color(instance, color_3)
	getPointer(instance).LeftLegColor = getPointer(color_3);
end
function property.abi.prop_humanoid_description_neck_accessory(addr, instance)
	writeStringReturn(addr, getPointer(instance).NeckAccessory);
end
function property.abi.prop_set_humanoid_description_neck_accessory(instance, addr, length)
	getPointer(instance).NeckAccessory = readString(addr, length);
end
function property.abi.prop_humanoid_description_pants(instance)
	return getPointer(instance).Pants;
end
function property.abi.prop_set_humanoid_description_pants(instance, number)
	getPointer(instance).Pants = number;
end
function property.abi.prop_humanoid_description_proportion_scale(instance)
	return getPointer(instance).ProportionScale;
end
function property.abi.prop_set_humanoid_description_proportion_scale(instance, number)
	getPointer(instance).ProportionScale = number;
end
function property.abi.prop_humanoid_description_right_arm(instance)
	return getPointer(instance).RightArm;
end
function property.abi.prop_set_humanoid_description_right_arm(instance, number)
	getPointer(instance).RightArm = number;
end
function property.abi.prop_humanoid_description_right_arm_color(instance)
	return createPointer(getPointer(instance).RightArmColor);
end
function property.abi.prop_set_humanoid_description_right_arm_color(instance, color_3)
	getPointer(instance).RightArmColor = getPointer(color_3);
end
function property.abi.prop_humanoid_description_right_leg(instance)
	return getPointer(instance).RightLeg;
end
function property.abi.prop_set_humanoid_description_right_leg(instance, number)
	getPointer(instance).RightLeg = number;
end
function property.abi.prop_humanoid_description_right_leg_color(instance)
	return createPointer(getPointer(instance).RightLegColor);
end
function property.abi.prop_set_humanoid_description_right_leg_color(instance, color_3)
	getPointer(instance).RightLegColor = getPointer(color_3);
end
function property.abi.prop_humanoid_description_run_animation(instance)
	return getPointer(instance).RunAnimation;
end
function property.abi.prop_set_humanoid_description_run_animation(instance, number)
	getPointer(instance).RunAnimation = number;
end
function property.abi.prop_humanoid_description_shirt(instance)
	return getPointer(instance).Shirt;
end
function property.abi.prop_set_humanoid_description_shirt(instance, number)
	getPointer(instance).Shirt = number;
end
function property.abi.prop_humanoid_description_shoulders_accessory(addr, instance)
	writeStringReturn(addr, getPointer(instance).ShouldersAccessory);
end
function property.abi.prop_set_humanoid_description_shoulders_accessory(instance, addr, length)
	getPointer(instance).ShouldersAccessory = readString(addr, length);
end
function property.abi.prop_humanoid_description_swim_animation(instance)
	return getPointer(instance).SwimAnimation;
end
function property.abi.prop_set_humanoid_description_swim_animation(instance, number)
	getPointer(instance).SwimAnimation = number;
end
function property.abi.prop_humanoid_description_torso(instance)
	return getPointer(instance).Torso;
end
function property.abi.prop_set_humanoid_description_torso(instance, number)
	getPointer(instance).Torso = number;
end
function property.abi.prop_humanoid_description_torso_color(instance)
	return createPointer(getPointer(instance).TorsoColor);
end
function property.abi.prop_set_humanoid_description_torso_color(instance, color_3)
	getPointer(instance).TorsoColor = getPointer(color_3);
end
function property.abi.prop_humanoid_description_waist_accessory(addr, instance)
	writeStringReturn(addr, getPointer(instance).WaistAccessory);
end
function property.abi.prop_set_humanoid_description_waist_accessory(instance, addr, length)
	getPointer(instance).WaistAccessory = readString(addr, length);
end
function property.abi.prop_humanoid_description_walk_animation(instance)
	return getPointer(instance).WalkAnimation;
end
function property.abi.prop_set_humanoid_description_walk_animation(instance, number)
	getPointer(instance).WalkAnimation = number;
end
function property.abi.prop_humanoid_description_width_scale(instance)
	return getPointer(instance).WidthScale;
end
function property.abi.prop_set_humanoid_description_width_scale(instance, number)
	getPointer(instance).WidthScale = number;
end
function property.abi.prop_importer_base_settings_id(addr, instance)
	writeStringReturn(addr, getPointer(instance).Id);
end
function property.abi.prop_importer_base_settings_import_name(addr, instance)
	writeStringReturn(addr, getPointer(instance).ImportName);
end
function property.abi.prop_set_importer_base_settings_import_name(instance, addr, length)
	getPointer(instance).ImportName = readString(addr, length);
end
function property.abi.prop_importer_base_settings_should_import(instance)
	return getPointer(instance).ShouldImport and 1 or 0;
end
function property.abi.prop_set_importer_base_settings_should_import(instance, bool)
	getPointer(instance).ShouldImport = bool and 1 or 0;
end
function property.abi.prop_importer_group_settings_anchored(instance)
	return getPointer(instance).Anchored and 1 or 0;
end
function property.abi.prop_set_importer_group_settings_anchored(instance, bool)
	getPointer(instance).Anchored = bool and 1 or 0;
end
function property.abi.prop_importer_group_settings_import_as_model_asset(instance)
	return getPointer(instance).ImportAsModelAsset and 1 or 0;
end
function property.abi.prop_set_importer_group_settings_import_as_model_asset(instance, bool)
	getPointer(instance).ImportAsModelAsset = bool and 1 or 0;
end
function property.abi.prop_importer_group_settings_insert_in_workspace(instance)
	return getPointer(instance).InsertInWorkspace and 1 or 0;
end
function property.abi.prop_set_importer_group_settings_insert_in_workspace(instance, bool)
	getPointer(instance).InsertInWorkspace = bool and 1 or 0;
end
function property.abi.prop_importer_material_settings_diffuse_file_path(addr, instance)
	writeStringReturn(addr, getPointer(instance).DiffuseFilePath);
end
function property.abi.prop_importer_material_settings_is_pbr(instance)
	return getPointer(instance).IsPbr and 1 or 0;
end
function property.abi.prop_importer_material_settings_metalness_file_path(addr, instance)
	writeStringReturn(addr, getPointer(instance).MetalnessFilePath);
end
function property.abi.prop_importer_material_settings_normal_file_path(addr, instance)
	writeStringReturn(addr, getPointer(instance).NormalFilePath);
end
function property.abi.prop_importer_material_settings_roughness_file_path(addr, instance)
	writeStringReturn(addr, getPointer(instance).RoughnessFilePath);
end
function property.abi.prop_importer_mesh_settings_anchored(instance)
	return getPointer(instance).Anchored and 1 or 0;
end
function property.abi.prop_set_importer_mesh_settings_anchored(instance, bool)
	getPointer(instance).Anchored = bool and 1 or 0;
end
function property.abi.prop_importer_mesh_settings_cage_manifold(instance)
	return getPointer(instance).CageManifold and 1 or 0;
end
function property.abi.prop_importer_mesh_settings_cage_manifold_preview(instance)
	return getPointer(instance).CageManifoldPreview and 1 or 0;
end
function property.abi.prop_set_importer_mesh_settings_cage_manifold_preview(instance, bool)
	getPointer(instance).CageManifoldPreview = bool and 1 or 0;
end
function property.abi.prop_importer_mesh_settings_cage_no_overlapping_vertices(instance)
	return getPointer(instance).CageNoOverlappingVertices and 1 or 0;
end
function property.abi.prop_importer_mesh_settings_cage_no_overlapping_vertices_preview(instance)
	return getPointer(instance).CageNoOverlappingVerticesPreview and 1 or 0;
end
function property.abi.prop_set_importer_mesh_settings_cage_no_overlapping_vertices_preview(instance, bool)
	getPointer(instance).CageNoOverlappingVerticesPreview = bool and 1 or 0;
end
function property.abi.prop_importer_mesh_settings_cage_uv_matched(instance)
	return getPointer(instance).CageUVMatched and 1 or 0;
end
function property.abi.prop_importer_mesh_settings_cage_uv_matched_preview(instance)
	return getPointer(instance).CageUVMatchedPreview and 1 or 0;
end
function property.abi.prop_set_importer_mesh_settings_cage_uv_matched_preview(instance, bool)
	getPointer(instance).CageUVMatchedPreview = bool and 1 or 0;
end
function property.abi.prop_importer_mesh_settings_dimensions(instance)
	return createPointer(getPointer(instance).Dimensions);
end
function property.abi.prop_importer_mesh_settings_double_sided(instance)
	return getPointer(instance).DoubleSided and 1 or 0;
end
function property.abi.prop_set_importer_mesh_settings_double_sided(instance, bool)
	getPointer(instance).DoubleSided = bool and 1 or 0;
end
function property.abi.prop_importer_mesh_settings_ignore_vertex_colors(instance)
	return getPointer(instance).IgnoreVertexColors and 1 or 0;
end
function property.abi.prop_set_importer_mesh_settings_ignore_vertex_colors(instance, bool)
	getPointer(instance).IgnoreVertexColors = bool and 1 or 0;
end
function property.abi.prop_importer_mesh_settings_polygon_count(instance)
	return getPointer(instance).PolygonCount;
end
function property.abi.prop_importer_mesh_settings_use_imported_pivot(instance)
	return getPointer(instance).UseImportedPivot and 1 or 0;
end
function property.abi.prop_set_importer_mesh_settings_use_imported_pivot(instance, bool)
	getPointer(instance).UseImportedPivot = bool and 1 or 0;
end
function property.abi.prop_importer_root_settings_anchored(instance)
	return getPointer(instance).Anchored and 1 or 0;
end
function property.abi.prop_set_importer_root_settings_anchored(instance, bool)
	getPointer(instance).Anchored = bool and 1 or 0;
end
function property.abi.prop_importer_root_settings_file_dimensions(instance)
	return createPointer(getPointer(instance).FileDimensions);
end
function property.abi.prop_importer_root_settings_import_as_model_asset(instance)
	return getPointer(instance).ImportAsModelAsset and 1 or 0;
end
function property.abi.prop_set_importer_root_settings_import_as_model_asset(instance, bool)
	getPointer(instance).ImportAsModelAsset = bool and 1 or 0;
end
function property.abi.prop_importer_root_settings_insert_in_workspace(instance)
	return getPointer(instance).InsertInWorkspace and 1 or 0;
end
function property.abi.prop_set_importer_root_settings_insert_in_workspace(instance, bool)
	getPointer(instance).InsertInWorkspace = bool and 1 or 0;
end
function property.abi.prop_importer_root_settings_insert_with_scene_position(instance)
	return getPointer(instance).InsertWithScenePosition and 1 or 0;
end
function property.abi.prop_set_importer_root_settings_insert_with_scene_position(instance, bool)
	getPointer(instance).InsertWithScenePosition = bool and 1 or 0;
end
function property.abi.prop_importer_root_settings_invert_negative_faces(instance)
	return getPointer(instance).InvertNegativeFaces and 1 or 0;
end
function property.abi.prop_set_importer_root_settings_invert_negative_faces(instance, bool)
	getPointer(instance).InvertNegativeFaces = bool and 1 or 0;
end
function property.abi.prop_importer_root_settings_merge_meshes(instance)
	return getPointer(instance).MergeMeshes and 1 or 0;
end
function property.abi.prop_set_importer_root_settings_merge_meshes(instance, bool)
	getPointer(instance).MergeMeshes = bool and 1 or 0;
end
function property.abi.prop_importer_root_settings_polygon_count(instance)
	return getPointer(instance).PolygonCount;
end
function property.abi.prop_importer_root_settings_rig_type(instance)
end
function property.abi.prop_importer_root_settings_scale_unit(instance)
end
function property.abi.prop_importer_root_settings_use_scene_origin_as_pivot(instance)
	return getPointer(instance).UseSceneOriginAsPivot and 1 or 0;
end
function property.abi.prop_set_importer_root_settings_use_scene_origin_as_pivot(instance, bool)
	getPointer(instance).UseSceneOriginAsPivot = bool and 1 or 0;
end
function property.abi.prop_importer_root_settings_world_forward(instance)
end
function property.abi.prop_importer_root_settings_world_up(instance)
end
function property.abi.prop_input_object_delta(instance)
	return createPointer(getPointer(instance).Delta);
end
function property.abi.prop_set_input_object_delta(instance, vector_3)
	getPointer(instance).Delta = getPointer(vector_3);
end
function property.abi.prop_input_object_key_code(instance)
end
function property.abi.prop_input_object_position(instance)
	return createPointer(getPointer(instance).Position);
end
function property.abi.prop_set_input_object_position(instance, vector_3)
	getPointer(instance).Position = getPointer(vector_3);
end
function property.abi.prop_input_object_user_input_state(instance)
end
function property.abi.prop_input_object_user_input_type(instance)
end
function property.abi.prop_insert_service_allow_client_insert_models(instance)
	return getPointer(instance).AllowClientInsertModels and 1 or 0;
end
function property.abi.prop_set_insert_service_allow_client_insert_models(instance, bool)
	getPointer(instance).AllowClientInsertModels = bool and 1 or 0;
end
function property.abi.prop_insert_service_allow_insert_free_models(instance)
	return getPointer(instance).AllowInsertFreeModels and 1 or 0;
end
function property.abi.prop_set_insert_service_allow_insert_free_models(instance, bool)
	getPointer(instance).AllowInsertFreeModels = bool and 1 or 0;
end
function property.abi.prop_joint_instance_active(instance)
	return getPointer(instance).Active and 1 or 0;
end
function property.abi.prop_joint_instance_c_0(instance)
	return createPointer(getPointer(instance).C0);
end
function property.abi.prop_set_joint_instance_c_0(instance, c_frame)
	getPointer(instance).C0 = getPointer(c_frame);
end
function property.abi.prop_joint_instance_c_1(instance)
	return createPointer(getPointer(instance).C1);
end
function property.abi.prop_set_joint_instance_c_1(instance, c_frame)
	getPointer(instance).C1 = getPointer(c_frame);
end
function property.abi.prop_joint_instance_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_joint_instance_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_joint_instance_part_0(addr, instance)
	local value = getPointer(instance).Part0;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_joint_instance_part_0(instance, isSome, value)
	getPointer(instance).Part0 = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_joint_instance_part_1(addr, instance)
	local value = getPointer(instance).Part1;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_joint_instance_part_1(instance, isSome, value)
	getPointer(instance).Part1 = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_dynamic_rotate_base_angle(instance)
	return getPointer(instance).BaseAngle;
end
function property.abi.prop_set_dynamic_rotate_base_angle(instance, number)
	getPointer(instance).BaseAngle = number;
end
function property.abi.prop_glue_f_0(instance)
	return createPointer(getPointer(instance).F0);
end
function property.abi.prop_set_glue_f_0(instance, vector_3)
	getPointer(instance).F0 = getPointer(vector_3);
end
function property.abi.prop_glue_f_1(instance)
	return createPointer(getPointer(instance).F1);
end
function property.abi.prop_set_glue_f_1(instance, vector_3)
	getPointer(instance).F1 = getPointer(vector_3);
end
function property.abi.prop_glue_f_2(instance)
	return createPointer(getPointer(instance).F2);
end
function property.abi.prop_set_glue_f_2(instance, vector_3)
	getPointer(instance).F2 = getPointer(vector_3);
end
function property.abi.prop_glue_f_3(instance)
	return createPointer(getPointer(instance).F3);
end
function property.abi.prop_set_glue_f_3(instance, vector_3)
	getPointer(instance).F3 = getPointer(vector_3);
end
function property.abi.prop_motor_current_angle(instance)
	return getPointer(instance).CurrentAngle;
end
function property.abi.prop_set_motor_current_angle(instance, number)
	getPointer(instance).CurrentAngle = number;
end
function property.abi.prop_motor_desired_angle(instance)
	return getPointer(instance).DesiredAngle;
end
function property.abi.prop_set_motor_desired_angle(instance, number)
	getPointer(instance).DesiredAngle = number;
end
function property.abi.prop_motor_max_velocity(instance)
	return getPointer(instance).MaxVelocity;
end
function property.abi.prop_set_motor_max_velocity(instance, number)
	getPointer(instance).MaxVelocity = number;
end
function property.abi.prop_motor_6_d_child_name(addr, instance)
	writeStringReturn(addr, getPointer(instance).ChildName);
end
function property.abi.prop_motor_6_d_parent_name(addr, instance)
	writeStringReturn(addr, getPointer(instance).ParentName);
end
function property.abi.prop_motor_6_d_transform(instance)
	return createPointer(getPointer(instance).Transform);
end
function property.abi.prop_set_motor_6_d_transform(instance, c_frame)
	getPointer(instance).Transform = getPointer(c_frame);
end
function property.abi.prop_velocity_motor_current_angle(instance)
	return getPointer(instance).CurrentAngle;
end
function property.abi.prop_set_velocity_motor_current_angle(instance, number)
	getPointer(instance).CurrentAngle = number;
end
function property.abi.prop_velocity_motor_desired_angle(instance)
	return getPointer(instance).DesiredAngle;
end
function property.abi.prop_set_velocity_motor_desired_angle(instance, number)
	getPointer(instance).DesiredAngle = number;
end
function property.abi.prop_velocity_motor_max_velocity(instance)
	return getPointer(instance).MaxVelocity;
end
function property.abi.prop_set_velocity_motor_max_velocity(instance, number)
	getPointer(instance).MaxVelocity = number;
end
function property.abi.prop_keyframe_time(instance)
	return getPointer(instance).Time;
end
function property.abi.prop_set_keyframe_time(instance, number)
	getPointer(instance).Time = number;
end
function property.abi.prop_keyframe_marker_value(addr, instance)
	writeStringReturn(addr, getPointer(instance).Value);
end
function property.abi.prop_set_keyframe_marker_value(instance, addr, length)
	getPointer(instance).Value = readString(addr, length);
end
function property.abi.prop_light_brightness(instance)
	return getPointer(instance).Brightness;
end
function property.abi.prop_set_light_brightness(instance, number)
	getPointer(instance).Brightness = number;
end
function property.abi.prop_light_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_light_color(instance, color_3)
	getPointer(instance).Color = getPointer(color_3);
end
function property.abi.prop_light_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_light_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_light_shadows(instance)
	return getPointer(instance).Shadows and 1 or 0;
end
function property.abi.prop_set_light_shadows(instance, bool)
	getPointer(instance).Shadows = bool and 1 or 0;
end
function property.abi.prop_point_light_range(instance)
	return getPointer(instance).Range;
end
function property.abi.prop_set_point_light_range(instance, number)
	getPointer(instance).Range = number;
end
function property.abi.prop_spot_light_angle(instance)
	return getPointer(instance).Angle;
end
function property.abi.prop_set_spot_light_angle(instance, number)
	getPointer(instance).Angle = number;
end
function property.abi.prop_spot_light_face(instance)
end
function property.abi.prop_spot_light_range(instance)
	return getPointer(instance).Range;
end
function property.abi.prop_set_spot_light_range(instance, number)
	getPointer(instance).Range = number;
end
function property.abi.prop_surface_light_angle(instance)
	return getPointer(instance).Angle;
end
function property.abi.prop_set_surface_light_angle(instance, number)
	getPointer(instance).Angle = number;
end
function property.abi.prop_surface_light_face(instance)
end
function property.abi.prop_surface_light_range(instance)
	return getPointer(instance).Range;
end
function property.abi.prop_set_surface_light_range(instance, number)
	getPointer(instance).Range = number;
end
function property.abi.prop_lighting_ambient(instance)
	return createPointer(getPointer(instance).Ambient);
end
function property.abi.prop_set_lighting_ambient(instance, color_3)
	getPointer(instance).Ambient = getPointer(color_3);
end
function property.abi.prop_lighting_brightness(instance)
	return getPointer(instance).Brightness;
end
function property.abi.prop_set_lighting_brightness(instance, number)
	getPointer(instance).Brightness = number;
end
function property.abi.prop_lighting_clock_time(instance)
	return getPointer(instance).ClockTime;
end
function property.abi.prop_set_lighting_clock_time(instance, number)
	getPointer(instance).ClockTime = number;
end
function property.abi.prop_lighting_color_shift_bottom(instance)
	return createPointer(getPointer(instance).ColorShift_Bottom);
end
function property.abi.prop_set_lighting_color_shift_bottom(instance, color_3)
	getPointer(instance).ColorShift_Bottom = getPointer(color_3);
end
function property.abi.prop_lighting_color_shift_top(instance)
	return createPointer(getPointer(instance).ColorShift_Top);
end
function property.abi.prop_set_lighting_color_shift_top(instance, color_3)
	getPointer(instance).ColorShift_Top = getPointer(color_3);
end
function property.abi.prop_lighting_environment_diffuse_scale(instance)
	return getPointer(instance).EnvironmentDiffuseScale;
end
function property.abi.prop_set_lighting_environment_diffuse_scale(instance, number)
	getPointer(instance).EnvironmentDiffuseScale = number;
end
function property.abi.prop_lighting_environment_specular_scale(instance)
	return getPointer(instance).EnvironmentSpecularScale;
end
function property.abi.prop_set_lighting_environment_specular_scale(instance, number)
	getPointer(instance).EnvironmentSpecularScale = number;
end
function property.abi.prop_lighting_exposure_compensation(instance)
	return getPointer(instance).ExposureCompensation;
end
function property.abi.prop_set_lighting_exposure_compensation(instance, number)
	getPointer(instance).ExposureCompensation = number;
end
function property.abi.prop_lighting_fog_color(instance)
	return createPointer(getPointer(instance).FogColor);
end
function property.abi.prop_set_lighting_fog_color(instance, color_3)
	getPointer(instance).FogColor = getPointer(color_3);
end
function property.abi.prop_lighting_fog_end(instance)
	return getPointer(instance).FogEnd;
end
function property.abi.prop_set_lighting_fog_end(instance, number)
	getPointer(instance).FogEnd = number;
end
function property.abi.prop_lighting_fog_start(instance)
	return getPointer(instance).FogStart;
end
function property.abi.prop_set_lighting_fog_start(instance, number)
	getPointer(instance).FogStart = number;
end
function property.abi.prop_lighting_geographic_latitude(instance)
	return getPointer(instance).GeographicLatitude;
end
function property.abi.prop_set_lighting_geographic_latitude(instance, number)
	getPointer(instance).GeographicLatitude = number;
end
function property.abi.prop_lighting_global_shadows(instance)
	return getPointer(instance).GlobalShadows and 1 or 0;
end
function property.abi.prop_set_lighting_global_shadows(instance, bool)
	getPointer(instance).GlobalShadows = bool and 1 or 0;
end
function property.abi.prop_lighting_outdoor_ambient(instance)
	return createPointer(getPointer(instance).OutdoorAmbient);
end
function property.abi.prop_set_lighting_outdoor_ambient(instance, color_3)
	getPointer(instance).OutdoorAmbient = getPointer(color_3);
end
function property.abi.prop_lighting_outlines(instance)
	return getPointer(instance).Outlines and 1 or 0;
end
function property.abi.prop_set_lighting_outlines(instance, bool)
	getPointer(instance).Outlines = bool and 1 or 0;
end
function property.abi.prop_lighting_shadow_color(instance)
	return createPointer(getPointer(instance).ShadowColor);
end
function property.abi.prop_set_lighting_shadow_color(instance, color_3)
	getPointer(instance).ShadowColor = getPointer(color_3);
end
function property.abi.prop_lighting_shadow_softness(instance)
	return getPointer(instance).ShadowSoftness;
end
function property.abi.prop_set_lighting_shadow_softness(instance, number)
	getPointer(instance).ShadowSoftness = number;
end
function property.abi.prop_lighting_technology(instance)
end
function property.abi.prop_lighting_time_of_day(addr, instance)
	writeStringReturn(addr, getPointer(instance).TimeOfDay);
end
function property.abi.prop_set_lighting_time_of_day(instance, addr, length)
	getPointer(instance).TimeOfDay = readString(addr, length);
end
function property.abi.prop_localization_service_roblox_locale_id(addr, instance)
	writeStringReturn(addr, getPointer(instance).RobloxLocaleId);
end
function property.abi.prop_localization_service_system_locale_id(addr, instance)
	writeStringReturn(addr, getPointer(instance).SystemLocaleId);
end
function property.abi.prop_localization_table_development_language(addr, instance)
	writeStringReturn(addr, getPointer(instance).DevelopmentLanguage);
end
function property.abi.prop_set_localization_table_development_language(instance, addr, length)
	getPointer(instance).DevelopmentLanguage = readString(addr, length);
end
function property.abi.prop_localization_table_root(addr, instance)
	local value = getPointer(instance).Root;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_localization_table_root(instance, isSome, value)
	getPointer(instance).Root = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_localization_table_source_locale_id(addr, instance)
	writeStringReturn(addr, getPointer(instance).SourceLocaleId);
end
function property.abi.prop_set_localization_table_source_locale_id(instance, addr, length)
	getPointer(instance).SourceLocaleId = readString(addr, length);
end
function property.abi.prop_lod_data_entity_entity_lod_enabled(instance)
	return getPointer(instance).EntityLodEnabled and 1 or 0;
end
function property.abi.prop_set_lod_data_entity_entity_lod_enabled(instance, bool)
	getPointer(instance).EntityLodEnabled = bool and 1 or 0;
end
function property.abi.prop_lua_source_container_current_editor(addr, instance)
	local value = getPointer(instance).CurrentEditor;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_lua_source_container_current_editor(instance, isSome, value)
	getPointer(instance).CurrentEditor = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_base_script_disabled(instance)
	return getPointer(instance).Disabled and 1 or 0;
end
function property.abi.prop_set_base_script_disabled(instance, bool)
	getPointer(instance).Disabled = bool and 1 or 0;
end
function property.abi.prop_base_script_linked_source(instance)
	return createPointer(getPointer(instance).LinkedSource);
end
function property.abi.prop_set_base_script_linked_source(instance, content)
	getPointer(instance).LinkedSource = getPointer(content);
end
function property.abi.prop_module_script_linked_source(instance)
	return createPointer(getPointer(instance).LinkedSource);
end
function property.abi.prop_set_module_script_linked_source(instance, content)
	getPointer(instance).LinkedSource = getPointer(content);
end
function property.abi.prop_marker_curve_length(instance)
	return getPointer(instance).Length;
end
function property.abi.prop_material_variant_material_pattern(instance)
end
function property.abi.prop_material_variant_studs_per_tile(instance)
	return getPointer(instance).StudsPerTile;
end
function property.abi.prop_set_material_variant_studs_per_tile(instance, number)
	getPointer(instance).StudsPerTile = number;
end
function property.abi.prop_mouse_hit(instance)
	return createPointer(getPointer(instance).Hit);
end
function property.abi.prop_mouse_icon(instance)
	return createPointer(getPointer(instance).Icon);
end
function property.abi.prop_set_mouse_icon(instance, content)
	getPointer(instance).Icon = getPointer(content);
end
function property.abi.prop_mouse_origin(instance)
	return createPointer(getPointer(instance).Origin);
end
function property.abi.prop_mouse_target(addr, instance)
	local value = getPointer(instance).Target;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_mouse_target_filter(addr, instance)
	local value = getPointer(instance).TargetFilter;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_mouse_target_filter(instance, isSome, value)
	getPointer(instance).TargetFilter = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_mouse_target_surface(instance)
end
function property.abi.prop_mouse_unit_ray(instance)
	return createPointer(getPointer(instance).UnitRay);
end
function property.abi.prop_mouse_view_size_x(instance)
	return getPointer(instance).ViewSizeX;
end
function property.abi.prop_mouse_view_size_y(instance)
	return getPointer(instance).ViewSizeY;
end
function property.abi.prop_mouse_x(instance)
	return getPointer(instance).X;
end
function property.abi.prop_mouse_y(instance)
	return getPointer(instance).Y;
end
function property.abi.prop_no_collision_constraint_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_no_collision_constraint_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_no_collision_constraint_part_0(addr, instance)
	local value = getPointer(instance).Part0;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_no_collision_constraint_part_0(instance, isSome, value)
	getPointer(instance).Part0 = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_no_collision_constraint_part_1(addr, instance)
	local value = getPointer(instance).Part1;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_no_collision_constraint_part_1(instance, isSome, value)
	getPointer(instance).Part1 = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_base_part_anchored(instance)
	return getPointer(instance).Anchored and 1 or 0;
end
function property.abi.prop_set_base_part_anchored(instance, bool)
	getPointer(instance).Anchored = bool and 1 or 0;
end
function property.abi.prop_base_part_assembly_angular_velocity(instance)
	return createPointer(getPointer(instance).AssemblyAngularVelocity);
end
function property.abi.prop_set_base_part_assembly_angular_velocity(instance, vector_3)
	getPointer(instance).AssemblyAngularVelocity = getPointer(vector_3);
end
function property.abi.prop_base_part_assembly_center_of_mass(instance)
	return createPointer(getPointer(instance).AssemblyCenterOfMass);
end
function property.abi.prop_base_part_assembly_linear_velocity(instance)
	return createPointer(getPointer(instance).AssemblyLinearVelocity);
end
function property.abi.prop_set_base_part_assembly_linear_velocity(instance, vector_3)
	getPointer(instance).AssemblyLinearVelocity = getPointer(vector_3);
end
function property.abi.prop_base_part_assembly_mass(instance)
	return getPointer(instance).AssemblyMass;
end
function property.abi.prop_base_part_assembly_root_part(addr, instance)
	local value = getPointer(instance).AssemblyRootPart;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_base_part_back_param_a(instance)
	return getPointer(instance).BackParamA;
end
function property.abi.prop_set_base_part_back_param_a(instance, number)
	getPointer(instance).BackParamA = number;
end
function property.abi.prop_base_part_back_param_b(instance)
	return getPointer(instance).BackParamB;
end
function property.abi.prop_set_base_part_back_param_b(instance, number)
	getPointer(instance).BackParamB = number;
end
function property.abi.prop_base_part_back_surface(instance)
end
function property.abi.prop_base_part_back_surface_input(instance)
end
function property.abi.prop_base_part_bottom_param_a(instance)
	return getPointer(instance).BottomParamA;
end
function property.abi.prop_set_base_part_bottom_param_a(instance, number)
	getPointer(instance).BottomParamA = number;
end
function property.abi.prop_base_part_bottom_param_b(instance)
	return getPointer(instance).BottomParamB;
end
function property.abi.prop_set_base_part_bottom_param_b(instance, number)
	getPointer(instance).BottomParamB = number;
end
function property.abi.prop_base_part_bottom_surface(instance)
end
function property.abi.prop_base_part_bottom_surface_input(instance)
end
function property.abi.prop_base_part_brick_color(instance)
	return createPointer(getPointer(instance).BrickColor);
end
function property.abi.prop_set_base_part_brick_color(instance, brick_color)
	getPointer(instance).BrickColor = getPointer(brick_color);
end
function property.abi.prop_base_part_c_frame(instance)
	return createPointer(getPointer(instance).CFrame);
end
function property.abi.prop_set_base_part_c_frame(instance, c_frame)
	getPointer(instance).CFrame = getPointer(c_frame);
end
function property.abi.prop_base_part_can_collide(instance)
	return getPointer(instance).CanCollide and 1 or 0;
end
function property.abi.prop_set_base_part_can_collide(instance, bool)
	getPointer(instance).CanCollide = bool and 1 or 0;
end
function property.abi.prop_base_part_can_query(instance)
	return getPointer(instance).CanQuery and 1 or 0;
end
function property.abi.prop_set_base_part_can_query(instance, bool)
	getPointer(instance).CanQuery = bool and 1 or 0;
end
function property.abi.prop_base_part_can_touch(instance)
	return getPointer(instance).CanTouch and 1 or 0;
end
function property.abi.prop_set_base_part_can_touch(instance, bool)
	getPointer(instance).CanTouch = bool and 1 or 0;
end
function property.abi.prop_base_part_cast_shadow(instance)
	return getPointer(instance).CastShadow and 1 or 0;
end
function property.abi.prop_set_base_part_cast_shadow(instance, bool)
	getPointer(instance).CastShadow = bool and 1 or 0;
end
function property.abi.prop_base_part_center_of_mass(instance)
	return createPointer(getPointer(instance).CenterOfMass);
end
function property.abi.prop_base_part_collision_group_id(instance)
	return getPointer(instance).CollisionGroupId;
end
function property.abi.prop_set_base_part_collision_group_id(instance, number)
	getPointer(instance).CollisionGroupId = number;
end
function property.abi.prop_base_part_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_base_part_color(instance, color_3)
	getPointer(instance).Color = getPointer(color_3);
end
function property.abi.prop_base_part_custom_physical_properties(instance)
	return createPointer(getPointer(instance).CustomPhysicalProperties);
end
function property.abi.prop_set_base_part_custom_physical_properties(instance, physical_properties)
	getPointer(instance).CustomPhysicalProperties = getPointer(physical_properties);
end
function property.abi.prop_base_part_elasticity(instance)
	return getPointer(instance).Elasticity;
end
function property.abi.prop_set_base_part_elasticity(instance, number)
	getPointer(instance).Elasticity = number;
end
function property.abi.prop_base_part_friction(instance)
	return getPointer(instance).Friction;
end
function property.abi.prop_set_base_part_friction(instance, number)
	getPointer(instance).Friction = number;
end
function property.abi.prop_base_part_front_param_a(instance)
	return getPointer(instance).FrontParamA;
end
function property.abi.prop_set_base_part_front_param_a(instance, number)
	getPointer(instance).FrontParamA = number;
end
function property.abi.prop_base_part_front_param_b(instance)
	return getPointer(instance).FrontParamB;
end
function property.abi.prop_set_base_part_front_param_b(instance, number)
	getPointer(instance).FrontParamB = number;
end
function property.abi.prop_base_part_front_surface(instance)
end
function property.abi.prop_base_part_front_surface_input(instance)
end
function property.abi.prop_base_part_left_param_a(instance)
	return getPointer(instance).LeftParamA;
end
function property.abi.prop_set_base_part_left_param_a(instance, number)
	getPointer(instance).LeftParamA = number;
end
function property.abi.prop_base_part_left_param_b(instance)
	return getPointer(instance).LeftParamB;
end
function property.abi.prop_set_base_part_left_param_b(instance, number)
	getPointer(instance).LeftParamB = number;
end
function property.abi.prop_base_part_left_surface(instance)
end
function property.abi.prop_base_part_left_surface_input(instance)
end
function property.abi.prop_base_part_local_transparency_modifier(instance)
	return getPointer(instance).LocalTransparencyModifier;
end
function property.abi.prop_set_base_part_local_transparency_modifier(instance, number)
	getPointer(instance).LocalTransparencyModifier = number;
end
function property.abi.prop_base_part_locked(instance)
	return getPointer(instance).Locked and 1 or 0;
end
function property.abi.prop_set_base_part_locked(instance, bool)
	getPointer(instance).Locked = bool and 1 or 0;
end
function property.abi.prop_base_part_mass(instance)
	return getPointer(instance).Mass;
end
function property.abi.prop_base_part_massless(instance)
	return getPointer(instance).Massless and 1 or 0;
end
function property.abi.prop_set_base_part_massless(instance, bool)
	getPointer(instance).Massless = bool and 1 or 0;
end
function property.abi.prop_base_part_material(instance)
end
function property.abi.prop_base_part_material_variant(addr, instance)
	writeStringReturn(addr, getPointer(instance).MaterialVariant);
end
function property.abi.prop_set_base_part_material_variant(instance, addr, length)
	getPointer(instance).MaterialVariant = readString(addr, length);
end
function property.abi.prop_base_part_orientation(instance)
	return createPointer(getPointer(instance).Orientation);
end
function property.abi.prop_set_base_part_orientation(instance, vector_3)
	getPointer(instance).Orientation = getPointer(vector_3);
end
function property.abi.prop_base_part_pivot_offset(instance)
	return createPointer(getPointer(instance).PivotOffset);
end
function property.abi.prop_set_base_part_pivot_offset(instance, c_frame)
	getPointer(instance).PivotOffset = getPointer(c_frame);
end
function property.abi.prop_base_part_position(instance)
	return createPointer(getPointer(instance).Position);
end
function property.abi.prop_set_base_part_position(instance, vector_3)
	getPointer(instance).Position = getPointer(vector_3);
end
function property.abi.prop_base_part_receive_age(instance)
	return getPointer(instance).ReceiveAge;
end
function property.abi.prop_base_part_reflectance(instance)
	return getPointer(instance).Reflectance;
end
function property.abi.prop_set_base_part_reflectance(instance, number)
	getPointer(instance).Reflectance = number;
end
function property.abi.prop_base_part_resize_increment(instance)
	return getPointer(instance).ResizeIncrement;
end
function property.abi.prop_base_part_resizeable_faces(instance)
	return createPointer(getPointer(instance).ResizeableFaces);
end
function property.abi.prop_base_part_right_param_a(instance)
	return getPointer(instance).RightParamA;
end
function property.abi.prop_set_base_part_right_param_a(instance, number)
	getPointer(instance).RightParamA = number;
end
function property.abi.prop_base_part_right_param_b(instance)
	return getPointer(instance).RightParamB;
end
function property.abi.prop_set_base_part_right_param_b(instance, number)
	getPointer(instance).RightParamB = number;
end
function property.abi.prop_base_part_right_surface(instance)
end
function property.abi.prop_base_part_right_surface_input(instance)
end
function property.abi.prop_base_part_root_priority(instance)
	return getPointer(instance).RootPriority;
end
function property.abi.prop_set_base_part_root_priority(instance, number)
	getPointer(instance).RootPriority = number;
end
function property.abi.prop_base_part_rot_velocity(instance)
	return createPointer(getPointer(instance).RotVelocity);
end
function property.abi.prop_set_base_part_rot_velocity(instance, vector_3)
	getPointer(instance).RotVelocity = getPointer(vector_3);
end
function property.abi.prop_base_part_rotation(instance)
	return createPointer(getPointer(instance).Rotation);
end
function property.abi.prop_set_base_part_rotation(instance, vector_3)
	getPointer(instance).Rotation = getPointer(vector_3);
end
function property.abi.prop_base_part_size(instance)
	return createPointer(getPointer(instance).Size);
end
function property.abi.prop_set_base_part_size(instance, vector_3)
	getPointer(instance).Size = getPointer(vector_3);
end
function property.abi.prop_base_part_specific_gravity(instance)
	return getPointer(instance).SpecificGravity;
end
function property.abi.prop_base_part_top_param_a(instance)
	return getPointer(instance).TopParamA;
end
function property.abi.prop_set_base_part_top_param_a(instance, number)
	getPointer(instance).TopParamA = number;
end
function property.abi.prop_base_part_top_param_b(instance)
	return getPointer(instance).TopParamB;
end
function property.abi.prop_set_base_part_top_param_b(instance, number)
	getPointer(instance).TopParamB = number;
end
function property.abi.prop_base_part_top_surface(instance)
end
function property.abi.prop_base_part_top_surface_input(instance)
end
function property.abi.prop_base_part_transparency(instance)
	return getPointer(instance).Transparency;
end
function property.abi.prop_set_base_part_transparency(instance, number)
	getPointer(instance).Transparency = number;
end
function property.abi.prop_base_part_velocity(instance)
	return createPointer(getPointer(instance).Velocity);
end
function property.abi.prop_set_base_part_velocity(instance, vector_3)
	getPointer(instance).Velocity = getPointer(vector_3);
end
function property.abi.prop_form_factor_part_form_factor(instance)
end
function property.abi.prop_part_shape(instance)
end
function property.abi.prop_seat_disabled(instance)
	return getPointer(instance).Disabled and 1 or 0;
end
function property.abi.prop_set_seat_disabled(instance, bool)
	getPointer(instance).Disabled = bool and 1 or 0;
end
function property.abi.prop_seat_occupant(addr, instance)
	local value = getPointer(instance).Occupant;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_skateboard_platform_controller(addr, instance)
	local value = getPointer(instance).Controller;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_skateboard_platform_controlling_humanoid(addr, instance)
	local value = getPointer(instance).ControllingHumanoid;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_skateboard_platform_steer(instance)
	return getPointer(instance).Steer;
end
function property.abi.prop_set_skateboard_platform_steer(instance, number)
	getPointer(instance).Steer = number;
end
function property.abi.prop_skateboard_platform_sticky_wheels(instance)
	return getPointer(instance).StickyWheels and 1 or 0;
end
function property.abi.prop_set_skateboard_platform_sticky_wheels(instance, bool)
	getPointer(instance).StickyWheels = bool and 1 or 0;
end
function property.abi.prop_skateboard_platform_throttle(instance)
	return getPointer(instance).Throttle;
end
function property.abi.prop_set_skateboard_platform_throttle(instance, number)
	getPointer(instance).Throttle = number;
end
function property.abi.prop_spawn_location_allow_team_change_on_touch(instance)
	return getPointer(instance).AllowTeamChangeOnTouch and 1 or 0;
end
function property.abi.prop_set_spawn_location_allow_team_change_on_touch(instance, bool)
	getPointer(instance).AllowTeamChangeOnTouch = bool and 1 or 0;
end
function property.abi.prop_spawn_location_duration(instance)
	return getPointer(instance).Duration;
end
function property.abi.prop_set_spawn_location_duration(instance, number)
	getPointer(instance).Duration = number;
end
function property.abi.prop_spawn_location_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_spawn_location_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_spawn_location_neutral(instance)
	return getPointer(instance).Neutral and 1 or 0;
end
function property.abi.prop_set_spawn_location_neutral(instance, bool)
	getPointer(instance).Neutral = bool and 1 or 0;
end
function property.abi.prop_spawn_location_team_color(instance)
	return createPointer(getPointer(instance).TeamColor);
end
function property.abi.prop_set_spawn_location_team_color(instance, brick_color)
	getPointer(instance).TeamColor = getPointer(brick_color);
end
function property.abi.prop_terrain_decoration(instance)
	return getPointer(instance).Decoration and 1 or 0;
end
function property.abi.prop_set_terrain_decoration(instance, bool)
	getPointer(instance).Decoration = bool and 1 or 0;
end
function property.abi.prop_terrain_is_smooth(instance)
	return getPointer(instance).IsSmooth and 1 or 0;
end
function property.abi.prop_terrain_material_colors(instance)
	return createPointer(getPointer(instance).MaterialColors);
end
function property.abi.prop_set_terrain_material_colors(instance, binary_string)
	getPointer(instance).MaterialColors = getPointer(binary_string);
end
function property.abi.prop_terrain_max_extents(instance)
	return createPointer(getPointer(instance).MaxExtents);
end
function property.abi.prop_terrain_water_color(instance)
	return createPointer(getPointer(instance).WaterColor);
end
function property.abi.prop_set_terrain_water_color(instance, color_3)
	getPointer(instance).WaterColor = getPointer(color_3);
end
function property.abi.prop_terrain_water_reflectance(instance)
	return getPointer(instance).WaterReflectance;
end
function property.abi.prop_set_terrain_water_reflectance(instance, number)
	getPointer(instance).WaterReflectance = number;
end
function property.abi.prop_terrain_water_transparency(instance)
	return getPointer(instance).WaterTransparency;
end
function property.abi.prop_set_terrain_water_transparency(instance, number)
	getPointer(instance).WaterTransparency = number;
end
function property.abi.prop_terrain_water_wave_size(instance)
	return getPointer(instance).WaterWaveSize;
end
function property.abi.prop_set_terrain_water_wave_size(instance, number)
	getPointer(instance).WaterWaveSize = number;
end
function property.abi.prop_terrain_water_wave_speed(instance)
	return getPointer(instance).WaterWaveSpeed;
end
function property.abi.prop_set_terrain_water_wave_speed(instance, number)
	getPointer(instance).WaterWaveSpeed = number;
end
function property.abi.prop_triangle_mesh_part_collision_fidelity(instance)
end
function property.abi.prop_mesh_part_double_sided(instance)
	return getPointer(instance).DoubleSided and 1 or 0;
end
function property.abi.prop_mesh_part_has_joint_offset(instance)
	return getPointer(instance).HasJointOffset and 1 or 0;
end
function property.abi.prop_mesh_part_has_skinned_mesh(instance)
	return getPointer(instance).HasSkinnedMesh and 1 or 0;
end
function property.abi.prop_mesh_part_joint_offset(instance)
	return createPointer(getPointer(instance).JointOffset);
end
function property.abi.prop_mesh_part_mesh_id(instance)
	return createPointer(getPointer(instance).MeshId);
end
function property.abi.prop_mesh_part_mesh_size(instance)
	return createPointer(getPointer(instance).MeshSize);
end
function property.abi.prop_mesh_part_render_fidelity(instance)
end
function property.abi.prop_mesh_part_texture_id(instance)
	return createPointer(getPointer(instance).TextureID);
end
function property.abi.prop_set_mesh_part_texture_id(instance, content)
	getPointer(instance).TextureID = getPointer(content);
end
function property.abi.prop_part_operation_render_fidelity(instance)
end
function property.abi.prop_part_operation_smoothing_angle(instance)
	return getPointer(instance).SmoothingAngle;
end
function property.abi.prop_part_operation_triangle_count(instance)
	return getPointer(instance).TriangleCount;
end
function property.abi.prop_part_operation_use_part_color(instance)
	return getPointer(instance).UsePartColor and 1 or 0;
end
function property.abi.prop_set_part_operation_use_part_color(instance, bool)
	getPointer(instance).UsePartColor = bool and 1 or 0;
end
function property.abi.prop_truss_part_style(instance)
end
function property.abi.prop_vehicle_seat_are_hinges_detected(instance)
	return getPointer(instance).AreHingesDetected;
end
function property.abi.prop_vehicle_seat_disabled(instance)
	return getPointer(instance).Disabled and 1 or 0;
end
function property.abi.prop_set_vehicle_seat_disabled(instance, bool)
	getPointer(instance).Disabled = bool and 1 or 0;
end
function property.abi.prop_vehicle_seat_heads_up_display(instance)
	return getPointer(instance).HeadsUpDisplay and 1 or 0;
end
function property.abi.prop_set_vehicle_seat_heads_up_display(instance, bool)
	getPointer(instance).HeadsUpDisplay = bool and 1 or 0;
end
function property.abi.prop_vehicle_seat_max_speed(instance)
	return getPointer(instance).MaxSpeed;
end
function property.abi.prop_set_vehicle_seat_max_speed(instance, number)
	getPointer(instance).MaxSpeed = number;
end
function property.abi.prop_vehicle_seat_occupant(addr, instance)
	local value = getPointer(instance).Occupant;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_vehicle_seat_steer(instance)
	return getPointer(instance).Steer;
end
function property.abi.prop_set_vehicle_seat_steer(instance, number)
	getPointer(instance).Steer = number;
end
function property.abi.prop_vehicle_seat_steer_float(instance)
	return getPointer(instance).SteerFloat;
end
function property.abi.prop_set_vehicle_seat_steer_float(instance, number)
	getPointer(instance).SteerFloat = number;
end
function property.abi.prop_vehicle_seat_throttle(instance)
	return getPointer(instance).Throttle;
end
function property.abi.prop_set_vehicle_seat_throttle(instance, number)
	getPointer(instance).Throttle = number;
end
function property.abi.prop_vehicle_seat_throttle_float(instance)
	return getPointer(instance).ThrottleFloat;
end
function property.abi.prop_set_vehicle_seat_throttle_float(instance, number)
	getPointer(instance).ThrottleFloat = number;
end
function property.abi.prop_vehicle_seat_torque(instance)
	return getPointer(instance).Torque;
end
function property.abi.prop_set_vehicle_seat_torque(instance, number)
	getPointer(instance).Torque = number;
end
function property.abi.prop_vehicle_seat_turn_speed(instance)
	return getPointer(instance).TurnSpeed;
end
function property.abi.prop_set_vehicle_seat_turn_speed(instance, number)
	getPointer(instance).TurnSpeed = number;
end
function property.abi.prop_model_primary_part(addr, instance)
	local value = getPointer(instance).PrimaryPart;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_model_primary_part(instance, isSome, value)
	getPointer(instance).PrimaryPart = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_model_world_pivot(instance)
	return createPointer(getPointer(instance).WorldPivot);
end
function property.abi.prop_set_model_world_pivot(instance, c_frame)
	getPointer(instance).WorldPivot = getPointer(c_frame);
end
function property.abi.prop_workspace_allow_third_party_sales(instance)
	return getPointer(instance).AllowThirdPartySales and 1 or 0;
end
function property.abi.prop_set_workspace_allow_third_party_sales(instance, bool)
	getPointer(instance).AllowThirdPartySales = bool and 1 or 0;
end
function property.abi.prop_workspace_animation_weighted_blend_fix(instance)
end
function property.abi.prop_workspace_client_animator_throttling(instance)
end
function property.abi.prop_workspace_current_camera(addr, instance)
	local value = getPointer(instance).CurrentCamera;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_workspace_current_camera(instance, isSome, value)
	getPointer(instance).CurrentCamera = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_workspace_distributed_game_time(instance)
	return getPointer(instance).DistributedGameTime;
end
function property.abi.prop_set_workspace_distributed_game_time(instance, number)
	getPointer(instance).DistributedGameTime = number;
end
function property.abi.prop_workspace_fallen_parts_destroy_height(instance)
	return getPointer(instance).FallenPartsDestroyHeight;
end
function property.abi.prop_workspace_filtering_enabled(instance)
	return getPointer(instance).FilteringEnabled and 1 or 0;
end
function property.abi.prop_workspace_global_wind(instance)
	return createPointer(getPointer(instance).GlobalWind);
end
function property.abi.prop_set_workspace_global_wind(instance, vector_3)
	getPointer(instance).GlobalWind = getPointer(vector_3);
end
function property.abi.prop_workspace_gravity(instance)
	return getPointer(instance).Gravity;
end
function property.abi.prop_set_workspace_gravity(instance, number)
	getPointer(instance).Gravity = number;
end
function property.abi.prop_workspace_humanoid_only_set_collisions_on_state_change(instance)
end
function property.abi.prop_workspace_interpolation_throttling(instance)
end
function property.abi.prop_workspace_mesh_part_heads_and_accessories(instance)
end
function property.abi.prop_workspace_physics_inertia_and_volume_fix(instance)
end
function property.abi.prop_workspace_physics_simulation_rate(instance)
end
function property.abi.prop_workspace_physics_stepping_method(instance)
end
function property.abi.prop_workspace_replicate_instance_destroy_setting(instance)
end
function property.abi.prop_workspace_retargeting(instance)
end
function property.abi.prop_workspace_signal_behavior(instance)
end
function property.abi.prop_workspace_stream_out_behavior(instance)
end
function property.abi.prop_workspace_streaming_enabled(instance)
	return getPointer(instance).StreamingEnabled and 1 or 0;
end
function property.abi.prop_workspace_streaming_min_radius(instance)
	return getPointer(instance).StreamingMinRadius;
end
function property.abi.prop_set_workspace_streaming_min_radius(instance, number)
	getPointer(instance).StreamingMinRadius = number;
end
function property.abi.prop_workspace_streaming_pause_mode(instance)
end
function property.abi.prop_workspace_streaming_target_radius(instance)
	return getPointer(instance).StreamingTargetRadius;
end
function property.abi.prop_set_workspace_streaming_target_radius(instance, number)
	getPointer(instance).StreamingTargetRadius = number;
end
function property.abi.prop_workspace_terrain(addr, instance)
	local value = getPointer(instance).Terrain;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_workspace_touches_use_collision_groups(instance)
	return getPointer(instance).TouchesUseCollisionGroups and 1 or 0;
end
function property.abi.prop_set_workspace_touches_use_collision_groups(instance, bool)
	getPointer(instance).TouchesUseCollisionGroups = bool and 1 or 0;
end
function property.abi.prop_package_link_auto_update(instance)
	return getPointer(instance).AutoUpdate and 1 or 0;
end
function property.abi.prop_set_package_link_auto_update(instance, bool)
	getPointer(instance).AutoUpdate = bool and 1 or 0;
end
function property.abi.prop_package_link_creator(addr, instance)
	writeStringReturn(addr, getPointer(instance).Creator);
end
function property.abi.prop_package_link_package_asset_name(addr, instance)
	writeStringReturn(addr, getPointer(instance).PackageAssetName);
end
function property.abi.prop_package_link_package_id(instance)
	return createPointer(getPointer(instance).PackageId);
end
function property.abi.prop_package_link_permission_level(instance)
end
function property.abi.prop_package_link_version_number(instance)
	return getPointer(instance).VersionNumber;
end
function property.abi.prop_pages_is_finished(instance)
	return getPointer(instance).IsFinished and 1 or 0;
end
function property.abi.prop_particle_emitter_acceleration(instance)
	return createPointer(getPointer(instance).Acceleration);
end
function property.abi.prop_set_particle_emitter_acceleration(instance, vector_3)
	getPointer(instance).Acceleration = getPointer(vector_3);
end
function property.abi.prop_particle_emitter_brightness(instance)
	return getPointer(instance).Brightness;
end
function property.abi.prop_set_particle_emitter_brightness(instance, number)
	getPointer(instance).Brightness = number;
end
function property.abi.prop_particle_emitter_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_particle_emitter_color(instance, color_sequence)
	getPointer(instance).Color = getPointer(color_sequence);
end
function property.abi.prop_particle_emitter_drag(instance)
	return getPointer(instance).Drag;
end
function property.abi.prop_set_particle_emitter_drag(instance, number)
	getPointer(instance).Drag = number;
end
function property.abi.prop_particle_emitter_emission_direction(instance)
end
function property.abi.prop_particle_emitter_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_particle_emitter_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_particle_emitter_flipbook_framerate(instance)
	return createPointer(getPointer(instance).FlipbookFramerate);
end
function property.abi.prop_set_particle_emitter_flipbook_framerate(instance, number_range)
	getPointer(instance).FlipbookFramerate = getPointer(number_range);
end
function property.abi.prop_particle_emitter_flipbook_incompatible(addr, instance)
	writeStringReturn(addr, getPointer(instance).FlipbookIncompatible);
end
function property.abi.prop_set_particle_emitter_flipbook_incompatible(instance, addr, length)
	getPointer(instance).FlipbookIncompatible = readString(addr, length);
end
function property.abi.prop_particle_emitter_flipbook_layout(instance)
end
function property.abi.prop_particle_emitter_flipbook_mode(instance)
end
function property.abi.prop_particle_emitter_flipbook_start_random(instance)
	return getPointer(instance).FlipbookStartRandom and 1 or 0;
end
function property.abi.prop_set_particle_emitter_flipbook_start_random(instance, bool)
	getPointer(instance).FlipbookStartRandom = bool and 1 or 0;
end
function property.abi.prop_particle_emitter_lifetime(instance)
	return createPointer(getPointer(instance).Lifetime);
end
function property.abi.prop_set_particle_emitter_lifetime(instance, number_range)
	getPointer(instance).Lifetime = getPointer(number_range);
end
function property.abi.prop_particle_emitter_light_emission(instance)
	return getPointer(instance).LightEmission;
end
function property.abi.prop_set_particle_emitter_light_emission(instance, number)
	getPointer(instance).LightEmission = number;
end
function property.abi.prop_particle_emitter_light_influence(instance)
	return getPointer(instance).LightInfluence;
end
function property.abi.prop_set_particle_emitter_light_influence(instance, number)
	getPointer(instance).LightInfluence = number;
end
function property.abi.prop_particle_emitter_locked_to_part(instance)
	return getPointer(instance).LockedToPart and 1 or 0;
end
function property.abi.prop_set_particle_emitter_locked_to_part(instance, bool)
	getPointer(instance).LockedToPart = bool and 1 or 0;
end
function property.abi.prop_particle_emitter_orientation(instance)
end
function property.abi.prop_particle_emitter_rate(instance)
	return getPointer(instance).Rate;
end
function property.abi.prop_set_particle_emitter_rate(instance, number)
	getPointer(instance).Rate = number;
end
function property.abi.prop_particle_emitter_rot_speed(instance)
	return createPointer(getPointer(instance).RotSpeed);
end
function property.abi.prop_set_particle_emitter_rot_speed(instance, number_range)
	getPointer(instance).RotSpeed = getPointer(number_range);
end
function property.abi.prop_particle_emitter_rotation(instance)
	return createPointer(getPointer(instance).Rotation);
end
function property.abi.prop_set_particle_emitter_rotation(instance, number_range)
	getPointer(instance).Rotation = getPointer(number_range);
end
function property.abi.prop_particle_emitter_shape(instance)
end
function property.abi.prop_particle_emitter_shape_in_out(instance)
end
function property.abi.prop_particle_emitter_shape_partial(instance)
	return getPointer(instance).ShapePartial;
end
function property.abi.prop_set_particle_emitter_shape_partial(instance, number)
	getPointer(instance).ShapePartial = number;
end
function property.abi.prop_particle_emitter_shape_style(instance)
end
function property.abi.prop_particle_emitter_size(instance)
	return createPointer(getPointer(instance).Size);
end
function property.abi.prop_set_particle_emitter_size(instance, number_sequence)
	getPointer(instance).Size = getPointer(number_sequence);
end
function property.abi.prop_particle_emitter_speed(instance)
	return createPointer(getPointer(instance).Speed);
end
function property.abi.prop_set_particle_emitter_speed(instance, number_range)
	getPointer(instance).Speed = getPointer(number_range);
end
function property.abi.prop_particle_emitter_spread_angle(instance)
	return createPointer(getPointer(instance).SpreadAngle);
end
function property.abi.prop_set_particle_emitter_spread_angle(instance, vector_2)
	getPointer(instance).SpreadAngle = getPointer(vector_2);
end
function property.abi.prop_particle_emitter_squash(instance)
	return createPointer(getPointer(instance).Squash);
end
function property.abi.prop_set_particle_emitter_squash(instance, number_sequence)
	getPointer(instance).Squash = getPointer(number_sequence);
end
function property.abi.prop_particle_emitter_texture(instance)
	return createPointer(getPointer(instance).Texture);
end
function property.abi.prop_set_particle_emitter_texture(instance, content)
	getPointer(instance).Texture = getPointer(content);
end
function property.abi.prop_particle_emitter_time_scale(instance)
	return getPointer(instance).TimeScale;
end
function property.abi.prop_set_particle_emitter_time_scale(instance, number)
	getPointer(instance).TimeScale = number;
end
function property.abi.prop_particle_emitter_transparency(instance)
	return createPointer(getPointer(instance).Transparency);
end
function property.abi.prop_set_particle_emitter_transparency(instance, number_sequence)
	getPointer(instance).Transparency = getPointer(number_sequence);
end
function property.abi.prop_particle_emitter_velocity_inheritance(instance)
	return getPointer(instance).VelocityInheritance;
end
function property.abi.prop_set_particle_emitter_velocity_inheritance(instance, number)
	getPointer(instance).VelocityInheritance = number;
end
function property.abi.prop_particle_emitter_velocity_spread(instance)
	return getPointer(instance).VelocitySpread;
end
function property.abi.prop_set_particle_emitter_velocity_spread(instance, number)
	getPointer(instance).VelocitySpread = number;
end
function property.abi.prop_particle_emitter_z_offset(instance)
	return getPointer(instance).ZOffset;
end
function property.abi.prop_set_particle_emitter_z_offset(instance, number)
	getPointer(instance).ZOffset = number;
end
function property.abi.prop_path_status(instance)
end
function property.abi.prop_pathfinding_link_attachment_0(addr, instance)
	local value = getPointer(instance).Attachment0;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_pathfinding_link_attachment_0(instance, isSome, value)
	getPointer(instance).Attachment0 = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_pathfinding_link_attachment_1(addr, instance)
	local value = getPointer(instance).Attachment1;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_pathfinding_link_attachment_1(instance, isSome, value)
	getPointer(instance).Attachment1 = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_pathfinding_link_is_bidirectional(instance)
	return getPointer(instance).IsBidirectional and 1 or 0;
end
function property.abi.prop_set_pathfinding_link_is_bidirectional(instance, bool)
	getPointer(instance).IsBidirectional = bool and 1 or 0;
end
function property.abi.prop_pathfinding_link_label(addr, instance)
	writeStringReturn(addr, getPointer(instance).Label);
end
function property.abi.prop_set_pathfinding_link_label(instance, addr, length)
	getPointer(instance).Label = readString(addr, length);
end
function property.abi.prop_pathfinding_modifier_label(addr, instance)
	writeStringReturn(addr, getPointer(instance).Label);
end
function property.abi.prop_set_pathfinding_modifier_label(instance, addr, length)
	getPointer(instance).Label = readString(addr, length);
end
function property.abi.prop_pathfinding_modifier_pass_through(instance)
	return getPointer(instance).PassThrough and 1 or 0;
end
function property.abi.prop_set_pathfinding_modifier_pass_through(instance, bool)
	getPointer(instance).PassThrough = bool and 1 or 0;
end
function property.abi.prop_pathfinding_service_empty_cutoff(instance)
	return getPointer(instance).EmptyCutoff;
end
function property.abi.prop_set_pathfinding_service_empty_cutoff(instance, number)
	getPointer(instance).EmptyCutoff = number;
end
function property.abi.prop_player_account_age(instance)
	return getPointer(instance).AccountAge;
end
function property.abi.prop_player_auto_jump_enabled(instance)
	return getPointer(instance).AutoJumpEnabled and 1 or 0;
end
function property.abi.prop_set_player_auto_jump_enabled(instance, bool)
	getPointer(instance).AutoJumpEnabled = bool and 1 or 0;
end
function property.abi.prop_player_camera_max_zoom_distance(instance)
	return getPointer(instance).CameraMaxZoomDistance;
end
function property.abi.prop_set_player_camera_max_zoom_distance(instance, number)
	getPointer(instance).CameraMaxZoomDistance = number;
end
function property.abi.prop_player_camera_min_zoom_distance(instance)
	return getPointer(instance).CameraMinZoomDistance;
end
function property.abi.prop_set_player_camera_min_zoom_distance(instance, number)
	getPointer(instance).CameraMinZoomDistance = number;
end
function property.abi.prop_player_camera_mode(instance)
end
function property.abi.prop_player_can_load_character_appearance(instance)
	return getPointer(instance).CanLoadCharacterAppearance and 1 or 0;
end
function property.abi.prop_set_player_can_load_character_appearance(instance, bool)
	getPointer(instance).CanLoadCharacterAppearance = bool and 1 or 0;
end
function property.abi.prop_player_character(addr, instance)
	local value = getPointer(instance).Character;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_player_character(instance, isSome, value)
	getPointer(instance).Character = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_player_character_appearance(addr, instance)
	writeStringReturn(addr, getPointer(instance).CharacterAppearance);
end
function property.abi.prop_set_player_character_appearance(instance, addr, length)
	getPointer(instance).CharacterAppearance = readString(addr, length);
end
function property.abi.prop_player_character_appearance_id(instance)
	return getPointer(instance).CharacterAppearanceId;
end
function property.abi.prop_set_player_character_appearance_id(instance, number)
	getPointer(instance).CharacterAppearanceId = number;
end
function property.abi.prop_player_data_complexity(instance)
	return getPointer(instance).DataComplexity;
end
function property.abi.prop_player_data_ready(instance)
	return getPointer(instance).DataReady and 1 or 0;
end
function property.abi.prop_player_dev_camera_occlusion_mode(instance)
end
function property.abi.prop_player_dev_computer_camera_mode(instance)
end
function property.abi.prop_player_dev_computer_movement_mode(instance)
end
function property.abi.prop_player_dev_enable_mouse_lock(instance)
	return getPointer(instance).DevEnableMouseLock and 1 or 0;
end
function property.abi.prop_set_player_dev_enable_mouse_lock(instance, bool)
	getPointer(instance).DevEnableMouseLock = bool and 1 or 0;
end
function property.abi.prop_player_dev_touch_camera_mode(instance)
end
function property.abi.prop_player_dev_touch_movement_mode(instance)
end
function property.abi.prop_player_display_name(addr, instance)
	writeStringReturn(addr, getPointer(instance).DisplayName);
end
function property.abi.prop_set_player_display_name(instance, addr, length)
	getPointer(instance).DisplayName = readString(addr, length);
end
function property.abi.prop_player_follow_user_id(instance)
	return getPointer(instance).FollowUserId;
end
function property.abi.prop_player_gameplay_paused(instance)
	return getPointer(instance).GameplayPaused and 1 or 0;
end
function property.abi.prop_player_health_display_distance(instance)
	return getPointer(instance).HealthDisplayDistance;
end
function property.abi.prop_set_player_health_display_distance(instance, number)
	getPointer(instance).HealthDisplayDistance = number;
end
function property.abi.prop_player_locale_id(addr, instance)
	writeStringReturn(addr, getPointer(instance).LocaleId);
end
function property.abi.prop_player_membership_type(instance)
end
function property.abi.prop_player_name_display_distance(instance)
	return getPointer(instance).NameDisplayDistance;
end
function property.abi.prop_set_player_name_display_distance(instance, number)
	getPointer(instance).NameDisplayDistance = number;
end
function property.abi.prop_player_neutral(instance)
	return getPointer(instance).Neutral and 1 or 0;
end
function property.abi.prop_set_player_neutral(instance, bool)
	getPointer(instance).Neutral = bool and 1 or 0;
end
function property.abi.prop_player_replication_focus(addr, instance)
	local value = getPointer(instance).ReplicationFocus;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_player_replication_focus(instance, isSome, value)
	getPointer(instance).ReplicationFocus = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_player_respawn_location(addr, instance)
	local value = getPointer(instance).RespawnLocation;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_player_respawn_location(instance, isSome, value)
	getPointer(instance).RespawnLocation = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_player_team(addr, instance)
	local value = getPointer(instance).Team;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_player_team(instance, isSome, value)
	getPointer(instance).Team = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_player_team_color(instance)
	return createPointer(getPointer(instance).TeamColor);
end
function property.abi.prop_set_player_team_color(instance, brick_color)
	getPointer(instance).TeamColor = getPointer(brick_color);
end
function property.abi.prop_player_user_id(instance)
	return getPointer(instance).UserId;
end
function property.abi.prop_set_player_user_id(instance, number)
	getPointer(instance).UserId = number;
end
function property.abi.prop_players_bubble_chat(instance)
	return getPointer(instance).BubbleChat and 1 or 0;
end
function property.abi.prop_players_character_auto_loads(instance)
	return getPointer(instance).CharacterAutoLoads and 1 or 0;
end
function property.abi.prop_set_players_character_auto_loads(instance, bool)
	getPointer(instance).CharacterAutoLoads = bool and 1 or 0;
end
function property.abi.prop_players_classic_chat(instance)
	return getPointer(instance).ClassicChat and 1 or 0;
end
function property.abi.prop_players_local_player(addr, instance)
	local value = getPointer(instance).LocalPlayer;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_players_max_players(instance)
	return getPointer(instance).MaxPlayers;
end
function property.abi.prop_players_num_players(instance)
	return getPointer(instance).NumPlayers;
end
function property.abi.prop_players_preferred_players(instance)
	return getPointer(instance).PreferredPlayers;
end
function property.abi.prop_players_respawn_time(instance)
	return getPointer(instance).RespawnTime;
end
function property.abi.prop_set_players_respawn_time(instance, number)
	getPointer(instance).RespawnTime = number;
end
function property.abi.prop_pose_base_easing_direction(instance)
end
function property.abi.prop_pose_base_easing_style(instance)
end
function property.abi.prop_pose_base_weight(instance)
	return getPointer(instance).Weight;
end
function property.abi.prop_set_pose_base_weight(instance, number)
	getPointer(instance).Weight = number;
end
function property.abi.prop_number_pose_value(instance)
	return getPointer(instance).Value;
end
function property.abi.prop_set_number_pose_value(instance, number)
	getPointer(instance).Value = number;
end
function property.abi.prop_pose_c_frame(instance)
	return createPointer(getPointer(instance).CFrame);
end
function property.abi.prop_set_pose_c_frame(instance, c_frame)
	getPointer(instance).CFrame = getPointer(c_frame);
end
function property.abi.prop_pose_mask_weight(instance)
	return getPointer(instance).MaskWeight;
end
function property.abi.prop_set_pose_mask_weight(instance, number)
	getPointer(instance).MaskWeight = number;
end
function property.abi.prop_post_effect_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_post_effect_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_bloom_effect_intensity(instance)
	return getPointer(instance).Intensity;
end
function property.abi.prop_set_bloom_effect_intensity(instance, number)
	getPointer(instance).Intensity = number;
end
function property.abi.prop_bloom_effect_size(instance)
	return getPointer(instance).Size;
end
function property.abi.prop_set_bloom_effect_size(instance, number)
	getPointer(instance).Size = number;
end
function property.abi.prop_bloom_effect_threshold(instance)
	return getPointer(instance).Threshold;
end
function property.abi.prop_set_bloom_effect_threshold(instance, number)
	getPointer(instance).Threshold = number;
end
function property.abi.prop_blur_effect_size(instance)
	return getPointer(instance).Size;
end
function property.abi.prop_set_blur_effect_size(instance, number)
	getPointer(instance).Size = number;
end
function property.abi.prop_color_correction_effect_brightness(instance)
	return getPointer(instance).Brightness;
end
function property.abi.prop_set_color_correction_effect_brightness(instance, number)
	getPointer(instance).Brightness = number;
end
function property.abi.prop_color_correction_effect_contrast(instance)
	return getPointer(instance).Contrast;
end
function property.abi.prop_set_color_correction_effect_contrast(instance, number)
	getPointer(instance).Contrast = number;
end
function property.abi.prop_color_correction_effect_saturation(instance)
	return getPointer(instance).Saturation;
end
function property.abi.prop_set_color_correction_effect_saturation(instance, number)
	getPointer(instance).Saturation = number;
end
function property.abi.prop_color_correction_effect_tint_color(instance)
	return createPointer(getPointer(instance).TintColor);
end
function property.abi.prop_set_color_correction_effect_tint_color(instance, color_3)
	getPointer(instance).TintColor = getPointer(color_3);
end
function property.abi.prop_depth_of_field_effect_far_intensity(instance)
	return getPointer(instance).FarIntensity;
end
function property.abi.prop_set_depth_of_field_effect_far_intensity(instance, number)
	getPointer(instance).FarIntensity = number;
end
function property.abi.prop_depth_of_field_effect_focus_distance(instance)
	return getPointer(instance).FocusDistance;
end
function property.abi.prop_set_depth_of_field_effect_focus_distance(instance, number)
	getPointer(instance).FocusDistance = number;
end
function property.abi.prop_depth_of_field_effect_in_focus_radius(instance)
	return getPointer(instance).InFocusRadius;
end
function property.abi.prop_set_depth_of_field_effect_in_focus_radius(instance, number)
	getPointer(instance).InFocusRadius = number;
end
function property.abi.prop_depth_of_field_effect_near_intensity(instance)
	return getPointer(instance).NearIntensity;
end
function property.abi.prop_set_depth_of_field_effect_near_intensity(instance, number)
	getPointer(instance).NearIntensity = number;
end
function property.abi.prop_sun_rays_effect_intensity(instance)
	return getPointer(instance).Intensity;
end
function property.abi.prop_set_sun_rays_effect_intensity(instance, number)
	getPointer(instance).Intensity = number;
end
function property.abi.prop_sun_rays_effect_spread(instance)
	return getPointer(instance).Spread;
end
function property.abi.prop_set_sun_rays_effect_spread(instance, number)
	getPointer(instance).Spread = number;
end
function property.abi.prop_proximity_prompt_action_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).ActionText);
end
function property.abi.prop_set_proximity_prompt_action_text(instance, addr, length)
	getPointer(instance).ActionText = readString(addr, length);
end
function property.abi.prop_proximity_prompt_auto_localize(instance)
	return getPointer(instance).AutoLocalize and 1 or 0;
end
function property.abi.prop_set_proximity_prompt_auto_localize(instance, bool)
	getPointer(instance).AutoLocalize = bool and 1 or 0;
end
function property.abi.prop_proximity_prompt_clickable_prompt(instance)
	return getPointer(instance).ClickablePrompt and 1 or 0;
end
function property.abi.prop_set_proximity_prompt_clickable_prompt(instance, bool)
	getPointer(instance).ClickablePrompt = bool and 1 or 0;
end
function property.abi.prop_proximity_prompt_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_proximity_prompt_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_proximity_prompt_exclusivity(instance)
end
function property.abi.prop_proximity_prompt_gamepad_key_code(instance)
end
function property.abi.prop_proximity_prompt_hold_duration(instance)
	return getPointer(instance).HoldDuration;
end
function property.abi.prop_set_proximity_prompt_hold_duration(instance, number)
	getPointer(instance).HoldDuration = number;
end
function property.abi.prop_proximity_prompt_keyboard_key_code(instance)
end
function property.abi.prop_proximity_prompt_max_activation_distance(instance)
	return getPointer(instance).MaxActivationDistance;
end
function property.abi.prop_set_proximity_prompt_max_activation_distance(instance, number)
	getPointer(instance).MaxActivationDistance = number;
end
function property.abi.prop_proximity_prompt_object_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).ObjectText);
end
function property.abi.prop_set_proximity_prompt_object_text(instance, addr, length)
	getPointer(instance).ObjectText = readString(addr, length);
end
function property.abi.prop_proximity_prompt_requires_line_of_sight(instance)
	return getPointer(instance).RequiresLineOfSight and 1 or 0;
end
function property.abi.prop_set_proximity_prompt_requires_line_of_sight(instance, bool)
	getPointer(instance).RequiresLineOfSight = bool and 1 or 0;
end
function property.abi.prop_proximity_prompt_root_localization_table(addr, instance)
	local value = getPointer(instance).RootLocalizationTable;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_proximity_prompt_root_localization_table(instance, isSome, value)
	getPointer(instance).RootLocalizationTable = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_proximity_prompt_style(instance)
end
function property.abi.prop_proximity_prompt_ui_offset(instance)
	return createPointer(getPointer(instance).UIOffset);
end
function property.abi.prop_set_proximity_prompt_ui_offset(instance, vector_2)
	getPointer(instance).UIOffset = getPointer(vector_2);
end
function property.abi.prop_proximity_prompt_service_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_proximity_prompt_service_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_proximity_prompt_service_max_prompts_visible(instance)
	return getPointer(instance).MaxPromptsVisible;
end
function property.abi.prop_set_proximity_prompt_service_max_prompts_visible(instance, number)
	getPointer(instance).MaxPromptsVisible = number;
end
function property.abi.prop_rotation_curve_length(instance)
	return getPointer(instance).Length;
end
function property.abi.prop_screenshot_hud_camera_button_icon(instance)
	return createPointer(getPointer(instance).CameraButtonIcon);
end
function property.abi.prop_set_screenshot_hud_camera_button_icon(instance, content)
	getPointer(instance).CameraButtonIcon = getPointer(content);
end
function property.abi.prop_screenshot_hud_camera_button_position(instance)
	return createPointer(getPointer(instance).CameraButtonPosition);
end
function property.abi.prop_set_screenshot_hud_camera_button_position(instance, u_dim_2)
	getPointer(instance).CameraButtonPosition = getPointer(u_dim_2);
end
function property.abi.prop_screenshot_hud_close_button_position(instance)
	return createPointer(getPointer(instance).CloseButtonPosition);
end
function property.abi.prop_set_screenshot_hud_close_button_position(instance, u_dim_2)
	getPointer(instance).CloseButtonPosition = getPointer(u_dim_2);
end
function property.abi.prop_screenshot_hud_close_when_screenshot_taken(instance)
	return getPointer(instance).CloseWhenScreenshotTaken and 1 or 0;
end
function property.abi.prop_set_screenshot_hud_close_when_screenshot_taken(instance, bool)
	getPointer(instance).CloseWhenScreenshotTaken = bool and 1 or 0;
end
function property.abi.prop_screenshot_hud_experience_name_overlay_enabled(instance)
	return getPointer(instance).ExperienceNameOverlayEnabled and 1 or 0;
end
function property.abi.prop_set_screenshot_hud_experience_name_overlay_enabled(instance, bool)
	getPointer(instance).ExperienceNameOverlayEnabled = bool and 1 or 0;
end
function property.abi.prop_screenshot_hud_overlay_font(instance)
end
function property.abi.prop_screenshot_hud_username_overlay_enabled(instance)
	return getPointer(instance).UsernameOverlayEnabled and 1 or 0;
end
function property.abi.prop_set_screenshot_hud_username_overlay_enabled(instance, bool)
	getPointer(instance).UsernameOverlayEnabled = bool and 1 or 0;
end
function property.abi.prop_screenshot_hud_visible(instance)
	return getPointer(instance).Visible and 1 or 0;
end
function property.abi.prop_set_screenshot_hud_visible(instance, bool)
	getPointer(instance).Visible = bool and 1 or 0;
end
function property.abi.prop_server_script_service_load_string_enabled(instance)
	return getPointer(instance).LoadStringEnabled and 1 or 0;
end
function property.abi.prop_set_server_script_service_load_string_enabled(instance, bool)
	getPointer(instance).LoadStringEnabled = bool and 1 or 0;
end
function property.abi.prop_data_model_creator_id(instance)
	return getPointer(instance).CreatorId;
end
function property.abi.prop_data_model_creator_type(instance)
end
function property.abi.prop_data_model_game_id(instance)
	return getPointer(instance).GameId;
end
function property.abi.prop_data_model_gear_genre_setting(instance)
end
function property.abi.prop_data_model_genre(instance)
end
function property.abi.prop_data_model_job_id(addr, instance)
	writeStringReturn(addr, getPointer(instance).JobId);
end
function property.abi.prop_data_model_place_id(instance)
	return getPointer(instance).PlaceId;
end
function property.abi.prop_data_model_place_version(instance)
	return getPointer(instance).PlaceVersion;
end
function property.abi.prop_data_model_private_server_id(addr, instance)
	writeStringReturn(addr, getPointer(instance).PrivateServerId);
end
function property.abi.prop_data_model_private_server_owner_id(instance)
	return getPointer(instance).PrivateServerOwnerId;
end
function property.abi.prop_data_model_vip_server_id(addr, instance)
	writeStringReturn(addr, getPointer(instance).VIPServerId);
end
function property.abi.prop_data_model_vip_server_owner_id(instance)
	return getPointer(instance).VIPServerOwnerId;
end
function property.abi.prop_data_model_workspace(addr, instance)
	local value = getPointer(instance).Workspace;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_sky_celestial_bodies_shown(instance)
	return getPointer(instance).CelestialBodiesShown and 1 or 0;
end
function property.abi.prop_set_sky_celestial_bodies_shown(instance, bool)
	getPointer(instance).CelestialBodiesShown = bool and 1 or 0;
end
function property.abi.prop_sky_moon_angular_size(instance)
	return getPointer(instance).MoonAngularSize;
end
function property.abi.prop_set_sky_moon_angular_size(instance, number)
	getPointer(instance).MoonAngularSize = number;
end
function property.abi.prop_sky_moon_texture_id(instance)
	return createPointer(getPointer(instance).MoonTextureId);
end
function property.abi.prop_set_sky_moon_texture_id(instance, content)
	getPointer(instance).MoonTextureId = getPointer(content);
end
function property.abi.prop_sky_skybox_bk(instance)
	return createPointer(getPointer(instance).SkyboxBk);
end
function property.abi.prop_set_sky_skybox_bk(instance, content)
	getPointer(instance).SkyboxBk = getPointer(content);
end
function property.abi.prop_sky_skybox_dn(instance)
	return createPointer(getPointer(instance).SkyboxDn);
end
function property.abi.prop_set_sky_skybox_dn(instance, content)
	getPointer(instance).SkyboxDn = getPointer(content);
end
function property.abi.prop_sky_skybox_ft(instance)
	return createPointer(getPointer(instance).SkyboxFt);
end
function property.abi.prop_set_sky_skybox_ft(instance, content)
	getPointer(instance).SkyboxFt = getPointer(content);
end
function property.abi.prop_sky_skybox_lf(instance)
	return createPointer(getPointer(instance).SkyboxLf);
end
function property.abi.prop_set_sky_skybox_lf(instance, content)
	getPointer(instance).SkyboxLf = getPointer(content);
end
function property.abi.prop_sky_skybox_rt(instance)
	return createPointer(getPointer(instance).SkyboxRt);
end
function property.abi.prop_set_sky_skybox_rt(instance, content)
	getPointer(instance).SkyboxRt = getPointer(content);
end
function property.abi.prop_sky_skybox_up(instance)
	return createPointer(getPointer(instance).SkyboxUp);
end
function property.abi.prop_set_sky_skybox_up(instance, content)
	getPointer(instance).SkyboxUp = getPointer(content);
end
function property.abi.prop_sky_star_count(instance)
	return getPointer(instance).StarCount;
end
function property.abi.prop_set_sky_star_count(instance, number)
	getPointer(instance).StarCount = number;
end
function property.abi.prop_sky_sun_angular_size(instance)
	return getPointer(instance).SunAngularSize;
end
function property.abi.prop_set_sky_sun_angular_size(instance, number)
	getPointer(instance).SunAngularSize = number;
end
function property.abi.prop_sky_sun_texture_id(instance)
	return createPointer(getPointer(instance).SunTextureId);
end
function property.abi.prop_set_sky_sun_texture_id(instance, content)
	getPointer(instance).SunTextureId = getPointer(content);
end
function property.abi.prop_smoke_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_smoke_color(instance, color_3)
	getPointer(instance).Color = getPointer(color_3);
end
function property.abi.prop_smoke_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_smoke_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_smoke_opacity(instance)
	return getPointer(instance).Opacity;
end
function property.abi.prop_set_smoke_opacity(instance, number)
	getPointer(instance).Opacity = number;
end
function property.abi.prop_smoke_rise_velocity(instance)
	return getPointer(instance).RiseVelocity;
end
function property.abi.prop_set_smoke_rise_velocity(instance, number)
	getPointer(instance).RiseVelocity = number;
end
function property.abi.prop_smoke_size(instance)
	return getPointer(instance).Size;
end
function property.abi.prop_set_smoke_size(instance, number)
	getPointer(instance).Size = number;
end
function property.abi.prop_smoke_time_scale(instance)
	return getPointer(instance).TimeScale;
end
function property.abi.prop_set_smoke_time_scale(instance, number)
	getPointer(instance).TimeScale = number;
end
function property.abi.prop_sound_emitter_size(instance)
	return getPointer(instance).EmitterSize;
end
function property.abi.prop_set_sound_emitter_size(instance, number)
	getPointer(instance).EmitterSize = number;
end
function property.abi.prop_sound_is_loaded(instance)
	return getPointer(instance).IsLoaded and 1 or 0;
end
function property.abi.prop_sound_is_paused(instance)
	return getPointer(instance).IsPaused and 1 or 0;
end
function property.abi.prop_sound_is_playing(instance)
	return getPointer(instance).IsPlaying and 1 or 0;
end
function property.abi.prop_sound_looped(instance)
	return getPointer(instance).Looped and 1 or 0;
end
function property.abi.prop_set_sound_looped(instance, bool)
	getPointer(instance).Looped = bool and 1 or 0;
end
function property.abi.prop_sound_max_distance(instance)
	return getPointer(instance).MaxDistance;
end
function property.abi.prop_set_sound_max_distance(instance, number)
	getPointer(instance).MaxDistance = number;
end
function property.abi.prop_sound_min_distance(instance)
	return getPointer(instance).MinDistance;
end
function property.abi.prop_set_sound_min_distance(instance, number)
	getPointer(instance).MinDistance = number;
end
function property.abi.prop_sound_pitch(instance)
	return getPointer(instance).Pitch;
end
function property.abi.prop_set_sound_pitch(instance, number)
	getPointer(instance).Pitch = number;
end
function property.abi.prop_sound_play_on_remove(instance)
	return getPointer(instance).PlayOnRemove and 1 or 0;
end
function property.abi.prop_set_sound_play_on_remove(instance, bool)
	getPointer(instance).PlayOnRemove = bool and 1 or 0;
end
function property.abi.prop_sound_playback_loudness(instance)
	return getPointer(instance).PlaybackLoudness;
end
function property.abi.prop_sound_playback_speed(instance)
	return getPointer(instance).PlaybackSpeed;
end
function property.abi.prop_set_sound_playback_speed(instance, number)
	getPointer(instance).PlaybackSpeed = number;
end
function property.abi.prop_sound_playing(instance)
	return getPointer(instance).Playing and 1 or 0;
end
function property.abi.prop_set_sound_playing(instance, bool)
	getPointer(instance).Playing = bool and 1 or 0;
end
function property.abi.prop_sound_roll_off_max_distance(instance)
	return getPointer(instance).RollOffMaxDistance;
end
function property.abi.prop_set_sound_roll_off_max_distance(instance, number)
	getPointer(instance).RollOffMaxDistance = number;
end
function property.abi.prop_sound_roll_off_min_distance(instance)
	return getPointer(instance).RollOffMinDistance;
end
function property.abi.prop_set_sound_roll_off_min_distance(instance, number)
	getPointer(instance).RollOffMinDistance = number;
end
function property.abi.prop_sound_roll_off_mode(instance)
end
function property.abi.prop_sound_sound_group(addr, instance)
	local value = getPointer(instance).SoundGroup;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_sound_sound_group(instance, isSome, value)
	getPointer(instance).SoundGroup = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_sound_sound_id(instance)
	return createPointer(getPointer(instance).SoundId);
end
function property.abi.prop_set_sound_sound_id(instance, content)
	getPointer(instance).SoundId = getPointer(content);
end
function property.abi.prop_sound_time_length(instance)
	return getPointer(instance).TimeLength;
end
function property.abi.prop_sound_time_position(instance)
	return getPointer(instance).TimePosition;
end
function property.abi.prop_set_sound_time_position(instance, number)
	getPointer(instance).TimePosition = number;
end
function property.abi.prop_sound_volume(instance)
	return getPointer(instance).Volume;
end
function property.abi.prop_set_sound_volume(instance, number)
	getPointer(instance).Volume = number;
end
function property.abi.prop_sound_effect_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_sound_effect_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_sound_effect_priority(instance)
	return getPointer(instance).Priority;
end
function property.abi.prop_set_sound_effect_priority(instance, number)
	getPointer(instance).Priority = number;
end
function property.abi.prop_chorus_sound_effect_depth(instance)
	return getPointer(instance).Depth;
end
function property.abi.prop_set_chorus_sound_effect_depth(instance, number)
	getPointer(instance).Depth = number;
end
function property.abi.prop_chorus_sound_effect_mix(instance)
	return getPointer(instance).Mix;
end
function property.abi.prop_set_chorus_sound_effect_mix(instance, number)
	getPointer(instance).Mix = number;
end
function property.abi.prop_chorus_sound_effect_rate(instance)
	return getPointer(instance).Rate;
end
function property.abi.prop_set_chorus_sound_effect_rate(instance, number)
	getPointer(instance).Rate = number;
end
function property.abi.prop_compressor_sound_effect_attack(instance)
	return getPointer(instance).Attack;
end
function property.abi.prop_set_compressor_sound_effect_attack(instance, number)
	getPointer(instance).Attack = number;
end
function property.abi.prop_compressor_sound_effect_gain_makeup(instance)
	return getPointer(instance).GainMakeup;
end
function property.abi.prop_set_compressor_sound_effect_gain_makeup(instance, number)
	getPointer(instance).GainMakeup = number;
end
function property.abi.prop_compressor_sound_effect_ratio(instance)
	return getPointer(instance).Ratio;
end
function property.abi.prop_set_compressor_sound_effect_ratio(instance, number)
	getPointer(instance).Ratio = number;
end
function property.abi.prop_compressor_sound_effect_release(instance)
	return getPointer(instance).Release;
end
function property.abi.prop_set_compressor_sound_effect_release(instance, number)
	getPointer(instance).Release = number;
end
function property.abi.prop_compressor_sound_effect_side_chain(addr, instance)
	local value = getPointer(instance).SideChain;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_compressor_sound_effect_side_chain(instance, isSome, value)
	getPointer(instance).SideChain = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_compressor_sound_effect_threshold(instance)
	return getPointer(instance).Threshold;
end
function property.abi.prop_set_compressor_sound_effect_threshold(instance, number)
	getPointer(instance).Threshold = number;
end
function property.abi.prop_channel_selector_sound_effect_channel(instance)
	return getPointer(instance).Channel;
end
function property.abi.prop_set_channel_selector_sound_effect_channel(instance, number)
	getPointer(instance).Channel = number;
end
function property.abi.prop_distortion_sound_effect_level(instance)
	return getPointer(instance).Level;
end
function property.abi.prop_set_distortion_sound_effect_level(instance, number)
	getPointer(instance).Level = number;
end
function property.abi.prop_echo_sound_effect_delay(instance)
	return getPointer(instance).Delay;
end
function property.abi.prop_set_echo_sound_effect_delay(instance, number)
	getPointer(instance).Delay = number;
end
function property.abi.prop_echo_sound_effect_dry_level(instance)
	return getPointer(instance).DryLevel;
end
function property.abi.prop_set_echo_sound_effect_dry_level(instance, number)
	getPointer(instance).DryLevel = number;
end
function property.abi.prop_echo_sound_effect_feedback(instance)
	return getPointer(instance).Feedback;
end
function property.abi.prop_set_echo_sound_effect_feedback(instance, number)
	getPointer(instance).Feedback = number;
end
function property.abi.prop_echo_sound_effect_wet_level(instance)
	return getPointer(instance).WetLevel;
end
function property.abi.prop_set_echo_sound_effect_wet_level(instance, number)
	getPointer(instance).WetLevel = number;
end
function property.abi.prop_equalizer_sound_effect_high_gain(instance)
	return getPointer(instance).HighGain;
end
function property.abi.prop_set_equalizer_sound_effect_high_gain(instance, number)
	getPointer(instance).HighGain = number;
end
function property.abi.prop_equalizer_sound_effect_low_gain(instance)
	return getPointer(instance).LowGain;
end
function property.abi.prop_set_equalizer_sound_effect_low_gain(instance, number)
	getPointer(instance).LowGain = number;
end
function property.abi.prop_equalizer_sound_effect_mid_gain(instance)
	return getPointer(instance).MidGain;
end
function property.abi.prop_set_equalizer_sound_effect_mid_gain(instance, number)
	getPointer(instance).MidGain = number;
end
function property.abi.prop_flange_sound_effect_depth(instance)
	return getPointer(instance).Depth;
end
function property.abi.prop_set_flange_sound_effect_depth(instance, number)
	getPointer(instance).Depth = number;
end
function property.abi.prop_flange_sound_effect_mix(instance)
	return getPointer(instance).Mix;
end
function property.abi.prop_set_flange_sound_effect_mix(instance, number)
	getPointer(instance).Mix = number;
end
function property.abi.prop_flange_sound_effect_rate(instance)
	return getPointer(instance).Rate;
end
function property.abi.prop_set_flange_sound_effect_rate(instance, number)
	getPointer(instance).Rate = number;
end
function property.abi.prop_pitch_shift_sound_effect_octave(instance)
	return getPointer(instance).Octave;
end
function property.abi.prop_set_pitch_shift_sound_effect_octave(instance, number)
	getPointer(instance).Octave = number;
end
function property.abi.prop_reverb_sound_effect_decay_time(instance)
	return getPointer(instance).DecayTime;
end
function property.abi.prop_set_reverb_sound_effect_decay_time(instance, number)
	getPointer(instance).DecayTime = number;
end
function property.abi.prop_reverb_sound_effect_density(instance)
	return getPointer(instance).Density;
end
function property.abi.prop_set_reverb_sound_effect_density(instance, number)
	getPointer(instance).Density = number;
end
function property.abi.prop_reverb_sound_effect_diffusion(instance)
	return getPointer(instance).Diffusion;
end
function property.abi.prop_set_reverb_sound_effect_diffusion(instance, number)
	getPointer(instance).Diffusion = number;
end
function property.abi.prop_reverb_sound_effect_dry_level(instance)
	return getPointer(instance).DryLevel;
end
function property.abi.prop_set_reverb_sound_effect_dry_level(instance, number)
	getPointer(instance).DryLevel = number;
end
function property.abi.prop_reverb_sound_effect_wet_level(instance)
	return getPointer(instance).WetLevel;
end
function property.abi.prop_set_reverb_sound_effect_wet_level(instance, number)
	getPointer(instance).WetLevel = number;
end
function property.abi.prop_tremolo_sound_effect_depth(instance)
	return getPointer(instance).Depth;
end
function property.abi.prop_set_tremolo_sound_effect_depth(instance, number)
	getPointer(instance).Depth = number;
end
function property.abi.prop_tremolo_sound_effect_duty(instance)
	return getPointer(instance).Duty;
end
function property.abi.prop_set_tremolo_sound_effect_duty(instance, number)
	getPointer(instance).Duty = number;
end
function property.abi.prop_tremolo_sound_effect_frequency(instance)
	return getPointer(instance).Frequency;
end
function property.abi.prop_set_tremolo_sound_effect_frequency(instance, number)
	getPointer(instance).Frequency = number;
end
function property.abi.prop_sound_group_volume(instance)
	return getPointer(instance).Volume;
end
function property.abi.prop_set_sound_group_volume(instance, number)
	getPointer(instance).Volume = number;
end
function property.abi.prop_sound_service_ambient_reverb(instance)
end
function property.abi.prop_sound_service_distance_factor(instance)
	return getPointer(instance).DistanceFactor;
end
function property.abi.prop_set_sound_service_distance_factor(instance, number)
	getPointer(instance).DistanceFactor = number;
end
function property.abi.prop_sound_service_doppler_scale(instance)
	return getPointer(instance).DopplerScale;
end
function property.abi.prop_set_sound_service_doppler_scale(instance, number)
	getPointer(instance).DopplerScale = number;
end
function property.abi.prop_sound_service_respect_filtering_enabled(instance)
	return getPointer(instance).RespectFilteringEnabled and 1 or 0;
end
function property.abi.prop_set_sound_service_respect_filtering_enabled(instance, bool)
	getPointer(instance).RespectFilteringEnabled = bool and 1 or 0;
end
function property.abi.prop_sound_service_rolloff_scale(instance)
	return getPointer(instance).RolloffScale;
end
function property.abi.prop_set_sound_service_rolloff_scale(instance, number)
	getPointer(instance).RolloffScale = number;
end
function property.abi.prop_sparkles_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_sparkles_color(instance, color_3)
	getPointer(instance).Color = getPointer(color_3);
end
function property.abi.prop_sparkles_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_sparkles_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_sparkles_sparkle_color(instance)
	return createPointer(getPointer(instance).SparkleColor);
end
function property.abi.prop_set_sparkles_sparkle_color(instance, color_3)
	getPointer(instance).SparkleColor = getPointer(color_3);
end
function property.abi.prop_sparkles_time_scale(instance)
	return getPointer(instance).TimeScale;
end
function property.abi.prop_set_sparkles_time_scale(instance, number)
	getPointer(instance).TimeScale = number;
end
function property.abi.prop_speaker_channel_count(instance)
	return getPointer(instance).ChannelCount;
end
function property.abi.prop_speaker_playback_loudness(instance)
	return getPointer(instance).PlaybackLoudness;
end
function property.abi.prop_speaker_roll_off_max_distance(instance)
	return getPointer(instance).RollOffMaxDistance;
end
function property.abi.prop_set_speaker_roll_off_max_distance(instance, number)
	getPointer(instance).RollOffMaxDistance = number;
end
function property.abi.prop_speaker_roll_off_min_distance(instance)
	return getPointer(instance).RollOffMinDistance;
end
function property.abi.prop_set_speaker_roll_off_min_distance(instance, number)
	getPointer(instance).RollOffMinDistance = number;
end
function property.abi.prop_speaker_roll_off_mode(instance)
end
function property.abi.prop_speaker_sound_group(addr, instance)
	local value = getPointer(instance).SoundGroup;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_speaker_sound_group(instance, isSome, value)
	getPointer(instance).SoundGroup = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_speaker_source(addr, instance)
	local value = getPointer(instance).Source;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_speaker_source(instance, isSome, value)
	getPointer(instance).Source = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_speaker_volume(instance)
	return getPointer(instance).Volume;
end
function property.abi.prop_set_speaker_volume(instance, number)
	getPointer(instance).Volume = number;
end
function property.abi.prop_starter_player_allow_custom_animations(instance)
	return getPointer(instance).AllowCustomAnimations and 1 or 0;
end
function property.abi.prop_starter_player_auto_jump_enabled(instance)
	return getPointer(instance).AutoJumpEnabled and 1 or 0;
end
function property.abi.prop_set_starter_player_auto_jump_enabled(instance, bool)
	getPointer(instance).AutoJumpEnabled = bool and 1 or 0;
end
function property.abi.prop_starter_player_camera_max_zoom_distance(instance)
	return getPointer(instance).CameraMaxZoomDistance;
end
function property.abi.prop_set_starter_player_camera_max_zoom_distance(instance, number)
	getPointer(instance).CameraMaxZoomDistance = number;
end
function property.abi.prop_starter_player_camera_min_zoom_distance(instance)
	return getPointer(instance).CameraMinZoomDistance;
end
function property.abi.prop_set_starter_player_camera_min_zoom_distance(instance, number)
	getPointer(instance).CameraMinZoomDistance = number;
end
function property.abi.prop_starter_player_camera_mode(instance)
end
function property.abi.prop_starter_player_character_jump_height(instance)
	return getPointer(instance).CharacterJumpHeight;
end
function property.abi.prop_set_starter_player_character_jump_height(instance, number)
	getPointer(instance).CharacterJumpHeight = number;
end
function property.abi.prop_starter_player_character_jump_power(instance)
	return getPointer(instance).CharacterJumpPower;
end
function property.abi.prop_set_starter_player_character_jump_power(instance, number)
	getPointer(instance).CharacterJumpPower = number;
end
function property.abi.prop_starter_player_character_max_slope_angle(instance)
	return getPointer(instance).CharacterMaxSlopeAngle;
end
function property.abi.prop_set_starter_player_character_max_slope_angle(instance, number)
	getPointer(instance).CharacterMaxSlopeAngle = number;
end
function property.abi.prop_starter_player_character_use_jump_power(instance)
	return getPointer(instance).CharacterUseJumpPower and 1 or 0;
end
function property.abi.prop_set_starter_player_character_use_jump_power(instance, bool)
	getPointer(instance).CharacterUseJumpPower = bool and 1 or 0;
end
function property.abi.prop_starter_player_character_walk_speed(instance)
	return getPointer(instance).CharacterWalkSpeed;
end
function property.abi.prop_set_starter_player_character_walk_speed(instance, number)
	getPointer(instance).CharacterWalkSpeed = number;
end
function property.abi.prop_starter_player_dev_camera_occlusion_mode(instance)
end
function property.abi.prop_starter_player_dev_computer_camera_movement_mode(instance)
end
function property.abi.prop_starter_player_dev_computer_movement_mode(instance)
end
function property.abi.prop_starter_player_dev_touch_camera_movement_mode(instance)
end
function property.abi.prop_starter_player_dev_touch_movement_mode(instance)
end
function property.abi.prop_starter_player_enable_mouse_lock_option(instance)
	return getPointer(instance).EnableMouseLockOption and 1 or 0;
end
function property.abi.prop_set_starter_player_enable_mouse_lock_option(instance, bool)
	getPointer(instance).EnableMouseLockOption = bool and 1 or 0;
end
function property.abi.prop_starter_player_health_display_distance(instance)
	return getPointer(instance).HealthDisplayDistance;
end
function property.abi.prop_set_starter_player_health_display_distance(instance, number)
	getPointer(instance).HealthDisplayDistance = number;
end
function property.abi.prop_starter_player_load_character_appearance(instance)
	return getPointer(instance).LoadCharacterAppearance and 1 or 0;
end
function property.abi.prop_set_starter_player_load_character_appearance(instance, bool)
	getPointer(instance).LoadCharacterAppearance = bool and 1 or 0;
end
function property.abi.prop_starter_player_name_display_distance(instance)
	return getPointer(instance).NameDisplayDistance;
end
function property.abi.prop_set_starter_player_name_display_distance(instance, number)
	getPointer(instance).NameDisplayDistance = number;
end
function property.abi.prop_starter_player_user_emotes_enabled(instance)
	return getPointer(instance).UserEmotesEnabled and 1 or 0;
end
function property.abi.prop_set_starter_player_user_emotes_enabled(instance, bool)
	getPointer(instance).UserEmotesEnabled = bool and 1 or 0;
end
function property.abi.prop_stats_contacts_count(instance)
	return getPointer(instance).ContactsCount;
end
function property.abi.prop_stats_data_receive_kbps(instance)
	return getPointer(instance).DataReceiveKbps;
end
function property.abi.prop_stats_data_send_kbps(instance)
	return getPointer(instance).DataSendKbps;
end
function property.abi.prop_stats_heartbeat_time_ms(instance)
	return getPointer(instance).HeartbeatTimeMs;
end
function property.abi.prop_stats_instance_count(instance)
	return getPointer(instance).InstanceCount;
end
function property.abi.prop_stats_moving_primitives_count(instance)
	return getPointer(instance).MovingPrimitivesCount;
end
function property.abi.prop_stats_physics_receive_kbps(instance)
	return getPointer(instance).PhysicsReceiveKbps;
end
function property.abi.prop_stats_physics_send_kbps(instance)
	return getPointer(instance).PhysicsSendKbps;
end
function property.abi.prop_stats_physics_step_time_ms(instance)
	return getPointer(instance).PhysicsStepTimeMs;
end
function property.abi.prop_stats_primitives_count(instance)
	return getPointer(instance).PrimitivesCount;
end
function property.abi.prop_team_auto_assignable(instance)
	return getPointer(instance).AutoAssignable and 1 or 0;
end
function property.abi.prop_set_team_auto_assignable(instance, bool)
	getPointer(instance).AutoAssignable = bool and 1 or 0;
end
function property.abi.prop_team_auto_color_characters(instance)
	return getPointer(instance).AutoColorCharacters and 1 or 0;
end
function property.abi.prop_set_team_auto_color_characters(instance, bool)
	getPointer(instance).AutoColorCharacters = bool and 1 or 0;
end
function property.abi.prop_team_score(instance)
	return getPointer(instance).Score;
end
function property.abi.prop_set_team_score(instance, number)
	getPointer(instance).Score = number;
end
function property.abi.prop_team_team_color(instance)
	return createPointer(getPointer(instance).TeamColor);
end
function property.abi.prop_set_team_team_color(instance, brick_color)
	getPointer(instance).TeamColor = getPointer(brick_color);
end
function property.abi.prop_teleport_async_result_private_server_id(addr, instance)
	writeStringReturn(addr, getPointer(instance).PrivateServerId);
end
function property.abi.prop_teleport_async_result_reserved_server_access_code(addr, instance)
	writeStringReturn(addr, getPointer(instance).ReservedServerAccessCode);
end
function property.abi.prop_teleport_options_reserved_server_access_code(addr, instance)
	writeStringReturn(addr, getPointer(instance).ReservedServerAccessCode);
end
function property.abi.prop_set_teleport_options_reserved_server_access_code(instance, addr, length)
	getPointer(instance).ReservedServerAccessCode = readString(addr, length);
end
function property.abi.prop_teleport_options_server_instance_id(addr, instance)
	writeStringReturn(addr, getPointer(instance).ServerInstanceId);
end
function property.abi.prop_set_teleport_options_server_instance_id(instance, addr, length)
	getPointer(instance).ServerInstanceId = readString(addr, length);
end
function property.abi.prop_teleport_options_should_reserve_server(instance)
	return getPointer(instance).ShouldReserveServer and 1 or 0;
end
function property.abi.prop_set_teleport_options_should_reserve_server(instance, bool)
	getPointer(instance).ShouldReserveServer = bool and 1 or 0;
end
function property.abi.prop_teleport_service_customized_teleport_ui(instance)
	return getPointer(instance).CustomizedTeleportUI and 1 or 0;
end
function property.abi.prop_set_teleport_service_customized_teleport_ui(instance, bool)
	getPointer(instance).CustomizedTeleportUI = bool and 1 or 0;
end
function property.abi.prop_terrain_detail_face(instance)
end
function property.abi.prop_terrain_detail_material_pattern(instance)
end
function property.abi.prop_terrain_detail_studs_per_tile(instance)
	return getPointer(instance).StudsPerTile;
end
function property.abi.prop_set_terrain_detail_studs_per_tile(instance, number)
	getPointer(instance).StudsPerTile = number;
end
function property.abi.prop_terrain_region_is_smooth(instance)
	return getPointer(instance).IsSmooth and 1 or 0;
end
function property.abi.prop_terrain_region_size_in_cells(instance)
	return createPointer(getPointer(instance).SizeInCells);
end
function property.abi.prop_text_chat_command_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_text_chat_command_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_text_chat_command_primary_alias(addr, instance)
	writeStringReturn(addr, getPointer(instance).PrimaryAlias);
end
function property.abi.prop_set_text_chat_command_primary_alias(instance, addr, length)
	getPointer(instance).PrimaryAlias = readString(addr, length);
end
function property.abi.prop_text_chat_command_secondary_alias(addr, instance)
	writeStringReturn(addr, getPointer(instance).SecondaryAlias);
end
function property.abi.prop_set_text_chat_command_secondary_alias(instance, addr, length)
	getPointer(instance).SecondaryAlias = readString(addr, length);
end
function property.abi.prop_chat_input_bar_configuration_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_chat_input_bar_configuration_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_chat_input_bar_configuration_target_text_channel(addr, instance)
	local value = getPointer(instance).TargetTextChannel;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_chat_input_bar_configuration_target_text_channel(instance, isSome, value)
	getPointer(instance).TargetTextChannel = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_chat_window_configuration_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_chat_window_configuration_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_text_chat_message_message_id(addr, instance)
	writeStringReturn(addr, getPointer(instance).MessageId);
end
function property.abi.prop_set_text_chat_message_message_id(instance, addr, length)
	getPointer(instance).MessageId = readString(addr, length);
end
function property.abi.prop_text_chat_message_metadata(addr, instance)
	writeStringReturn(addr, getPointer(instance).Metadata);
end
function property.abi.prop_set_text_chat_message_metadata(instance, addr, length)
	getPointer(instance).Metadata = readString(addr, length);
end
function property.abi.prop_text_chat_message_prefix_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).PrefixText);
end
function property.abi.prop_set_text_chat_message_prefix_text(instance, addr, length)
	getPointer(instance).PrefixText = readString(addr, length);
end
function property.abi.prop_text_chat_message_status(instance)
end
function property.abi.prop_text_chat_message_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).Text);
end
function property.abi.prop_set_text_chat_message_text(instance, addr, length)
	getPointer(instance).Text = readString(addr, length);
end
function property.abi.prop_text_chat_message_text_channel(addr, instance)
	local value = getPointer(instance).TextChannel;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_text_chat_message_text_channel(instance, isSome, value)
	getPointer(instance).TextChannel = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_text_chat_message_text_source(addr, instance)
	local value = getPointer(instance).TextSource;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_text_chat_message_text_source(instance, isSome, value)
	getPointer(instance).TextSource = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_text_chat_message_timestamp(instance)
	return createPointer(getPointer(instance).Timestamp);
end
function property.abi.prop_set_text_chat_message_timestamp(instance, date_time)
	getPointer(instance).Timestamp = getPointer(date_time);
end
function property.abi.prop_text_chat_message_properties_prefix_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).PrefixText);
end
function property.abi.prop_set_text_chat_message_properties_prefix_text(instance, addr, length)
	getPointer(instance).PrefixText = readString(addr, length);
end
function property.abi.prop_text_chat_message_properties_text(addr, instance)
	writeStringReturn(addr, getPointer(instance).Text);
end
function property.abi.prop_set_text_chat_message_properties_text(instance, addr, length)
	getPointer(instance).Text = readString(addr, length);
end
function property.abi.prop_text_chat_service_chat_version(instance)
end
function property.abi.prop_text_chat_service_create_default_commands(instance)
	return getPointer(instance).CreateDefaultCommands and 1 or 0;
end
function property.abi.prop_text_chat_service_create_default_text_channels(instance)
	return getPointer(instance).CreateDefaultTextChannels and 1 or 0;
end
function property.abi.prop_text_source_can_send(instance)
	return getPointer(instance).CanSend and 1 or 0;
end
function property.abi.prop_set_text_source_can_send(instance, bool)
	getPointer(instance).CanSend = bool and 1 or 0;
end
function property.abi.prop_text_source_user_id(instance)
	return getPointer(instance).UserId;
end
function property.abi.prop_trail_attachment_0(addr, instance)
	local value = getPointer(instance).Attachment0;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_trail_attachment_0(instance, isSome, value)
	getPointer(instance).Attachment0 = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_trail_attachment_1(addr, instance)
	local value = getPointer(instance).Attachment1;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_trail_attachment_1(instance, isSome, value)
	getPointer(instance).Attachment1 = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_trail_brightness(instance)
	return getPointer(instance).Brightness;
end
function property.abi.prop_set_trail_brightness(instance, number)
	getPointer(instance).Brightness = number;
end
function property.abi.prop_trail_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_trail_color(instance, color_sequence)
	getPointer(instance).Color = getPointer(color_sequence);
end
function property.abi.prop_trail_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_trail_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_trail_face_camera(instance)
	return getPointer(instance).FaceCamera and 1 or 0;
end
function property.abi.prop_set_trail_face_camera(instance, bool)
	getPointer(instance).FaceCamera = bool and 1 or 0;
end
function property.abi.prop_trail_lifetime(instance)
	return getPointer(instance).Lifetime;
end
function property.abi.prop_set_trail_lifetime(instance, number)
	getPointer(instance).Lifetime = number;
end
function property.abi.prop_trail_light_emission(instance)
	return getPointer(instance).LightEmission;
end
function property.abi.prop_set_trail_light_emission(instance, number)
	getPointer(instance).LightEmission = number;
end
function property.abi.prop_trail_light_influence(instance)
	return getPointer(instance).LightInfluence;
end
function property.abi.prop_set_trail_light_influence(instance, number)
	getPointer(instance).LightInfluence = number;
end
function property.abi.prop_trail_max_length(instance)
	return getPointer(instance).MaxLength;
end
function property.abi.prop_set_trail_max_length(instance, number)
	getPointer(instance).MaxLength = number;
end
function property.abi.prop_trail_min_length(instance)
	return getPointer(instance).MinLength;
end
function property.abi.prop_set_trail_min_length(instance, number)
	getPointer(instance).MinLength = number;
end
function property.abi.prop_trail_texture(instance)
	return createPointer(getPointer(instance).Texture);
end
function property.abi.prop_set_trail_texture(instance, content)
	getPointer(instance).Texture = getPointer(content);
end
function property.abi.prop_trail_texture_length(instance)
	return getPointer(instance).TextureLength;
end
function property.abi.prop_set_trail_texture_length(instance, number)
	getPointer(instance).TextureLength = number;
end
function property.abi.prop_trail_texture_mode(instance)
end
function property.abi.prop_trail_transparency(instance)
	return createPointer(getPointer(instance).Transparency);
end
function property.abi.prop_set_trail_transparency(instance, number_sequence)
	getPointer(instance).Transparency = getPointer(number_sequence);
end
function property.abi.prop_trail_width_scale(instance)
	return createPointer(getPointer(instance).WidthScale);
end
function property.abi.prop_set_trail_width_scale(instance, number_sequence)
	getPointer(instance).WidthScale = getPointer(number_sequence);
end
function property.abi.prop_translator_locale_id(addr, instance)
	writeStringReturn(addr, getPointer(instance).LocaleId);
end
function property.abi.prop_tween_base_playback_state(instance)
end
function property.abi.prop_tween_instance(addr, instance)
	local value = getPointer(instance).Instance;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_tween_tween_info(instance)
	return createPointer(getPointer(instance).TweenInfo);
end
function property.abi.prop_ui_aspect_ratio_constraint_aspect_ratio(instance)
	return getPointer(instance).AspectRatio;
end
function property.abi.prop_set_ui_aspect_ratio_constraint_aspect_ratio(instance, number)
	getPointer(instance).AspectRatio = number;
end
function property.abi.prop_ui_aspect_ratio_constraint_aspect_type(instance)
end
function property.abi.prop_ui_aspect_ratio_constraint_dominant_axis(instance)
end
function property.abi.prop_ui_size_constraint_max_size(instance)
	return createPointer(getPointer(instance).MaxSize);
end
function property.abi.prop_set_ui_size_constraint_max_size(instance, vector_2)
	getPointer(instance).MaxSize = getPointer(vector_2);
end
function property.abi.prop_ui_size_constraint_min_size(instance)
	return createPointer(getPointer(instance).MinSize);
end
function property.abi.prop_set_ui_size_constraint_min_size(instance, vector_2)
	getPointer(instance).MinSize = getPointer(vector_2);
end
function property.abi.prop_ui_text_size_constraint_max_text_size(instance)
	return getPointer(instance).MaxTextSize;
end
function property.abi.prop_set_ui_text_size_constraint_max_text_size(instance, number)
	getPointer(instance).MaxTextSize = number;
end
function property.abi.prop_ui_text_size_constraint_min_text_size(instance)
	return getPointer(instance).MinTextSize;
end
function property.abi.prop_set_ui_text_size_constraint_min_text_size(instance, number)
	getPointer(instance).MinTextSize = number;
end
function property.abi.prop_ui_corner_corner_radius(instance)
	return createPointer(getPointer(instance).CornerRadius);
end
function property.abi.prop_set_ui_corner_corner_radius(instance, u_dim)
	getPointer(instance).CornerRadius = getPointer(u_dim);
end
function property.abi.prop_ui_gradient_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_ui_gradient_color(instance, color_sequence)
	getPointer(instance).Color = getPointer(color_sequence);
end
function property.abi.prop_ui_gradient_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_ui_gradient_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_ui_gradient_offset(instance)
	return createPointer(getPointer(instance).Offset);
end
function property.abi.prop_set_ui_gradient_offset(instance, vector_2)
	getPointer(instance).Offset = getPointer(vector_2);
end
function property.abi.prop_ui_gradient_rotation(instance)
	return getPointer(instance).Rotation;
end
function property.abi.prop_set_ui_gradient_rotation(instance, number)
	getPointer(instance).Rotation = number;
end
function property.abi.prop_ui_gradient_transparency(instance)
	return createPointer(getPointer(instance).Transparency);
end
function property.abi.prop_set_ui_gradient_transparency(instance, number_sequence)
	getPointer(instance).Transparency = getPointer(number_sequence);
end
function property.abi.prop_ui_grid_style_layout_absolute_content_size(instance)
	return createPointer(getPointer(instance).AbsoluteContentSize);
end
function property.abi.prop_ui_grid_style_layout_fill_direction(instance)
end
function property.abi.prop_ui_grid_style_layout_horizontal_alignment(instance)
end
function property.abi.prop_ui_grid_style_layout_sort_order(instance)
end
function property.abi.prop_ui_grid_style_layout_vertical_alignment(instance)
end
function property.abi.prop_ui_grid_layout_absolute_cell_count(instance)
	return createPointer(getPointer(instance).AbsoluteCellCount);
end
function property.abi.prop_ui_grid_layout_absolute_cell_size(instance)
	return createPointer(getPointer(instance).AbsoluteCellSize);
end
function property.abi.prop_ui_grid_layout_cell_padding(instance)
	return createPointer(getPointer(instance).CellPadding);
end
function property.abi.prop_set_ui_grid_layout_cell_padding(instance, u_dim_2)
	getPointer(instance).CellPadding = getPointer(u_dim_2);
end
function property.abi.prop_ui_grid_layout_cell_size(instance)
	return createPointer(getPointer(instance).CellSize);
end
function property.abi.prop_set_ui_grid_layout_cell_size(instance, u_dim_2)
	getPointer(instance).CellSize = getPointer(u_dim_2);
end
function property.abi.prop_ui_grid_layout_fill_direction_max_cells(instance)
	return getPointer(instance).FillDirectionMaxCells;
end
function property.abi.prop_set_ui_grid_layout_fill_direction_max_cells(instance, number)
	getPointer(instance).FillDirectionMaxCells = number;
end
function property.abi.prop_ui_grid_layout_start_corner(instance)
end
function property.abi.prop_ui_list_layout_padding(instance)
	return createPointer(getPointer(instance).Padding);
end
function property.abi.prop_set_ui_list_layout_padding(instance, u_dim)
	getPointer(instance).Padding = getPointer(u_dim);
end
function property.abi.prop_ui_page_layout_animated(instance)
	return getPointer(instance).Animated and 1 or 0;
end
function property.abi.prop_set_ui_page_layout_animated(instance, bool)
	getPointer(instance).Animated = bool and 1 or 0;
end
function property.abi.prop_ui_page_layout_circular(instance)
	return getPointer(instance).Circular and 1 or 0;
end
function property.abi.prop_set_ui_page_layout_circular(instance, bool)
	getPointer(instance).Circular = bool and 1 or 0;
end
function property.abi.prop_ui_page_layout_current_page(addr, instance)
	local value = getPointer(instance).CurrentPage;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_ui_page_layout_easing_direction(instance)
end
function property.abi.prop_ui_page_layout_easing_style(instance)
end
function property.abi.prop_ui_page_layout_gamepad_input_enabled(instance)
	return getPointer(instance).GamepadInputEnabled and 1 or 0;
end
function property.abi.prop_set_ui_page_layout_gamepad_input_enabled(instance, bool)
	getPointer(instance).GamepadInputEnabled = bool and 1 or 0;
end
function property.abi.prop_ui_page_layout_padding(instance)
	return createPointer(getPointer(instance).Padding);
end
function property.abi.prop_set_ui_page_layout_padding(instance, u_dim)
	getPointer(instance).Padding = getPointer(u_dim);
end
function property.abi.prop_ui_page_layout_scroll_wheel_input_enabled(instance)
	return getPointer(instance).ScrollWheelInputEnabled and 1 or 0;
end
function property.abi.prop_set_ui_page_layout_scroll_wheel_input_enabled(instance, bool)
	getPointer(instance).ScrollWheelInputEnabled = bool and 1 or 0;
end
function property.abi.prop_ui_page_layout_touch_input_enabled(instance)
	return getPointer(instance).TouchInputEnabled and 1 or 0;
end
function property.abi.prop_set_ui_page_layout_touch_input_enabled(instance, bool)
	getPointer(instance).TouchInputEnabled = bool and 1 or 0;
end
function property.abi.prop_ui_page_layout_tween_time(instance)
	return getPointer(instance).TweenTime;
end
function property.abi.prop_set_ui_page_layout_tween_time(instance, number)
	getPointer(instance).TweenTime = number;
end
function property.abi.prop_ui_table_layout_fill_empty_space_columns(instance)
	return getPointer(instance).FillEmptySpaceColumns and 1 or 0;
end
function property.abi.prop_set_ui_table_layout_fill_empty_space_columns(instance, bool)
	getPointer(instance).FillEmptySpaceColumns = bool and 1 or 0;
end
function property.abi.prop_ui_table_layout_fill_empty_space_rows(instance)
	return getPointer(instance).FillEmptySpaceRows and 1 or 0;
end
function property.abi.prop_set_ui_table_layout_fill_empty_space_rows(instance, bool)
	getPointer(instance).FillEmptySpaceRows = bool and 1 or 0;
end
function property.abi.prop_ui_table_layout_major_axis(instance)
end
function property.abi.prop_ui_table_layout_padding(instance)
	return createPointer(getPointer(instance).Padding);
end
function property.abi.prop_set_ui_table_layout_padding(instance, u_dim_2)
	getPointer(instance).Padding = getPointer(u_dim_2);
end
function property.abi.prop_ui_padding_padding_bottom(instance)
	return createPointer(getPointer(instance).PaddingBottom);
end
function property.abi.prop_set_ui_padding_padding_bottom(instance, u_dim)
	getPointer(instance).PaddingBottom = getPointer(u_dim);
end
function property.abi.prop_ui_padding_padding_left(instance)
	return createPointer(getPointer(instance).PaddingLeft);
end
function property.abi.prop_set_ui_padding_padding_left(instance, u_dim)
	getPointer(instance).PaddingLeft = getPointer(u_dim);
end
function property.abi.prop_ui_padding_padding_right(instance)
	return createPointer(getPointer(instance).PaddingRight);
end
function property.abi.prop_set_ui_padding_padding_right(instance, u_dim)
	getPointer(instance).PaddingRight = getPointer(u_dim);
end
function property.abi.prop_ui_padding_padding_top(instance)
	return createPointer(getPointer(instance).PaddingTop);
end
function property.abi.prop_set_ui_padding_padding_top(instance, u_dim)
	getPointer(instance).PaddingTop = getPointer(u_dim);
end
function property.abi.prop_ui_scale_scale(instance)
	return getPointer(instance).Scale;
end
function property.abi.prop_set_ui_scale_scale(instance, number)
	getPointer(instance).Scale = number;
end
function property.abi.prop_ui_stroke_apply_stroke_mode(instance)
end
function property.abi.prop_ui_stroke_color(instance)
	return createPointer(getPointer(instance).Color);
end
function property.abi.prop_set_ui_stroke_color(instance, color_3)
	getPointer(instance).Color = getPointer(color_3);
end
function property.abi.prop_ui_stroke_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_ui_stroke_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_ui_stroke_line_join_mode(instance)
end
function property.abi.prop_ui_stroke_thickness(instance)
	return getPointer(instance).Thickness;
end
function property.abi.prop_set_ui_stroke_thickness(instance, number)
	getPointer(instance).Thickness = number;
end
function property.abi.prop_ui_stroke_transparency(instance)
	return getPointer(instance).Transparency;
end
function property.abi.prop_set_ui_stroke_transparency(instance, number)
	getPointer(instance).Transparency = number;
end
function property.abi.prop_user_game_settings_computer_camera_movement_mode(instance)
end
function property.abi.prop_user_game_settings_computer_movement_mode(instance)
end
function property.abi.prop_user_game_settings_control_mode(instance)
end
function property.abi.prop_user_game_settings_gamepad_camera_sensitivity(instance)
	return getPointer(instance).GamepadCameraSensitivity;
end
function property.abi.prop_set_user_game_settings_gamepad_camera_sensitivity(instance, number)
	getPointer(instance).GamepadCameraSensitivity = number;
end
function property.abi.prop_user_game_settings_mouse_sensitivity(instance)
	return getPointer(instance).MouseSensitivity;
end
function property.abi.prop_set_user_game_settings_mouse_sensitivity(instance, number)
	getPointer(instance).MouseSensitivity = number;
end
function property.abi.prop_user_game_settings_rcc_profiler_record_frame_rate(instance)
	return getPointer(instance).RCCProfilerRecordFrameRate;
end
function property.abi.prop_set_user_game_settings_rcc_profiler_record_frame_rate(instance, number)
	getPointer(instance).RCCProfilerRecordFrameRate = number;
end
function property.abi.prop_user_game_settings_rcc_profiler_record_time_frame(instance)
	return getPointer(instance).RCCProfilerRecordTimeFrame;
end
function property.abi.prop_set_user_game_settings_rcc_profiler_record_time_frame(instance, number)
	getPointer(instance).RCCProfilerRecordTimeFrame = number;
end
function property.abi.prop_user_game_settings_rotation_type(instance)
end
function property.abi.prop_user_game_settings_saved_quality_level(instance)
end
function property.abi.prop_user_game_settings_touch_camera_movement_mode(instance)
end
function property.abi.prop_user_game_settings_touch_movement_mode(instance)
end
function property.abi.prop_user_game_settings_vignette_enabled(instance)
	return getPointer(instance).VignetteEnabled and 1 or 0;
end
function property.abi.prop_user_input_service_accelerometer_enabled(instance)
	return getPointer(instance).AccelerometerEnabled and 1 or 0;
end
function property.abi.prop_user_input_service_gamepad_enabled(instance)
	return getPointer(instance).GamepadEnabled and 1 or 0;
end
function property.abi.prop_user_input_service_gyroscope_enabled(instance)
	return getPointer(instance).GyroscopeEnabled and 1 or 0;
end
function property.abi.prop_user_input_service_keyboard_enabled(instance)
	return getPointer(instance).KeyboardEnabled and 1 or 0;
end
function property.abi.prop_user_input_service_modal_enabled(instance)
	return getPointer(instance).ModalEnabled and 1 or 0;
end
function property.abi.prop_set_user_input_service_modal_enabled(instance, bool)
	getPointer(instance).ModalEnabled = bool and 1 or 0;
end
function property.abi.prop_user_input_service_mouse_behavior(instance)
end
function property.abi.prop_user_input_service_mouse_delta_sensitivity(instance)
	return getPointer(instance).MouseDeltaSensitivity;
end
function property.abi.prop_set_user_input_service_mouse_delta_sensitivity(instance, number)
	getPointer(instance).MouseDeltaSensitivity = number;
end
function property.abi.prop_user_input_service_mouse_enabled(instance)
	return getPointer(instance).MouseEnabled and 1 or 0;
end
function property.abi.prop_user_input_service_mouse_icon_enabled(instance)
	return getPointer(instance).MouseIconEnabled and 1 or 0;
end
function property.abi.prop_set_user_input_service_mouse_icon_enabled(instance, bool)
	getPointer(instance).MouseIconEnabled = bool and 1 or 0;
end
function property.abi.prop_user_input_service_on_screen_keyboard_position(instance)
	return createPointer(getPointer(instance).OnScreenKeyboardPosition);
end
function property.abi.prop_user_input_service_on_screen_keyboard_size(instance)
	return createPointer(getPointer(instance).OnScreenKeyboardSize);
end
function property.abi.prop_user_input_service_on_screen_keyboard_visible(instance)
	return getPointer(instance).OnScreenKeyboardVisible and 1 or 0;
end
function property.abi.prop_user_input_service_touch_enabled(instance)
	return getPointer(instance).TouchEnabled and 1 or 0;
end
function property.abi.prop_user_input_service_user_head_c_frame(instance)
	return createPointer(getPointer(instance).UserHeadCFrame);
end
function property.abi.prop_user_input_service_vr_enabled(instance)
	return getPointer(instance).VREnabled and 1 or 0;
end
function property.abi.prop_vr_service_gui_input_user_c_frame(instance)
end
function property.abi.prop_vr_service_vr_enabled(instance)
	return getPointer(instance).VREnabled and 1 or 0;
end
function property.abi.prop_bool_value_value(instance)
	return getPointer(instance).Value and 1 or 0;
end
function property.abi.prop_set_bool_value_value(instance, bool)
	getPointer(instance).Value = bool and 1 or 0;
end
function property.abi.prop_brick_color_value_value(instance)
	return createPointer(getPointer(instance).Value);
end
function property.abi.prop_set_brick_color_value_value(instance, brick_color)
	getPointer(instance).Value = getPointer(brick_color);
end
function property.abi.prop_c_frame_value_value(instance)
	return createPointer(getPointer(instance).Value);
end
function property.abi.prop_set_c_frame_value_value(instance, c_frame)
	getPointer(instance).Value = getPointer(c_frame);
end
function property.abi.prop_color_3_value_value(instance)
	return createPointer(getPointer(instance).Value);
end
function property.abi.prop_set_color_3_value_value(instance, color_3)
	getPointer(instance).Value = getPointer(color_3);
end
function property.abi.prop_double_constrained_value_constrained_value(instance)
	return getPointer(instance).ConstrainedValue;
end
function property.abi.prop_set_double_constrained_value_constrained_value(instance, number)
	getPointer(instance).ConstrainedValue = number;
end
function property.abi.prop_double_constrained_value_max_value(instance)
	return getPointer(instance).MaxValue;
end
function property.abi.prop_set_double_constrained_value_max_value(instance, number)
	getPointer(instance).MaxValue = number;
end
function property.abi.prop_double_constrained_value_min_value(instance)
	return getPointer(instance).MinValue;
end
function property.abi.prop_set_double_constrained_value_min_value(instance, number)
	getPointer(instance).MinValue = number;
end
function property.abi.prop_double_constrained_value_value(instance)
	return getPointer(instance).Value;
end
function property.abi.prop_set_double_constrained_value_value(instance, number)
	getPointer(instance).Value = number;
end
function property.abi.prop_int_constrained_value_constrained_value(instance)
	return getPointer(instance).ConstrainedValue;
end
function property.abi.prop_set_int_constrained_value_constrained_value(instance, number)
	getPointer(instance).ConstrainedValue = number;
end
function property.abi.prop_int_constrained_value_max_value(instance)
	return getPointer(instance).MaxValue;
end
function property.abi.prop_set_int_constrained_value_max_value(instance, number)
	getPointer(instance).MaxValue = number;
end
function property.abi.prop_int_constrained_value_min_value(instance)
	return getPointer(instance).MinValue;
end
function property.abi.prop_set_int_constrained_value_min_value(instance, number)
	getPointer(instance).MinValue = number;
end
function property.abi.prop_int_constrained_value_value(instance)
	return getPointer(instance).Value;
end
function property.abi.prop_set_int_constrained_value_value(instance, number)
	getPointer(instance).Value = number;
end
function property.abi.prop_int_value_value(instance)
	return getPointer(instance).Value;
end
function property.abi.prop_set_int_value_value(instance, number)
	getPointer(instance).Value = number;
end
function property.abi.prop_number_value_value(instance)
	return getPointer(instance).Value;
end
function property.abi.prop_set_number_value_value(instance, number)
	getPointer(instance).Value = number;
end
function property.abi.prop_object_value_value(addr, instance)
	local value = getPointer(instance).Value;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_object_value_value(instance, isSome, value)
	getPointer(instance).Value = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_ray_value_value(instance)
	return createPointer(getPointer(instance).Value);
end
function property.abi.prop_set_ray_value_value(instance, ray)
	getPointer(instance).Value = getPointer(ray);
end
function property.abi.prop_string_value_value(addr, instance)
	writeStringReturn(addr, getPointer(instance).Value);
end
function property.abi.prop_set_string_value_value(instance, addr, length)
	getPointer(instance).Value = readString(addr, length);
end
function property.abi.prop_vector_3_value_value(instance)
	return createPointer(getPointer(instance).Value);
end
function property.abi.prop_set_vector_3_value_value(instance, vector_3)
	getPointer(instance).Value = getPointer(vector_3);
end
function property.abi.prop_voice_chat_internal_voice_chat_state(instance)
end
function property.abi.prop_voice_source_user_id(instance)
	return getPointer(instance).UserId;
end
function property.abi.prop_weld_constraint_active(instance)
	return getPointer(instance).Active and 1 or 0;
end
function property.abi.prop_weld_constraint_enabled(instance)
	return getPointer(instance).Enabled and 1 or 0;
end
function property.abi.prop_set_weld_constraint_enabled(instance, bool)
	getPointer(instance).Enabled = bool and 1 or 0;
end
function property.abi.prop_weld_constraint_part_0(addr, instance)
	local value = getPointer(instance).Part0;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_weld_constraint_part_0(instance, isSome, value)
	getPointer(instance).Part0 = if isSome == 1 then getPointer(value) else nil;
end
function property.abi.prop_weld_constraint_part_1(addr, instance)
	local value = getPointer(instance).Part1;
	store_i32(memory, addr, value and 1 or 0);
	store_i32(memory, addr + 4, value and createPointer(value) or 0);
end
function property.abi.prop_set_weld_constraint_part_1(instance, isSome, value)
	getPointer(instance).Part1 = if isSome == 1 then getPointer(value) else nil;
end
return property;
