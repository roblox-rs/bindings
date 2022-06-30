#[allow(dead_code)]
#[allow(improper_ctypes)]
extern "C" {
	fn dyn_instance_clear_all_children(instance: u32, name: &str);
	fn dyn_instance_clone(instance: u32, name: &str) -> Option<Instance>;
	fn dyn_instance_destroy(instance: u32, name: &str);
	fn dyn_instance_find_first_ancestor(instance: u32, name: &str, p_name: &str) -> Option<Instance>;
	fn dyn_instance_find_first_ancestor_of_class(instance: u32, name: &str, p_className: &str) -> Option<Instance>;
	fn dyn_instance_find_first_ancestor_which_is_a(instance: u32, name: &str, p_className: &str) -> Option<Instance>;
	fn dyn_instance_find_first_child(instance: u32, name: &str, p_name: &str, p_recursive: bool) -> Option<Instance>;
	fn dyn_instance_find_first_child_of_class(instance: u32, name: &str, p_className: &str) -> Option<Instance>;
	fn dyn_instance_find_first_child_which_is_a(instance: u32, name: &str, p_className: &str, p_recursive: bool) -> Option<Instance>;
	fn dyn_instance_find_first_descendant(instance: u32, name: &str, p_name: &str) -> Option<Instance>;
	fn dyn_instance_get_actor(instance: u32, name: &str) -> Option<Actor>;
	fn dyn_instance_get_attribute(instance: u32, name: &str, p_attribute: &str);
	fn dyn_instance_get_attribute_changed_signal(instance: u32, name: &str, p_attribute: &str) -> RBXScriptSignal;
	fn dyn_instance_get_attributes(instance: u32, name: &str);
	fn dyn_instance_get_children(instance: u32, name: &str) -> Objects;
	fn dyn_instance_get_descendants(instance: u32, name: &str);
	fn dyn_instance_get_full_name(instance: u32, name: &str) -> String;
	fn dyn_instance_get_property_changed_signal(instance: u32, name: &str, p_property: &str) -> RBXScriptSignal;
	fn dyn_instance_is_a(instance: u32, name: &str, p_className: &str) -> bool;
	fn dyn_instance_is_ancestor_of(instance: u32, name: &str, p_descendant: Option<Instance>) -> bool;
	fn dyn_instance_is_descendant_of(instance: u32, name: &str, p_ancestor: Option<Instance>) -> bool;
	fn dyn_instance_set_attribute(instance: u32, name: &str, p_attribute: &str);
	fn dyn_instance_wait_for_child(instance: u32, name: &str, p_childName: &str, p_timeOut: f64) -> Option<Instance>;
	fn dyn_keyframe_sequence_add_keyframe(instance: u32, name: &str, p_keyframe: Option<Instance>);
	fn dyn_keyframe_sequence_get_keyframes(instance: u32, name: &str) -> Objects;
	fn dyn_keyframe_sequence_remove_keyframe(instance: u32, name: &str, p_keyframe: Option<Instance>);
	fn dyn_animation_track_adjust_speed(instance: u32, name: &str, p_speed: f64);
	fn dyn_animation_track_adjust_weight(instance: u32, name: &str, p_weight: f64, p_fadeTime: f64);
	fn dyn_animation_track_get_marker_reached_signal(instance: u32, name: &str, p_name: &str) -> RBXScriptSignal;
	fn dyn_animation_track_get_time_of_keyframe(instance: u32, name: &str, p_keyframeName: &str) -> f64;
	fn dyn_animation_track_play(instance: u32, name: &str, p_fadeTime: f64, p_weight: f64, p_speed: f64);
	fn dyn_animation_track_stop(instance: u32, name: &str, p_fadeTime: f64);
	fn dyn_animator_apply_joint_velocities(instance: u32, name: &str);
	fn dyn_animator_get_playing_animation_tracks(instance: u32, name: &str);
	fn dyn_animator_load_animation(instance: u32, name: &str, p_animation: Option<Animation>) -> Option<AnimationTrack>;
	fn dyn_tool_activate(instance: u32, name: &str);
	fn dyn_tool_deactivate(instance: u32, name: &str);
	fn dyn_base_player_gui_get_gui_objects_at_position(instance: u32, name: &str, p_x: f64, p_y: f64) -> Objects;
	fn dyn_starter_gui_get_core_gui_enabled(instance: u32, name: &str) -> bool;
	fn dyn_starter_gui_set_core(instance: u32, name: &str, p_parameterName: &str);
	fn dyn_starter_gui_set_core_gui_enabled(instance: u32, name: &str, p_enabled: bool);
	fn dyn_starter_gui_get_core(instance: u32, name: &str, p_parameterName: &str);
	fn dyn_beam_set_texture_offset(instance: u32, name: &str, p_offset: f64);
	fn dyn_bindable_event_fire(instance: u32, name: &str);
	fn dyn_bindable_function_invoke(instance: u32, name: &str);
	fn dyn_body_position_get_last_force(instance: u32, name: &str) -> Vector3;
	fn dyn_body_velocity_get_last_force(instance: u32, name: &str) -> Vector3;
	fn dyn_rocket_propulsion_abort(instance: u32, name: &str);
	fn dyn_rocket_propulsion_fire(instance: u32, name: &str);
	fn dyn_camera_get_pan_speed(instance: u32, name: &str) -> f64;
	fn dyn_camera_get_parts_obscuring_target(instance: u32, name: &str, p_ignoreList: Objects) -> Objects;
	fn dyn_camera_get_render_c_frame(instance: u32, name: &str) -> CFrame;
	fn dyn_camera_get_roll(instance: u32, name: &str) -> f64;
	fn dyn_camera_get_tilt_speed(instance: u32, name: &str) -> f64;
	fn dyn_camera_screen_point_to_ray(instance: u32, name: &str, p_x: f64, p_y: f64, p_depth: f64) -> Ray;
	fn dyn_camera_set_camera_pan_mode(instance: u32, name: &str);
	fn dyn_camera_set_roll(instance: u32, name: &str, p_rollAngle: f64);
	fn dyn_camera_viewport_point_to_ray(instance: u32, name: &str, p_x: f64, p_y: f64, p_depth: f64) -> Ray;
	fn dyn_camera_world_to_screen_point(instance: u32, name: &str, p_worldPoint: Vector3);
	fn dyn_camera_world_to_viewport_point(instance: u32, name: &str, p_worldPoint: Vector3);
	fn dyn_controller_bind_button(instance: u32, name: &str, p_caption: &str);
	fn dyn_controller_get_button(instance: u32, name: &str) -> bool;
	fn dyn_controller_unbind_button(instance: u32, name: &str);
	fn dyn_data_store_increment_options_get_metadata(instance: u32, name: &str);
	fn dyn_data_store_increment_options_set_metadata(instance: u32, name: &str);
	fn dyn_data_store_key_info_get_metadata(instance: u32, name: &str);
	fn dyn_data_store_key_info_get_user_ids(instance: u32, name: &str);
	fn dyn_data_store_options_set_experimental_features(instance: u32, name: &str);
	fn dyn_data_store_set_options_get_metadata(instance: u32, name: &str);
	fn dyn_data_store_set_options_set_metadata(instance: u32, name: &str);
	fn dyn_dialog_get_current_players(instance: u32, name: &str) -> Objects;
	fn dyn_dragger_axis_rotate(instance: u32, name: &str);
	fn dyn_dragger_mouse_down(instance: u32, name: &str, p_mousePart: Option<Instance>, p_pointOnMousePart: Vector3, p_parts: Objects);
	fn dyn_dragger_mouse_move(instance: u32, name: &str, p_mouseRay: Ray);
	fn dyn_dragger_mouse_up(instance: u32, name: &str);
	fn dyn_euler_rotation_curve_get_angles_at_time(instance: u32, name: &str, p_time: f64);
	fn dyn_euler_rotation_curve_get_rotation_at_time(instance: u32, name: &str, p_time: f64) -> CFrame;
	fn dyn_euler_rotation_curve_x(instance: u32, name: &str) -> Option<FloatCurve>;
	fn dyn_euler_rotation_curve_y(instance: u32, name: &str) -> Option<FloatCurve>;
	fn dyn_euler_rotation_curve_z(instance: u32, name: &str) -> Option<FloatCurve>;
	fn dyn_float_curve_get_key_at_index(instance: u32, name: &str, p_index: f64) -> FloatCurveKey;
	fn dyn_float_curve_get_key_indices_at_time(instance: u32, name: &str, p_time: f64);
	fn dyn_float_curve_get_keys(instance: u32, name: &str);
	fn dyn_float_curve_get_value_at_time(instance: u32, name: &str, p_time: f64);
	fn dyn_float_curve_insert_key(instance: u32, name: &str, p_key: FloatCurveKey);
	fn dyn_float_curve_remove_key_at_index(instance: u32, name: &str, p_startingIndex: f64, p_count: f64) -> f64;
	fn dyn_float_curve_set_keys(instance: u32, name: &str) -> f64;
	fn dyn_global_data_store_get_async(instance: u32, name: &str, p_key: &str);
	fn dyn_global_data_store_increment_async(instance: u32, name: &str, p_key: &str, p_delta: f64, p_options: Option<DataStoreIncrementOptions>);
	fn dyn_global_data_store_remove_async(instance: u32, name: &str, p_key: &str);
	fn dyn_global_data_store_set_async(instance: u32, name: &str, p_key: &str, p_options: Option<DataStoreSetOptions>);
	fn dyn_global_data_store_update_async(instance: u32, name: &str, p_key: &str, p_transformFunction: Function);
	fn dyn_data_store_get_version_async(instance: u32, name: &str, p_key: &str, p_version: &str);
	fn dyn_data_store_list_keys_async(instance: u32, name: &str, p_prefix: &str, p_pageSize: f64) -> Option<DataStoreKeyPages>;
	fn dyn_data_store_list_versions_async(instance: u32, name: &str, p_key: &str, p_minDate: f64, p_maxDate: f64, p_pageSize: f64) -> Option<DataStoreVersionPages>;
	fn dyn_data_store_remove_version_async(instance: u32, name: &str, p_key: &str, p_version: &str);
	fn dyn_ordered_data_store_get_sorted_async(instance: u32, name: &str, p_ascending: bool, p_pagesize: f64) -> Option<Instance>;
	fn dyn_gui_object_tween_position(instance: u32, name: &str, p_endPosition: UDim2, p_time: f64, p_override: bool, p_callback: Function) -> bool;
	fn dyn_gui_object_tween_size(instance: u32, name: &str, p_endSize: UDim2, p_time: f64, p_override: bool, p_callback: Function) -> bool;
	fn dyn_gui_object_tween_size_and_position(instance: u32, name: &str, p_endSize: UDim2, p_endPosition: UDim2, p_time: f64, p_override: bool, p_callback: Function) -> bool;
	fn dyn_text_box_capture_focus(instance: u32, name: &str);
	fn dyn_text_box_is_focused(instance: u32, name: &str) -> bool;
	fn dyn_text_box_release_focus(instance: u32, name: &str, p_submitted: bool);
	fn dyn_video_frame_pause(instance: u32, name: &str);
	fn dyn_video_frame_play(instance: u32, name: &str);
	fn dyn_humanoid_add_accessory(instance: u32, name: &str, p_accessory: Option<Instance>);
	fn dyn_humanoid_build_rig_from_attachments(instance: u32, name: &str);
	fn dyn_humanoid_change_state(instance: u32, name: &str);
	fn dyn_humanoid_equip_tool(instance: u32, name: &str, p_tool: Option<Instance>);
	fn dyn_humanoid_get_accessories(instance: u32, name: &str);
	fn dyn_humanoid_get_applied_description(instance: u32, name: &str) -> Option<HumanoidDescription>;
	fn dyn_humanoid_get_body_part_r_15(instance: u32, name: &str, p_part: Option<Instance>);
	fn dyn_humanoid_get_limb(instance: u32, name: &str, p_part: Option<Instance>);
	fn dyn_humanoid_get_state(instance: u32, name: &str);
	fn dyn_humanoid_get_state_enabled(instance: u32, name: &str) -> bool;
	fn dyn_humanoid_move(instance: u32, name: &str, p_moveDirection: Vector3, p_relativeToCamera: bool);
	fn dyn_humanoid_move_to(instance: u32, name: &str, p_location: Vector3, p_part: Option<Instance>);
	fn dyn_humanoid_remove_accessories(instance: u32, name: &str);
	fn dyn_humanoid_replace_body_part_r_15(instance: u32, name: &str, p_part: Option<BasePart>) -> bool;
	fn dyn_humanoid_set_state_enabled(instance: u32, name: &str, p_enabled: bool);
	fn dyn_humanoid_take_damage(instance: u32, name: &str, p_amount: f64);
	fn dyn_humanoid_unequip_tools(instance: u32, name: &str);
	fn dyn_humanoid_apply_description(instance: u32, name: &str, p_humanoidDescription: Option<HumanoidDescription>);
	fn dyn_humanoid_apply_description_reset(instance: u32, name: &str, p_humanoidDescription: Option<HumanoidDescription>);
	fn dyn_humanoid_play_emote(instance: u32, name: &str, p_emoteName: &str) -> bool;
	fn dyn_humanoid_description_add_emote(instance: u32, name: &str, p_name: &str, p_assetId: f64);
	fn dyn_humanoid_description_get_accessories(instance: u32, name: &str, p_includeRigidAccessories: bool);
	fn dyn_humanoid_description_get_emotes(instance: u32, name: &str);
	fn dyn_humanoid_description_get_equipped_emotes(instance: u32, name: &str);
	fn dyn_humanoid_description_remove_emote(instance: u32, name: &str, p_name: &str);
	fn dyn_humanoid_description_set_accessories(instance: u32, name: &str, p_includeRigidAccessories: bool);
	fn dyn_humanoid_description_set_emotes(instance: u32, name: &str);
	fn dyn_humanoid_description_set_equipped_emotes(instance: u32, name: &str);
	fn dyn_input_object_is_modifier_key_down(instance: u32, name: &str) -> bool;
	fn dyn_motor_set_desired_angle(instance: u32, name: &str, p_value: f64);
	fn dyn_keyframe_add_marker(instance: u32, name: &str, p_marker: Option<Instance>);
	fn dyn_keyframe_add_pose(instance: u32, name: &str, p_pose: Option<Instance>);
	fn dyn_keyframe_get_markers(instance: u32, name: &str) -> Objects;
	fn dyn_keyframe_get_poses(instance: u32, name: &str) -> Objects;
	fn dyn_keyframe_remove_marker(instance: u32, name: &str, p_marker: Option<Instance>);
	fn dyn_keyframe_remove_pose(instance: u32, name: &str, p_pose: Option<Instance>);
	fn dyn_lighting_get_minutes_after_midnight(instance: u32, name: &str) -> f64;
	fn dyn_lighting_get_moon_direction(instance: u32, name: &str) -> Vector3;
	fn dyn_lighting_get_moon_phase(instance: u32, name: &str) -> f64;
	fn dyn_lighting_get_sun_direction(instance: u32, name: &str) -> Vector3;
	fn dyn_lighting_set_minutes_after_midnight(instance: u32, name: &str, p_minutes: f64);
	fn dyn_localization_table_get_entries(instance: u32, name: &str);
	fn dyn_localization_table_get_translator(instance: u32, name: &str, p_localeId: &str) -> Option<Instance>;
	fn dyn_localization_table_remove_entry(instance: u32, name: &str, p_key: &str, p_source: &str, p_context: &str);
	fn dyn_localization_table_remove_entry_value(instance: u32, name: &str, p_key: &str, p_source: &str, p_context: &str, p_localeId: &str);
	fn dyn_localization_table_remove_target_locale(instance: u32, name: &str, p_localeId: &str);
	fn dyn_localization_table_set_entries(instance: u32, name: &str);
	fn dyn_localization_table_set_entry_context(instance: u32, name: &str, p_key: &str, p_source: &str, p_context: &str, p_newContext: &str);
	fn dyn_localization_table_set_entry_example(instance: u32, name: &str, p_key: &str, p_source: &str, p_context: &str, p_example: &str);
	fn dyn_localization_table_set_entry_key(instance: u32, name: &str, p_key: &str, p_source: &str, p_context: &str, p_newKey: &str);
	fn dyn_localization_table_set_entry_source(instance: u32, name: &str, p_key: &str, p_source: &str, p_context: &str, p_newSource: &str);
	fn dyn_localization_table_set_entry_value(instance: u32, name: &str, p_key: &str, p_source: &str, p_context: &str, p_localeId: &str, p_text: &str);
	fn dyn_marker_curve_get_marker_at_index(instance: u32, name: &str, p_index: f64);
	fn dyn_marker_curve_get_markers(instance: u32, name: &str);
	fn dyn_marker_curve_insert_marker_at_time(instance: u32, name: &str, p_time: f64, p_marker: &str);
	fn dyn_marker_curve_remove_marker_at_index(instance: u32, name: &str, p_startingIndex: f64, p_count: f64) -> f64;
	fn dyn_memory_store_queue_add_async(instance: u32, name: &str, p_expiration: f64, p_priority: f64);
	fn dyn_memory_store_queue_read_async(instance: u32, name: &str, p_count: f64, p_allOrNothing: bool, p_waitTimeout: f64);
	fn dyn_memory_store_queue_remove_async(instance: u32, name: &str, p_id: &str);
	fn dyn_memory_store_sorted_map_get_async(instance: u32, name: &str, p_key: &str);
	fn dyn_memory_store_sorted_map_get_range_async(instance: u32, name: &str, p_count: f64, p_exclusiveLowerBound: &str, p_exclusiveUpperBound: &str);
	fn dyn_memory_store_sorted_map_remove_async(instance: u32, name: &str, p_key: &str);
	fn dyn_memory_store_sorted_map_set_async(instance: u32, name: &str, p_key: &str, p_expiration: f64) -> bool;
	fn dyn_memory_store_sorted_map_update_async(instance: u32, name: &str, p_key: &str, p_transformFunction: Function, p_expiration: f64);
	fn dyn_pv_instance_get_pivot(instance: u32, name: &str) -> CFrame;
	fn dyn_pv_instance_pivot_to(instance: u32, name: &str, p_targetCFrame: CFrame);
	fn dyn_base_part_apply_angular_impulse(instance: u32, name: &str, p_impulse: Vector3);
	fn dyn_base_part_apply_impulse(instance: u32, name: &str, p_impulse: Vector3);
	fn dyn_base_part_apply_impulse_at_position(instance: u32, name: &str, p_impulse: Vector3, p_position: Vector3);
	fn dyn_base_part_break_joints(instance: u32, name: &str);
	fn dyn_base_part_can_collide_with(instance: u32, name: &str, p_part: Option<BasePart>) -> bool;
	fn dyn_base_part_can_set_network_ownership(instance: u32, name: &str);
	fn dyn_base_part_get_connected_parts(instance: u32, name: &str, p_recursive: bool) -> Objects;
	fn dyn_base_part_get_joints(instance: u32, name: &str) -> Objects;
	fn dyn_base_part_get_mass(instance: u32, name: &str) -> f64;
	fn dyn_base_part_get_network_owner(instance: u32, name: &str) -> Option<Instance>;
	fn dyn_base_part_get_network_ownership_auto(instance: u32, name: &str) -> bool;
	fn dyn_base_part_get_root_part(instance: u32, name: &str) -> Option<Instance>;
	fn dyn_base_part_get_touching_parts(instance: u32, name: &str) -> Objects;
	fn dyn_base_part_get_velocity_at_position(instance: u32, name: &str, p_position: Vector3) -> Vector3;
	fn dyn_base_part_is_grounded(instance: u32, name: &str) -> bool;
	fn dyn_base_part_make_joints(instance: u32, name: &str);
	fn dyn_base_part_resize(instance: u32, name: &str, p_deltaAmount: f64) -> bool;
	fn dyn_base_part_set_network_owner(instance: u32, name: &str, p_playerInstance: Option<Player>);
	fn dyn_base_part_set_network_ownership_auto(instance: u32, name: &str);
	fn dyn_base_part_subtract_async(instance: u32, name: &str, p_parts: Objects) -> Option<Instance>;
	fn dyn_base_part_union_async(instance: u32, name: &str, p_parts: Objects) -> Option<Instance>;
	fn dyn_seat_sit(instance: u32, name: &str, p_humanoid: Option<Instance>);
	fn dyn_skateboard_platform_apply_specific_impulse(instance: u32, name: &str, p_impulseWorld: Vector3);
	fn dyn_terrain_cell_center_to_world(instance: u32, name: &str, p_x: f64, p_y: f64, p_z: f64) -> Vector3;
	fn dyn_terrain_cell_corner_to_world(instance: u32, name: &str, p_x: f64, p_y: f64, p_z: f64) -> Vector3;
	fn dyn_terrain_clear(instance: u32, name: &str);
	fn dyn_terrain_copy_region(instance: u32, name: &str, p_region: Region3int16) -> Option<TerrainRegion>;
	fn dyn_terrain_count_cells(instance: u32, name: &str) -> f64;
	fn dyn_terrain_fill_ball(instance: u32, name: &str, p_center: Vector3, p_radius: f64);
	fn dyn_terrain_fill_block(instance: u32, name: &str, p_cframe: CFrame, p_size: Vector3);
	fn dyn_terrain_fill_cylinder(instance: u32, name: &str, p_cframe: CFrame, p_height: f64, p_radius: f64);
	fn dyn_terrain_fill_region(instance: u32, name: &str, p_region: Region3, p_resolution: f64);
	fn dyn_terrain_fill_wedge(instance: u32, name: &str, p_cframe: CFrame, p_size: Vector3);
	fn dyn_terrain_get_material_color(instance: u32, name: &str) -> Color3;
	fn dyn_terrain_paste_region(instance: u32, name: &str, p_region: Option<TerrainRegion>, p_corner: Vector3int16, p_pasteEmptyCells: bool);
	fn dyn_terrain_read_voxels(instance: u32, name: &str, p_region: Region3, p_resolution: f64);
	fn dyn_terrain_replace_material(instance: u32, name: &str, p_region: Region3, p_resolution: f64);
	fn dyn_terrain_set_material_color(instance: u32, name: &str, p_value: Color3);
	fn dyn_terrain_world_to_cell(instance: u32, name: &str, p_position: Vector3) -> Vector3;
	fn dyn_terrain_world_to_cell_prefer_empty(instance: u32, name: &str, p_position: Vector3) -> Vector3;
	fn dyn_terrain_world_to_cell_prefer_solid(instance: u32, name: &str, p_position: Vector3) -> Vector3;
	fn dyn_terrain_write_voxels(instance: u32, name: &str, p_region: Region3, p_resolution: f64);
	fn dyn_mesh_part_apply_mesh(instance: u32, name: &str, p_meshPart: Option<Instance>);
	fn dyn_vehicle_seat_sit(instance: u32, name: &str, p_humanoid: Option<Instance>);
	fn dyn_model_break_joints(instance: u32, name: &str);
	fn dyn_model_get_bounding_box(instance: u32, name: &str);
	fn dyn_model_get_extents_size(instance: u32, name: &str) -> Vector3;
	fn dyn_model_get_primary_part_c_frame(instance: u32, name: &str) -> CFrame;
	fn dyn_model_make_joints(instance: u32, name: &str);
	fn dyn_model_move_to(instance: u32, name: &str, p_position: Vector3);
	fn dyn_model_set_primary_part_c_frame(instance: u32, name: &str, p_cframe: CFrame);
	fn dyn_model_translate_by(instance: u32, name: &str, p_delta: Vector3);
	fn dyn_world_root_are_parts_touching_others(instance: u32, name: &str, p_partList: Objects, p_overlapIgnored: f64) -> bool;
	fn dyn_world_root_bulk_move_to(instance: u32, name: &str, p_partList: Objects);
	fn dyn_world_root_get_part_bounds_in_box(instance: u32, name: &str, p_cframe: CFrame, p_size: Vector3, p_overlapParams: OverlapParams) -> Objects;
	fn dyn_world_root_get_part_bounds_in_radius(instance: u32, name: &str, p_position: Vector3, p_radius: f64, p_overlapParams: OverlapParams) -> Objects;
	fn dyn_world_root_get_parts_in_part(instance: u32, name: &str, p_part: Option<BasePart>, p_overlapParams: OverlapParams) -> Objects;
	fn dyn_world_root_raycast(instance: u32, name: &str, p_origin: Vector3, p_direction: Vector3, p_raycastParams: RaycastParams) -> RaycastResult;
	fn dyn_workspace_get_num_awake_parts(instance: u32, name: &str) -> f64;
	fn dyn_workspace_get_physics_throttling(instance: u32, name: &str) -> f64;
	fn dyn_workspace_get_real_physics_fps(instance: u32, name: &str) -> f64;
	fn dyn_workspace_get_server_time_now(instance: u32, name: &str) -> f64;
	fn dyn_workspace_join_to_outsiders(instance: u32, name: &str, p_objects: Objects);
	fn dyn_workspace_pgs_is_enabled(instance: u32, name: &str) -> bool;
	fn dyn_workspace_unjoin_from_outsiders(instance: u32, name: &str, p_objects: Objects);
	fn dyn_pages_get_current_page(instance: u32, name: &str);
	fn dyn_pages_advance_to_next_page_async(instance: u32, name: &str);
	fn dyn_particle_emitter_clear(instance: u32, name: &str);
	fn dyn_particle_emitter_emit(instance: u32, name: &str, p_particleCount: f64);
	fn dyn_path_get_waypoints(instance: u32, name: &str);
	fn dyn_path_check_occlusion_async(instance: u32, name: &str, p_start: f64) -> f64;
	fn dyn_path_compute_async(instance: u32, name: &str, p_start: Vector3, p_finish: Vector3);
	fn dyn_player_clear_character_appearance(instance: u32, name: &str);
	fn dyn_player_distance_from_character(instance: u32, name: &str, p_point: Vector3) -> f64;
	fn dyn_player_get_join_data(instance: u32, name: &str);
	fn dyn_player_get_mouse(instance: u32, name: &str) -> Option<Mouse>;
	fn dyn_player_get_network_ping(instance: u32, name: &str) -> f64;
	fn dyn_player_has_appearance_loaded(instance: u32, name: &str) -> bool;
	fn dyn_player_kick(instance: u32, name: &str, p_message: &str);
	fn dyn_player_move(instance: u32, name: &str, p_walkDirection: Vector3, p_relativeToCamera: bool);
	fn dyn_player_get_friends_online(instance: u32, name: &str, p_maxFriends: f64);
	fn dyn_player_get_rank_in_group(instance: u32, name: &str, p_groupId: f64) -> f64;
	fn dyn_player_get_role_in_group(instance: u32, name: &str, p_groupId: f64) -> String;
	fn dyn_player_is_friends_with(instance: u32, name: &str, p_userId: f64) -> bool;
	fn dyn_player_is_in_group(instance: u32, name: &str, p_groupId: f64) -> bool;
	fn dyn_player_load_character(instance: u32, name: &str);
	fn dyn_player_load_character_with_humanoid_description(instance: u32, name: &str, p_humanoidDescription: Option<HumanoidDescription>);
	fn dyn_player_request_stream_around_async(instance: u32, name: &str, p_position: Vector3, p_timeOut: f64);
	fn dyn_player_scripts_clear_computer_camera_movement_modes(instance: u32, name: &str);
	fn dyn_player_scripts_clear_computer_movement_modes(instance: u32, name: &str);
	fn dyn_player_scripts_clear_touch_camera_movement_modes(instance: u32, name: &str);
	fn dyn_player_scripts_clear_touch_movement_modes(instance: u32, name: &str);
	fn dyn_player_scripts_register_computer_camera_movement_mode(instance: u32, name: &str);
	fn dyn_player_scripts_register_computer_movement_mode(instance: u32, name: &str);
	fn dyn_player_scripts_register_touch_camera_movement_mode(instance: u32, name: &str);
	fn dyn_player_scripts_register_touch_movement_mode(instance: u32, name: &str);
	fn dyn_pose_add_sub_pose(instance: u32, name: &str, p_pose: Option<Instance>);
	fn dyn_pose_get_sub_poses(instance: u32, name: &str) -> Objects;
	fn dyn_pose_remove_sub_pose(instance: u32, name: &str, p_pose: Option<Instance>);
	fn dyn_proximity_prompt_input_hold_begin(instance: u32, name: &str);
	fn dyn_proximity_prompt_input_hold_end(instance: u32, name: &str);
	fn dyn_remote_event_fire_all_clients(instance: u32, name: &str);
	fn dyn_remote_event_fire_client(instance: u32, name: &str, p_player: Option<Player>);
	fn dyn_remote_event_fire_server(instance: u32, name: &str);
	fn dyn_remote_function_invoke_client(instance: u32, name: &str, p_player: Option<Player>);
	fn dyn_remote_function_invoke_server(instance: u32, name: &str);
	fn dyn_replicated_first_remove_default_loading_screen(instance: u32, name: &str);
	fn dyn_rotation_curve_get_key_at_index(instance: u32, name: &str, p_index: f64) -> RotationCurveKey;
	fn dyn_rotation_curve_get_key_indices_at_time(instance: u32, name: &str, p_time: f64);
	fn dyn_rotation_curve_get_keys(instance: u32, name: &str);
	fn dyn_rotation_curve_get_value_at_time(instance: u32, name: &str, p_time: f64) -> Option<CFrame>;
	fn dyn_rotation_curve_insert_key(instance: u32, name: &str, p_key: RotationCurveKey);
	fn dyn_rotation_curve_remove_key_at_index(instance: u32, name: &str, p_startingIndex: f64, p_count: f64) -> f64;
	fn dyn_rotation_curve_set_keys(instance: u32, name: &str) -> f64;
	fn dyn_service_provider_find_service(instance: u32, name: &str, p_className: &str) -> Option<Instance>;
	fn dyn_service_provider_get_service(instance: u32, name: &str, p_className: &str) -> Option<Instance>;
	fn dyn_data_model_bind_to_close(instance: u32, name: &str, p_function: Function);
	fn dyn_data_model_is_loaded(instance: u32, name: &str) -> bool;
	fn dyn_user_settings_is_user_feature_enabled(instance: u32, name: &str, p_name: &str) -> bool;
	fn dyn_user_settings_reset(instance: u32, name: &str);
	fn dyn_sound_pause(instance: u32, name: &str);
	fn dyn_sound_play(instance: u32, name: &str);
	fn dyn_sound_resume(instance: u32, name: &str);
	fn dyn_sound_stop(instance: u32, name: &str);
	fn dyn_team_get_players(instance: u32, name: &str) -> Objects;
	fn dyn_teams_get_teams(instance: u32, name: &str) -> Objects;
	fn dyn_teleport_options_get_teleport_data(instance: u32, name: &str);
	fn dyn_teleport_options_set_teleport_data(instance: u32, name: &str);
	fn dyn_text_channel_display_system_message(instance: u32, name: &str, p_systemMessage: &str, p_metadata: &str) -> Option<TextChatMessage>;
	fn dyn_text_channel_add_user_async(instance: u32, name: &str, p_userId: f64);
	fn dyn_text_channel_send_async(instance: u32, name: &str, p_message: &str, p_metadata: &str) -> Option<TextChatMessage>;
	fn dyn_text_filter_result_get_chat_for_user_async(instance: u32, name: &str, p_toUserId: f64) -> String;
	fn dyn_text_filter_result_get_non_chat_string_for_broadcast_async(instance: u32, name: &str) -> String;
	fn dyn_text_filter_result_get_non_chat_string_for_user_async(instance: u32, name: &str, p_toUserId: f64) -> String;
	fn dyn_trail_clear(instance: u32, name: &str);
	fn dyn_translator_format_by_key(instance: u32, name: &str, p_key: &str) -> String;
	fn dyn_translator_translate(instance: u32, name: &str, p_context: Option<Instance>, p_text: &str) -> String;
	fn dyn_tween_base_cancel(instance: u32, name: &str);
	fn dyn_tween_base_pause(instance: u32, name: &str);
	fn dyn_tween_base_play(instance: u32, name: &str);
	fn dyn_ui_page_layout_jump_to(instance: u32, name: &str, p_page: Option<Instance>);
	fn dyn_ui_page_layout_jump_to_index(instance: u32, name: &str, p_index: f64);
	fn dyn_ui_page_layout_next(instance: u32, name: &str);
	fn dyn_ui_page_layout_previous(instance: u32, name: &str);
	fn dyn_user_game_settings_get_camera_y_invert_value(instance: u32, name: &str) -> f64;
	fn dyn_user_game_settings_get_onboarding_completed(instance: u32, name: &str, p_onboardingId: &str) -> bool;
	fn dyn_user_game_settings_in_full_screen(instance: u32, name: &str) -> bool;
	fn dyn_user_game_settings_in_studio_mode(instance: u32, name: &str) -> bool;
	fn dyn_user_game_settings_set_camera_y_invert_visible(instance: u32, name: &str);
	fn dyn_user_game_settings_set_gamepad_camera_sensitivity_visible(instance: u32, name: &str);
	fn dyn_user_game_settings_set_onboarding_completed(instance: u32, name: &str, p_onboardingId: &str);
	fn dyn_vector_3_curve_get_value_at_time(instance: u32, name: &str, p_time: f64);
	fn dyn_vector_3_curve_x(instance: u32, name: &str) -> Option<FloatCurve>;
	fn dyn_vector_3_curve_y(instance: u32, name: &str) -> Option<FloatCurve>;
	fn dyn_vector_3_curve_z(instance: u32, name: &str) -> Option<FloatCurve>;
}
#[allow(unused_imports)]
use super::*;
pub trait RobloxCreatable {
	fn instance_new() -> Self;
}
macro_rules! creatable {
	($name:ident) => {
		impl RobloxCreatable for $name {
			fn instance_new() -> $name {
				unsafe { Self(instance_new(stringify!($name))) }
			}
		}
	}
}
macro_rules! impl_instance_exclusive {
	($name:ident) => {
		impl std::convert::TryFrom<Instance> for $name {
			type Error = ();
			fn try_from(value: Instance) -> Result<Self, Self::Error> {
				unsafe {
					if instance_is_a(value.to_ptr(), stringify!($name)) {
						Ok($name(value.to_ptr()))
					} else {
						Err(())
					}
				}
			}
		}
	}
}
macro_rules! impl_instance {
	($name:ident) => {
		#[allow(dead_code)]
		#[repr(C)]
		pub struct $name(u32);
		impl $name {
			pub fn new<I: RobloxCreatable>() -> I {
				I::instance_new()
			}
			pub(crate) fn to_ptr(&self) -> u32 { self.0 }
			pub fn get_child(&self, name: &str) -> Instance {
				match unsafe { get_child(self.0, name) } {
					0 => panic!("No child by the name of '{}' was found", name),
					id => Instance(id),
				}
			}
		}
		impl Clone for $name {
			fn clone(&self) -> Self {
				unsafe { Self(clone_pointer(self.to_ptr())) }
			}
		}
		impl Copy for $name {}
		impl $name {
			pub fn archivable(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Archivable") }
			}
			pub fn set_archivable(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Archivable", value) }
			}
			pub fn class_name(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "ClassName") }
			}
			pub fn name(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Name") }
			}
			pub fn set_name(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "Name", &value) }
			}
			pub fn parent(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "Parent").map(|id| Instance(id)) }
			}
			pub fn set_parent(&self, value: Option<Instance>) {
				unsafe { set_instance_property(self.to_ptr(), "Parent", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn fn_clear_all_children(&self) {
				unsafe { dyn_instance_clear_all_children(self.to_ptr(), "ClearAllChildren") }
			}
			pub fn fn_clone(&self) -> Option<Instance> {
				unsafe { dyn_instance_clone(self.to_ptr(), "Clone") }
			}
			pub fn fn_destroy(&self) {
				unsafe { dyn_instance_destroy(self.to_ptr(), "Destroy") }
			}
			pub fn fn_find_first_ancestor(&self, name: &str) -> Option<Instance> {
				unsafe { dyn_instance_find_first_ancestor(self.to_ptr(), "FindFirstAncestor", name) }
			}
			pub fn fn_find_first_ancestor_of_class(&self, class_name: &str) -> Option<Instance> {
				unsafe { dyn_instance_find_first_ancestor_of_class(self.to_ptr(), "FindFirstAncestorOfClass", class_name) }
			}
			pub fn fn_find_first_ancestor_which_is_a(&self, class_name: &str) -> Option<Instance> {
				unsafe { dyn_instance_find_first_ancestor_which_is_a(self.to_ptr(), "FindFirstAncestorWhichIsA", class_name) }
			}
			pub fn fn_find_first_child(&self, name: &str, recursive: bool) -> Option<Instance> {
				unsafe { dyn_instance_find_first_child(self.to_ptr(), "FindFirstChild", name, recursive) }
			}
			pub fn fn_find_first_child_of_class(&self, class_name: &str) -> Option<Instance> {
				unsafe { dyn_instance_find_first_child_of_class(self.to_ptr(), "FindFirstChildOfClass", class_name) }
			}
			pub fn fn_find_first_child_which_is_a(&self, class_name: &str, recursive: bool) -> Option<Instance> {
				unsafe { dyn_instance_find_first_child_which_is_a(self.to_ptr(), "FindFirstChildWhichIsA", class_name, recursive) }
			}
			pub fn fn_find_first_descendant(&self, name: &str) -> Option<Instance> {
				unsafe { dyn_instance_find_first_descendant(self.to_ptr(), "FindFirstDescendant", name) }
			}
			pub fn fn_get_actor(&self) -> Option<Actor> {
				unsafe { dyn_instance_get_actor(self.to_ptr(), "GetActor") }
			}
			pub fn fn_get_attribute(&self, attribute: &str) {
				unsafe { dyn_instance_get_attribute(self.to_ptr(), "GetAttribute", attribute) }
			}
			pub fn fn_get_attribute_changed_signal(&self, attribute: &str) -> RBXScriptSignal {
				unsafe { dyn_instance_get_attribute_changed_signal(self.to_ptr(), "GetAttributeChangedSignal", attribute) }
			}
			pub fn fn_get_attributes(&self) {
				unsafe { dyn_instance_get_attributes(self.to_ptr(), "GetAttributes") }
			}
			pub fn fn_get_children(&self) -> Objects {
				unsafe { dyn_instance_get_children(self.to_ptr(), "GetChildren") }
			}
			pub fn fn_get_descendants(&self) {
				unsafe { dyn_instance_get_descendants(self.to_ptr(), "GetDescendants") }
			}
			pub fn fn_get_full_name(&self) -> String {
				unsafe { dyn_instance_get_full_name(self.to_ptr(), "GetFullName") }
			}
			pub fn fn_get_property_changed_signal(&self, property: &str) -> RBXScriptSignal {
				unsafe { dyn_instance_get_property_changed_signal(self.to_ptr(), "GetPropertyChangedSignal", property) }
			}
			pub fn fn_is_a(&self, class_name: &str) -> bool {
				unsafe { dyn_instance_is_a(self.to_ptr(), "IsA", class_name) }
			}
			pub fn fn_is_ancestor_of(&self, descendant: Option<Instance>) -> bool {
				unsafe { dyn_instance_is_ancestor_of(self.to_ptr(), "IsAncestorOf", descendant) }
			}
			pub fn fn_is_descendant_of(&self, ancestor: Option<Instance>) -> bool {
				unsafe { dyn_instance_is_descendant_of(self.to_ptr(), "IsDescendantOf", ancestor) }
			}
			pub fn fn_wait_for_child(&self, child_name: &str, time_out: f64) -> Option<Instance> {
				unsafe { dyn_instance_wait_for_child(self.to_ptr(), "WaitForChild", child_name, time_out) }
			}
		}
	}
}
macro_rules! impl_accoutrement {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn attachment_forward(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "AttachmentForward")) }
			}
			pub fn set_attachment_forward(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "AttachmentForward", value.to_ptr()) }
			}
			pub fn attachment_point(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "AttachmentPoint")) }
			}
			pub fn set_attachment_point(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "AttachmentPoint", value.to_ptr()) }
			}
			pub fn attachment_pos(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "AttachmentPos")) }
			}
			pub fn set_attachment_pos(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "AttachmentPos", value.to_ptr()) }
			}
			pub fn attachment_right(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "AttachmentRight")) }
			}
			pub fn set_attachment_right(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "AttachmentRight", value.to_ptr()) }
			}
			pub fn attachment_up(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "AttachmentUp")) }
			}
			pub fn set_attachment_up(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "AttachmentUp", value.to_ptr()) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_accessory {
	($name:ident) => {
		impl_accoutrement!($name);
		impl $name {
		}
		impl From<$name> for Accoutrement {
			fn from(value: $name) -> Accoutrement {
				Accoutrement(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_hat {
	($name:ident) => {
		impl_accoutrement!($name);
		impl $name {
		}
		impl From<$name> for Accoutrement {
			fn from(value: $name) -> Accoutrement {
				Accoutrement(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn animation_id(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "AnimationId")) }
			}
			pub fn set_animation_id(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "AnimationId", value.to_ptr()) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation_clip {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn r#loop(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Loop") }
			}
			pub fn set_loop(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Loop", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_curve_animation {
	($name:ident) => {
		impl_animation_clip!($name);
		impl $name {
		}
		impl From<$name> for AnimationClip {
			fn from(value: $name) -> AnimationClip {
				AnimationClip(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_keyframe_sequence {
	($name:ident) => {
		impl_animation_clip!($name);
		impl $name {
			pub fn fn_add_keyframe(&self, keyframe: Option<Instance>) {
				unsafe { dyn_keyframe_sequence_add_keyframe(self.to_ptr(), "AddKeyframe", keyframe) }
			}
			pub fn fn_get_keyframes(&self) -> Objects {
				unsafe { dyn_keyframe_sequence_get_keyframes(self.to_ptr(), "GetKeyframes") }
			}
			pub fn fn_remove_keyframe(&self, keyframe: Option<Instance>) {
				unsafe { dyn_keyframe_sequence_remove_keyframe(self.to_ptr(), "RemoveKeyframe", keyframe) }
			}
		}
		impl From<$name> for AnimationClip {
			fn from(value: $name) -> AnimationClip {
				AnimationClip(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation_controller {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation_rig_data {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation_stream_track {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn is_playing(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsPlaying") }
			}
			pub fn weight_current(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WeightCurrent") }
			}
			pub fn weight_target(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WeightTarget") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation_track {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn animation(&self) -> Option<Animation> {
				unsafe { get_instance_property(self.to_ptr(), "Animation").map(|id| Animation(id)) }
			}
			pub fn is_playing(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsPlaying") }
			}
			pub fn length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Length") }
			}
			pub fn looped(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Looped") }
			}
			pub fn set_looped(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Looped", value) }
			}
			pub fn speed(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Speed") }
			}
			pub fn time_position(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TimePosition") }
			}
			pub fn set_time_position(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TimePosition", value) }
			}
			pub fn weight_current(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WeightCurrent") }
			}
			pub fn weight_target(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WeightTarget") }
			}
			pub fn fn_adjust_speed(&self, speed: f64) {
				unsafe { dyn_animation_track_adjust_speed(self.to_ptr(), "AdjustSpeed", speed) }
			}
			pub fn fn_adjust_weight(&self, weight: f64, fade_time: f64) {
				unsafe { dyn_animation_track_adjust_weight(self.to_ptr(), "AdjustWeight", weight, fade_time) }
			}
			pub fn fn_get_marker_reached_signal(&self, name: &str) -> RBXScriptSignal {
				unsafe { dyn_animation_track_get_marker_reached_signal(self.to_ptr(), "GetMarkerReachedSignal", name) }
			}
			pub fn fn_get_time_of_keyframe(&self, keyframe_name: &str) -> f64 {
				unsafe { dyn_animation_track_get_time_of_keyframe(self.to_ptr(), "GetTimeOfKeyframe", keyframe_name) }
			}
			pub fn fn_play(&self, fade_time: f64, weight: f64, speed: f64) {
				unsafe { dyn_animation_track_play(self.to_ptr(), "Play", fade_time, weight, speed) }
			}
			pub fn fn_stop(&self, fade_time: f64) {
				unsafe { dyn_animation_track_stop(self.to_ptr(), "Stop", fade_time) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animator {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_get_playing_animation_tracks(&self) {
				unsafe { dyn_animator_get_playing_animation_tracks(self.to_ptr(), "GetPlayingAnimationTracks") }
			}
			pub fn fn_load_animation(&self, animation: Option<Animation>) -> Option<AnimationTrack> {
				unsafe { dyn_animator_load_animation(self.to_ptr(), "LoadAnimation", animation) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_atmosphere {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Color")) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn decay(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Decay")) }
			}
			pub fn set_decay(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Decay", value.to_ptr()) }
			}
			pub fn density(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Density") }
			}
			pub fn set_density(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Density", value) }
			}
			pub fn glare(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Glare") }
			}
			pub fn set_glare(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Glare", value) }
			}
			pub fn haze(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Haze") }
			}
			pub fn set_haze(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Haze", value) }
			}
			pub fn offset(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Offset") }
			}
			pub fn set_offset(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Offset", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_attachment {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn axis(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Axis")) }
			}
			pub fn set_axis(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Axis", value.to_ptr()) }
			}
			pub fn c_frame(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "CFrame")) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "CFrame", value.to_ptr()) }
			}
			pub fn orientation(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Orientation")) }
			}
			pub fn set_orientation(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Orientation", value.to_ptr()) }
			}
			pub fn position(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Position")) }
			}
			pub fn set_position(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Position", value.to_ptr()) }
			}
			#[deprecated]
			pub fn rotation(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Rotation")) }
			}
			#[deprecated]
			pub fn set_rotation(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Rotation", value.to_ptr()) }
			}
			pub fn secondary_axis(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "SecondaryAxis")) }
			}
			pub fn set_secondary_axis(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "SecondaryAxis", value.to_ptr()) }
			}
			pub fn visible(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Visible") }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Visible", value) }
			}
			pub fn world_axis(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "WorldAxis")) }
			}
			pub fn set_world_axis(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "WorldAxis", value.to_ptr()) }
			}
			pub fn world_c_frame(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "WorldCFrame")) }
			}
			pub fn set_world_c_frame(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "WorldCFrame", value.to_ptr()) }
			}
			pub fn world_orientation(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "WorldOrientation")) }
			}
			pub fn set_world_orientation(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "WorldOrientation", value.to_ptr()) }
			}
			pub fn world_position(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "WorldPosition")) }
			}
			pub fn set_world_position(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "WorldPosition", value.to_ptr()) }
			}
			#[deprecated]
			pub fn world_rotation(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "WorldRotation")) }
			}
			pub fn world_secondary_axis(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "WorldSecondaryAxis")) }
			}
			pub fn set_world_secondary_axis(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "WorldSecondaryAxis", value.to_ptr()) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_bone {
	($name:ident) => {
		impl_attachment!($name);
		impl $name {
			pub fn transform(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "Transform")) }
			}
			pub fn set_transform(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "Transform", value.to_ptr()) }
			}
			pub fn transformed_c_frame(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "TransformedCFrame")) }
			}
			pub fn transformed_world_c_frame(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "TransformedWorldCFrame")) }
			}
		}
		impl From<$name> for Attachment {
			fn from(value: $name) -> Attachment {
				Attachment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_backpack {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_backpack_item {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn texture_id(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "TextureId")) }
			}
			pub fn set_texture_id(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "TextureId", value.to_ptr()) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_tool {
	($name:ident) => {
		impl_backpack_item!($name);
		impl $name {
			pub fn can_be_dropped(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CanBeDropped") }
			}
			pub fn set_can_be_dropped(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "CanBeDropped", value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn grip(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "Grip")) }
			}
			pub fn set_grip(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "Grip", value.to_ptr()) }
			}
			pub fn grip_forward(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "GripForward")) }
			}
			pub fn set_grip_forward(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "GripForward", value.to_ptr()) }
			}
			pub fn grip_pos(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "GripPos")) }
			}
			pub fn set_grip_pos(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "GripPos", value.to_ptr()) }
			}
			pub fn grip_right(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "GripRight")) }
			}
			pub fn set_grip_right(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "GripRight", value.to_ptr()) }
			}
			pub fn grip_up(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "GripUp")) }
			}
			pub fn set_grip_up(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "GripUp", value.to_ptr()) }
			}
			pub fn manual_activation_only(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ManualActivationOnly") }
			}
			pub fn set_manual_activation_only(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ManualActivationOnly", value) }
			}
			pub fn requires_handle(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "RequiresHandle") }
			}
			pub fn set_requires_handle(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "RequiresHandle", value) }
			}
			pub fn tool_tip(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "ToolTip") }
			}
			pub fn set_tool_tip(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "ToolTip", &value) }
			}
			pub fn fn_activate(&self) {
				unsafe { dyn_tool_activate(self.to_ptr(), "Activate") }
			}
			pub fn fn_deactivate(&self) {
				unsafe { dyn_tool_deactivate(self.to_ptr(), "Deactivate") }
			}
		}
		impl From<$name> for BackpackItem {
			fn from(value: $name) -> BackpackItem {
				BackpackItem(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_base_player_gui {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_get_gui_objects_at_position(&self, x: f64, y: f64) -> Objects {
				unsafe { dyn_base_player_gui_get_gui_objects_at_position(self.to_ptr(), "GetGuiObjectsAtPosition", x, y) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_player_gui {
	($name:ident) => {
		impl_base_player_gui!($name);
		impl $name {
			pub fn selection_image_object(&self) -> Option<GuiObject> {
				unsafe { get_instance_property(self.to_ptr(), "SelectionImageObject").map(|id| GuiObject(id)) }
			}
			pub fn set_selection_image_object(&self, value: Option<GuiObject>) {
				unsafe { set_instance_property(self.to_ptr(), "SelectionImageObject", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
		}
		impl From<$name> for BasePlayerGui {
			fn from(value: $name) -> BasePlayerGui {
				BasePlayerGui(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_starter_gui {
	($name:ident) => {
		impl_base_player_gui!($name);
		impl $name {
			#[deprecated]
			pub fn reset_player_gui_on_spawn(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ResetPlayerGuiOnSpawn") }
			}
			#[deprecated]
			pub fn set_reset_player_gui_on_spawn(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ResetPlayerGuiOnSpawn", value) }
			}
			pub fn show_development_gui(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ShowDevelopmentGui") }
			}
			pub fn set_show_development_gui(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ShowDevelopmentGui", value) }
			}
			pub fn fn_get_core(&self, parameter_name: &str) {
				unsafe { dyn_starter_gui_get_core(self.to_ptr(), "GetCore", parameter_name) }
			}
		}
		impl From<$name> for BasePlayerGui {
			fn from(value: $name) -> BasePlayerGui {
				BasePlayerGui(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_base_wrap {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn cage_mesh_id(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "CageMeshId")) }
			}
			pub fn cage_origin(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "CageOrigin")) }
			}
			pub fn cage_origin_world(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "CageOriginWorld")) }
			}
			pub fn import_origin(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "ImportOrigin")) }
			}
			pub fn import_origin_world(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "ImportOriginWorld")) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_wrap_layer {
	($name:ident) => {
		impl_base_wrap!($name);
		impl $name {
			pub fn bind_offset(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "BindOffset")) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn order(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Order") }
			}
			pub fn set_order(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Order", value) }
			}
			pub fn puffiness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Puffiness") }
			}
			pub fn set_puffiness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Puffiness", value) }
			}
			pub fn reference_mesh_id(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "ReferenceMeshId")) }
			}
			pub fn reference_origin(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "ReferenceOrigin")) }
			}
			pub fn reference_origin_world(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "ReferenceOriginWorld")) }
			}
			pub fn shrink_factor(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ShrinkFactor") }
			}
		}
		impl From<$name> for BaseWrap {
			fn from(value: $name) -> BaseWrap {
				BaseWrap(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_wrap_target {
	($name:ident) => {
		impl_base_wrap!($name);
		impl $name {
			pub fn stiffness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Stiffness") }
			}
		}
		impl From<$name> for BaseWrap {
			fn from(value: $name) -> BaseWrap {
				BaseWrap(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_beam {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn attachment_0(&self) -> Option<Attachment> {
				unsafe { get_instance_property(self.to_ptr(), "Attachment0").map(|id| Attachment(id)) }
			}
			pub fn set_attachment_0(&self, value: Option<Attachment>) {
				unsafe { set_instance_property(self.to_ptr(), "Attachment0", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn attachment_1(&self) -> Option<Attachment> {
				unsafe { get_instance_property(self.to_ptr(), "Attachment1").map(|id| Attachment(id)) }
			}
			pub fn set_attachment_1(&self, value: Option<Attachment>) {
				unsafe { set_instance_property(self.to_ptr(), "Attachment1", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn brightness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Brightness") }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Brightness", value) }
			}
			pub fn color(&self) -> ColorSequence {
				unsafe { ColorSequence(get_datatype_property(self.to_ptr(), "Color")) }
			}
			pub fn set_color(&self, value: ColorSequence) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn curve_size_0(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CurveSize0") }
			}
			pub fn set_curve_size_0(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CurveSize0", value) }
			}
			pub fn curve_size_1(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CurveSize1") }
			}
			pub fn set_curve_size_1(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CurveSize1", value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn face_camera(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "FaceCamera") }
			}
			pub fn set_face_camera(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "FaceCamera", value) }
			}
			pub fn light_emission(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LightEmission") }
			}
			pub fn set_light_emission(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LightEmission", value) }
			}
			pub fn light_influence(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LightInfluence") }
			}
			pub fn set_light_influence(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LightInfluence", value) }
			}
			pub fn segments(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Segments") }
			}
			pub fn set_segments(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Segments", value) }
			}
			pub fn texture(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "Texture")) }
			}
			pub fn set_texture(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "Texture", value.to_ptr()) }
			}
			pub fn texture_length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TextureLength") }
			}
			pub fn set_texture_length(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TextureLength", value) }
			}
			pub fn texture_speed(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TextureSpeed") }
			}
			pub fn set_texture_speed(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TextureSpeed", value) }
			}
			pub fn transparency(&self) -> NumberSequence {
				unsafe { NumberSequence(get_datatype_property(self.to_ptr(), "Transparency")) }
			}
			pub fn set_transparency(&self, value: NumberSequence) {
				unsafe { set_datatype_property(self.to_ptr(), "Transparency", value.to_ptr()) }
			}
			pub fn width_0(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Width0") }
			}
			pub fn set_width_0(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Width0", value) }
			}
			pub fn width_1(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Width1") }
			}
			pub fn set_width_1(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Width1", value) }
			}
			pub fn z_offset(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ZOffset") }
			}
			pub fn set_z_offset(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ZOffset", value) }
			}
			pub fn fn_set_texture_offset(&self, offset: f64) {
				unsafe { dyn_beam_set_texture_offset(self.to_ptr(), "SetTextureOffset", offset) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_bindable_event {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_bindable_function {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_body_mover {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_body_angular_velocity {
	($name:ident) => {
		impl_body_mover!($name);
		impl $name {
			pub fn angular_velocity(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "AngularVelocity")) }
			}
			pub fn set_angular_velocity(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "AngularVelocity", value.to_ptr()) }
			}
			pub fn max_torque(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "MaxTorque")) }
			}
			pub fn set_max_torque(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "MaxTorque", value.to_ptr()) }
			}
			pub fn p(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "P") }
			}
			pub fn set_p(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "P", value) }
			}
		}
		impl From<$name> for BodyMover {
			fn from(value: $name) -> BodyMover {
				BodyMover(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_body_force {
	($name:ident) => {
		impl_body_mover!($name);
		impl $name {
			pub fn force(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Force")) }
			}
			pub fn set_force(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Force", value.to_ptr()) }
			}
		}
		impl From<$name> for BodyMover {
			fn from(value: $name) -> BodyMover {
				BodyMover(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_body_gyro {
	($name:ident) => {
		impl_body_mover!($name);
		impl $name {
			pub fn c_frame(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "CFrame")) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "CFrame", value.to_ptr()) }
			}
			pub fn d(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "D") }
			}
			pub fn set_d(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "D", value) }
			}
			pub fn max_torque(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "MaxTorque")) }
			}
			pub fn set_max_torque(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "MaxTorque", value.to_ptr()) }
			}
			pub fn p(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "P") }
			}
			pub fn set_p(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "P", value) }
			}
		}
		impl From<$name> for BodyMover {
			fn from(value: $name) -> BodyMover {
				BodyMover(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_body_position {
	($name:ident) => {
		impl_body_mover!($name);
		impl $name {
			pub fn d(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "D") }
			}
			pub fn set_d(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "D", value) }
			}
			pub fn max_force(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "MaxForce")) }
			}
			pub fn set_max_force(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "MaxForce", value.to_ptr()) }
			}
			pub fn p(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "P") }
			}
			pub fn set_p(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "P", value) }
			}
			pub fn position(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Position")) }
			}
			pub fn set_position(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Position", value.to_ptr()) }
			}
			pub fn fn_get_last_force(&self) -> Vector3 {
				unsafe { dyn_body_position_get_last_force(self.to_ptr(), "GetLastForce") }
			}
		}
		impl From<$name> for BodyMover {
			fn from(value: $name) -> BodyMover {
				BodyMover(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_body_thrust {
	($name:ident) => {
		impl_body_mover!($name);
		impl $name {
			pub fn force(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Force")) }
			}
			pub fn set_force(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Force", value.to_ptr()) }
			}
			pub fn location(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Location")) }
			}
			pub fn set_location(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Location", value.to_ptr()) }
			}
		}
		impl From<$name> for BodyMover {
			fn from(value: $name) -> BodyMover {
				BodyMover(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_body_velocity {
	($name:ident) => {
		impl_body_mover!($name);
		impl $name {
			pub fn max_force(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "MaxForce")) }
			}
			pub fn set_max_force(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "MaxForce", value.to_ptr()) }
			}
			pub fn p(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "P") }
			}
			pub fn set_p(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "P", value) }
			}
			pub fn velocity(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Velocity")) }
			}
			pub fn set_velocity(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Velocity", value.to_ptr()) }
			}
			pub fn fn_get_last_force(&self) -> Vector3 {
				unsafe { dyn_body_velocity_get_last_force(self.to_ptr(), "GetLastForce") }
			}
		}
		impl From<$name> for BodyMover {
			fn from(value: $name) -> BodyMover {
				BodyMover(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_rocket_propulsion {
	($name:ident) => {
		impl_body_mover!($name);
		impl $name {
			pub fn cartoon_factor(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CartoonFactor") }
			}
			pub fn set_cartoon_factor(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CartoonFactor", value) }
			}
			pub fn max_speed(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxSpeed") }
			}
			pub fn set_max_speed(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxSpeed", value) }
			}
			pub fn max_thrust(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxThrust") }
			}
			pub fn set_max_thrust(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxThrust", value) }
			}
			pub fn max_torque(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "MaxTorque")) }
			}
			pub fn set_max_torque(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "MaxTorque", value.to_ptr()) }
			}
			pub fn target(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "Target").map(|id| BasePart(id)) }
			}
			pub fn set_target(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "Target", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn target_offset(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "TargetOffset")) }
			}
			pub fn set_target_offset(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "TargetOffset", value.to_ptr()) }
			}
			pub fn target_radius(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TargetRadius") }
			}
			pub fn set_target_radius(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TargetRadius", value) }
			}
			pub fn thrust_d(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ThrustD") }
			}
			pub fn set_thrust_d(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ThrustD", value) }
			}
			pub fn thrust_p(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ThrustP") }
			}
			pub fn set_thrust_p(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ThrustP", value) }
			}
			pub fn turn_d(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TurnD") }
			}
			pub fn set_turn_d(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TurnD", value) }
			}
			pub fn turn_p(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TurnP") }
			}
			pub fn set_turn_p(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TurnP", value) }
			}
			pub fn fn_abort(&self) {
				unsafe { dyn_rocket_propulsion_abort(self.to_ptr(), "Abort") }
			}
			pub fn fn_fire(&self) {
				unsafe { dyn_rocket_propulsion_fire(self.to_ptr(), "Fire") }
			}
		}
		impl From<$name> for BodyMover {
			fn from(value: $name) -> BodyMover {
				BodyMover(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_camera {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn c_frame(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "CFrame")) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "CFrame", value.to_ptr()) }
			}
			pub fn camera_subject(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "CameraSubject").map(|id| Instance(id)) }
			}
			pub fn set_camera_subject(&self, value: Option<Instance>) {
				unsafe { set_instance_property(self.to_ptr(), "CameraSubject", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			#[deprecated]
			pub fn coordinate_frame(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "CoordinateFrame")) }
			}
			#[deprecated]
			pub fn set_coordinate_frame(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "CoordinateFrame", value.to_ptr()) }
			}
			pub fn diagonal_field_of_view(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DiagonalFieldOfView") }
			}
			pub fn set_diagonal_field_of_view(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DiagonalFieldOfView", value) }
			}
			pub fn field_of_view(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "FieldOfView") }
			}
			pub fn set_field_of_view(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "FieldOfView", value) }
			}
			pub fn focus(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "Focus")) }
			}
			pub fn set_focus(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "Focus", value.to_ptr()) }
			}
			pub fn head_locked(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "HeadLocked") }
			}
			pub fn set_head_locked(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "HeadLocked", value) }
			}
			pub fn head_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "HeadScale") }
			}
			pub fn set_head_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "HeadScale", value) }
			}
			pub fn max_axis_field_of_view(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxAxisFieldOfView") }
			}
			pub fn set_max_axis_field_of_view(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxAxisFieldOfView", value) }
			}
			pub fn near_plane_z(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "NearPlaneZ") }
			}
			pub fn viewport_size(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "ViewportSize")) }
			}
			pub fn fn_get_pan_speed(&self) -> f64 {
				unsafe { dyn_camera_get_pan_speed(self.to_ptr(), "GetPanSpeed") }
			}
			pub fn fn_get_render_c_frame(&self) -> CFrame {
				unsafe { dyn_camera_get_render_c_frame(self.to_ptr(), "GetRenderCFrame") }
			}
			pub fn fn_get_roll(&self) -> f64 {
				unsafe { dyn_camera_get_roll(self.to_ptr(), "GetRoll") }
			}
			pub fn fn_get_tilt_speed(&self) -> f64 {
				unsafe { dyn_camera_get_tilt_speed(self.to_ptr(), "GetTiltSpeed") }
			}
			pub fn fn_screen_point_to_ray(&self, x: f64, y: f64, depth: f64) -> Ray {
				unsafe { dyn_camera_screen_point_to_ray(self.to_ptr(), "ScreenPointToRay", x, y, depth) }
			}
			pub fn fn_set_roll(&self, roll_angle: f64) {
				unsafe { dyn_camera_set_roll(self.to_ptr(), "SetRoll", roll_angle) }
			}
			pub fn fn_viewport_point_to_ray(&self, x: f64, y: f64, depth: f64) -> Ray {
				unsafe { dyn_camera_viewport_point_to_ray(self.to_ptr(), "ViewportPointToRay", x, y, depth) }
			}
			pub fn fn_world_to_screen_point(&self, world_point: Vector3) {
				unsafe { dyn_camera_world_to_screen_point(self.to_ptr(), "WorldToScreenPoint", world_point) }
			}
			pub fn fn_world_to_viewport_point(&self, world_point: Vector3) {
				unsafe { dyn_camera_world_to_viewport_point(self.to_ptr(), "WorldToViewportPoint", world_point) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_character_appearance {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_body_colors {
	($name:ident) => {
		impl_character_appearance!($name);
		impl $name {
			pub fn head_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "HeadColor")) }
			}
			pub fn set_head_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "HeadColor", value.to_ptr()) }
			}
			pub fn head_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "HeadColor3")) }
			}
			pub fn set_head_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "HeadColor3", value.to_ptr()) }
			}
			pub fn left_arm_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "LeftArmColor")) }
			}
			pub fn set_left_arm_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "LeftArmColor", value.to_ptr()) }
			}
			pub fn left_arm_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "LeftArmColor3")) }
			}
			pub fn set_left_arm_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "LeftArmColor3", value.to_ptr()) }
			}
			pub fn left_leg_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "LeftLegColor")) }
			}
			pub fn set_left_leg_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "LeftLegColor", value.to_ptr()) }
			}
			pub fn left_leg_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "LeftLegColor3")) }
			}
			pub fn set_left_leg_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "LeftLegColor3", value.to_ptr()) }
			}
			pub fn right_arm_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "RightArmColor")) }
			}
			pub fn set_right_arm_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "RightArmColor", value.to_ptr()) }
			}
			pub fn right_arm_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "RightArmColor3")) }
			}
			pub fn set_right_arm_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "RightArmColor3", value.to_ptr()) }
			}
			pub fn right_leg_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "RightLegColor")) }
			}
			pub fn set_right_leg_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "RightLegColor", value.to_ptr()) }
			}
			pub fn right_leg_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "RightLegColor3")) }
			}
			pub fn set_right_leg_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "RightLegColor3", value.to_ptr()) }
			}
			pub fn torso_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "TorsoColor")) }
			}
			pub fn set_torso_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "TorsoColor", value.to_ptr()) }
			}
			pub fn torso_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "TorsoColor3")) }
			}
			pub fn set_torso_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "TorsoColor3", value.to_ptr()) }
			}
		}
		impl From<$name> for CharacterAppearance {
			fn from(value: $name) -> CharacterAppearance {
				CharacterAppearance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_character_mesh {
	($name:ident) => {
		impl_character_appearance!($name);
		impl $name {
			pub fn base_texture_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "BaseTextureId") }
			}
			pub fn set_base_texture_id(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "BaseTextureId", value) }
			}
			pub fn mesh_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MeshId") }
			}
			pub fn set_mesh_id(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MeshId", value) }
			}
			pub fn overlay_texture_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "OverlayTextureId") }
			}
			pub fn set_overlay_texture_id(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "OverlayTextureId", value) }
			}
		}
		impl From<$name> for CharacterAppearance {
			fn from(value: $name) -> CharacterAppearance {
				CharacterAppearance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_clothing {
	($name:ident) => {
		impl_character_appearance!($name);
		impl $name {
			pub fn color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Color3")) }
			}
			pub fn set_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Color3", value.to_ptr()) }
			}
		}
		impl From<$name> for CharacterAppearance {
			fn from(value: $name) -> CharacterAppearance {
				CharacterAppearance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pants {
	($name:ident) => {
		impl_clothing!($name);
		impl $name {
			pub fn pants_template(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "PantsTemplate")) }
			}
			pub fn set_pants_template(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "PantsTemplate", value.to_ptr()) }
			}
		}
		impl From<$name> for Clothing {
			fn from(value: $name) -> Clothing {
				Clothing(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_shirt {
	($name:ident) => {
		impl_clothing!($name);
		impl $name {
			pub fn shirt_template(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "ShirtTemplate")) }
			}
			pub fn set_shirt_template(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "ShirtTemplate", value.to_ptr()) }
			}
		}
		impl From<$name> for Clothing {
			fn from(value: $name) -> Clothing {
				Clothing(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_shirt_graphic {
	($name:ident) => {
		impl_character_appearance!($name);
		impl $name {
			pub fn color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Color3")) }
			}
			pub fn set_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Color3", value.to_ptr()) }
			}
			pub fn graphic(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "Graphic")) }
			}
			pub fn set_graphic(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "Graphic", value.to_ptr()) }
			}
		}
		impl From<$name> for CharacterAppearance {
			fn from(value: $name) -> CharacterAppearance {
				CharacterAppearance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_click_detector {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn cursor_icon(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "CursorIcon")) }
			}
			pub fn set_cursor_icon(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "CursorIcon", value.to_ptr()) }
			}
			pub fn max_activation_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxActivationDistance") }
			}
			pub fn set_max_activation_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxActivationDistance", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_clouds {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Color")) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn cover(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Cover") }
			}
			pub fn set_cover(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Cover", value) }
			}
			pub fn density(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Density") }
			}
			pub fn set_density(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Density", value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_command_instance {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn allow_gui_access_points(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AllowGUIAccessPoints") }
			}
			pub fn display_name(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "DisplayName") }
			}
			pub fn set_display_name(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "DisplayName", &value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_constraint {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn active(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Active") }
			}
			pub fn attachment_0(&self) -> Option<Attachment> {
				unsafe { get_instance_property(self.to_ptr(), "Attachment0").map(|id| Attachment(id)) }
			}
			pub fn set_attachment_0(&self, value: Option<Attachment>) {
				unsafe { set_instance_property(self.to_ptr(), "Attachment0", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn attachment_1(&self) -> Option<Attachment> {
				unsafe { get_instance_property(self.to_ptr(), "Attachment1").map(|id| Attachment(id)) }
			}
			pub fn set_attachment_1(&self, value: Option<Attachment>) {
				unsafe { set_instance_property(self.to_ptr(), "Attachment1", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "Color")) }
			}
			pub fn set_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn visible(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Visible") }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Visible", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_align_orientation {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn c_frame(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "CFrame")) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "CFrame", value.to_ptr()) }
			}
			pub fn max_angular_velocity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxAngularVelocity") }
			}
			pub fn set_max_angular_velocity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxAngularVelocity", value) }
			}
			pub fn max_torque(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxTorque") }
			}
			pub fn set_max_torque(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxTorque", value) }
			}
			pub fn primary_axis(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "PrimaryAxis")) }
			}
			pub fn set_primary_axis(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "PrimaryAxis", value.to_ptr()) }
			}
			pub fn primary_axis_only(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "PrimaryAxisOnly") }
			}
			pub fn set_primary_axis_only(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "PrimaryAxisOnly", value) }
			}
			pub fn reaction_torque_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ReactionTorqueEnabled") }
			}
			pub fn set_reaction_torque_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ReactionTorqueEnabled", value) }
			}
			pub fn responsiveness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Responsiveness") }
			}
			pub fn set_responsiveness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Responsiveness", value) }
			}
			pub fn rigidity_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "RigidityEnabled") }
			}
			pub fn set_rigidity_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "RigidityEnabled", value) }
			}
			pub fn secondary_axis(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "SecondaryAxis")) }
			}
			pub fn set_secondary_axis(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "SecondaryAxis", value.to_ptr()) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_align_position {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn apply_at_center_of_mass(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ApplyAtCenterOfMass") }
			}
			pub fn set_apply_at_center_of_mass(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ApplyAtCenterOfMass", value) }
			}
			pub fn max_force(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxForce") }
			}
			pub fn set_max_force(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxForce", value) }
			}
			pub fn max_velocity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxVelocity") }
			}
			pub fn set_max_velocity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxVelocity", value) }
			}
			pub fn position(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Position")) }
			}
			pub fn set_position(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Position", value.to_ptr()) }
			}
			pub fn reaction_force_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ReactionForceEnabled") }
			}
			pub fn set_reaction_force_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ReactionForceEnabled", value) }
			}
			pub fn responsiveness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Responsiveness") }
			}
			pub fn set_responsiveness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Responsiveness", value) }
			}
			pub fn rigidity_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "RigidityEnabled") }
			}
			pub fn set_rigidity_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "RigidityEnabled", value) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_angular_velocity {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn angular_velocity(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "AngularVelocity")) }
			}
			pub fn set_angular_velocity(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "AngularVelocity", value.to_ptr()) }
			}
			pub fn max_torque(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxTorque") }
			}
			pub fn set_max_torque(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxTorque", value) }
			}
			pub fn reaction_torque_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ReactionTorqueEnabled") }
			}
			pub fn set_reaction_torque_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ReactionTorqueEnabled", value) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ball_socket_constraint {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn limits_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "LimitsEnabled") }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "LimitsEnabled", value) }
			}
			pub fn max_friction_torque(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxFrictionTorque") }
			}
			pub fn set_max_friction_torque(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxFrictionTorque", value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Radius") }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Radius", value) }
			}
			pub fn restitution(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Restitution") }
			}
			pub fn set_restitution(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Restitution", value) }
			}
			pub fn twist_limits_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TwistLimitsEnabled") }
			}
			pub fn set_twist_limits_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "TwistLimitsEnabled", value) }
			}
			pub fn twist_lower_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TwistLowerAngle") }
			}
			pub fn set_twist_lower_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TwistLowerAngle", value) }
			}
			pub fn twist_upper_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TwistUpperAngle") }
			}
			pub fn set_twist_upper_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TwistUpperAngle", value) }
			}
			pub fn upper_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "UpperAngle") }
			}
			pub fn set_upper_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "UpperAngle", value) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_hinge_constraint {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn angular_responsiveness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "AngularResponsiveness") }
			}
			pub fn set_angular_responsiveness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "AngularResponsiveness", value) }
			}
			pub fn angular_speed(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "AngularSpeed") }
			}
			pub fn set_angular_speed(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "AngularSpeed", value) }
			}
			pub fn angular_velocity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "AngularVelocity") }
			}
			pub fn set_angular_velocity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "AngularVelocity", value) }
			}
			pub fn current_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CurrentAngle") }
			}
			pub fn limits_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "LimitsEnabled") }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "LimitsEnabled", value) }
			}
			pub fn lower_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LowerAngle") }
			}
			pub fn set_lower_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LowerAngle", value) }
			}
			pub fn motor_max_acceleration(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MotorMaxAcceleration") }
			}
			pub fn set_motor_max_acceleration(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MotorMaxAcceleration", value) }
			}
			pub fn motor_max_torque(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MotorMaxTorque") }
			}
			pub fn set_motor_max_torque(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MotorMaxTorque", value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Radius") }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Radius", value) }
			}
			pub fn restitution(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Restitution") }
			}
			pub fn set_restitution(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Restitution", value) }
			}
			pub fn servo_max_torque(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ServoMaxTorque") }
			}
			pub fn set_servo_max_torque(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ServoMaxTorque", value) }
			}
			pub fn target_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TargetAngle") }
			}
			pub fn set_target_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TargetAngle", value) }
			}
			pub fn upper_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "UpperAngle") }
			}
			pub fn set_upper_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "UpperAngle", value) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_line_force {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn apply_at_center_of_mass(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ApplyAtCenterOfMass") }
			}
			pub fn set_apply_at_center_of_mass(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ApplyAtCenterOfMass", value) }
			}
			pub fn inverse_square_law(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "InverseSquareLaw") }
			}
			pub fn set_inverse_square_law(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "InverseSquareLaw", value) }
			}
			pub fn magnitude(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Magnitude") }
			}
			pub fn set_magnitude(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Magnitude", value) }
			}
			pub fn max_force(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxForce") }
			}
			pub fn set_max_force(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxForce", value) }
			}
			pub fn reaction_force_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ReactionForceEnabled") }
			}
			pub fn set_reaction_force_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ReactionForceEnabled", value) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_linear_velocity {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn line_direction(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "LineDirection")) }
			}
			pub fn set_line_direction(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "LineDirection", value.to_ptr()) }
			}
			pub fn line_velocity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LineVelocity") }
			}
			pub fn set_line_velocity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LineVelocity", value) }
			}
			pub fn max_force(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxForce") }
			}
			pub fn set_max_force(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxForce", value) }
			}
			pub fn plane_velocity(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "PlaneVelocity")) }
			}
			pub fn set_plane_velocity(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "PlaneVelocity", value.to_ptr()) }
			}
			pub fn primary_tangent_axis(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "PrimaryTangentAxis")) }
			}
			pub fn set_primary_tangent_axis(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "PrimaryTangentAxis", value.to_ptr()) }
			}
			pub fn secondary_tangent_axis(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "SecondaryTangentAxis")) }
			}
			pub fn set_secondary_tangent_axis(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "SecondaryTangentAxis", value.to_ptr()) }
			}
			pub fn vector_velocity(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "VectorVelocity")) }
			}
			pub fn set_vector_velocity(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "VectorVelocity", value.to_ptr()) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_plane_constraint {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_plane {
	($name:ident) => {
		impl_plane_constraint!($name);
		impl $name {
		}
		impl From<$name> for PlaneConstraint {
			fn from(value: $name) -> PlaneConstraint {
				PlaneConstraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_rigid_constraint {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn broken(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Broken") }
			}
			pub fn destruction_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "DestructionEnabled") }
			}
			pub fn set_destruction_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "DestructionEnabled", value) }
			}
			pub fn destruction_force(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DestructionForce") }
			}
			pub fn set_destruction_force(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DestructionForce", value) }
			}
			pub fn destruction_torque(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DestructionTorque") }
			}
			pub fn set_destruction_torque(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DestructionTorque", value) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_rod_constraint {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn current_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CurrentDistance") }
			}
			pub fn length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Length") }
			}
			pub fn set_length(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Length", value) }
			}
			pub fn limit_angle_0(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LimitAngle0") }
			}
			pub fn set_limit_angle_0(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LimitAngle0", value) }
			}
			pub fn limit_angle_1(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LimitAngle1") }
			}
			pub fn set_limit_angle_1(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LimitAngle1", value) }
			}
			pub fn limits_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "LimitsEnabled") }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "LimitsEnabled", value) }
			}
			pub fn thickness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Thickness") }
			}
			pub fn set_thickness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Thickness", value) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_rope_constraint {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn current_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CurrentDistance") }
			}
			pub fn length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Length") }
			}
			pub fn set_length(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Length", value) }
			}
			pub fn restitution(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Restitution") }
			}
			pub fn set_restitution(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Restitution", value) }
			}
			pub fn thickness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Thickness") }
			}
			pub fn set_thickness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Thickness", value) }
			}
			pub fn winch_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "WinchEnabled") }
			}
			pub fn set_winch_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "WinchEnabled", value) }
			}
			pub fn winch_force(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WinchForce") }
			}
			pub fn set_winch_force(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WinchForce", value) }
			}
			pub fn winch_responsiveness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WinchResponsiveness") }
			}
			pub fn set_winch_responsiveness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WinchResponsiveness", value) }
			}
			pub fn winch_speed(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WinchSpeed") }
			}
			pub fn set_winch_speed(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WinchSpeed", value) }
			}
			pub fn winch_target(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WinchTarget") }
			}
			pub fn set_winch_target(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WinchTarget", value) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_sliding_ball_constraint {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn current_position(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CurrentPosition") }
			}
			pub fn limits_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "LimitsEnabled") }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "LimitsEnabled", value) }
			}
			pub fn linear_responsiveness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LinearResponsiveness") }
			}
			pub fn set_linear_responsiveness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LinearResponsiveness", value) }
			}
			pub fn lower_limit(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LowerLimit") }
			}
			pub fn set_lower_limit(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LowerLimit", value) }
			}
			pub fn motor_max_acceleration(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MotorMaxAcceleration") }
			}
			pub fn set_motor_max_acceleration(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MotorMaxAcceleration", value) }
			}
			pub fn motor_max_force(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MotorMaxForce") }
			}
			pub fn set_motor_max_force(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MotorMaxForce", value) }
			}
			pub fn restitution(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Restitution") }
			}
			pub fn set_restitution(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Restitution", value) }
			}
			pub fn servo_max_force(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ServoMaxForce") }
			}
			pub fn set_servo_max_force(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ServoMaxForce", value) }
			}
			pub fn size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Size") }
			}
			pub fn set_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Size", value) }
			}
			pub fn speed(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Speed") }
			}
			pub fn set_speed(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Speed", value) }
			}
			pub fn target_position(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TargetPosition") }
			}
			pub fn set_target_position(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TargetPosition", value) }
			}
			pub fn upper_limit(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "UpperLimit") }
			}
			pub fn set_upper_limit(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "UpperLimit", value) }
			}
			pub fn velocity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Velocity") }
			}
			pub fn set_velocity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Velocity", value) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_cylindrical_constraint {
	($name:ident) => {
		impl_sliding_ball_constraint!($name);
		impl $name {
			pub fn angular_limits_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AngularLimitsEnabled") }
			}
			pub fn set_angular_limits_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AngularLimitsEnabled", value) }
			}
			pub fn angular_responsiveness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "AngularResponsiveness") }
			}
			pub fn set_angular_responsiveness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "AngularResponsiveness", value) }
			}
			pub fn angular_restitution(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "AngularRestitution") }
			}
			pub fn set_angular_restitution(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "AngularRestitution", value) }
			}
			pub fn angular_speed(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "AngularSpeed") }
			}
			pub fn set_angular_speed(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "AngularSpeed", value) }
			}
			pub fn angular_velocity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "AngularVelocity") }
			}
			pub fn set_angular_velocity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "AngularVelocity", value) }
			}
			pub fn current_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CurrentAngle") }
			}
			pub fn inclination_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "InclinationAngle") }
			}
			pub fn set_inclination_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "InclinationAngle", value) }
			}
			pub fn lower_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LowerAngle") }
			}
			pub fn set_lower_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LowerAngle", value) }
			}
			pub fn motor_max_angular_acceleration(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MotorMaxAngularAcceleration") }
			}
			pub fn set_motor_max_angular_acceleration(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MotorMaxAngularAcceleration", value) }
			}
			pub fn motor_max_torque(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MotorMaxTorque") }
			}
			pub fn set_motor_max_torque(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MotorMaxTorque", value) }
			}
			pub fn rotation_axis_visible(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "RotationAxisVisible") }
			}
			pub fn set_rotation_axis_visible(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "RotationAxisVisible", value) }
			}
			pub fn servo_max_torque(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ServoMaxTorque") }
			}
			pub fn set_servo_max_torque(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ServoMaxTorque", value) }
			}
			pub fn target_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TargetAngle") }
			}
			pub fn set_target_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TargetAngle", value) }
			}
			pub fn upper_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "UpperAngle") }
			}
			pub fn set_upper_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "UpperAngle", value) }
			}
			pub fn world_rotation_axis(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "WorldRotationAxis")) }
			}
		}
		impl From<$name> for SlidingBallConstraint {
			fn from(value: $name) -> SlidingBallConstraint {
				SlidingBallConstraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_prismatic_constraint {
	($name:ident) => {
		impl_sliding_ball_constraint!($name);
		impl $name {
		}
		impl From<$name> for SlidingBallConstraint {
			fn from(value: $name) -> SlidingBallConstraint {
				SlidingBallConstraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_spring_constraint {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn coils(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Coils") }
			}
			pub fn set_coils(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Coils", value) }
			}
			pub fn current_length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CurrentLength") }
			}
			pub fn damping(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Damping") }
			}
			pub fn set_damping(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Damping", value) }
			}
			pub fn free_length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "FreeLength") }
			}
			pub fn set_free_length(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "FreeLength", value) }
			}
			pub fn limits_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "LimitsEnabled") }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "LimitsEnabled", value) }
			}
			pub fn max_force(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxForce") }
			}
			pub fn set_max_force(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxForce", value) }
			}
			pub fn max_length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxLength") }
			}
			pub fn set_max_length(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxLength", value) }
			}
			pub fn min_length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MinLength") }
			}
			pub fn set_min_length(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MinLength", value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Radius") }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Radius", value) }
			}
			pub fn stiffness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Stiffness") }
			}
			pub fn set_stiffness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Stiffness", value) }
			}
			pub fn thickness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Thickness") }
			}
			pub fn set_thickness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Thickness", value) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_torque {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn torque(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Torque")) }
			}
			pub fn set_torque(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Torque", value.to_ptr()) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_torsion_spring_constraint {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn coils(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Coils") }
			}
			pub fn set_coils(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Coils", value) }
			}
			pub fn current_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CurrentAngle") }
			}
			pub fn damping(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Damping") }
			}
			pub fn set_damping(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Damping", value) }
			}
			#[deprecated]
			pub fn limit_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "LimitEnabled") }
			}
			#[deprecated]
			pub fn set_limit_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "LimitEnabled", value) }
			}
			pub fn limits_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "LimitsEnabled") }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "LimitsEnabled", value) }
			}
			pub fn max_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxAngle") }
			}
			pub fn set_max_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxAngle", value) }
			}
			pub fn max_torque(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxTorque") }
			}
			pub fn set_max_torque(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxTorque", value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Radius") }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Radius", value) }
			}
			pub fn restitution(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Restitution") }
			}
			pub fn set_restitution(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Restitution", value) }
			}
			pub fn stiffness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Stiffness") }
			}
			pub fn set_stiffness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Stiffness", value) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_universal_constraint {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn limits_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "LimitsEnabled") }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "LimitsEnabled", value) }
			}
			pub fn max_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxAngle") }
			}
			pub fn set_max_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxAngle", value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Radius") }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Radius", value) }
			}
			pub fn restitution(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Restitution") }
			}
			pub fn set_restitution(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Restitution", value) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_vector_force {
	($name:ident) => {
		impl_constraint!($name);
		impl $name {
			pub fn apply_at_center_of_mass(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ApplyAtCenterOfMass") }
			}
			pub fn set_apply_at_center_of_mass(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ApplyAtCenterOfMass", value) }
			}
			pub fn force(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Force")) }
			}
			pub fn set_force(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Force", value.to_ptr()) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_controller {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_humanoid_controller {
	($name:ident) => {
		impl_controller!($name);
		impl $name {
		}
		impl From<$name> for Controller {
			fn from(value: $name) -> Controller {
				Controller(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_skateboard_controller {
	($name:ident) => {
		impl_controller!($name);
		impl $name {
			pub fn steer(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Steer") }
			}
			pub fn throttle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Throttle") }
			}
		}
		impl From<$name> for Controller {
			fn from(value: $name) -> Controller {
				Controller(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_vehicle_controller {
	($name:ident) => {
		impl_controller!($name);
		impl $name {
		}
		impl From<$name> for Controller {
			fn from(value: $name) -> Controller {
				Controller(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_model_mesh {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn offset(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Offset")) }
			}
			pub fn set_offset(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Offset", value.to_ptr()) }
			}
			pub fn scale(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Scale")) }
			}
			pub fn set_scale(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Scale", value.to_ptr()) }
			}
			pub fn vertex_color(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "VertexColor")) }
			}
			pub fn set_vertex_color(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "VertexColor", value.to_ptr()) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_bevel_mesh {
	($name:ident) => {
		impl_data_model_mesh!($name);
		impl $name {
		}
		impl From<$name> for DataModelMesh {
			fn from(value: $name) -> DataModelMesh {
				DataModelMesh(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_block_mesh {
	($name:ident) => {
		impl_bevel_mesh!($name);
		impl $name {
		}
		impl From<$name> for BevelMesh {
			fn from(value: $name) -> BevelMesh {
				BevelMesh(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_cylinder_mesh {
	($name:ident) => {
		impl_bevel_mesh!($name);
		impl $name {
		}
		impl From<$name> for BevelMesh {
			fn from(value: $name) -> BevelMesh {
				BevelMesh(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_file_mesh {
	($name:ident) => {
		impl_data_model_mesh!($name);
		impl $name {
			pub fn mesh_id(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "MeshId")) }
			}
			pub fn set_mesh_id(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "MeshId", value.to_ptr()) }
			}
			pub fn texture_id(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "TextureId")) }
			}
			pub fn set_texture_id(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "TextureId", value.to_ptr()) }
			}
		}
		impl From<$name> for DataModelMesh {
			fn from(value: $name) -> DataModelMesh {
				DataModelMesh(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_special_mesh {
	($name:ident) => {
		impl_file_mesh!($name);
		impl $name {
		}
		impl From<$name> for FileMesh {
			fn from(value: $name) -> FileMesh {
				FileMesh(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_increment_options {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_get_metadata(&self) {
				unsafe { dyn_data_store_increment_options_get_metadata(self.to_ptr(), "GetMetadata") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_info {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn created_time(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CreatedTime") }
			}
			pub fn data_store_name(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "DataStoreName") }
			}
			pub fn updated_time(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "UpdatedTime") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_key {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn key_name(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "KeyName") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_key_info {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn created_time(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CreatedTime") }
			}
			pub fn updated_time(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "UpdatedTime") }
			}
			pub fn version(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Version") }
			}
			pub fn fn_get_metadata(&self) {
				unsafe { dyn_data_store_key_info_get_metadata(self.to_ptr(), "GetMetadata") }
			}
			pub fn fn_get_user_ids(&self) {
				unsafe { dyn_data_store_key_info_get_user_ids(self.to_ptr(), "GetUserIds") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_object_version_info {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn created_time(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CreatedTime") }
			}
			pub fn is_deleted(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsDeleted") }
			}
			pub fn version(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Version") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_options {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn all_scopes(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AllScopes") }
			}
			pub fn set_all_scopes(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AllScopes", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_set_options {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_get_metadata(&self) {
				unsafe { dyn_data_store_set_options_get_metadata(self.to_ptr(), "GetMetadata") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_dialog {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn conversation_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ConversationDistance") }
			}
			pub fn set_conversation_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ConversationDistance", value) }
			}
			pub fn goodbye_choice_active(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "GoodbyeChoiceActive") }
			}
			pub fn set_goodbye_choice_active(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "GoodbyeChoiceActive", value) }
			}
			pub fn goodbye_dialog(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "GoodbyeDialog") }
			}
			pub fn set_goodbye_dialog(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "GoodbyeDialog", &value) }
			}
			pub fn in_use(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "InUse") }
			}
			pub fn set_in_use(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "InUse", value) }
			}
			pub fn initial_prompt(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "InitialPrompt") }
			}
			pub fn set_initial_prompt(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "InitialPrompt", &value) }
			}
			pub fn trigger_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TriggerDistance") }
			}
			pub fn set_trigger_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TriggerDistance", value) }
			}
			pub fn trigger_offset(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "TriggerOffset")) }
			}
			pub fn set_trigger_offset(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "TriggerOffset", value.to_ptr()) }
			}
			pub fn fn_get_current_players(&self) -> Objects {
				unsafe { dyn_dialog_get_current_players(self.to_ptr(), "GetCurrentPlayers") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_dialog_choice {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn goodbye_choice_active(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "GoodbyeChoiceActive") }
			}
			pub fn set_goodbye_choice_active(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "GoodbyeChoiceActive", value) }
			}
			pub fn goodbye_dialog(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "GoodbyeDialog") }
			}
			pub fn set_goodbye_dialog(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "GoodbyeDialog", &value) }
			}
			pub fn response_dialog(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "ResponseDialog") }
			}
			pub fn set_response_dialog(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "ResponseDialog", &value) }
			}
			pub fn user_dialog(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "UserDialog") }
			}
			pub fn set_user_dialog(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "UserDialog", &value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_dragger {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_mouse_down(&self, mouse_part: Option<Instance>, point_on_mouse_part: Vector3, parts: Objects) {
				unsafe { dyn_dragger_mouse_down(self.to_ptr(), "MouseDown", mouse_part, point_on_mouse_part, parts) }
			}
			pub fn fn_mouse_move(&self, mouse_ray: Ray) {
				unsafe { dyn_dragger_mouse_move(self.to_ptr(), "MouseMove", mouse_ray) }
			}
			pub fn fn_mouse_up(&self) {
				unsafe { dyn_dragger_mouse_up(self.to_ptr(), "MouseUp") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_euler_rotation_curve {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_get_angles_at_time(&self, time: f64) {
				unsafe { dyn_euler_rotation_curve_get_angles_at_time(self.to_ptr(), "GetAnglesAtTime", time) }
			}
			pub fn fn_get_rotation_at_time(&self, time: f64) -> CFrame {
				unsafe { dyn_euler_rotation_curve_get_rotation_at_time(self.to_ptr(), "GetRotationAtTime", time) }
			}
			pub fn fn_x(&self) -> Option<FloatCurve> {
				unsafe { dyn_euler_rotation_curve_x(self.to_ptr(), "X") }
			}
			pub fn fn_y(&self) -> Option<FloatCurve> {
				unsafe { dyn_euler_rotation_curve_y(self.to_ptr(), "Y") }
			}
			pub fn fn_z(&self) -> Option<FloatCurve> {
				unsafe { dyn_euler_rotation_curve_z(self.to_ptr(), "Z") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_explosion {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn blast_pressure(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "BlastPressure") }
			}
			pub fn set_blast_pressure(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "BlastPressure", value) }
			}
			pub fn blast_radius(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "BlastRadius") }
			}
			pub fn set_blast_radius(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "BlastRadius", value) }
			}
			pub fn destroy_joint_radius_percent(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DestroyJointRadiusPercent") }
			}
			pub fn set_destroy_joint_radius_percent(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DestroyJointRadiusPercent", value) }
			}
			pub fn position(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Position")) }
			}
			pub fn set_position(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Position", value.to_ptr()) }
			}
			pub fn time_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TimeScale") }
			}
			pub fn set_time_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TimeScale", value) }
			}
			pub fn visible(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Visible") }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Visible", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_face_instance {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_decal {
	($name:ident) => {
		impl_face_instance!($name);
		impl $name {
			pub fn color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Color3")) }
			}
			pub fn set_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Color3", value.to_ptr()) }
			}
			pub fn local_transparency_modifier(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LocalTransparencyModifier") }
			}
			pub fn set_local_transparency_modifier(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LocalTransparencyModifier", value) }
			}
			#[deprecated]
			pub fn shiny(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Shiny") }
			}
			#[deprecated]
			pub fn set_shiny(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Shiny", value) }
			}
			#[deprecated]
			pub fn specular(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Specular") }
			}
			#[deprecated]
			pub fn set_specular(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Specular", value) }
			}
			pub fn texture(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "Texture")) }
			}
			pub fn set_texture(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "Texture", value.to_ptr()) }
			}
			pub fn transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Transparency") }
			}
			pub fn set_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Transparency", value) }
			}
			pub fn z_index(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ZIndex") }
			}
			pub fn set_z_index(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ZIndex", value) }
			}
		}
		impl From<$name> for FaceInstance {
			fn from(value: $name) -> FaceInstance {
				FaceInstance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_texture {
	($name:ident) => {
		impl_decal!($name);
		impl $name {
			pub fn offset_studs_u(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "OffsetStudsU") }
			}
			pub fn set_offset_studs_u(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "OffsetStudsU", value) }
			}
			pub fn offset_studs_v(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "OffsetStudsV") }
			}
			pub fn set_offset_studs_v(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "OffsetStudsV", value) }
			}
			pub fn studs_per_tile_u(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "StudsPerTileU") }
			}
			pub fn set_studs_per_tile_u(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "StudsPerTileU", value) }
			}
			pub fn studs_per_tile_v(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "StudsPerTileV") }
			}
			pub fn set_studs_per_tile_v(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "StudsPerTileV", value) }
			}
		}
		impl From<$name> for Decal {
			fn from(value: $name) -> Decal {
				Decal(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_feature {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_fire {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Color")) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn heat(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Heat") }
			}
			pub fn set_heat(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Heat", value) }
			}
			pub fn secondary_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "SecondaryColor")) }
			}
			pub fn set_secondary_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "SecondaryColor", value.to_ptr()) }
			}
			pub fn size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Size") }
			}
			pub fn set_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Size", value) }
			}
			pub fn time_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TimeScale") }
			}
			pub fn set_time_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TimeScale", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_float_curve {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Length") }
			}
			pub fn fn_get_key_at_index(&self, index: f64) -> FloatCurveKey {
				unsafe { dyn_float_curve_get_key_at_index(self.to_ptr(), "GetKeyAtIndex", index) }
			}
			pub fn fn_get_key_indices_at_time(&self, time: f64) {
				unsafe { dyn_float_curve_get_key_indices_at_time(self.to_ptr(), "GetKeyIndicesAtTime", time) }
			}
			pub fn fn_get_keys(&self) {
				unsafe { dyn_float_curve_get_keys(self.to_ptr(), "GetKeys") }
			}
			pub fn fn_get_value_at_time(&self, time: f64) {
				unsafe { dyn_float_curve_get_value_at_time(self.to_ptr(), "GetValueAtTime", time) }
			}
			pub fn fn_insert_key(&self, key: FloatCurveKey) {
				unsafe { dyn_float_curve_insert_key(self.to_ptr(), "InsertKey", key) }
			}
			pub fn fn_remove_key_at_index(&self, starting_index: f64, count: f64) -> f64 {
				unsafe { dyn_float_curve_remove_key_at_index(self.to_ptr(), "RemoveKeyAtIndex", starting_index, count) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_folder {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_force_field {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn visible(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Visible") }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Visible", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_get_text_bounds_params {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn font(&self) -> Font {
				unsafe { Font(get_datatype_property(self.to_ptr(), "Font")) }
			}
			pub fn set_font(&self, value: Font) {
				unsafe { set_datatype_property(self.to_ptr(), "Font", value.to_ptr()) }
			}
			pub fn size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Size") }
			}
			pub fn set_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Size", value) }
			}
			pub fn text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Text") }
			}
			pub fn set_text(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "Text", &value) }
			}
			pub fn width(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Width") }
			}
			pub fn set_width(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Width", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_global_data_store {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_get_async(&self, key: &str) {
				unsafe { dyn_global_data_store_get_async(self.to_ptr(), "GetAsync", key) }
			}
			pub fn fn_remove_async(&self, key: &str) {
				unsafe { dyn_global_data_store_remove_async(self.to_ptr(), "RemoveAsync", key) }
			}
			pub fn fn_update_async(&self, key: &str, transform_function: Function) {
				unsafe { dyn_global_data_store_update_async(self.to_ptr(), "UpdateAsync", key, transform_function) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store {
	($name:ident) => {
		impl_global_data_store!($name);
		impl $name {
			pub fn fn_get_version_async(&self, key: &str, version: &str) {
				unsafe { dyn_data_store_get_version_async(self.to_ptr(), "GetVersionAsync", key, version) }
			}
			pub fn fn_list_keys_async(&self, prefix: &str, page_size: f64) -> Option<DataStoreKeyPages> {
				unsafe { dyn_data_store_list_keys_async(self.to_ptr(), "ListKeysAsync", prefix, page_size) }
			}
			pub fn fn_remove_version_async(&self, key: &str, version: &str) {
				unsafe { dyn_data_store_remove_version_async(self.to_ptr(), "RemoveVersionAsync", key, version) }
			}
		}
		impl From<$name> for GlobalDataStore {
			fn from(value: $name) -> GlobalDataStore {
				GlobalDataStore(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ordered_data_store {
	($name:ident) => {
		impl_global_data_store!($name);
		impl $name {
		}
		impl From<$name> for GlobalDataStore {
			fn from(value: $name) -> GlobalDataStore {
				GlobalDataStore(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_gui_base {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_gui_base_2_d {
	($name:ident) => {
		impl_gui_base!($name);
		impl $name {
			pub fn absolute_position(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "AbsolutePosition")) }
			}
			pub fn absolute_rotation(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "AbsoluteRotation") }
			}
			pub fn absolute_size(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "AbsoluteSize")) }
			}
			pub fn auto_localize(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AutoLocalize") }
			}
			pub fn set_auto_localize(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AutoLocalize", value) }
			}
			#[deprecated]
			pub fn localize(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Localize") }
			}
			#[deprecated]
			pub fn set_localize(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Localize", value) }
			}
			pub fn root_localization_table(&self) -> Option<LocalizationTable> {
				unsafe { get_instance_property(self.to_ptr(), "RootLocalizationTable").map(|id| LocalizationTable(id)) }
			}
			pub fn set_root_localization_table(&self, value: Option<LocalizationTable>) {
				unsafe { set_instance_property(self.to_ptr(), "RootLocalizationTable", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn selection_group(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "SelectionGroup") }
			}
			pub fn set_selection_group(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "SelectionGroup", value) }
			}
		}
		impl From<$name> for GuiBase {
			fn from(value: $name) -> GuiBase {
				GuiBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_gui_object {
	($name:ident) => {
		impl_gui_base_2_d!($name);
		impl $name {
			pub fn active(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Active") }
			}
			pub fn set_active(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Active", value) }
			}
			pub fn anchor_point(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "AnchorPoint")) }
			}
			pub fn set_anchor_point(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "AnchorPoint", value.to_ptr()) }
			}
			#[deprecated]
			pub fn background_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "BackgroundColor")) }
			}
			#[deprecated]
			pub fn set_background_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "BackgroundColor", value.to_ptr()) }
			}
			pub fn background_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "BackgroundColor3")) }
			}
			pub fn set_background_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "BackgroundColor3", value.to_ptr()) }
			}
			pub fn background_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "BackgroundTransparency") }
			}
			pub fn set_background_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "BackgroundTransparency", value) }
			}
			#[deprecated]
			pub fn border_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "BorderColor")) }
			}
			#[deprecated]
			pub fn set_border_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "BorderColor", value.to_ptr()) }
			}
			pub fn border_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "BorderColor3")) }
			}
			pub fn set_border_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "BorderColor3", value.to_ptr()) }
			}
			pub fn border_size_pixel(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "BorderSizePixel") }
			}
			pub fn set_border_size_pixel(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "BorderSizePixel", value) }
			}
			pub fn clips_descendants(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ClipsDescendants") }
			}
			pub fn set_clips_descendants(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ClipsDescendants", value) }
			}
			#[deprecated]
			pub fn draggable(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Draggable") }
			}
			#[deprecated]
			pub fn set_draggable(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Draggable", value) }
			}
			pub fn layout_order(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LayoutOrder") }
			}
			pub fn set_layout_order(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LayoutOrder", value) }
			}
			pub fn next_selection_down(&self) -> Option<GuiObject> {
				unsafe { get_instance_property(self.to_ptr(), "NextSelectionDown").map(|id| GuiObject(id)) }
			}
			pub fn set_next_selection_down(&self, value: Option<GuiObject>) {
				unsafe { set_instance_property(self.to_ptr(), "NextSelectionDown", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn next_selection_left(&self) -> Option<GuiObject> {
				unsafe { get_instance_property(self.to_ptr(), "NextSelectionLeft").map(|id| GuiObject(id)) }
			}
			pub fn set_next_selection_left(&self, value: Option<GuiObject>) {
				unsafe { set_instance_property(self.to_ptr(), "NextSelectionLeft", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn next_selection_right(&self) -> Option<GuiObject> {
				unsafe { get_instance_property(self.to_ptr(), "NextSelectionRight").map(|id| GuiObject(id)) }
			}
			pub fn set_next_selection_right(&self, value: Option<GuiObject>) {
				unsafe { set_instance_property(self.to_ptr(), "NextSelectionRight", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn next_selection_up(&self) -> Option<GuiObject> {
				unsafe { get_instance_property(self.to_ptr(), "NextSelectionUp").map(|id| GuiObject(id)) }
			}
			pub fn set_next_selection_up(&self, value: Option<GuiObject>) {
				unsafe { set_instance_property(self.to_ptr(), "NextSelectionUp", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn position(&self) -> UDim2 {
				unsafe { UDim2(get_datatype_property(self.to_ptr(), "Position")) }
			}
			pub fn set_position(&self, value: UDim2) {
				unsafe { set_datatype_property(self.to_ptr(), "Position", value.to_ptr()) }
			}
			pub fn rotation(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Rotation") }
			}
			pub fn set_rotation(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Rotation", value) }
			}
			pub fn selectable(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Selectable") }
			}
			pub fn set_selectable(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Selectable", value) }
			}
			pub fn selection_image_object(&self) -> Option<GuiObject> {
				unsafe { get_instance_property(self.to_ptr(), "SelectionImageObject").map(|id| GuiObject(id)) }
			}
			pub fn set_selection_image_object(&self, value: Option<GuiObject>) {
				unsafe { set_instance_property(self.to_ptr(), "SelectionImageObject", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn selection_order(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "SelectionOrder") }
			}
			pub fn set_selection_order(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "SelectionOrder", value) }
			}
			pub fn size(&self) -> UDim2 {
				unsafe { UDim2(get_datatype_property(self.to_ptr(), "Size")) }
			}
			pub fn set_size(&self, value: UDim2) {
				unsafe { set_datatype_property(self.to_ptr(), "Size", value.to_ptr()) }
			}
			pub fn transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Transparency") }
			}
			pub fn set_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Transparency", value) }
			}
			pub fn visible(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Visible") }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Visible", value) }
			}
			pub fn z_index(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ZIndex") }
			}
			pub fn set_z_index(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ZIndex", value) }
			}
		}
		impl From<$name> for GuiBase2d {
			fn from(value: $name) -> GuiBase2d {
				GuiBase2d(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_canvas_group {
	($name:ident) => {
		impl_gui_object!($name);
		impl $name {
			pub fn group_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "GroupColor3")) }
			}
			pub fn set_group_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "GroupColor3", value.to_ptr()) }
			}
			pub fn group_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "GroupTransparency") }
			}
			pub fn set_group_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "GroupTransparency", value) }
			}
		}
		impl From<$name> for GuiObject {
			fn from(value: $name) -> GuiObject {
				GuiObject(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_frame {
	($name:ident) => {
		impl_gui_object!($name);
		impl $name {
		}
		impl From<$name> for GuiObject {
			fn from(value: $name) -> GuiObject {
				GuiObject(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_gui_button {
	($name:ident) => {
		impl_gui_object!($name);
		impl $name {
			pub fn auto_button_color(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AutoButtonColor") }
			}
			pub fn set_auto_button_color(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AutoButtonColor", value) }
			}
			pub fn modal(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Modal") }
			}
			pub fn set_modal(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Modal", value) }
			}
			pub fn selected(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Selected") }
			}
			pub fn set_selected(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Selected", value) }
			}
		}
		impl From<$name> for GuiObject {
			fn from(value: $name) -> GuiObject {
				GuiObject(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_image_button {
	($name:ident) => {
		impl_gui_button!($name);
		impl $name {
			pub fn hover_image(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "HoverImage")) }
			}
			pub fn set_hover_image(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "HoverImage", value.to_ptr()) }
			}
			pub fn image(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "Image")) }
			}
			pub fn set_image(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "Image", value.to_ptr()) }
			}
			pub fn image_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "ImageColor3")) }
			}
			pub fn set_image_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "ImageColor3", value.to_ptr()) }
			}
			pub fn image_rect_offset(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "ImageRectOffset")) }
			}
			pub fn set_image_rect_offset(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "ImageRectOffset", value.to_ptr()) }
			}
			pub fn image_rect_size(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "ImageRectSize")) }
			}
			pub fn set_image_rect_size(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "ImageRectSize", value.to_ptr()) }
			}
			pub fn image_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ImageTransparency") }
			}
			pub fn set_image_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ImageTransparency", value) }
			}
			pub fn is_loaded(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsLoaded") }
			}
			pub fn pressed_image(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "PressedImage")) }
			}
			pub fn set_pressed_image(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "PressedImage", value.to_ptr()) }
			}
			pub fn slice_center(&self) -> Rect {
				unsafe { Rect(get_datatype_property(self.to_ptr(), "SliceCenter")) }
			}
			pub fn set_slice_center(&self, value: Rect) {
				unsafe { set_datatype_property(self.to_ptr(), "SliceCenter", value.to_ptr()) }
			}
			pub fn slice_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "SliceScale") }
			}
			pub fn set_slice_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "SliceScale", value) }
			}
			pub fn tile_size(&self) -> UDim2 {
				unsafe { UDim2(get_datatype_property(self.to_ptr(), "TileSize")) }
			}
			pub fn set_tile_size(&self, value: UDim2) {
				unsafe { set_datatype_property(self.to_ptr(), "TileSize", value.to_ptr()) }
			}
		}
		impl From<$name> for GuiButton {
			fn from(value: $name) -> GuiButton {
				GuiButton(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_button {
	($name:ident) => {
		impl_gui_button!($name);
		impl $name {
			pub fn content_text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "ContentText") }
			}
			pub fn font_face(&self) -> Font {
				unsafe { Font(get_datatype_property(self.to_ptr(), "FontFace")) }
			}
			pub fn set_font_face(&self, value: Font) {
				unsafe { set_datatype_property(self.to_ptr(), "FontFace", value.to_ptr()) }
			}
			pub fn line_height(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LineHeight") }
			}
			pub fn set_line_height(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LineHeight", value) }
			}
			pub fn localized_text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "LocalizedText") }
			}
			pub fn max_visible_graphemes(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxVisibleGraphemes") }
			}
			pub fn set_max_visible_graphemes(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxVisibleGraphemes", value) }
			}
			pub fn rich_text(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "RichText") }
			}
			pub fn set_rich_text(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "RichText", value) }
			}
			pub fn text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Text") }
			}
			pub fn set_text(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "Text", &value) }
			}
			pub fn text_bounds(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "TextBounds")) }
			}
			#[deprecated]
			pub fn text_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "TextColor")) }
			}
			#[deprecated]
			pub fn set_text_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "TextColor", value.to_ptr()) }
			}
			pub fn text_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "TextColor3")) }
			}
			pub fn set_text_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "TextColor3", value.to_ptr()) }
			}
			pub fn text_fits(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TextFits") }
			}
			pub fn text_scaled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TextScaled") }
			}
			pub fn set_text_scaled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "TextScaled", value) }
			}
			pub fn text_size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TextSize") }
			}
			pub fn set_text_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TextSize", value) }
			}
			pub fn text_stroke_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "TextStrokeColor3")) }
			}
			pub fn set_text_stroke_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "TextStrokeColor3", value.to_ptr()) }
			}
			pub fn text_stroke_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TextStrokeTransparency") }
			}
			pub fn set_text_stroke_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TextStrokeTransparency", value) }
			}
			pub fn text_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TextTransparency") }
			}
			pub fn set_text_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TextTransparency", value) }
			}
			#[deprecated]
			pub fn text_wrap(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TextWrap") }
			}
			#[deprecated]
			pub fn set_text_wrap(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "TextWrap", value) }
			}
			pub fn text_wrapped(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TextWrapped") }
			}
			pub fn set_text_wrapped(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "TextWrapped", value) }
			}
		}
		impl From<$name> for GuiButton {
			fn from(value: $name) -> GuiButton {
				GuiButton(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_gui_label {
	($name:ident) => {
		impl_gui_object!($name);
		impl $name {
		}
		impl From<$name> for GuiObject {
			fn from(value: $name) -> GuiObject {
				GuiObject(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_image_label {
	($name:ident) => {
		impl_gui_label!($name);
		impl $name {
			pub fn image(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "Image")) }
			}
			pub fn set_image(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "Image", value.to_ptr()) }
			}
			pub fn image_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "ImageColor3")) }
			}
			pub fn set_image_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "ImageColor3", value.to_ptr()) }
			}
			pub fn image_rect_offset(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "ImageRectOffset")) }
			}
			pub fn set_image_rect_offset(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "ImageRectOffset", value.to_ptr()) }
			}
			pub fn image_rect_size(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "ImageRectSize")) }
			}
			pub fn set_image_rect_size(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "ImageRectSize", value.to_ptr()) }
			}
			pub fn image_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ImageTransparency") }
			}
			pub fn set_image_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ImageTransparency", value) }
			}
			pub fn is_loaded(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsLoaded") }
			}
			pub fn slice_center(&self) -> Rect {
				unsafe { Rect(get_datatype_property(self.to_ptr(), "SliceCenter")) }
			}
			pub fn set_slice_center(&self, value: Rect) {
				unsafe { set_datatype_property(self.to_ptr(), "SliceCenter", value.to_ptr()) }
			}
			pub fn slice_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "SliceScale") }
			}
			pub fn set_slice_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "SliceScale", value) }
			}
			pub fn tile_size(&self) -> UDim2 {
				unsafe { UDim2(get_datatype_property(self.to_ptr(), "TileSize")) }
			}
			pub fn set_tile_size(&self, value: UDim2) {
				unsafe { set_datatype_property(self.to_ptr(), "TileSize", value.to_ptr()) }
			}
		}
		impl From<$name> for GuiLabel {
			fn from(value: $name) -> GuiLabel {
				GuiLabel(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_label {
	($name:ident) => {
		impl_gui_label!($name);
		impl $name {
			pub fn content_text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "ContentText") }
			}
			pub fn font_face(&self) -> Font {
				unsafe { Font(get_datatype_property(self.to_ptr(), "FontFace")) }
			}
			pub fn set_font_face(&self, value: Font) {
				unsafe { set_datatype_property(self.to_ptr(), "FontFace", value.to_ptr()) }
			}
			pub fn line_height(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LineHeight") }
			}
			pub fn set_line_height(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LineHeight", value) }
			}
			pub fn localized_text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "LocalizedText") }
			}
			pub fn max_visible_graphemes(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxVisibleGraphemes") }
			}
			pub fn set_max_visible_graphemes(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxVisibleGraphemes", value) }
			}
			pub fn rich_text(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "RichText") }
			}
			pub fn set_rich_text(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "RichText", value) }
			}
			pub fn text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Text") }
			}
			pub fn set_text(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "Text", &value) }
			}
			pub fn text_bounds(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "TextBounds")) }
			}
			#[deprecated]
			pub fn text_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "TextColor")) }
			}
			#[deprecated]
			pub fn set_text_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "TextColor", value.to_ptr()) }
			}
			pub fn text_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "TextColor3")) }
			}
			pub fn set_text_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "TextColor3", value.to_ptr()) }
			}
			pub fn text_fits(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TextFits") }
			}
			pub fn text_scaled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TextScaled") }
			}
			pub fn set_text_scaled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "TextScaled", value) }
			}
			pub fn text_size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TextSize") }
			}
			pub fn set_text_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TextSize", value) }
			}
			pub fn text_stroke_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "TextStrokeColor3")) }
			}
			pub fn set_text_stroke_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "TextStrokeColor3", value.to_ptr()) }
			}
			pub fn text_stroke_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TextStrokeTransparency") }
			}
			pub fn set_text_stroke_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TextStrokeTransparency", value) }
			}
			pub fn text_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TextTransparency") }
			}
			pub fn set_text_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TextTransparency", value) }
			}
			#[deprecated]
			pub fn text_wrap(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TextWrap") }
			}
			#[deprecated]
			pub fn set_text_wrap(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "TextWrap", value) }
			}
			pub fn text_wrapped(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TextWrapped") }
			}
			pub fn set_text_wrapped(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "TextWrapped", value) }
			}
		}
		impl From<$name> for GuiLabel {
			fn from(value: $name) -> GuiLabel {
				GuiLabel(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_scrolling_frame {
	($name:ident) => {
		impl_gui_object!($name);
		impl $name {
			pub fn absolute_canvas_size(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "AbsoluteCanvasSize")) }
			}
			pub fn absolute_window_size(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "AbsoluteWindowSize")) }
			}
			pub fn bottom_image(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "BottomImage")) }
			}
			pub fn set_bottom_image(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "BottomImage", value.to_ptr()) }
			}
			pub fn canvas_position(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "CanvasPosition")) }
			}
			pub fn set_canvas_position(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "CanvasPosition", value.to_ptr()) }
			}
			pub fn canvas_size(&self) -> UDim2 {
				unsafe { UDim2(get_datatype_property(self.to_ptr(), "CanvasSize")) }
			}
			pub fn set_canvas_size(&self, value: UDim2) {
				unsafe { set_datatype_property(self.to_ptr(), "CanvasSize", value.to_ptr()) }
			}
			pub fn mid_image(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "MidImage")) }
			}
			pub fn set_mid_image(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "MidImage", value.to_ptr()) }
			}
			pub fn scroll_bar_image_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "ScrollBarImageColor3")) }
			}
			pub fn set_scroll_bar_image_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "ScrollBarImageColor3", value.to_ptr()) }
			}
			pub fn scroll_bar_image_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ScrollBarImageTransparency") }
			}
			pub fn set_scroll_bar_image_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ScrollBarImageTransparency", value) }
			}
			pub fn scroll_bar_thickness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ScrollBarThickness") }
			}
			pub fn set_scroll_bar_thickness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ScrollBarThickness", value) }
			}
			pub fn scrolling_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ScrollingEnabled") }
			}
			pub fn set_scrolling_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ScrollingEnabled", value) }
			}
			pub fn top_image(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "TopImage")) }
			}
			pub fn set_top_image(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "TopImage", value.to_ptr()) }
			}
		}
		impl From<$name> for GuiObject {
			fn from(value: $name) -> GuiObject {
				GuiObject(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_box {
	($name:ident) => {
		impl_gui_object!($name);
		impl $name {
			pub fn clear_text_on_focus(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ClearTextOnFocus") }
			}
			pub fn set_clear_text_on_focus(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ClearTextOnFocus", value) }
			}
			pub fn content_text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "ContentText") }
			}
			pub fn cursor_position(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CursorPosition") }
			}
			pub fn set_cursor_position(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CursorPosition", value) }
			}
			pub fn font_face(&self) -> Font {
				unsafe { Font(get_datatype_property(self.to_ptr(), "FontFace")) }
			}
			pub fn set_font_face(&self, value: Font) {
				unsafe { set_datatype_property(self.to_ptr(), "FontFace", value.to_ptr()) }
			}
			pub fn line_height(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LineHeight") }
			}
			pub fn set_line_height(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LineHeight", value) }
			}
			pub fn max_visible_graphemes(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxVisibleGraphemes") }
			}
			pub fn set_max_visible_graphemes(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxVisibleGraphemes", value) }
			}
			pub fn multi_line(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "MultiLine") }
			}
			pub fn set_multi_line(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "MultiLine", value) }
			}
			pub fn placeholder_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "PlaceholderColor3")) }
			}
			pub fn set_placeholder_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "PlaceholderColor3", value.to_ptr()) }
			}
			pub fn placeholder_text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "PlaceholderText") }
			}
			pub fn set_placeholder_text(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "PlaceholderText", &value) }
			}
			pub fn rich_text(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "RichText") }
			}
			pub fn set_rich_text(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "RichText", value) }
			}
			pub fn selection_start(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "SelectionStart") }
			}
			pub fn set_selection_start(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "SelectionStart", value) }
			}
			pub fn show_native_input(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ShowNativeInput") }
			}
			pub fn set_show_native_input(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ShowNativeInput", value) }
			}
			pub fn text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Text") }
			}
			pub fn set_text(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "Text", &value) }
			}
			pub fn text_bounds(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "TextBounds")) }
			}
			#[deprecated]
			pub fn text_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "TextColor")) }
			}
			#[deprecated]
			pub fn set_text_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "TextColor", value.to_ptr()) }
			}
			pub fn text_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "TextColor3")) }
			}
			pub fn set_text_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "TextColor3", value.to_ptr()) }
			}
			pub fn text_editable(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TextEditable") }
			}
			pub fn set_text_editable(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "TextEditable", value) }
			}
			pub fn text_fits(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TextFits") }
			}
			pub fn text_scaled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TextScaled") }
			}
			pub fn set_text_scaled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "TextScaled", value) }
			}
			pub fn text_size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TextSize") }
			}
			pub fn set_text_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TextSize", value) }
			}
			pub fn text_stroke_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "TextStrokeColor3")) }
			}
			pub fn set_text_stroke_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "TextStrokeColor3", value.to_ptr()) }
			}
			pub fn text_stroke_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TextStrokeTransparency") }
			}
			pub fn set_text_stroke_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TextStrokeTransparency", value) }
			}
			pub fn text_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TextTransparency") }
			}
			pub fn set_text_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TextTransparency", value) }
			}
			#[deprecated]
			pub fn text_wrap(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TextWrap") }
			}
			#[deprecated]
			pub fn set_text_wrap(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "TextWrap", value) }
			}
			pub fn text_wrapped(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TextWrapped") }
			}
			pub fn set_text_wrapped(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "TextWrapped", value) }
			}
			pub fn fn_capture_focus(&self) {
				unsafe { dyn_text_box_capture_focus(self.to_ptr(), "CaptureFocus") }
			}
			pub fn fn_is_focused(&self) -> bool {
				unsafe { dyn_text_box_is_focused(self.to_ptr(), "IsFocused") }
			}
			pub fn fn_release_focus(&self, submitted: bool) {
				unsafe { dyn_text_box_release_focus(self.to_ptr(), "ReleaseFocus", submitted) }
			}
		}
		impl From<$name> for GuiObject {
			fn from(value: $name) -> GuiObject {
				GuiObject(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_video_frame {
	($name:ident) => {
		impl_gui_object!($name);
		impl $name {
			pub fn is_loaded(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsLoaded") }
			}
			pub fn looped(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Looped") }
			}
			pub fn set_looped(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Looped", value) }
			}
			pub fn playing(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Playing") }
			}
			pub fn set_playing(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Playing", value) }
			}
			pub fn resolution(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "Resolution")) }
			}
			pub fn time_length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TimeLength") }
			}
			pub fn time_position(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TimePosition") }
			}
			pub fn set_time_position(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TimePosition", value) }
			}
			pub fn video(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "Video")) }
			}
			pub fn set_video(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "Video", value.to_ptr()) }
			}
			pub fn volume(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Volume") }
			}
			pub fn set_volume(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Volume", value) }
			}
			pub fn fn_pause(&self) {
				unsafe { dyn_video_frame_pause(self.to_ptr(), "Pause") }
			}
			pub fn fn_play(&self) {
				unsafe { dyn_video_frame_play(self.to_ptr(), "Play") }
			}
		}
		impl From<$name> for GuiObject {
			fn from(value: $name) -> GuiObject {
				GuiObject(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_viewport_frame {
	($name:ident) => {
		impl_gui_object!($name);
		impl $name {
			pub fn ambient(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Ambient")) }
			}
			pub fn set_ambient(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Ambient", value.to_ptr()) }
			}
			pub fn current_camera(&self) -> Option<Camera> {
				unsafe { get_instance_property(self.to_ptr(), "CurrentCamera").map(|id| Camera(id)) }
			}
			pub fn set_current_camera(&self, value: Option<Camera>) {
				unsafe { set_instance_property(self.to_ptr(), "CurrentCamera", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn image_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "ImageColor3")) }
			}
			pub fn set_image_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "ImageColor3", value.to_ptr()) }
			}
			pub fn image_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ImageTransparency") }
			}
			pub fn set_image_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ImageTransparency", value) }
			}
			pub fn light_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "LightColor")) }
			}
			pub fn set_light_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "LightColor", value.to_ptr()) }
			}
			pub fn light_direction(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "LightDirection")) }
			}
			pub fn set_light_direction(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "LightDirection", value.to_ptr()) }
			}
		}
		impl From<$name> for GuiObject {
			fn from(value: $name) -> GuiObject {
				GuiObject(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_layer_collector {
	($name:ident) => {
		impl_gui_base_2_d!($name);
		impl $name {
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn reset_on_spawn(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ResetOnSpawn") }
			}
			pub fn set_reset_on_spawn(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ResetOnSpawn", value) }
			}
		}
		impl From<$name> for GuiBase2d {
			fn from(value: $name) -> GuiBase2d {
				GuiBase2d(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_billboard_gui {
	($name:ident) => {
		impl_layer_collector!($name);
		impl $name {
			pub fn active(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Active") }
			}
			pub fn set_active(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Active", value) }
			}
			pub fn adornee(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "Adornee").map(|id| Instance(id)) }
			}
			pub fn set_adornee(&self, value: Option<Instance>) {
				unsafe { set_instance_property(self.to_ptr(), "Adornee", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn always_on_top(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AlwaysOnTop") }
			}
			pub fn set_always_on_top(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AlwaysOnTop", value) }
			}
			pub fn brightness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Brightness") }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Brightness", value) }
			}
			pub fn clips_descendants(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ClipsDescendants") }
			}
			pub fn set_clips_descendants(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ClipsDescendants", value) }
			}
			pub fn current_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CurrentDistance") }
			}
			pub fn distance_lower_limit(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DistanceLowerLimit") }
			}
			pub fn set_distance_lower_limit(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DistanceLowerLimit", value) }
			}
			pub fn distance_step(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DistanceStep") }
			}
			pub fn set_distance_step(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DistanceStep", value) }
			}
			pub fn distance_upper_limit(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DistanceUpperLimit") }
			}
			pub fn set_distance_upper_limit(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DistanceUpperLimit", value) }
			}
			pub fn extents_offset(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "ExtentsOffset")) }
			}
			pub fn set_extents_offset(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "ExtentsOffset", value.to_ptr()) }
			}
			pub fn extents_offset_world_space(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "ExtentsOffsetWorldSpace")) }
			}
			pub fn set_extents_offset_world_space(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "ExtentsOffsetWorldSpace", value.to_ptr()) }
			}
			pub fn light_influence(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LightInfluence") }
			}
			pub fn set_light_influence(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LightInfluence", value) }
			}
			pub fn max_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxDistance") }
			}
			pub fn set_max_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxDistance", value) }
			}
			pub fn player_to_hide_from(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "PlayerToHideFrom").map(|id| Instance(id)) }
			}
			pub fn set_player_to_hide_from(&self, value: Option<Instance>) {
				unsafe { set_instance_property(self.to_ptr(), "PlayerToHideFrom", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn size(&self) -> UDim2 {
				unsafe { UDim2(get_datatype_property(self.to_ptr(), "Size")) }
			}
			pub fn set_size(&self, value: UDim2) {
				unsafe { set_datatype_property(self.to_ptr(), "Size", value.to_ptr()) }
			}
			pub fn size_offset(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "SizeOffset")) }
			}
			pub fn set_size_offset(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "SizeOffset", value.to_ptr()) }
			}
			pub fn studs_offset(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "StudsOffset")) }
			}
			pub fn set_studs_offset(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "StudsOffset", value.to_ptr()) }
			}
			pub fn studs_offset_world_space(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "StudsOffsetWorldSpace")) }
			}
			pub fn set_studs_offset_world_space(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "StudsOffsetWorldSpace", value.to_ptr()) }
			}
		}
		impl From<$name> for LayerCollector {
			fn from(value: $name) -> LayerCollector {
				LayerCollector(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_screen_gui {
	($name:ident) => {
		impl_layer_collector!($name);
		impl $name {
			pub fn display_order(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DisplayOrder") }
			}
			pub fn set_display_order(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DisplayOrder", value) }
			}
			pub fn ignore_gui_inset(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IgnoreGuiInset") }
			}
			pub fn set_ignore_gui_inset(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "IgnoreGuiInset", value) }
			}
		}
		impl From<$name> for LayerCollector {
			fn from(value: $name) -> LayerCollector {
				LayerCollector(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_surface_gui {
	($name:ident) => {
		impl_layer_collector!($name);
		impl $name {
			pub fn active(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Active") }
			}
			pub fn set_active(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Active", value) }
			}
			pub fn adornee(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "Adornee").map(|id| Instance(id)) }
			}
			pub fn set_adornee(&self, value: Option<Instance>) {
				unsafe { set_instance_property(self.to_ptr(), "Adornee", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn always_on_top(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AlwaysOnTop") }
			}
			pub fn set_always_on_top(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AlwaysOnTop", value) }
			}
			pub fn brightness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Brightness") }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Brightness", value) }
			}
			pub fn canvas_size(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "CanvasSize")) }
			}
			pub fn set_canvas_size(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "CanvasSize", value.to_ptr()) }
			}
			pub fn clips_descendants(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ClipsDescendants") }
			}
			pub fn set_clips_descendants(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ClipsDescendants", value) }
			}
			pub fn light_influence(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LightInfluence") }
			}
			pub fn set_light_influence(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LightInfluence", value) }
			}
			pub fn pixels_per_stud(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "PixelsPerStud") }
			}
			pub fn set_pixels_per_stud(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "PixelsPerStud", value) }
			}
			pub fn tool_punch_through_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ToolPunchThroughDistance") }
			}
			pub fn set_tool_punch_through_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ToolPunchThroughDistance", value) }
			}
			pub fn z_offset(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ZOffset") }
			}
			pub fn set_z_offset(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ZOffset", value) }
			}
		}
		impl From<$name> for LayerCollector {
			fn from(value: $name) -> LayerCollector {
				LayerCollector(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_gui_base_3_d {
	($name:ident) => {
		impl_gui_base!($name);
		impl $name {
			#[deprecated]
			pub fn color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "Color")) }
			}
			#[deprecated]
			pub fn set_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Color3")) }
			}
			pub fn set_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Color3", value.to_ptr()) }
			}
			pub fn transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Transparency") }
			}
			pub fn set_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Transparency", value) }
			}
			pub fn visible(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Visible") }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Visible", value) }
			}
		}
		impl From<$name> for GuiBase {
			fn from(value: $name) -> GuiBase {
				GuiBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_floor_wire {
	($name:ident) => {
		impl_gui_base_3_d!($name);
		impl $name {
			pub fn cycle_offset(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CycleOffset") }
			}
			pub fn set_cycle_offset(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CycleOffset", value) }
			}
			pub fn from(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "From").map(|id| BasePart(id)) }
			}
			pub fn set_from(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "From", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn studs_between_textures(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "StudsBetweenTextures") }
			}
			pub fn set_studs_between_textures(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "StudsBetweenTextures", value) }
			}
			pub fn texture(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "Texture")) }
			}
			pub fn set_texture(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "Texture", value.to_ptr()) }
			}
			pub fn texture_size(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "TextureSize")) }
			}
			pub fn set_texture_size(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "TextureSize", value.to_ptr()) }
			}
			pub fn to(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "To").map(|id| BasePart(id)) }
			}
			pub fn set_to(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "To", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn velocity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Velocity") }
			}
			pub fn set_velocity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Velocity", value) }
			}
			pub fn wire_radius(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WireRadius") }
			}
			pub fn set_wire_radius(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WireRadius", value) }
			}
		}
		impl From<$name> for GuiBase3d {
			fn from(value: $name) -> GuiBase3d {
				GuiBase3d(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_instance_adornment {
	($name:ident) => {
		impl_gui_base_3_d!($name);
		impl $name {
			pub fn adornee(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "Adornee").map(|id| Instance(id)) }
			}
			pub fn set_adornee(&self, value: Option<Instance>) {
				unsafe { set_instance_property(self.to_ptr(), "Adornee", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
		}
		impl From<$name> for GuiBase3d {
			fn from(value: $name) -> GuiBase3d {
				GuiBase3d(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_selection_box {
	($name:ident) => {
		impl_instance_adornment!($name);
		impl $name {
			pub fn line_thickness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LineThickness") }
			}
			pub fn set_line_thickness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LineThickness", value) }
			}
			#[deprecated]
			pub fn surface_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "SurfaceColor")) }
			}
			#[deprecated]
			pub fn set_surface_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "SurfaceColor", value.to_ptr()) }
			}
			pub fn surface_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "SurfaceColor3")) }
			}
			pub fn set_surface_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "SurfaceColor3", value.to_ptr()) }
			}
			pub fn surface_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "SurfaceTransparency") }
			}
			pub fn set_surface_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "SurfaceTransparency", value) }
			}
		}
		impl From<$name> for InstanceAdornment {
			fn from(value: $name) -> InstanceAdornment {
				InstanceAdornment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pv_adornment {
	($name:ident) => {
		impl_gui_base_3_d!($name);
		impl $name {
			pub fn adornee(&self) -> Option<PVInstance> {
				unsafe { get_instance_property(self.to_ptr(), "Adornee").map(|id| PVInstance(id)) }
			}
			pub fn set_adornee(&self, value: Option<PVInstance>) {
				unsafe { set_instance_property(self.to_ptr(), "Adornee", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
		}
		impl From<$name> for GuiBase3d {
			fn from(value: $name) -> GuiBase3d {
				GuiBase3d(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_handle_adornment {
	($name:ident) => {
		impl_pv_adornment!($name);
		impl $name {
			pub fn always_on_top(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AlwaysOnTop") }
			}
			pub fn set_always_on_top(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AlwaysOnTop", value) }
			}
			pub fn c_frame(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "CFrame")) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "CFrame", value.to_ptr()) }
			}
			pub fn size_relative_offset(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "SizeRelativeOffset")) }
			}
			pub fn set_size_relative_offset(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "SizeRelativeOffset", value.to_ptr()) }
			}
			pub fn z_index(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ZIndex") }
			}
			pub fn set_z_index(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ZIndex", value) }
			}
		}
		impl From<$name> for PVAdornment {
			fn from(value: $name) -> PVAdornment {
				PVAdornment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_box_handle_adornment {
	($name:ident) => {
		impl_handle_adornment!($name);
		impl $name {
			pub fn size(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Size")) }
			}
			pub fn set_size(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Size", value.to_ptr()) }
			}
		}
		impl From<$name> for HandleAdornment {
			fn from(value: $name) -> HandleAdornment {
				HandleAdornment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_cone_handle_adornment {
	($name:ident) => {
		impl_handle_adornment!($name);
		impl $name {
			pub fn height(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Height") }
			}
			pub fn set_height(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Height", value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Radius") }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Radius", value) }
			}
		}
		impl From<$name> for HandleAdornment {
			fn from(value: $name) -> HandleAdornment {
				HandleAdornment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_cylinder_handle_adornment {
	($name:ident) => {
		impl_handle_adornment!($name);
		impl $name {
			pub fn angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Angle") }
			}
			pub fn set_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Angle", value) }
			}
			pub fn height(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Height") }
			}
			pub fn set_height(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Height", value) }
			}
			pub fn inner_radius(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "InnerRadius") }
			}
			pub fn set_inner_radius(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "InnerRadius", value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Radius") }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Radius", value) }
			}
		}
		impl From<$name> for HandleAdornment {
			fn from(value: $name) -> HandleAdornment {
				HandleAdornment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_image_handle_adornment {
	($name:ident) => {
		impl_handle_adornment!($name);
		impl $name {
			pub fn image(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "Image")) }
			}
			pub fn set_image(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "Image", value.to_ptr()) }
			}
			pub fn size(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "Size")) }
			}
			pub fn set_size(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "Size", value.to_ptr()) }
			}
		}
		impl From<$name> for HandleAdornment {
			fn from(value: $name) -> HandleAdornment {
				HandleAdornment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_line_handle_adornment {
	($name:ident) => {
		impl_handle_adornment!($name);
		impl $name {
			pub fn length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Length") }
			}
			pub fn set_length(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Length", value) }
			}
			pub fn thickness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Thickness") }
			}
			pub fn set_thickness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Thickness", value) }
			}
		}
		impl From<$name> for HandleAdornment {
			fn from(value: $name) -> HandleAdornment {
				HandleAdornment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_sphere_handle_adornment {
	($name:ident) => {
		impl_handle_adornment!($name);
		impl $name {
			pub fn radius(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Radius") }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Radius", value) }
			}
		}
		impl From<$name> for HandleAdornment {
			fn from(value: $name) -> HandleAdornment {
				HandleAdornment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_parabola_adornment {
	($name:ident) => {
		impl_pv_adornment!($name);
		impl $name {
		}
		impl From<$name> for PVAdornment {
			fn from(value: $name) -> PVAdornment {
				PVAdornment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_selection_sphere {
	($name:ident) => {
		impl_pv_adornment!($name);
		impl $name {
			#[deprecated]
			pub fn surface_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "SurfaceColor")) }
			}
			#[deprecated]
			pub fn set_surface_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "SurfaceColor", value.to_ptr()) }
			}
			pub fn surface_color_3(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "SurfaceColor3")) }
			}
			pub fn set_surface_color_3(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "SurfaceColor3", value.to_ptr()) }
			}
			pub fn surface_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "SurfaceTransparency") }
			}
			pub fn set_surface_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "SurfaceTransparency", value) }
			}
		}
		impl From<$name> for PVAdornment {
			fn from(value: $name) -> PVAdornment {
				PVAdornment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_part_adornment {
	($name:ident) => {
		impl_gui_base_3_d!($name);
		impl $name {
			pub fn adornee(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "Adornee").map(|id| BasePart(id)) }
			}
			pub fn set_adornee(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "Adornee", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
		}
		impl From<$name> for GuiBase3d {
			fn from(value: $name) -> GuiBase3d {
				GuiBase3d(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_handles_base {
	($name:ident) => {
		impl_part_adornment!($name);
		impl $name {
		}
		impl From<$name> for PartAdornment {
			fn from(value: $name) -> PartAdornment {
				PartAdornment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_arc_handles {
	($name:ident) => {
		impl_handles_base!($name);
		impl $name {
			pub fn axes(&self) -> Axes {
				unsafe { Axes(get_datatype_property(self.to_ptr(), "Axes")) }
			}
			pub fn set_axes(&self, value: Axes) {
				unsafe { set_datatype_property(self.to_ptr(), "Axes", value.to_ptr()) }
			}
		}
		impl From<$name> for HandlesBase {
			fn from(value: $name) -> HandlesBase {
				HandlesBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_handles {
	($name:ident) => {
		impl_handles_base!($name);
		impl $name {
			pub fn faces(&self) -> Faces {
				unsafe { Faces(get_datatype_property(self.to_ptr(), "Faces")) }
			}
			pub fn set_faces(&self, value: Faces) {
				unsafe { set_datatype_property(self.to_ptr(), "Faces", value.to_ptr()) }
			}
		}
		impl From<$name> for HandlesBase {
			fn from(value: $name) -> HandlesBase {
				HandlesBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_surface_selection {
	($name:ident) => {
		impl_part_adornment!($name);
		impl $name {
		}
		impl From<$name> for PartAdornment {
			fn from(value: $name) -> PartAdornment {
				PartAdornment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_selection_lasso {
	($name:ident) => {
		impl_gui_base_3_d!($name);
		impl $name {
			pub fn humanoid(&self) -> Option<Humanoid> {
				unsafe { get_instance_property(self.to_ptr(), "Humanoid").map(|id| Humanoid(id)) }
			}
			pub fn set_humanoid(&self, value: Option<Humanoid>) {
				unsafe { set_instance_property(self.to_ptr(), "Humanoid", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
		}
		impl From<$name> for GuiBase3d {
			fn from(value: $name) -> GuiBase3d {
				GuiBase3d(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_selection_part_lasso {
	($name:ident) => {
		impl_selection_lasso!($name);
		impl $name {
			pub fn part(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "Part").map(|id| BasePart(id)) }
			}
			pub fn set_part(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "Part", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
		}
		impl From<$name> for SelectionLasso {
			fn from(value: $name) -> SelectionLasso {
				SelectionLasso(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_selection_point_lasso {
	($name:ident) => {
		impl_selection_lasso!($name);
		impl $name {
			pub fn point(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Point")) }
			}
			pub fn set_point(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Point", value.to_ptr()) }
			}
		}
		impl From<$name> for SelectionLasso {
			fn from(value: $name) -> SelectionLasso {
				SelectionLasso(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_highlight {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn adornee(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "Adornee").map(|id| Instance(id)) }
			}
			pub fn set_adornee(&self, value: Option<Instance>) {
				unsafe { set_instance_property(self.to_ptr(), "Adornee", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn fill_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "FillColor")) }
			}
			pub fn set_fill_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "FillColor", value.to_ptr()) }
			}
			pub fn fill_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "FillTransparency") }
			}
			pub fn set_fill_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "FillTransparency", value) }
			}
			pub fn outline_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "OutlineColor")) }
			}
			pub fn set_outline_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "OutlineColor", value.to_ptr()) }
			}
			pub fn outline_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "OutlineTransparency") }
			}
			pub fn set_outline_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "OutlineTransparency", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_humanoid {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn auto_jump_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AutoJumpEnabled") }
			}
			pub fn set_auto_jump_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AutoJumpEnabled", value) }
			}
			pub fn auto_rotate(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AutoRotate") }
			}
			pub fn set_auto_rotate(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AutoRotate", value) }
			}
			pub fn automatic_scaling_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AutomaticScalingEnabled") }
			}
			pub fn set_automatic_scaling_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AutomaticScalingEnabled", value) }
			}
			pub fn break_joints_on_death(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "BreakJointsOnDeath") }
			}
			pub fn set_break_joints_on_death(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "BreakJointsOnDeath", value) }
			}
			pub fn camera_offset(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "CameraOffset")) }
			}
			pub fn set_camera_offset(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "CameraOffset", value.to_ptr()) }
			}
			pub fn display_name(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "DisplayName") }
			}
			pub fn set_display_name(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "DisplayName", &value) }
			}
			pub fn health(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Health") }
			}
			pub fn set_health(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Health", value) }
			}
			pub fn health_display_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "HealthDisplayDistance") }
			}
			pub fn set_health_display_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "HealthDisplayDistance", value) }
			}
			pub fn hip_height(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "HipHeight") }
			}
			pub fn set_hip_height(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "HipHeight", value) }
			}
			pub fn jump(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Jump") }
			}
			pub fn set_jump(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Jump", value) }
			}
			pub fn jump_height(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "JumpHeight") }
			}
			pub fn set_jump_height(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "JumpHeight", value) }
			}
			pub fn jump_power(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "JumpPower") }
			}
			pub fn set_jump_power(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "JumpPower", value) }
			}
			#[deprecated]
			pub fn left_leg(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "LeftLeg").map(|id| BasePart(id)) }
			}
			#[deprecated]
			pub fn set_left_leg(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "LeftLeg", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn max_health(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxHealth") }
			}
			pub fn set_max_health(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxHealth", value) }
			}
			pub fn max_slope_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxSlopeAngle") }
			}
			pub fn set_max_slope_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxSlopeAngle", value) }
			}
			pub fn move_direction(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "MoveDirection")) }
			}
			pub fn name_display_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "NameDisplayDistance") }
			}
			pub fn set_name_display_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "NameDisplayDistance", value) }
			}
			pub fn platform_stand(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "PlatformStand") }
			}
			pub fn set_platform_stand(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "PlatformStand", value) }
			}
			pub fn requires_neck(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "RequiresNeck") }
			}
			pub fn set_requires_neck(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "RequiresNeck", value) }
			}
			#[deprecated]
			pub fn right_leg(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "RightLeg").map(|id| BasePart(id)) }
			}
			#[deprecated]
			pub fn set_right_leg(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "RightLeg", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn root_part(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "RootPart").map(|id| BasePart(id)) }
			}
			pub fn seat_part(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "SeatPart").map(|id| BasePart(id)) }
			}
			pub fn sit(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Sit") }
			}
			pub fn set_sit(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Sit", value) }
			}
			pub fn target_point(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "TargetPoint")) }
			}
			pub fn set_target_point(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "TargetPoint", value.to_ptr()) }
			}
			#[deprecated]
			pub fn torso(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "Torso").map(|id| BasePart(id)) }
			}
			#[deprecated]
			pub fn set_torso(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "Torso", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn use_jump_power(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "UseJumpPower") }
			}
			pub fn set_use_jump_power(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "UseJumpPower", value) }
			}
			pub fn walk_speed(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WalkSpeed") }
			}
			pub fn set_walk_speed(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WalkSpeed", value) }
			}
			pub fn walk_to_part(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "WalkToPart").map(|id| BasePart(id)) }
			}
			pub fn set_walk_to_part(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "WalkToPart", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn walk_to_point(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "WalkToPoint")) }
			}
			pub fn set_walk_to_point(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "WalkToPoint", value.to_ptr()) }
			}
			pub fn fn_add_accessory(&self, accessory: Option<Instance>) {
				unsafe { dyn_humanoid_add_accessory(self.to_ptr(), "AddAccessory", accessory) }
			}
			pub fn fn_build_rig_from_attachments(&self) {
				unsafe { dyn_humanoid_build_rig_from_attachments(self.to_ptr(), "BuildRigFromAttachments") }
			}
			pub fn fn_equip_tool(&self, tool: Option<Instance>) {
				unsafe { dyn_humanoid_equip_tool(self.to_ptr(), "EquipTool", tool) }
			}
			pub fn fn_get_accessories(&self) {
				unsafe { dyn_humanoid_get_accessories(self.to_ptr(), "GetAccessories") }
			}
			pub fn fn_get_applied_description(&self) -> Option<HumanoidDescription> {
				unsafe { dyn_humanoid_get_applied_description(self.to_ptr(), "GetAppliedDescription") }
			}
			pub fn fn_get_body_part_r_15(&self, part: Option<Instance>) {
				unsafe { dyn_humanoid_get_body_part_r_15(self.to_ptr(), "GetBodyPartR15", part) }
			}
			pub fn fn_get_limb(&self, part: Option<Instance>) {
				unsafe { dyn_humanoid_get_limb(self.to_ptr(), "GetLimb", part) }
			}
			pub fn fn_get_state(&self) {
				unsafe { dyn_humanoid_get_state(self.to_ptr(), "GetState") }
			}
			pub fn fn_move(&self, move_direction: Vector3, relative_to_camera: bool) {
				unsafe { dyn_humanoid_move(self.to_ptr(), "Move", move_direction, relative_to_camera) }
			}
			pub fn fn_move_to(&self, location: Vector3, part: Option<Instance>) {
				unsafe { dyn_humanoid_move_to(self.to_ptr(), "MoveTo", location, part) }
			}
			pub fn fn_remove_accessories(&self) {
				unsafe { dyn_humanoid_remove_accessories(self.to_ptr(), "RemoveAccessories") }
			}
			pub fn fn_take_damage(&self, amount: f64) {
				unsafe { dyn_humanoid_take_damage(self.to_ptr(), "TakeDamage", amount) }
			}
			pub fn fn_unequip_tools(&self) {
				unsafe { dyn_humanoid_unequip_tools(self.to_ptr(), "UnequipTools") }
			}
			pub fn fn_play_emote(&self, emote_name: &str) -> bool {
				unsafe { dyn_humanoid_play_emote(self.to_ptr(), "PlayEmote", emote_name) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_humanoid_description {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn back_accessory(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "BackAccessory") }
			}
			pub fn set_back_accessory(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "BackAccessory", &value) }
			}
			pub fn body_type_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "BodyTypeScale") }
			}
			pub fn set_body_type_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "BodyTypeScale", value) }
			}
			pub fn climb_animation(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ClimbAnimation") }
			}
			pub fn set_climb_animation(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ClimbAnimation", value) }
			}
			pub fn depth_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DepthScale") }
			}
			pub fn set_depth_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DepthScale", value) }
			}
			pub fn face(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Face") }
			}
			pub fn set_face(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Face", value) }
			}
			pub fn face_accessory(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "FaceAccessory") }
			}
			pub fn set_face_accessory(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "FaceAccessory", &value) }
			}
			pub fn fall_animation(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "FallAnimation") }
			}
			pub fn set_fall_animation(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "FallAnimation", value) }
			}
			pub fn front_accessory(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "FrontAccessory") }
			}
			pub fn set_front_accessory(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "FrontAccessory", &value) }
			}
			pub fn graphic_t_shirt(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "GraphicTShirt") }
			}
			pub fn set_graphic_t_shirt(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "GraphicTShirt", value) }
			}
			pub fn hair_accessory(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "HairAccessory") }
			}
			pub fn set_hair_accessory(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "HairAccessory", &value) }
			}
			pub fn hat_accessory(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "HatAccessory") }
			}
			pub fn set_hat_accessory(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "HatAccessory", &value) }
			}
			pub fn head(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Head") }
			}
			pub fn set_head(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Head", value) }
			}
			pub fn head_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "HeadColor")) }
			}
			pub fn set_head_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "HeadColor", value.to_ptr()) }
			}
			pub fn head_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "HeadScale") }
			}
			pub fn set_head_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "HeadScale", value) }
			}
			pub fn height_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "HeightScale") }
			}
			pub fn set_height_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "HeightScale", value) }
			}
			pub fn idle_animation(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "IdleAnimation") }
			}
			pub fn set_idle_animation(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "IdleAnimation", value) }
			}
			pub fn jump_animation(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "JumpAnimation") }
			}
			pub fn set_jump_animation(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "JumpAnimation", value) }
			}
			pub fn left_arm(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LeftArm") }
			}
			pub fn set_left_arm(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LeftArm", value) }
			}
			pub fn left_arm_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "LeftArmColor")) }
			}
			pub fn set_left_arm_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "LeftArmColor", value.to_ptr()) }
			}
			pub fn left_leg(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LeftLeg") }
			}
			pub fn set_left_leg(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LeftLeg", value) }
			}
			pub fn left_leg_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "LeftLegColor")) }
			}
			pub fn set_left_leg_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "LeftLegColor", value.to_ptr()) }
			}
			pub fn neck_accessory(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "NeckAccessory") }
			}
			pub fn set_neck_accessory(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "NeckAccessory", &value) }
			}
			pub fn pants(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Pants") }
			}
			pub fn set_pants(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Pants", value) }
			}
			pub fn proportion_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ProportionScale") }
			}
			pub fn set_proportion_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ProportionScale", value) }
			}
			pub fn right_arm(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "RightArm") }
			}
			pub fn set_right_arm(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "RightArm", value) }
			}
			pub fn right_arm_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "RightArmColor")) }
			}
			pub fn set_right_arm_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "RightArmColor", value.to_ptr()) }
			}
			pub fn right_leg(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "RightLeg") }
			}
			pub fn set_right_leg(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "RightLeg", value) }
			}
			pub fn right_leg_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "RightLegColor")) }
			}
			pub fn set_right_leg_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "RightLegColor", value.to_ptr()) }
			}
			pub fn run_animation(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "RunAnimation") }
			}
			pub fn set_run_animation(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "RunAnimation", value) }
			}
			pub fn shirt(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Shirt") }
			}
			pub fn set_shirt(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Shirt", value) }
			}
			pub fn shoulders_accessory(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "ShouldersAccessory") }
			}
			pub fn set_shoulders_accessory(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "ShouldersAccessory", &value) }
			}
			pub fn swim_animation(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "SwimAnimation") }
			}
			pub fn set_swim_animation(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "SwimAnimation", value) }
			}
			pub fn torso(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Torso") }
			}
			pub fn set_torso(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Torso", value) }
			}
			pub fn torso_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "TorsoColor")) }
			}
			pub fn set_torso_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "TorsoColor", value.to_ptr()) }
			}
			pub fn waist_accessory(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "WaistAccessory") }
			}
			pub fn set_waist_accessory(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "WaistAccessory", &value) }
			}
			pub fn walk_animation(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WalkAnimation") }
			}
			pub fn set_walk_animation(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WalkAnimation", value) }
			}
			pub fn width_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WidthScale") }
			}
			pub fn set_width_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WidthScale", value) }
			}
			pub fn fn_add_emote(&self, name: &str, asset_id: f64) {
				unsafe { dyn_humanoid_description_add_emote(self.to_ptr(), "AddEmote", name, asset_id) }
			}
			pub fn fn_get_accessories(&self, include_rigid_accessories: bool) {
				unsafe { dyn_humanoid_description_get_accessories(self.to_ptr(), "GetAccessories", include_rigid_accessories) }
			}
			pub fn fn_get_emotes(&self) {
				unsafe { dyn_humanoid_description_get_emotes(self.to_ptr(), "GetEmotes") }
			}
			pub fn fn_get_equipped_emotes(&self) {
				unsafe { dyn_humanoid_description_get_equipped_emotes(self.to_ptr(), "GetEquippedEmotes") }
			}
			pub fn fn_remove_emote(&self, name: &str) {
				unsafe { dyn_humanoid_description_remove_emote(self.to_ptr(), "RemoveEmote", name) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_importer_base_settings {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn id(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Id") }
			}
			pub fn import_name(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "ImportName") }
			}
			pub fn set_import_name(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "ImportName", &value) }
			}
			pub fn should_import(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ShouldImport") }
			}
			pub fn set_should_import(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ShouldImport", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_importer_group_settings {
	($name:ident) => {
		impl_importer_base_settings!($name);
		impl $name {
			pub fn anchored(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Anchored") }
			}
			pub fn set_anchored(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Anchored", value) }
			}
			pub fn import_as_model_asset(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ImportAsModelAsset") }
			}
			pub fn set_import_as_model_asset(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ImportAsModelAsset", value) }
			}
			pub fn insert_in_workspace(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "InsertInWorkspace") }
			}
			pub fn set_insert_in_workspace(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "InsertInWorkspace", value) }
			}
		}
		impl From<$name> for ImporterBaseSettings {
			fn from(value: $name) -> ImporterBaseSettings {
				ImporterBaseSettings(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_importer_material_settings {
	($name:ident) => {
		impl_importer_base_settings!($name);
		impl $name {
			pub fn diffuse_file_path(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "DiffuseFilePath") }
			}
			pub fn is_pbr(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsPbr") }
			}
			pub fn metalness_file_path(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "MetalnessFilePath") }
			}
			pub fn normal_file_path(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "NormalFilePath") }
			}
			pub fn roughness_file_path(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "RoughnessFilePath") }
			}
		}
		impl From<$name> for ImporterBaseSettings {
			fn from(value: $name) -> ImporterBaseSettings {
				ImporterBaseSettings(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_importer_mesh_settings {
	($name:ident) => {
		impl_importer_base_settings!($name);
		impl $name {
			pub fn anchored(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Anchored") }
			}
			pub fn set_anchored(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Anchored", value) }
			}
			pub fn cage_manifold(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CageManifold") }
			}
			pub fn cage_manifold_preview(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CageManifoldPreview") }
			}
			pub fn set_cage_manifold_preview(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "CageManifoldPreview", value) }
			}
			pub fn cage_no_overlapping_vertices(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CageNoOverlappingVertices") }
			}
			pub fn cage_no_overlapping_vertices_preview(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CageNoOverlappingVerticesPreview") }
			}
			pub fn set_cage_no_overlapping_vertices_preview(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "CageNoOverlappingVerticesPreview", value) }
			}
			pub fn cage_uv_matched(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CageUVMatched") }
			}
			pub fn cage_uv_matched_preview(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CageUVMatchedPreview") }
			}
			pub fn set_cage_uv_matched_preview(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "CageUVMatchedPreview", value) }
			}
			pub fn dimensions(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Dimensions")) }
			}
			pub fn double_sided(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "DoubleSided") }
			}
			pub fn set_double_sided(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "DoubleSided", value) }
			}
			pub fn ignore_vertex_colors(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IgnoreVertexColors") }
			}
			pub fn set_ignore_vertex_colors(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "IgnoreVertexColors", value) }
			}
			pub fn polygon_count(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "PolygonCount") }
			}
			pub fn use_imported_pivot(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "UseImportedPivot") }
			}
			pub fn set_use_imported_pivot(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "UseImportedPivot", value) }
			}
		}
		impl From<$name> for ImporterBaseSettings {
			fn from(value: $name) -> ImporterBaseSettings {
				ImporterBaseSettings(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_importer_root_settings {
	($name:ident) => {
		impl_importer_base_settings!($name);
		impl $name {
			pub fn anchored(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Anchored") }
			}
			pub fn set_anchored(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Anchored", value) }
			}
			pub fn file_dimensions(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "FileDimensions")) }
			}
			pub fn import_as_model_asset(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ImportAsModelAsset") }
			}
			pub fn set_import_as_model_asset(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ImportAsModelAsset", value) }
			}
			pub fn insert_in_workspace(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "InsertInWorkspace") }
			}
			pub fn set_insert_in_workspace(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "InsertInWorkspace", value) }
			}
			pub fn insert_with_scene_position(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "InsertWithScenePosition") }
			}
			pub fn set_insert_with_scene_position(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "InsertWithScenePosition", value) }
			}
			pub fn invert_negative_faces(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "InvertNegativeFaces") }
			}
			pub fn set_invert_negative_faces(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "InvertNegativeFaces", value) }
			}
			pub fn merge_meshes(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "MergeMeshes") }
			}
			pub fn set_merge_meshes(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "MergeMeshes", value) }
			}
			pub fn polygon_count(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "PolygonCount") }
			}
			pub fn use_scene_origin_as_pivot(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "UseSceneOriginAsPivot") }
			}
			pub fn set_use_scene_origin_as_pivot(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "UseSceneOriginAsPivot", value) }
			}
		}
		impl From<$name> for ImporterBaseSettings {
			fn from(value: $name) -> ImporterBaseSettings {
				ImporterBaseSettings(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_input_object {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn delta(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Delta")) }
			}
			pub fn set_delta(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Delta", value.to_ptr()) }
			}
			pub fn position(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Position")) }
			}
			pub fn set_position(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Position", value.to_ptr()) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_joint_instance {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn active(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Active") }
			}
			pub fn c_0(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "C0")) }
			}
			pub fn set_c_0(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "C0", value.to_ptr()) }
			}
			pub fn c_1(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "C1")) }
			}
			pub fn set_c_1(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "C1", value.to_ptr()) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn part_0(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "Part0").map(|id| BasePart(id)) }
			}
			pub fn set_part_0(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "Part0", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn part_1(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "Part1").map(|id| BasePart(id)) }
			}
			pub fn set_part_1(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "Part1", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_dynamic_rotate {
	($name:ident) => {
		impl_joint_instance!($name);
		impl $name {
			pub fn base_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "BaseAngle") }
			}
			pub fn set_base_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "BaseAngle", value) }
			}
		}
		impl From<$name> for JointInstance {
			fn from(value: $name) -> JointInstance {
				JointInstance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_glue {
	($name:ident) => {
		impl_joint_instance!($name);
		impl $name {
			pub fn f_0(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "F0")) }
			}
			pub fn set_f_0(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "F0", value.to_ptr()) }
			}
			pub fn f_1(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "F1")) }
			}
			pub fn set_f_1(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "F1", value.to_ptr()) }
			}
			pub fn f_2(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "F2")) }
			}
			pub fn set_f_2(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "F2", value.to_ptr()) }
			}
			pub fn f_3(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "F3")) }
			}
			pub fn set_f_3(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "F3", value.to_ptr()) }
			}
		}
		impl From<$name> for JointInstance {
			fn from(value: $name) -> JointInstance {
				JointInstance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_motor {
	($name:ident) => {
		impl_joint_instance!($name);
		impl $name {
			pub fn current_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CurrentAngle") }
			}
			pub fn set_current_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CurrentAngle", value) }
			}
			pub fn desired_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DesiredAngle") }
			}
			pub fn set_desired_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DesiredAngle", value) }
			}
			pub fn max_velocity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxVelocity") }
			}
			pub fn set_max_velocity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxVelocity", value) }
			}
			pub fn fn_set_desired_angle(&self, value: f64) {
				unsafe { dyn_motor_set_desired_angle(self.to_ptr(), "SetDesiredAngle", value) }
			}
		}
		impl From<$name> for JointInstance {
			fn from(value: $name) -> JointInstance {
				JointInstance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_motor_6_d {
	($name:ident) => {
		impl_motor!($name);
		impl $name {
			pub fn transform(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "Transform")) }
			}
			pub fn set_transform(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "Transform", value.to_ptr()) }
			}
		}
		impl From<$name> for Motor {
			fn from(value: $name) -> Motor {
				Motor(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_velocity_motor {
	($name:ident) => {
		impl_joint_instance!($name);
		impl $name {
			pub fn current_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CurrentAngle") }
			}
			pub fn set_current_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CurrentAngle", value) }
			}
			pub fn desired_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DesiredAngle") }
			}
			pub fn set_desired_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DesiredAngle", value) }
			}
			pub fn max_velocity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxVelocity") }
			}
			pub fn set_max_velocity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxVelocity", value) }
			}
		}
		impl From<$name> for JointInstance {
			fn from(value: $name) -> JointInstance {
				JointInstance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_keyframe {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn time(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Time") }
			}
			pub fn set_time(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Time", value) }
			}
			pub fn fn_add_marker(&self, marker: Option<Instance>) {
				unsafe { dyn_keyframe_add_marker(self.to_ptr(), "AddMarker", marker) }
			}
			pub fn fn_add_pose(&self, pose: Option<Instance>) {
				unsafe { dyn_keyframe_add_pose(self.to_ptr(), "AddPose", pose) }
			}
			pub fn fn_get_markers(&self) -> Objects {
				unsafe { dyn_keyframe_get_markers(self.to_ptr(), "GetMarkers") }
			}
			pub fn fn_get_poses(&self) -> Objects {
				unsafe { dyn_keyframe_get_poses(self.to_ptr(), "GetPoses") }
			}
			pub fn fn_remove_marker(&self, marker: Option<Instance>) {
				unsafe { dyn_keyframe_remove_marker(self.to_ptr(), "RemoveMarker", marker) }
			}
			pub fn fn_remove_pose(&self, pose: Option<Instance>) {
				unsafe { dyn_keyframe_remove_pose(self.to_ptr(), "RemovePose", pose) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_keyframe_marker {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn value(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Value") }
			}
			pub fn set_value(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "Value", &value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_light {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn brightness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Brightness") }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Brightness", value) }
			}
			pub fn color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Color")) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn shadows(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Shadows") }
			}
			pub fn set_shadows(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Shadows", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_point_light {
	($name:ident) => {
		impl_light!($name);
		impl $name {
			pub fn range(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Range") }
			}
			pub fn set_range(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Range", value) }
			}
		}
		impl From<$name> for Light {
			fn from(value: $name) -> Light {
				Light(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_spot_light {
	($name:ident) => {
		impl_light!($name);
		impl $name {
			pub fn angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Angle") }
			}
			pub fn set_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Angle", value) }
			}
			pub fn range(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Range") }
			}
			pub fn set_range(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Range", value) }
			}
		}
		impl From<$name> for Light {
			fn from(value: $name) -> Light {
				Light(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_surface_light {
	($name:ident) => {
		impl_light!($name);
		impl $name {
			pub fn angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Angle") }
			}
			pub fn set_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Angle", value) }
			}
			pub fn range(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Range") }
			}
			pub fn set_range(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Range", value) }
			}
		}
		impl From<$name> for Light {
			fn from(value: $name) -> Light {
				Light(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_lighting {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn ambient(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Ambient")) }
			}
			pub fn set_ambient(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Ambient", value.to_ptr()) }
			}
			pub fn brightness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Brightness") }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Brightness", value) }
			}
			pub fn clock_time(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ClockTime") }
			}
			pub fn set_clock_time(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ClockTime", value) }
			}
			pub fn color_shift_bottom(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "ColorShift_Bottom")) }
			}
			pub fn set_color_shift_bottom(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "ColorShift_Bottom", value.to_ptr()) }
			}
			pub fn color_shift_top(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "ColorShift_Top")) }
			}
			pub fn set_color_shift_top(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "ColorShift_Top", value.to_ptr()) }
			}
			pub fn environment_diffuse_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "EnvironmentDiffuseScale") }
			}
			pub fn set_environment_diffuse_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "EnvironmentDiffuseScale", value) }
			}
			pub fn environment_specular_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "EnvironmentSpecularScale") }
			}
			pub fn set_environment_specular_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "EnvironmentSpecularScale", value) }
			}
			pub fn exposure_compensation(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ExposureCompensation") }
			}
			pub fn set_exposure_compensation(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ExposureCompensation", value) }
			}
			pub fn fog_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "FogColor")) }
			}
			pub fn set_fog_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "FogColor", value.to_ptr()) }
			}
			pub fn fog_end(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "FogEnd") }
			}
			pub fn set_fog_end(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "FogEnd", value) }
			}
			pub fn fog_start(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "FogStart") }
			}
			pub fn set_fog_start(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "FogStart", value) }
			}
			pub fn geographic_latitude(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "GeographicLatitude") }
			}
			pub fn set_geographic_latitude(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "GeographicLatitude", value) }
			}
			pub fn global_shadows(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "GlobalShadows") }
			}
			pub fn set_global_shadows(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "GlobalShadows", value) }
			}
			pub fn outdoor_ambient(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "OutdoorAmbient")) }
			}
			pub fn set_outdoor_ambient(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "OutdoorAmbient", value.to_ptr()) }
			}
			#[deprecated]
			pub fn outlines(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Outlines") }
			}
			#[deprecated]
			pub fn set_outlines(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Outlines", value) }
			}
			#[deprecated]
			pub fn shadow_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "ShadowColor")) }
			}
			#[deprecated]
			pub fn set_shadow_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "ShadowColor", value.to_ptr()) }
			}
			pub fn shadow_softness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ShadowSoftness") }
			}
			pub fn set_shadow_softness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ShadowSoftness", value) }
			}
			pub fn time_of_day(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "TimeOfDay") }
			}
			pub fn set_time_of_day(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "TimeOfDay", &value) }
			}
			pub fn fn_get_minutes_after_midnight(&self) -> f64 {
				unsafe { dyn_lighting_get_minutes_after_midnight(self.to_ptr(), "GetMinutesAfterMidnight") }
			}
			pub fn fn_get_moon_direction(&self) -> Vector3 {
				unsafe { dyn_lighting_get_moon_direction(self.to_ptr(), "GetMoonDirection") }
			}
			pub fn fn_get_moon_phase(&self) -> f64 {
				unsafe { dyn_lighting_get_moon_phase(self.to_ptr(), "GetMoonPhase") }
			}
			pub fn fn_get_sun_direction(&self) -> Vector3 {
				unsafe { dyn_lighting_get_sun_direction(self.to_ptr(), "GetSunDirection") }
			}
			pub fn fn_set_minutes_after_midnight(&self, minutes: f64) {
				unsafe { dyn_lighting_set_minutes_after_midnight(self.to_ptr(), "SetMinutesAfterMidnight", minutes) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_localization_table {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			#[deprecated]
			pub fn development_language(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "DevelopmentLanguage") }
			}
			#[deprecated]
			pub fn set_development_language(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "DevelopmentLanguage", &value) }
			}
			#[deprecated]
			pub fn root(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "Root").map(|id| Instance(id)) }
			}
			#[deprecated]
			pub fn set_root(&self, value: Option<Instance>) {
				unsafe { set_instance_property(self.to_ptr(), "Root", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn source_locale_id(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "SourceLocaleId") }
			}
			pub fn set_source_locale_id(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "SourceLocaleId", &value) }
			}
			pub fn fn_get_entries(&self) {
				unsafe { dyn_localization_table_get_entries(self.to_ptr(), "GetEntries") }
			}
			pub fn fn_get_translator(&self, locale_id: &str) -> Option<Instance> {
				unsafe { dyn_localization_table_get_translator(self.to_ptr(), "GetTranslator", locale_id) }
			}
			pub fn fn_remove_entry(&self, key: &str, source: &str, context: &str) {
				unsafe { dyn_localization_table_remove_entry(self.to_ptr(), "RemoveEntry", key, source, context) }
			}
			pub fn fn_remove_entry_value(&self, key: &str, source: &str, context: &str, locale_id: &str) {
				unsafe { dyn_localization_table_remove_entry_value(self.to_ptr(), "RemoveEntryValue", key, source, context, locale_id) }
			}
			pub fn fn_remove_target_locale(&self, locale_id: &str) {
				unsafe { dyn_localization_table_remove_target_locale(self.to_ptr(), "RemoveTargetLocale", locale_id) }
			}
			pub fn fn_set_entry_context(&self, key: &str, source: &str, context: &str, new_context: &str) {
				unsafe { dyn_localization_table_set_entry_context(self.to_ptr(), "SetEntryContext", key, source, context, new_context) }
			}
			pub fn fn_set_entry_example(&self, key: &str, source: &str, context: &str, example: &str) {
				unsafe { dyn_localization_table_set_entry_example(self.to_ptr(), "SetEntryExample", key, source, context, example) }
			}
			pub fn fn_set_entry_key(&self, key: &str, source: &str, context: &str, new_key: &str) {
				unsafe { dyn_localization_table_set_entry_key(self.to_ptr(), "SetEntryKey", key, source, context, new_key) }
			}
			pub fn fn_set_entry_source(&self, key: &str, source: &str, context: &str, new_source: &str) {
				unsafe { dyn_localization_table_set_entry_source(self.to_ptr(), "SetEntrySource", key, source, context, new_source) }
			}
			pub fn fn_set_entry_value(&self, key: &str, source: &str, context: &str, locale_id: &str, text: &str) {
				unsafe { dyn_localization_table_set_entry_value(self.to_ptr(), "SetEntryValue", key, source, context, locale_id, text) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_lod_data_entity {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn entity_lod_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "EntityLodEnabled") }
			}
			pub fn set_entity_lod_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "EntityLodEnabled", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_lua_source_container {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_base_script {
	($name:ident) => {
		impl_lua_source_container!($name);
		impl $name {
			pub fn disabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Disabled") }
			}
			pub fn set_disabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Disabled", value) }
			}
			pub fn linked_source(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "LinkedSource")) }
			}
			pub fn set_linked_source(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "LinkedSource", value.to_ptr()) }
			}
		}
		impl From<$name> for LuaSourceContainer {
			fn from(value: $name) -> LuaSourceContainer {
				LuaSourceContainer(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_script {
	($name:ident) => {
		impl_base_script!($name);
		impl $name {
		}
		impl From<$name> for BaseScript {
			fn from(value: $name) -> BaseScript {
				BaseScript(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_local_script {
	($name:ident) => {
		impl_script!($name);
		impl $name {
		}
		impl From<$name> for Script {
			fn from(value: $name) -> Script {
				Script(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_module_script {
	($name:ident) => {
		impl_lua_source_container!($name);
		impl $name {
			pub fn linked_source(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "LinkedSource")) }
			}
			pub fn set_linked_source(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "LinkedSource", value.to_ptr()) }
			}
		}
		impl From<$name> for LuaSourceContainer {
			fn from(value: $name) -> LuaSourceContainer {
				LuaSourceContainer(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_marker_curve {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Length") }
			}
			pub fn fn_get_marker_at_index(&self, index: f64) {
				unsafe { dyn_marker_curve_get_marker_at_index(self.to_ptr(), "GetMarkerAtIndex", index) }
			}
			pub fn fn_get_markers(&self) {
				unsafe { dyn_marker_curve_get_markers(self.to_ptr(), "GetMarkers") }
			}
			pub fn fn_insert_marker_at_time(&self, time: f64, marker: &str) {
				unsafe { dyn_marker_curve_insert_marker_at_time(self.to_ptr(), "InsertMarkerAtTime", time, marker) }
			}
			pub fn fn_remove_marker_at_index(&self, starting_index: f64, count: f64) -> f64 {
				unsafe { dyn_marker_curve_remove_marker_at_index(self.to_ptr(), "RemoveMarkerAtIndex", starting_index, count) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_material_variant {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn studs_per_tile(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "StudsPerTile") }
			}
			pub fn set_studs_per_tile(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "StudsPerTile", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_memory_store_queue {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_read_async(&self, count: f64, all_or_nothing: bool, wait_timeout: f64) {
				unsafe { dyn_memory_store_queue_read_async(self.to_ptr(), "ReadAsync", count, all_or_nothing, wait_timeout) }
			}
			pub fn fn_remove_async(&self, id: &str) {
				unsafe { dyn_memory_store_queue_remove_async(self.to_ptr(), "RemoveAsync", id) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_memory_store_sorted_map {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_get_async(&self, key: &str) {
				unsafe { dyn_memory_store_sorted_map_get_async(self.to_ptr(), "GetAsync", key) }
			}
			pub fn fn_remove_async(&self, key: &str) {
				unsafe { dyn_memory_store_sorted_map_remove_async(self.to_ptr(), "RemoveAsync", key) }
			}
			pub fn fn_update_async(&self, key: &str, transform_function: Function, expiration: f64) {
				unsafe { dyn_memory_store_sorted_map_update_async(self.to_ptr(), "UpdateAsync", key, transform_function, expiration) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_mouse {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn hit(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "Hit")) }
			}
			pub fn icon(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "Icon")) }
			}
			pub fn set_icon(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "Icon", value.to_ptr()) }
			}
			pub fn origin(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "Origin")) }
			}
			pub fn target(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "Target").map(|id| BasePart(id)) }
			}
			pub fn target_filter(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "TargetFilter").map(|id| Instance(id)) }
			}
			pub fn set_target_filter(&self, value: Option<Instance>) {
				unsafe { set_instance_property(self.to_ptr(), "TargetFilter", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn unit_ray(&self) -> Ray {
				unsafe { Ray(get_datatype_property(self.to_ptr(), "UnitRay")) }
			}
			pub fn view_size_x(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ViewSizeX") }
			}
			pub fn view_size_y(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ViewSizeY") }
			}
			pub fn x(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "X") }
			}
			pub fn y(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Y") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_player_mouse {
	($name:ident) => {
		impl_mouse!($name);
		impl $name {
		}
		impl From<$name> for Mouse {
			fn from(value: $name) -> Mouse {
				Mouse(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_network_marker {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_no_collision_constraint {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn part_0(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "Part0").map(|id| BasePart(id)) }
			}
			pub fn set_part_0(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "Part0", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn part_1(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "Part1").map(|id| BasePart(id)) }
			}
			pub fn set_part_1(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "Part1", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pv_instance {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_get_pivot(&self) -> CFrame {
				unsafe { dyn_pv_instance_get_pivot(self.to_ptr(), "GetPivot") }
			}
			pub fn fn_pivot_to(&self, target_c_frame: CFrame) {
				unsafe { dyn_pv_instance_pivot_to(self.to_ptr(), "PivotTo", target_c_frame) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_base_part {
	($name:ident) => {
		impl_pv_instance!($name);
		impl $name {
			pub fn anchored(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Anchored") }
			}
			pub fn set_anchored(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Anchored", value) }
			}
			pub fn assembly_angular_velocity(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "AssemblyAngularVelocity")) }
			}
			pub fn set_assembly_angular_velocity(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "AssemblyAngularVelocity", value.to_ptr()) }
			}
			pub fn assembly_center_of_mass(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "AssemblyCenterOfMass")) }
			}
			pub fn assembly_linear_velocity(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "AssemblyLinearVelocity")) }
			}
			pub fn set_assembly_linear_velocity(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "AssemblyLinearVelocity", value.to_ptr()) }
			}
			pub fn assembly_mass(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "AssemblyMass") }
			}
			pub fn assembly_root_part(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "AssemblyRootPart").map(|id| BasePart(id)) }
			}
			#[deprecated]
			pub fn back_param_a(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "BackParamA") }
			}
			#[deprecated]
			pub fn set_back_param_a(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "BackParamA", value) }
			}
			#[deprecated]
			pub fn back_param_b(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "BackParamB") }
			}
			#[deprecated]
			pub fn set_back_param_b(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "BackParamB", value) }
			}
			#[deprecated]
			pub fn bottom_param_a(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "BottomParamA") }
			}
			#[deprecated]
			pub fn set_bottom_param_a(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "BottomParamA", value) }
			}
			#[deprecated]
			pub fn bottom_param_b(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "BottomParamB") }
			}
			#[deprecated]
			pub fn set_bottom_param_b(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "BottomParamB", value) }
			}
			pub fn brick_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "BrickColor")) }
			}
			pub fn set_brick_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "BrickColor", value.to_ptr()) }
			}
			pub fn c_frame(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "CFrame")) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "CFrame", value.to_ptr()) }
			}
			pub fn can_collide(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CanCollide") }
			}
			pub fn set_can_collide(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "CanCollide", value) }
			}
			pub fn can_query(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CanQuery") }
			}
			pub fn set_can_query(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "CanQuery", value) }
			}
			pub fn can_touch(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CanTouch") }
			}
			pub fn set_can_touch(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "CanTouch", value) }
			}
			pub fn cast_shadow(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CastShadow") }
			}
			pub fn set_cast_shadow(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "CastShadow", value) }
			}
			pub fn center_of_mass(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "CenterOfMass")) }
			}
			pub fn collision_group_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CollisionGroupId") }
			}
			pub fn set_collision_group_id(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CollisionGroupId", value) }
			}
			pub fn color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Color")) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn custom_physical_properties(&self) -> PhysicalProperties {
				unsafe { PhysicalProperties(get_datatype_property(self.to_ptr(), "CustomPhysicalProperties")) }
			}
			pub fn set_custom_physical_properties(&self, value: PhysicalProperties) {
				unsafe { set_datatype_property(self.to_ptr(), "CustomPhysicalProperties", value.to_ptr()) }
			}
			#[deprecated]
			pub fn elasticity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Elasticity") }
			}
			#[deprecated]
			pub fn set_elasticity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Elasticity", value) }
			}
			#[deprecated]
			pub fn friction(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Friction") }
			}
			#[deprecated]
			pub fn set_friction(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Friction", value) }
			}
			#[deprecated]
			pub fn front_param_a(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "FrontParamA") }
			}
			#[deprecated]
			pub fn set_front_param_a(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "FrontParamA", value) }
			}
			#[deprecated]
			pub fn front_param_b(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "FrontParamB") }
			}
			#[deprecated]
			pub fn set_front_param_b(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "FrontParamB", value) }
			}
			#[deprecated]
			pub fn left_param_a(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LeftParamA") }
			}
			#[deprecated]
			pub fn set_left_param_a(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LeftParamA", value) }
			}
			#[deprecated]
			pub fn left_param_b(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LeftParamB") }
			}
			#[deprecated]
			pub fn set_left_param_b(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LeftParamB", value) }
			}
			pub fn local_transparency_modifier(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LocalTransparencyModifier") }
			}
			pub fn set_local_transparency_modifier(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LocalTransparencyModifier", value) }
			}
			pub fn locked(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Locked") }
			}
			pub fn set_locked(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Locked", value) }
			}
			pub fn mass(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Mass") }
			}
			pub fn massless(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Massless") }
			}
			pub fn set_massless(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Massless", value) }
			}
			pub fn material_variant(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "MaterialVariant") }
			}
			pub fn set_material_variant(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "MaterialVariant", &value) }
			}
			pub fn orientation(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Orientation")) }
			}
			pub fn set_orientation(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Orientation", value.to_ptr()) }
			}
			pub fn pivot_offset(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "PivotOffset")) }
			}
			pub fn set_pivot_offset(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "PivotOffset", value.to_ptr()) }
			}
			pub fn position(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Position")) }
			}
			pub fn set_position(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Position", value.to_ptr()) }
			}
			pub fn receive_age(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ReceiveAge") }
			}
			pub fn reflectance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Reflectance") }
			}
			pub fn set_reflectance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Reflectance", value) }
			}
			pub fn resize_increment(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ResizeIncrement") }
			}
			pub fn resizeable_faces(&self) -> Faces {
				unsafe { Faces(get_datatype_property(self.to_ptr(), "ResizeableFaces")) }
			}
			#[deprecated]
			pub fn right_param_a(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "RightParamA") }
			}
			#[deprecated]
			pub fn set_right_param_a(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "RightParamA", value) }
			}
			#[deprecated]
			pub fn right_param_b(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "RightParamB") }
			}
			#[deprecated]
			pub fn set_right_param_b(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "RightParamB", value) }
			}
			pub fn root_priority(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "RootPriority") }
			}
			pub fn set_root_priority(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "RootPriority", value) }
			}
			#[deprecated]
			pub fn rot_velocity(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "RotVelocity")) }
			}
			#[deprecated]
			pub fn set_rot_velocity(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "RotVelocity", value.to_ptr()) }
			}
			pub fn rotation(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Rotation")) }
			}
			pub fn set_rotation(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Rotation", value.to_ptr()) }
			}
			pub fn size(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Size")) }
			}
			pub fn set_size(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Size", value.to_ptr()) }
			}
			#[deprecated]
			pub fn specific_gravity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "SpecificGravity") }
			}
			#[deprecated]
			pub fn top_param_a(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TopParamA") }
			}
			#[deprecated]
			pub fn set_top_param_a(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TopParamA", value) }
			}
			#[deprecated]
			pub fn top_param_b(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TopParamB") }
			}
			#[deprecated]
			pub fn set_top_param_b(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TopParamB", value) }
			}
			pub fn transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Transparency") }
			}
			pub fn set_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Transparency", value) }
			}
			#[deprecated]
			pub fn velocity(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Velocity")) }
			}
			#[deprecated]
			pub fn set_velocity(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Velocity", value.to_ptr()) }
			}
			pub fn fn_apply_angular_impulse(&self, impulse: Vector3) {
				unsafe { dyn_base_part_apply_angular_impulse(self.to_ptr(), "ApplyAngularImpulse", impulse) }
			}
			pub fn fn_apply_impulse(&self, impulse: Vector3) {
				unsafe { dyn_base_part_apply_impulse(self.to_ptr(), "ApplyImpulse", impulse) }
			}
			pub fn fn_apply_impulse_at_position(&self, impulse: Vector3, position: Vector3) {
				unsafe { dyn_base_part_apply_impulse_at_position(self.to_ptr(), "ApplyImpulseAtPosition", impulse, position) }
			}
			pub fn fn_break_joints(&self) {
				unsafe { dyn_base_part_break_joints(self.to_ptr(), "BreakJoints") }
			}
			pub fn fn_can_collide_with(&self, part: Option<BasePart>) -> bool {
				unsafe { dyn_base_part_can_collide_with(self.to_ptr(), "CanCollideWith", part) }
			}
			pub fn fn_can_set_network_ownership(&self) {
				unsafe { dyn_base_part_can_set_network_ownership(self.to_ptr(), "CanSetNetworkOwnership") }
			}
			pub fn fn_get_connected_parts(&self, recursive: bool) -> Objects {
				unsafe { dyn_base_part_get_connected_parts(self.to_ptr(), "GetConnectedParts", recursive) }
			}
			pub fn fn_get_joints(&self) -> Objects {
				unsafe { dyn_base_part_get_joints(self.to_ptr(), "GetJoints") }
			}
			pub fn fn_get_mass(&self) -> f64 {
				unsafe { dyn_base_part_get_mass(self.to_ptr(), "GetMass") }
			}
			pub fn fn_get_network_owner(&self) -> Option<Instance> {
				unsafe { dyn_base_part_get_network_owner(self.to_ptr(), "GetNetworkOwner") }
			}
			pub fn fn_get_network_ownership_auto(&self) -> bool {
				unsafe { dyn_base_part_get_network_ownership_auto(self.to_ptr(), "GetNetworkOwnershipAuto") }
			}
			pub fn fn_get_root_part(&self) -> Option<Instance> {
				unsafe { dyn_base_part_get_root_part(self.to_ptr(), "GetRootPart") }
			}
			pub fn fn_get_touching_parts(&self) -> Objects {
				unsafe { dyn_base_part_get_touching_parts(self.to_ptr(), "GetTouchingParts") }
			}
			pub fn fn_get_velocity_at_position(&self, position: Vector3) -> Vector3 {
				unsafe { dyn_base_part_get_velocity_at_position(self.to_ptr(), "GetVelocityAtPosition", position) }
			}
			pub fn fn_is_grounded(&self) -> bool {
				unsafe { dyn_base_part_is_grounded(self.to_ptr(), "IsGrounded") }
			}
			pub fn fn_make_joints(&self) {
				unsafe { dyn_base_part_make_joints(self.to_ptr(), "MakeJoints") }
			}
			pub fn fn_set_network_owner(&self, player_instance: Option<Player>) {
				unsafe { dyn_base_part_set_network_owner(self.to_ptr(), "SetNetworkOwner", player_instance) }
			}
			pub fn fn_set_network_ownership_auto(&self) {
				unsafe { dyn_base_part_set_network_ownership_auto(self.to_ptr(), "SetNetworkOwnershipAuto") }
			}
		}
		impl From<$name> for PVInstance {
			fn from(value: $name) -> PVInstance {
				PVInstance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_corner_wedge_part {
	($name:ident) => {
		impl_base_part!($name);
		impl $name {
		}
		impl From<$name> for BasePart {
			fn from(value: $name) -> BasePart {
				BasePart(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_form_factor_part {
	($name:ident) => {
		impl_base_part!($name);
		impl $name {
		}
		impl From<$name> for BasePart {
			fn from(value: $name) -> BasePart {
				BasePart(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_part {
	($name:ident) => {
		impl_form_factor_part!($name);
		impl $name {
		}
		impl From<$name> for FormFactorPart {
			fn from(value: $name) -> FormFactorPart {
				FormFactorPart(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_platform {
	($name:ident) => {
		impl_part!($name);
		impl $name {
		}
		impl From<$name> for Part {
			fn from(value: $name) -> Part {
				Part(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_seat {
	($name:ident) => {
		impl_part!($name);
		impl $name {
			pub fn disabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Disabled") }
			}
			pub fn set_disabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Disabled", value) }
			}
			pub fn occupant(&self) -> Option<Humanoid> {
				unsafe { get_instance_property(self.to_ptr(), "Occupant").map(|id| Humanoid(id)) }
			}
			pub fn fn_sit(&self, humanoid: Option<Instance>) {
				unsafe { dyn_seat_sit(self.to_ptr(), "Sit", humanoid) }
			}
		}
		impl From<$name> for Part {
			fn from(value: $name) -> Part {
				Part(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_skateboard_platform {
	($name:ident) => {
		impl_part!($name);
		impl $name {
			pub fn controller(&self) -> Option<SkateboardController> {
				unsafe { get_instance_property(self.to_ptr(), "Controller").map(|id| SkateboardController(id)) }
			}
			pub fn controlling_humanoid(&self) -> Option<Humanoid> {
				unsafe { get_instance_property(self.to_ptr(), "ControllingHumanoid").map(|id| Humanoid(id)) }
			}
			pub fn steer(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Steer") }
			}
			pub fn set_steer(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Steer", value) }
			}
			pub fn sticky_wheels(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "StickyWheels") }
			}
			pub fn set_sticky_wheels(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "StickyWheels", value) }
			}
			pub fn throttle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Throttle") }
			}
			pub fn set_throttle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Throttle", value) }
			}
			pub fn fn_apply_specific_impulse(&self, impulse_world: Vector3) {
				unsafe { dyn_skateboard_platform_apply_specific_impulse(self.to_ptr(), "ApplySpecificImpulse", impulse_world) }
			}
		}
		impl From<$name> for Part {
			fn from(value: $name) -> Part {
				Part(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_spawn_location {
	($name:ident) => {
		impl_part!($name);
		impl $name {
			pub fn allow_team_change_on_touch(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AllowTeamChangeOnTouch") }
			}
			pub fn set_allow_team_change_on_touch(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AllowTeamChangeOnTouch", value) }
			}
			pub fn duration(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Duration") }
			}
			pub fn set_duration(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Duration", value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn neutral(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Neutral") }
			}
			pub fn set_neutral(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Neutral", value) }
			}
			pub fn team_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "TeamColor")) }
			}
			pub fn set_team_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "TeamColor", value.to_ptr()) }
			}
		}
		impl From<$name> for Part {
			fn from(value: $name) -> Part {
				Part(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_wedge_part {
	($name:ident) => {
		impl_form_factor_part!($name);
		impl $name {
		}
		impl From<$name> for FormFactorPart {
			fn from(value: $name) -> FormFactorPart {
				FormFactorPart(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_terrain {
	($name:ident) => {
		impl_base_part!($name);
		impl $name {
			#[deprecated]
			pub fn is_smooth(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsSmooth") }
			}
			pub fn max_extents(&self) -> Region3int16 {
				unsafe { Region3int16(get_datatype_property(self.to_ptr(), "MaxExtents")) }
			}
			pub fn water_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "WaterColor")) }
			}
			pub fn set_water_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "WaterColor", value.to_ptr()) }
			}
			pub fn water_reflectance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WaterReflectance") }
			}
			pub fn set_water_reflectance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WaterReflectance", value) }
			}
			pub fn water_transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WaterTransparency") }
			}
			pub fn set_water_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WaterTransparency", value) }
			}
			pub fn water_wave_size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WaterWaveSize") }
			}
			pub fn set_water_wave_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WaterWaveSize", value) }
			}
			pub fn water_wave_speed(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WaterWaveSpeed") }
			}
			pub fn set_water_wave_speed(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WaterWaveSpeed", value) }
			}
			pub fn fn_cell_center_to_world(&self, x: f64, y: f64, z: f64) -> Vector3 {
				unsafe { dyn_terrain_cell_center_to_world(self.to_ptr(), "CellCenterToWorld", x, y, z) }
			}
			pub fn fn_cell_corner_to_world(&self, x: f64, y: f64, z: f64) -> Vector3 {
				unsafe { dyn_terrain_cell_corner_to_world(self.to_ptr(), "CellCornerToWorld", x, y, z) }
			}
			pub fn fn_clear(&self) {
				unsafe { dyn_terrain_clear(self.to_ptr(), "Clear") }
			}
			pub fn fn_copy_region(&self, region: Region3int16) -> Option<TerrainRegion> {
				unsafe { dyn_terrain_copy_region(self.to_ptr(), "CopyRegion", region) }
			}
			pub fn fn_count_cells(&self) -> f64 {
				unsafe { dyn_terrain_count_cells(self.to_ptr(), "CountCells") }
			}
			pub fn fn_paste_region(&self, region: Option<TerrainRegion>, corner: Vector3int16, paste_empty_cells: bool) {
				unsafe { dyn_terrain_paste_region(self.to_ptr(), "PasteRegion", region, corner, paste_empty_cells) }
			}
			pub fn fn_read_voxels(&self, region: Region3, resolution: f64) {
				unsafe { dyn_terrain_read_voxels(self.to_ptr(), "ReadVoxels", region, resolution) }
			}
			pub fn fn_world_to_cell(&self, position: Vector3) -> Vector3 {
				unsafe { dyn_terrain_world_to_cell(self.to_ptr(), "WorldToCell", position) }
			}
			pub fn fn_world_to_cell_prefer_empty(&self, position: Vector3) -> Vector3 {
				unsafe { dyn_terrain_world_to_cell_prefer_empty(self.to_ptr(), "WorldToCellPreferEmpty", position) }
			}
			pub fn fn_world_to_cell_prefer_solid(&self, position: Vector3) -> Vector3 {
				unsafe { dyn_terrain_world_to_cell_prefer_solid(self.to_ptr(), "WorldToCellPreferSolid", position) }
			}
		}
		impl From<$name> for BasePart {
			fn from(value: $name) -> BasePart {
				BasePart(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_triangle_mesh_part {
	($name:ident) => {
		impl_base_part!($name);
		impl $name {
		}
		impl From<$name> for BasePart {
			fn from(value: $name) -> BasePart {
				BasePart(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_mesh_part {
	($name:ident) => {
		impl_triangle_mesh_part!($name);
		impl $name {
			pub fn double_sided(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "DoubleSided") }
			}
			pub fn has_joint_offset(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "HasJointOffset") }
			}
			pub fn has_skinned_mesh(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "HasSkinnedMesh") }
			}
			pub fn joint_offset(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "JointOffset")) }
			}
			pub fn mesh_id(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "MeshId")) }
			}
			pub fn mesh_size(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "MeshSize")) }
			}
			pub fn texture_id(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "TextureID")) }
			}
			pub fn set_texture_id(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "TextureID", value.to_ptr()) }
			}
			pub fn fn_apply_mesh(&self, mesh_part: Option<Instance>) {
				unsafe { dyn_mesh_part_apply_mesh(self.to_ptr(), "ApplyMesh", mesh_part) }
			}
		}
		impl From<$name> for TriangleMeshPart {
			fn from(value: $name) -> TriangleMeshPart {
				TriangleMeshPart(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_part_operation {
	($name:ident) => {
		impl_triangle_mesh_part!($name);
		impl $name {
			pub fn smoothing_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "SmoothingAngle") }
			}
			pub fn triangle_count(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TriangleCount") }
			}
			pub fn use_part_color(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "UsePartColor") }
			}
			pub fn set_use_part_color(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "UsePartColor", value) }
			}
		}
		impl From<$name> for TriangleMeshPart {
			fn from(value: $name) -> TriangleMeshPart {
				TriangleMeshPart(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_negate_operation {
	($name:ident) => {
		impl_part_operation!($name);
		impl $name {
		}
		impl From<$name> for PartOperation {
			fn from(value: $name) -> PartOperation {
				PartOperation(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_union_operation {
	($name:ident) => {
		impl_part_operation!($name);
		impl $name {
		}
		impl From<$name> for PartOperation {
			fn from(value: $name) -> PartOperation {
				PartOperation(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_truss_part {
	($name:ident) => {
		impl_base_part!($name);
		impl $name {
		}
		impl From<$name> for BasePart {
			fn from(value: $name) -> BasePart {
				BasePart(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_vehicle_seat {
	($name:ident) => {
		impl_base_part!($name);
		impl $name {
			pub fn are_hinges_detected(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "AreHingesDetected") }
			}
			pub fn disabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Disabled") }
			}
			pub fn set_disabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Disabled", value) }
			}
			pub fn heads_up_display(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "HeadsUpDisplay") }
			}
			pub fn set_heads_up_display(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "HeadsUpDisplay", value) }
			}
			pub fn max_speed(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxSpeed") }
			}
			pub fn set_max_speed(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxSpeed", value) }
			}
			pub fn occupant(&self) -> Option<Humanoid> {
				unsafe { get_instance_property(self.to_ptr(), "Occupant").map(|id| Humanoid(id)) }
			}
			pub fn steer(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Steer") }
			}
			pub fn set_steer(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Steer", value) }
			}
			pub fn steer_float(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "SteerFloat") }
			}
			pub fn set_steer_float(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "SteerFloat", value) }
			}
			pub fn throttle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Throttle") }
			}
			pub fn set_throttle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Throttle", value) }
			}
			pub fn throttle_float(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ThrottleFloat") }
			}
			pub fn set_throttle_float(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ThrottleFloat", value) }
			}
			pub fn torque(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Torque") }
			}
			pub fn set_torque(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Torque", value) }
			}
			pub fn turn_speed(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TurnSpeed") }
			}
			pub fn set_turn_speed(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TurnSpeed", value) }
			}
			pub fn fn_sit(&self, humanoid: Option<Instance>) {
				unsafe { dyn_vehicle_seat_sit(self.to_ptr(), "Sit", humanoid) }
			}
		}
		impl From<$name> for BasePart {
			fn from(value: $name) -> BasePart {
				BasePart(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_model {
	($name:ident) => {
		impl_pv_instance!($name);
		impl $name {
			pub fn primary_part(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "PrimaryPart").map(|id| BasePart(id)) }
			}
			pub fn set_primary_part(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "PrimaryPart", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn world_pivot(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "WorldPivot")) }
			}
			pub fn set_world_pivot(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "WorldPivot", value.to_ptr()) }
			}
			pub fn fn_break_joints(&self) {
				unsafe { dyn_model_break_joints(self.to_ptr(), "BreakJoints") }
			}
			pub fn fn_get_bounding_box(&self) {
				unsafe { dyn_model_get_bounding_box(self.to_ptr(), "GetBoundingBox") }
			}
			pub fn fn_get_extents_size(&self) -> Vector3 {
				unsafe { dyn_model_get_extents_size(self.to_ptr(), "GetExtentsSize") }
			}
			pub fn fn_get_primary_part_c_frame(&self) -> CFrame {
				unsafe { dyn_model_get_primary_part_c_frame(self.to_ptr(), "GetPrimaryPartCFrame") }
			}
			pub fn fn_make_joints(&self) {
				unsafe { dyn_model_make_joints(self.to_ptr(), "MakeJoints") }
			}
			pub fn fn_move_to(&self, position: Vector3) {
				unsafe { dyn_model_move_to(self.to_ptr(), "MoveTo", position) }
			}
			pub fn fn_set_primary_part_c_frame(&self, cframe: CFrame) {
				unsafe { dyn_model_set_primary_part_c_frame(self.to_ptr(), "SetPrimaryPartCFrame", cframe) }
			}
			pub fn fn_translate_by(&self, delta: Vector3) {
				unsafe { dyn_model_translate_by(self.to_ptr(), "TranslateBy", delta) }
			}
		}
		impl From<$name> for PVInstance {
			fn from(value: $name) -> PVInstance {
				PVInstance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_actor {
	($name:ident) => {
		impl_model!($name);
		impl $name {
		}
		impl From<$name> for Model {
			fn from(value: $name) -> Model {
				Model(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_world_root {
	($name:ident) => {
		impl_model!($name);
		impl $name {
			pub fn fn_are_parts_touching_others(&self, part_list: Objects, overlap_ignored: f64) -> bool {
				unsafe { dyn_world_root_are_parts_touching_others(self.to_ptr(), "ArePartsTouchingOthers", part_list, overlap_ignored) }
			}
			pub fn fn_get_part_bounds_in_box(&self, cframe: CFrame, size: Vector3, overlap_params: OverlapParams) -> Objects {
				unsafe { dyn_world_root_get_part_bounds_in_box(self.to_ptr(), "GetPartBoundsInBox", cframe, size, overlap_params) }
			}
			pub fn fn_get_part_bounds_in_radius(&self, position: Vector3, radius: f64, overlap_params: OverlapParams) -> Objects {
				unsafe { dyn_world_root_get_part_bounds_in_radius(self.to_ptr(), "GetPartBoundsInRadius", position, radius, overlap_params) }
			}
			pub fn fn_get_parts_in_part(&self, part: Option<BasePart>, overlap_params: OverlapParams) -> Objects {
				unsafe { dyn_world_root_get_parts_in_part(self.to_ptr(), "GetPartsInPart", part, overlap_params) }
			}
			pub fn fn_raycast(&self, origin: Vector3, direction: Vector3, raycast_params: RaycastParams) -> RaycastResult {
				unsafe { dyn_world_root_raycast(self.to_ptr(), "Raycast", origin, direction, raycast_params) }
			}
		}
		impl From<$name> for Model {
			fn from(value: $name) -> Model {
				Model(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_workspace {
	($name:ident) => {
		impl_world_root!($name);
		impl $name {
			pub fn allow_third_party_sales(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AllowThirdPartySales") }
			}
			pub fn set_allow_third_party_sales(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AllowThirdPartySales", value) }
			}
			pub fn current_camera(&self) -> Option<Camera> {
				unsafe { get_instance_property(self.to_ptr(), "CurrentCamera").map(|id| Camera(id)) }
			}
			pub fn set_current_camera(&self, value: Option<Camera>) {
				unsafe { set_instance_property(self.to_ptr(), "CurrentCamera", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn distributed_game_time(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DistributedGameTime") }
			}
			pub fn set_distributed_game_time(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DistributedGameTime", value) }
			}
			pub fn fallen_parts_destroy_height(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "FallenPartsDestroyHeight") }
			}
			#[deprecated]
			pub fn filtering_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "FilteringEnabled") }
			}
			pub fn global_wind(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "GlobalWind")) }
			}
			pub fn set_global_wind(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "GlobalWind", value.to_ptr()) }
			}
			pub fn gravity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Gravity") }
			}
			pub fn set_gravity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Gravity", value) }
			}
			pub fn streaming_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "StreamingEnabled") }
			}
			pub fn terrain(&self) -> Option<Terrain> {
				unsafe { get_instance_property(self.to_ptr(), "Terrain").map(|id| Terrain(id)) }
			}
			pub fn fn_get_num_awake_parts(&self) -> f64 {
				unsafe { dyn_workspace_get_num_awake_parts(self.to_ptr(), "GetNumAwakeParts") }
			}
			pub fn fn_get_physics_throttling(&self) -> f64 {
				unsafe { dyn_workspace_get_physics_throttling(self.to_ptr(), "GetPhysicsThrottling") }
			}
			pub fn fn_get_real_physics_fps(&self) -> f64 {
				unsafe { dyn_workspace_get_real_physics_fps(self.to_ptr(), "GetRealPhysicsFPS") }
			}
			pub fn fn_get_server_time_now(&self) -> f64 {
				unsafe { dyn_workspace_get_server_time_now(self.to_ptr(), "GetServerTimeNow") }
			}
			pub fn fn_pgs_is_enabled(&self) -> bool {
				unsafe { dyn_workspace_pgs_is_enabled(self.to_ptr(), "PGSIsEnabled") }
			}
			pub fn fn_unjoin_from_outsiders(&self, objects: Objects) {
				unsafe { dyn_workspace_unjoin_from_outsiders(self.to_ptr(), "UnjoinFromOutsiders", objects) }
			}
		}
		impl From<$name> for WorldRoot {
			fn from(value: $name) -> WorldRoot {
				WorldRoot(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_world_model {
	($name:ident) => {
		impl_world_root!($name);
		impl $name {
		}
		impl From<$name> for WorldRoot {
			fn from(value: $name) -> WorldRoot {
				WorldRoot(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_package_link {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn package_id(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "PackageId")) }
			}
			pub fn version_number(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "VersionNumber") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pages {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn is_finished(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsFinished") }
			}
			pub fn fn_get_current_page(&self) {
				unsafe { dyn_pages_get_current_page(self.to_ptr(), "GetCurrentPage") }
			}
			pub fn fn_advance_to_next_page_async(&self) {
				unsafe { dyn_pages_advance_to_next_page_async(self.to_ptr(), "AdvanceToNextPageAsync") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_catalog_pages {
	($name:ident) => {
		impl_pages!($name);
		impl $name {
		}
		impl From<$name> for Pages {
			fn from(value: $name) -> Pages {
				Pages(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_key_pages {
	($name:ident) => {
		impl_pages!($name);
		impl $name {
		}
		impl From<$name> for Pages {
			fn from(value: $name) -> Pages {
				Pages(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_listing_pages {
	($name:ident) => {
		impl_pages!($name);
		impl $name {
		}
		impl From<$name> for Pages {
			fn from(value: $name) -> Pages {
				Pages(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_pages {
	($name:ident) => {
		impl_pages!($name);
		impl $name {
		}
		impl From<$name> for Pages {
			fn from(value: $name) -> Pages {
				Pages(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_version_pages {
	($name:ident) => {
		impl_pages!($name);
		impl $name {
		}
		impl From<$name> for Pages {
			fn from(value: $name) -> Pages {
				Pages(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_friend_pages {
	($name:ident) => {
		impl_pages!($name);
		impl $name {
		}
		impl From<$name> for Pages {
			fn from(value: $name) -> Pages {
				Pages(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_inventory_pages {
	($name:ident) => {
		impl_pages!($name);
		impl $name {
		}
		impl From<$name> for Pages {
			fn from(value: $name) -> Pages {
				Pages(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_emotes_pages {
	($name:ident) => {
		impl_inventory_pages!($name);
		impl $name {
		}
		impl From<$name> for InventoryPages {
			fn from(value: $name) -> InventoryPages {
				InventoryPages(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_outfit_pages {
	($name:ident) => {
		impl_pages!($name);
		impl $name {
		}
		impl From<$name> for Pages {
			fn from(value: $name) -> Pages {
				Pages(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_standard_pages {
	($name:ident) => {
		impl_pages!($name);
		impl $name {
		}
		impl From<$name> for Pages {
			fn from(value: $name) -> Pages {
				Pages(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_particle_emitter {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn acceleration(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Acceleration")) }
			}
			pub fn set_acceleration(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Acceleration", value.to_ptr()) }
			}
			pub fn brightness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Brightness") }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Brightness", value) }
			}
			pub fn color(&self) -> ColorSequence {
				unsafe { ColorSequence(get_datatype_property(self.to_ptr(), "Color")) }
			}
			pub fn set_color(&self, value: ColorSequence) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn drag(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Drag") }
			}
			pub fn set_drag(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Drag", value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn flipbook_framerate(&self) -> NumberRange {
				unsafe { NumberRange(get_datatype_property(self.to_ptr(), "FlipbookFramerate")) }
			}
			pub fn set_flipbook_framerate(&self, value: NumberRange) {
				unsafe { set_datatype_property(self.to_ptr(), "FlipbookFramerate", value.to_ptr()) }
			}
			pub fn flipbook_incompatible(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "FlipbookIncompatible") }
			}
			pub fn set_flipbook_incompatible(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "FlipbookIncompatible", &value) }
			}
			pub fn flipbook_start_random(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "FlipbookStartRandom") }
			}
			pub fn set_flipbook_start_random(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "FlipbookStartRandom", value) }
			}
			pub fn lifetime(&self) -> NumberRange {
				unsafe { NumberRange(get_datatype_property(self.to_ptr(), "Lifetime")) }
			}
			pub fn set_lifetime(&self, value: NumberRange) {
				unsafe { set_datatype_property(self.to_ptr(), "Lifetime", value.to_ptr()) }
			}
			pub fn light_emission(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LightEmission") }
			}
			pub fn set_light_emission(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LightEmission", value) }
			}
			pub fn light_influence(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LightInfluence") }
			}
			pub fn set_light_influence(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LightInfluence", value) }
			}
			pub fn locked_to_part(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "LockedToPart") }
			}
			pub fn set_locked_to_part(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "LockedToPart", value) }
			}
			pub fn rate(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Rate") }
			}
			pub fn set_rate(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Rate", value) }
			}
			pub fn rot_speed(&self) -> NumberRange {
				unsafe { NumberRange(get_datatype_property(self.to_ptr(), "RotSpeed")) }
			}
			pub fn set_rot_speed(&self, value: NumberRange) {
				unsafe { set_datatype_property(self.to_ptr(), "RotSpeed", value.to_ptr()) }
			}
			pub fn rotation(&self) -> NumberRange {
				unsafe { NumberRange(get_datatype_property(self.to_ptr(), "Rotation")) }
			}
			pub fn set_rotation(&self, value: NumberRange) {
				unsafe { set_datatype_property(self.to_ptr(), "Rotation", value.to_ptr()) }
			}
			pub fn shape_partial(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ShapePartial") }
			}
			pub fn set_shape_partial(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ShapePartial", value) }
			}
			pub fn size(&self) -> NumberSequence {
				unsafe { NumberSequence(get_datatype_property(self.to_ptr(), "Size")) }
			}
			pub fn set_size(&self, value: NumberSequence) {
				unsafe { set_datatype_property(self.to_ptr(), "Size", value.to_ptr()) }
			}
			pub fn speed(&self) -> NumberRange {
				unsafe { NumberRange(get_datatype_property(self.to_ptr(), "Speed")) }
			}
			pub fn set_speed(&self, value: NumberRange) {
				unsafe { set_datatype_property(self.to_ptr(), "Speed", value.to_ptr()) }
			}
			pub fn spread_angle(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "SpreadAngle")) }
			}
			pub fn set_spread_angle(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "SpreadAngle", value.to_ptr()) }
			}
			pub fn squash(&self) -> NumberSequence {
				unsafe { NumberSequence(get_datatype_property(self.to_ptr(), "Squash")) }
			}
			pub fn set_squash(&self, value: NumberSequence) {
				unsafe { set_datatype_property(self.to_ptr(), "Squash", value.to_ptr()) }
			}
			pub fn texture(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "Texture")) }
			}
			pub fn set_texture(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "Texture", value.to_ptr()) }
			}
			pub fn time_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TimeScale") }
			}
			pub fn set_time_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TimeScale", value) }
			}
			pub fn transparency(&self) -> NumberSequence {
				unsafe { NumberSequence(get_datatype_property(self.to_ptr(), "Transparency")) }
			}
			pub fn set_transparency(&self, value: NumberSequence) {
				unsafe { set_datatype_property(self.to_ptr(), "Transparency", value.to_ptr()) }
			}
			pub fn velocity_inheritance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "VelocityInheritance") }
			}
			pub fn set_velocity_inheritance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "VelocityInheritance", value) }
			}
			#[deprecated]
			pub fn velocity_spread(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "VelocitySpread") }
			}
			#[deprecated]
			pub fn set_velocity_spread(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "VelocitySpread", value) }
			}
			pub fn z_offset(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ZOffset") }
			}
			pub fn set_z_offset(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ZOffset", value) }
			}
			pub fn fn_clear(&self) {
				unsafe { dyn_particle_emitter_clear(self.to_ptr(), "Clear") }
			}
			pub fn fn_emit(&self, particle_count: f64) {
				unsafe { dyn_particle_emitter_emit(self.to_ptr(), "Emit", particle_count) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_path {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_get_waypoints(&self) {
				unsafe { dyn_path_get_waypoints(self.to_ptr(), "GetWaypoints") }
			}
			pub fn fn_check_occlusion_async(&self, start: f64) -> f64 {
				unsafe { dyn_path_check_occlusion_async(self.to_ptr(), "CheckOcclusionAsync", start) }
			}
			pub fn fn_compute_async(&self, start: Vector3, finish: Vector3) {
				unsafe { dyn_path_compute_async(self.to_ptr(), "ComputeAsync", start, finish) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pathfinding_link {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn attachment_0(&self) -> Option<Attachment> {
				unsafe { get_instance_property(self.to_ptr(), "Attachment0").map(|id| Attachment(id)) }
			}
			pub fn set_attachment_0(&self, value: Option<Attachment>) {
				unsafe { set_instance_property(self.to_ptr(), "Attachment0", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn attachment_1(&self) -> Option<Attachment> {
				unsafe { get_instance_property(self.to_ptr(), "Attachment1").map(|id| Attachment(id)) }
			}
			pub fn set_attachment_1(&self, value: Option<Attachment>) {
				unsafe { set_instance_property(self.to_ptr(), "Attachment1", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn is_bidirectional(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsBidirectional") }
			}
			pub fn set_is_bidirectional(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "IsBidirectional", value) }
			}
			pub fn label(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Label") }
			}
			pub fn set_label(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "Label", &value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pathfinding_modifier {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn label(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Label") }
			}
			pub fn set_label(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "Label", &value) }
			}
			pub fn pass_through(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "PassThrough") }
			}
			pub fn set_pass_through(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "PassThrough", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_player {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn account_age(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "AccountAge") }
			}
			pub fn auto_jump_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AutoJumpEnabled") }
			}
			pub fn set_auto_jump_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AutoJumpEnabled", value) }
			}
			pub fn camera_max_zoom_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CameraMaxZoomDistance") }
			}
			pub fn set_camera_max_zoom_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CameraMaxZoomDistance", value) }
			}
			pub fn camera_min_zoom_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CameraMinZoomDistance") }
			}
			pub fn set_camera_min_zoom_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CameraMinZoomDistance", value) }
			}
			pub fn can_load_character_appearance(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CanLoadCharacterAppearance") }
			}
			pub fn set_can_load_character_appearance(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "CanLoadCharacterAppearance", value) }
			}
			pub fn character(&self) -> Option<Model> {
				unsafe { get_instance_property(self.to_ptr(), "Character").map(|id| Model(id)) }
			}
			pub fn set_character(&self, value: Option<Model>) {
				unsafe { set_instance_property(self.to_ptr(), "Character", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			#[deprecated]
			pub fn character_appearance(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "CharacterAppearance") }
			}
			#[deprecated]
			pub fn set_character_appearance(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "CharacterAppearance", &value) }
			}
			pub fn character_appearance_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CharacterAppearanceId") }
			}
			pub fn set_character_appearance_id(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CharacterAppearanceId", value) }
			}
			#[deprecated]
			pub fn data_complexity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DataComplexity") }
			}
			#[deprecated]
			pub fn data_ready(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "DataReady") }
			}
			pub fn dev_enable_mouse_lock(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "DevEnableMouseLock") }
			}
			pub fn set_dev_enable_mouse_lock(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "DevEnableMouseLock", value) }
			}
			pub fn display_name(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "DisplayName") }
			}
			pub fn set_display_name(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "DisplayName", &value) }
			}
			pub fn follow_user_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "FollowUserId") }
			}
			pub fn gameplay_paused(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "GameplayPaused") }
			}
			pub fn health_display_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "HealthDisplayDistance") }
			}
			pub fn set_health_display_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "HealthDisplayDistance", value) }
			}
			pub fn locale_id(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "LocaleId") }
			}
			pub fn name_display_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "NameDisplayDistance") }
			}
			pub fn set_name_display_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "NameDisplayDistance", value) }
			}
			pub fn neutral(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Neutral") }
			}
			pub fn set_neutral(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Neutral", value) }
			}
			pub fn replication_focus(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "ReplicationFocus").map(|id| Instance(id)) }
			}
			pub fn set_replication_focus(&self, value: Option<Instance>) {
				unsafe { set_instance_property(self.to_ptr(), "ReplicationFocus", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn respawn_location(&self) -> Option<SpawnLocation> {
				unsafe { get_instance_property(self.to_ptr(), "RespawnLocation").map(|id| SpawnLocation(id)) }
			}
			pub fn set_respawn_location(&self, value: Option<SpawnLocation>) {
				unsafe { set_instance_property(self.to_ptr(), "RespawnLocation", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn team(&self) -> Option<Team> {
				unsafe { get_instance_property(self.to_ptr(), "Team").map(|id| Team(id)) }
			}
			pub fn set_team(&self, value: Option<Team>) {
				unsafe { set_instance_property(self.to_ptr(), "Team", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn team_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "TeamColor")) }
			}
			pub fn set_team_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "TeamColor", value.to_ptr()) }
			}
			pub fn user_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "UserId") }
			}
			pub fn set_user_id(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "UserId", value) }
			}
			pub fn fn_clear_character_appearance(&self) {
				unsafe { dyn_player_clear_character_appearance(self.to_ptr(), "ClearCharacterAppearance") }
			}
			pub fn fn_distance_from_character(&self, point: Vector3) -> f64 {
				unsafe { dyn_player_distance_from_character(self.to_ptr(), "DistanceFromCharacter", point) }
			}
			pub fn fn_get_join_data(&self) {
				unsafe { dyn_player_get_join_data(self.to_ptr(), "GetJoinData") }
			}
			pub fn fn_get_mouse(&self) -> Option<Mouse> {
				unsafe { dyn_player_get_mouse(self.to_ptr(), "GetMouse") }
			}
			pub fn fn_get_network_ping(&self) -> f64 {
				unsafe { dyn_player_get_network_ping(self.to_ptr(), "GetNetworkPing") }
			}
			pub fn fn_has_appearance_loaded(&self) -> bool {
				unsafe { dyn_player_has_appearance_loaded(self.to_ptr(), "HasAppearanceLoaded") }
			}
			pub fn fn_kick(&self, message: &str) {
				unsafe { dyn_player_kick(self.to_ptr(), "Kick", message) }
			}
			pub fn fn_move(&self, walk_direction: Vector3, relative_to_camera: bool) {
				unsafe { dyn_player_move(self.to_ptr(), "Move", walk_direction, relative_to_camera) }
			}
			pub fn fn_get_friends_online(&self, max_friends: f64) {
				unsafe { dyn_player_get_friends_online(self.to_ptr(), "GetFriendsOnline", max_friends) }
			}
			pub fn fn_get_rank_in_group(&self, group_id: f64) -> f64 {
				unsafe { dyn_player_get_rank_in_group(self.to_ptr(), "GetRankInGroup", group_id) }
			}
			pub fn fn_get_role_in_group(&self, group_id: f64) -> String {
				unsafe { dyn_player_get_role_in_group(self.to_ptr(), "GetRoleInGroup", group_id) }
			}
			pub fn fn_is_friends_with(&self, user_id: f64) -> bool {
				unsafe { dyn_player_is_friends_with(self.to_ptr(), "IsFriendsWith", user_id) }
			}
			pub fn fn_is_in_group(&self, group_id: f64) -> bool {
				unsafe { dyn_player_is_in_group(self.to_ptr(), "IsInGroup", group_id) }
			}
			pub fn fn_load_character(&self) {
				unsafe { dyn_player_load_character(self.to_ptr(), "LoadCharacter") }
			}
			pub fn fn_load_character_with_humanoid_description(&self, humanoid_description: Option<HumanoidDescription>) {
				unsafe { dyn_player_load_character_with_humanoid_description(self.to_ptr(), "LoadCharacterWithHumanoidDescription", humanoid_description) }
			}
			pub fn fn_request_stream_around_async(&self, position: Vector3, time_out: f64) {
				unsafe { dyn_player_request_stream_around_async(self.to_ptr(), "RequestStreamAroundAsync", position, time_out) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_player_scripts {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_clear_computer_camera_movement_modes(&self) {
				unsafe { dyn_player_scripts_clear_computer_camera_movement_modes(self.to_ptr(), "ClearComputerCameraMovementModes") }
			}
			pub fn fn_clear_computer_movement_modes(&self) {
				unsafe { dyn_player_scripts_clear_computer_movement_modes(self.to_ptr(), "ClearComputerMovementModes") }
			}
			pub fn fn_clear_touch_camera_movement_modes(&self) {
				unsafe { dyn_player_scripts_clear_touch_camera_movement_modes(self.to_ptr(), "ClearTouchCameraMovementModes") }
			}
			pub fn fn_clear_touch_movement_modes(&self) {
				unsafe { dyn_player_scripts_clear_touch_movement_modes(self.to_ptr(), "ClearTouchMovementModes") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pose_base {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn weight(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Weight") }
			}
			pub fn set_weight(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Weight", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_number_pose {
	($name:ident) => {
		impl_pose_base!($name);
		impl $name {
			pub fn value(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Value") }
			}
			pub fn set_value(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Value", value) }
			}
		}
		impl From<$name> for PoseBase {
			fn from(value: $name) -> PoseBase {
				PoseBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pose {
	($name:ident) => {
		impl_pose_base!($name);
		impl $name {
			pub fn c_frame(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "CFrame")) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "CFrame", value.to_ptr()) }
			}
			#[deprecated]
			pub fn mask_weight(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaskWeight") }
			}
			#[deprecated]
			pub fn set_mask_weight(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaskWeight", value) }
			}
			pub fn fn_add_sub_pose(&self, pose: Option<Instance>) {
				unsafe { dyn_pose_add_sub_pose(self.to_ptr(), "AddSubPose", pose) }
			}
			pub fn fn_get_sub_poses(&self) -> Objects {
				unsafe { dyn_pose_get_sub_poses(self.to_ptr(), "GetSubPoses") }
			}
			pub fn fn_remove_sub_pose(&self, pose: Option<Instance>) {
				unsafe { dyn_pose_remove_sub_pose(self.to_ptr(), "RemoveSubPose", pose) }
			}
		}
		impl From<$name> for PoseBase {
			fn from(value: $name) -> PoseBase {
				PoseBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_post_effect {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_bloom_effect {
	($name:ident) => {
		impl_post_effect!($name);
		impl $name {
			pub fn intensity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Intensity") }
			}
			pub fn set_intensity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Intensity", value) }
			}
			pub fn size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Size") }
			}
			pub fn set_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Size", value) }
			}
			pub fn threshold(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Threshold") }
			}
			pub fn set_threshold(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Threshold", value) }
			}
		}
		impl From<$name> for PostEffect {
			fn from(value: $name) -> PostEffect {
				PostEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_blur_effect {
	($name:ident) => {
		impl_post_effect!($name);
		impl $name {
			pub fn size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Size") }
			}
			pub fn set_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Size", value) }
			}
		}
		impl From<$name> for PostEffect {
			fn from(value: $name) -> PostEffect {
				PostEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_color_correction_effect {
	($name:ident) => {
		impl_post_effect!($name);
		impl $name {
			pub fn brightness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Brightness") }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Brightness", value) }
			}
			pub fn contrast(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Contrast") }
			}
			pub fn set_contrast(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Contrast", value) }
			}
			pub fn saturation(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Saturation") }
			}
			pub fn set_saturation(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Saturation", value) }
			}
			pub fn tint_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "TintColor")) }
			}
			pub fn set_tint_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "TintColor", value.to_ptr()) }
			}
		}
		impl From<$name> for PostEffect {
			fn from(value: $name) -> PostEffect {
				PostEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_depth_of_field_effect {
	($name:ident) => {
		impl_post_effect!($name);
		impl $name {
			pub fn far_intensity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "FarIntensity") }
			}
			pub fn set_far_intensity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "FarIntensity", value) }
			}
			pub fn focus_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "FocusDistance") }
			}
			pub fn set_focus_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "FocusDistance", value) }
			}
			pub fn in_focus_radius(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "InFocusRadius") }
			}
			pub fn set_in_focus_radius(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "InFocusRadius", value) }
			}
			pub fn near_intensity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "NearIntensity") }
			}
			pub fn set_near_intensity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "NearIntensity", value) }
			}
		}
		impl From<$name> for PostEffect {
			fn from(value: $name) -> PostEffect {
				PostEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_sun_rays_effect {
	($name:ident) => {
		impl_post_effect!($name);
		impl $name {
			pub fn intensity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Intensity") }
			}
			pub fn set_intensity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Intensity", value) }
			}
			pub fn spread(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Spread") }
			}
			pub fn set_spread(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Spread", value) }
			}
		}
		impl From<$name> for PostEffect {
			fn from(value: $name) -> PostEffect {
				PostEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_proximity_prompt {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn action_text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "ActionText") }
			}
			pub fn set_action_text(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "ActionText", &value) }
			}
			pub fn auto_localize(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AutoLocalize") }
			}
			pub fn set_auto_localize(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AutoLocalize", value) }
			}
			pub fn clickable_prompt(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ClickablePrompt") }
			}
			pub fn set_clickable_prompt(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ClickablePrompt", value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn hold_duration(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "HoldDuration") }
			}
			pub fn set_hold_duration(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "HoldDuration", value) }
			}
			pub fn max_activation_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxActivationDistance") }
			}
			pub fn set_max_activation_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxActivationDistance", value) }
			}
			pub fn object_text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "ObjectText") }
			}
			pub fn set_object_text(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "ObjectText", &value) }
			}
			pub fn requires_line_of_sight(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "RequiresLineOfSight") }
			}
			pub fn set_requires_line_of_sight(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "RequiresLineOfSight", value) }
			}
			pub fn root_localization_table(&self) -> Option<LocalizationTable> {
				unsafe { get_instance_property(self.to_ptr(), "RootLocalizationTable").map(|id| LocalizationTable(id)) }
			}
			pub fn set_root_localization_table(&self, value: Option<LocalizationTable>) {
				unsafe { set_instance_property(self.to_ptr(), "RootLocalizationTable", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn ui_offset(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "UIOffset")) }
			}
			pub fn set_ui_offset(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "UIOffset", value.to_ptr()) }
			}
			pub fn fn_input_hold_begin(&self) {
				unsafe { dyn_proximity_prompt_input_hold_begin(self.to_ptr(), "InputHoldBegin") }
			}
			pub fn fn_input_hold_end(&self) {
				unsafe { dyn_proximity_prompt_input_hold_end(self.to_ptr(), "InputHoldEnd") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_remote_event {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_remote_function {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_replicated_first {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_remove_default_loading_screen(&self) {
				unsafe { dyn_replicated_first_remove_default_loading_screen(self.to_ptr(), "RemoveDefaultLoadingScreen") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_replicated_storage {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_rotation_curve {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Length") }
			}
			pub fn fn_get_key_at_index(&self, index: f64) -> RotationCurveKey {
				unsafe { dyn_rotation_curve_get_key_at_index(self.to_ptr(), "GetKeyAtIndex", index) }
			}
			pub fn fn_get_key_indices_at_time(&self, time: f64) {
				unsafe { dyn_rotation_curve_get_key_indices_at_time(self.to_ptr(), "GetKeyIndicesAtTime", time) }
			}
			pub fn fn_get_keys(&self) {
				unsafe { dyn_rotation_curve_get_keys(self.to_ptr(), "GetKeys") }
			}
			pub fn fn_get_value_at_time(&self, time: f64) -> Option<CFrame> {
				unsafe { dyn_rotation_curve_get_value_at_time(self.to_ptr(), "GetValueAtTime", time) }
			}
			pub fn fn_insert_key(&self, key: RotationCurveKey) {
				unsafe { dyn_rotation_curve_insert_key(self.to_ptr(), "InsertKey", key) }
			}
			pub fn fn_remove_key_at_index(&self, starting_index: f64, count: f64) -> f64 {
				unsafe { dyn_rotation_curve_remove_key_at_index(self.to_ptr(), "RemoveKeyAtIndex", starting_index, count) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_screenshot_hud {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn camera_button_icon(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "CameraButtonIcon")) }
			}
			pub fn set_camera_button_icon(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "CameraButtonIcon", value.to_ptr()) }
			}
			pub fn camera_button_position(&self) -> UDim2 {
				unsafe { UDim2(get_datatype_property(self.to_ptr(), "CameraButtonPosition")) }
			}
			pub fn set_camera_button_position(&self, value: UDim2) {
				unsafe { set_datatype_property(self.to_ptr(), "CameraButtonPosition", value.to_ptr()) }
			}
			pub fn close_button_position(&self) -> UDim2 {
				unsafe { UDim2(get_datatype_property(self.to_ptr(), "CloseButtonPosition")) }
			}
			pub fn set_close_button_position(&self, value: UDim2) {
				unsafe { set_datatype_property(self.to_ptr(), "CloseButtonPosition", value.to_ptr()) }
			}
			pub fn close_when_screenshot_taken(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CloseWhenScreenshotTaken") }
			}
			pub fn set_close_when_screenshot_taken(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "CloseWhenScreenshotTaken", value) }
			}
			pub fn experience_name_overlay_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ExperienceNameOverlayEnabled") }
			}
			pub fn set_experience_name_overlay_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ExperienceNameOverlayEnabled", value) }
			}
			pub fn username_overlay_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "UsernameOverlayEnabled") }
			}
			pub fn set_username_overlay_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "UsernameOverlayEnabled", value) }
			}
			pub fn visible(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Visible") }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Visible", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_server_script_service {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_server_storage {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_service_provider {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_find_service(&self, class_name: &str) -> Option<Instance> {
				unsafe { dyn_service_provider_find_service(self.to_ptr(), "FindService", class_name) }
			}
			pub fn fn_get_service(&self, class_name: &str) -> Option<Instance> {
				unsafe { dyn_service_provider_get_service(self.to_ptr(), "GetService", class_name) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_model {
	($name:ident) => {
		impl_service_provider!($name);
		impl $name {
			pub fn creator_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CreatorId") }
			}
			pub fn game_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "GameId") }
			}
			pub fn job_id(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "JobId") }
			}
			pub fn place_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "PlaceId") }
			}
			pub fn place_version(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "PlaceVersion") }
			}
			pub fn private_server_id(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "PrivateServerId") }
			}
			pub fn private_server_owner_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "PrivateServerOwnerId") }
			}
			#[deprecated]
			pub fn vip_server_id(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "VIPServerId") }
			}
			#[deprecated]
			pub fn vip_server_owner_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "VIPServerOwnerId") }
			}
			pub fn workspace(&self) -> Option<Workspace> {
				unsafe { get_instance_property(self.to_ptr(), "Workspace").map(|id| Workspace(id)) }
			}
			pub fn fn_bind_to_close(&self, function: Function) {
				unsafe { dyn_data_model_bind_to_close(self.to_ptr(), "BindToClose", function) }
			}
			pub fn fn_is_loaded(&self) -> bool {
				unsafe { dyn_data_model_is_loaded(self.to_ptr(), "IsLoaded") }
			}
		}
		impl From<$name> for ServiceProvider {
			fn from(value: $name) -> ServiceProvider {
				ServiceProvider(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_generic_settings {
	($name:ident) => {
		impl_service_provider!($name);
		impl $name {
		}
		impl From<$name> for ServiceProvider {
			fn from(value: $name) -> ServiceProvider {
				ServiceProvider(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_user_settings {
	($name:ident) => {
		impl_generic_settings!($name);
		impl $name {
			pub fn fn_is_user_feature_enabled(&self, name: &str) -> bool {
				unsafe { dyn_user_settings_is_user_feature_enabled(self.to_ptr(), "IsUserFeatureEnabled", name) }
			}
			pub fn fn_reset(&self) {
				unsafe { dyn_user_settings_reset(self.to_ptr(), "Reset") }
			}
		}
		impl From<$name> for GenericSettings {
			fn from(value: $name) -> GenericSettings {
				GenericSettings(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_sky {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn celestial_bodies_shown(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CelestialBodiesShown") }
			}
			pub fn set_celestial_bodies_shown(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "CelestialBodiesShown", value) }
			}
			pub fn moon_angular_size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MoonAngularSize") }
			}
			pub fn set_moon_angular_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MoonAngularSize", value) }
			}
			pub fn moon_texture_id(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "MoonTextureId")) }
			}
			pub fn set_moon_texture_id(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "MoonTextureId", value.to_ptr()) }
			}
			pub fn skybox_bk(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "SkyboxBk")) }
			}
			pub fn set_skybox_bk(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "SkyboxBk", value.to_ptr()) }
			}
			pub fn skybox_dn(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "SkyboxDn")) }
			}
			pub fn set_skybox_dn(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "SkyboxDn", value.to_ptr()) }
			}
			pub fn skybox_ft(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "SkyboxFt")) }
			}
			pub fn set_skybox_ft(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "SkyboxFt", value.to_ptr()) }
			}
			pub fn skybox_lf(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "SkyboxLf")) }
			}
			pub fn set_skybox_lf(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "SkyboxLf", value.to_ptr()) }
			}
			pub fn skybox_rt(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "SkyboxRt")) }
			}
			pub fn set_skybox_rt(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "SkyboxRt", value.to_ptr()) }
			}
			pub fn skybox_up(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "SkyboxUp")) }
			}
			pub fn set_skybox_up(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "SkyboxUp", value.to_ptr()) }
			}
			pub fn star_count(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "StarCount") }
			}
			pub fn set_star_count(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "StarCount", value) }
			}
			pub fn sun_angular_size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "SunAngularSize") }
			}
			pub fn set_sun_angular_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "SunAngularSize", value) }
			}
			pub fn sun_texture_id(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "SunTextureId")) }
			}
			pub fn set_sun_texture_id(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "SunTextureId", value.to_ptr()) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_smoke {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Color")) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn opacity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Opacity") }
			}
			pub fn set_opacity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Opacity", value) }
			}
			pub fn rise_velocity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "RiseVelocity") }
			}
			pub fn set_rise_velocity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "RiseVelocity", value) }
			}
			pub fn size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Size") }
			}
			pub fn set_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Size", value) }
			}
			pub fn time_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TimeScale") }
			}
			pub fn set_time_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TimeScale", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_sound {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			#[deprecated]
			pub fn emitter_size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "EmitterSize") }
			}
			#[deprecated]
			pub fn set_emitter_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "EmitterSize", value) }
			}
			pub fn is_loaded(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsLoaded") }
			}
			pub fn is_paused(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsPaused") }
			}
			pub fn is_playing(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsPlaying") }
			}
			pub fn looped(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Looped") }
			}
			pub fn set_looped(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Looped", value) }
			}
			#[deprecated]
			pub fn max_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxDistance") }
			}
			#[deprecated]
			pub fn set_max_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxDistance", value) }
			}
			#[deprecated]
			pub fn min_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MinDistance") }
			}
			#[deprecated]
			pub fn set_min_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MinDistance", value) }
			}
			#[deprecated]
			pub fn pitch(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Pitch") }
			}
			#[deprecated]
			pub fn set_pitch(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Pitch", value) }
			}
			pub fn play_on_remove(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "PlayOnRemove") }
			}
			pub fn set_play_on_remove(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "PlayOnRemove", value) }
			}
			pub fn playback_loudness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "PlaybackLoudness") }
			}
			pub fn playback_speed(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "PlaybackSpeed") }
			}
			pub fn set_playback_speed(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "PlaybackSpeed", value) }
			}
			pub fn playing(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Playing") }
			}
			pub fn set_playing(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Playing", value) }
			}
			pub fn roll_off_max_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "RollOffMaxDistance") }
			}
			pub fn set_roll_off_max_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "RollOffMaxDistance", value) }
			}
			pub fn roll_off_min_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "RollOffMinDistance") }
			}
			pub fn set_roll_off_min_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "RollOffMinDistance", value) }
			}
			pub fn sound_group(&self) -> Option<SoundGroup> {
				unsafe { get_instance_property(self.to_ptr(), "SoundGroup").map(|id| SoundGroup(id)) }
			}
			pub fn set_sound_group(&self, value: Option<SoundGroup>) {
				unsafe { set_instance_property(self.to_ptr(), "SoundGroup", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn sound_id(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "SoundId")) }
			}
			pub fn set_sound_id(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "SoundId", value.to_ptr()) }
			}
			pub fn time_length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TimeLength") }
			}
			pub fn time_position(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TimePosition") }
			}
			pub fn set_time_position(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TimePosition", value) }
			}
			pub fn volume(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Volume") }
			}
			pub fn set_volume(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Volume", value) }
			}
			pub fn fn_pause(&self) {
				unsafe { dyn_sound_pause(self.to_ptr(), "Pause") }
			}
			pub fn fn_play(&self) {
				unsafe { dyn_sound_play(self.to_ptr(), "Play") }
			}
			pub fn fn_resume(&self) {
				unsafe { dyn_sound_resume(self.to_ptr(), "Resume") }
			}
			pub fn fn_stop(&self) {
				unsafe { dyn_sound_stop(self.to_ptr(), "Stop") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_sound_effect {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn priority(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Priority") }
			}
			pub fn set_priority(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Priority", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_chorus_sound_effect {
	($name:ident) => {
		impl_sound_effect!($name);
		impl $name {
			pub fn depth(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Depth") }
			}
			pub fn set_depth(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Depth", value) }
			}
			pub fn mix(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Mix") }
			}
			pub fn set_mix(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Mix", value) }
			}
			pub fn rate(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Rate") }
			}
			pub fn set_rate(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Rate", value) }
			}
		}
		impl From<$name> for SoundEffect {
			fn from(value: $name) -> SoundEffect {
				SoundEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_compressor_sound_effect {
	($name:ident) => {
		impl_sound_effect!($name);
		impl $name {
			pub fn attack(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Attack") }
			}
			pub fn set_attack(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Attack", value) }
			}
			pub fn gain_makeup(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "GainMakeup") }
			}
			pub fn set_gain_makeup(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "GainMakeup", value) }
			}
			pub fn ratio(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Ratio") }
			}
			pub fn set_ratio(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Ratio", value) }
			}
			pub fn release(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Release") }
			}
			pub fn set_release(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Release", value) }
			}
			pub fn side_chain(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "SideChain").map(|id| Instance(id)) }
			}
			pub fn set_side_chain(&self, value: Option<Instance>) {
				unsafe { set_instance_property(self.to_ptr(), "SideChain", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn threshold(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Threshold") }
			}
			pub fn set_threshold(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Threshold", value) }
			}
		}
		impl From<$name> for SoundEffect {
			fn from(value: $name) -> SoundEffect {
				SoundEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_custom_sound_effect {
	($name:ident) => {
		impl_sound_effect!($name);
		impl $name {
		}
		impl From<$name> for SoundEffect {
			fn from(value: $name) -> SoundEffect {
				SoundEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_channel_selector_sound_effect {
	($name:ident) => {
		impl_custom_sound_effect!($name);
		impl $name {
			pub fn channel(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Channel") }
			}
			pub fn set_channel(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Channel", value) }
			}
		}
		impl From<$name> for CustomSoundEffect {
			fn from(value: $name) -> CustomSoundEffect {
				CustomSoundEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_distortion_sound_effect {
	($name:ident) => {
		impl_sound_effect!($name);
		impl $name {
			pub fn level(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Level") }
			}
			pub fn set_level(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Level", value) }
			}
		}
		impl From<$name> for SoundEffect {
			fn from(value: $name) -> SoundEffect {
				SoundEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_echo_sound_effect {
	($name:ident) => {
		impl_sound_effect!($name);
		impl $name {
			pub fn delay(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Delay") }
			}
			pub fn set_delay(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Delay", value) }
			}
			pub fn dry_level(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DryLevel") }
			}
			pub fn set_dry_level(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DryLevel", value) }
			}
			pub fn feedback(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Feedback") }
			}
			pub fn set_feedback(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Feedback", value) }
			}
			pub fn wet_level(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WetLevel") }
			}
			pub fn set_wet_level(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WetLevel", value) }
			}
		}
		impl From<$name> for SoundEffect {
			fn from(value: $name) -> SoundEffect {
				SoundEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_equalizer_sound_effect {
	($name:ident) => {
		impl_sound_effect!($name);
		impl $name {
			pub fn high_gain(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "HighGain") }
			}
			pub fn set_high_gain(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "HighGain", value) }
			}
			pub fn low_gain(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LowGain") }
			}
			pub fn set_low_gain(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LowGain", value) }
			}
			pub fn mid_gain(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MidGain") }
			}
			pub fn set_mid_gain(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MidGain", value) }
			}
		}
		impl From<$name> for SoundEffect {
			fn from(value: $name) -> SoundEffect {
				SoundEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_flange_sound_effect {
	($name:ident) => {
		impl_sound_effect!($name);
		impl $name {
			pub fn depth(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Depth") }
			}
			pub fn set_depth(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Depth", value) }
			}
			pub fn mix(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Mix") }
			}
			pub fn set_mix(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Mix", value) }
			}
			pub fn rate(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Rate") }
			}
			pub fn set_rate(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Rate", value) }
			}
		}
		impl From<$name> for SoundEffect {
			fn from(value: $name) -> SoundEffect {
				SoundEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pitch_shift_sound_effect {
	($name:ident) => {
		impl_sound_effect!($name);
		impl $name {
			pub fn octave(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Octave") }
			}
			pub fn set_octave(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Octave", value) }
			}
		}
		impl From<$name> for SoundEffect {
			fn from(value: $name) -> SoundEffect {
				SoundEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_reverb_sound_effect {
	($name:ident) => {
		impl_sound_effect!($name);
		impl $name {
			pub fn decay_time(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DecayTime") }
			}
			pub fn set_decay_time(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DecayTime", value) }
			}
			pub fn density(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Density") }
			}
			pub fn set_density(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Density", value) }
			}
			pub fn diffusion(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Diffusion") }
			}
			pub fn set_diffusion(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Diffusion", value) }
			}
			pub fn dry_level(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "DryLevel") }
			}
			pub fn set_dry_level(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "DryLevel", value) }
			}
			pub fn wet_level(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "WetLevel") }
			}
			pub fn set_wet_level(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "WetLevel", value) }
			}
		}
		impl From<$name> for SoundEffect {
			fn from(value: $name) -> SoundEffect {
				SoundEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_tremolo_sound_effect {
	($name:ident) => {
		impl_sound_effect!($name);
		impl $name {
			pub fn depth(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Depth") }
			}
			pub fn set_depth(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Depth", value) }
			}
			pub fn duty(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Duty") }
			}
			pub fn set_duty(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Duty", value) }
			}
			pub fn frequency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Frequency") }
			}
			pub fn set_frequency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Frequency", value) }
			}
		}
		impl From<$name> for SoundEffect {
			fn from(value: $name) -> SoundEffect {
				SoundEffect(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_sound_group {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn volume(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Volume") }
			}
			pub fn set_volume(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Volume", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_sparkles {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Color")) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn sparkle_color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "SparkleColor")) }
			}
			pub fn set_sparkle_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "SparkleColor", value.to_ptr()) }
			}
			pub fn time_scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TimeScale") }
			}
			pub fn set_time_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TimeScale", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_speaker {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn channel_count(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ChannelCount") }
			}
			pub fn playback_loudness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "PlaybackLoudness") }
			}
			pub fn roll_off_max_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "RollOffMaxDistance") }
			}
			pub fn set_roll_off_max_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "RollOffMaxDistance", value) }
			}
			pub fn roll_off_min_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "RollOffMinDistance") }
			}
			pub fn set_roll_off_min_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "RollOffMinDistance", value) }
			}
			pub fn sound_group(&self) -> Option<SoundGroup> {
				unsafe { get_instance_property(self.to_ptr(), "SoundGroup").map(|id| SoundGroup(id)) }
			}
			pub fn set_sound_group(&self, value: Option<SoundGroup>) {
				unsafe { set_instance_property(self.to_ptr(), "SoundGroup", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn source(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "Source").map(|id| Instance(id)) }
			}
			pub fn set_source(&self, value: Option<Instance>) {
				unsafe { set_instance_property(self.to_ptr(), "Source", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn volume(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Volume") }
			}
			pub fn set_volume(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Volume", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_starter_gear {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_starter_pack {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_starter_player {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn allow_custom_animations(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AllowCustomAnimations") }
			}
			pub fn auto_jump_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AutoJumpEnabled") }
			}
			pub fn set_auto_jump_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AutoJumpEnabled", value) }
			}
			pub fn camera_max_zoom_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CameraMaxZoomDistance") }
			}
			pub fn set_camera_max_zoom_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CameraMaxZoomDistance", value) }
			}
			pub fn camera_min_zoom_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CameraMinZoomDistance") }
			}
			pub fn set_camera_min_zoom_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CameraMinZoomDistance", value) }
			}
			pub fn character_jump_height(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CharacterJumpHeight") }
			}
			pub fn set_character_jump_height(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CharacterJumpHeight", value) }
			}
			pub fn character_jump_power(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CharacterJumpPower") }
			}
			pub fn set_character_jump_power(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CharacterJumpPower", value) }
			}
			pub fn character_max_slope_angle(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CharacterMaxSlopeAngle") }
			}
			pub fn set_character_max_slope_angle(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CharacterMaxSlopeAngle", value) }
			}
			pub fn character_use_jump_power(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CharacterUseJumpPower") }
			}
			pub fn set_character_use_jump_power(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "CharacterUseJumpPower", value) }
			}
			pub fn character_walk_speed(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "CharacterWalkSpeed") }
			}
			pub fn set_character_walk_speed(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "CharacterWalkSpeed", value) }
			}
			pub fn enable_mouse_lock_option(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "EnableMouseLockOption") }
			}
			pub fn set_enable_mouse_lock_option(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "EnableMouseLockOption", value) }
			}
			pub fn health_display_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "HealthDisplayDistance") }
			}
			pub fn set_health_display_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "HealthDisplayDistance", value) }
			}
			pub fn load_character_appearance(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "LoadCharacterAppearance") }
			}
			pub fn set_load_character_appearance(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "LoadCharacterAppearance", value) }
			}
			pub fn name_display_distance(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "NameDisplayDistance") }
			}
			pub fn set_name_display_distance(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "NameDisplayDistance", value) }
			}
			pub fn user_emotes_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "UserEmotesEnabled") }
			}
			pub fn set_user_emotes_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "UserEmotesEnabled", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_starter_player_scripts {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_starter_character_scripts {
	($name:ident) => {
		impl_starter_player_scripts!($name);
		impl $name {
		}
		impl From<$name> for StarterPlayerScripts {
			fn from(value: $name) -> StarterPlayerScripts {
				StarterPlayerScripts(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_surface_appearance {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_team {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn auto_assignable(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AutoAssignable") }
			}
			pub fn set_auto_assignable(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AutoAssignable", value) }
			}
			#[deprecated]
			pub fn auto_color_characters(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "AutoColorCharacters") }
			}
			#[deprecated]
			pub fn set_auto_color_characters(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "AutoColorCharacters", value) }
			}
			#[deprecated]
			pub fn score(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Score") }
			}
			#[deprecated]
			pub fn set_score(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Score", value) }
			}
			pub fn team_color(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "TeamColor")) }
			}
			pub fn set_team_color(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "TeamColor", value.to_ptr()) }
			}
			pub fn fn_get_players(&self) -> Objects {
				unsafe { dyn_team_get_players(self.to_ptr(), "GetPlayers") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_teams {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_get_teams(&self) -> Objects {
				unsafe { dyn_teams_get_teams(self.to_ptr(), "GetTeams") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_teleport_async_result {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn private_server_id(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "PrivateServerId") }
			}
			pub fn reserved_server_access_code(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "ReservedServerAccessCode") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_teleport_options {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn reserved_server_access_code(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "ReservedServerAccessCode") }
			}
			pub fn set_reserved_server_access_code(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "ReservedServerAccessCode", &value) }
			}
			pub fn server_instance_id(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "ServerInstanceId") }
			}
			pub fn set_server_instance_id(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "ServerInstanceId", &value) }
			}
			pub fn should_reserve_server(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ShouldReserveServer") }
			}
			pub fn set_should_reserve_server(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ShouldReserveServer", value) }
			}
			pub fn fn_get_teleport_data(&self) {
				unsafe { dyn_teleport_options_get_teleport_data(self.to_ptr(), "GetTeleportData") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_terrain_detail {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn studs_per_tile(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "StudsPerTile") }
			}
			pub fn set_studs_per_tile(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "StudsPerTile", value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_terrain_region {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			#[deprecated]
			pub fn is_smooth(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "IsSmooth") }
			}
			pub fn size_in_cells(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "SizeInCells")) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_channel {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_display_system_message(&self, system_message: &str, metadata: &str) -> Option<TextChatMessage> {
				unsafe { dyn_text_channel_display_system_message(self.to_ptr(), "DisplaySystemMessage", system_message, metadata) }
			}
			pub fn fn_add_user_async(&self, user_id: f64) {
				unsafe { dyn_text_channel_add_user_async(self.to_ptr(), "AddUserAsync", user_id) }
			}
			pub fn fn_send_async(&self, message: &str, metadata: &str) -> Option<TextChatMessage> {
				unsafe { dyn_text_channel_send_async(self.to_ptr(), "SendAsync", message, metadata) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_chat_command {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn primary_alias(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "PrimaryAlias") }
			}
			pub fn set_primary_alias(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "PrimaryAlias", &value) }
			}
			pub fn secondary_alias(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "SecondaryAlias") }
			}
			pub fn set_secondary_alias(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "SecondaryAlias", &value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_chat_configurations {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_chat_input_bar_configuration {
	($name:ident) => {
		impl_text_chat_configurations!($name);
		impl $name {
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn target_text_channel(&self) -> Option<TextChannel> {
				unsafe { get_instance_property(self.to_ptr(), "TargetTextChannel").map(|id| TextChannel(id)) }
			}
			pub fn set_target_text_channel(&self, value: Option<TextChannel>) {
				unsafe { set_instance_property(self.to_ptr(), "TargetTextChannel", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
		}
		impl From<$name> for TextChatConfigurations {
			fn from(value: $name) -> TextChatConfigurations {
				TextChatConfigurations(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_chat_window_configuration {
	($name:ident) => {
		impl_text_chat_configurations!($name);
		impl $name {
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
		}
		impl From<$name> for TextChatConfigurations {
			fn from(value: $name) -> TextChatConfigurations {
				TextChatConfigurations(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_chat_message {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn message_id(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "MessageId") }
			}
			pub fn set_message_id(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "MessageId", &value) }
			}
			pub fn metadata(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Metadata") }
			}
			pub fn set_metadata(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "Metadata", &value) }
			}
			pub fn prefix_text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "PrefixText") }
			}
			pub fn set_prefix_text(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "PrefixText", &value) }
			}
			pub fn text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Text") }
			}
			pub fn set_text(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "Text", &value) }
			}
			pub fn text_channel(&self) -> Option<TextChannel> {
				unsafe { get_instance_property(self.to_ptr(), "TextChannel").map(|id| TextChannel(id)) }
			}
			pub fn set_text_channel(&self, value: Option<TextChannel>) {
				unsafe { set_instance_property(self.to_ptr(), "TextChannel", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn text_source(&self) -> Option<TextSource> {
				unsafe { get_instance_property(self.to_ptr(), "TextSource").map(|id| TextSource(id)) }
			}
			pub fn set_text_source(&self, value: Option<TextSource>) {
				unsafe { set_instance_property(self.to_ptr(), "TextSource", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn timestamp(&self) -> DateTime {
				unsafe { DateTime(get_datatype_property(self.to_ptr(), "Timestamp")) }
			}
			pub fn set_timestamp(&self, value: DateTime) {
				unsafe { set_datatype_property(self.to_ptr(), "Timestamp", value.to_ptr()) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_chat_message_properties {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn prefix_text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "PrefixText") }
			}
			pub fn set_prefix_text(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "PrefixText", &value) }
			}
			pub fn text(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Text") }
			}
			pub fn set_text(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "Text", &value) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_filter_result {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_get_chat_for_user_async(&self, to_user_id: f64) -> String {
				unsafe { dyn_text_filter_result_get_chat_for_user_async(self.to_ptr(), "GetChatForUserAsync", to_user_id) }
			}
			pub fn fn_get_non_chat_string_for_broadcast_async(&self) -> String {
				unsafe { dyn_text_filter_result_get_non_chat_string_for_broadcast_async(self.to_ptr(), "GetNonChatStringForBroadcastAsync") }
			}
			pub fn fn_get_non_chat_string_for_user_async(&self, to_user_id: f64) -> String {
				unsafe { dyn_text_filter_result_get_non_chat_string_for_user_async(self.to_ptr(), "GetNonChatStringForUserAsync", to_user_id) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_source {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn can_send(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "CanSend") }
			}
			pub fn set_can_send(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "CanSend", value) }
			}
			pub fn user_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "UserId") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_touch_transmitter {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_trail {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn attachment_0(&self) -> Option<Attachment> {
				unsafe { get_instance_property(self.to_ptr(), "Attachment0").map(|id| Attachment(id)) }
			}
			pub fn set_attachment_0(&self, value: Option<Attachment>) {
				unsafe { set_instance_property(self.to_ptr(), "Attachment0", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn attachment_1(&self) -> Option<Attachment> {
				unsafe { get_instance_property(self.to_ptr(), "Attachment1").map(|id| Attachment(id)) }
			}
			pub fn set_attachment_1(&self, value: Option<Attachment>) {
				unsafe { set_instance_property(self.to_ptr(), "Attachment1", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn brightness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Brightness") }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Brightness", value) }
			}
			pub fn color(&self) -> ColorSequence {
				unsafe { ColorSequence(get_datatype_property(self.to_ptr(), "Color")) }
			}
			pub fn set_color(&self, value: ColorSequence) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn face_camera(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "FaceCamera") }
			}
			pub fn set_face_camera(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "FaceCamera", value) }
			}
			pub fn lifetime(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Lifetime") }
			}
			pub fn set_lifetime(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Lifetime", value) }
			}
			pub fn light_emission(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LightEmission") }
			}
			pub fn set_light_emission(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LightEmission", value) }
			}
			pub fn light_influence(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "LightInfluence") }
			}
			pub fn set_light_influence(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "LightInfluence", value) }
			}
			pub fn max_length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxLength") }
			}
			pub fn set_max_length(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxLength", value) }
			}
			pub fn min_length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MinLength") }
			}
			pub fn set_min_length(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MinLength", value) }
			}
			pub fn texture(&self) -> Content {
				unsafe { Content(get_datatype_property(self.to_ptr(), "Texture")) }
			}
			pub fn set_texture(&self, value: Content) {
				unsafe { set_datatype_property(self.to_ptr(), "Texture", value.to_ptr()) }
			}
			pub fn texture_length(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TextureLength") }
			}
			pub fn set_texture_length(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TextureLength", value) }
			}
			pub fn transparency(&self) -> NumberSequence {
				unsafe { NumberSequence(get_datatype_property(self.to_ptr(), "Transparency")) }
			}
			pub fn set_transparency(&self, value: NumberSequence) {
				unsafe { set_datatype_property(self.to_ptr(), "Transparency", value.to_ptr()) }
			}
			pub fn width_scale(&self) -> NumberSequence {
				unsafe { NumberSequence(get_datatype_property(self.to_ptr(), "WidthScale")) }
			}
			pub fn set_width_scale(&self, value: NumberSequence) {
				unsafe { set_datatype_property(self.to_ptr(), "WidthScale", value.to_ptr()) }
			}
			pub fn fn_clear(&self) {
				unsafe { dyn_trail_clear(self.to_ptr(), "Clear") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_translator {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn locale_id(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "LocaleId") }
			}
			pub fn fn_translate(&self, context: Option<Instance>, text: &str) -> String {
				unsafe { dyn_translator_translate(self.to_ptr(), "Translate", context, text) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_tween_base {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_cancel(&self) {
				unsafe { dyn_tween_base_cancel(self.to_ptr(), "Cancel") }
			}
			pub fn fn_pause(&self) {
				unsafe { dyn_tween_base_pause(self.to_ptr(), "Pause") }
			}
			pub fn fn_play(&self) {
				unsafe { dyn_tween_base_play(self.to_ptr(), "Play") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_tween {
	($name:ident) => {
		impl_tween_base!($name);
		impl $name {
			pub fn instance(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "Instance").map(|id| Instance(id)) }
			}
			pub fn tween_info(&self) -> TweenInfo {
				unsafe { TweenInfo(get_datatype_property(self.to_ptr(), "TweenInfo")) }
			}
		}
		impl From<$name> for TweenBase {
			fn from(value: $name) -> TweenBase {
				TweenBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_base {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_component {
	($name:ident) => {
		impl_ui_base!($name);
		impl $name {
		}
		impl From<$name> for UIBase {
			fn from(value: $name) -> UIBase {
				UIBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_constraint {
	($name:ident) => {
		impl_ui_component!($name);
		impl $name {
		}
		impl From<$name> for UIComponent {
			fn from(value: $name) -> UIComponent {
				UIComponent(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_aspect_ratio_constraint {
	($name:ident) => {
		impl_ui_constraint!($name);
		impl $name {
			pub fn aspect_ratio(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "AspectRatio") }
			}
			pub fn set_aspect_ratio(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "AspectRatio", value) }
			}
		}
		impl From<$name> for UIConstraint {
			fn from(value: $name) -> UIConstraint {
				UIConstraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_size_constraint {
	($name:ident) => {
		impl_ui_constraint!($name);
		impl $name {
			pub fn max_size(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "MaxSize")) }
			}
			pub fn set_max_size(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "MaxSize", value.to_ptr()) }
			}
			pub fn min_size(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "MinSize")) }
			}
			pub fn set_min_size(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "MinSize", value.to_ptr()) }
			}
		}
		impl From<$name> for UIConstraint {
			fn from(value: $name) -> UIConstraint {
				UIConstraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_text_size_constraint {
	($name:ident) => {
		impl_ui_constraint!($name);
		impl $name {
			pub fn max_text_size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxTextSize") }
			}
			pub fn set_max_text_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxTextSize", value) }
			}
			pub fn min_text_size(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MinTextSize") }
			}
			pub fn set_min_text_size(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MinTextSize", value) }
			}
		}
		impl From<$name> for UIConstraint {
			fn from(value: $name) -> UIConstraint {
				UIConstraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_corner {
	($name:ident) => {
		impl_ui_component!($name);
		impl $name {
			pub fn corner_radius(&self) -> UDim {
				unsafe { UDim(get_datatype_property(self.to_ptr(), "CornerRadius")) }
			}
			pub fn set_corner_radius(&self, value: UDim) {
				unsafe { set_datatype_property(self.to_ptr(), "CornerRadius", value.to_ptr()) }
			}
		}
		impl From<$name> for UIComponent {
			fn from(value: $name) -> UIComponent {
				UIComponent(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_gradient {
	($name:ident) => {
		impl_ui_component!($name);
		impl $name {
			pub fn color(&self) -> ColorSequence {
				unsafe { ColorSequence(get_datatype_property(self.to_ptr(), "Color")) }
			}
			pub fn set_color(&self, value: ColorSequence) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn offset(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "Offset")) }
			}
			pub fn set_offset(&self, value: Vector2) {
				unsafe { set_datatype_property(self.to_ptr(), "Offset", value.to_ptr()) }
			}
			pub fn rotation(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Rotation") }
			}
			pub fn set_rotation(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Rotation", value) }
			}
			pub fn transparency(&self) -> NumberSequence {
				unsafe { NumberSequence(get_datatype_property(self.to_ptr(), "Transparency")) }
			}
			pub fn set_transparency(&self, value: NumberSequence) {
				unsafe { set_datatype_property(self.to_ptr(), "Transparency", value.to_ptr()) }
			}
		}
		impl From<$name> for UIComponent {
			fn from(value: $name) -> UIComponent {
				UIComponent(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_layout {
	($name:ident) => {
		impl_ui_component!($name);
		impl $name {
		}
		impl From<$name> for UIComponent {
			fn from(value: $name) -> UIComponent {
				UIComponent(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_grid_style_layout {
	($name:ident) => {
		impl_ui_layout!($name);
		impl $name {
			pub fn absolute_content_size(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "AbsoluteContentSize")) }
			}
		}
		impl From<$name> for UILayout {
			fn from(value: $name) -> UILayout {
				UILayout(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_grid_layout {
	($name:ident) => {
		impl_ui_grid_style_layout!($name);
		impl $name {
			pub fn absolute_cell_count(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "AbsoluteCellCount")) }
			}
			pub fn absolute_cell_size(&self) -> Vector2 {
				unsafe { Vector2(get_datatype_property(self.to_ptr(), "AbsoluteCellSize")) }
			}
			pub fn cell_padding(&self) -> UDim2 {
				unsafe { UDim2(get_datatype_property(self.to_ptr(), "CellPadding")) }
			}
			pub fn set_cell_padding(&self, value: UDim2) {
				unsafe { set_datatype_property(self.to_ptr(), "CellPadding", value.to_ptr()) }
			}
			pub fn cell_size(&self) -> UDim2 {
				unsafe { UDim2(get_datatype_property(self.to_ptr(), "CellSize")) }
			}
			pub fn set_cell_size(&self, value: UDim2) {
				unsafe { set_datatype_property(self.to_ptr(), "CellSize", value.to_ptr()) }
			}
			pub fn fill_direction_max_cells(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "FillDirectionMaxCells") }
			}
			pub fn set_fill_direction_max_cells(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "FillDirectionMaxCells", value) }
			}
		}
		impl From<$name> for UIGridStyleLayout {
			fn from(value: $name) -> UIGridStyleLayout {
				UIGridStyleLayout(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_list_layout {
	($name:ident) => {
		impl_ui_grid_style_layout!($name);
		impl $name {
			pub fn padding(&self) -> UDim {
				unsafe { UDim(get_datatype_property(self.to_ptr(), "Padding")) }
			}
			pub fn set_padding(&self, value: UDim) {
				unsafe { set_datatype_property(self.to_ptr(), "Padding", value.to_ptr()) }
			}
		}
		impl From<$name> for UIGridStyleLayout {
			fn from(value: $name) -> UIGridStyleLayout {
				UIGridStyleLayout(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_page_layout {
	($name:ident) => {
		impl_ui_grid_style_layout!($name);
		impl $name {
			pub fn animated(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Animated") }
			}
			pub fn set_animated(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Animated", value) }
			}
			pub fn circular(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Circular") }
			}
			pub fn set_circular(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Circular", value) }
			}
			pub fn current_page(&self) -> Option<GuiObject> {
				unsafe { get_instance_property(self.to_ptr(), "CurrentPage").map(|id| GuiObject(id)) }
			}
			pub fn gamepad_input_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "GamepadInputEnabled") }
			}
			pub fn set_gamepad_input_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "GamepadInputEnabled", value) }
			}
			pub fn padding(&self) -> UDim {
				unsafe { UDim(get_datatype_property(self.to_ptr(), "Padding")) }
			}
			pub fn set_padding(&self, value: UDim) {
				unsafe { set_datatype_property(self.to_ptr(), "Padding", value.to_ptr()) }
			}
			pub fn scroll_wheel_input_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "ScrollWheelInputEnabled") }
			}
			pub fn set_scroll_wheel_input_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "ScrollWheelInputEnabled", value) }
			}
			pub fn touch_input_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "TouchInputEnabled") }
			}
			pub fn set_touch_input_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "TouchInputEnabled", value) }
			}
			pub fn tween_time(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "TweenTime") }
			}
			pub fn set_tween_time(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "TweenTime", value) }
			}
			pub fn fn_jump_to(&self, page: Option<Instance>) {
				unsafe { dyn_ui_page_layout_jump_to(self.to_ptr(), "JumpTo", page) }
			}
			pub fn fn_jump_to_index(&self, index: f64) {
				unsafe { dyn_ui_page_layout_jump_to_index(self.to_ptr(), "JumpToIndex", index) }
			}
			pub fn fn_next(&self) {
				unsafe { dyn_ui_page_layout_next(self.to_ptr(), "Next") }
			}
			pub fn fn_previous(&self) {
				unsafe { dyn_ui_page_layout_previous(self.to_ptr(), "Previous") }
			}
		}
		impl From<$name> for UIGridStyleLayout {
			fn from(value: $name) -> UIGridStyleLayout {
				UIGridStyleLayout(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_table_layout {
	($name:ident) => {
		impl_ui_grid_style_layout!($name);
		impl $name {
			pub fn fill_empty_space_columns(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "FillEmptySpaceColumns") }
			}
			pub fn set_fill_empty_space_columns(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "FillEmptySpaceColumns", value) }
			}
			pub fn fill_empty_space_rows(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "FillEmptySpaceRows") }
			}
			pub fn set_fill_empty_space_rows(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "FillEmptySpaceRows", value) }
			}
			pub fn padding(&self) -> UDim2 {
				unsafe { UDim2(get_datatype_property(self.to_ptr(), "Padding")) }
			}
			pub fn set_padding(&self, value: UDim2) {
				unsafe { set_datatype_property(self.to_ptr(), "Padding", value.to_ptr()) }
			}
		}
		impl From<$name> for UIGridStyleLayout {
			fn from(value: $name) -> UIGridStyleLayout {
				UIGridStyleLayout(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_padding {
	($name:ident) => {
		impl_ui_component!($name);
		impl $name {
			pub fn padding_bottom(&self) -> UDim {
				unsafe { UDim(get_datatype_property(self.to_ptr(), "PaddingBottom")) }
			}
			pub fn set_padding_bottom(&self, value: UDim) {
				unsafe { set_datatype_property(self.to_ptr(), "PaddingBottom", value.to_ptr()) }
			}
			pub fn padding_left(&self) -> UDim {
				unsafe { UDim(get_datatype_property(self.to_ptr(), "PaddingLeft")) }
			}
			pub fn set_padding_left(&self, value: UDim) {
				unsafe { set_datatype_property(self.to_ptr(), "PaddingLeft", value.to_ptr()) }
			}
			pub fn padding_right(&self) -> UDim {
				unsafe { UDim(get_datatype_property(self.to_ptr(), "PaddingRight")) }
			}
			pub fn set_padding_right(&self, value: UDim) {
				unsafe { set_datatype_property(self.to_ptr(), "PaddingRight", value.to_ptr()) }
			}
			pub fn padding_top(&self) -> UDim {
				unsafe { UDim(get_datatype_property(self.to_ptr(), "PaddingTop")) }
			}
			pub fn set_padding_top(&self, value: UDim) {
				unsafe { set_datatype_property(self.to_ptr(), "PaddingTop", value.to_ptr()) }
			}
		}
		impl From<$name> for UIComponent {
			fn from(value: $name) -> UIComponent {
				UIComponent(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_scale {
	($name:ident) => {
		impl_ui_component!($name);
		impl $name {
			pub fn scale(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Scale") }
			}
			pub fn set_scale(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Scale", value) }
			}
		}
		impl From<$name> for UIComponent {
			fn from(value: $name) -> UIComponent {
				UIComponent(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_stroke {
	($name:ident) => {
		impl_ui_component!($name);
		impl $name {
			pub fn color(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Color")) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Color", value.to_ptr()) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn thickness(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Thickness") }
			}
			pub fn set_thickness(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Thickness", value) }
			}
			pub fn transparency(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Transparency") }
			}
			pub fn set_transparency(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Transparency", value) }
			}
		}
		impl From<$name> for UIComponent {
			fn from(value: $name) -> UIComponent {
				UIComponent(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_user_game_settings {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn gamepad_camera_sensitivity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "GamepadCameraSensitivity") }
			}
			pub fn set_gamepad_camera_sensitivity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "GamepadCameraSensitivity", value) }
			}
			pub fn mouse_sensitivity(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MouseSensitivity") }
			}
			pub fn set_mouse_sensitivity(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MouseSensitivity", value) }
			}
			pub fn rcc_profiler_record_frame_rate(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "RCCProfilerRecordFrameRate") }
			}
			pub fn set_rcc_profiler_record_frame_rate(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "RCCProfilerRecordFrameRate", value) }
			}
			pub fn rcc_profiler_record_time_frame(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "RCCProfilerRecordTimeFrame") }
			}
			pub fn set_rcc_profiler_record_time_frame(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "RCCProfilerRecordTimeFrame", value) }
			}
			pub fn vignette_enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "VignetteEnabled") }
			}
			pub fn fn_get_camera_y_invert_value(&self) -> f64 {
				unsafe { dyn_user_game_settings_get_camera_y_invert_value(self.to_ptr(), "GetCameraYInvertValue") }
			}
			pub fn fn_get_onboarding_completed(&self, onboarding_id: &str) -> bool {
				unsafe { dyn_user_game_settings_get_onboarding_completed(self.to_ptr(), "GetOnboardingCompleted", onboarding_id) }
			}
			pub fn fn_in_full_screen(&self) -> bool {
				unsafe { dyn_user_game_settings_in_full_screen(self.to_ptr(), "InFullScreen") }
			}
			pub fn fn_in_studio_mode(&self) -> bool {
				unsafe { dyn_user_game_settings_in_studio_mode(self.to_ptr(), "InStudioMode") }
			}
			pub fn fn_set_camera_y_invert_visible(&self) {
				unsafe { dyn_user_game_settings_set_camera_y_invert_visible(self.to_ptr(), "SetCameraYInvertVisible") }
			}
			pub fn fn_set_gamepad_camera_sensitivity_visible(&self) {
				unsafe { dyn_user_game_settings_set_gamepad_camera_sensitivity_visible(self.to_ptr(), "SetGamepadCameraSensitivityVisible") }
			}
			pub fn fn_set_onboarding_completed(&self, onboarding_id: &str) {
				unsafe { dyn_user_game_settings_set_onboarding_completed(self.to_ptr(), "SetOnboardingCompleted", onboarding_id) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_value_base {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_bool_value {
	($name:ident) => {
		impl_value_base!($name);
		impl $name {
			pub fn value(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Value") }
			}
			pub fn set_value(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Value", value) }
			}
		}
		impl From<$name> for ValueBase {
			fn from(value: $name) -> ValueBase {
				ValueBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_brick_color_value {
	($name:ident) => {
		impl_value_base!($name);
		impl $name {
			pub fn value(&self) -> BrickColor {
				unsafe { BrickColor(get_datatype_property(self.to_ptr(), "Value")) }
			}
			pub fn set_value(&self, value: BrickColor) {
				unsafe { set_datatype_property(self.to_ptr(), "Value", value.to_ptr()) }
			}
		}
		impl From<$name> for ValueBase {
			fn from(value: $name) -> ValueBase {
				ValueBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_c_frame_value {
	($name:ident) => {
		impl_value_base!($name);
		impl $name {
			pub fn value(&self) -> CFrame {
				unsafe { CFrame(get_datatype_property(self.to_ptr(), "Value")) }
			}
			pub fn set_value(&self, value: CFrame) {
				unsafe { set_datatype_property(self.to_ptr(), "Value", value.to_ptr()) }
			}
		}
		impl From<$name> for ValueBase {
			fn from(value: $name) -> ValueBase {
				ValueBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_color_3_value {
	($name:ident) => {
		impl_value_base!($name);
		impl $name {
			pub fn value(&self) -> Color3 {
				unsafe { Color3(get_datatype_property(self.to_ptr(), "Value")) }
			}
			pub fn set_value(&self, value: Color3) {
				unsafe { set_datatype_property(self.to_ptr(), "Value", value.to_ptr()) }
			}
		}
		impl From<$name> for ValueBase {
			fn from(value: $name) -> ValueBase {
				ValueBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_double_constrained_value {
	($name:ident) => {
		impl_value_base!($name);
		impl $name {
			pub fn constrained_value(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ConstrainedValue") }
			}
			pub fn set_constrained_value(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ConstrainedValue", value) }
			}
			pub fn max_value(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxValue") }
			}
			pub fn set_max_value(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxValue", value) }
			}
			pub fn min_value(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MinValue") }
			}
			pub fn set_min_value(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MinValue", value) }
			}
			pub fn value(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Value") }
			}
			pub fn set_value(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Value", value) }
			}
		}
		impl From<$name> for ValueBase {
			fn from(value: $name) -> ValueBase {
				ValueBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_int_constrained_value {
	($name:ident) => {
		impl_value_base!($name);
		impl $name {
			pub fn constrained_value(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "ConstrainedValue") }
			}
			pub fn set_constrained_value(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "ConstrainedValue", value) }
			}
			pub fn max_value(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MaxValue") }
			}
			pub fn set_max_value(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MaxValue", value) }
			}
			pub fn min_value(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "MinValue") }
			}
			pub fn set_min_value(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "MinValue", value) }
			}
			pub fn value(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Value") }
			}
			pub fn set_value(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Value", value) }
			}
		}
		impl From<$name> for ValueBase {
			fn from(value: $name) -> ValueBase {
				ValueBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_int_value {
	($name:ident) => {
		impl_value_base!($name);
		impl $name {
			pub fn value(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Value") }
			}
			pub fn set_value(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Value", value) }
			}
		}
		impl From<$name> for ValueBase {
			fn from(value: $name) -> ValueBase {
				ValueBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_number_value {
	($name:ident) => {
		impl_value_base!($name);
		impl $name {
			pub fn value(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "Value") }
			}
			pub fn set_value(&self, value: f64) {
				unsafe { set_float_property(self.to_ptr(), "Value", value) }
			}
		}
		impl From<$name> for ValueBase {
			fn from(value: $name) -> ValueBase {
				ValueBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_object_value {
	($name:ident) => {
		impl_value_base!($name);
		impl $name {
			pub fn value(&self) -> Option<Instance> {
				unsafe { get_instance_property(self.to_ptr(), "Value").map(|id| Instance(id)) }
			}
			pub fn set_value(&self, value: Option<Instance>) {
				unsafe { set_instance_property(self.to_ptr(), "Value", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
		}
		impl From<$name> for ValueBase {
			fn from(value: $name) -> ValueBase {
				ValueBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ray_value {
	($name:ident) => {
		impl_value_base!($name);
		impl $name {
			pub fn value(&self) -> Ray {
				unsafe { Ray(get_datatype_property(self.to_ptr(), "Value")) }
			}
			pub fn set_value(&self, value: Ray) {
				unsafe { set_datatype_property(self.to_ptr(), "Value", value.to_ptr()) }
			}
		}
		impl From<$name> for ValueBase {
			fn from(value: $name) -> ValueBase {
				ValueBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_string_value {
	($name:ident) => {
		impl_value_base!($name);
		impl $name {
			pub fn value(&self) -> String {
				unsafe { get_string_property(self.to_ptr(), "Value") }
			}
			pub fn set_value(&self, value: &str) {
				unsafe { set_string_property(self.to_ptr(), "Value", &value) }
			}
		}
		impl From<$name> for ValueBase {
			fn from(value: $name) -> ValueBase {
				ValueBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_vector_3_value {
	($name:ident) => {
		impl_value_base!($name);
		impl $name {
			pub fn value(&self) -> Vector3 {
				unsafe { Vector3(get_datatype_property(self.to_ptr(), "Value")) }
			}
			pub fn set_value(&self, value: Vector3) {
				unsafe { set_datatype_property(self.to_ptr(), "Value", value.to_ptr()) }
			}
		}
		impl From<$name> for ValueBase {
			fn from(value: $name) -> ValueBase {
				ValueBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_vector_3_curve {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn fn_get_value_at_time(&self, time: f64) {
				unsafe { dyn_vector_3_curve_get_value_at_time(self.to_ptr(), "GetValueAtTime", time) }
			}
			pub fn fn_x(&self) -> Option<FloatCurve> {
				unsafe { dyn_vector_3_curve_x(self.to_ptr(), "X") }
			}
			pub fn fn_y(&self) -> Option<FloatCurve> {
				unsafe { dyn_vector_3_curve_y(self.to_ptr(), "Y") }
			}
			pub fn fn_z(&self) -> Option<FloatCurve> {
				unsafe { dyn_vector_3_curve_z(self.to_ptr(), "Z") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_voice_channel {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_voice_source {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn user_id(&self) -> f64 {
				unsafe { get_float_property(self.to_ptr(), "UserId") }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_weld_constraint {
	($name:ident) => {
		impl_instance!($name);
		impl $name {
			pub fn active(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Active") }
			}
			pub fn enabled(&self) -> bool {
				unsafe { get_bool_property(self.to_ptr(), "Enabled") }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { set_bool_property(self.to_ptr(), "Enabled", value) }
			}
			pub fn part_0(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "Part0").map(|id| BasePart(id)) }
			}
			pub fn set_part_0(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "Part0", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
			pub fn part_1(&self) -> Option<BasePart> {
				unsafe { get_instance_property(self.to_ptr(), "Part1").map(|id| BasePart(id)) }
			}
			pub fn set_part_1(&self, value: Option<BasePart>) {
				unsafe { set_instance_property(self.to_ptr(), "Part1", value.map(|v| v.to_ptr()).unwrap_or(0)) }
			}
		}
		impl_instance_exclusive!($name);
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
impl_instance!(Instance);
impl_accoutrement!(Accoutrement);
impl_accessory!(Accessory);
impl_hat!(Hat);
impl_animation!(Animation);
impl_animation_clip!(AnimationClip);
impl_curve_animation!(CurveAnimation);
impl_keyframe_sequence!(KeyframeSequence);
impl_animation_controller!(AnimationController);
impl_animation_rig_data!(AnimationRigData);
impl_animation_stream_track!(AnimationStreamTrack);
impl_animation_track!(AnimationTrack);
impl_animator!(Animator);
impl_atmosphere!(Atmosphere);
impl_attachment!(Attachment);
impl_bone!(Bone);
impl_backpack!(Backpack);
impl_backpack_item!(BackpackItem);
impl_tool!(Tool);
impl_base_player_gui!(BasePlayerGui);
impl_player_gui!(PlayerGui);
impl_starter_gui!(StarterGui);
impl_base_wrap!(BaseWrap);
impl_wrap_layer!(WrapLayer);
impl_wrap_target!(WrapTarget);
impl_beam!(Beam);
impl_bindable_event!(BindableEvent);
impl_bindable_function!(BindableFunction);
impl_body_mover!(BodyMover);
impl_body_angular_velocity!(BodyAngularVelocity);
impl_body_force!(BodyForce);
impl_body_gyro!(BodyGyro);
impl_body_position!(BodyPosition);
impl_body_thrust!(BodyThrust);
impl_body_velocity!(BodyVelocity);
impl_rocket_propulsion!(RocketPropulsion);
impl_camera!(Camera);
impl_character_appearance!(CharacterAppearance);
impl_body_colors!(BodyColors);
impl_character_mesh!(CharacterMesh);
impl_clothing!(Clothing);
impl_pants!(Pants);
impl_shirt!(Shirt);
impl_shirt_graphic!(ShirtGraphic);
impl_click_detector!(ClickDetector);
impl_clouds!(Clouds);
impl_command_instance!(CommandInstance);
impl_constraint!(Constraint);
impl_align_orientation!(AlignOrientation);
impl_align_position!(AlignPosition);
impl_angular_velocity!(AngularVelocity);
impl_ball_socket_constraint!(BallSocketConstraint);
impl_hinge_constraint!(HingeConstraint);
impl_line_force!(LineForce);
impl_linear_velocity!(LinearVelocity);
impl_plane_constraint!(PlaneConstraint);
impl_plane!(Plane);
impl_rigid_constraint!(RigidConstraint);
impl_rod_constraint!(RodConstraint);
impl_rope_constraint!(RopeConstraint);
impl_sliding_ball_constraint!(SlidingBallConstraint);
impl_cylindrical_constraint!(CylindricalConstraint);
impl_prismatic_constraint!(PrismaticConstraint);
impl_spring_constraint!(SpringConstraint);
impl_torque!(Torque);
impl_torsion_spring_constraint!(TorsionSpringConstraint);
impl_universal_constraint!(UniversalConstraint);
impl_vector_force!(VectorForce);
impl_controller!(Controller);
impl_humanoid_controller!(HumanoidController);
impl_skateboard_controller!(SkateboardController);
impl_vehicle_controller!(VehicleController);
impl_data_model_mesh!(DataModelMesh);
impl_bevel_mesh!(BevelMesh);
impl_block_mesh!(BlockMesh);
impl_cylinder_mesh!(CylinderMesh);
impl_file_mesh!(FileMesh);
impl_special_mesh!(SpecialMesh);
impl_data_store_increment_options!(DataStoreIncrementOptions);
impl_data_store_info!(DataStoreInfo);
impl_data_store_key!(DataStoreKey);
impl_data_store_key_info!(DataStoreKeyInfo);
impl_data_store_object_version_info!(DataStoreObjectVersionInfo);
impl_data_store_options!(DataStoreOptions);
impl_data_store_set_options!(DataStoreSetOptions);
impl_dialog!(Dialog);
impl_dialog_choice!(DialogChoice);
impl_dragger!(Dragger);
impl_euler_rotation_curve!(EulerRotationCurve);
impl_explosion!(Explosion);
impl_face_instance!(FaceInstance);
impl_decal!(Decal);
impl_texture!(Texture);
impl_feature!(Feature);
impl_fire!(Fire);
impl_float_curve!(FloatCurve);
impl_folder!(Folder);
impl_force_field!(ForceField);
impl_get_text_bounds_params!(GetTextBoundsParams);
impl_global_data_store!(GlobalDataStore);
impl_data_store!(DataStore);
impl_ordered_data_store!(OrderedDataStore);
impl_gui_base!(GuiBase);
impl_gui_base_2_d!(GuiBase2d);
impl_gui_object!(GuiObject);
impl_canvas_group!(CanvasGroup);
impl_frame!(Frame);
impl_gui_button!(GuiButton);
impl_image_button!(ImageButton);
impl_text_button!(TextButton);
impl_gui_label!(GuiLabel);
impl_image_label!(ImageLabel);
impl_text_label!(TextLabel);
impl_scrolling_frame!(ScrollingFrame);
impl_text_box!(TextBox);
impl_video_frame!(VideoFrame);
impl_viewport_frame!(ViewportFrame);
impl_layer_collector!(LayerCollector);
impl_billboard_gui!(BillboardGui);
impl_screen_gui!(ScreenGui);
impl_surface_gui!(SurfaceGui);
impl_gui_base_3_d!(GuiBase3d);
impl_floor_wire!(FloorWire);
impl_instance_adornment!(InstanceAdornment);
impl_selection_box!(SelectionBox);
impl_pv_adornment!(PVAdornment);
impl_handle_adornment!(HandleAdornment);
impl_box_handle_adornment!(BoxHandleAdornment);
impl_cone_handle_adornment!(ConeHandleAdornment);
impl_cylinder_handle_adornment!(CylinderHandleAdornment);
impl_image_handle_adornment!(ImageHandleAdornment);
impl_line_handle_adornment!(LineHandleAdornment);
impl_sphere_handle_adornment!(SphereHandleAdornment);
impl_parabola_adornment!(ParabolaAdornment);
impl_selection_sphere!(SelectionSphere);
impl_part_adornment!(PartAdornment);
impl_handles_base!(HandlesBase);
impl_arc_handles!(ArcHandles);
impl_handles!(Handles);
impl_surface_selection!(SurfaceSelection);
impl_selection_lasso!(SelectionLasso);
impl_selection_part_lasso!(SelectionPartLasso);
impl_selection_point_lasso!(SelectionPointLasso);
impl_highlight!(Highlight);
impl_humanoid!(Humanoid);
impl_humanoid_description!(HumanoidDescription);
impl_importer_base_settings!(ImporterBaseSettings);
impl_importer_group_settings!(ImporterGroupSettings);
impl_importer_material_settings!(ImporterMaterialSettings);
impl_importer_mesh_settings!(ImporterMeshSettings);
impl_importer_root_settings!(ImporterRootSettings);
impl_input_object!(InputObject);
impl_joint_instance!(JointInstance);
impl_dynamic_rotate!(DynamicRotate);
impl_glue!(Glue);
impl_motor!(Motor);
impl_motor_6_d!(Motor6D);
impl_velocity_motor!(VelocityMotor);
impl_keyframe!(Keyframe);
impl_keyframe_marker!(KeyframeMarker);
impl_light!(Light);
impl_point_light!(PointLight);
impl_spot_light!(SpotLight);
impl_surface_light!(SurfaceLight);
impl_lighting!(Lighting);
impl_localization_table!(LocalizationTable);
impl_lod_data_entity!(LodDataEntity);
impl_lua_source_container!(LuaSourceContainer);
impl_base_script!(BaseScript);
impl_script!(Script);
impl_local_script!(LocalScript);
impl_module_script!(ModuleScript);
impl_marker_curve!(MarkerCurve);
impl_material_variant!(MaterialVariant);
impl_memory_store_queue!(MemoryStoreQueue);
impl_memory_store_sorted_map!(MemoryStoreSortedMap);
impl_mouse!(Mouse);
impl_player_mouse!(PlayerMouse);
impl_network_marker!(NetworkMarker);
impl_no_collision_constraint!(NoCollisionConstraint);
impl_pv_instance!(PVInstance);
impl_base_part!(BasePart);
impl_corner_wedge_part!(CornerWedgePart);
impl_form_factor_part!(FormFactorPart);
impl_part!(Part);
impl_platform!(Platform);
impl_seat!(Seat);
impl_skateboard_platform!(SkateboardPlatform);
impl_spawn_location!(SpawnLocation);
impl_wedge_part!(WedgePart);
impl_terrain!(Terrain);
impl_triangle_mesh_part!(TriangleMeshPart);
impl_mesh_part!(MeshPart);
impl_part_operation!(PartOperation);
impl_negate_operation!(NegateOperation);
impl_union_operation!(UnionOperation);
impl_truss_part!(TrussPart);
impl_vehicle_seat!(VehicleSeat);
impl_model!(Model);
impl_actor!(Actor);
impl_world_root!(WorldRoot);
impl_workspace!(Workspace);
impl_world_model!(WorldModel);
impl_package_link!(PackageLink);
impl_pages!(Pages);
impl_catalog_pages!(CatalogPages);
impl_data_store_key_pages!(DataStoreKeyPages);
impl_data_store_listing_pages!(DataStoreListingPages);
impl_data_store_pages!(DataStorePages);
impl_data_store_version_pages!(DataStoreVersionPages);
impl_friend_pages!(FriendPages);
impl_inventory_pages!(InventoryPages);
impl_emotes_pages!(EmotesPages);
impl_outfit_pages!(OutfitPages);
impl_standard_pages!(StandardPages);
impl_particle_emitter!(ParticleEmitter);
impl_path!(Path);
impl_pathfinding_link!(PathfindingLink);
impl_pathfinding_modifier!(PathfindingModifier);
impl_player!(Player);
impl_player_scripts!(PlayerScripts);
impl_pose_base!(PoseBase);
impl_number_pose!(NumberPose);
impl_pose!(Pose);
impl_post_effect!(PostEffect);
impl_bloom_effect!(BloomEffect);
impl_blur_effect!(BlurEffect);
impl_color_correction_effect!(ColorCorrectionEffect);
impl_depth_of_field_effect!(DepthOfFieldEffect);
impl_sun_rays_effect!(SunRaysEffect);
impl_proximity_prompt!(ProximityPrompt);
impl_remote_event!(RemoteEvent);
impl_remote_function!(RemoteFunction);
impl_replicated_first!(ReplicatedFirst);
impl_replicated_storage!(ReplicatedStorage);
impl_rotation_curve!(RotationCurve);
impl_screenshot_hud!(ScreenshotHud);
impl_server_script_service!(ServerScriptService);
impl_server_storage!(ServerStorage);
impl_service_provider!(ServiceProvider);
impl_data_model!(DataModel);
impl_generic_settings!(GenericSettings);
impl_user_settings!(UserSettings);
impl_sky!(Sky);
impl_smoke!(Smoke);
impl_sound!(Sound);
impl_sound_effect!(SoundEffect);
impl_chorus_sound_effect!(ChorusSoundEffect);
impl_compressor_sound_effect!(CompressorSoundEffect);
impl_custom_sound_effect!(CustomSoundEffect);
impl_channel_selector_sound_effect!(ChannelSelectorSoundEffect);
impl_distortion_sound_effect!(DistortionSoundEffect);
impl_echo_sound_effect!(EchoSoundEffect);
impl_equalizer_sound_effect!(EqualizerSoundEffect);
impl_flange_sound_effect!(FlangeSoundEffect);
impl_pitch_shift_sound_effect!(PitchShiftSoundEffect);
impl_reverb_sound_effect!(ReverbSoundEffect);
impl_tremolo_sound_effect!(TremoloSoundEffect);
impl_sound_group!(SoundGroup);
impl_sparkles!(Sparkles);
impl_speaker!(Speaker);
impl_starter_gear!(StarterGear);
impl_starter_pack!(StarterPack);
impl_starter_player!(StarterPlayer);
impl_starter_player_scripts!(StarterPlayerScripts);
impl_starter_character_scripts!(StarterCharacterScripts);
impl_surface_appearance!(SurfaceAppearance);
impl_team!(Team);
impl_teams!(Teams);
impl_teleport_async_result!(TeleportAsyncResult);
impl_teleport_options!(TeleportOptions);
impl_terrain_detail!(TerrainDetail);
impl_terrain_region!(TerrainRegion);
impl_text_channel!(TextChannel);
impl_text_chat_command!(TextChatCommand);
impl_text_chat_configurations!(TextChatConfigurations);
impl_chat_input_bar_configuration!(ChatInputBarConfiguration);
impl_chat_window_configuration!(ChatWindowConfiguration);
impl_text_chat_message!(TextChatMessage);
impl_text_chat_message_properties!(TextChatMessageProperties);
impl_text_filter_result!(TextFilterResult);
impl_text_source!(TextSource);
impl_touch_transmitter!(TouchTransmitter);
impl_trail!(Trail);
impl_translator!(Translator);
impl_tween_base!(TweenBase);
impl_tween!(Tween);
impl_ui_base!(UIBase);
impl_ui_component!(UIComponent);
impl_ui_constraint!(UIConstraint);
impl_ui_aspect_ratio_constraint!(UIAspectRatioConstraint);
impl_ui_size_constraint!(UISizeConstraint);
impl_ui_text_size_constraint!(UITextSizeConstraint);
impl_ui_corner!(UICorner);
impl_ui_gradient!(UIGradient);
impl_ui_layout!(UILayout);
impl_ui_grid_style_layout!(UIGridStyleLayout);
impl_ui_grid_layout!(UIGridLayout);
impl_ui_list_layout!(UIListLayout);
impl_ui_page_layout!(UIPageLayout);
impl_ui_table_layout!(UITableLayout);
impl_ui_padding!(UIPadding);
impl_ui_scale!(UIScale);
impl_ui_stroke!(UIStroke);
impl_user_game_settings!(UserGameSettings);
impl_value_base!(ValueBase);
impl_bool_value!(BoolValue);
impl_brick_color_value!(BrickColorValue);
impl_c_frame_value!(CFrameValue);
impl_color_3_value!(Color3Value);
impl_double_constrained_value!(DoubleConstrainedValue);
impl_int_constrained_value!(IntConstrainedValue);
impl_int_value!(IntValue);
impl_number_value!(NumberValue);
impl_object_value!(ObjectValue);
impl_ray_value!(RayValue);
impl_string_value!(StringValue);
impl_vector_3_value!(Vector3Value);
impl_vector_3_curve!(Vector3Curve);
impl_voice_channel!(VoiceChannel);
impl_voice_source!(VoiceSource);
impl_weld_constraint!(WeldConstraint);
creatable!(Accoutrement);
creatable!(Accessory);
creatable!(Hat);
creatable!(Animation);
creatable!(CurveAnimation);
creatable!(KeyframeSequence);
creatable!(AnimationController);
creatable!(AnimationRigData);
creatable!(Animator);
creatable!(Atmosphere);
creatable!(Attachment);
creatable!(Bone);
creatable!(Backpack);
creatable!(Tool);
creatable!(WrapLayer);
creatable!(WrapTarget);
creatable!(Beam);
creatable!(BindableEvent);
creatable!(BindableFunction);
creatable!(BodyAngularVelocity);
creatable!(BodyForce);
creatable!(BodyGyro);
creatable!(BodyPosition);
creatable!(BodyThrust);
creatable!(BodyVelocity);
creatable!(RocketPropulsion);
creatable!(Camera);
creatable!(BodyColors);
creatable!(CharacterMesh);
creatable!(Pants);
creatable!(Shirt);
creatable!(ShirtGraphic);
creatable!(ClickDetector);
creatable!(Clouds);
creatable!(AlignOrientation);
creatable!(AlignPosition);
creatable!(AngularVelocity);
creatable!(BallSocketConstraint);
creatable!(HingeConstraint);
creatable!(LineForce);
creatable!(LinearVelocity);
creatable!(PlaneConstraint);
creatable!(Plane);
creatable!(RigidConstraint);
creatable!(RodConstraint);
creatable!(RopeConstraint);
creatable!(CylindricalConstraint);
creatable!(PrismaticConstraint);
creatable!(SpringConstraint);
creatable!(Torque);
creatable!(TorsionSpringConstraint);
creatable!(UniversalConstraint);
creatable!(VectorForce);
creatable!(HumanoidController);
creatable!(SkateboardController);
creatable!(VehicleController);
creatable!(BlockMesh);
creatable!(CylinderMesh);
creatable!(FileMesh);
creatable!(SpecialMesh);
creatable!(DataStoreIncrementOptions);
creatable!(DataStoreOptions);
creatable!(DataStoreSetOptions);
creatable!(Dialog);
creatable!(DialogChoice);
creatable!(Dragger);
creatable!(EulerRotationCurve);
creatable!(Explosion);
creatable!(Decal);
creatable!(Texture);
creatable!(Fire);
creatable!(FloatCurve);
creatable!(Folder);
creatable!(ForceField);
creatable!(GetTextBoundsParams);
creatable!(CanvasGroup);
creatable!(Frame);
creatable!(ImageButton);
creatable!(TextButton);
creatable!(ImageLabel);
creatable!(TextLabel);
creatable!(ScrollingFrame);
creatable!(TextBox);
creatable!(VideoFrame);
creatable!(ViewportFrame);
creatable!(BillboardGui);
creatable!(ScreenGui);
creatable!(SurfaceGui);
creatable!(FloorWire);
creatable!(SelectionBox);
creatable!(BoxHandleAdornment);
creatable!(ConeHandleAdornment);
creatable!(CylinderHandleAdornment);
creatable!(ImageHandleAdornment);
creatable!(LineHandleAdornment);
creatable!(SphereHandleAdornment);
creatable!(ParabolaAdornment);
creatable!(SelectionSphere);
creatable!(ArcHandles);
creatable!(Handles);
creatable!(SurfaceSelection);
creatable!(SelectionPartLasso);
creatable!(SelectionPointLasso);
creatable!(Highlight);
creatable!(Humanoid);
creatable!(HumanoidDescription);
creatable!(Glue);
creatable!(Motor);
creatable!(Motor6D);
creatable!(VelocityMotor);
creatable!(Keyframe);
creatable!(KeyframeMarker);
creatable!(PointLight);
creatable!(SpotLight);
creatable!(SurfaceLight);
creatable!(LocalizationTable);
creatable!(Script);
creatable!(LocalScript);
creatable!(ModuleScript);
creatable!(MarkerCurve);
creatable!(MaterialVariant);
creatable!(NoCollisionConstraint);
creatable!(CornerWedgePart);
creatable!(Part);
creatable!(Seat);
creatable!(SkateboardPlatform);
creatable!(SpawnLocation);
creatable!(WedgePart);
creatable!(MeshPart);
creatable!(PartOperation);
creatable!(NegateOperation);
creatable!(UnionOperation);
creatable!(TrussPart);
creatable!(VehicleSeat);
creatable!(Model);
creatable!(Actor);
creatable!(WorldModel);
creatable!(ParticleEmitter);
creatable!(PathfindingLink);
creatable!(PathfindingModifier);
creatable!(Player);
creatable!(NumberPose);
creatable!(Pose);
creatable!(BloomEffect);
creatable!(BlurEffect);
creatable!(ColorCorrectionEffect);
creatable!(DepthOfFieldEffect);
creatable!(SunRaysEffect);
creatable!(ProximityPrompt);
creatable!(RemoteEvent);
creatable!(RemoteFunction);
creatable!(RotationCurve);
creatable!(Sky);
creatable!(Smoke);
creatable!(Sound);
creatable!(ChorusSoundEffect);
creatable!(CompressorSoundEffect);
creatable!(ChannelSelectorSoundEffect);
creatable!(DistortionSoundEffect);
creatable!(EchoSoundEffect);
creatable!(EqualizerSoundEffect);
creatable!(FlangeSoundEffect);
creatable!(PitchShiftSoundEffect);
creatable!(ReverbSoundEffect);
creatable!(TremoloSoundEffect);
creatable!(SoundGroup);
creatable!(Sparkles);
creatable!(Speaker);
creatable!(StarterGear);
creatable!(SurfaceAppearance);
creatable!(Team);
creatable!(TeleportOptions);
creatable!(TerrainDetail);
creatable!(TerrainRegion);
creatable!(TextChannel);
creatable!(TextChatCommand);
creatable!(TextChatMessageProperties);
creatable!(Trail);
creatable!(Tween);
creatable!(UIAspectRatioConstraint);
creatable!(UISizeConstraint);
creatable!(UITextSizeConstraint);
creatable!(UICorner);
creatable!(UIGradient);
creatable!(UIGridLayout);
creatable!(UIListLayout);
creatable!(UIPageLayout);
creatable!(UITableLayout);
creatable!(UIPadding);
creatable!(UIScale);
creatable!(UIStroke);
creatable!(BoolValue);
creatable!(BrickColorValue);
creatable!(CFrameValue);
creatable!(Color3Value);
creatable!(DoubleConstrainedValue);
creatable!(IntConstrainedValue);
creatable!(IntValue);
creatable!(NumberValue);
creatable!(ObjectValue);
creatable!(RayValue);
creatable!(StringValue);
creatable!(Vector3Value);
creatable!(Vector3Curve);
creatable!(VoiceChannel);
creatable!(WeldConstraint);
