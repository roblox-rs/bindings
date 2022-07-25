#[allow(dead_code)]
#[allow(improper_ctypes)]
extern "C" {
	fn get_game() -> DataModel;
	fn dyn_instance_clear_all_children(instance: u32);
	fn dyn_instance_clone(instance: u32) -> Option<Instance>;
	fn dyn_instance_destroy(instance: u32);
	fn dyn_instance_find_first_ancestor(instance: u32, p_name: &str) -> Option<Instance>;
	fn dyn_instance_find_first_ancestor_of_class(instance: u32, p_className: &str) -> Option<Instance>;
	fn dyn_instance_find_first_ancestor_which_is_a(instance: u32, p_className: &str) -> Option<Instance>;
	fn dyn_instance_find_first_child(instance: u32, p_name: &str, p_recursive: bool) -> Option<Instance>;
	fn dyn_instance_find_first_child_of_class(instance: u32, p_className: &str) -> Option<Instance>;
	fn dyn_instance_find_first_child_which_is_a(instance: u32, p_className: &str, p_recursive: bool) -> Option<Instance>;
	fn dyn_instance_find_first_descendant(instance: u32, p_name: &str) -> Option<Instance>;
	fn dyn_instance_get_actor(instance: u32) -> Option<Actor>;
	fn dyn_instance_get_attribute(instance: u32, p_attribute: &str);
	fn dyn_instance_get_attribute_changed_signal(instance: u32, p_attribute: &str) -> RbxScriptSignal;
	fn dyn_instance_get_attributes(instance: u32);
	fn dyn_instance_get_children(instance: u32) -> Objects;
	fn dyn_instance_get_descendants(instance: u32);
	fn dyn_instance_get_full_name(instance: u32) -> String;
	fn dyn_instance_get_property_changed_signal(instance: u32, p_property: &str) -> RbxScriptSignal;
	fn dyn_instance_is_a(instance: u32, p_className: &str) -> bool;
	fn dyn_instance_is_ancestor_of(instance: u32, p_descendant: Option<Instance>) -> bool;
	fn dyn_instance_is_descendant_of(instance: u32, p_ancestor: Option<Instance>) -> bool;
	fn dyn_instance_remove(instance: u32);
	fn dyn_instance_set_attribute(instance: u32, p_attribute: &str);
	fn dyn_instance_wait_for_child(instance: u32, p_childName: &str, p_timeOut: f64) -> Option<Instance>;
	fn prop_instance_archivable(instance: u32) -> bool;
	fn prop_set_instance_archivable(instance: u32, value: bool);
	fn prop_instance_class_name(instance: u32) -> String;
	fn prop_instance_name(instance: u32) -> String;
	fn prop_set_instance_name(instance: u32, value: &str);
	fn prop_instance_parent(instance: u32) -> Option<Instance>;
	fn prop_set_instance_parent(instance: u32, value: Option<Instance>);
	fn connect_instance_ancestry_changed(instance: u32, callback: Box<dyn Fn(Option<Instance>, Option<Instance>)>) -> u32;
	fn connect_instance_attribute_changed(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_instance_changed(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_instance_child_added(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn connect_instance_child_removed(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn connect_instance_descendant_added(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn connect_instance_descendant_removing(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn connect_instance_destroying(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn prop_accoutrement_attachment_forward(instance: u32) -> Vector3;
	fn prop_set_accoutrement_attachment_forward(instance: u32, value: Vector3);
	fn prop_accoutrement_attachment_point(instance: u32) -> CFrame;
	fn prop_set_accoutrement_attachment_point(instance: u32, value: CFrame);
	fn prop_accoutrement_attachment_pos(instance: u32) -> Vector3;
	fn prop_set_accoutrement_attachment_pos(instance: u32, value: Vector3);
	fn prop_accoutrement_attachment_right(instance: u32) -> Vector3;
	fn prop_set_accoutrement_attachment_right(instance: u32, value: Vector3);
	fn prop_accoutrement_attachment_up(instance: u32) -> Vector3;
	fn prop_set_accoutrement_attachment_up(instance: u32, value: Vector3);
	fn dyn_analytics_service_fire_custom_event(instance: u32, p_player: Option<Instance>, p_eventCategory: &str);
	fn dyn_analytics_service_fire_event(instance: u32, p_category: &str);
	fn dyn_analytics_service_fire_in_game_economy_event(instance: u32, p_player: Option<Instance>, p_itemName: &str, p_itemCategory: &str, p_amount: f64, p_currency: &str);
	fn dyn_analytics_service_fire_log_event(instance: u32, p_player: Option<Instance>, p_message: &str);
	fn dyn_analytics_service_fire_player_progression_event(instance: u32, p_player: Option<Instance>, p_category: &str);
	fn prop_animation_animation_id(instance: u32) -> Content;
	fn prop_set_animation_animation_id(instance: u32, value: Content);
	fn prop_animation_clip_loop(instance: u32) -> bool;
	fn prop_set_animation_clip_loop(instance: u32, value: bool);
	fn dyn_keyframe_sequence_add_keyframe(instance: u32, p_keyframe: Option<Instance>);
	fn dyn_keyframe_sequence_get_keyframes(instance: u32) -> Objects;
	fn dyn_keyframe_sequence_remove_keyframe(instance: u32, p_keyframe: Option<Instance>);
	fn dyn_animation_clip_provider_register_active_animation_clip(instance: u32, p_animationClip: Option<AnimationClip>) -> Content;
	fn dyn_animation_clip_provider_register_animation_clip(instance: u32, p_animationClip: Option<AnimationClip>) -> Content;
	fn dyn_animation_clip_provider_get_animation_clip_async(instance: u32, p_assetId: Content) -> Option<AnimationClip>;
	fn dyn_animation_clip_provider_get_animations(instance: u32, p_userId: f64) -> Option<Instance>;
	fn dyn_animation_controller_get_playing_animation_tracks(instance: u32);
	fn dyn_animation_controller_load_animation(instance: u32, p_animation: Option<Animation>) -> Option<AnimationTrack>;
	fn connect_animation_controller_animation_played(instance: u32, callback: Box<dyn Fn(Option<AnimationTrack>)>) -> u32;
	fn prop_animation_stream_track_is_playing(instance: u32) -> bool;
	fn prop_animation_stream_track_weight_current(instance: u32) -> f64;
	fn prop_animation_stream_track_weight_target(instance: u32) -> f64;
	fn dyn_animation_track_adjust_speed(instance: u32, p_speed: f64);
	fn dyn_animation_track_adjust_weight(instance: u32, p_weight: f64, p_fadeTime: f64);
	fn dyn_animation_track_get_marker_reached_signal(instance: u32, p_name: &str) -> RbxScriptSignal;
	fn dyn_animation_track_get_time_of_keyframe(instance: u32, p_keyframeName: &str) -> f64;
	fn dyn_animation_track_play(instance: u32, p_fadeTime: f64, p_weight: f64, p_speed: f64);
	fn dyn_animation_track_stop(instance: u32, p_fadeTime: f64);
	fn prop_animation_track_animation(instance: u32) -> Option<Animation>;
	fn prop_animation_track_is_playing(instance: u32) -> bool;
	fn prop_animation_track_length(instance: u32) -> f64;
	fn prop_animation_track_looped(instance: u32) -> bool;
	fn prop_set_animation_track_looped(instance: u32, value: bool);
	fn prop_animation_track_speed(instance: u32) -> f64;
	fn prop_animation_track_time_position(instance: u32) -> f64;
	fn prop_set_animation_track_time_position(instance: u32, value: f64);
	fn prop_animation_track_weight_current(instance: u32) -> f64;
	fn prop_animation_track_weight_target(instance: u32) -> f64;
	fn connect_animation_track_did_loop(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_animation_track_keyframe_reached(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_animation_track_stopped(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn dyn_animator_apply_joint_velocities(instance: u32);
	fn dyn_animator_get_playing_animation_tracks(instance: u32);
	fn dyn_animator_load_animation(instance: u32, p_animation: Option<Animation>) -> Option<AnimationTrack>;
	fn connect_animator_animation_played(instance: u32, callback: Box<dyn Fn(Option<AnimationTrack>)>) -> u32;
	fn connect_asset_import_service_progress_update(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn connect_asset_import_service_upload_finished(instance: u32, callback: Box<dyn Fn(bool, ())>) -> u32;
	fn dyn_asset_service_create_place_async(instance: u32, p_placeName: &str, p_templatePlaceID: f64, p_description: &str) -> f64;
	fn dyn_asset_service_create_place_in_player_inventory_async(instance: u32, p_player: Option<Instance>, p_placeName: &str, p_templatePlaceID: f64, p_description: &str) -> f64;
	fn dyn_asset_service_get_asset_ids_for_package(instance: u32, p_packageAssetId: f64);
	fn dyn_asset_service_get_bundle_details_async(instance: u32, p_bundleId: f64);
	fn dyn_asset_service_get_creator_asset_id(instance: u32, p_creationID: f64) -> f64;
	fn dyn_asset_service_get_game_places_async(instance: u32) -> Option<Instance>;
	fn dyn_asset_service_save_place_async(instance: u32);
	fn prop_atmosphere_color(instance: u32) -> Color3;
	fn prop_set_atmosphere_color(instance: u32, value: Color3);
	fn prop_atmosphere_decay(instance: u32) -> Color3;
	fn prop_set_atmosphere_decay(instance: u32, value: Color3);
	fn prop_atmosphere_density(instance: u32) -> f64;
	fn prop_set_atmosphere_density(instance: u32, value: f64);
	fn prop_atmosphere_glare(instance: u32) -> f64;
	fn prop_set_atmosphere_glare(instance: u32, value: f64);
	fn prop_atmosphere_haze(instance: u32) -> f64;
	fn prop_set_atmosphere_haze(instance: u32, value: f64);
	fn prop_atmosphere_offset(instance: u32) -> f64;
	fn prop_set_atmosphere_offset(instance: u32, value: f64);
	fn dyn_attachment_get_axis(instance: u32) -> Vector3;
	fn dyn_attachment_get_secondary_axis(instance: u32) -> Vector3;
	fn dyn_attachment_set_axis(instance: u32, p_axis: Vector3);
	fn dyn_attachment_set_secondary_axis(instance: u32, p_axis: Vector3);
	fn prop_attachment_axis(instance: u32) -> Vector3;
	fn prop_set_attachment_axis(instance: u32, value: Vector3);
	fn prop_attachment_c_frame(instance: u32) -> CFrame;
	fn prop_set_attachment_c_frame(instance: u32, value: CFrame);
	fn prop_attachment_orientation(instance: u32) -> Vector3;
	fn prop_set_attachment_orientation(instance: u32, value: Vector3);
	fn prop_attachment_position(instance: u32) -> Vector3;
	fn prop_set_attachment_position(instance: u32, value: Vector3);
	fn prop_attachment_rotation(instance: u32) -> Vector3;
	fn prop_set_attachment_rotation(instance: u32, value: Vector3);
	fn prop_attachment_secondary_axis(instance: u32) -> Vector3;
	fn prop_set_attachment_secondary_axis(instance: u32, value: Vector3);
	fn prop_attachment_visible(instance: u32) -> bool;
	fn prop_set_attachment_visible(instance: u32, value: bool);
	fn prop_attachment_world_axis(instance: u32) -> Vector3;
	fn prop_set_attachment_world_axis(instance: u32, value: Vector3);
	fn prop_attachment_world_c_frame(instance: u32) -> CFrame;
	fn prop_set_attachment_world_c_frame(instance: u32, value: CFrame);
	fn prop_attachment_world_orientation(instance: u32) -> Vector3;
	fn prop_set_attachment_world_orientation(instance: u32, value: Vector3);
	fn prop_attachment_world_position(instance: u32) -> Vector3;
	fn prop_set_attachment_world_position(instance: u32, value: Vector3);
	fn prop_attachment_world_rotation(instance: u32) -> Vector3;
	fn prop_attachment_world_secondary_axis(instance: u32) -> Vector3;
	fn prop_set_attachment_world_secondary_axis(instance: u32, value: Vector3);
	fn prop_bone_transform(instance: u32) -> CFrame;
	fn prop_set_bone_transform(instance: u32, value: CFrame);
	fn prop_bone_transformed_c_frame(instance: u32) -> CFrame;
	fn prop_bone_transformed_world_c_frame(instance: u32) -> CFrame;
	fn dyn_avatar_editor_service_prompt_allow_inventory_read_access(instance: u32);
	fn dyn_avatar_editor_service_prompt_create_outfit(instance: u32, p_outfit: Option<HumanoidDescription>);
	fn dyn_avatar_editor_service_prompt_delete_outfit(instance: u32, p_outfitId: f64);
	fn dyn_avatar_editor_service_prompt_rename_outfit(instance: u32, p_outfitId: f64);
	fn dyn_avatar_editor_service_prompt_save_avatar(instance: u32, p_humanoidDescription: Option<HumanoidDescription>);
	fn dyn_avatar_editor_service_prompt_set_favorite(instance: u32, p_itemId: f64, p_shouldFavorite: bool);
	fn dyn_avatar_editor_service_prompt_update_outfit(instance: u32, p_outfitId: f64, p_updatedOutfit: Option<HumanoidDescription>);
	fn dyn_avatar_editor_service_check_apply_default_clothing(instance: u32, p_humanoidDescription: Option<HumanoidDescription>) -> Option<HumanoidDescription>;
	fn dyn_avatar_editor_service_get_avatar_rules(instance: u32);
	fn dyn_avatar_editor_service_get_batch_item_details(instance: u32);
	fn dyn_avatar_editor_service_get_favorite(instance: u32, p_itemId: f64) -> bool;
	fn dyn_avatar_editor_service_get_inventory(instance: u32) -> Option<InventoryPages>;
	fn dyn_avatar_editor_service_get_item_details(instance: u32, p_itemId: f64);
	fn dyn_avatar_editor_service_get_outfits(instance: u32) -> Option<OutfitPages>;
	fn dyn_avatar_editor_service_get_recommended_assets(instance: u32, p_contextAssetId: f64);
	fn dyn_avatar_editor_service_get_recommended_bundles(instance: u32, p_bundleId: f64);
	fn dyn_avatar_editor_service_search_catalog(instance: u32, p_searchParameters: CatalogSearchParams) -> Option<CatalogPages>;
	fn connect_avatar_editor_service_prompt_allow_inventory_read_access_completed(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_avatar_editor_service_prompt_create_outfit_completed(instance: u32, callback: Box<dyn Fn((), ())>) -> u32;
	fn connect_avatar_editor_service_prompt_delete_outfit_completed(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_avatar_editor_service_prompt_rename_outfit_completed(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_avatar_editor_service_prompt_save_avatar_completed(instance: u32, callback: Box<dyn Fn((), Option<HumanoidDescription>)>) -> u32;
	fn connect_avatar_editor_service_prompt_set_favorite_completed(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_avatar_editor_service_prompt_update_outfit_completed(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn prop_backpack_item_texture_id(instance: u32) -> Content;
	fn prop_set_backpack_item_texture_id(instance: u32, value: Content);
	fn dyn_tool_activate(instance: u32);
	fn dyn_tool_deactivate(instance: u32);
	fn prop_tool_can_be_dropped(instance: u32) -> bool;
	fn prop_set_tool_can_be_dropped(instance: u32, value: bool);
	fn prop_tool_enabled(instance: u32) -> bool;
	fn prop_set_tool_enabled(instance: u32, value: bool);
	fn prop_tool_grip(instance: u32) -> CFrame;
	fn prop_set_tool_grip(instance: u32, value: CFrame);
	fn prop_tool_grip_forward(instance: u32) -> Vector3;
	fn prop_set_tool_grip_forward(instance: u32, value: Vector3);
	fn prop_tool_grip_pos(instance: u32) -> Vector3;
	fn prop_set_tool_grip_pos(instance: u32, value: Vector3);
	fn prop_tool_grip_right(instance: u32) -> Vector3;
	fn prop_set_tool_grip_right(instance: u32, value: Vector3);
	fn prop_tool_grip_up(instance: u32) -> Vector3;
	fn prop_set_tool_grip_up(instance: u32, value: Vector3);
	fn prop_tool_manual_activation_only(instance: u32) -> bool;
	fn prop_set_tool_manual_activation_only(instance: u32, value: bool);
	fn prop_tool_requires_handle(instance: u32) -> bool;
	fn prop_set_tool_requires_handle(instance: u32, value: bool);
	fn prop_tool_tool_tip(instance: u32) -> String;
	fn prop_set_tool_tool_tip(instance: u32, value: &str);
	fn connect_tool_activated(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_tool_deactivated(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_tool_equipped(instance: u32, callback: Box<dyn Fn(Option<Mouse>)>) -> u32;
	fn connect_tool_unequipped(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn dyn_badge_service_award_badge(instance: u32, p_userId: f64, p_badgeId: f64) -> bool;
	fn dyn_badge_service_get_badge_info_async(instance: u32, p_badgeId: f64);
	fn dyn_badge_service_is_disabled(instance: u32, p_badgeId: f64) -> bool;
	fn dyn_badge_service_is_legal(instance: u32, p_badgeId: f64) -> bool;
	fn dyn_badge_service_user_has_badge(instance: u32, p_userId: f64, p_badgeId: f64) -> bool;
	fn dyn_badge_service_user_has_badge_async(instance: u32, p_userId: f64, p_badgeId: f64) -> bool;
	fn dyn_base_player_gui_get_gui_objects_at_position(instance: u32, p_x: f64, p_y: f64) -> Objects;
	fn dyn_player_gui_get_topbar_transparency(instance: u32) -> f64;
	fn dyn_player_gui_set_topbar_transparency(instance: u32, p_transparency: f64);
	fn prop_player_gui_selection_image_object(instance: u32) -> Option<GuiObject>;
	fn prop_set_player_gui_selection_image_object(instance: u32, value: Option<GuiObject>);
	fn connect_player_gui_topbar_transparency_changed_signal(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn dyn_starter_gui_get_core_gui_enabled(instance: u32) -> bool;
	fn dyn_starter_gui_set_core(instance: u32, p_parameterName: &str);
	fn dyn_starter_gui_set_core_gui_enabled(instance: u32, p_enabled: bool);
	fn dyn_starter_gui_get_core(instance: u32, p_parameterName: &str);
	fn prop_starter_gui_reset_player_gui_on_spawn(instance: u32) -> bool;
	fn prop_set_starter_gui_reset_player_gui_on_spawn(instance: u32, value: bool);
	fn prop_starter_gui_show_development_gui(instance: u32) -> bool;
	fn prop_set_starter_gui_show_development_gui(instance: u32, value: bool);
	fn prop_base_wrap_cage_mesh_id(instance: u32) -> Content;
	fn prop_base_wrap_cage_origin(instance: u32) -> CFrame;
	fn prop_base_wrap_cage_origin_world(instance: u32) -> CFrame;
	fn prop_base_wrap_import_origin(instance: u32) -> CFrame;
	fn prop_base_wrap_import_origin_world(instance: u32) -> CFrame;
	fn prop_wrap_layer_bind_offset(instance: u32) -> CFrame;
	fn prop_wrap_layer_color(instance: u32) -> Color3;
	fn prop_set_wrap_layer_color(instance: u32, value: Color3);
	fn prop_wrap_layer_enabled(instance: u32) -> bool;
	fn prop_set_wrap_layer_enabled(instance: u32, value: bool);
	fn prop_wrap_layer_order(instance: u32) -> f64;
	fn prop_set_wrap_layer_order(instance: u32, value: f64);
	fn prop_wrap_layer_puffiness(instance: u32) -> f64;
	fn prop_set_wrap_layer_puffiness(instance: u32, value: f64);
	fn prop_wrap_layer_reference_mesh_id(instance: u32) -> Content;
	fn prop_wrap_layer_reference_origin(instance: u32) -> CFrame;
	fn prop_wrap_layer_reference_origin_world(instance: u32) -> CFrame;
	fn prop_wrap_layer_shrink_factor(instance: u32) -> f64;
	fn prop_wrap_target_color(instance: u32) -> Color3;
	fn prop_set_wrap_target_color(instance: u32, value: Color3);
	fn prop_wrap_target_stiffness(instance: u32) -> f64;
	fn dyn_beam_set_texture_offset(instance: u32, p_offset: f64);
	fn prop_beam_attachment_0(instance: u32) -> Option<Attachment>;
	fn prop_set_beam_attachment_0(instance: u32, value: Option<Attachment>);
	fn prop_beam_attachment_1(instance: u32) -> Option<Attachment>;
	fn prop_set_beam_attachment_1(instance: u32, value: Option<Attachment>);
	fn prop_beam_brightness(instance: u32) -> f64;
	fn prop_set_beam_brightness(instance: u32, value: f64);
	fn prop_beam_color(instance: u32) -> ColorSequence;
	fn prop_set_beam_color(instance: u32, value: ColorSequence);
	fn prop_beam_curve_size_0(instance: u32) -> f64;
	fn prop_set_beam_curve_size_0(instance: u32, value: f64);
	fn prop_beam_curve_size_1(instance: u32) -> f64;
	fn prop_set_beam_curve_size_1(instance: u32, value: f64);
	fn prop_beam_enabled(instance: u32) -> bool;
	fn prop_set_beam_enabled(instance: u32, value: bool);
	fn prop_beam_face_camera(instance: u32) -> bool;
	fn prop_set_beam_face_camera(instance: u32, value: bool);
	fn prop_beam_light_emission(instance: u32) -> f64;
	fn prop_set_beam_light_emission(instance: u32, value: f64);
	fn prop_beam_light_influence(instance: u32) -> f64;
	fn prop_set_beam_light_influence(instance: u32, value: f64);
	fn prop_beam_segments(instance: u32) -> f64;
	fn prop_set_beam_segments(instance: u32, value: f64);
	fn prop_beam_texture(instance: u32) -> Content;
	fn prop_set_beam_texture(instance: u32, value: Content);
	fn prop_beam_texture_length(instance: u32) -> f64;
	fn prop_set_beam_texture_length(instance: u32, value: f64);
	fn prop_beam_texture_speed(instance: u32) -> f64;
	fn prop_set_beam_texture_speed(instance: u32, value: f64);
	fn prop_beam_transparency(instance: u32) -> NumberSequence;
	fn prop_set_beam_transparency(instance: u32, value: NumberSequence);
	fn prop_beam_width_0(instance: u32) -> f64;
	fn prop_set_beam_width_0(instance: u32, value: f64);
	fn prop_beam_width_1(instance: u32) -> f64;
	fn prop_set_beam_width_1(instance: u32, value: f64);
	fn prop_beam_z_offset(instance: u32) -> f64;
	fn prop_set_beam_z_offset(instance: u32, value: f64);
	fn dyn_bindable_event_fire(instance: u32);
	fn connect_bindable_event_event(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn dyn_bindable_function_invoke(instance: u32);
	fn prop_body_angular_velocity_angular_velocity(instance: u32) -> Vector3;
	fn prop_set_body_angular_velocity_angular_velocity(instance: u32, value: Vector3);
	fn prop_body_angular_velocity_max_torque(instance: u32) -> Vector3;
	fn prop_set_body_angular_velocity_max_torque(instance: u32, value: Vector3);
	fn prop_body_angular_velocity_p(instance: u32) -> f64;
	fn prop_set_body_angular_velocity_p(instance: u32, value: f64);
	fn prop_body_force_force(instance: u32) -> Vector3;
	fn prop_set_body_force_force(instance: u32, value: Vector3);
	fn prop_body_gyro_c_frame(instance: u32) -> CFrame;
	fn prop_set_body_gyro_c_frame(instance: u32, value: CFrame);
	fn prop_body_gyro_d(instance: u32) -> f64;
	fn prop_set_body_gyro_d(instance: u32, value: f64);
	fn prop_body_gyro_max_torque(instance: u32) -> Vector3;
	fn prop_set_body_gyro_max_torque(instance: u32, value: Vector3);
	fn prop_body_gyro_p(instance: u32) -> f64;
	fn prop_set_body_gyro_p(instance: u32, value: f64);
	fn prop_body_position_d(instance: u32) -> f64;
	fn prop_set_body_position_d(instance: u32, value: f64);
	fn prop_body_position_max_force(instance: u32) -> Vector3;
	fn prop_set_body_position_max_force(instance: u32, value: Vector3);
	fn prop_body_position_p(instance: u32) -> f64;
	fn prop_set_body_position_p(instance: u32, value: f64);
	fn prop_body_position_position(instance: u32) -> Vector3;
	fn prop_set_body_position_position(instance: u32, value: Vector3);
	fn connect_body_position_reached_target(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn prop_body_thrust_force(instance: u32) -> Vector3;
	fn prop_set_body_thrust_force(instance: u32, value: Vector3);
	fn prop_body_thrust_location(instance: u32) -> Vector3;
	fn prop_set_body_thrust_location(instance: u32, value: Vector3);
	fn prop_body_velocity_max_force(instance: u32) -> Vector3;
	fn prop_set_body_velocity_max_force(instance: u32, value: Vector3);
	fn prop_body_velocity_p(instance: u32) -> f64;
	fn prop_set_body_velocity_p(instance: u32, value: f64);
	fn prop_body_velocity_velocity(instance: u32) -> Vector3;
	fn prop_set_body_velocity_velocity(instance: u32, value: Vector3);
	fn prop_rocket_propulsion_cartoon_factor(instance: u32) -> f64;
	fn prop_set_rocket_propulsion_cartoon_factor(instance: u32, value: f64);
	fn prop_rocket_propulsion_max_speed(instance: u32) -> f64;
	fn prop_set_rocket_propulsion_max_speed(instance: u32, value: f64);
	fn prop_rocket_propulsion_max_thrust(instance: u32) -> f64;
	fn prop_set_rocket_propulsion_max_thrust(instance: u32, value: f64);
	fn prop_rocket_propulsion_max_torque(instance: u32) -> Vector3;
	fn prop_set_rocket_propulsion_max_torque(instance: u32, value: Vector3);
	fn prop_rocket_propulsion_target(instance: u32) -> Option<BasePart>;
	fn prop_set_rocket_propulsion_target(instance: u32, value: Option<BasePart>);
	fn prop_rocket_propulsion_target_offset(instance: u32) -> Vector3;
	fn prop_set_rocket_propulsion_target_offset(instance: u32, value: Vector3);
	fn prop_rocket_propulsion_target_radius(instance: u32) -> f64;
	fn prop_set_rocket_propulsion_target_radius(instance: u32, value: f64);
	fn prop_rocket_propulsion_thrust_d(instance: u32) -> f64;
	fn prop_set_rocket_propulsion_thrust_d(instance: u32, value: f64);
	fn prop_rocket_propulsion_thrust_p(instance: u32) -> f64;
	fn prop_set_rocket_propulsion_thrust_p(instance: u32, value: f64);
	fn prop_rocket_propulsion_turn_d(instance: u32) -> f64;
	fn prop_set_rocket_propulsion_turn_d(instance: u32, value: f64);
	fn prop_rocket_propulsion_turn_p(instance: u32) -> f64;
	fn prop_set_rocket_propulsion_turn_p(instance: u32, value: f64);
	fn connect_rocket_propulsion_reached_target(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn dyn_camera_get_largest_cutoff_distance(instance: u32, p_ignoreList: Objects) -> f64;
	fn dyn_camera_get_pan_speed(instance: u32) -> f64;
	fn dyn_camera_get_parts_obscuring_target(instance: u32, p_ignoreList: Objects) -> Objects;
	fn dyn_camera_get_render_c_frame(instance: u32) -> CFrame;
	fn dyn_camera_get_roll(instance: u32) -> f64;
	fn dyn_camera_get_tilt_speed(instance: u32) -> f64;
	fn dyn_camera_interpolate(instance: u32, p_endPos: CFrame, p_endFocus: CFrame, p_duration: f64);
	fn dyn_camera_pan_units(instance: u32, p_units: f64);
	fn dyn_camera_screen_point_to_ray(instance: u32, p_x: f64, p_y: f64, p_depth: f64) -> Ray;
	fn dyn_camera_set_camera_pan_mode(instance: u32);
	fn dyn_camera_set_roll(instance: u32, p_rollAngle: f64);
	fn dyn_camera_tilt_units(instance: u32, p_units: f64) -> bool;
	fn dyn_camera_viewport_point_to_ray(instance: u32, p_x: f64, p_y: f64, p_depth: f64) -> Ray;
	fn dyn_camera_world_to_screen_point(instance: u32, p_worldPoint: Vector3);
	fn dyn_camera_world_to_viewport_point(instance: u32, p_worldPoint: Vector3);
	fn prop_camera_c_frame(instance: u32) -> CFrame;
	fn prop_set_camera_c_frame(instance: u32, value: CFrame);
	fn prop_camera_camera_subject(instance: u32) -> Option<Instance>;
	fn prop_set_camera_camera_subject(instance: u32, value: Option<Instance>);
	fn prop_camera_coordinate_frame(instance: u32) -> CFrame;
	fn prop_set_camera_coordinate_frame(instance: u32, value: CFrame);
	fn prop_camera_diagonal_field_of_view(instance: u32) -> f64;
	fn prop_set_camera_diagonal_field_of_view(instance: u32, value: f64);
	fn prop_camera_field_of_view(instance: u32) -> f64;
	fn prop_set_camera_field_of_view(instance: u32, value: f64);
	fn prop_camera_focus(instance: u32) -> CFrame;
	fn prop_set_camera_focus(instance: u32, value: CFrame);
	fn prop_camera_head_locked(instance: u32) -> bool;
	fn prop_set_camera_head_locked(instance: u32, value: bool);
	fn prop_camera_head_scale(instance: u32) -> f64;
	fn prop_set_camera_head_scale(instance: u32, value: f64);
	fn prop_camera_max_axis_field_of_view(instance: u32) -> f64;
	fn prop_set_camera_max_axis_field_of_view(instance: u32, value: f64);
	fn prop_camera_near_plane_z(instance: u32) -> f64;
	fn prop_camera_viewport_size(instance: u32) -> Vector2;
	fn connect_camera_interpolation_finished(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn prop_body_colors_head_color(instance: u32) -> BrickColor;
	fn prop_set_body_colors_head_color(instance: u32, value: BrickColor);
	fn prop_body_colors_head_color_3(instance: u32) -> Color3;
	fn prop_set_body_colors_head_color_3(instance: u32, value: Color3);
	fn prop_body_colors_left_arm_color(instance: u32) -> BrickColor;
	fn prop_set_body_colors_left_arm_color(instance: u32, value: BrickColor);
	fn prop_body_colors_left_arm_color_3(instance: u32) -> Color3;
	fn prop_set_body_colors_left_arm_color_3(instance: u32, value: Color3);
	fn prop_body_colors_left_leg_color(instance: u32) -> BrickColor;
	fn prop_set_body_colors_left_leg_color(instance: u32, value: BrickColor);
	fn prop_body_colors_left_leg_color_3(instance: u32) -> Color3;
	fn prop_set_body_colors_left_leg_color_3(instance: u32, value: Color3);
	fn prop_body_colors_right_arm_color(instance: u32) -> BrickColor;
	fn prop_set_body_colors_right_arm_color(instance: u32, value: BrickColor);
	fn prop_body_colors_right_arm_color_3(instance: u32) -> Color3;
	fn prop_set_body_colors_right_arm_color_3(instance: u32, value: Color3);
	fn prop_body_colors_right_leg_color(instance: u32) -> BrickColor;
	fn prop_set_body_colors_right_leg_color(instance: u32, value: BrickColor);
	fn prop_body_colors_right_leg_color_3(instance: u32) -> Color3;
	fn prop_set_body_colors_right_leg_color_3(instance: u32, value: Color3);
	fn prop_body_colors_torso_color(instance: u32) -> BrickColor;
	fn prop_set_body_colors_torso_color(instance: u32, value: BrickColor);
	fn prop_body_colors_torso_color_3(instance: u32) -> Color3;
	fn prop_set_body_colors_torso_color_3(instance: u32, value: Color3);
	fn prop_character_mesh_base_texture_id(instance: u32) -> f64;
	fn prop_set_character_mesh_base_texture_id(instance: u32, value: f64);
	fn prop_character_mesh_mesh_id(instance: u32) -> f64;
	fn prop_set_character_mesh_mesh_id(instance: u32, value: f64);
	fn prop_character_mesh_overlay_texture_id(instance: u32) -> f64;
	fn prop_set_character_mesh_overlay_texture_id(instance: u32, value: f64);
	fn prop_clothing_color_3(instance: u32) -> Color3;
	fn prop_set_clothing_color_3(instance: u32, value: Color3);
	fn prop_pants_pants_template(instance: u32) -> Content;
	fn prop_set_pants_pants_template(instance: u32, value: Content);
	fn prop_shirt_shirt_template(instance: u32) -> Content;
	fn prop_set_shirt_shirt_template(instance: u32, value: Content);
	fn prop_shirt_graphic_color_3(instance: u32) -> Color3;
	fn prop_set_shirt_graphic_color_3(instance: u32, value: Color3);
	fn prop_shirt_graphic_graphic(instance: u32) -> Content;
	fn prop_set_shirt_graphic_graphic(instance: u32, value: Content);
	fn dyn_chat_chat(instance: u32, p_partOrCharacter: Option<Instance>, p_message: &str);
	fn dyn_chat_invoke_chat_callback(instance: u32);
	fn dyn_chat_register_chat_callback(instance: u32, p_callbackFunction: Function);
	fn dyn_chat_set_bubble_chat_settings(instance: u32);
	fn dyn_chat_can_user_chat_async(instance: u32, p_userId: f64) -> bool;
	fn dyn_chat_can_users_chat_async(instance: u32, p_userIdFrom: f64, p_userIdTo: f64) -> bool;
	fn dyn_chat_filter_string_async(instance: u32, p_stringToFilter: &str, p_playerFrom: Option<Player>, p_playerTo: Option<Player>) -> String;
	fn dyn_chat_filter_string_for_broadcast(instance: u32, p_stringToFilter: &str, p_playerFrom: Option<Player>) -> String;
	fn dyn_chat_filter_string_for_player_async(instance: u32, p_stringToFilter: &str, p_playerToFilterFor: Option<Player>) -> String;
	fn prop_chat_bubble_chat_enabled(instance: u32) -> bool;
	fn prop_set_chat_bubble_chat_enabled(instance: u32, value: bool);
	fn prop_chat_load_default_chat(instance: u32) -> bool;
	fn connect_chat_chatted(instance: u32, callback: Box<dyn Fn(Option<Instance>, String, ())>) -> u32;
	fn prop_click_detector_cursor_icon(instance: u32) -> Content;
	fn prop_set_click_detector_cursor_icon(instance: u32, value: Content);
	fn prop_click_detector_max_activation_distance(instance: u32) -> f64;
	fn prop_set_click_detector_max_activation_distance(instance: u32, value: f64);
	fn connect_click_detector_mouse_click(instance: u32, callback: Box<dyn Fn(Option<Player>)>) -> u32;
	fn connect_click_detector_mouse_hover_enter(instance: u32, callback: Box<dyn Fn(Option<Player>)>) -> u32;
	fn connect_click_detector_mouse_hover_leave(instance: u32, callback: Box<dyn Fn(Option<Player>)>) -> u32;
	fn connect_click_detector_right_mouse_click(instance: u32, callback: Box<dyn Fn(Option<Player>)>) -> u32;
	fn prop_clouds_color(instance: u32) -> Color3;
	fn prop_set_clouds_color(instance: u32, value: Color3);
	fn prop_clouds_cover(instance: u32) -> f64;
	fn prop_set_clouds_cover(instance: u32, value: f64);
	fn prop_clouds_density(instance: u32) -> f64;
	fn prop_set_clouds_density(instance: u32, value: f64);
	fn prop_clouds_enabled(instance: u32) -> bool;
	fn prop_set_clouds_enabled(instance: u32, value: bool);
	fn dyn_collection_service_add_tag(instance: u32, p_instance: Option<Instance>, p_tag: &str);
	fn dyn_collection_service_get_all_tags(instance: u32);
	fn dyn_collection_service_get_collection(instance: u32, p_class: &str) -> Objects;
	fn dyn_collection_service_get_instance_added_signal(instance: u32, p_tag: &str) -> RbxScriptSignal;
	fn dyn_collection_service_get_instance_removed_signal(instance: u32, p_tag: &str) -> RbxScriptSignal;
	fn dyn_collection_service_get_tagged(instance: u32, p_tag: &str) -> Objects;
	fn dyn_collection_service_get_tags(instance: u32, p_instance: Option<Instance>);
	fn dyn_collection_service_has_tag(instance: u32, p_instance: Option<Instance>, p_tag: &str) -> bool;
	fn dyn_collection_service_remove_tag(instance: u32, p_instance: Option<Instance>, p_tag: &str);
	fn connect_collection_service_item_added(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn connect_collection_service_item_removed(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn connect_collection_service_tag_added(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_collection_service_tag_removed(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn prop_command_instance_allow_gui_access_points(instance: u32) -> bool;
	fn prop_command_instance_display_name(instance: u32) -> String;
	fn prop_set_command_instance_display_name(instance: u32, value: &str);
	fn prop_constraint_active(instance: u32) -> bool;
	fn prop_constraint_attachment_0(instance: u32) -> Option<Attachment>;
	fn prop_set_constraint_attachment_0(instance: u32, value: Option<Attachment>);
	fn prop_constraint_attachment_1(instance: u32) -> Option<Attachment>;
	fn prop_set_constraint_attachment_1(instance: u32, value: Option<Attachment>);
	fn prop_constraint_color(instance: u32) -> BrickColor;
	fn prop_set_constraint_color(instance: u32, value: BrickColor);
	fn prop_constraint_enabled(instance: u32) -> bool;
	fn prop_set_constraint_enabled(instance: u32, value: bool);
	fn prop_constraint_visible(instance: u32) -> bool;
	fn prop_set_constraint_visible(instance: u32, value: bool);
	fn prop_align_orientation_c_frame(instance: u32) -> CFrame;
	fn prop_set_align_orientation_c_frame(instance: u32, value: CFrame);
	fn prop_align_orientation_max_angular_velocity(instance: u32) -> f64;
	fn prop_set_align_orientation_max_angular_velocity(instance: u32, value: f64);
	fn prop_align_orientation_max_torque(instance: u32) -> f64;
	fn prop_set_align_orientation_max_torque(instance: u32, value: f64);
	fn prop_align_orientation_primary_axis(instance: u32) -> Vector3;
	fn prop_set_align_orientation_primary_axis(instance: u32, value: Vector3);
	fn prop_align_orientation_primary_axis_only(instance: u32) -> bool;
	fn prop_set_align_orientation_primary_axis_only(instance: u32, value: bool);
	fn prop_align_orientation_reaction_torque_enabled(instance: u32) -> bool;
	fn prop_set_align_orientation_reaction_torque_enabled(instance: u32, value: bool);
	fn prop_align_orientation_responsiveness(instance: u32) -> f64;
	fn prop_set_align_orientation_responsiveness(instance: u32, value: f64);
	fn prop_align_orientation_rigidity_enabled(instance: u32) -> bool;
	fn prop_set_align_orientation_rigidity_enabled(instance: u32, value: bool);
	fn prop_align_orientation_secondary_axis(instance: u32) -> Vector3;
	fn prop_set_align_orientation_secondary_axis(instance: u32, value: Vector3);
	fn prop_align_position_apply_at_center_of_mass(instance: u32) -> bool;
	fn prop_set_align_position_apply_at_center_of_mass(instance: u32, value: bool);
	fn prop_align_position_max_force(instance: u32) -> f64;
	fn prop_set_align_position_max_force(instance: u32, value: f64);
	fn prop_align_position_max_velocity(instance: u32) -> f64;
	fn prop_set_align_position_max_velocity(instance: u32, value: f64);
	fn prop_align_position_position(instance: u32) -> Vector3;
	fn prop_set_align_position_position(instance: u32, value: Vector3);
	fn prop_align_position_reaction_force_enabled(instance: u32) -> bool;
	fn prop_set_align_position_reaction_force_enabled(instance: u32, value: bool);
	fn prop_align_position_responsiveness(instance: u32) -> f64;
	fn prop_set_align_position_responsiveness(instance: u32, value: f64);
	fn prop_align_position_rigidity_enabled(instance: u32) -> bool;
	fn prop_set_align_position_rigidity_enabled(instance: u32, value: bool);
	fn prop_angular_velocity_angular_velocity(instance: u32) -> Vector3;
	fn prop_set_angular_velocity_angular_velocity(instance: u32, value: Vector3);
	fn prop_angular_velocity_max_torque(instance: u32) -> f64;
	fn prop_set_angular_velocity_max_torque(instance: u32, value: f64);
	fn prop_angular_velocity_reaction_torque_enabled(instance: u32) -> bool;
	fn prop_set_angular_velocity_reaction_torque_enabled(instance: u32, value: bool);
	fn prop_ball_socket_constraint_limits_enabled(instance: u32) -> bool;
	fn prop_set_ball_socket_constraint_limits_enabled(instance: u32, value: bool);
	fn prop_ball_socket_constraint_max_friction_torque(instance: u32) -> f64;
	fn prop_set_ball_socket_constraint_max_friction_torque(instance: u32, value: f64);
	fn prop_ball_socket_constraint_radius(instance: u32) -> f64;
	fn prop_set_ball_socket_constraint_radius(instance: u32, value: f64);
	fn prop_ball_socket_constraint_restitution(instance: u32) -> f64;
	fn prop_set_ball_socket_constraint_restitution(instance: u32, value: f64);
	fn prop_ball_socket_constraint_twist_limits_enabled(instance: u32) -> bool;
	fn prop_set_ball_socket_constraint_twist_limits_enabled(instance: u32, value: bool);
	fn prop_ball_socket_constraint_twist_lower_angle(instance: u32) -> f64;
	fn prop_set_ball_socket_constraint_twist_lower_angle(instance: u32, value: f64);
	fn prop_ball_socket_constraint_twist_upper_angle(instance: u32) -> f64;
	fn prop_set_ball_socket_constraint_twist_upper_angle(instance: u32, value: f64);
	fn prop_ball_socket_constraint_upper_angle(instance: u32) -> f64;
	fn prop_set_ball_socket_constraint_upper_angle(instance: u32, value: f64);
	fn prop_hinge_constraint_angular_responsiveness(instance: u32) -> f64;
	fn prop_set_hinge_constraint_angular_responsiveness(instance: u32, value: f64);
	fn prop_hinge_constraint_angular_speed(instance: u32) -> f64;
	fn prop_set_hinge_constraint_angular_speed(instance: u32, value: f64);
	fn prop_hinge_constraint_angular_velocity(instance: u32) -> f64;
	fn prop_set_hinge_constraint_angular_velocity(instance: u32, value: f64);
	fn prop_hinge_constraint_current_angle(instance: u32) -> f64;
	fn prop_hinge_constraint_limits_enabled(instance: u32) -> bool;
	fn prop_set_hinge_constraint_limits_enabled(instance: u32, value: bool);
	fn prop_hinge_constraint_lower_angle(instance: u32) -> f64;
	fn prop_set_hinge_constraint_lower_angle(instance: u32, value: f64);
	fn prop_hinge_constraint_motor_max_acceleration(instance: u32) -> f64;
	fn prop_set_hinge_constraint_motor_max_acceleration(instance: u32, value: f64);
	fn prop_hinge_constraint_motor_max_torque(instance: u32) -> f64;
	fn prop_set_hinge_constraint_motor_max_torque(instance: u32, value: f64);
	fn prop_hinge_constraint_radius(instance: u32) -> f64;
	fn prop_set_hinge_constraint_radius(instance: u32, value: f64);
	fn prop_hinge_constraint_restitution(instance: u32) -> f64;
	fn prop_set_hinge_constraint_restitution(instance: u32, value: f64);
	fn prop_hinge_constraint_servo_max_torque(instance: u32) -> f64;
	fn prop_set_hinge_constraint_servo_max_torque(instance: u32, value: f64);
	fn prop_hinge_constraint_target_angle(instance: u32) -> f64;
	fn prop_set_hinge_constraint_target_angle(instance: u32, value: f64);
	fn prop_hinge_constraint_upper_angle(instance: u32) -> f64;
	fn prop_set_hinge_constraint_upper_angle(instance: u32, value: f64);
	fn prop_line_force_apply_at_center_of_mass(instance: u32) -> bool;
	fn prop_set_line_force_apply_at_center_of_mass(instance: u32, value: bool);
	fn prop_line_force_inverse_square_law(instance: u32) -> bool;
	fn prop_set_line_force_inverse_square_law(instance: u32, value: bool);
	fn prop_line_force_magnitude(instance: u32) -> f64;
	fn prop_set_line_force_magnitude(instance: u32, value: f64);
	fn prop_line_force_max_force(instance: u32) -> f64;
	fn prop_set_line_force_max_force(instance: u32, value: f64);
	fn prop_line_force_reaction_force_enabled(instance: u32) -> bool;
	fn prop_set_line_force_reaction_force_enabled(instance: u32, value: bool);
	fn prop_linear_velocity_line_direction(instance: u32) -> Vector3;
	fn prop_set_linear_velocity_line_direction(instance: u32, value: Vector3);
	fn prop_linear_velocity_line_velocity(instance: u32) -> f64;
	fn prop_set_linear_velocity_line_velocity(instance: u32, value: f64);
	fn prop_linear_velocity_max_force(instance: u32) -> f64;
	fn prop_set_linear_velocity_max_force(instance: u32, value: f64);
	fn prop_linear_velocity_plane_velocity(instance: u32) -> Vector2;
	fn prop_set_linear_velocity_plane_velocity(instance: u32, value: Vector2);
	fn prop_linear_velocity_primary_tangent_axis(instance: u32) -> Vector3;
	fn prop_set_linear_velocity_primary_tangent_axis(instance: u32, value: Vector3);
	fn prop_linear_velocity_secondary_tangent_axis(instance: u32) -> Vector3;
	fn prop_set_linear_velocity_secondary_tangent_axis(instance: u32, value: Vector3);
	fn prop_linear_velocity_vector_velocity(instance: u32) -> Vector3;
	fn prop_set_linear_velocity_vector_velocity(instance: u32, value: Vector3);
	fn prop_rigid_constraint_broken(instance: u32) -> bool;
	fn prop_rigid_constraint_destruction_enabled(instance: u32) -> bool;
	fn prop_set_rigid_constraint_destruction_enabled(instance: u32, value: bool);
	fn prop_rigid_constraint_destruction_force(instance: u32) -> f64;
	fn prop_set_rigid_constraint_destruction_force(instance: u32, value: f64);
	fn prop_rigid_constraint_destruction_torque(instance: u32) -> f64;
	fn prop_set_rigid_constraint_destruction_torque(instance: u32, value: f64);
	fn prop_rod_constraint_current_distance(instance: u32) -> f64;
	fn prop_rod_constraint_length(instance: u32) -> f64;
	fn prop_set_rod_constraint_length(instance: u32, value: f64);
	fn prop_rod_constraint_limit_angle_0(instance: u32) -> f64;
	fn prop_set_rod_constraint_limit_angle_0(instance: u32, value: f64);
	fn prop_rod_constraint_limit_angle_1(instance: u32) -> f64;
	fn prop_set_rod_constraint_limit_angle_1(instance: u32, value: f64);
	fn prop_rod_constraint_limits_enabled(instance: u32) -> bool;
	fn prop_set_rod_constraint_limits_enabled(instance: u32, value: bool);
	fn prop_rod_constraint_thickness(instance: u32) -> f64;
	fn prop_set_rod_constraint_thickness(instance: u32, value: f64);
	fn prop_rope_constraint_current_distance(instance: u32) -> f64;
	fn prop_rope_constraint_length(instance: u32) -> f64;
	fn prop_set_rope_constraint_length(instance: u32, value: f64);
	fn prop_rope_constraint_restitution(instance: u32) -> f64;
	fn prop_set_rope_constraint_restitution(instance: u32, value: f64);
	fn prop_rope_constraint_thickness(instance: u32) -> f64;
	fn prop_set_rope_constraint_thickness(instance: u32, value: f64);
	fn prop_rope_constraint_winch_enabled(instance: u32) -> bool;
	fn prop_set_rope_constraint_winch_enabled(instance: u32, value: bool);
	fn prop_rope_constraint_winch_force(instance: u32) -> f64;
	fn prop_set_rope_constraint_winch_force(instance: u32, value: f64);
	fn prop_rope_constraint_winch_responsiveness(instance: u32) -> f64;
	fn prop_set_rope_constraint_winch_responsiveness(instance: u32, value: f64);
	fn prop_rope_constraint_winch_speed(instance: u32) -> f64;
	fn prop_set_rope_constraint_winch_speed(instance: u32, value: f64);
	fn prop_rope_constraint_winch_target(instance: u32) -> f64;
	fn prop_set_rope_constraint_winch_target(instance: u32, value: f64);
	fn prop_sliding_ball_constraint_current_position(instance: u32) -> f64;
	fn prop_sliding_ball_constraint_limits_enabled(instance: u32) -> bool;
	fn prop_set_sliding_ball_constraint_limits_enabled(instance: u32, value: bool);
	fn prop_sliding_ball_constraint_linear_responsiveness(instance: u32) -> f64;
	fn prop_set_sliding_ball_constraint_linear_responsiveness(instance: u32, value: f64);
	fn prop_sliding_ball_constraint_lower_limit(instance: u32) -> f64;
	fn prop_set_sliding_ball_constraint_lower_limit(instance: u32, value: f64);
	fn prop_sliding_ball_constraint_motor_max_acceleration(instance: u32) -> f64;
	fn prop_set_sliding_ball_constraint_motor_max_acceleration(instance: u32, value: f64);
	fn prop_sliding_ball_constraint_motor_max_force(instance: u32) -> f64;
	fn prop_set_sliding_ball_constraint_motor_max_force(instance: u32, value: f64);
	fn prop_sliding_ball_constraint_restitution(instance: u32) -> f64;
	fn prop_set_sliding_ball_constraint_restitution(instance: u32, value: f64);
	fn prop_sliding_ball_constraint_servo_max_force(instance: u32) -> f64;
	fn prop_set_sliding_ball_constraint_servo_max_force(instance: u32, value: f64);
	fn prop_sliding_ball_constraint_size(instance: u32) -> f64;
	fn prop_set_sliding_ball_constraint_size(instance: u32, value: f64);
	fn prop_sliding_ball_constraint_speed(instance: u32) -> f64;
	fn prop_set_sliding_ball_constraint_speed(instance: u32, value: f64);
	fn prop_sliding_ball_constraint_target_position(instance: u32) -> f64;
	fn prop_set_sliding_ball_constraint_target_position(instance: u32, value: f64);
	fn prop_sliding_ball_constraint_upper_limit(instance: u32) -> f64;
	fn prop_set_sliding_ball_constraint_upper_limit(instance: u32, value: f64);
	fn prop_sliding_ball_constraint_velocity(instance: u32) -> f64;
	fn prop_set_sliding_ball_constraint_velocity(instance: u32, value: f64);
	fn prop_cylindrical_constraint_angular_limits_enabled(instance: u32) -> bool;
	fn prop_set_cylindrical_constraint_angular_limits_enabled(instance: u32, value: bool);
	fn prop_cylindrical_constraint_angular_responsiveness(instance: u32) -> f64;
	fn prop_set_cylindrical_constraint_angular_responsiveness(instance: u32, value: f64);
	fn prop_cylindrical_constraint_angular_restitution(instance: u32) -> f64;
	fn prop_set_cylindrical_constraint_angular_restitution(instance: u32, value: f64);
	fn prop_cylindrical_constraint_angular_speed(instance: u32) -> f64;
	fn prop_set_cylindrical_constraint_angular_speed(instance: u32, value: f64);
	fn prop_cylindrical_constraint_angular_velocity(instance: u32) -> f64;
	fn prop_set_cylindrical_constraint_angular_velocity(instance: u32, value: f64);
	fn prop_cylindrical_constraint_current_angle(instance: u32) -> f64;
	fn prop_cylindrical_constraint_inclination_angle(instance: u32) -> f64;
	fn prop_set_cylindrical_constraint_inclination_angle(instance: u32, value: f64);
	fn prop_cylindrical_constraint_lower_angle(instance: u32) -> f64;
	fn prop_set_cylindrical_constraint_lower_angle(instance: u32, value: f64);
	fn prop_cylindrical_constraint_motor_max_angular_acceleration(instance: u32) -> f64;
	fn prop_set_cylindrical_constraint_motor_max_angular_acceleration(instance: u32, value: f64);
	fn prop_cylindrical_constraint_motor_max_torque(instance: u32) -> f64;
	fn prop_set_cylindrical_constraint_motor_max_torque(instance: u32, value: f64);
	fn prop_cylindrical_constraint_rotation_axis_visible(instance: u32) -> bool;
	fn prop_set_cylindrical_constraint_rotation_axis_visible(instance: u32, value: bool);
	fn prop_cylindrical_constraint_servo_max_torque(instance: u32) -> f64;
	fn prop_set_cylindrical_constraint_servo_max_torque(instance: u32, value: f64);
	fn prop_cylindrical_constraint_target_angle(instance: u32) -> f64;
	fn prop_set_cylindrical_constraint_target_angle(instance: u32, value: f64);
	fn prop_cylindrical_constraint_upper_angle(instance: u32) -> f64;
	fn prop_set_cylindrical_constraint_upper_angle(instance: u32, value: f64);
	fn prop_cylindrical_constraint_world_rotation_axis(instance: u32) -> Vector3;
	fn prop_spring_constraint_coils(instance: u32) -> f64;
	fn prop_set_spring_constraint_coils(instance: u32, value: f64);
	fn prop_spring_constraint_current_length(instance: u32) -> f64;
	fn prop_spring_constraint_damping(instance: u32) -> f64;
	fn prop_set_spring_constraint_damping(instance: u32, value: f64);
	fn prop_spring_constraint_free_length(instance: u32) -> f64;
	fn prop_set_spring_constraint_free_length(instance: u32, value: f64);
	fn prop_spring_constraint_limits_enabled(instance: u32) -> bool;
	fn prop_set_spring_constraint_limits_enabled(instance: u32, value: bool);
	fn prop_spring_constraint_max_force(instance: u32) -> f64;
	fn prop_set_spring_constraint_max_force(instance: u32, value: f64);
	fn prop_spring_constraint_max_length(instance: u32) -> f64;
	fn prop_set_spring_constraint_max_length(instance: u32, value: f64);
	fn prop_spring_constraint_min_length(instance: u32) -> f64;
	fn prop_set_spring_constraint_min_length(instance: u32, value: f64);
	fn prop_spring_constraint_radius(instance: u32) -> f64;
	fn prop_set_spring_constraint_radius(instance: u32, value: f64);
	fn prop_spring_constraint_stiffness(instance: u32) -> f64;
	fn prop_set_spring_constraint_stiffness(instance: u32, value: f64);
	fn prop_spring_constraint_thickness(instance: u32) -> f64;
	fn prop_set_spring_constraint_thickness(instance: u32, value: f64);
	fn prop_torque_torque(instance: u32) -> Vector3;
	fn prop_set_torque_torque(instance: u32, value: Vector3);
	fn prop_torsion_spring_constraint_coils(instance: u32) -> f64;
	fn prop_set_torsion_spring_constraint_coils(instance: u32, value: f64);
	fn prop_torsion_spring_constraint_current_angle(instance: u32) -> f64;
	fn prop_torsion_spring_constraint_damping(instance: u32) -> f64;
	fn prop_set_torsion_spring_constraint_damping(instance: u32, value: f64);
	fn prop_torsion_spring_constraint_limit_enabled(instance: u32) -> bool;
	fn prop_set_torsion_spring_constraint_limit_enabled(instance: u32, value: bool);
	fn prop_torsion_spring_constraint_limits_enabled(instance: u32) -> bool;
	fn prop_set_torsion_spring_constraint_limits_enabled(instance: u32, value: bool);
	fn prop_torsion_spring_constraint_max_angle(instance: u32) -> f64;
	fn prop_set_torsion_spring_constraint_max_angle(instance: u32, value: f64);
	fn prop_torsion_spring_constraint_max_torque(instance: u32) -> f64;
	fn prop_set_torsion_spring_constraint_max_torque(instance: u32, value: f64);
	fn prop_torsion_spring_constraint_radius(instance: u32) -> f64;
	fn prop_set_torsion_spring_constraint_radius(instance: u32, value: f64);
	fn prop_torsion_spring_constraint_restitution(instance: u32) -> f64;
	fn prop_set_torsion_spring_constraint_restitution(instance: u32, value: f64);
	fn prop_torsion_spring_constraint_stiffness(instance: u32) -> f64;
	fn prop_set_torsion_spring_constraint_stiffness(instance: u32, value: f64);
	fn prop_universal_constraint_limits_enabled(instance: u32) -> bool;
	fn prop_set_universal_constraint_limits_enabled(instance: u32, value: bool);
	fn prop_universal_constraint_max_angle(instance: u32) -> f64;
	fn prop_set_universal_constraint_max_angle(instance: u32, value: f64);
	fn prop_universal_constraint_radius(instance: u32) -> f64;
	fn prop_set_universal_constraint_radius(instance: u32, value: f64);
	fn prop_universal_constraint_restitution(instance: u32) -> f64;
	fn prop_set_universal_constraint_restitution(instance: u32, value: f64);
	fn prop_vector_force_apply_at_center_of_mass(instance: u32) -> bool;
	fn prop_set_vector_force_apply_at_center_of_mass(instance: u32, value: bool);
	fn prop_vector_force_force(instance: u32) -> Vector3;
	fn prop_set_vector_force_force(instance: u32, value: Vector3);
	fn dyn_content_provider_list_encrypted_assets(instance: u32);
	fn dyn_content_provider_preload(instance: u32, p_contentId: Content);
	fn dyn_content_provider_register_default_encryption_key(instance: u32, p_encryptionKey: &str);
	fn dyn_content_provider_register_default_session_key(instance: u32, p_sessionKey: &str);
	fn dyn_content_provider_register_encrypted_asset(instance: u32, p_assetId: Content, p_encryptionKey: &str);
	fn dyn_content_provider_register_session_encrypted_asset(instance: u32, p_contentId: Content, p_sessionKey: &str);
	fn dyn_content_provider_unregister_default_encryption_key(instance: u32);
	fn dyn_content_provider_unregister_encrypted_asset(instance: u32, p_assetId: Content);
	fn dyn_content_provider_preload_async(instance: u32, p_callbackFunction: Function);
	fn prop_content_provider_base_url(instance: u32) -> String;
	fn prop_content_provider_request_queue_size(instance: u32) -> f64;
	fn connect_content_provider_asset_fetch_failed(instance: u32, callback: Box<dyn Fn(Content)>) -> u32;
	fn dyn_context_action_service_bind_action(instance: u32, p_actionName: &str, p_functionToBind: Function, p_createTouchButton: bool);
	fn dyn_context_action_service_bind_action_at_priority(instance: u32, p_actionName: &str, p_functionToBind: Function, p_createTouchButton: bool, p_priorityLevel: f64);
	fn dyn_context_action_service_bind_action_to_input_types(instance: u32, p_actionName: &str, p_functionToBind: Function, p_createTouchButton: bool);
	fn dyn_context_action_service_bind_activate(instance: u32);
	fn dyn_context_action_service_get_all_bound_action_info(instance: u32);
	fn dyn_context_action_service_get_bound_action_info(instance: u32, p_actionName: &str);
	fn dyn_context_action_service_get_current_local_tool_icon(instance: u32) -> String;
	fn dyn_context_action_service_set_description(instance: u32, p_actionName: &str, p_description: &str);
	fn dyn_context_action_service_set_image(instance: u32, p_actionName: &str, p_image: &str);
	fn dyn_context_action_service_set_position(instance: u32, p_actionName: &str, p_position: UDim2);
	fn dyn_context_action_service_set_title(instance: u32, p_actionName: &str, p_title: &str);
	fn dyn_context_action_service_unbind_action(instance: u32, p_actionName: &str);
	fn dyn_context_action_service_unbind_activate(instance: u32);
	fn dyn_context_action_service_unbind_all_actions(instance: u32);
	fn dyn_context_action_service_get_button(instance: u32, p_actionName: &str) -> Option<Instance>;
	fn connect_context_action_service_local_tool_equipped(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn connect_context_action_service_local_tool_unequipped(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn dyn_controller_bind_button(instance: u32, p_caption: &str);
	fn dyn_controller_get_button(instance: u32) -> bool;
	fn dyn_controller_unbind_button(instance: u32);
	fn connect_controller_button_changed(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn prop_skateboard_controller_steer(instance: u32) -> f64;
	fn prop_skateboard_controller_throttle(instance: u32) -> f64;
	fn connect_skateboard_controller_axis_changed(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn prop_data_model_mesh_offset(instance: u32) -> Vector3;
	fn prop_set_data_model_mesh_offset(instance: u32, value: Vector3);
	fn prop_data_model_mesh_scale(instance: u32) -> Vector3;
	fn prop_set_data_model_mesh_scale(instance: u32, value: Vector3);
	fn prop_data_model_mesh_vertex_color(instance: u32) -> Vector3;
	fn prop_set_data_model_mesh_vertex_color(instance: u32, value: Vector3);
	fn prop_file_mesh_mesh_id(instance: u32) -> Content;
	fn prop_set_file_mesh_mesh_id(instance: u32, value: Content);
	fn prop_file_mesh_texture_id(instance: u32) -> Content;
	fn prop_set_file_mesh_texture_id(instance: u32, value: Content);
	fn dyn_data_store_increment_options_get_metadata(instance: u32);
	fn dyn_data_store_increment_options_set_metadata(instance: u32);
	fn prop_data_store_info_created_time(instance: u32) -> f64;
	fn prop_data_store_info_data_store_name(instance: u32) -> String;
	fn prop_data_store_info_updated_time(instance: u32) -> f64;
	fn prop_data_store_key_key_name(instance: u32) -> String;
	fn dyn_data_store_key_info_get_metadata(instance: u32);
	fn dyn_data_store_key_info_get_user_ids(instance: u32);
	fn prop_data_store_key_info_created_time(instance: u32) -> f64;
	fn prop_data_store_key_info_updated_time(instance: u32) -> f64;
	fn prop_data_store_key_info_version(instance: u32) -> String;
	fn prop_data_store_object_version_info_created_time(instance: u32) -> f64;
	fn prop_data_store_object_version_info_is_deleted(instance: u32) -> bool;
	fn prop_data_store_object_version_info_version(instance: u32) -> String;
	fn dyn_data_store_options_set_experimental_features(instance: u32);
	fn prop_data_store_options_all_scopes(instance: u32) -> bool;
	fn prop_set_data_store_options_all_scopes(instance: u32, value: bool);
	fn dyn_data_store_service_get_data_store(instance: u32, p_name: &str, p_scope: &str, p_options: Option<Instance>) -> Option<GlobalDataStore>;
	fn dyn_data_store_service_get_global_data_store(instance: u32) -> Option<GlobalDataStore>;
	fn dyn_data_store_service_get_ordered_data_store(instance: u32, p_name: &str, p_scope: &str) -> Option<OrderedDataStore>;
	fn dyn_data_store_service_get_request_budget_for_request_type(instance: u32) -> f64;
	fn dyn_data_store_service_list_data_stores_async(instance: u32, p_prefix: &str, p_pageSize: f64) -> Option<DataStoreListingPages>;
	fn dyn_data_store_set_options_get_metadata(instance: u32);
	fn dyn_data_store_set_options_set_metadata(instance: u32);
	fn dyn_debris_add_item(instance: u32, p_item: Option<Instance>, p_lifetime: f64);
	fn prop_debris_max_items(instance: u32) -> f64;
	fn prop_set_debris_max_items(instance: u32, value: f64);
	fn dyn_dialog_get_current_players(instance: u32) -> Objects;
	fn prop_dialog_conversation_distance(instance: u32) -> f64;
	fn prop_set_dialog_conversation_distance(instance: u32, value: f64);
	fn prop_dialog_goodbye_choice_active(instance: u32) -> bool;
	fn prop_set_dialog_goodbye_choice_active(instance: u32, value: bool);
	fn prop_dialog_goodbye_dialog(instance: u32) -> String;
	fn prop_set_dialog_goodbye_dialog(instance: u32, value: &str);
	fn prop_dialog_in_use(instance: u32) -> bool;
	fn prop_set_dialog_in_use(instance: u32, value: bool);
	fn prop_dialog_initial_prompt(instance: u32) -> String;
	fn prop_set_dialog_initial_prompt(instance: u32, value: &str);
	fn prop_dialog_trigger_distance(instance: u32) -> f64;
	fn prop_set_dialog_trigger_distance(instance: u32, value: f64);
	fn prop_dialog_trigger_offset(instance: u32) -> Vector3;
	fn prop_set_dialog_trigger_offset(instance: u32, value: Vector3);
	fn connect_dialog_dialog_choice_selected(instance: u32, callback: Box<dyn Fn(Option<Instance>, Option<Instance>)>) -> u32;
	fn prop_dialog_choice_goodbye_choice_active(instance: u32) -> bool;
	fn prop_set_dialog_choice_goodbye_choice_active(instance: u32, value: bool);
	fn prop_dialog_choice_goodbye_dialog(instance: u32) -> String;
	fn prop_set_dialog_choice_goodbye_dialog(instance: u32, value: &str);
	fn prop_dialog_choice_response_dialog(instance: u32) -> String;
	fn prop_set_dialog_choice_response_dialog(instance: u32, value: &str);
	fn prop_dialog_choice_user_dialog(instance: u32) -> String;
	fn prop_set_dialog_choice_user_dialog(instance: u32, value: &str);
	fn dyn_dragger_axis_rotate(instance: u32);
	fn dyn_dragger_mouse_down(instance: u32, p_mousePart: Option<Instance>, p_pointOnMousePart: Vector3, p_parts: Objects);
	fn dyn_dragger_mouse_move(instance: u32, p_mouseRay: Ray);
	fn dyn_dragger_mouse_up(instance: u32);
	fn prop_dragger_service_align_dragged_objects(instance: u32) -> bool;
	fn prop_dragger_service_angle_snap_enabled(instance: u32) -> bool;
	fn prop_dragger_service_angle_snap_increment(instance: u32) -> f64;
	fn prop_dragger_service_animate_hover(instance: u32) -> bool;
	fn prop_dragger_service_collisions_enabled(instance: u32) -> bool;
	fn prop_dragger_service_geometry_snap_color(instance: u32) -> Color3;
	fn prop_dragger_service_hover_animate_frequency(instance: u32) -> f64;
	fn prop_dragger_service_hover_thickness(instance: u32) -> f64;
	fn prop_dragger_service_joints_enabled(instance: u32) -> bool;
	fn prop_dragger_service_linear_snap_enabled(instance: u32) -> bool;
	fn prop_dragger_service_linear_snap_increment(instance: u32) -> f64;
	fn prop_dragger_service_show_hover(instance: u32) -> bool;
	fn prop_dragger_service_show_pivot_indicator(instance: u32) -> bool;
	fn prop_set_dragger_service_show_pivot_indicator(instance: u32, value: bool);
	fn dyn_euler_rotation_curve_get_angles_at_time(instance: u32, p_time: f64);
	fn dyn_euler_rotation_curve_get_rotation_at_time(instance: u32, p_time: f64) -> CFrame;
	fn dyn_euler_rotation_curve_x(instance: u32) -> Option<FloatCurve>;
	fn dyn_euler_rotation_curve_y(instance: u32) -> Option<FloatCurve>;
	fn dyn_euler_rotation_curve_z(instance: u32) -> Option<FloatCurve>;
	fn prop_explosion_blast_pressure(instance: u32) -> f64;
	fn prop_set_explosion_blast_pressure(instance: u32, value: f64);
	fn prop_explosion_blast_radius(instance: u32) -> f64;
	fn prop_set_explosion_blast_radius(instance: u32, value: f64);
	fn prop_explosion_destroy_joint_radius_percent(instance: u32) -> f64;
	fn prop_set_explosion_destroy_joint_radius_percent(instance: u32, value: f64);
	fn prop_explosion_position(instance: u32) -> Vector3;
	fn prop_set_explosion_position(instance: u32, value: Vector3);
	fn prop_explosion_time_scale(instance: u32) -> f64;
	fn prop_set_explosion_time_scale(instance: u32, value: f64);
	fn prop_explosion_visible(instance: u32) -> bool;
	fn prop_set_explosion_visible(instance: u32, value: bool);
	fn connect_explosion_hit(instance: u32, callback: Box<dyn Fn(Option<BasePart>, f64)>) -> u32;
	fn prop_decal_color_3(instance: u32) -> Color3;
	fn prop_set_decal_color_3(instance: u32, value: Color3);
	fn prop_decal_local_transparency_modifier(instance: u32) -> f64;
	fn prop_set_decal_local_transparency_modifier(instance: u32, value: f64);
	fn prop_decal_shiny(instance: u32) -> f64;
	fn prop_set_decal_shiny(instance: u32, value: f64);
	fn prop_decal_specular(instance: u32) -> f64;
	fn prop_set_decal_specular(instance: u32, value: f64);
	fn prop_decal_texture(instance: u32) -> Content;
	fn prop_set_decal_texture(instance: u32, value: Content);
	fn prop_decal_transparency(instance: u32) -> f64;
	fn prop_set_decal_transparency(instance: u32, value: f64);
	fn prop_decal_z_index(instance: u32) -> f64;
	fn prop_set_decal_z_index(instance: u32, value: f64);
	fn prop_texture_offset_studs_u(instance: u32) -> f64;
	fn prop_set_texture_offset_studs_u(instance: u32, value: f64);
	fn prop_texture_offset_studs_v(instance: u32) -> f64;
	fn prop_set_texture_offset_studs_v(instance: u32, value: f64);
	fn prop_texture_studs_per_tile_u(instance: u32) -> f64;
	fn prop_set_texture_studs_per_tile_u(instance: u32, value: f64);
	fn prop_texture_studs_per_tile_v(instance: u32) -> f64;
	fn prop_set_texture_studs_per_tile_v(instance: u32, value: f64);
	fn prop_fire_color(instance: u32) -> Color3;
	fn prop_set_fire_color(instance: u32, value: Color3);
	fn prop_fire_enabled(instance: u32) -> bool;
	fn prop_set_fire_enabled(instance: u32, value: bool);
	fn prop_fire_heat(instance: u32) -> f64;
	fn prop_set_fire_heat(instance: u32, value: f64);
	fn prop_fire_secondary_color(instance: u32) -> Color3;
	fn prop_set_fire_secondary_color(instance: u32, value: Color3);
	fn prop_fire_size(instance: u32) -> f64;
	fn prop_set_fire_size(instance: u32, value: f64);
	fn prop_fire_time_scale(instance: u32) -> f64;
	fn prop_set_fire_time_scale(instance: u32, value: f64);
	fn dyn_float_curve_get_key_at_index(instance: u32, p_index: f64) -> FloatCurveKey;
	fn dyn_float_curve_get_key_indices_at_time(instance: u32, p_time: f64);
	fn dyn_float_curve_get_keys(instance: u32);
	fn dyn_float_curve_get_value_at_time(instance: u32, p_time: f64);
	fn dyn_float_curve_insert_key(instance: u32, p_key: FloatCurveKey);
	fn dyn_float_curve_remove_key_at_index(instance: u32, p_startingIndex: f64, p_count: f64) -> f64;
	fn dyn_float_curve_set_keys(instance: u32) -> f64;
	fn prop_float_curve_length(instance: u32) -> f64;
	fn prop_force_field_visible(instance: u32) -> bool;
	fn prop_set_force_field_visible(instance: u32, value: bool);
	fn dyn_game_pass_service_player_has_pass(instance: u32, p_player: Option<Player>, p_gamePassId: f64) -> bool;
	fn prop_get_text_bounds_params_font(instance: u32) -> Font;
	fn prop_set_get_text_bounds_params_font(instance: u32, value: Font);
	fn prop_get_text_bounds_params_size(instance: u32) -> f64;
	fn prop_set_get_text_bounds_params_size(instance: u32, value: f64);
	fn prop_get_text_bounds_params_text(instance: u32) -> String;
	fn prop_set_get_text_bounds_params_text(instance: u32, value: &str);
	fn prop_get_text_bounds_params_width(instance: u32) -> f64;
	fn prop_set_get_text_bounds_params_width(instance: u32, value: f64);
	fn dyn_global_data_store_on_update(instance: u32, p_key: &str, p_callback: Function) -> RbxScriptConnection;
	fn dyn_global_data_store_get_async(instance: u32, p_key: &str);
	fn dyn_global_data_store_increment_async(instance: u32, p_key: &str, p_delta: f64, p_options: Option<DataStoreIncrementOptions>);
	fn dyn_global_data_store_remove_async(instance: u32, p_key: &str);
	fn dyn_global_data_store_set_async(instance: u32, p_key: &str, p_options: Option<DataStoreSetOptions>);
	fn dyn_global_data_store_update_async(instance: u32, p_key: &str, p_transformFunction: Function);
	fn dyn_data_store_get_version_async(instance: u32, p_key: &str, p_version: &str);
	fn dyn_data_store_list_keys_async(instance: u32, p_prefix: &str, p_pageSize: f64) -> Option<DataStoreKeyPages>;
	fn dyn_data_store_list_versions_async(instance: u32, p_key: &str, p_minDate: f64, p_maxDate: f64, p_pageSize: f64) -> Option<DataStoreVersionPages>;
	fn dyn_data_store_remove_version_async(instance: u32, p_key: &str, p_version: &str);
	fn dyn_ordered_data_store_get_sorted_async(instance: u32, p_ascending: bool, p_pagesize: f64) -> Option<Instance>;
	fn dyn_group_service_get_allies_async(instance: u32, p_groupId: f64) -> Option<StandardPages>;
	fn dyn_group_service_get_enemies_async(instance: u32, p_groupId: f64) -> Option<StandardPages>;
	fn dyn_group_service_get_group_info_async(instance: u32, p_groupId: f64);
	fn dyn_group_service_get_groups_async(instance: u32, p_userId: f64);
	fn prop_gui_base_2_d_absolute_position(instance: u32) -> Vector2;
	fn prop_gui_base_2_d_absolute_rotation(instance: u32) -> f64;
	fn prop_gui_base_2_d_absolute_size(instance: u32) -> Vector2;
	fn prop_gui_base_2_d_auto_localize(instance: u32) -> bool;
	fn prop_set_gui_base_2_d_auto_localize(instance: u32, value: bool);
	fn prop_gui_base_2_d_localize(instance: u32) -> bool;
	fn prop_set_gui_base_2_d_localize(instance: u32, value: bool);
	fn prop_gui_base_2_d_root_localization_table(instance: u32) -> Option<LocalizationTable>;
	fn prop_set_gui_base_2_d_root_localization_table(instance: u32, value: Option<LocalizationTable>);
	fn prop_gui_base_2_d_selection_group(instance: u32) -> bool;
	fn prop_set_gui_base_2_d_selection_group(instance: u32, value: bool);
	fn connect_gui_base_2_d_selection_changed(instance: u32, callback: Box<dyn Fn(bool, Option<GuiObject>, Option<GuiObject>)>) -> u32;
	fn dyn_gui_object_tween_position(instance: u32, p_endPosition: UDim2, p_time: f64, p_override: bool, p_callback: Function) -> bool;
	fn dyn_gui_object_tween_size(instance: u32, p_endSize: UDim2, p_time: f64, p_override: bool, p_callback: Function) -> bool;
	fn dyn_gui_object_tween_size_and_position(instance: u32, p_endSize: UDim2, p_endPosition: UDim2, p_time: f64, p_override: bool, p_callback: Function) -> bool;
	fn prop_gui_object_active(instance: u32) -> bool;
	fn prop_set_gui_object_active(instance: u32, value: bool);
	fn prop_gui_object_anchor_point(instance: u32) -> Vector2;
	fn prop_set_gui_object_anchor_point(instance: u32, value: Vector2);
	fn prop_gui_object_background_color(instance: u32) -> BrickColor;
	fn prop_set_gui_object_background_color(instance: u32, value: BrickColor);
	fn prop_gui_object_background_color_3(instance: u32) -> Color3;
	fn prop_set_gui_object_background_color_3(instance: u32, value: Color3);
	fn prop_gui_object_background_transparency(instance: u32) -> f64;
	fn prop_set_gui_object_background_transparency(instance: u32, value: f64);
	fn prop_gui_object_border_color(instance: u32) -> BrickColor;
	fn prop_set_gui_object_border_color(instance: u32, value: BrickColor);
	fn prop_gui_object_border_color_3(instance: u32) -> Color3;
	fn prop_set_gui_object_border_color_3(instance: u32, value: Color3);
	fn prop_gui_object_border_size_pixel(instance: u32) -> f64;
	fn prop_set_gui_object_border_size_pixel(instance: u32, value: f64);
	fn prop_gui_object_clips_descendants(instance: u32) -> bool;
	fn prop_set_gui_object_clips_descendants(instance: u32, value: bool);
	fn prop_gui_object_draggable(instance: u32) -> bool;
	fn prop_set_gui_object_draggable(instance: u32, value: bool);
	fn prop_gui_object_layout_order(instance: u32) -> f64;
	fn prop_set_gui_object_layout_order(instance: u32, value: f64);
	fn prop_gui_object_next_selection_down(instance: u32) -> Option<GuiObject>;
	fn prop_set_gui_object_next_selection_down(instance: u32, value: Option<GuiObject>);
	fn prop_gui_object_next_selection_left(instance: u32) -> Option<GuiObject>;
	fn prop_set_gui_object_next_selection_left(instance: u32, value: Option<GuiObject>);
	fn prop_gui_object_next_selection_right(instance: u32) -> Option<GuiObject>;
	fn prop_set_gui_object_next_selection_right(instance: u32, value: Option<GuiObject>);
	fn prop_gui_object_next_selection_up(instance: u32) -> Option<GuiObject>;
	fn prop_set_gui_object_next_selection_up(instance: u32, value: Option<GuiObject>);
	fn prop_gui_object_position(instance: u32) -> UDim2;
	fn prop_set_gui_object_position(instance: u32, value: UDim2);
	fn prop_gui_object_rotation(instance: u32) -> f64;
	fn prop_set_gui_object_rotation(instance: u32, value: f64);
	fn prop_gui_object_selectable(instance: u32) -> bool;
	fn prop_set_gui_object_selectable(instance: u32, value: bool);
	fn prop_gui_object_selection_image_object(instance: u32) -> Option<GuiObject>;
	fn prop_set_gui_object_selection_image_object(instance: u32, value: Option<GuiObject>);
	fn prop_gui_object_selection_order(instance: u32) -> f64;
	fn prop_set_gui_object_selection_order(instance: u32, value: f64);
	fn prop_gui_object_size(instance: u32) -> UDim2;
	fn prop_set_gui_object_size(instance: u32, value: UDim2);
	fn prop_gui_object_transparency(instance: u32) -> f64;
	fn prop_set_gui_object_transparency(instance: u32, value: f64);
	fn prop_gui_object_visible(instance: u32) -> bool;
	fn prop_set_gui_object_visible(instance: u32, value: bool);
	fn prop_gui_object_z_index(instance: u32) -> f64;
	fn prop_set_gui_object_z_index(instance: u32, value: f64);
	fn connect_gui_object_drag_begin(instance: u32, callback: Box<dyn Fn(UDim2)>) -> u32;
	fn connect_gui_object_drag_stopped(instance: u32, callback: Box<dyn Fn(f64, f64)>) -> u32;
	fn connect_gui_object_input_began(instance: u32, callback: Box<dyn Fn(Option<InputObject>)>) -> u32;
	fn connect_gui_object_input_changed(instance: u32, callback: Box<dyn Fn(Option<InputObject>)>) -> u32;
	fn connect_gui_object_input_ended(instance: u32, callback: Box<dyn Fn(Option<InputObject>)>) -> u32;
	fn connect_gui_object_mouse_enter(instance: u32, callback: Box<dyn Fn(f64, f64)>) -> u32;
	fn connect_gui_object_mouse_leave(instance: u32, callback: Box<dyn Fn(f64, f64)>) -> u32;
	fn connect_gui_object_mouse_moved(instance: u32, callback: Box<dyn Fn(f64, f64)>) -> u32;
	fn connect_gui_object_mouse_wheel_backward(instance: u32, callback: Box<dyn Fn(f64, f64)>) -> u32;
	fn connect_gui_object_mouse_wheel_forward(instance: u32, callback: Box<dyn Fn(f64, f64)>) -> u32;
	fn connect_gui_object_selection_gained(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_gui_object_selection_lost(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_gui_object_touch_long_press(instance: u32, callback: Box<dyn Fn((), ())>) -> u32;
	fn connect_gui_object_touch_pan(instance: u32, callback: Box<dyn Fn((), Vector2, Vector2, ())>) -> u32;
	fn connect_gui_object_touch_pinch(instance: u32, callback: Box<dyn Fn((), f64, f64, ())>) -> u32;
	fn connect_gui_object_touch_rotate(instance: u32, callback: Box<dyn Fn((), f64, f64, ())>) -> u32;
	fn connect_gui_object_touch_swipe(instance: u32, callback: Box<dyn Fn((), f64)>) -> u32;
	fn connect_gui_object_touch_tap(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn prop_canvas_group_group_color_3(instance: u32) -> Color3;
	fn prop_set_canvas_group_group_color_3(instance: u32, value: Color3);
	fn prop_canvas_group_group_transparency(instance: u32) -> f64;
	fn prop_set_canvas_group_group_transparency(instance: u32, value: f64);
	fn prop_gui_button_auto_button_color(instance: u32) -> bool;
	fn prop_set_gui_button_auto_button_color(instance: u32, value: bool);
	fn prop_gui_button_modal(instance: u32) -> bool;
	fn prop_set_gui_button_modal(instance: u32, value: bool);
	fn prop_gui_button_selected(instance: u32) -> bool;
	fn prop_set_gui_button_selected(instance: u32, value: bool);
	fn connect_gui_button_activated(instance: u32, callback: Box<dyn Fn(Option<InputObject>, f64)>) -> u32;
	fn connect_gui_button_mouse_button_1_click(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_gui_button_mouse_button_1_down(instance: u32, callback: Box<dyn Fn(f64, f64)>) -> u32;
	fn connect_gui_button_mouse_button_1_up(instance: u32, callback: Box<dyn Fn(f64, f64)>) -> u32;
	fn connect_gui_button_mouse_button_2_click(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_gui_button_mouse_button_2_down(instance: u32, callback: Box<dyn Fn(f64, f64)>) -> u32;
	fn connect_gui_button_mouse_button_2_up(instance: u32, callback: Box<dyn Fn(f64, f64)>) -> u32;
	fn prop_image_button_hover_image(instance: u32) -> Content;
	fn prop_set_image_button_hover_image(instance: u32, value: Content);
	fn prop_image_button_image(instance: u32) -> Content;
	fn prop_set_image_button_image(instance: u32, value: Content);
	fn prop_image_button_image_color_3(instance: u32) -> Color3;
	fn prop_set_image_button_image_color_3(instance: u32, value: Color3);
	fn prop_image_button_image_rect_offset(instance: u32) -> Vector2;
	fn prop_set_image_button_image_rect_offset(instance: u32, value: Vector2);
	fn prop_image_button_image_rect_size(instance: u32) -> Vector2;
	fn prop_set_image_button_image_rect_size(instance: u32, value: Vector2);
	fn prop_image_button_image_transparency(instance: u32) -> f64;
	fn prop_set_image_button_image_transparency(instance: u32, value: f64);
	fn prop_image_button_is_loaded(instance: u32) -> bool;
	fn prop_image_button_pressed_image(instance: u32) -> Content;
	fn prop_set_image_button_pressed_image(instance: u32, value: Content);
	fn prop_image_button_slice_center(instance: u32) -> Rect;
	fn prop_set_image_button_slice_center(instance: u32, value: Rect);
	fn prop_image_button_slice_scale(instance: u32) -> f64;
	fn prop_set_image_button_slice_scale(instance: u32, value: f64);
	fn prop_image_button_tile_size(instance: u32) -> UDim2;
	fn prop_set_image_button_tile_size(instance: u32, value: UDim2);
	fn prop_text_button_content_text(instance: u32) -> String;
	fn prop_text_button_font_face(instance: u32) -> Font;
	fn prop_set_text_button_font_face(instance: u32, value: Font);
	fn prop_text_button_line_height(instance: u32) -> f64;
	fn prop_set_text_button_line_height(instance: u32, value: f64);
	fn prop_text_button_localized_text(instance: u32) -> String;
	fn prop_text_button_max_visible_graphemes(instance: u32) -> f64;
	fn prop_set_text_button_max_visible_graphemes(instance: u32, value: f64);
	fn prop_text_button_rich_text(instance: u32) -> bool;
	fn prop_set_text_button_rich_text(instance: u32, value: bool);
	fn prop_text_button_text(instance: u32) -> String;
	fn prop_set_text_button_text(instance: u32, value: &str);
	fn prop_text_button_text_bounds(instance: u32) -> Vector2;
	fn prop_text_button_text_color(instance: u32) -> BrickColor;
	fn prop_set_text_button_text_color(instance: u32, value: BrickColor);
	fn prop_text_button_text_color_3(instance: u32) -> Color3;
	fn prop_set_text_button_text_color_3(instance: u32, value: Color3);
	fn prop_text_button_text_fits(instance: u32) -> bool;
	fn prop_text_button_text_scaled(instance: u32) -> bool;
	fn prop_set_text_button_text_scaled(instance: u32, value: bool);
	fn prop_text_button_text_size(instance: u32) -> f64;
	fn prop_set_text_button_text_size(instance: u32, value: f64);
	fn prop_text_button_text_stroke_color_3(instance: u32) -> Color3;
	fn prop_set_text_button_text_stroke_color_3(instance: u32, value: Color3);
	fn prop_text_button_text_stroke_transparency(instance: u32) -> f64;
	fn prop_set_text_button_text_stroke_transparency(instance: u32, value: f64);
	fn prop_text_button_text_transparency(instance: u32) -> f64;
	fn prop_set_text_button_text_transparency(instance: u32, value: f64);
	fn prop_text_button_text_wrap(instance: u32) -> bool;
	fn prop_set_text_button_text_wrap(instance: u32, value: bool);
	fn prop_text_button_text_wrapped(instance: u32) -> bool;
	fn prop_set_text_button_text_wrapped(instance: u32, value: bool);
	fn prop_image_label_image(instance: u32) -> Content;
	fn prop_set_image_label_image(instance: u32, value: Content);
	fn prop_image_label_image_color_3(instance: u32) -> Color3;
	fn prop_set_image_label_image_color_3(instance: u32, value: Color3);
	fn prop_image_label_image_rect_offset(instance: u32) -> Vector2;
	fn prop_set_image_label_image_rect_offset(instance: u32, value: Vector2);
	fn prop_image_label_image_rect_size(instance: u32) -> Vector2;
	fn prop_set_image_label_image_rect_size(instance: u32, value: Vector2);
	fn prop_image_label_image_transparency(instance: u32) -> f64;
	fn prop_set_image_label_image_transparency(instance: u32, value: f64);
	fn prop_image_label_is_loaded(instance: u32) -> bool;
	fn prop_image_label_slice_center(instance: u32) -> Rect;
	fn prop_set_image_label_slice_center(instance: u32, value: Rect);
	fn prop_image_label_slice_scale(instance: u32) -> f64;
	fn prop_set_image_label_slice_scale(instance: u32, value: f64);
	fn prop_image_label_tile_size(instance: u32) -> UDim2;
	fn prop_set_image_label_tile_size(instance: u32, value: UDim2);
	fn prop_text_label_content_text(instance: u32) -> String;
	fn prop_text_label_font_face(instance: u32) -> Font;
	fn prop_set_text_label_font_face(instance: u32, value: Font);
	fn prop_text_label_line_height(instance: u32) -> f64;
	fn prop_set_text_label_line_height(instance: u32, value: f64);
	fn prop_text_label_localized_text(instance: u32) -> String;
	fn prop_text_label_max_visible_graphemes(instance: u32) -> f64;
	fn prop_set_text_label_max_visible_graphemes(instance: u32, value: f64);
	fn prop_text_label_rich_text(instance: u32) -> bool;
	fn prop_set_text_label_rich_text(instance: u32, value: bool);
	fn prop_text_label_text(instance: u32) -> String;
	fn prop_set_text_label_text(instance: u32, value: &str);
	fn prop_text_label_text_bounds(instance: u32) -> Vector2;
	fn prop_text_label_text_color(instance: u32) -> BrickColor;
	fn prop_set_text_label_text_color(instance: u32, value: BrickColor);
	fn prop_text_label_text_color_3(instance: u32) -> Color3;
	fn prop_set_text_label_text_color_3(instance: u32, value: Color3);
	fn prop_text_label_text_fits(instance: u32) -> bool;
	fn prop_text_label_text_scaled(instance: u32) -> bool;
	fn prop_set_text_label_text_scaled(instance: u32, value: bool);
	fn prop_text_label_text_size(instance: u32) -> f64;
	fn prop_set_text_label_text_size(instance: u32, value: f64);
	fn prop_text_label_text_stroke_color_3(instance: u32) -> Color3;
	fn prop_set_text_label_text_stroke_color_3(instance: u32, value: Color3);
	fn prop_text_label_text_stroke_transparency(instance: u32) -> f64;
	fn prop_set_text_label_text_stroke_transparency(instance: u32, value: f64);
	fn prop_text_label_text_transparency(instance: u32) -> f64;
	fn prop_set_text_label_text_transparency(instance: u32, value: f64);
	fn prop_text_label_text_wrap(instance: u32) -> bool;
	fn prop_set_text_label_text_wrap(instance: u32, value: bool);
	fn prop_text_label_text_wrapped(instance: u32) -> bool;
	fn prop_set_text_label_text_wrapped(instance: u32, value: bool);
	fn prop_scrolling_frame_absolute_canvas_size(instance: u32) -> Vector2;
	fn prop_scrolling_frame_absolute_window_size(instance: u32) -> Vector2;
	fn prop_scrolling_frame_bottom_image(instance: u32) -> Content;
	fn prop_set_scrolling_frame_bottom_image(instance: u32, value: Content);
	fn prop_scrolling_frame_canvas_position(instance: u32) -> Vector2;
	fn prop_set_scrolling_frame_canvas_position(instance: u32, value: Vector2);
	fn prop_scrolling_frame_canvas_size(instance: u32) -> UDim2;
	fn prop_set_scrolling_frame_canvas_size(instance: u32, value: UDim2);
	fn prop_scrolling_frame_mid_image(instance: u32) -> Content;
	fn prop_set_scrolling_frame_mid_image(instance: u32, value: Content);
	fn prop_scrolling_frame_scroll_bar_image_color_3(instance: u32) -> Color3;
	fn prop_set_scrolling_frame_scroll_bar_image_color_3(instance: u32, value: Color3);
	fn prop_scrolling_frame_scroll_bar_image_transparency(instance: u32) -> f64;
	fn prop_set_scrolling_frame_scroll_bar_image_transparency(instance: u32, value: f64);
	fn prop_scrolling_frame_scroll_bar_thickness(instance: u32) -> f64;
	fn prop_set_scrolling_frame_scroll_bar_thickness(instance: u32, value: f64);
	fn prop_scrolling_frame_scrolling_enabled(instance: u32) -> bool;
	fn prop_set_scrolling_frame_scrolling_enabled(instance: u32, value: bool);
	fn prop_scrolling_frame_top_image(instance: u32) -> Content;
	fn prop_set_scrolling_frame_top_image(instance: u32, value: Content);
	fn dyn_text_box_capture_focus(instance: u32);
	fn dyn_text_box_is_focused(instance: u32) -> bool;
	fn dyn_text_box_release_focus(instance: u32, p_submitted: bool);
	fn prop_text_box_clear_text_on_focus(instance: u32) -> bool;
	fn prop_set_text_box_clear_text_on_focus(instance: u32, value: bool);
	fn prop_text_box_content_text(instance: u32) -> String;
	fn prop_text_box_cursor_position(instance: u32) -> f64;
	fn prop_set_text_box_cursor_position(instance: u32, value: f64);
	fn prop_text_box_font_face(instance: u32) -> Font;
	fn prop_set_text_box_font_face(instance: u32, value: Font);
	fn prop_text_box_line_height(instance: u32) -> f64;
	fn prop_set_text_box_line_height(instance: u32, value: f64);
	fn prop_text_box_max_visible_graphemes(instance: u32) -> f64;
	fn prop_set_text_box_max_visible_graphemes(instance: u32, value: f64);
	fn prop_text_box_multi_line(instance: u32) -> bool;
	fn prop_set_text_box_multi_line(instance: u32, value: bool);
	fn prop_text_box_placeholder_color_3(instance: u32) -> Color3;
	fn prop_set_text_box_placeholder_color_3(instance: u32, value: Color3);
	fn prop_text_box_placeholder_text(instance: u32) -> String;
	fn prop_set_text_box_placeholder_text(instance: u32, value: &str);
	fn prop_text_box_rich_text(instance: u32) -> bool;
	fn prop_set_text_box_rich_text(instance: u32, value: bool);
	fn prop_text_box_selection_start(instance: u32) -> f64;
	fn prop_set_text_box_selection_start(instance: u32, value: f64);
	fn prop_text_box_show_native_input(instance: u32) -> bool;
	fn prop_set_text_box_show_native_input(instance: u32, value: bool);
	fn prop_text_box_text(instance: u32) -> String;
	fn prop_set_text_box_text(instance: u32, value: &str);
	fn prop_text_box_text_bounds(instance: u32) -> Vector2;
	fn prop_text_box_text_color(instance: u32) -> BrickColor;
	fn prop_set_text_box_text_color(instance: u32, value: BrickColor);
	fn prop_text_box_text_color_3(instance: u32) -> Color3;
	fn prop_set_text_box_text_color_3(instance: u32, value: Color3);
	fn prop_text_box_text_editable(instance: u32) -> bool;
	fn prop_set_text_box_text_editable(instance: u32, value: bool);
	fn prop_text_box_text_fits(instance: u32) -> bool;
	fn prop_text_box_text_scaled(instance: u32) -> bool;
	fn prop_set_text_box_text_scaled(instance: u32, value: bool);
	fn prop_text_box_text_size(instance: u32) -> f64;
	fn prop_set_text_box_text_size(instance: u32, value: f64);
	fn prop_text_box_text_stroke_color_3(instance: u32) -> Color3;
	fn prop_set_text_box_text_stroke_color_3(instance: u32, value: Color3);
	fn prop_text_box_text_stroke_transparency(instance: u32) -> f64;
	fn prop_set_text_box_text_stroke_transparency(instance: u32, value: f64);
	fn prop_text_box_text_transparency(instance: u32) -> f64;
	fn prop_set_text_box_text_transparency(instance: u32, value: f64);
	fn prop_text_box_text_wrap(instance: u32) -> bool;
	fn prop_set_text_box_text_wrap(instance: u32, value: bool);
	fn prop_text_box_text_wrapped(instance: u32) -> bool;
	fn prop_set_text_box_text_wrapped(instance: u32, value: bool);
	fn connect_text_box_focus_lost(instance: u32, callback: Box<dyn Fn(bool, Option<InputObject>)>) -> u32;
	fn connect_text_box_focused(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_text_box_return_pressed_from_on_screen_keyboard(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn dyn_video_frame_pause(instance: u32);
	fn dyn_video_frame_play(instance: u32);
	fn prop_video_frame_is_loaded(instance: u32) -> bool;
	fn prop_video_frame_looped(instance: u32) -> bool;
	fn prop_set_video_frame_looped(instance: u32, value: bool);
	fn prop_video_frame_playing(instance: u32) -> bool;
	fn prop_set_video_frame_playing(instance: u32, value: bool);
	fn prop_video_frame_resolution(instance: u32) -> Vector2;
	fn prop_video_frame_time_length(instance: u32) -> f64;
	fn prop_video_frame_time_position(instance: u32) -> f64;
	fn prop_set_video_frame_time_position(instance: u32, value: f64);
	fn prop_video_frame_video(instance: u32) -> Content;
	fn prop_set_video_frame_video(instance: u32, value: Content);
	fn prop_video_frame_volume(instance: u32) -> f64;
	fn prop_set_video_frame_volume(instance: u32, value: f64);
	fn connect_video_frame_did_loop(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_video_frame_ended(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_video_frame_loaded(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_video_frame_paused(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_video_frame_played(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn prop_viewport_frame_ambient(instance: u32) -> Color3;
	fn prop_set_viewport_frame_ambient(instance: u32, value: Color3);
	fn prop_viewport_frame_current_camera(instance: u32) -> Option<Camera>;
	fn prop_set_viewport_frame_current_camera(instance: u32, value: Option<Camera>);
	fn prop_viewport_frame_image_color_3(instance: u32) -> Color3;
	fn prop_set_viewport_frame_image_color_3(instance: u32, value: Color3);
	fn prop_viewport_frame_image_transparency(instance: u32) -> f64;
	fn prop_set_viewport_frame_image_transparency(instance: u32, value: f64);
	fn prop_viewport_frame_light_color(instance: u32) -> Color3;
	fn prop_set_viewport_frame_light_color(instance: u32, value: Color3);
	fn prop_viewport_frame_light_direction(instance: u32) -> Vector3;
	fn prop_set_viewport_frame_light_direction(instance: u32, value: Vector3);
	fn dyn_layer_collector_get_layout_node_tree(instance: u32);
	fn prop_layer_collector_enabled(instance: u32) -> bool;
	fn prop_set_layer_collector_enabled(instance: u32, value: bool);
	fn prop_layer_collector_reset_on_spawn(instance: u32) -> bool;
	fn prop_set_layer_collector_reset_on_spawn(instance: u32, value: bool);
	fn prop_billboard_gui_active(instance: u32) -> bool;
	fn prop_set_billboard_gui_active(instance: u32, value: bool);
	fn prop_billboard_gui_adornee(instance: u32) -> Option<Instance>;
	fn prop_set_billboard_gui_adornee(instance: u32, value: Option<Instance>);
	fn prop_billboard_gui_always_on_top(instance: u32) -> bool;
	fn prop_set_billboard_gui_always_on_top(instance: u32, value: bool);
	fn prop_billboard_gui_brightness(instance: u32) -> f64;
	fn prop_set_billboard_gui_brightness(instance: u32, value: f64);
	fn prop_billboard_gui_clips_descendants(instance: u32) -> bool;
	fn prop_set_billboard_gui_clips_descendants(instance: u32, value: bool);
	fn prop_billboard_gui_current_distance(instance: u32) -> f64;
	fn prop_billboard_gui_distance_lower_limit(instance: u32) -> f64;
	fn prop_set_billboard_gui_distance_lower_limit(instance: u32, value: f64);
	fn prop_billboard_gui_distance_step(instance: u32) -> f64;
	fn prop_set_billboard_gui_distance_step(instance: u32, value: f64);
	fn prop_billboard_gui_distance_upper_limit(instance: u32) -> f64;
	fn prop_set_billboard_gui_distance_upper_limit(instance: u32, value: f64);
	fn prop_billboard_gui_extents_offset(instance: u32) -> Vector3;
	fn prop_set_billboard_gui_extents_offset(instance: u32, value: Vector3);
	fn prop_billboard_gui_extents_offset_world_space(instance: u32) -> Vector3;
	fn prop_set_billboard_gui_extents_offset_world_space(instance: u32, value: Vector3);
	fn prop_billboard_gui_light_influence(instance: u32) -> f64;
	fn prop_set_billboard_gui_light_influence(instance: u32, value: f64);
	fn prop_billboard_gui_max_distance(instance: u32) -> f64;
	fn prop_set_billboard_gui_max_distance(instance: u32, value: f64);
	fn prop_billboard_gui_player_to_hide_from(instance: u32) -> Option<Instance>;
	fn prop_set_billboard_gui_player_to_hide_from(instance: u32, value: Option<Instance>);
	fn prop_billboard_gui_size(instance: u32) -> UDim2;
	fn prop_set_billboard_gui_size(instance: u32, value: UDim2);
	fn prop_billboard_gui_size_offset(instance: u32) -> Vector2;
	fn prop_set_billboard_gui_size_offset(instance: u32, value: Vector2);
	fn prop_billboard_gui_studs_offset(instance: u32) -> Vector3;
	fn prop_set_billboard_gui_studs_offset(instance: u32, value: Vector3);
	fn prop_billboard_gui_studs_offset_world_space(instance: u32) -> Vector3;
	fn prop_set_billboard_gui_studs_offset_world_space(instance: u32, value: Vector3);
	fn prop_screen_gui_display_order(instance: u32) -> f64;
	fn prop_set_screen_gui_display_order(instance: u32, value: f64);
	fn prop_screen_gui_ignore_gui_inset(instance: u32) -> bool;
	fn prop_set_screen_gui_ignore_gui_inset(instance: u32, value: bool);
	fn prop_surface_gui_active(instance: u32) -> bool;
	fn prop_set_surface_gui_active(instance: u32, value: bool);
	fn prop_surface_gui_adornee(instance: u32) -> Option<Instance>;
	fn prop_set_surface_gui_adornee(instance: u32, value: Option<Instance>);
	fn prop_surface_gui_always_on_top(instance: u32) -> bool;
	fn prop_set_surface_gui_always_on_top(instance: u32, value: bool);
	fn prop_surface_gui_brightness(instance: u32) -> f64;
	fn prop_set_surface_gui_brightness(instance: u32, value: f64);
	fn prop_surface_gui_canvas_size(instance: u32) -> Vector2;
	fn prop_set_surface_gui_canvas_size(instance: u32, value: Vector2);
	fn prop_surface_gui_clips_descendants(instance: u32) -> bool;
	fn prop_set_surface_gui_clips_descendants(instance: u32, value: bool);
	fn prop_surface_gui_light_influence(instance: u32) -> f64;
	fn prop_set_surface_gui_light_influence(instance: u32, value: f64);
	fn prop_surface_gui_pixels_per_stud(instance: u32) -> f64;
	fn prop_set_surface_gui_pixels_per_stud(instance: u32, value: f64);
	fn prop_surface_gui_tool_punch_through_distance(instance: u32) -> f64;
	fn prop_set_surface_gui_tool_punch_through_distance(instance: u32, value: f64);
	fn prop_surface_gui_z_offset(instance: u32) -> f64;
	fn prop_set_surface_gui_z_offset(instance: u32, value: f64);
	fn prop_gui_base_3_d_color(instance: u32) -> BrickColor;
	fn prop_set_gui_base_3_d_color(instance: u32, value: BrickColor);
	fn prop_gui_base_3_d_color_3(instance: u32) -> Color3;
	fn prop_set_gui_base_3_d_color_3(instance: u32, value: Color3);
	fn prop_gui_base_3_d_transparency(instance: u32) -> f64;
	fn prop_set_gui_base_3_d_transparency(instance: u32, value: f64);
	fn prop_gui_base_3_d_visible(instance: u32) -> bool;
	fn prop_set_gui_base_3_d_visible(instance: u32, value: bool);
	fn prop_floor_wire_cycle_offset(instance: u32) -> f64;
	fn prop_set_floor_wire_cycle_offset(instance: u32, value: f64);
	fn prop_floor_wire_from(instance: u32) -> Option<BasePart>;
	fn prop_set_floor_wire_from(instance: u32, value: Option<BasePart>);
	fn prop_floor_wire_studs_between_textures(instance: u32) -> f64;
	fn prop_set_floor_wire_studs_between_textures(instance: u32, value: f64);
	fn prop_floor_wire_texture(instance: u32) -> Content;
	fn prop_set_floor_wire_texture(instance: u32, value: Content);
	fn prop_floor_wire_texture_size(instance: u32) -> Vector2;
	fn prop_set_floor_wire_texture_size(instance: u32, value: Vector2);
	fn prop_floor_wire_to(instance: u32) -> Option<BasePart>;
	fn prop_set_floor_wire_to(instance: u32, value: Option<BasePart>);
	fn prop_floor_wire_velocity(instance: u32) -> f64;
	fn prop_set_floor_wire_velocity(instance: u32, value: f64);
	fn prop_floor_wire_wire_radius(instance: u32) -> f64;
	fn prop_set_floor_wire_wire_radius(instance: u32, value: f64);
	fn prop_instance_adornment_adornee(instance: u32) -> Option<Instance>;
	fn prop_set_instance_adornment_adornee(instance: u32, value: Option<Instance>);
	fn prop_selection_box_line_thickness(instance: u32) -> f64;
	fn prop_set_selection_box_line_thickness(instance: u32, value: f64);
	fn prop_selection_box_surface_color(instance: u32) -> BrickColor;
	fn prop_set_selection_box_surface_color(instance: u32, value: BrickColor);
	fn prop_selection_box_surface_color_3(instance: u32) -> Color3;
	fn prop_set_selection_box_surface_color_3(instance: u32, value: Color3);
	fn prop_selection_box_surface_transparency(instance: u32) -> f64;
	fn prop_set_selection_box_surface_transparency(instance: u32, value: f64);
	fn prop_pv_adornment_adornee(instance: u32) -> Option<PVInstance>;
	fn prop_set_pv_adornment_adornee(instance: u32, value: Option<PVInstance>);
	fn prop_handle_adornment_always_on_top(instance: u32) -> bool;
	fn prop_set_handle_adornment_always_on_top(instance: u32, value: bool);
	fn prop_handle_adornment_c_frame(instance: u32) -> CFrame;
	fn prop_set_handle_adornment_c_frame(instance: u32, value: CFrame);
	fn prop_handle_adornment_size_relative_offset(instance: u32) -> Vector3;
	fn prop_set_handle_adornment_size_relative_offset(instance: u32, value: Vector3);
	fn prop_handle_adornment_z_index(instance: u32) -> f64;
	fn prop_set_handle_adornment_z_index(instance: u32, value: f64);
	fn connect_handle_adornment_mouse_button_1_down(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_handle_adornment_mouse_button_1_up(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_handle_adornment_mouse_enter(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_handle_adornment_mouse_leave(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn prop_box_handle_adornment_size(instance: u32) -> Vector3;
	fn prop_set_box_handle_adornment_size(instance: u32, value: Vector3);
	fn prop_cone_handle_adornment_height(instance: u32) -> f64;
	fn prop_set_cone_handle_adornment_height(instance: u32, value: f64);
	fn prop_cone_handle_adornment_radius(instance: u32) -> f64;
	fn prop_set_cone_handle_adornment_radius(instance: u32, value: f64);
	fn prop_cylinder_handle_adornment_angle(instance: u32) -> f64;
	fn prop_set_cylinder_handle_adornment_angle(instance: u32, value: f64);
	fn prop_cylinder_handle_adornment_height(instance: u32) -> f64;
	fn prop_set_cylinder_handle_adornment_height(instance: u32, value: f64);
	fn prop_cylinder_handle_adornment_inner_radius(instance: u32) -> f64;
	fn prop_set_cylinder_handle_adornment_inner_radius(instance: u32, value: f64);
	fn prop_cylinder_handle_adornment_radius(instance: u32) -> f64;
	fn prop_set_cylinder_handle_adornment_radius(instance: u32, value: f64);
	fn prop_image_handle_adornment_image(instance: u32) -> Content;
	fn prop_set_image_handle_adornment_image(instance: u32, value: Content);
	fn prop_image_handle_adornment_size(instance: u32) -> Vector2;
	fn prop_set_image_handle_adornment_size(instance: u32, value: Vector2);
	fn prop_line_handle_adornment_length(instance: u32) -> f64;
	fn prop_set_line_handle_adornment_length(instance: u32, value: f64);
	fn prop_line_handle_adornment_thickness(instance: u32) -> f64;
	fn prop_set_line_handle_adornment_thickness(instance: u32, value: f64);
	fn prop_sphere_handle_adornment_radius(instance: u32) -> f64;
	fn prop_set_sphere_handle_adornment_radius(instance: u32, value: f64);
	fn prop_selection_sphere_surface_color(instance: u32) -> BrickColor;
	fn prop_set_selection_sphere_surface_color(instance: u32, value: BrickColor);
	fn prop_selection_sphere_surface_color_3(instance: u32) -> Color3;
	fn prop_set_selection_sphere_surface_color_3(instance: u32, value: Color3);
	fn prop_selection_sphere_surface_transparency(instance: u32) -> f64;
	fn prop_set_selection_sphere_surface_transparency(instance: u32, value: f64);
	fn prop_part_adornment_adornee(instance: u32) -> Option<BasePart>;
	fn prop_set_part_adornment_adornee(instance: u32, value: Option<BasePart>);
	fn prop_arc_handles_axes(instance: u32) -> Axes;
	fn prop_set_arc_handles_axes(instance: u32, value: Axes);
	fn connect_arc_handles_mouse_button_1_down(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_arc_handles_mouse_button_1_up(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_arc_handles_mouse_drag(instance: u32, callback: Box<dyn Fn((), f64, f64)>) -> u32;
	fn connect_arc_handles_mouse_enter(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_arc_handles_mouse_leave(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn prop_handles_faces(instance: u32) -> Faces;
	fn prop_set_handles_faces(instance: u32, value: Faces);
	fn connect_handles_mouse_button_1_down(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_handles_mouse_button_1_up(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_handles_mouse_drag(instance: u32, callback: Box<dyn Fn((), f64)>) -> u32;
	fn connect_handles_mouse_enter(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_handles_mouse_leave(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn prop_selection_lasso_humanoid(instance: u32) -> Option<Humanoid>;
	fn prop_set_selection_lasso_humanoid(instance: u32, value: Option<Humanoid>);
	fn prop_selection_part_lasso_part(instance: u32) -> Option<BasePart>;
	fn prop_set_selection_part_lasso_part(instance: u32, value: Option<BasePart>);
	fn prop_selection_point_lasso_point(instance: u32) -> Vector3;
	fn prop_set_selection_point_lasso_point(instance: u32, value: Vector3);
	fn dyn_gui_service_add_selection_parent(instance: u32, p_selectionName: &str, p_selectionParent: Option<Instance>);
	fn dyn_gui_service_add_selection_tuple(instance: u32, p_selectionName: &str);
	fn dyn_gui_service_close_inspect_menu(instance: u32);
	fn dyn_gui_service_get_emotes_menu_open(instance: u32) -> bool;
	fn dyn_gui_service_get_gameplay_paused_notification_enabled(instance: u32) -> bool;
	fn dyn_gui_service_get_gui_inset(instance: u32);
	fn dyn_gui_service_get_inspect_menu_enabled(instance: u32) -> bool;
	fn dyn_gui_service_inspect_player_from_humanoid_description(instance: u32, p_humanoidDescription: Option<Instance>, p_name: &str);
	fn dyn_gui_service_inspect_player_from_user_id(instance: u32, p_userId: f64);
	fn dyn_gui_service_is_ten_foot_interface(instance: u32) -> bool;
	fn dyn_gui_service_remove_selection_group(instance: u32, p_selectionName: &str);
	fn dyn_gui_service_select(instance: u32, p_selectionParent: Option<Instance>);
	fn dyn_gui_service_set_emotes_menu_open(instance: u32, p_isOpen: bool);
	fn dyn_gui_service_set_gameplay_paused_notification_enabled(instance: u32, p_enabled: bool);
	fn dyn_gui_service_set_inspect_menu_enabled(instance: u32, p_enabled: bool);
	fn prop_gui_service_auto_select_gui_enabled(instance: u32) -> bool;
	fn prop_set_gui_service_auto_select_gui_enabled(instance: u32, value: bool);
	fn prop_gui_service_core_gui_navigation_enabled(instance: u32) -> bool;
	fn prop_set_gui_service_core_gui_navigation_enabled(instance: u32, value: bool);
	fn prop_gui_service_gui_navigation_enabled(instance: u32) -> bool;
	fn prop_set_gui_service_gui_navigation_enabled(instance: u32, value: bool);
	fn prop_gui_service_is_modal_dialog(instance: u32) -> bool;
	fn prop_gui_service_is_windows(instance: u32) -> bool;
	fn prop_gui_service_menu_is_open(instance: u32) -> bool;
	fn prop_gui_service_selected_object(instance: u32) -> Option<GuiObject>;
	fn prop_set_gui_service_selected_object(instance: u32, value: Option<GuiObject>);
	fn prop_gui_service_touch_controls_enabled(instance: u32) -> bool;
	fn prop_set_gui_service_touch_controls_enabled(instance: u32, value: bool);
	fn connect_gui_service_menu_closed(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_gui_service_menu_opened(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn dyn_haptic_service_get_motor(instance: u32);
	fn dyn_haptic_service_is_motor_supported(instance: u32) -> bool;
	fn dyn_haptic_service_is_vibration_supported(instance: u32) -> bool;
	fn dyn_haptic_service_set_motor(instance: u32);
	fn prop_highlight_adornee(instance: u32) -> Option<Instance>;
	fn prop_set_highlight_adornee(instance: u32, value: Option<Instance>);
	fn prop_highlight_enabled(instance: u32) -> bool;
	fn prop_set_highlight_enabled(instance: u32, value: bool);
	fn prop_highlight_fill_color(instance: u32) -> Color3;
	fn prop_set_highlight_fill_color(instance: u32, value: Color3);
	fn prop_highlight_fill_transparency(instance: u32) -> f64;
	fn prop_set_highlight_fill_transparency(instance: u32, value: f64);
	fn prop_highlight_outline_color(instance: u32) -> Color3;
	fn prop_set_highlight_outline_color(instance: u32, value: Color3);
	fn prop_highlight_outline_transparency(instance: u32) -> f64;
	fn prop_set_highlight_outline_transparency(instance: u32, value: f64);
	fn dyn_http_service_generate_guid(instance: u32, p_wrapInCurlyBraces: bool) -> String;
	fn dyn_http_service_json_decode(instance: u32, p_input: &str);
	fn dyn_http_service_json_encode(instance: u32) -> String;
	fn dyn_http_service_url_encode(instance: u32, p_input: &str) -> String;
	fn dyn_http_service_get_async(instance: u32, p_url: &str, p_nocache: bool) -> String;
	fn dyn_http_service_post_async(instance: u32, p_url: &str, p_data: &str, p_compress: bool) -> String;
	fn dyn_http_service_request_async(instance: u32);
	fn dyn_humanoid_add_accessory(instance: u32, p_accessory: Option<Instance>);
	fn dyn_humanoid_add_custom_status(instance: u32, p_status: &str) -> bool;
	fn dyn_humanoid_add_status(instance: u32) -> bool;
	fn dyn_humanoid_build_rig_from_attachments(instance: u32);
	fn dyn_humanoid_change_state(instance: u32);
	fn dyn_humanoid_equip_tool(instance: u32, p_tool: Option<Instance>);
	fn dyn_humanoid_get_accessories(instance: u32);
	fn dyn_humanoid_get_applied_description(instance: u32) -> Option<HumanoidDescription>;
	fn dyn_humanoid_get_body_part_r_15(instance: u32, p_part: Option<Instance>);
	fn dyn_humanoid_get_limb(instance: u32, p_part: Option<Instance>);
	fn dyn_humanoid_get_playing_animation_tracks(instance: u32);
	fn dyn_humanoid_get_state(instance: u32);
	fn dyn_humanoid_get_state_enabled(instance: u32) -> bool;
	fn dyn_humanoid_get_statuses(instance: u32);
	fn dyn_humanoid_has_custom_status(instance: u32, p_status: &str) -> bool;
	fn dyn_humanoid_has_status(instance: u32) -> bool;
	fn dyn_humanoid_load_animation(instance: u32, p_animation: Option<Animation>) -> Option<AnimationTrack>;
	fn dyn_humanoid_move(instance: u32, p_moveDirection: Vector3, p_relativeToCamera: bool);
	fn dyn_humanoid_move_to(instance: u32, p_location: Vector3, p_part: Option<Instance>);
	fn dyn_humanoid_remove_accessories(instance: u32);
	fn dyn_humanoid_remove_custom_status(instance: u32, p_status: &str) -> bool;
	fn dyn_humanoid_remove_status(instance: u32) -> bool;
	fn dyn_humanoid_replace_body_part_r_15(instance: u32, p_part: Option<BasePart>) -> bool;
	fn dyn_humanoid_set_state_enabled(instance: u32, p_enabled: bool);
	fn dyn_humanoid_take_damage(instance: u32, p_amount: f64);
	fn dyn_humanoid_unequip_tools(instance: u32);
	fn dyn_humanoid_apply_description(instance: u32, p_humanoidDescription: Option<HumanoidDescription>);
	fn dyn_humanoid_apply_description_reset(instance: u32, p_humanoidDescription: Option<HumanoidDescription>);
	fn dyn_humanoid_play_emote(instance: u32, p_emoteName: &str) -> bool;
	fn prop_humanoid_auto_jump_enabled(instance: u32) -> bool;
	fn prop_set_humanoid_auto_jump_enabled(instance: u32, value: bool);
	fn prop_humanoid_auto_rotate(instance: u32) -> bool;
	fn prop_set_humanoid_auto_rotate(instance: u32, value: bool);
	fn prop_humanoid_automatic_scaling_enabled(instance: u32) -> bool;
	fn prop_set_humanoid_automatic_scaling_enabled(instance: u32, value: bool);
	fn prop_humanoid_break_joints_on_death(instance: u32) -> bool;
	fn prop_set_humanoid_break_joints_on_death(instance: u32, value: bool);
	fn prop_humanoid_camera_offset(instance: u32) -> Vector3;
	fn prop_set_humanoid_camera_offset(instance: u32, value: Vector3);
	fn prop_humanoid_display_name(instance: u32) -> String;
	fn prop_set_humanoid_display_name(instance: u32, value: &str);
	fn prop_humanoid_health(instance: u32) -> f64;
	fn prop_set_humanoid_health(instance: u32, value: f64);
	fn prop_humanoid_health_display_distance(instance: u32) -> f64;
	fn prop_set_humanoid_health_display_distance(instance: u32, value: f64);
	fn prop_humanoid_hip_height(instance: u32) -> f64;
	fn prop_set_humanoid_hip_height(instance: u32, value: f64);
	fn prop_humanoid_jump(instance: u32) -> bool;
	fn prop_set_humanoid_jump(instance: u32, value: bool);
	fn prop_humanoid_jump_height(instance: u32) -> f64;
	fn prop_set_humanoid_jump_height(instance: u32, value: f64);
	fn prop_humanoid_jump_power(instance: u32) -> f64;
	fn prop_set_humanoid_jump_power(instance: u32, value: f64);
	fn prop_humanoid_left_leg(instance: u32) -> Option<BasePart>;
	fn prop_set_humanoid_left_leg(instance: u32, value: Option<BasePart>);
	fn prop_humanoid_max_health(instance: u32) -> f64;
	fn prop_set_humanoid_max_health(instance: u32, value: f64);
	fn prop_humanoid_max_slope_angle(instance: u32) -> f64;
	fn prop_set_humanoid_max_slope_angle(instance: u32, value: f64);
	fn prop_humanoid_move_direction(instance: u32) -> Vector3;
	fn prop_humanoid_name_display_distance(instance: u32) -> f64;
	fn prop_set_humanoid_name_display_distance(instance: u32, value: f64);
	fn prop_humanoid_platform_stand(instance: u32) -> bool;
	fn prop_set_humanoid_platform_stand(instance: u32, value: bool);
	fn prop_humanoid_requires_neck(instance: u32) -> bool;
	fn prop_set_humanoid_requires_neck(instance: u32, value: bool);
	fn prop_humanoid_right_leg(instance: u32) -> Option<BasePart>;
	fn prop_set_humanoid_right_leg(instance: u32, value: Option<BasePart>);
	fn prop_humanoid_root_part(instance: u32) -> Option<BasePart>;
	fn prop_humanoid_seat_part(instance: u32) -> Option<BasePart>;
	fn prop_humanoid_sit(instance: u32) -> bool;
	fn prop_set_humanoid_sit(instance: u32, value: bool);
	fn prop_humanoid_target_point(instance: u32) -> Vector3;
	fn prop_set_humanoid_target_point(instance: u32, value: Vector3);
	fn prop_humanoid_torso(instance: u32) -> Option<BasePart>;
	fn prop_set_humanoid_torso(instance: u32, value: Option<BasePart>);
	fn prop_humanoid_use_jump_power(instance: u32) -> bool;
	fn prop_set_humanoid_use_jump_power(instance: u32, value: bool);
	fn prop_humanoid_walk_speed(instance: u32) -> f64;
	fn prop_set_humanoid_walk_speed(instance: u32, value: f64);
	fn prop_humanoid_walk_to_part(instance: u32) -> Option<BasePart>;
	fn prop_set_humanoid_walk_to_part(instance: u32, value: Option<BasePart>);
	fn prop_humanoid_walk_to_point(instance: u32) -> Vector3;
	fn prop_set_humanoid_walk_to_point(instance: u32, value: Vector3);
	fn connect_humanoid_animation_played(instance: u32, callback: Box<dyn Fn(Option<AnimationTrack>)>) -> u32;
	fn connect_humanoid_climbing(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn connect_humanoid_custom_status_added(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_humanoid_custom_status_removed(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_humanoid_died(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_humanoid_falling_down(instance: u32, callback: Box<dyn Fn(bool)>) -> u32;
	fn connect_humanoid_free_falling(instance: u32, callback: Box<dyn Fn(bool)>) -> u32;
	fn connect_humanoid_getting_up(instance: u32, callback: Box<dyn Fn(bool)>) -> u32;
	fn connect_humanoid_health_changed(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn connect_humanoid_jumping(instance: u32, callback: Box<dyn Fn(bool)>) -> u32;
	fn connect_humanoid_move_to_finished(instance: u32, callback: Box<dyn Fn(bool)>) -> u32;
	fn connect_humanoid_platform_standing(instance: u32, callback: Box<dyn Fn(bool)>) -> u32;
	fn connect_humanoid_ragdoll(instance: u32, callback: Box<dyn Fn(bool)>) -> u32;
	fn connect_humanoid_running(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn connect_humanoid_seated(instance: u32, callback: Box<dyn Fn(bool, Option<BasePart>)>) -> u32;
	fn connect_humanoid_state_changed(instance: u32, callback: Box<dyn Fn((), ())>) -> u32;
	fn connect_humanoid_state_enabled_changed(instance: u32, callback: Box<dyn Fn((), bool)>) -> u32;
	fn connect_humanoid_status_added(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_humanoid_status_removed(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_humanoid_strafing(instance: u32, callback: Box<dyn Fn(bool)>) -> u32;
	fn connect_humanoid_swimming(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn connect_humanoid_touched(instance: u32, callback: Box<dyn Fn(Option<BasePart>, Option<BasePart>)>) -> u32;
	fn dyn_humanoid_description_add_emote(instance: u32, p_name: &str, p_assetId: f64);
	fn dyn_humanoid_description_get_accessories(instance: u32, p_includeRigidAccessories: bool);
	fn dyn_humanoid_description_get_emotes(instance: u32);
	fn dyn_humanoid_description_get_equipped_emotes(instance: u32);
	fn dyn_humanoid_description_remove_emote(instance: u32, p_name: &str);
	fn dyn_humanoid_description_set_accessories(instance: u32, p_includeRigidAccessories: bool);
	fn dyn_humanoid_description_set_emotes(instance: u32);
	fn dyn_humanoid_description_set_equipped_emotes(instance: u32);
	fn prop_humanoid_description_accessory_blob(instance: u32) -> String;
	fn prop_set_humanoid_description_accessory_blob(instance: u32, value: &str);
	fn prop_humanoid_description_back_accessory(instance: u32) -> String;
	fn prop_set_humanoid_description_back_accessory(instance: u32, value: &str);
	fn prop_humanoid_description_body_type_scale(instance: u32) -> f64;
	fn prop_set_humanoid_description_body_type_scale(instance: u32, value: f64);
	fn prop_humanoid_description_climb_animation(instance: u32) -> f64;
	fn prop_set_humanoid_description_climb_animation(instance: u32, value: f64);
	fn prop_humanoid_description_depth_scale(instance: u32) -> f64;
	fn prop_set_humanoid_description_depth_scale(instance: u32, value: f64);
	fn prop_humanoid_description_face(instance: u32) -> f64;
	fn prop_set_humanoid_description_face(instance: u32, value: f64);
	fn prop_humanoid_description_face_accessory(instance: u32) -> String;
	fn prop_set_humanoid_description_face_accessory(instance: u32, value: &str);
	fn prop_humanoid_description_fall_animation(instance: u32) -> f64;
	fn prop_set_humanoid_description_fall_animation(instance: u32, value: f64);
	fn prop_humanoid_description_front_accessory(instance: u32) -> String;
	fn prop_set_humanoid_description_front_accessory(instance: u32, value: &str);
	fn prop_humanoid_description_graphic_t_shirt(instance: u32) -> f64;
	fn prop_set_humanoid_description_graphic_t_shirt(instance: u32, value: f64);
	fn prop_humanoid_description_hair_accessory(instance: u32) -> String;
	fn prop_set_humanoid_description_hair_accessory(instance: u32, value: &str);
	fn prop_humanoid_description_hat_accessory(instance: u32) -> String;
	fn prop_set_humanoid_description_hat_accessory(instance: u32, value: &str);
	fn prop_humanoid_description_head(instance: u32) -> f64;
	fn prop_set_humanoid_description_head(instance: u32, value: f64);
	fn prop_humanoid_description_head_color(instance: u32) -> Color3;
	fn prop_set_humanoid_description_head_color(instance: u32, value: Color3);
	fn prop_humanoid_description_head_scale(instance: u32) -> f64;
	fn prop_set_humanoid_description_head_scale(instance: u32, value: f64);
	fn prop_humanoid_description_height_scale(instance: u32) -> f64;
	fn prop_set_humanoid_description_height_scale(instance: u32, value: f64);
	fn prop_humanoid_description_idle_animation(instance: u32) -> f64;
	fn prop_set_humanoid_description_idle_animation(instance: u32, value: f64);
	fn prop_humanoid_description_jump_animation(instance: u32) -> f64;
	fn prop_set_humanoid_description_jump_animation(instance: u32, value: f64);
	fn prop_humanoid_description_left_arm(instance: u32) -> f64;
	fn prop_set_humanoid_description_left_arm(instance: u32, value: f64);
	fn prop_humanoid_description_left_arm_color(instance: u32) -> Color3;
	fn prop_set_humanoid_description_left_arm_color(instance: u32, value: Color3);
	fn prop_humanoid_description_left_leg(instance: u32) -> f64;
	fn prop_set_humanoid_description_left_leg(instance: u32, value: f64);
	fn prop_humanoid_description_left_leg_color(instance: u32) -> Color3;
	fn prop_set_humanoid_description_left_leg_color(instance: u32, value: Color3);
	fn prop_humanoid_description_neck_accessory(instance: u32) -> String;
	fn prop_set_humanoid_description_neck_accessory(instance: u32, value: &str);
	fn prop_humanoid_description_pants(instance: u32) -> f64;
	fn prop_set_humanoid_description_pants(instance: u32, value: f64);
	fn prop_humanoid_description_proportion_scale(instance: u32) -> f64;
	fn prop_set_humanoid_description_proportion_scale(instance: u32, value: f64);
	fn prop_humanoid_description_right_arm(instance: u32) -> f64;
	fn prop_set_humanoid_description_right_arm(instance: u32, value: f64);
	fn prop_humanoid_description_right_arm_color(instance: u32) -> Color3;
	fn prop_set_humanoid_description_right_arm_color(instance: u32, value: Color3);
	fn prop_humanoid_description_right_leg(instance: u32) -> f64;
	fn prop_set_humanoid_description_right_leg(instance: u32, value: f64);
	fn prop_humanoid_description_right_leg_color(instance: u32) -> Color3;
	fn prop_set_humanoid_description_right_leg_color(instance: u32, value: Color3);
	fn prop_humanoid_description_run_animation(instance: u32) -> f64;
	fn prop_set_humanoid_description_run_animation(instance: u32, value: f64);
	fn prop_humanoid_description_shirt(instance: u32) -> f64;
	fn prop_set_humanoid_description_shirt(instance: u32, value: f64);
	fn prop_humanoid_description_shoulders_accessory(instance: u32) -> String;
	fn prop_set_humanoid_description_shoulders_accessory(instance: u32, value: &str);
	fn prop_humanoid_description_swim_animation(instance: u32) -> f64;
	fn prop_set_humanoid_description_swim_animation(instance: u32, value: f64);
	fn prop_humanoid_description_torso(instance: u32) -> f64;
	fn prop_set_humanoid_description_torso(instance: u32, value: f64);
	fn prop_humanoid_description_torso_color(instance: u32) -> Color3;
	fn prop_set_humanoid_description_torso_color(instance: u32, value: Color3);
	fn prop_humanoid_description_waist_accessory(instance: u32) -> String;
	fn prop_set_humanoid_description_waist_accessory(instance: u32, value: &str);
	fn prop_humanoid_description_walk_animation(instance: u32) -> f64;
	fn prop_set_humanoid_description_walk_animation(instance: u32, value: f64);
	fn prop_humanoid_description_width_scale(instance: u32) -> f64;
	fn prop_set_humanoid_description_width_scale(instance: u32, value: f64);
	fn connect_humanoid_description_emotes_changed(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_humanoid_description_equipped_emotes_changed(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn prop_importer_base_settings_id(instance: u32) -> String;
	fn prop_importer_base_settings_import_name(instance: u32) -> String;
	fn prop_set_importer_base_settings_import_name(instance: u32, value: &str);
	fn prop_importer_base_settings_should_import(instance: u32) -> bool;
	fn prop_set_importer_base_settings_should_import(instance: u32, value: bool);
	fn prop_importer_group_settings_anchored(instance: u32) -> bool;
	fn prop_set_importer_group_settings_anchored(instance: u32, value: bool);
	fn prop_importer_group_settings_import_as_model_asset(instance: u32) -> bool;
	fn prop_set_importer_group_settings_import_as_model_asset(instance: u32, value: bool);
	fn prop_importer_group_settings_insert_in_workspace(instance: u32) -> bool;
	fn prop_set_importer_group_settings_insert_in_workspace(instance: u32, value: bool);
	fn prop_importer_material_settings_diffuse_file_path(instance: u32) -> String;
	fn prop_importer_material_settings_is_pbr(instance: u32) -> bool;
	fn prop_importer_material_settings_metalness_file_path(instance: u32) -> String;
	fn prop_importer_material_settings_normal_file_path(instance: u32) -> String;
	fn prop_importer_material_settings_roughness_file_path(instance: u32) -> String;
	fn prop_importer_mesh_settings_anchored(instance: u32) -> bool;
	fn prop_set_importer_mesh_settings_anchored(instance: u32, value: bool);
	fn prop_importer_mesh_settings_cage_manifold(instance: u32) -> bool;
	fn prop_importer_mesh_settings_cage_manifold_preview(instance: u32) -> bool;
	fn prop_set_importer_mesh_settings_cage_manifold_preview(instance: u32, value: bool);
	fn prop_importer_mesh_settings_cage_no_overlapping_vertices(instance: u32) -> bool;
	fn prop_importer_mesh_settings_cage_no_overlapping_vertices_preview(instance: u32) -> bool;
	fn prop_set_importer_mesh_settings_cage_no_overlapping_vertices_preview(instance: u32, value: bool);
	fn prop_importer_mesh_settings_cage_uv_matched(instance: u32) -> bool;
	fn prop_importer_mesh_settings_cage_uv_matched_preview(instance: u32) -> bool;
	fn prop_set_importer_mesh_settings_cage_uv_matched_preview(instance: u32, value: bool);
	fn prop_importer_mesh_settings_dimensions(instance: u32) -> Vector3;
	fn prop_importer_mesh_settings_double_sided(instance: u32) -> bool;
	fn prop_set_importer_mesh_settings_double_sided(instance: u32, value: bool);
	fn prop_importer_mesh_settings_ignore_vertex_colors(instance: u32) -> bool;
	fn prop_set_importer_mesh_settings_ignore_vertex_colors(instance: u32, value: bool);
	fn prop_importer_mesh_settings_polygon_count(instance: u32) -> f64;
	fn prop_importer_mesh_settings_use_imported_pivot(instance: u32) -> bool;
	fn prop_set_importer_mesh_settings_use_imported_pivot(instance: u32, value: bool);
	fn prop_importer_root_settings_anchored(instance: u32) -> bool;
	fn prop_set_importer_root_settings_anchored(instance: u32, value: bool);
	fn prop_importer_root_settings_file_dimensions(instance: u32) -> Vector3;
	fn prop_importer_root_settings_import_as_model_asset(instance: u32) -> bool;
	fn prop_set_importer_root_settings_import_as_model_asset(instance: u32, value: bool);
	fn prop_importer_root_settings_insert_in_workspace(instance: u32) -> bool;
	fn prop_set_importer_root_settings_insert_in_workspace(instance: u32, value: bool);
	fn prop_importer_root_settings_insert_with_scene_position(instance: u32) -> bool;
	fn prop_set_importer_root_settings_insert_with_scene_position(instance: u32, value: bool);
	fn prop_importer_root_settings_invert_negative_faces(instance: u32) -> bool;
	fn prop_set_importer_root_settings_invert_negative_faces(instance: u32, value: bool);
	fn prop_importer_root_settings_merge_meshes(instance: u32) -> bool;
	fn prop_set_importer_root_settings_merge_meshes(instance: u32, value: bool);
	fn prop_importer_root_settings_polygon_count(instance: u32) -> f64;
	fn prop_importer_root_settings_use_scene_origin_as_pivot(instance: u32) -> bool;
	fn prop_set_importer_root_settings_use_scene_origin_as_pivot(instance: u32, value: bool);
	fn dyn_input_object_is_modifier_key_down(instance: u32) -> bool;
	fn prop_input_object_delta(instance: u32) -> Vector3;
	fn prop_set_input_object_delta(instance: u32, value: Vector3);
	fn prop_input_object_position(instance: u32) -> Vector3;
	fn prop_set_input_object_position(instance: u32, value: Vector3);
	fn dyn_insert_service_approve_asset_id(instance: u32, p_assetId: f64);
	fn dyn_insert_service_approve_asset_version_id(instance: u32, p_assetVersionId: f64);
	fn dyn_insert_service_insert(instance: u32, p_instance: Option<Instance>);
	fn dyn_insert_service_get_base_categories(instance: u32);
	fn dyn_insert_service_get_base_sets(instance: u32);
	fn dyn_insert_service_get_collection(instance: u32, p_categoryId: f64);
	fn dyn_insert_service_get_free_decals(instance: u32, p_searchText: &str, p_pageNum: f64);
	fn dyn_insert_service_get_free_models(instance: u32, p_searchText: &str, p_pageNum: f64);
	fn dyn_insert_service_get_latest_asset_version_async(instance: u32, p_assetId: f64) -> f64;
	fn dyn_insert_service_get_user_categories(instance: u32, p_userId: f64);
	fn dyn_insert_service_get_user_sets(instance: u32, p_userId: f64);
	fn dyn_insert_service_load_asset(instance: u32, p_assetId: f64) -> Option<Instance>;
	fn dyn_insert_service_load_asset_version(instance: u32, p_assetVersionId: f64) -> Option<Instance>;
	fn prop_insert_service_allow_client_insert_models(instance: u32) -> bool;
	fn prop_set_insert_service_allow_client_insert_models(instance: u32, value: bool);
	fn prop_insert_service_allow_insert_free_models(instance: u32) -> bool;
	fn prop_set_insert_service_allow_insert_free_models(instance: u32, value: bool);
	fn prop_joint_instance_active(instance: u32) -> bool;
	fn prop_joint_instance_c_0(instance: u32) -> CFrame;
	fn prop_set_joint_instance_c_0(instance: u32, value: CFrame);
	fn prop_joint_instance_c_1(instance: u32) -> CFrame;
	fn prop_set_joint_instance_c_1(instance: u32, value: CFrame);
	fn prop_joint_instance_enabled(instance: u32) -> bool;
	fn prop_set_joint_instance_enabled(instance: u32, value: bool);
	fn prop_joint_instance_part_0(instance: u32) -> Option<BasePart>;
	fn prop_set_joint_instance_part_0(instance: u32, value: Option<BasePart>);
	fn prop_joint_instance_part_1(instance: u32) -> Option<BasePart>;
	fn prop_set_joint_instance_part_1(instance: u32, value: Option<BasePart>);
	fn prop_dynamic_rotate_base_angle(instance: u32) -> f64;
	fn prop_set_dynamic_rotate_base_angle(instance: u32, value: f64);
	fn prop_glue_f_0(instance: u32) -> Vector3;
	fn prop_set_glue_f_0(instance: u32, value: Vector3);
	fn prop_glue_f_1(instance: u32) -> Vector3;
	fn prop_set_glue_f_1(instance: u32, value: Vector3);
	fn prop_glue_f_2(instance: u32) -> Vector3;
	fn prop_set_glue_f_2(instance: u32, value: Vector3);
	fn prop_glue_f_3(instance: u32) -> Vector3;
	fn prop_set_glue_f_3(instance: u32, value: Vector3);
	fn dyn_motor_set_desired_angle(instance: u32, p_value: f64);
	fn prop_motor_current_angle(instance: u32) -> f64;
	fn prop_set_motor_current_angle(instance: u32, value: f64);
	fn prop_motor_desired_angle(instance: u32) -> f64;
	fn prop_set_motor_desired_angle(instance: u32, value: f64);
	fn prop_motor_max_velocity(instance: u32) -> f64;
	fn prop_set_motor_max_velocity(instance: u32, value: f64);
	fn prop_motor_6_d_child_name(instance: u32) -> String;
	fn prop_motor_6_d_parent_name(instance: u32) -> String;
	fn prop_motor_6_d_transform(instance: u32) -> CFrame;
	fn prop_set_motor_6_d_transform(instance: u32, value: CFrame);
	fn prop_velocity_motor_current_angle(instance: u32) -> f64;
	fn prop_set_velocity_motor_current_angle(instance: u32, value: f64);
	fn prop_velocity_motor_desired_angle(instance: u32) -> f64;
	fn prop_set_velocity_motor_desired_angle(instance: u32, value: f64);
	fn prop_velocity_motor_max_velocity(instance: u32) -> f64;
	fn prop_set_velocity_motor_max_velocity(instance: u32, value: f64);
	fn dyn_keyframe_add_marker(instance: u32, p_marker: Option<Instance>);
	fn dyn_keyframe_add_pose(instance: u32, p_pose: Option<Instance>);
	fn dyn_keyframe_get_markers(instance: u32) -> Objects;
	fn dyn_keyframe_get_poses(instance: u32) -> Objects;
	fn dyn_keyframe_remove_marker(instance: u32, p_marker: Option<Instance>);
	fn dyn_keyframe_remove_pose(instance: u32, p_pose: Option<Instance>);
	fn prop_keyframe_time(instance: u32) -> f64;
	fn prop_set_keyframe_time(instance: u32, value: f64);
	fn prop_keyframe_marker_value(instance: u32) -> String;
	fn prop_set_keyframe_marker_value(instance: u32, value: &str);
	fn dyn_keyframe_sequence_provider_register_active_keyframe_sequence(instance: u32, p_keyframeSequence: Option<Instance>) -> Content;
	fn dyn_keyframe_sequence_provider_register_keyframe_sequence(instance: u32, p_keyframeSequence: Option<Instance>) -> Content;
	fn dyn_keyframe_sequence_provider_get_animations(instance: u32, p_userId: f64) -> Option<Instance>;
	fn dyn_keyframe_sequence_provider_get_keyframe_sequence_async(instance: u32, p_assetId: Content) -> Option<Instance>;
	fn prop_light_brightness(instance: u32) -> f64;
	fn prop_set_light_brightness(instance: u32, value: f64);
	fn prop_light_color(instance: u32) -> Color3;
	fn prop_set_light_color(instance: u32, value: Color3);
	fn prop_light_enabled(instance: u32) -> bool;
	fn prop_set_light_enabled(instance: u32, value: bool);
	fn prop_light_shadows(instance: u32) -> bool;
	fn prop_set_light_shadows(instance: u32, value: bool);
	fn prop_point_light_range(instance: u32) -> f64;
	fn prop_set_point_light_range(instance: u32, value: f64);
	fn prop_spot_light_angle(instance: u32) -> f64;
	fn prop_set_spot_light_angle(instance: u32, value: f64);
	fn prop_spot_light_range(instance: u32) -> f64;
	fn prop_set_spot_light_range(instance: u32, value: f64);
	fn prop_surface_light_angle(instance: u32) -> f64;
	fn prop_set_surface_light_angle(instance: u32, value: f64);
	fn prop_surface_light_range(instance: u32) -> f64;
	fn prop_set_surface_light_range(instance: u32, value: f64);
	fn dyn_lighting_get_minutes_after_midnight(instance: u32) -> f64;
	fn dyn_lighting_get_moon_direction(instance: u32) -> Vector3;
	fn dyn_lighting_get_moon_phase(instance: u32) -> f64;
	fn dyn_lighting_get_sun_direction(instance: u32) -> Vector3;
	fn dyn_lighting_set_minutes_after_midnight(instance: u32, p_minutes: f64);
	fn prop_lighting_ambient(instance: u32) -> Color3;
	fn prop_set_lighting_ambient(instance: u32, value: Color3);
	fn prop_lighting_brightness(instance: u32) -> f64;
	fn prop_set_lighting_brightness(instance: u32, value: f64);
	fn prop_lighting_clock_time(instance: u32) -> f64;
	fn prop_set_lighting_clock_time(instance: u32, value: f64);
	fn prop_lighting_color_shift_bottom(instance: u32) -> Color3;
	fn prop_set_lighting_color_shift_bottom(instance: u32, value: Color3);
	fn prop_lighting_color_shift_top(instance: u32) -> Color3;
	fn prop_set_lighting_color_shift_top(instance: u32, value: Color3);
	fn prop_lighting_environment_diffuse_scale(instance: u32) -> f64;
	fn prop_set_lighting_environment_diffuse_scale(instance: u32, value: f64);
	fn prop_lighting_environment_specular_scale(instance: u32) -> f64;
	fn prop_set_lighting_environment_specular_scale(instance: u32, value: f64);
	fn prop_lighting_exposure_compensation(instance: u32) -> f64;
	fn prop_set_lighting_exposure_compensation(instance: u32, value: f64);
	fn prop_lighting_fog_color(instance: u32) -> Color3;
	fn prop_set_lighting_fog_color(instance: u32, value: Color3);
	fn prop_lighting_fog_end(instance: u32) -> f64;
	fn prop_set_lighting_fog_end(instance: u32, value: f64);
	fn prop_lighting_fog_start(instance: u32) -> f64;
	fn prop_set_lighting_fog_start(instance: u32, value: f64);
	fn prop_lighting_geographic_latitude(instance: u32) -> f64;
	fn prop_set_lighting_geographic_latitude(instance: u32, value: f64);
	fn prop_lighting_global_shadows(instance: u32) -> bool;
	fn prop_set_lighting_global_shadows(instance: u32, value: bool);
	fn prop_lighting_outdoor_ambient(instance: u32) -> Color3;
	fn prop_set_lighting_outdoor_ambient(instance: u32, value: Color3);
	fn prop_lighting_outlines(instance: u32) -> bool;
	fn prop_set_lighting_outlines(instance: u32, value: bool);
	fn prop_lighting_shadow_color(instance: u32) -> Color3;
	fn prop_set_lighting_shadow_color(instance: u32, value: Color3);
	fn prop_lighting_shadow_softness(instance: u32) -> f64;
	fn prop_set_lighting_shadow_softness(instance: u32, value: f64);
	fn prop_lighting_time_of_day(instance: u32) -> String;
	fn prop_set_lighting_time_of_day(instance: u32, value: &str);
	fn connect_lighting_lighting_changed(instance: u32, callback: Box<dyn Fn(bool)>) -> u32;
	fn dyn_localization_service_get_corescript_localizations(instance: u32) -> Objects;
	fn dyn_localization_service_get_table_entries(instance: u32, p_instance: Option<Instance>);
	fn dyn_localization_service_get_translator_for_player(instance: u32, p_player: Option<Instance>) -> Option<Instance>;
	fn dyn_localization_service_get_country_region_for_player_async(instance: u32, p_player: Option<Instance>) -> String;
	fn dyn_localization_service_get_translator_for_locale_async(instance: u32, p_locale: &str) -> Option<Instance>;
	fn dyn_localization_service_get_translator_for_player_async(instance: u32, p_player: Option<Instance>) -> Option<Instance>;
	fn prop_localization_service_roblox_locale_id(instance: u32) -> String;
	fn prop_localization_service_system_locale_id(instance: u32) -> String;
	fn dyn_localization_table_get_contents(instance: u32) -> String;
	fn dyn_localization_table_get_entries(instance: u32);
	fn dyn_localization_table_get_string(instance: u32, p_targetLocaleId: &str, p_key: &str) -> String;
	fn dyn_localization_table_get_translator(instance: u32, p_localeId: &str) -> Option<Instance>;
	fn dyn_localization_table_remove_entry(instance: u32, p_key: &str, p_source: &str, p_context: &str);
	fn dyn_localization_table_remove_entry_value(instance: u32, p_key: &str, p_source: &str, p_context: &str, p_localeId: &str);
	fn dyn_localization_table_remove_key(instance: u32, p_key: &str);
	fn dyn_localization_table_remove_target_locale(instance: u32, p_localeId: &str);
	fn dyn_localization_table_set_contents(instance: u32, p_contents: &str);
	fn dyn_localization_table_set_entries(instance: u32);
	fn dyn_localization_table_set_entry(instance: u32, p_key: &str, p_targetLocaleId: &str, p_text: &str);
	fn dyn_localization_table_set_entry_context(instance: u32, p_key: &str, p_source: &str, p_context: &str, p_newContext: &str);
	fn dyn_localization_table_set_entry_example(instance: u32, p_key: &str, p_source: &str, p_context: &str, p_example: &str);
	fn dyn_localization_table_set_entry_key(instance: u32, p_key: &str, p_source: &str, p_context: &str, p_newKey: &str);
	fn dyn_localization_table_set_entry_source(instance: u32, p_key: &str, p_source: &str, p_context: &str, p_newSource: &str);
	fn dyn_localization_table_set_entry_value(instance: u32, p_key: &str, p_source: &str, p_context: &str, p_localeId: &str, p_text: &str);
	fn prop_localization_table_development_language(instance: u32) -> String;
	fn prop_set_localization_table_development_language(instance: u32, value: &str);
	fn prop_localization_table_root(instance: u32) -> Option<Instance>;
	fn prop_set_localization_table_root(instance: u32, value: Option<Instance>);
	fn prop_localization_table_source_locale_id(instance: u32) -> String;
	fn prop_set_localization_table_source_locale_id(instance: u32, value: &str);
	fn prop_lod_data_entity_entity_lod_enabled(instance: u32) -> bool;
	fn prop_set_lod_data_entity_entity_lod_enabled(instance: u32, value: bool);
	fn dyn_log_service_get_log_history(instance: u32);
	fn connect_log_service_message_out(instance: u32, callback: Box<dyn Fn(String, ())>) -> u32;
	fn prop_lua_source_container_current_editor(instance: u32) -> Option<Instance>;
	fn prop_set_lua_source_container_current_editor(instance: u32, value: Option<Instance>);
	fn prop_base_script_disabled(instance: u32) -> bool;
	fn prop_set_base_script_disabled(instance: u32, value: bool);
	fn prop_base_script_linked_source(instance: u32) -> Content;
	fn prop_set_base_script_linked_source(instance: u32, value: Content);
	fn prop_module_script_linked_source(instance: u32) -> Content;
	fn prop_set_module_script_linked_source(instance: u32, value: Content);
	fn dyn_marker_curve_get_marker_at_index(instance: u32, p_index: f64);
	fn dyn_marker_curve_get_markers(instance: u32);
	fn dyn_marker_curve_insert_marker_at_time(instance: u32, p_time: f64, p_marker: &str);
	fn dyn_marker_curve_remove_marker_at_index(instance: u32, p_startingIndex: f64, p_count: f64) -> f64;
	fn prop_marker_curve_length(instance: u32) -> f64;
	fn dyn_marketplace_service_prompt_bundle_purchase(instance: u32, p_player: Option<Instance>, p_bundleId: f64);
	fn dyn_marketplace_service_prompt_game_pass_purchase(instance: u32, p_player: Option<Instance>, p_gamePassId: f64);
	fn dyn_marketplace_service_prompt_premium_purchase(instance: u32, p_player: Option<Instance>);
	fn dyn_marketplace_service_prompt_product_purchase(instance: u32, p_player: Option<Instance>, p_productId: f64, p_equipIfPurchased: bool);
	fn dyn_marketplace_service_prompt_purchase(instance: u32, p_player: Option<Instance>, p_assetId: f64, p_equipIfPurchased: bool);
	fn dyn_marketplace_service_prompt_subscription_cancellation(instance: u32, p_player: Option<Instance>, p_subscriptionId: f64);
	fn dyn_marketplace_service_prompt_subscription_purchase(instance: u32, p_player: Option<Instance>, p_subscriptionId: f64);
	fn dyn_marketplace_service_get_developer_products_async(instance: u32) -> Option<Instance>;
	fn dyn_marketplace_service_get_product_info(instance: u32, p_assetId: f64);
	fn dyn_marketplace_service_is_player_subscribed(instance: u32, p_player: Option<Instance>, p_subscriptionId: f64) -> bool;
	fn dyn_marketplace_service_player_owns_asset(instance: u32, p_player: Option<Instance>, p_assetId: f64) -> bool;
	fn dyn_marketplace_service_player_owns_bundle(instance: u32, p_player: Option<Player>, p_bundleId: f64) -> bool;
	fn dyn_marketplace_service_user_owns_game_pass_async(instance: u32, p_userId: f64, p_gamePassId: f64) -> bool;
	fn connect_marketplace_service_prompt_bundle_purchase_finished(instance: u32, callback: Box<dyn Fn(Option<Instance>, f64, bool)>) -> u32;
	fn connect_marketplace_service_prompt_game_pass_purchase_finished(instance: u32, callback: Box<dyn Fn(Option<Instance>, f64, bool)>) -> u32;
	fn connect_marketplace_service_prompt_premium_purchase_finished(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_marketplace_service_prompt_product_purchase_finished(instance: u32, callback: Box<dyn Fn(f64, f64, bool)>) -> u32;
	fn connect_marketplace_service_prompt_purchase_finished(instance: u32, callback: Box<dyn Fn(Option<Instance>, f64, bool)>) -> u32;
	fn connect_marketplace_service_prompt_subscription_cancellation_finished(instance: u32, callback: Box<dyn Fn(Option<Instance>, f64, bool)>) -> u32;
	fn connect_marketplace_service_prompt_subscription_purchase_finished(instance: u32, callback: Box<dyn Fn(Option<Instance>, f64, bool)>) -> u32;
	fn dyn_material_service_get_base_material_override(instance: u32) -> String;
	fn dyn_material_service_get_material_variant(instance: u32, p_name: &str) -> Option<MaterialVariant>;
	fn dyn_material_service_set_base_material_override(instance: u32, p_name: &str);
	fn prop_material_variant_studs_per_tile(instance: u32) -> f64;
	fn prop_set_material_variant_studs_per_tile(instance: u32, value: f64);
	fn dyn_memory_store_queue_add_async(instance: u32, p_expiration: f64, p_priority: f64);
	fn dyn_memory_store_queue_read_async(instance: u32, p_count: f64, p_allOrNothing: bool, p_waitTimeout: f64);
	fn dyn_memory_store_queue_remove_async(instance: u32, p_id: &str);
	fn dyn_memory_store_service_get_queue(instance: u32, p_name: &str, p_invisibilityTimeout: f64) -> Option<MemoryStoreQueue>;
	fn dyn_memory_store_service_get_sorted_map(instance: u32, p_name: &str) -> Option<MemoryStoreSortedMap>;
	fn dyn_memory_store_sorted_map_get_async(instance: u32, p_key: &str);
	fn dyn_memory_store_sorted_map_get_range_async(instance: u32, p_count: f64, p_exclusiveLowerBound: &str, p_exclusiveUpperBound: &str);
	fn dyn_memory_store_sorted_map_remove_async(instance: u32, p_key: &str);
	fn dyn_memory_store_sorted_map_set_async(instance: u32, p_key: &str, p_expiration: f64) -> bool;
	fn dyn_memory_store_sorted_map_update_async(instance: u32, p_key: &str, p_transformFunction: Function, p_expiration: f64);
	fn dyn_messaging_service_publish_async(instance: u32, p_topic: &str);
	fn dyn_messaging_service_subscribe_async(instance: u32, p_topic: &str, p_callback: Function) -> RbxScriptConnection;
	fn prop_mouse_hit(instance: u32) -> CFrame;
	fn prop_mouse_icon(instance: u32) -> Content;
	fn prop_set_mouse_icon(instance: u32, value: Content);
	fn prop_mouse_origin(instance: u32) -> CFrame;
	fn prop_mouse_target(instance: u32) -> Option<BasePart>;
	fn prop_mouse_target_filter(instance: u32) -> Option<Instance>;
	fn prop_set_mouse_target_filter(instance: u32, value: Option<Instance>);
	fn prop_mouse_unit_ray(instance: u32) -> Ray;
	fn prop_mouse_view_size_x(instance: u32) -> f64;
	fn prop_mouse_view_size_y(instance: u32) -> f64;
	fn prop_mouse_x(instance: u32) -> f64;
	fn prop_mouse_y(instance: u32) -> f64;
	fn connect_mouse_button_1_down(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_mouse_button_1_up(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_mouse_button_2_down(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_mouse_button_2_up(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_mouse_idle(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_mouse_key_down(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_mouse_key_up(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_mouse_move(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_mouse_wheel_backward(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_mouse_wheel_forward(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_network_marker_received(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn prop_no_collision_constraint_enabled(instance: u32) -> bool;
	fn prop_set_no_collision_constraint_enabled(instance: u32, value: bool);
	fn prop_no_collision_constraint_part_0(instance: u32) -> Option<BasePart>;
	fn prop_set_no_collision_constraint_part_0(instance: u32, value: Option<BasePart>);
	fn prop_no_collision_constraint_part_1(instance: u32) -> Option<BasePart>;
	fn prop_set_no_collision_constraint_part_1(instance: u32, value: Option<BasePart>);
	fn dyn_pv_instance_get_pivot(instance: u32) -> CFrame;
	fn dyn_pv_instance_pivot_to(instance: u32, p_targetCFrame: CFrame);
	fn dyn_base_part_apply_angular_impulse(instance: u32, p_impulse: Vector3);
	fn dyn_base_part_apply_impulse(instance: u32, p_impulse: Vector3);
	fn dyn_base_part_apply_impulse_at_position(instance: u32, p_impulse: Vector3, p_position: Vector3);
	fn dyn_base_part_break_joints(instance: u32);
	fn dyn_base_part_can_collide_with(instance: u32, p_part: Option<BasePart>) -> bool;
	fn dyn_base_part_can_set_network_ownership(instance: u32);
	fn dyn_base_part_get_connected_parts(instance: u32, p_recursive: bool) -> Objects;
	fn dyn_base_part_get_joints(instance: u32) -> Objects;
	fn dyn_base_part_get_mass(instance: u32) -> f64;
	fn dyn_base_part_get_network_owner(instance: u32) -> Option<Instance>;
	fn dyn_base_part_get_network_ownership_auto(instance: u32) -> bool;
	fn dyn_base_part_get_render_c_frame(instance: u32) -> CFrame;
	fn dyn_base_part_get_root_part(instance: u32) -> Option<Instance>;
	fn dyn_base_part_get_touching_parts(instance: u32) -> Objects;
	fn dyn_base_part_get_velocity_at_position(instance: u32, p_position: Vector3) -> Vector3;
	fn dyn_base_part_is_grounded(instance: u32) -> bool;
	fn dyn_base_part_make_joints(instance: u32);
	fn dyn_base_part_resize(instance: u32, p_deltaAmount: f64) -> bool;
	fn dyn_base_part_set_network_owner(instance: u32, p_playerInstance: Option<Player>);
	fn dyn_base_part_set_network_ownership_auto(instance: u32);
	fn dyn_base_part_subtract_async(instance: u32, p_parts: Objects) -> Option<Instance>;
	fn dyn_base_part_union_async(instance: u32, p_parts: Objects) -> Option<Instance>;
	fn prop_base_part_anchored(instance: u32) -> bool;
	fn prop_set_base_part_anchored(instance: u32, value: bool);
	fn prop_base_part_assembly_angular_velocity(instance: u32) -> Vector3;
	fn prop_set_base_part_assembly_angular_velocity(instance: u32, value: Vector3);
	fn prop_base_part_assembly_center_of_mass(instance: u32) -> Vector3;
	fn prop_base_part_assembly_linear_velocity(instance: u32) -> Vector3;
	fn prop_set_base_part_assembly_linear_velocity(instance: u32, value: Vector3);
	fn prop_base_part_assembly_mass(instance: u32) -> f64;
	fn prop_base_part_assembly_root_part(instance: u32) -> Option<BasePart>;
	fn prop_base_part_back_param_a(instance: u32) -> f64;
	fn prop_set_base_part_back_param_a(instance: u32, value: f64);
	fn prop_base_part_back_param_b(instance: u32) -> f64;
	fn prop_set_base_part_back_param_b(instance: u32, value: f64);
	fn prop_base_part_bottom_param_a(instance: u32) -> f64;
	fn prop_set_base_part_bottom_param_a(instance: u32, value: f64);
	fn prop_base_part_bottom_param_b(instance: u32) -> f64;
	fn prop_set_base_part_bottom_param_b(instance: u32, value: f64);
	fn prop_base_part_brick_color(instance: u32) -> BrickColor;
	fn prop_set_base_part_brick_color(instance: u32, value: BrickColor);
	fn prop_base_part_c_frame(instance: u32) -> CFrame;
	fn prop_set_base_part_c_frame(instance: u32, value: CFrame);
	fn prop_base_part_can_collide(instance: u32) -> bool;
	fn prop_set_base_part_can_collide(instance: u32, value: bool);
	fn prop_base_part_can_query(instance: u32) -> bool;
	fn prop_set_base_part_can_query(instance: u32, value: bool);
	fn prop_base_part_can_touch(instance: u32) -> bool;
	fn prop_set_base_part_can_touch(instance: u32, value: bool);
	fn prop_base_part_cast_shadow(instance: u32) -> bool;
	fn prop_set_base_part_cast_shadow(instance: u32, value: bool);
	fn prop_base_part_center_of_mass(instance: u32) -> Vector3;
	fn prop_base_part_collision_group_id(instance: u32) -> f64;
	fn prop_set_base_part_collision_group_id(instance: u32, value: f64);
	fn prop_base_part_color(instance: u32) -> Color3;
	fn prop_set_base_part_color(instance: u32, value: Color3);
	fn prop_base_part_custom_physical_properties(instance: u32) -> PhysicalProperties;
	fn prop_set_base_part_custom_physical_properties(instance: u32, value: PhysicalProperties);
	fn prop_base_part_elasticity(instance: u32) -> f64;
	fn prop_set_base_part_elasticity(instance: u32, value: f64);
	fn prop_base_part_friction(instance: u32) -> f64;
	fn prop_set_base_part_friction(instance: u32, value: f64);
	fn prop_base_part_front_param_a(instance: u32) -> f64;
	fn prop_set_base_part_front_param_a(instance: u32, value: f64);
	fn prop_base_part_front_param_b(instance: u32) -> f64;
	fn prop_set_base_part_front_param_b(instance: u32, value: f64);
	fn prop_base_part_left_param_a(instance: u32) -> f64;
	fn prop_set_base_part_left_param_a(instance: u32, value: f64);
	fn prop_base_part_left_param_b(instance: u32) -> f64;
	fn prop_set_base_part_left_param_b(instance: u32, value: f64);
	fn prop_base_part_local_transparency_modifier(instance: u32) -> f64;
	fn prop_set_base_part_local_transparency_modifier(instance: u32, value: f64);
	fn prop_base_part_locked(instance: u32) -> bool;
	fn prop_set_base_part_locked(instance: u32, value: bool);
	fn prop_base_part_mass(instance: u32) -> f64;
	fn prop_base_part_massless(instance: u32) -> bool;
	fn prop_set_base_part_massless(instance: u32, value: bool);
	fn prop_base_part_material_variant(instance: u32) -> String;
	fn prop_set_base_part_material_variant(instance: u32, value: &str);
	fn prop_base_part_orientation(instance: u32) -> Vector3;
	fn prop_set_base_part_orientation(instance: u32, value: Vector3);
	fn prop_base_part_pivot_offset(instance: u32) -> CFrame;
	fn prop_set_base_part_pivot_offset(instance: u32, value: CFrame);
	fn prop_base_part_position(instance: u32) -> Vector3;
	fn prop_set_base_part_position(instance: u32, value: Vector3);
	fn prop_base_part_receive_age(instance: u32) -> f64;
	fn prop_base_part_reflectance(instance: u32) -> f64;
	fn prop_set_base_part_reflectance(instance: u32, value: f64);
	fn prop_base_part_resize_increment(instance: u32) -> f64;
	fn prop_base_part_resizeable_faces(instance: u32) -> Faces;
	fn prop_base_part_right_param_a(instance: u32) -> f64;
	fn prop_set_base_part_right_param_a(instance: u32, value: f64);
	fn prop_base_part_right_param_b(instance: u32) -> f64;
	fn prop_set_base_part_right_param_b(instance: u32, value: f64);
	fn prop_base_part_root_priority(instance: u32) -> f64;
	fn prop_set_base_part_root_priority(instance: u32, value: f64);
	fn prop_base_part_rot_velocity(instance: u32) -> Vector3;
	fn prop_set_base_part_rot_velocity(instance: u32, value: Vector3);
	fn prop_base_part_rotation(instance: u32) -> Vector3;
	fn prop_set_base_part_rotation(instance: u32, value: Vector3);
	fn prop_base_part_size(instance: u32) -> Vector3;
	fn prop_set_base_part_size(instance: u32, value: Vector3);
	fn prop_base_part_specific_gravity(instance: u32) -> f64;
	fn prop_base_part_top_param_a(instance: u32) -> f64;
	fn prop_set_base_part_top_param_a(instance: u32, value: f64);
	fn prop_base_part_top_param_b(instance: u32) -> f64;
	fn prop_set_base_part_top_param_b(instance: u32, value: f64);
	fn prop_base_part_transparency(instance: u32) -> f64;
	fn prop_set_base_part_transparency(instance: u32, value: f64);
	fn prop_base_part_velocity(instance: u32) -> Vector3;
	fn prop_set_base_part_velocity(instance: u32, value: Vector3);
	fn connect_base_part_local_simulation_touched(instance: u32, callback: Box<dyn Fn(Option<BasePart>)>) -> u32;
	fn connect_base_part_outfit_changed(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_base_part_stopped_touching(instance: u32, callback: Box<dyn Fn(Option<BasePart>)>) -> u32;
	fn connect_base_part_touch_ended(instance: u32, callback: Box<dyn Fn(Option<BasePart>)>) -> u32;
	fn connect_base_part_touched(instance: u32, callback: Box<dyn Fn(Option<BasePart>)>) -> u32;
	fn dyn_seat_sit(instance: u32, p_humanoid: Option<Instance>);
	fn prop_seat_disabled(instance: u32) -> bool;
	fn prop_set_seat_disabled(instance: u32, value: bool);
	fn prop_seat_occupant(instance: u32) -> Option<Humanoid>;
	fn prop_skateboard_platform_controller(instance: u32) -> Option<SkateboardController>;
	fn prop_skateboard_platform_controlling_humanoid(instance: u32) -> Option<Humanoid>;
	fn prop_skateboard_platform_steer(instance: u32) -> f64;
	fn prop_set_skateboard_platform_steer(instance: u32, value: f64);
	fn prop_skateboard_platform_sticky_wheels(instance: u32) -> bool;
	fn prop_set_skateboard_platform_sticky_wheels(instance: u32, value: bool);
	fn prop_skateboard_platform_throttle(instance: u32) -> f64;
	fn prop_set_skateboard_platform_throttle(instance: u32, value: f64);
	fn connect_skateboard_platform_equipped(instance: u32, callback: Box<dyn Fn(Option<Instance>, Option<Instance>)>) -> u32;
	fn connect_skateboard_platform_move_state_changed(instance: u32, callback: Box<dyn Fn((), ())>) -> u32;
	fn connect_skateboard_platform_unequipped(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn prop_spawn_location_allow_team_change_on_touch(instance: u32) -> bool;
	fn prop_set_spawn_location_allow_team_change_on_touch(instance: u32, value: bool);
	fn prop_spawn_location_duration(instance: u32) -> f64;
	fn prop_set_spawn_location_duration(instance: u32, value: f64);
	fn prop_spawn_location_enabled(instance: u32) -> bool;
	fn prop_set_spawn_location_enabled(instance: u32, value: bool);
	fn prop_spawn_location_neutral(instance: u32) -> bool;
	fn prop_set_spawn_location_neutral(instance: u32, value: bool);
	fn prop_spawn_location_team_color(instance: u32) -> BrickColor;
	fn prop_set_spawn_location_team_color(instance: u32, value: BrickColor);
	fn dyn_terrain_autowedge_cell(instance: u32, p_x: f64, p_y: f64, p_z: f64) -> bool;
	fn dyn_terrain_autowedge_cells(instance: u32, p_region: Region3int16);
	fn dyn_terrain_cell_center_to_world(instance: u32, p_x: f64, p_y: f64, p_z: f64) -> Vector3;
	fn dyn_terrain_cell_corner_to_world(instance: u32, p_x: f64, p_y: f64, p_z: f64) -> Vector3;
	fn dyn_terrain_clear(instance: u32);
	fn dyn_terrain_copy_region(instance: u32, p_region: Region3int16) -> Option<TerrainRegion>;
	fn dyn_terrain_count_cells(instance: u32) -> f64;
	fn dyn_terrain_fill_ball(instance: u32, p_center: Vector3, p_radius: f64);
	fn dyn_terrain_fill_block(instance: u32, p_cframe: CFrame, p_size: Vector3);
	fn dyn_terrain_fill_cylinder(instance: u32, p_cframe: CFrame, p_height: f64, p_radius: f64);
	fn dyn_terrain_fill_region(instance: u32, p_region: Region3, p_resolution: f64);
	fn dyn_terrain_fill_wedge(instance: u32, p_cframe: CFrame, p_size: Vector3);
	fn dyn_terrain_get_cell(instance: u32, p_x: f64, p_y: f64, p_z: f64);
	fn dyn_terrain_get_material_color(instance: u32) -> Color3;
	fn dyn_terrain_get_water_cell(instance: u32, p_x: f64, p_y: f64, p_z: f64);
	fn dyn_terrain_paste_region(instance: u32, p_region: Option<TerrainRegion>, p_corner: Vector3int16, p_pasteEmptyCells: bool);
	fn dyn_terrain_read_voxels(instance: u32, p_region: Region3, p_resolution: f64);
	fn dyn_terrain_replace_material(instance: u32, p_region: Region3, p_resolution: f64);
	fn dyn_terrain_set_cell(instance: u32, p_x: f64, p_y: f64, p_z: f64);
	fn dyn_terrain_set_cells(instance: u32, p_region: Region3int16);
	fn dyn_terrain_set_material_color(instance: u32, p_value: Color3);
	fn dyn_terrain_set_water_cell(instance: u32, p_x: f64, p_y: f64, p_z: f64);
	fn dyn_terrain_world_to_cell(instance: u32, p_position: Vector3) -> Vector3;
	fn dyn_terrain_world_to_cell_prefer_empty(instance: u32, p_position: Vector3) -> Vector3;
	fn dyn_terrain_world_to_cell_prefer_solid(instance: u32, p_position: Vector3) -> Vector3;
	fn dyn_terrain_write_voxels(instance: u32, p_region: Region3, p_resolution: f64);
	fn prop_terrain_decoration(instance: u32) -> bool;
	fn prop_set_terrain_decoration(instance: u32, value: bool);
	fn prop_terrain_is_smooth(instance: u32) -> bool;
	fn prop_terrain_material_colors(instance: u32) -> BinaryString;
	fn prop_set_terrain_material_colors(instance: u32, value: BinaryString);
	fn prop_terrain_max_extents(instance: u32) -> Region3int16;
	fn prop_terrain_water_color(instance: u32) -> Color3;
	fn prop_set_terrain_water_color(instance: u32, value: Color3);
	fn prop_terrain_water_reflectance(instance: u32) -> f64;
	fn prop_set_terrain_water_reflectance(instance: u32, value: f64);
	fn prop_terrain_water_transparency(instance: u32) -> f64;
	fn prop_set_terrain_water_transparency(instance: u32, value: f64);
	fn prop_terrain_water_wave_size(instance: u32) -> f64;
	fn prop_set_terrain_water_wave_size(instance: u32, value: f64);
	fn prop_terrain_water_wave_speed(instance: u32) -> f64;
	fn prop_set_terrain_water_wave_speed(instance: u32, value: f64);
	fn dyn_mesh_part_apply_mesh(instance: u32, p_meshPart: Option<Instance>);
	fn prop_mesh_part_double_sided(instance: u32) -> bool;
	fn prop_mesh_part_has_joint_offset(instance: u32) -> bool;
	fn prop_mesh_part_has_skinned_mesh(instance: u32) -> bool;
	fn prop_mesh_part_joint_offset(instance: u32) -> Vector3;
	fn prop_mesh_part_mesh_id(instance: u32) -> Content;
	fn prop_mesh_part_mesh_size(instance: u32) -> Vector3;
	fn prop_mesh_part_texture_id(instance: u32) -> Content;
	fn prop_set_mesh_part_texture_id(instance: u32, value: Content);
	fn prop_part_operation_smoothing_angle(instance: u32) -> f64;
	fn prop_part_operation_triangle_count(instance: u32) -> f64;
	fn prop_part_operation_use_part_color(instance: u32) -> bool;
	fn prop_set_part_operation_use_part_color(instance: u32, value: bool);
	fn dyn_vehicle_seat_sit(instance: u32, p_humanoid: Option<Instance>);
	fn prop_vehicle_seat_are_hinges_detected(instance: u32) -> f64;
	fn prop_vehicle_seat_disabled(instance: u32) -> bool;
	fn prop_set_vehicle_seat_disabled(instance: u32, value: bool);
	fn prop_vehicle_seat_heads_up_display(instance: u32) -> bool;
	fn prop_set_vehicle_seat_heads_up_display(instance: u32, value: bool);
	fn prop_vehicle_seat_max_speed(instance: u32) -> f64;
	fn prop_set_vehicle_seat_max_speed(instance: u32, value: f64);
	fn prop_vehicle_seat_occupant(instance: u32) -> Option<Humanoid>;
	fn prop_vehicle_seat_steer(instance: u32) -> f64;
	fn prop_set_vehicle_seat_steer(instance: u32, value: f64);
	fn prop_vehicle_seat_steer_float(instance: u32) -> f64;
	fn prop_set_vehicle_seat_steer_float(instance: u32, value: f64);
	fn prop_vehicle_seat_throttle(instance: u32) -> f64;
	fn prop_set_vehicle_seat_throttle(instance: u32, value: f64);
	fn prop_vehicle_seat_throttle_float(instance: u32) -> f64;
	fn prop_set_vehicle_seat_throttle_float(instance: u32, value: f64);
	fn prop_vehicle_seat_torque(instance: u32) -> f64;
	fn prop_set_vehicle_seat_torque(instance: u32, value: f64);
	fn prop_vehicle_seat_turn_speed(instance: u32) -> f64;
	fn prop_set_vehicle_seat_turn_speed(instance: u32, value: f64);
	fn dyn_model_break_joints(instance: u32);
	fn dyn_model_get_bounding_box(instance: u32);
	fn dyn_model_get_extents_size(instance: u32) -> Vector3;
	fn dyn_model_get_model_c_frame(instance: u32) -> CFrame;
	fn dyn_model_get_model_size(instance: u32) -> Vector3;
	fn dyn_model_get_primary_part_c_frame(instance: u32) -> CFrame;
	fn dyn_model_make_joints(instance: u32);
	fn dyn_model_move_to(instance: u32, p_position: Vector3);
	fn dyn_model_reset_orientation_to_identity(instance: u32);
	fn dyn_model_set_identity_orientation(instance: u32);
	fn dyn_model_set_primary_part_c_frame(instance: u32, p_cframe: CFrame);
	fn dyn_model_translate_by(instance: u32, p_delta: Vector3);
	fn prop_model_primary_part(instance: u32) -> Option<BasePart>;
	fn prop_set_model_primary_part(instance: u32, value: Option<BasePart>);
	fn prop_model_world_pivot(instance: u32) -> CFrame;
	fn prop_set_model_world_pivot(instance: u32, value: CFrame);
	fn dyn_world_root_are_parts_touching_others(instance: u32, p_partList: Objects, p_overlapIgnored: f64) -> bool;
	fn dyn_world_root_bulk_move_to(instance: u32, p_partList: Objects);
	fn dyn_world_root_find_part_on_ray(instance: u32, p_ray: Ray, p_ignoreDescendantsInstance: Option<Instance>, p_terrainCellsAreCubes: bool, p_ignoreWater: bool);
	fn dyn_world_root_find_part_on_ray_with_ignore_list(instance: u32, p_ray: Ray, p_ignoreDescendantsTable: Objects, p_terrainCellsAreCubes: bool, p_ignoreWater: bool);
	fn dyn_world_root_find_part_on_ray_with_whitelist(instance: u32, p_ray: Ray, p_whitelistDescendantsTable: Objects, p_ignoreWater: bool);
	fn dyn_world_root_find_parts_in_region_3(instance: u32, p_region: Region3, p_ignoreDescendantsInstance: Option<Instance>, p_maxParts: f64) -> Objects;
	fn dyn_world_root_find_parts_in_region_3_with_ignore_list(instance: u32, p_region: Region3, p_ignoreDescendantsTable: Objects, p_maxParts: f64) -> Objects;
	fn dyn_world_root_find_parts_in_region_3_with_white_list(instance: u32, p_region: Region3, p_whitelistDescendantsTable: Objects, p_maxParts: f64) -> Objects;
	fn dyn_world_root_get_part_bounds_in_box(instance: u32, p_cframe: CFrame, p_size: Vector3, p_overlapParams: OverlapParams) -> Objects;
	fn dyn_world_root_get_part_bounds_in_radius(instance: u32, p_position: Vector3, p_radius: f64, p_overlapParams: OverlapParams) -> Objects;
	fn dyn_world_root_get_parts_in_part(instance: u32, p_part: Option<BasePart>, p_overlapParams: OverlapParams) -> Objects;
	fn dyn_world_root_is_region_3_empty(instance: u32, p_region: Region3, p_ignoreDescendentsInstance: Option<Instance>) -> bool;
	fn dyn_world_root_is_region_3_empty_with_ignore_list(instance: u32, p_region: Region3, p_ignoreDescendentsTable: Objects) -> bool;
	fn dyn_world_root_raycast(instance: u32, p_origin: Vector3, p_direction: Vector3, p_raycastParams: RaycastParams) -> RaycastResult;
	fn dyn_workspace_get_num_awake_parts(instance: u32) -> f64;
	fn dyn_workspace_get_physics_throttling(instance: u32) -> f64;
	fn dyn_workspace_get_real_physics_fps(instance: u32) -> f64;
	fn dyn_workspace_get_server_time_now(instance: u32) -> f64;
	fn dyn_workspace_join_to_outsiders(instance: u32, p_objects: Objects);
	fn dyn_workspace_pgs_is_enabled(instance: u32) -> bool;
	fn dyn_workspace_unjoin_from_outsiders(instance: u32, p_objects: Objects);
	fn prop_workspace_allow_third_party_sales(instance: u32) -> bool;
	fn prop_set_workspace_allow_third_party_sales(instance: u32, value: bool);
	fn prop_workspace_current_camera(instance: u32) -> Option<Camera>;
	fn prop_set_workspace_current_camera(instance: u32, value: Option<Camera>);
	fn prop_workspace_distributed_game_time(instance: u32) -> f64;
	fn prop_set_workspace_distributed_game_time(instance: u32, value: f64);
	fn prop_workspace_fallen_parts_destroy_height(instance: u32) -> f64;
	fn prop_workspace_filtering_enabled(instance: u32) -> bool;
	fn prop_workspace_global_wind(instance: u32) -> Vector3;
	fn prop_set_workspace_global_wind(instance: u32, value: Vector3);
	fn prop_workspace_gravity(instance: u32) -> f64;
	fn prop_set_workspace_gravity(instance: u32, value: f64);
	fn prop_workspace_streaming_enabled(instance: u32) -> bool;
	fn prop_workspace_streaming_min_radius(instance: u32) -> f64;
	fn prop_set_workspace_streaming_min_radius(instance: u32, value: f64);
	fn prop_workspace_streaming_target_radius(instance: u32) -> f64;
	fn prop_set_workspace_streaming_target_radius(instance: u32, value: f64);
	fn prop_workspace_terrain(instance: u32) -> Option<Terrain>;
	fn prop_workspace_touches_use_collision_groups(instance: u32) -> bool;
	fn prop_set_workspace_touches_use_collision_groups(instance: u32, value: bool);
	fn prop_package_link_auto_update(instance: u32) -> bool;
	fn prop_set_package_link_auto_update(instance: u32, value: bool);
	fn prop_package_link_creator(instance: u32) -> String;
	fn prop_package_link_package_asset_name(instance: u32) -> String;
	fn prop_package_link_package_id(instance: u32) -> Content;
	fn prop_package_link_version_number(instance: u32) -> f64;
	fn dyn_pages_get_current_page(instance: u32);
	fn dyn_pages_advance_to_next_page_async(instance: u32);
	fn prop_pages_is_finished(instance: u32) -> bool;
	fn dyn_particle_emitter_clear(instance: u32);
	fn dyn_particle_emitter_emit(instance: u32, p_particleCount: f64);
	fn prop_particle_emitter_acceleration(instance: u32) -> Vector3;
	fn prop_set_particle_emitter_acceleration(instance: u32, value: Vector3);
	fn prop_particle_emitter_brightness(instance: u32) -> f64;
	fn prop_set_particle_emitter_brightness(instance: u32, value: f64);
	fn prop_particle_emitter_color(instance: u32) -> ColorSequence;
	fn prop_set_particle_emitter_color(instance: u32, value: ColorSequence);
	fn prop_particle_emitter_drag(instance: u32) -> f64;
	fn prop_set_particle_emitter_drag(instance: u32, value: f64);
	fn prop_particle_emitter_enabled(instance: u32) -> bool;
	fn prop_set_particle_emitter_enabled(instance: u32, value: bool);
	fn prop_particle_emitter_flipbook_framerate(instance: u32) -> NumberRange;
	fn prop_set_particle_emitter_flipbook_framerate(instance: u32, value: NumberRange);
	fn prop_particle_emitter_flipbook_incompatible(instance: u32) -> String;
	fn prop_set_particle_emitter_flipbook_incompatible(instance: u32, value: &str);
	fn prop_particle_emitter_flipbook_start_random(instance: u32) -> bool;
	fn prop_set_particle_emitter_flipbook_start_random(instance: u32, value: bool);
	fn prop_particle_emitter_lifetime(instance: u32) -> NumberRange;
	fn prop_set_particle_emitter_lifetime(instance: u32, value: NumberRange);
	fn prop_particle_emitter_light_emission(instance: u32) -> f64;
	fn prop_set_particle_emitter_light_emission(instance: u32, value: f64);
	fn prop_particle_emitter_light_influence(instance: u32) -> f64;
	fn prop_set_particle_emitter_light_influence(instance: u32, value: f64);
	fn prop_particle_emitter_locked_to_part(instance: u32) -> bool;
	fn prop_set_particle_emitter_locked_to_part(instance: u32, value: bool);
	fn prop_particle_emitter_rate(instance: u32) -> f64;
	fn prop_set_particle_emitter_rate(instance: u32, value: f64);
	fn prop_particle_emitter_rot_speed(instance: u32) -> NumberRange;
	fn prop_set_particle_emitter_rot_speed(instance: u32, value: NumberRange);
	fn prop_particle_emitter_rotation(instance: u32) -> NumberRange;
	fn prop_set_particle_emitter_rotation(instance: u32, value: NumberRange);
	fn prop_particle_emitter_shape_partial(instance: u32) -> f64;
	fn prop_set_particle_emitter_shape_partial(instance: u32, value: f64);
	fn prop_particle_emitter_size(instance: u32) -> NumberSequence;
	fn prop_set_particle_emitter_size(instance: u32, value: NumberSequence);
	fn prop_particle_emitter_speed(instance: u32) -> NumberRange;
	fn prop_set_particle_emitter_speed(instance: u32, value: NumberRange);
	fn prop_particle_emitter_spread_angle(instance: u32) -> Vector2;
	fn prop_set_particle_emitter_spread_angle(instance: u32, value: Vector2);
	fn prop_particle_emitter_squash(instance: u32) -> NumberSequence;
	fn prop_set_particle_emitter_squash(instance: u32, value: NumberSequence);
	fn prop_particle_emitter_texture(instance: u32) -> Content;
	fn prop_set_particle_emitter_texture(instance: u32, value: Content);
	fn prop_particle_emitter_time_scale(instance: u32) -> f64;
	fn prop_set_particle_emitter_time_scale(instance: u32, value: f64);
	fn prop_particle_emitter_transparency(instance: u32) -> NumberSequence;
	fn prop_set_particle_emitter_transparency(instance: u32, value: NumberSequence);
	fn prop_particle_emitter_velocity_inheritance(instance: u32) -> f64;
	fn prop_set_particle_emitter_velocity_inheritance(instance: u32, value: f64);
	fn prop_particle_emitter_velocity_spread(instance: u32) -> f64;
	fn prop_set_particle_emitter_velocity_spread(instance: u32, value: f64);
	fn prop_particle_emitter_z_offset(instance: u32) -> f64;
	fn prop_set_particle_emitter_z_offset(instance: u32, value: f64);
	fn dyn_path_get_point_coordinates(instance: u32);
	fn dyn_path_get_waypoints(instance: u32);
	fn dyn_path_check_occlusion_async(instance: u32, p_start: f64) -> f64;
	fn dyn_path_compute_async(instance: u32, p_start: Vector3, p_finish: Vector3);
	fn connect_path_blocked(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn connect_path_unblocked(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn prop_pathfinding_link_attachment_0(instance: u32) -> Option<Attachment>;
	fn prop_set_pathfinding_link_attachment_0(instance: u32, value: Option<Attachment>);
	fn prop_pathfinding_link_attachment_1(instance: u32) -> Option<Attachment>;
	fn prop_set_pathfinding_link_attachment_1(instance: u32, value: Option<Attachment>);
	fn prop_pathfinding_link_is_bidirectional(instance: u32) -> bool;
	fn prop_set_pathfinding_link_is_bidirectional(instance: u32, value: bool);
	fn prop_pathfinding_link_label(instance: u32) -> String;
	fn prop_set_pathfinding_link_label(instance: u32, value: &str);
	fn prop_pathfinding_modifier_label(instance: u32) -> String;
	fn prop_set_pathfinding_modifier_label(instance: u32, value: &str);
	fn prop_pathfinding_modifier_pass_through(instance: u32) -> bool;
	fn prop_set_pathfinding_modifier_pass_through(instance: u32, value: bool);
	fn dyn_pathfinding_service_create_path(instance: u32) -> Option<Instance>;
	fn dyn_pathfinding_service_compute_raw_path_async(instance: u32, p_start: Vector3, p_finish: Vector3, p_maxDistance: f64) -> Option<Instance>;
	fn dyn_pathfinding_service_compute_smooth_path_async(instance: u32, p_start: Vector3, p_finish: Vector3, p_maxDistance: f64) -> Option<Instance>;
	fn dyn_pathfinding_service_find_path_async(instance: u32, p_start: Vector3, p_finish: Vector3) -> Option<Instance>;
	fn prop_pathfinding_service_empty_cutoff(instance: u32) -> f64;
	fn prop_set_pathfinding_service_empty_cutoff(instance: u32, value: f64);
	fn dyn_physics_service_collision_group_contains_part(instance: u32, p_name: &str, p_part: Option<BasePart>) -> bool;
	fn dyn_physics_service_collision_group_set_collidable(instance: u32, p_name1: &str, p_name2: &str, p_collidable: bool);
	fn dyn_physics_service_collision_groups_are_collidable(instance: u32, p_name1: &str, p_name2: &str) -> bool;
	fn dyn_physics_service_create_collision_group(instance: u32, p_name: &str) -> f64;
	fn dyn_physics_service_get_collision_group_id(instance: u32, p_name: &str) -> f64;
	fn dyn_physics_service_get_collision_group_name(instance: u32, p_name: f64) -> String;
	fn dyn_physics_service_get_collision_groups(instance: u32);
	fn dyn_physics_service_get_max_collision_groups(instance: u32) -> f64;
	fn dyn_physics_service_remove_collision_group(instance: u32, p_name: &str);
	fn dyn_physics_service_rename_collision_group(instance: u32, p_from: &str, p_to: &str);
	fn dyn_physics_service_set_part_collision_group(instance: u32, p_part: Option<BasePart>, p_name: &str);
	fn dyn_player_clear_character_appearance(instance: u32);
	fn dyn_player_distance_from_character(instance: u32, p_point: Vector3) -> f64;
	fn dyn_player_get_join_data(instance: u32);
	fn dyn_player_get_mouse(instance: u32) -> Option<Mouse>;
	fn dyn_player_get_network_ping(instance: u32) -> f64;
	fn dyn_player_has_appearance_loaded(instance: u32) -> bool;
	fn dyn_player_is_user_available_for_experiment(instance: u32) -> bool;
	fn dyn_player_kick(instance: u32, p_message: &str);
	fn dyn_player_load_boolean(instance: u32, p_key: &str) -> bool;
	fn dyn_player_load_character_appearance(instance: u32, p_assetInstance: Option<Instance>);
	fn dyn_player_load_instance(instance: u32, p_key: &str) -> Option<Instance>;
	fn dyn_player_load_number(instance: u32, p_key: &str) -> f64;
	fn dyn_player_load_string(instance: u32, p_key: &str) -> String;
	fn dyn_player_move(instance: u32, p_walkDirection: Vector3, p_relativeToCamera: bool);
	fn dyn_player_save_boolean(instance: u32, p_key: &str, p_value: bool);
	fn dyn_player_save_instance(instance: u32, p_key: &str, p_value: Option<Instance>);
	fn dyn_player_save_number(instance: u32, p_key: &str, p_value: f64);
	fn dyn_player_save_string(instance: u32, p_key: &str, p_value: &str);
	fn dyn_player_get_friends_online(instance: u32, p_maxFriends: f64);
	fn dyn_player_get_rank_in_group(instance: u32, p_groupId: f64) -> f64;
	fn dyn_player_get_role_in_group(instance: u32, p_groupId: f64) -> String;
	fn dyn_player_is_best_friends_with(instance: u32, p_userId: f64) -> bool;
	fn dyn_player_is_friends_with(instance: u32, p_userId: f64) -> bool;
	fn dyn_player_is_in_group(instance: u32, p_groupId: f64) -> bool;
	fn dyn_player_load_character(instance: u32);
	fn dyn_player_load_character_with_humanoid_description(instance: u32, p_humanoidDescription: Option<HumanoidDescription>);
	fn dyn_player_request_stream_around_async(instance: u32, p_position: Vector3, p_timeOut: f64);
	fn dyn_player_wait_for_data_ready(instance: u32) -> bool;
	fn prop_player_account_age(instance: u32) -> f64;
	fn prop_player_auto_jump_enabled(instance: u32) -> bool;
	fn prop_set_player_auto_jump_enabled(instance: u32, value: bool);
	fn prop_player_camera_max_zoom_distance(instance: u32) -> f64;
	fn prop_set_player_camera_max_zoom_distance(instance: u32, value: f64);
	fn prop_player_camera_min_zoom_distance(instance: u32) -> f64;
	fn prop_set_player_camera_min_zoom_distance(instance: u32, value: f64);
	fn prop_player_can_load_character_appearance(instance: u32) -> bool;
	fn prop_set_player_can_load_character_appearance(instance: u32, value: bool);
	fn prop_player_character(instance: u32) -> Option<Model>;
	fn prop_set_player_character(instance: u32, value: Option<Model>);
	fn prop_player_character_appearance(instance: u32) -> String;
	fn prop_set_player_character_appearance(instance: u32, value: &str);
	fn prop_player_character_appearance_id(instance: u32) -> f64;
	fn prop_set_player_character_appearance_id(instance: u32, value: f64);
	fn prop_player_data_complexity(instance: u32) -> f64;
	fn prop_player_data_ready(instance: u32) -> bool;
	fn prop_player_dev_enable_mouse_lock(instance: u32) -> bool;
	fn prop_set_player_dev_enable_mouse_lock(instance: u32, value: bool);
	fn prop_player_display_name(instance: u32) -> String;
	fn prop_set_player_display_name(instance: u32, value: &str);
	fn prop_player_follow_user_id(instance: u32) -> f64;
	fn prop_player_gameplay_paused(instance: u32) -> bool;
	fn prop_player_health_display_distance(instance: u32) -> f64;
	fn prop_set_player_health_display_distance(instance: u32, value: f64);
	fn prop_player_locale_id(instance: u32) -> String;
	fn prop_player_name_display_distance(instance: u32) -> f64;
	fn prop_set_player_name_display_distance(instance: u32, value: f64);
	fn prop_player_neutral(instance: u32) -> bool;
	fn prop_set_player_neutral(instance: u32, value: bool);
	fn prop_player_replication_focus(instance: u32) -> Option<Instance>;
	fn prop_set_player_replication_focus(instance: u32, value: Option<Instance>);
	fn prop_player_respawn_location(instance: u32) -> Option<SpawnLocation>;
	fn prop_set_player_respawn_location(instance: u32, value: Option<SpawnLocation>);
	fn prop_player_team(instance: u32) -> Option<Team>;
	fn prop_set_player_team(instance: u32, value: Option<Team>);
	fn prop_player_team_color(instance: u32) -> BrickColor;
	fn prop_set_player_team_color(instance: u32, value: BrickColor);
	fn prop_player_user_id(instance: u32) -> f64;
	fn prop_set_player_user_id(instance: u32, value: f64);
	fn connect_player_character_added(instance: u32, callback: Box<dyn Fn(Option<Model>)>) -> u32;
	fn connect_player_character_appearance_loaded(instance: u32, callback: Box<dyn Fn(Option<Model>)>) -> u32;
	fn connect_player_character_removing(instance: u32, callback: Box<dyn Fn(Option<Model>)>) -> u32;
	fn connect_player_chatted(instance: u32, callback: Box<dyn Fn(String, Option<Player>)>) -> u32;
	fn connect_player_idled(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn connect_player_on_teleport(instance: u32, callback: Box<dyn Fn((), f64, String)>) -> u32;
	fn dyn_player_scripts_clear_computer_camera_movement_modes(instance: u32);
	fn dyn_player_scripts_clear_computer_movement_modes(instance: u32);
	fn dyn_player_scripts_clear_touch_camera_movement_modes(instance: u32);
	fn dyn_player_scripts_clear_touch_movement_modes(instance: u32);
	fn dyn_player_scripts_register_computer_camera_movement_mode(instance: u32);
	fn dyn_player_scripts_register_computer_movement_mode(instance: u32);
	fn dyn_player_scripts_register_touch_camera_movement_mode(instance: u32);
	fn dyn_player_scripts_register_touch_movement_mode(instance: u32);
	fn dyn_players_get_player_by_user_id(instance: u32, p_userId: f64) -> Option<Player>;
	fn dyn_players_get_player_from_character(instance: u32, p_character: Option<Model>) -> Option<Player>;
	fn dyn_players_get_players(instance: u32) -> Objects;
	fn dyn_players_create_humanoid_model_from_description(instance: u32, p_description: Option<HumanoidDescription>) -> Option<Model>;
	fn dyn_players_create_humanoid_model_from_user_id(instance: u32, p_userId: f64) -> Option<Model>;
	fn dyn_players_get_character_appearance_async(instance: u32, p_userId: f64) -> Option<Model>;
	fn dyn_players_get_character_appearance_info_async(instance: u32, p_userId: f64);
	fn dyn_players_get_friends_async(instance: u32, p_userId: f64) -> Option<FriendPages>;
	fn dyn_players_get_humanoid_description_from_outfit_id(instance: u32, p_outfitId: f64) -> Option<HumanoidDescription>;
	fn dyn_players_get_humanoid_description_from_user_id(instance: u32, p_userId: f64) -> Option<HumanoidDescription>;
	fn dyn_players_get_name_from_user_id_async(instance: u32, p_userId: f64) -> String;
	fn dyn_players_get_user_id_from_name_async(instance: u32, p_userName: &str) -> f64;
	fn dyn_players_get_user_thumbnail_async(instance: u32, p_userId: f64);
	fn prop_players_bubble_chat(instance: u32) -> bool;
	fn prop_players_character_auto_loads(instance: u32) -> bool;
	fn prop_set_players_character_auto_loads(instance: u32, value: bool);
	fn prop_players_classic_chat(instance: u32) -> bool;
	fn prop_players_local_player(instance: u32) -> Option<Player>;
	fn prop_players_max_players(instance: u32) -> f64;
	fn prop_players_num_players(instance: u32) -> f64;
	fn prop_players_preferred_players(instance: u32) -> f64;
	fn prop_players_respawn_time(instance: u32) -> f64;
	fn prop_set_players_respawn_time(instance: u32, value: f64);
	fn connect_players_player_added(instance: u32, callback: Box<dyn Fn(Option<Player>)>) -> u32;
	fn connect_players_player_membership_changed(instance: u32, callback: Box<dyn Fn(Option<Player>)>) -> u32;
	fn connect_players_player_removing(instance: u32, callback: Box<dyn Fn(Option<Player>)>) -> u32;
	fn dyn_policy_service_get_policy_info_for_player_async(instance: u32, p_player: Option<Instance>);
	fn prop_pose_base_weight(instance: u32) -> f64;
	fn prop_set_pose_base_weight(instance: u32, value: f64);
	fn prop_number_pose_value(instance: u32) -> f64;
	fn prop_set_number_pose_value(instance: u32, value: f64);
	fn dyn_pose_add_sub_pose(instance: u32, p_pose: Option<Instance>);
	fn dyn_pose_get_sub_poses(instance: u32) -> Objects;
	fn dyn_pose_remove_sub_pose(instance: u32, p_pose: Option<Instance>);
	fn prop_pose_c_frame(instance: u32) -> CFrame;
	fn prop_set_pose_c_frame(instance: u32, value: CFrame);
	fn prop_pose_mask_weight(instance: u32) -> f64;
	fn prop_set_pose_mask_weight(instance: u32, value: f64);
	fn prop_post_effect_enabled(instance: u32) -> bool;
	fn prop_set_post_effect_enabled(instance: u32, value: bool);
	fn prop_bloom_effect_intensity(instance: u32) -> f64;
	fn prop_set_bloom_effect_intensity(instance: u32, value: f64);
	fn prop_bloom_effect_size(instance: u32) -> f64;
	fn prop_set_bloom_effect_size(instance: u32, value: f64);
	fn prop_bloom_effect_threshold(instance: u32) -> f64;
	fn prop_set_bloom_effect_threshold(instance: u32, value: f64);
	fn prop_blur_effect_size(instance: u32) -> f64;
	fn prop_set_blur_effect_size(instance: u32, value: f64);
	fn prop_color_correction_effect_brightness(instance: u32) -> f64;
	fn prop_set_color_correction_effect_brightness(instance: u32, value: f64);
	fn prop_color_correction_effect_contrast(instance: u32) -> f64;
	fn prop_set_color_correction_effect_contrast(instance: u32, value: f64);
	fn prop_color_correction_effect_saturation(instance: u32) -> f64;
	fn prop_set_color_correction_effect_saturation(instance: u32, value: f64);
	fn prop_color_correction_effect_tint_color(instance: u32) -> Color3;
	fn prop_set_color_correction_effect_tint_color(instance: u32, value: Color3);
	fn prop_depth_of_field_effect_far_intensity(instance: u32) -> f64;
	fn prop_set_depth_of_field_effect_far_intensity(instance: u32, value: f64);
	fn prop_depth_of_field_effect_focus_distance(instance: u32) -> f64;
	fn prop_set_depth_of_field_effect_focus_distance(instance: u32, value: f64);
	fn prop_depth_of_field_effect_in_focus_radius(instance: u32) -> f64;
	fn prop_set_depth_of_field_effect_in_focus_radius(instance: u32, value: f64);
	fn prop_depth_of_field_effect_near_intensity(instance: u32) -> f64;
	fn prop_set_depth_of_field_effect_near_intensity(instance: u32, value: f64);
	fn prop_sun_rays_effect_intensity(instance: u32) -> f64;
	fn prop_set_sun_rays_effect_intensity(instance: u32, value: f64);
	fn prop_sun_rays_effect_spread(instance: u32) -> f64;
	fn prop_set_sun_rays_effect_spread(instance: u32, value: f64);
	fn dyn_proximity_prompt_input_hold_begin(instance: u32);
	fn dyn_proximity_prompt_input_hold_end(instance: u32);
	fn prop_proximity_prompt_action_text(instance: u32) -> String;
	fn prop_set_proximity_prompt_action_text(instance: u32, value: &str);
	fn prop_proximity_prompt_auto_localize(instance: u32) -> bool;
	fn prop_set_proximity_prompt_auto_localize(instance: u32, value: bool);
	fn prop_proximity_prompt_clickable_prompt(instance: u32) -> bool;
	fn prop_set_proximity_prompt_clickable_prompt(instance: u32, value: bool);
	fn prop_proximity_prompt_enabled(instance: u32) -> bool;
	fn prop_set_proximity_prompt_enabled(instance: u32, value: bool);
	fn prop_proximity_prompt_hold_duration(instance: u32) -> f64;
	fn prop_set_proximity_prompt_hold_duration(instance: u32, value: f64);
	fn prop_proximity_prompt_max_activation_distance(instance: u32) -> f64;
	fn prop_set_proximity_prompt_max_activation_distance(instance: u32, value: f64);
	fn prop_proximity_prompt_object_text(instance: u32) -> String;
	fn prop_set_proximity_prompt_object_text(instance: u32, value: &str);
	fn prop_proximity_prompt_requires_line_of_sight(instance: u32) -> bool;
	fn prop_set_proximity_prompt_requires_line_of_sight(instance: u32, value: bool);
	fn prop_proximity_prompt_root_localization_table(instance: u32) -> Option<LocalizationTable>;
	fn prop_set_proximity_prompt_root_localization_table(instance: u32, value: Option<LocalizationTable>);
	fn prop_proximity_prompt_ui_offset(instance: u32) -> Vector2;
	fn prop_set_proximity_prompt_ui_offset(instance: u32, value: Vector2);
	fn connect_proximity_prompt_prompt_button_hold_began(instance: u32, callback: Box<dyn Fn(Option<Player>)>) -> u32;
	fn connect_proximity_prompt_prompt_button_hold_ended(instance: u32, callback: Box<dyn Fn(Option<Player>)>) -> u32;
	fn connect_proximity_prompt_prompt_hidden(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_proximity_prompt_prompt_shown(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_proximity_prompt_trigger_ended(instance: u32, callback: Box<dyn Fn(Option<Player>)>) -> u32;
	fn connect_proximity_prompt_triggered(instance: u32, callback: Box<dyn Fn(Option<Player>)>) -> u32;
	fn prop_proximity_prompt_service_enabled(instance: u32) -> bool;
	fn prop_set_proximity_prompt_service_enabled(instance: u32, value: bool);
	fn prop_proximity_prompt_service_max_prompts_visible(instance: u32) -> f64;
	fn prop_set_proximity_prompt_service_max_prompts_visible(instance: u32, value: f64);
	fn connect_proximity_prompt_service_prompt_button_hold_began(instance: u32, callback: Box<dyn Fn(Option<ProximityPrompt>, Option<Player>)>) -> u32;
	fn connect_proximity_prompt_service_prompt_button_hold_ended(instance: u32, callback: Box<dyn Fn(Option<ProximityPrompt>, Option<Player>)>) -> u32;
	fn connect_proximity_prompt_service_prompt_hidden(instance: u32, callback: Box<dyn Fn(Option<ProximityPrompt>)>) -> u32;
	fn connect_proximity_prompt_service_prompt_shown(instance: u32, callback: Box<dyn Fn(Option<ProximityPrompt>, ())>) -> u32;
	fn connect_proximity_prompt_service_prompt_trigger_ended(instance: u32, callback: Box<dyn Fn(Option<ProximityPrompt>, Option<Player>)>) -> u32;
	fn connect_proximity_prompt_service_prompt_triggered(instance: u32, callback: Box<dyn Fn(Option<ProximityPrompt>, Option<Player>)>) -> u32;
	fn dyn_remote_event_fire_all_clients(instance: u32);
	fn dyn_remote_event_fire_client(instance: u32, p_player: Option<Player>);
	fn dyn_remote_event_fire_server(instance: u32);
	fn connect_remote_event_on_client_event(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_remote_event_on_server_event(instance: u32, callback: Box<dyn Fn(Option<Player>, ())>) -> u32;
	fn dyn_remote_function_invoke_client(instance: u32, p_player: Option<Player>);
	fn dyn_remote_function_invoke_server(instance: u32);
	fn dyn_replicated_first_remove_default_loading_screen(instance: u32);
	fn dyn_rotation_curve_get_key_at_index(instance: u32, p_index: f64) -> RotationCurveKey;
	fn dyn_rotation_curve_get_key_indices_at_time(instance: u32, p_time: f64);
	fn dyn_rotation_curve_get_keys(instance: u32);
	fn dyn_rotation_curve_get_value_at_time(instance: u32, p_time: f64) -> Option<CFrame>;
	fn dyn_rotation_curve_insert_key(instance: u32, p_key: RotationCurveKey);
	fn dyn_rotation_curve_remove_key_at_index(instance: u32, p_startingIndex: f64, p_count: f64) -> f64;
	fn dyn_rotation_curve_set_keys(instance: u32) -> f64;
	fn prop_rotation_curve_length(instance: u32) -> f64;
	fn dyn_run_service_bind_to_render_step(instance: u32, p_name: &str, p_priority: f64, p_function: Function);
	fn dyn_run_service_is_client(instance: u32) -> bool;
	fn dyn_run_service_is_run_mode(instance: u32) -> bool;
	fn dyn_run_service_is_running(instance: u32) -> bool;
	fn dyn_run_service_is_server(instance: u32) -> bool;
	fn dyn_run_service_is_studio(instance: u32) -> bool;
	fn dyn_run_service_unbind_from_render_step(instance: u32, p_name: &str);
	fn connect_run_service_heartbeat(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn connect_run_service_post_simulation(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn connect_run_service_pre_animation(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn connect_run_service_pre_render(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn connect_run_service_pre_simulation(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn connect_run_service_render_stepped(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn connect_run_service_stepped(instance: u32, callback: Box<dyn Fn(f64, f64)>) -> u32;
	fn prop_screenshot_hud_camera_button_icon(instance: u32) -> Content;
	fn prop_set_screenshot_hud_camera_button_icon(instance: u32, value: Content);
	fn prop_screenshot_hud_camera_button_position(instance: u32) -> UDim2;
	fn prop_set_screenshot_hud_camera_button_position(instance: u32, value: UDim2);
	fn prop_screenshot_hud_close_button_position(instance: u32) -> UDim2;
	fn prop_set_screenshot_hud_close_button_position(instance: u32, value: UDim2);
	fn prop_screenshot_hud_close_when_screenshot_taken(instance: u32) -> bool;
	fn prop_set_screenshot_hud_close_when_screenshot_taken(instance: u32, value: bool);
	fn prop_screenshot_hud_experience_name_overlay_enabled(instance: u32) -> bool;
	fn prop_set_screenshot_hud_experience_name_overlay_enabled(instance: u32, value: bool);
	fn prop_screenshot_hud_username_overlay_enabled(instance: u32) -> bool;
	fn prop_set_screenshot_hud_username_overlay_enabled(instance: u32, value: bool);
	fn prop_screenshot_hud_visible(instance: u32) -> bool;
	fn prop_set_screenshot_hud_visible(instance: u32, value: bool);
	fn connect_script_context_error(instance: u32, callback: Box<dyn Fn(String, String, Option<Instance>)>) -> u32;
	fn prop_server_script_service_load_string_enabled(instance: u32) -> bool;
	fn prop_set_server_script_service_load_string_enabled(instance: u32, value: bool);
	fn dyn_service_provider_find_service(instance: u32, p_className: &str) -> Option<Instance>;
	fn dyn_service_provider_get_service(instance: u32, p_className: &str) -> Option<Instance>;
	fn connect_service_provider_close(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_service_provider_service_added(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn connect_service_provider_service_removing(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn dyn_data_model_bind_to_close(instance: u32, p_function: Function);
	fn dyn_data_model_get_message(instance: u32) -> String;
	fn dyn_data_model_get_remote_build_mode(instance: u32) -> bool;
	fn dyn_data_model_is_gear_type_allowed(instance: u32) -> bool;
	fn dyn_data_model_is_loaded(instance: u32) -> bool;
	fn dyn_data_model_save_place(instance: u32) -> bool;
	fn prop_data_model_creator_id(instance: u32) -> f64;
	fn prop_data_model_game_id(instance: u32) -> f64;
	fn prop_data_model_job_id(instance: u32) -> String;
	fn prop_data_model_place_id(instance: u32) -> f64;
	fn prop_data_model_place_version(instance: u32) -> f64;
	fn prop_data_model_private_server_id(instance: u32) -> String;
	fn prop_data_model_private_server_owner_id(instance: u32) -> f64;
	fn prop_data_model_vip_server_id(instance: u32) -> String;
	fn prop_data_model_vip_server_owner_id(instance: u32) -> f64;
	fn prop_data_model_workspace(instance: u32) -> Option<Workspace>;
	fn connect_data_model_allowed_gear_type_changed(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_data_model_graphics_quality_change_request(instance: u32, callback: Box<dyn Fn(bool)>) -> u32;
	fn connect_data_model_item_changed(instance: u32, callback: Box<dyn Fn(Option<Instance>, String)>) -> u32;
	fn connect_data_model_loaded(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn dyn_user_settings_is_user_feature_enabled(instance: u32, p_name: &str) -> bool;
	fn dyn_user_settings_reset(instance: u32);
	fn prop_sky_celestial_bodies_shown(instance: u32) -> bool;
	fn prop_set_sky_celestial_bodies_shown(instance: u32, value: bool);
	fn prop_sky_moon_angular_size(instance: u32) -> f64;
	fn prop_set_sky_moon_angular_size(instance: u32, value: f64);
	fn prop_sky_moon_texture_id(instance: u32) -> Content;
	fn prop_set_sky_moon_texture_id(instance: u32, value: Content);
	fn prop_sky_skybox_bk(instance: u32) -> Content;
	fn prop_set_sky_skybox_bk(instance: u32, value: Content);
	fn prop_sky_skybox_dn(instance: u32) -> Content;
	fn prop_set_sky_skybox_dn(instance: u32, value: Content);
	fn prop_sky_skybox_ft(instance: u32) -> Content;
	fn prop_set_sky_skybox_ft(instance: u32, value: Content);
	fn prop_sky_skybox_lf(instance: u32) -> Content;
	fn prop_set_sky_skybox_lf(instance: u32, value: Content);
	fn prop_sky_skybox_rt(instance: u32) -> Content;
	fn prop_set_sky_skybox_rt(instance: u32, value: Content);
	fn prop_sky_skybox_up(instance: u32) -> Content;
	fn prop_set_sky_skybox_up(instance: u32, value: Content);
	fn prop_sky_star_count(instance: u32) -> f64;
	fn prop_set_sky_star_count(instance: u32, value: f64);
	fn prop_sky_sun_angular_size(instance: u32) -> f64;
	fn prop_set_sky_sun_angular_size(instance: u32, value: f64);
	fn prop_sky_sun_texture_id(instance: u32) -> Content;
	fn prop_set_sky_sun_texture_id(instance: u32, value: Content);
	fn prop_smoke_color(instance: u32) -> Color3;
	fn prop_set_smoke_color(instance: u32, value: Color3);
	fn prop_smoke_enabled(instance: u32) -> bool;
	fn prop_set_smoke_enabled(instance: u32, value: bool);
	fn prop_smoke_opacity(instance: u32) -> f64;
	fn prop_set_smoke_opacity(instance: u32, value: f64);
	fn prop_smoke_rise_velocity(instance: u32) -> f64;
	fn prop_set_smoke_rise_velocity(instance: u32, value: f64);
	fn prop_smoke_size(instance: u32) -> f64;
	fn prop_set_smoke_size(instance: u32, value: f64);
	fn prop_smoke_time_scale(instance: u32) -> f64;
	fn prop_set_smoke_time_scale(instance: u32, value: f64);
	fn dyn_social_service_prompt_game_invite(instance: u32, p_player: Option<Instance>);
	fn dyn_social_service_can_send_game_invite_async(instance: u32, p_player: Option<Instance>) -> bool;
	fn connect_social_service_game_invite_prompt_closed(instance: u32, callback: Box<dyn Fn(Option<Instance>, ())>) -> u32;
	fn dyn_sound_pause(instance: u32);
	fn dyn_sound_play(instance: u32);
	fn dyn_sound_resume(instance: u32);
	fn dyn_sound_stop(instance: u32);
	fn prop_sound_emitter_size(instance: u32) -> f64;
	fn prop_set_sound_emitter_size(instance: u32, value: f64);
	fn prop_sound_is_loaded(instance: u32) -> bool;
	fn prop_sound_is_paused(instance: u32) -> bool;
	fn prop_sound_is_playing(instance: u32) -> bool;
	fn prop_sound_looped(instance: u32) -> bool;
	fn prop_set_sound_looped(instance: u32, value: bool);
	fn prop_sound_max_distance(instance: u32) -> f64;
	fn prop_set_sound_max_distance(instance: u32, value: f64);
	fn prop_sound_min_distance(instance: u32) -> f64;
	fn prop_set_sound_min_distance(instance: u32, value: f64);
	fn prop_sound_pitch(instance: u32) -> f64;
	fn prop_set_sound_pitch(instance: u32, value: f64);
	fn prop_sound_play_on_remove(instance: u32) -> bool;
	fn prop_set_sound_play_on_remove(instance: u32, value: bool);
	fn prop_sound_playback_loudness(instance: u32) -> f64;
	fn prop_sound_playback_speed(instance: u32) -> f64;
	fn prop_set_sound_playback_speed(instance: u32, value: f64);
	fn prop_sound_playing(instance: u32) -> bool;
	fn prop_set_sound_playing(instance: u32, value: bool);
	fn prop_sound_roll_off_max_distance(instance: u32) -> f64;
	fn prop_set_sound_roll_off_max_distance(instance: u32, value: f64);
	fn prop_sound_roll_off_min_distance(instance: u32) -> f64;
	fn prop_set_sound_roll_off_min_distance(instance: u32, value: f64);
	fn prop_sound_sound_group(instance: u32) -> Option<SoundGroup>;
	fn prop_set_sound_sound_group(instance: u32, value: Option<SoundGroup>);
	fn prop_sound_sound_id(instance: u32) -> Content;
	fn prop_set_sound_sound_id(instance: u32, value: Content);
	fn prop_sound_time_length(instance: u32) -> f64;
	fn prop_sound_time_position(instance: u32) -> f64;
	fn prop_set_sound_time_position(instance: u32, value: f64);
	fn prop_sound_volume(instance: u32) -> f64;
	fn prop_set_sound_volume(instance: u32, value: f64);
	fn connect_sound_did_loop(instance: u32, callback: Box<dyn Fn(String, f64)>) -> u32;
	fn connect_sound_ended(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_sound_loaded(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_sound_paused(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_sound_played(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_sound_resumed(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn connect_sound_stopped(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn prop_sound_effect_enabled(instance: u32) -> bool;
	fn prop_set_sound_effect_enabled(instance: u32, value: bool);
	fn prop_sound_effect_priority(instance: u32) -> f64;
	fn prop_set_sound_effect_priority(instance: u32, value: f64);
	fn prop_chorus_sound_effect_depth(instance: u32) -> f64;
	fn prop_set_chorus_sound_effect_depth(instance: u32, value: f64);
	fn prop_chorus_sound_effect_mix(instance: u32) -> f64;
	fn prop_set_chorus_sound_effect_mix(instance: u32, value: f64);
	fn prop_chorus_sound_effect_rate(instance: u32) -> f64;
	fn prop_set_chorus_sound_effect_rate(instance: u32, value: f64);
	fn prop_compressor_sound_effect_attack(instance: u32) -> f64;
	fn prop_set_compressor_sound_effect_attack(instance: u32, value: f64);
	fn prop_compressor_sound_effect_gain_makeup(instance: u32) -> f64;
	fn prop_set_compressor_sound_effect_gain_makeup(instance: u32, value: f64);
	fn prop_compressor_sound_effect_ratio(instance: u32) -> f64;
	fn prop_set_compressor_sound_effect_ratio(instance: u32, value: f64);
	fn prop_compressor_sound_effect_release(instance: u32) -> f64;
	fn prop_set_compressor_sound_effect_release(instance: u32, value: f64);
	fn prop_compressor_sound_effect_side_chain(instance: u32) -> Option<Instance>;
	fn prop_set_compressor_sound_effect_side_chain(instance: u32, value: Option<Instance>);
	fn prop_compressor_sound_effect_threshold(instance: u32) -> f64;
	fn prop_set_compressor_sound_effect_threshold(instance: u32, value: f64);
	fn prop_channel_selector_sound_effect_channel(instance: u32) -> f64;
	fn prop_set_channel_selector_sound_effect_channel(instance: u32, value: f64);
	fn prop_distortion_sound_effect_level(instance: u32) -> f64;
	fn prop_set_distortion_sound_effect_level(instance: u32, value: f64);
	fn prop_echo_sound_effect_delay(instance: u32) -> f64;
	fn prop_set_echo_sound_effect_delay(instance: u32, value: f64);
	fn prop_echo_sound_effect_dry_level(instance: u32) -> f64;
	fn prop_set_echo_sound_effect_dry_level(instance: u32, value: f64);
	fn prop_echo_sound_effect_feedback(instance: u32) -> f64;
	fn prop_set_echo_sound_effect_feedback(instance: u32, value: f64);
	fn prop_echo_sound_effect_wet_level(instance: u32) -> f64;
	fn prop_set_echo_sound_effect_wet_level(instance: u32, value: f64);
	fn prop_equalizer_sound_effect_high_gain(instance: u32) -> f64;
	fn prop_set_equalizer_sound_effect_high_gain(instance: u32, value: f64);
	fn prop_equalizer_sound_effect_low_gain(instance: u32) -> f64;
	fn prop_set_equalizer_sound_effect_low_gain(instance: u32, value: f64);
	fn prop_equalizer_sound_effect_mid_gain(instance: u32) -> f64;
	fn prop_set_equalizer_sound_effect_mid_gain(instance: u32, value: f64);
	fn prop_flange_sound_effect_depth(instance: u32) -> f64;
	fn prop_set_flange_sound_effect_depth(instance: u32, value: f64);
	fn prop_flange_sound_effect_mix(instance: u32) -> f64;
	fn prop_set_flange_sound_effect_mix(instance: u32, value: f64);
	fn prop_flange_sound_effect_rate(instance: u32) -> f64;
	fn prop_set_flange_sound_effect_rate(instance: u32, value: f64);
	fn prop_pitch_shift_sound_effect_octave(instance: u32) -> f64;
	fn prop_set_pitch_shift_sound_effect_octave(instance: u32, value: f64);
	fn prop_reverb_sound_effect_decay_time(instance: u32) -> f64;
	fn prop_set_reverb_sound_effect_decay_time(instance: u32, value: f64);
	fn prop_reverb_sound_effect_density(instance: u32) -> f64;
	fn prop_set_reverb_sound_effect_density(instance: u32, value: f64);
	fn prop_reverb_sound_effect_diffusion(instance: u32) -> f64;
	fn prop_set_reverb_sound_effect_diffusion(instance: u32, value: f64);
	fn prop_reverb_sound_effect_dry_level(instance: u32) -> f64;
	fn prop_set_reverb_sound_effect_dry_level(instance: u32, value: f64);
	fn prop_reverb_sound_effect_wet_level(instance: u32) -> f64;
	fn prop_set_reverb_sound_effect_wet_level(instance: u32, value: f64);
	fn prop_tremolo_sound_effect_depth(instance: u32) -> f64;
	fn prop_set_tremolo_sound_effect_depth(instance: u32, value: f64);
	fn prop_tremolo_sound_effect_duty(instance: u32) -> f64;
	fn prop_set_tremolo_sound_effect_duty(instance: u32, value: f64);
	fn prop_tremolo_sound_effect_frequency(instance: u32) -> f64;
	fn prop_set_tremolo_sound_effect_frequency(instance: u32, value: f64);
	fn prop_sound_group_volume(instance: u32) -> f64;
	fn prop_set_sound_group_volume(instance: u32, value: f64);
	fn dyn_sound_service_get_listener(instance: u32);
	fn dyn_sound_service_play_local_sound(instance: u32, p_sound: Option<Instance>);
	fn dyn_sound_service_set_listener(instance: u32);
	fn prop_sound_service_distance_factor(instance: u32) -> f64;
	fn prop_set_sound_service_distance_factor(instance: u32, value: f64);
	fn prop_sound_service_doppler_scale(instance: u32) -> f64;
	fn prop_set_sound_service_doppler_scale(instance: u32, value: f64);
	fn prop_sound_service_respect_filtering_enabled(instance: u32) -> bool;
	fn prop_set_sound_service_respect_filtering_enabled(instance: u32, value: bool);
	fn prop_sound_service_rolloff_scale(instance: u32) -> f64;
	fn prop_set_sound_service_rolloff_scale(instance: u32, value: f64);
	fn prop_sparkles_color(instance: u32) -> Color3;
	fn prop_set_sparkles_color(instance: u32, value: Color3);
	fn prop_sparkles_enabled(instance: u32) -> bool;
	fn prop_set_sparkles_enabled(instance: u32, value: bool);
	fn prop_sparkles_sparkle_color(instance: u32) -> Color3;
	fn prop_set_sparkles_sparkle_color(instance: u32, value: Color3);
	fn prop_sparkles_time_scale(instance: u32) -> f64;
	fn prop_set_sparkles_time_scale(instance: u32, value: f64);
	fn prop_speaker_channel_count(instance: u32) -> f64;
	fn prop_speaker_playback_loudness(instance: u32) -> f64;
	fn prop_speaker_roll_off_max_distance(instance: u32) -> f64;
	fn prop_set_speaker_roll_off_max_distance(instance: u32, value: f64);
	fn prop_speaker_roll_off_min_distance(instance: u32) -> f64;
	fn prop_set_speaker_roll_off_min_distance(instance: u32, value: f64);
	fn prop_speaker_sound_group(instance: u32) -> Option<SoundGroup>;
	fn prop_set_speaker_sound_group(instance: u32, value: Option<SoundGroup>);
	fn prop_speaker_source(instance: u32) -> Option<Instance>;
	fn prop_set_speaker_source(instance: u32, value: Option<Instance>);
	fn prop_speaker_volume(instance: u32) -> f64;
	fn prop_set_speaker_volume(instance: u32, value: f64);
	fn prop_starter_player_allow_custom_animations(instance: u32) -> bool;
	fn prop_starter_player_auto_jump_enabled(instance: u32) -> bool;
	fn prop_set_starter_player_auto_jump_enabled(instance: u32, value: bool);
	fn prop_starter_player_camera_max_zoom_distance(instance: u32) -> f64;
	fn prop_set_starter_player_camera_max_zoom_distance(instance: u32, value: f64);
	fn prop_starter_player_camera_min_zoom_distance(instance: u32) -> f64;
	fn prop_set_starter_player_camera_min_zoom_distance(instance: u32, value: f64);
	fn prop_starter_player_character_jump_height(instance: u32) -> f64;
	fn prop_set_starter_player_character_jump_height(instance: u32, value: f64);
	fn prop_starter_player_character_jump_power(instance: u32) -> f64;
	fn prop_set_starter_player_character_jump_power(instance: u32, value: f64);
	fn prop_starter_player_character_max_slope_angle(instance: u32) -> f64;
	fn prop_set_starter_player_character_max_slope_angle(instance: u32, value: f64);
	fn prop_starter_player_character_use_jump_power(instance: u32) -> bool;
	fn prop_set_starter_player_character_use_jump_power(instance: u32, value: bool);
	fn prop_starter_player_character_walk_speed(instance: u32) -> f64;
	fn prop_set_starter_player_character_walk_speed(instance: u32, value: f64);
	fn prop_starter_player_enable_mouse_lock_option(instance: u32) -> bool;
	fn prop_set_starter_player_enable_mouse_lock_option(instance: u32, value: bool);
	fn prop_starter_player_health_display_distance(instance: u32) -> f64;
	fn prop_set_starter_player_health_display_distance(instance: u32, value: f64);
	fn prop_starter_player_load_character_appearance(instance: u32) -> bool;
	fn prop_set_starter_player_load_character_appearance(instance: u32, value: bool);
	fn prop_starter_player_name_display_distance(instance: u32) -> f64;
	fn prop_set_starter_player_name_display_distance(instance: u32, value: f64);
	fn prop_starter_player_user_emotes_enabled(instance: u32) -> bool;
	fn prop_set_starter_player_user_emotes_enabled(instance: u32, value: bool);
	fn dyn_stats_get_memory_usage_mb_for_tag(instance: u32) -> f64;
	fn dyn_stats_get_total_memory_usage_mb(instance: u32) -> f64;
	fn prop_stats_contacts_count(instance: u32) -> f64;
	fn prop_stats_data_receive_kbps(instance: u32) -> f64;
	fn prop_stats_data_send_kbps(instance: u32) -> f64;
	fn prop_stats_heartbeat_time_ms(instance: u32) -> f64;
	fn prop_stats_instance_count(instance: u32) -> f64;
	fn prop_stats_moving_primitives_count(instance: u32) -> f64;
	fn prop_stats_physics_receive_kbps(instance: u32) -> f64;
	fn prop_stats_physics_send_kbps(instance: u32) -> f64;
	fn prop_stats_physics_step_time_ms(instance: u32) -> f64;
	fn prop_stats_primitives_count(instance: u32) -> f64;
	fn dyn_team_get_players(instance: u32) -> Objects;
	fn prop_team_auto_assignable(instance: u32) -> bool;
	fn prop_set_team_auto_assignable(instance: u32, value: bool);
	fn prop_team_auto_color_characters(instance: u32) -> bool;
	fn prop_set_team_auto_color_characters(instance: u32, value: bool);
	fn prop_team_score(instance: u32) -> f64;
	fn prop_set_team_score(instance: u32, value: f64);
	fn prop_team_team_color(instance: u32) -> BrickColor;
	fn prop_set_team_team_color(instance: u32, value: BrickColor);
	fn connect_team_player_added(instance: u32, callback: Box<dyn Fn(Option<Player>)>) -> u32;
	fn connect_team_player_removed(instance: u32, callback: Box<dyn Fn(Option<Player>)>) -> u32;
	fn dyn_teams_get_teams(instance: u32) -> Objects;
	fn dyn_teams_rebalance_teams(instance: u32);
	fn prop_teleport_async_result_private_server_id(instance: u32) -> String;
	fn prop_teleport_async_result_reserved_server_access_code(instance: u32) -> String;
	fn dyn_teleport_options_get_teleport_data(instance: u32);
	fn dyn_teleport_options_set_teleport_data(instance: u32);
	fn prop_teleport_options_reserved_server_access_code(instance: u32) -> String;
	fn prop_set_teleport_options_reserved_server_access_code(instance: u32, value: &str);
	fn prop_teleport_options_server_instance_id(instance: u32) -> String;
	fn prop_set_teleport_options_server_instance_id(instance: u32, value: &str);
	fn prop_teleport_options_should_reserve_server(instance: u32) -> bool;
	fn prop_set_teleport_options_should_reserve_server(instance: u32, value: bool);
	fn dyn_teleport_service_get_arriving_teleport_gui(instance: u32) -> Option<Instance>;
	fn dyn_teleport_service_get_local_player_teleport_data(instance: u32);
	fn dyn_teleport_service_get_teleport_setting(instance: u32, p_setting: &str);
	fn dyn_teleport_service_set_teleport_gui(instance: u32, p_gui: Option<Instance>);
	fn dyn_teleport_service_set_teleport_setting(instance: u32, p_setting: &str);
	fn dyn_teleport_service_teleport(instance: u32, p_placeId: f64, p_player: Option<Instance>, p_customLoadingScreen: Option<Instance>);
	fn dyn_teleport_service_teleport_to_place_instance(instance: u32, p_placeId: f64, p_instanceId: &str, p_player: Option<Instance>, p_spawnName: &str, p_customLoadingScreen: Option<Instance>);
	fn dyn_teleport_service_teleport_to_private_server(instance: u32, p_placeId: f64, p_reservedServerAccessCode: &str, p_players: Objects, p_spawnName: &str, p_customLoadingScreen: Option<Instance>);
	fn dyn_teleport_service_teleport_to_spawn_by_name(instance: u32, p_placeId: f64, p_spawnName: &str, p_player: Option<Instance>, p_customLoadingScreen: Option<Instance>);
	fn dyn_teleport_service_get_player_place_instance_async(instance: u32, p_userId: f64);
	fn dyn_teleport_service_reserve_server(instance: u32, p_placeId: f64);
	fn dyn_teleport_service_teleport_async(instance: u32, p_placeId: f64, p_players: Objects, p_teleportOptions: Option<Instance>) -> Option<Instance>;
	fn dyn_teleport_service_teleport_party_async(instance: u32, p_placeId: f64, p_players: Objects, p_customLoadingScreen: Option<Instance>) -> String;
	fn prop_teleport_service_customized_teleport_ui(instance: u32) -> bool;
	fn prop_set_teleport_service_customized_teleport_ui(instance: u32, value: bool);
	fn connect_teleport_service_local_player_arrived_from_teleport(instance: u32, callback: Box<dyn Fn(Option<Instance>, ())>) -> u32;
	fn connect_teleport_service_teleport_init_failed(instance: u32, callback: Box<dyn Fn(Option<Instance>, (), String, f64, Option<Instance>)>) -> u32;
	fn prop_terrain_detail_studs_per_tile(instance: u32) -> f64;
	fn prop_set_terrain_detail_studs_per_tile(instance: u32, value: f64);
	fn prop_terrain_region_is_smooth(instance: u32) -> bool;
	fn prop_terrain_region_size_in_cells(instance: u32) -> Vector3;
	fn dyn_text_channel_display_system_message(instance: u32, p_systemMessage: &str, p_metadata: &str) -> Option<TextChatMessage>;
	fn dyn_text_channel_add_user_async(instance: u32, p_userId: f64);
	fn dyn_text_channel_send_async(instance: u32, p_message: &str, p_metadata: &str) -> Option<TextChatMessage>;
	fn connect_text_channel_message_received(instance: u32, callback: Box<dyn Fn(Option<TextChatMessage>)>) -> u32;
	fn prop_text_chat_command_enabled(instance: u32) -> bool;
	fn prop_set_text_chat_command_enabled(instance: u32, value: bool);
	fn prop_text_chat_command_primary_alias(instance: u32) -> String;
	fn prop_set_text_chat_command_primary_alias(instance: u32, value: &str);
	fn prop_text_chat_command_secondary_alias(instance: u32) -> String;
	fn prop_set_text_chat_command_secondary_alias(instance: u32, value: &str);
	fn connect_text_chat_command_triggered(instance: u32, callback: Box<dyn Fn(Option<TextSource>, String)>) -> u32;
	fn prop_chat_input_bar_configuration_enabled(instance: u32) -> bool;
	fn prop_set_chat_input_bar_configuration_enabled(instance: u32, value: bool);
	fn prop_chat_input_bar_configuration_target_text_channel(instance: u32) -> Option<TextChannel>;
	fn prop_set_chat_input_bar_configuration_target_text_channel(instance: u32, value: Option<TextChannel>);
	fn prop_chat_window_configuration_enabled(instance: u32) -> bool;
	fn prop_set_chat_window_configuration_enabled(instance: u32, value: bool);
	fn prop_text_chat_message_message_id(instance: u32) -> String;
	fn prop_set_text_chat_message_message_id(instance: u32, value: &str);
	fn prop_text_chat_message_metadata(instance: u32) -> String;
	fn prop_set_text_chat_message_metadata(instance: u32, value: &str);
	fn prop_text_chat_message_prefix_text(instance: u32) -> String;
	fn prop_set_text_chat_message_prefix_text(instance: u32, value: &str);
	fn prop_text_chat_message_text(instance: u32) -> String;
	fn prop_set_text_chat_message_text(instance: u32, value: &str);
	fn prop_text_chat_message_text_channel(instance: u32) -> Option<TextChannel>;
	fn prop_set_text_chat_message_text_channel(instance: u32, value: Option<TextChannel>);
	fn prop_text_chat_message_text_source(instance: u32) -> Option<TextSource>;
	fn prop_set_text_chat_message_text_source(instance: u32, value: Option<TextSource>);
	fn prop_text_chat_message_timestamp(instance: u32) -> DateTime;
	fn prop_set_text_chat_message_timestamp(instance: u32, value: DateTime);
	fn prop_text_chat_message_properties_prefix_text(instance: u32) -> String;
	fn prop_set_text_chat_message_properties_prefix_text(instance: u32, value: &str);
	fn prop_text_chat_message_properties_text(instance: u32) -> String;
	fn prop_set_text_chat_message_properties_text(instance: u32, value: &str);
	fn prop_text_chat_service_create_default_commands(instance: u32) -> bool;
	fn prop_text_chat_service_create_default_text_channels(instance: u32) -> bool;
	fn connect_text_chat_service_message_received(instance: u32, callback: Box<dyn Fn(Option<TextChatMessage>)>) -> u32;
	fn connect_text_chat_service_sending_message(instance: u32, callback: Box<dyn Fn(Option<TextChatMessage>)>) -> u32;
	fn dyn_text_filter_result_get_chat_for_user_async(instance: u32, p_toUserId: f64) -> String;
	fn dyn_text_filter_result_get_non_chat_string_for_broadcast_async(instance: u32) -> String;
	fn dyn_text_filter_result_get_non_chat_string_for_user_async(instance: u32, p_toUserId: f64) -> String;
	fn dyn_text_service_get_text_size(instance: u32, p_string: &str, p_fontSize: f64, p_frameSize: Vector2) -> Vector2;
	fn dyn_text_service_filter_string_async(instance: u32, p_stringToFilter: &str, p_fromUserId: f64) -> Option<Instance>;
	fn dyn_text_service_get_family_info_async(instance: u32, p_assetId: Content);
	fn dyn_text_service_get_text_bounds_async(instance: u32, p_params: Option<GetTextBoundsParams>) -> Vector2;
	fn prop_text_source_can_send(instance: u32) -> bool;
	fn prop_set_text_source_can_send(instance: u32, value: bool);
	fn prop_text_source_user_id(instance: u32) -> f64;
	fn dyn_trail_clear(instance: u32);
	fn prop_trail_attachment_0(instance: u32) -> Option<Attachment>;
	fn prop_set_trail_attachment_0(instance: u32, value: Option<Attachment>);
	fn prop_trail_attachment_1(instance: u32) -> Option<Attachment>;
	fn prop_set_trail_attachment_1(instance: u32, value: Option<Attachment>);
	fn prop_trail_brightness(instance: u32) -> f64;
	fn prop_set_trail_brightness(instance: u32, value: f64);
	fn prop_trail_color(instance: u32) -> ColorSequence;
	fn prop_set_trail_color(instance: u32, value: ColorSequence);
	fn prop_trail_enabled(instance: u32) -> bool;
	fn prop_set_trail_enabled(instance: u32, value: bool);
	fn prop_trail_face_camera(instance: u32) -> bool;
	fn prop_set_trail_face_camera(instance: u32, value: bool);
	fn prop_trail_lifetime(instance: u32) -> f64;
	fn prop_set_trail_lifetime(instance: u32, value: f64);
	fn prop_trail_light_emission(instance: u32) -> f64;
	fn prop_set_trail_light_emission(instance: u32, value: f64);
	fn prop_trail_light_influence(instance: u32) -> f64;
	fn prop_set_trail_light_influence(instance: u32, value: f64);
	fn prop_trail_max_length(instance: u32) -> f64;
	fn prop_set_trail_max_length(instance: u32, value: f64);
	fn prop_trail_min_length(instance: u32) -> f64;
	fn prop_set_trail_min_length(instance: u32, value: f64);
	fn prop_trail_texture(instance: u32) -> Content;
	fn prop_set_trail_texture(instance: u32, value: Content);
	fn prop_trail_texture_length(instance: u32) -> f64;
	fn prop_set_trail_texture_length(instance: u32, value: f64);
	fn prop_trail_transparency(instance: u32) -> NumberSequence;
	fn prop_set_trail_transparency(instance: u32, value: NumberSequence);
	fn prop_trail_width_scale(instance: u32) -> NumberSequence;
	fn prop_set_trail_width_scale(instance: u32, value: NumberSequence);
	fn dyn_translator_format_by_key(instance: u32, p_key: &str) -> String;
	fn dyn_translator_translate(instance: u32, p_context: Option<Instance>, p_text: &str) -> String;
	fn prop_translator_locale_id(instance: u32) -> String;
	fn dyn_tween_base_cancel(instance: u32);
	fn dyn_tween_base_pause(instance: u32);
	fn dyn_tween_base_play(instance: u32);
	fn connect_tween_base_completed(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn prop_tween_instance(instance: u32) -> Option<Instance>;
	fn prop_tween_tween_info(instance: u32) -> TweenInfo;
	fn dyn_tween_service_create(instance: u32, p_instance: Option<Instance>, p_tweenInfo: TweenInfo) -> Option<Tween>;
	fn dyn_tween_service_get_value(instance: u32, p_alpha: f64) -> f64;
	fn prop_ui_aspect_ratio_constraint_aspect_ratio(instance: u32) -> f64;
	fn prop_set_ui_aspect_ratio_constraint_aspect_ratio(instance: u32, value: f64);
	fn prop_ui_size_constraint_max_size(instance: u32) -> Vector2;
	fn prop_set_ui_size_constraint_max_size(instance: u32, value: Vector2);
	fn prop_ui_size_constraint_min_size(instance: u32) -> Vector2;
	fn prop_set_ui_size_constraint_min_size(instance: u32, value: Vector2);
	fn prop_ui_text_size_constraint_max_text_size(instance: u32) -> f64;
	fn prop_set_ui_text_size_constraint_max_text_size(instance: u32, value: f64);
	fn prop_ui_text_size_constraint_min_text_size(instance: u32) -> f64;
	fn prop_set_ui_text_size_constraint_min_text_size(instance: u32, value: f64);
	fn prop_ui_corner_corner_radius(instance: u32) -> UDim;
	fn prop_set_ui_corner_corner_radius(instance: u32, value: UDim);
	fn prop_ui_gradient_color(instance: u32) -> ColorSequence;
	fn prop_set_ui_gradient_color(instance: u32, value: ColorSequence);
	fn prop_ui_gradient_enabled(instance: u32) -> bool;
	fn prop_set_ui_gradient_enabled(instance: u32, value: bool);
	fn prop_ui_gradient_offset(instance: u32) -> Vector2;
	fn prop_set_ui_gradient_offset(instance: u32, value: Vector2);
	fn prop_ui_gradient_rotation(instance: u32) -> f64;
	fn prop_set_ui_gradient_rotation(instance: u32, value: f64);
	fn prop_ui_gradient_transparency(instance: u32) -> NumberSequence;
	fn prop_set_ui_gradient_transparency(instance: u32, value: NumberSequence);
	fn dyn_ui_grid_style_layout_apply_layout(instance: u32);
	fn dyn_ui_grid_style_layout_set_custom_sort_function(instance: u32, p_function: Function);
	fn prop_ui_grid_style_layout_absolute_content_size(instance: u32) -> Vector2;
	fn prop_ui_grid_layout_absolute_cell_count(instance: u32) -> Vector2;
	fn prop_ui_grid_layout_absolute_cell_size(instance: u32) -> Vector2;
	fn prop_ui_grid_layout_cell_padding(instance: u32) -> UDim2;
	fn prop_set_ui_grid_layout_cell_padding(instance: u32, value: UDim2);
	fn prop_ui_grid_layout_cell_size(instance: u32) -> UDim2;
	fn prop_set_ui_grid_layout_cell_size(instance: u32, value: UDim2);
	fn prop_ui_grid_layout_fill_direction_max_cells(instance: u32) -> f64;
	fn prop_set_ui_grid_layout_fill_direction_max_cells(instance: u32, value: f64);
	fn prop_ui_list_layout_padding(instance: u32) -> UDim;
	fn prop_set_ui_list_layout_padding(instance: u32, value: UDim);
	fn dyn_ui_page_layout_jump_to(instance: u32, p_page: Option<Instance>);
	fn dyn_ui_page_layout_jump_to_index(instance: u32, p_index: f64);
	fn dyn_ui_page_layout_next(instance: u32);
	fn dyn_ui_page_layout_previous(instance: u32);
	fn prop_ui_page_layout_animated(instance: u32) -> bool;
	fn prop_set_ui_page_layout_animated(instance: u32, value: bool);
	fn prop_ui_page_layout_circular(instance: u32) -> bool;
	fn prop_set_ui_page_layout_circular(instance: u32, value: bool);
	fn prop_ui_page_layout_current_page(instance: u32) -> Option<GuiObject>;
	fn prop_ui_page_layout_gamepad_input_enabled(instance: u32) -> bool;
	fn prop_set_ui_page_layout_gamepad_input_enabled(instance: u32, value: bool);
	fn prop_ui_page_layout_padding(instance: u32) -> UDim;
	fn prop_set_ui_page_layout_padding(instance: u32, value: UDim);
	fn prop_ui_page_layout_scroll_wheel_input_enabled(instance: u32) -> bool;
	fn prop_set_ui_page_layout_scroll_wheel_input_enabled(instance: u32, value: bool);
	fn prop_ui_page_layout_touch_input_enabled(instance: u32) -> bool;
	fn prop_set_ui_page_layout_touch_input_enabled(instance: u32, value: bool);
	fn prop_ui_page_layout_tween_time(instance: u32) -> f64;
	fn prop_set_ui_page_layout_tween_time(instance: u32, value: f64);
	fn connect_ui_page_layout_page_enter(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn connect_ui_page_layout_page_leave(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn connect_ui_page_layout_stopped(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn prop_ui_table_layout_fill_empty_space_columns(instance: u32) -> bool;
	fn prop_set_ui_table_layout_fill_empty_space_columns(instance: u32, value: bool);
	fn prop_ui_table_layout_fill_empty_space_rows(instance: u32) -> bool;
	fn prop_set_ui_table_layout_fill_empty_space_rows(instance: u32, value: bool);
	fn prop_ui_table_layout_padding(instance: u32) -> UDim2;
	fn prop_set_ui_table_layout_padding(instance: u32, value: UDim2);
	fn prop_ui_padding_padding_bottom(instance: u32) -> UDim;
	fn prop_set_ui_padding_padding_bottom(instance: u32, value: UDim);
	fn prop_ui_padding_padding_left(instance: u32) -> UDim;
	fn prop_set_ui_padding_padding_left(instance: u32, value: UDim);
	fn prop_ui_padding_padding_right(instance: u32) -> UDim;
	fn prop_set_ui_padding_padding_right(instance: u32, value: UDim);
	fn prop_ui_padding_padding_top(instance: u32) -> UDim;
	fn prop_set_ui_padding_padding_top(instance: u32, value: UDim);
	fn prop_ui_scale_scale(instance: u32) -> f64;
	fn prop_set_ui_scale_scale(instance: u32, value: f64);
	fn prop_ui_stroke_color(instance: u32) -> Color3;
	fn prop_set_ui_stroke_color(instance: u32, value: Color3);
	fn prop_ui_stroke_enabled(instance: u32) -> bool;
	fn prop_set_ui_stroke_enabled(instance: u32, value: bool);
	fn prop_ui_stroke_thickness(instance: u32) -> f64;
	fn prop_set_ui_stroke_thickness(instance: u32, value: f64);
	fn prop_ui_stroke_transparency(instance: u32) -> f64;
	fn prop_set_ui_stroke_transparency(instance: u32, value: f64);
	fn dyn_user_game_settings_get_camera_y_invert_value(instance: u32) -> f64;
	fn dyn_user_game_settings_get_onboarding_completed(instance: u32, p_onboardingId: &str) -> bool;
	fn dyn_user_game_settings_in_full_screen(instance: u32) -> bool;
	fn dyn_user_game_settings_in_studio_mode(instance: u32) -> bool;
	fn dyn_user_game_settings_set_camera_y_invert_visible(instance: u32);
	fn dyn_user_game_settings_set_gamepad_camera_sensitivity_visible(instance: u32);
	fn dyn_user_game_settings_set_onboarding_completed(instance: u32, p_onboardingId: &str);
	fn prop_user_game_settings_gamepad_camera_sensitivity(instance: u32) -> f64;
	fn prop_set_user_game_settings_gamepad_camera_sensitivity(instance: u32, value: f64);
	fn prop_user_game_settings_mouse_sensitivity(instance: u32) -> f64;
	fn prop_set_user_game_settings_mouse_sensitivity(instance: u32, value: f64);
	fn prop_user_game_settings_rcc_profiler_record_frame_rate(instance: u32) -> f64;
	fn prop_set_user_game_settings_rcc_profiler_record_frame_rate(instance: u32, value: f64);
	fn prop_user_game_settings_rcc_profiler_record_time_frame(instance: u32) -> f64;
	fn prop_set_user_game_settings_rcc_profiler_record_time_frame(instance: u32, value: f64);
	fn prop_user_game_settings_vignette_enabled(instance: u32) -> bool;
	fn connect_user_game_settings_fullscreen_changed(instance: u32, callback: Box<dyn Fn(bool)>) -> u32;
	fn connect_user_game_settings_studio_mode_changed(instance: u32, callback: Box<dyn Fn(bool)>) -> u32;
	fn dyn_user_input_service_gamepad_supports(instance: u32) -> bool;
	fn dyn_user_input_service_get_connected_gamepads(instance: u32);
	fn dyn_user_input_service_get_device_acceleration(instance: u32) -> Option<InputObject>;
	fn dyn_user_input_service_get_device_gravity(instance: u32) -> Option<InputObject>;
	fn dyn_user_input_service_get_device_rotation(instance: u32);
	fn dyn_user_input_service_get_focused_text_box(instance: u32) -> Option<TextBox>;
	fn dyn_user_input_service_get_gamepad_connected(instance: u32) -> bool;
	fn dyn_user_input_service_get_gamepad_state(instance: u32);
	fn dyn_user_input_service_get_keys_pressed(instance: u32);
	fn dyn_user_input_service_get_last_input_type(instance: u32);
	fn dyn_user_input_service_get_mouse_buttons_pressed(instance: u32);
	fn dyn_user_input_service_get_mouse_delta(instance: u32) -> Vector2;
	fn dyn_user_input_service_get_mouse_location(instance: u32) -> Vector2;
	fn dyn_user_input_service_get_navigation_gamepads(instance: u32);
	fn dyn_user_input_service_get_string_for_key_code(instance: u32) -> String;
	fn dyn_user_input_service_get_supported_gamepad_key_codes(instance: u32);
	fn dyn_user_input_service_get_user_c_frame(instance: u32) -> CFrame;
	fn dyn_user_input_service_is_gamepad_button_down(instance: u32) -> bool;
	fn dyn_user_input_service_is_key_down(instance: u32) -> bool;
	fn dyn_user_input_service_is_mouse_button_pressed(instance: u32) -> bool;
	fn dyn_user_input_service_is_navigation_gamepad(instance: u32) -> bool;
	fn dyn_user_input_service_recenter_user_head_c_frame(instance: u32);
	fn dyn_user_input_service_set_navigation_gamepad(instance: u32, p_enabled: bool);
	fn prop_user_input_service_accelerometer_enabled(instance: u32) -> bool;
	fn prop_user_input_service_gamepad_enabled(instance: u32) -> bool;
	fn prop_user_input_service_gyroscope_enabled(instance: u32) -> bool;
	fn prop_user_input_service_keyboard_enabled(instance: u32) -> bool;
	fn prop_user_input_service_modal_enabled(instance: u32) -> bool;
	fn prop_set_user_input_service_modal_enabled(instance: u32, value: bool);
	fn prop_user_input_service_mouse_delta_sensitivity(instance: u32) -> f64;
	fn prop_set_user_input_service_mouse_delta_sensitivity(instance: u32, value: f64);
	fn prop_user_input_service_mouse_enabled(instance: u32) -> bool;
	fn prop_user_input_service_mouse_icon_enabled(instance: u32) -> bool;
	fn prop_set_user_input_service_mouse_icon_enabled(instance: u32, value: bool);
	fn prop_user_input_service_on_screen_keyboard_position(instance: u32) -> Vector2;
	fn prop_user_input_service_on_screen_keyboard_size(instance: u32) -> Vector2;
	fn prop_user_input_service_on_screen_keyboard_visible(instance: u32) -> bool;
	fn prop_user_input_service_touch_enabled(instance: u32) -> bool;
	fn prop_user_input_service_user_head_c_frame(instance: u32) -> CFrame;
	fn prop_user_input_service_vr_enabled(instance: u32) -> bool;
	fn connect_user_input_service_device_acceleration_changed(instance: u32, callback: Box<dyn Fn(Option<InputObject>)>) -> u32;
	fn connect_user_input_service_device_gravity_changed(instance: u32, callback: Box<dyn Fn(Option<InputObject>)>) -> u32;
	fn connect_user_input_service_device_rotation_changed(instance: u32, callback: Box<dyn Fn(Option<InputObject>, CFrame)>) -> u32;
	fn connect_user_input_service_gamepad_connected(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_user_input_service_gamepad_disconnected(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_user_input_service_input_began(instance: u32, callback: Box<dyn Fn(Option<InputObject>, bool)>) -> u32;
	fn connect_user_input_service_input_changed(instance: u32, callback: Box<dyn Fn(Option<InputObject>, bool)>) -> u32;
	fn connect_user_input_service_input_ended(instance: u32, callback: Box<dyn Fn(Option<InputObject>, bool)>) -> u32;
	fn connect_user_input_service_jump_request(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_user_input_service_last_input_type_changed(instance: u32, callback: Box<dyn Fn(())>) -> u32;
	fn connect_user_input_service_pointer_action(instance: u32, callback: Box<dyn Fn(f64, Vector2, f64, bool)>) -> u32;
	fn connect_user_input_service_text_box_focus_released(instance: u32, callback: Box<dyn Fn(Option<TextBox>)>) -> u32;
	fn connect_user_input_service_text_box_focused(instance: u32, callback: Box<dyn Fn(Option<TextBox>)>) -> u32;
	fn connect_user_input_service_touch_ended(instance: u32, callback: Box<dyn Fn(Option<InputObject>, bool)>) -> u32;
	fn connect_user_input_service_touch_long_press(instance: u32, callback: Box<dyn Fn((), (), bool)>) -> u32;
	fn connect_user_input_service_touch_moved(instance: u32, callback: Box<dyn Fn(Option<InputObject>, bool)>) -> u32;
	fn connect_user_input_service_touch_pan(instance: u32, callback: Box<dyn Fn((), Vector2, Vector2, (), bool)>) -> u32;
	fn connect_user_input_service_touch_pinch(instance: u32, callback: Box<dyn Fn((), f64, f64, (), bool)>) -> u32;
	fn connect_user_input_service_touch_rotate(instance: u32, callback: Box<dyn Fn((), f64, f64, (), bool)>) -> u32;
	fn connect_user_input_service_touch_started(instance: u32, callback: Box<dyn Fn(Option<InputObject>, bool)>) -> u32;
	fn connect_user_input_service_touch_swipe(instance: u32, callback: Box<dyn Fn((), f64, bool)>) -> u32;
	fn connect_user_input_service_touch_tap(instance: u32, callback: Box<dyn Fn((), bool)>) -> u32;
	fn connect_user_input_service_touch_tap_in_world(instance: u32, callback: Box<dyn Fn(Vector2, bool)>) -> u32;
	fn connect_user_input_service_user_c_frame_changed(instance: u32, callback: Box<dyn Fn((), CFrame)>) -> u32;
	fn connect_user_input_service_window_focus_released(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn connect_user_input_service_window_focused(instance: u32, callback: Box<dyn Fn()>) -> u32;
	fn dyn_user_service_get_user_infos_by_user_ids_async(instance: u32);
	fn dyn_vr_service_get_touchpad_mode(instance: u32);
	fn dyn_vr_service_get_user_c_frame(instance: u32) -> CFrame;
	fn dyn_vr_service_get_user_c_frame_enabled(instance: u32) -> bool;
	fn dyn_vr_service_recenter_user_head_c_frame(instance: u32);
	fn dyn_vr_service_request_navigation(instance: u32, p_cframe: CFrame);
	fn dyn_vr_service_set_touchpad_mode(instance: u32);
	fn prop_vr_service_vr_enabled(instance: u32) -> bool;
	fn connect_vr_service_navigation_requested(instance: u32, callback: Box<dyn Fn(CFrame, ())>) -> u32;
	fn connect_vr_service_touchpad_mode_changed(instance: u32, callback: Box<dyn Fn((), ())>) -> u32;
	fn connect_vr_service_user_c_frame_changed(instance: u32, callback: Box<dyn Fn((), CFrame)>) -> u32;
	fn connect_vr_service_user_c_frame_enabled(instance: u32, callback: Box<dyn Fn((), bool)>) -> u32;
	fn prop_bool_value_value(instance: u32) -> bool;
	fn prop_set_bool_value_value(instance: u32, value: bool);
	fn connect_bool_value_changed(instance: u32, callback: Box<dyn Fn(bool)>) -> u32;
	fn prop_brick_color_value_value(instance: u32) -> BrickColor;
	fn prop_set_brick_color_value_value(instance: u32, value: BrickColor);
	fn connect_brick_color_value_changed(instance: u32, callback: Box<dyn Fn(BrickColor)>) -> u32;
	fn prop_c_frame_value_value(instance: u32) -> CFrame;
	fn prop_set_c_frame_value_value(instance: u32, value: CFrame);
	fn connect_c_frame_value_changed(instance: u32, callback: Box<dyn Fn(CFrame)>) -> u32;
	fn prop_color_3_value_value(instance: u32) -> Color3;
	fn prop_set_color_3_value_value(instance: u32, value: Color3);
	fn connect_color_3_value_changed(instance: u32, callback: Box<dyn Fn(Color3)>) -> u32;
	fn prop_double_constrained_value_constrained_value(instance: u32) -> f64;
	fn prop_set_double_constrained_value_constrained_value(instance: u32, value: f64);
	fn prop_double_constrained_value_max_value(instance: u32) -> f64;
	fn prop_set_double_constrained_value_max_value(instance: u32, value: f64);
	fn prop_double_constrained_value_min_value(instance: u32) -> f64;
	fn prop_set_double_constrained_value_min_value(instance: u32, value: f64);
	fn prop_double_constrained_value_value(instance: u32) -> f64;
	fn prop_set_double_constrained_value_value(instance: u32, value: f64);
	fn connect_double_constrained_value_changed(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn prop_int_constrained_value_constrained_value(instance: u32) -> f64;
	fn prop_set_int_constrained_value_constrained_value(instance: u32, value: f64);
	fn prop_int_constrained_value_max_value(instance: u32) -> f64;
	fn prop_set_int_constrained_value_max_value(instance: u32, value: f64);
	fn prop_int_constrained_value_min_value(instance: u32) -> f64;
	fn prop_set_int_constrained_value_min_value(instance: u32, value: f64);
	fn prop_int_constrained_value_value(instance: u32) -> f64;
	fn prop_set_int_constrained_value_value(instance: u32, value: f64);
	fn connect_int_constrained_value_changed(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn prop_int_value_value(instance: u32) -> f64;
	fn prop_set_int_value_value(instance: u32, value: f64);
	fn connect_int_value_changed(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn prop_number_value_value(instance: u32) -> f64;
	fn prop_set_number_value_value(instance: u32, value: f64);
	fn connect_number_value_changed(instance: u32, callback: Box<dyn Fn(f64)>) -> u32;
	fn prop_object_value_value(instance: u32) -> Option<Instance>;
	fn prop_set_object_value_value(instance: u32, value: Option<Instance>);
	fn connect_object_value_changed(instance: u32, callback: Box<dyn Fn(Option<Instance>)>) -> u32;
	fn prop_ray_value_value(instance: u32) -> Ray;
	fn prop_set_ray_value_value(instance: u32, value: Ray);
	fn connect_ray_value_changed(instance: u32, callback: Box<dyn Fn(Ray)>) -> u32;
	fn prop_string_value_value(instance: u32) -> String;
	fn prop_set_string_value_value(instance: u32, value: &str);
	fn connect_string_value_changed(instance: u32, callback: Box<dyn Fn(String)>) -> u32;
	fn prop_vector_3_value_value(instance: u32) -> Vector3;
	fn prop_set_vector_3_value_value(instance: u32, value: Vector3);
	fn connect_vector_3_value_changed(instance: u32, callback: Box<dyn Fn(Vector3)>) -> u32;
	fn dyn_vector_3_curve_get_value_at_time(instance: u32, p_time: f64);
	fn dyn_vector_3_curve_x(instance: u32) -> Option<FloatCurve>;
	fn dyn_vector_3_curve_y(instance: u32) -> Option<FloatCurve>;
	fn dyn_vector_3_curve_z(instance: u32) -> Option<FloatCurve>;
	fn dyn_voice_chat_internal_get_audio_processing_settings(instance: u32);
	fn dyn_voice_chat_internal_get_mic_devices(instance: u32);
	fn dyn_voice_chat_internal_get_participants(instance: u32);
	fn dyn_voice_chat_internal_get_speaker_devices(instance: u32);
	fn dyn_voice_chat_internal_get_voice_chat_api_version(instance: u32) -> f64;
	fn dyn_voice_chat_internal_get_voice_chat_available(instance: u32) -> f64;
	fn dyn_voice_chat_internal_is_publish_paused(instance: u32) -> bool;
	fn dyn_voice_chat_internal_is_subscribe_paused(instance: u32, p_userId: f64) -> bool;
	fn dyn_voice_chat_internal_join_by_group_id(instance: u32, p_groupId: &str, p_isMicMuted: bool) -> bool;
	fn dyn_voice_chat_internal_join_by_group_id_token(instance: u32, p_groupId: &str, p_isMicMuted: bool) -> bool;
	fn dyn_voice_chat_internal_leave(instance: u32);
	fn dyn_voice_chat_internal_publish_pause(instance: u32, p_paused: bool) -> bool;
	fn dyn_voice_chat_internal_set_mic_device(instance: u32, p_micDeviceName: &str, p_micDeviceGuid: &str);
	fn dyn_voice_chat_internal_set_speaker_device(instance: u32, p_speakerDeviceName: &str, p_speakerDeviceGuid: &str);
	fn dyn_voice_chat_internal_subscribe_pause(instance: u32, p_userId: f64, p_paused: bool) -> bool;
	fn dyn_voice_chat_internal_subscribe_pause_all(instance: u32, p_paused: bool) -> bool;
	fn dyn_voice_chat_internal_is_voice_enabled_for_user_id_async(instance: u32, p_userId: f64) -> bool;
	fn dyn_voice_chat_service_is_voice_enabled_for_user_id_async(instance: u32, p_userId: f64) -> bool;
	fn prop_voice_source_user_id(instance: u32) -> f64;
	fn prop_weld_constraint_active(instance: u32) -> bool;
	fn prop_weld_constraint_enabled(instance: u32) -> bool;
	fn prop_set_weld_constraint_enabled(instance: u32, value: bool);
	fn prop_weld_constraint_part_0(instance: u32) -> Option<BasePart>;
	fn prop_set_weld_constraint_part_0(instance: u32, value: Option<BasePart>);
	fn prop_weld_constraint_part_1(instance: u32) -> Option<BasePart>;
	fn prop_set_weld_constraint_part_1(instance: u32, value: Option<BasePart>);
}

#[allow(unused_imports)]
use super::*;
pub trait RobloxCreatable {
	fn new() -> Self;
}
macro_rules! creatable {
	($($name:ident)*) => {
		$(
			impl RobloxCreatable for $name {
				fn new() -> $name {
					unsafe { Self(instance_new(stringify!($name))) }
				}
			}
		)*
	}
}
macro_rules! impl_instance_exclusive {
	($name:ident) => {
		impl_instance!($name);
		impl std::convert::TryFrom<Instance> for $name {
			type Error = ();
			fn try_from(value: Instance) -> Result<Self, Self::Error> {
				if value.fn_is_a(stringify!($name)) {
					Ok($name(value.to_ptr()))
				} else {
					Err(())
				}
			}
		}
	}
}
macro_rules! impl_instance {
	($name:ident) => {
		#[allow(dead_code)]
		#[repr(transparent)]
		pub struct $name(u32);
		impl $name {
			#[allow(clippy::wrong_self_convention)]
			pub(crate) fn to_ptr(&self) -> u32 { self.0 }
			pub fn get_child(&self, name: &str) -> Instance {
				match self.fn_find_first_child(name, false) {
					None => panic!("No child by the name of '{}' was found", name),
					Some(instance) => instance,
				}
			}
			pub fn downcast<I: From<$name>>(&self) -> I {
				self.clone().into()
			}
		}
		impl Clone for $name {
			fn clone(&self) -> Self {
				unsafe { Self(clone_pointer(self.to_ptr())) }
			}
		}
		impl $name {
			pub fn archivable(&self) -> bool {
				unsafe { prop_instance_archivable(self.to_ptr()) }
			}
			pub fn set_archivable(&self, value: bool) {
				unsafe { prop_set_instance_archivable(self.to_ptr(), value) }
			}
			pub fn class_name(&self) -> String {
				unsafe { prop_instance_class_name(self.to_ptr()) }
			}
			pub fn name(&self) -> String {
				unsafe { prop_instance_name(self.to_ptr()) }
			}
			pub fn set_name(&self, value: &str) {
				unsafe { prop_set_instance_name(self.to_ptr(), value) }
			}
			pub fn parent(&self) -> Option<Instance> {
				unsafe { prop_instance_parent(self.to_ptr()) }
			}
			pub fn set_parent(&self, value: Option<Instance>) {
				unsafe { prop_set_instance_parent(self.to_ptr(), value) }
			}
			pub fn fn_clear_all_children(&self) {
				unsafe { dyn_instance_clear_all_children(self.to_ptr()) }
			}
			pub fn fn_clone(&self) -> Option<Instance> {
				unsafe { dyn_instance_clone(self.to_ptr()) }
			}
			pub fn fn_destroy(&self) {
				unsafe { dyn_instance_destroy(self.to_ptr()) }
			}
			pub fn fn_find_first_ancestor(&self, name: &str) -> Option<Instance> {
				unsafe { dyn_instance_find_first_ancestor(self.to_ptr(), name) }
			}
			pub fn fn_find_first_ancestor_of_class(&self, class_name: &str) -> Option<Instance> {
				unsafe { dyn_instance_find_first_ancestor_of_class(self.to_ptr(), class_name) }
			}
			pub fn fn_find_first_ancestor_which_is_a(&self, class_name: &str) -> Option<Instance> {
				unsafe { dyn_instance_find_first_ancestor_which_is_a(self.to_ptr(), class_name) }
			}
			pub fn fn_find_first_child(&self, name: &str, recursive: bool) -> Option<Instance> {
				unsafe { dyn_instance_find_first_child(self.to_ptr(), name, recursive) }
			}
			pub fn fn_find_first_child_of_class(&self, class_name: &str) -> Option<Instance> {
				unsafe { dyn_instance_find_first_child_of_class(self.to_ptr(), class_name) }
			}
			pub fn fn_find_first_child_which_is_a(&self, class_name: &str, recursive: bool) -> Option<Instance> {
				unsafe { dyn_instance_find_first_child_which_is_a(self.to_ptr(), class_name, recursive) }
			}
			pub fn fn_find_first_descendant(&self, name: &str) -> Option<Instance> {
				unsafe { dyn_instance_find_first_descendant(self.to_ptr(), name) }
			}
			pub fn fn_get_actor(&self) -> Option<Actor> {
				unsafe { dyn_instance_get_actor(self.to_ptr()) }
			}
			pub fn fn_get_attribute(&self, attribute: &str) {
				unsafe { dyn_instance_get_attribute(self.to_ptr(), attribute) }
			}
			pub fn fn_get_attribute_changed_signal(&self, attribute: &str) -> RbxScriptSignal {
				unsafe { dyn_instance_get_attribute_changed_signal(self.to_ptr(), attribute) }
			}
			pub fn fn_get_attributes(&self) {
				unsafe { dyn_instance_get_attributes(self.to_ptr()) }
			}
			pub fn fn_get_children(&self) -> Objects {
				unsafe { dyn_instance_get_children(self.to_ptr()) }
			}
			pub fn fn_get_descendants(&self) {
				unsafe { dyn_instance_get_descendants(self.to_ptr()) }
			}
			pub fn fn_get_full_name(&self) -> String {
				unsafe { dyn_instance_get_full_name(self.to_ptr()) }
			}
			pub fn fn_get_property_changed_signal(&self, property: &str) -> RbxScriptSignal {
				unsafe { dyn_instance_get_property_changed_signal(self.to_ptr(), property) }
			}
			pub fn fn_is_a(&self, class_name: &str) -> bool {
				unsafe { dyn_instance_is_a(self.to_ptr(), class_name) }
			}
			pub fn fn_is_ancestor_of(&self, descendant: Option<Instance>) -> bool {
				unsafe { dyn_instance_is_ancestor_of(self.to_ptr(), descendant) }
			}
			pub fn fn_is_descendant_of(&self, ancestor: Option<Instance>) -> bool {
				unsafe { dyn_instance_is_descendant_of(self.to_ptr(), ancestor) }
			}
			pub fn fn_remove(&self) {
				unsafe { dyn_instance_remove(self.to_ptr()) }
			}
			pub fn fn_wait_for_child(&self, child_name: &str, time_out: f64) -> Option<Instance> {
				unsafe { dyn_instance_wait_for_child(self.to_ptr(), child_name, time_out) }
			}
			pub fn on_ancestry_changed<F: 'static + Fn(Option<Instance>, Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_instance_ancestry_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_attribute_changed<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_instance_attribute_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_instance_changed<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_instance_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_child_added<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_instance_child_added(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_child_removed<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_instance_child_removed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_descendant_added<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_instance_descendant_added(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_descendant_removing<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_instance_descendant_removing(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_destroying<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_instance_destroying(self.to_ptr(), Box::new(callback))) }
			}
		}
	}
}
macro_rules! impl_accoutrement {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn attachment_forward(&self) -> Vector3 {
				unsafe { prop_accoutrement_attachment_forward(self.to_ptr()) }
			}
			pub fn set_attachment_forward(&self, value: Vector3) {
				unsafe { prop_set_accoutrement_attachment_forward(self.to_ptr(), value) }
			}
			pub fn attachment_point(&self) -> CFrame {
				unsafe { prop_accoutrement_attachment_point(self.to_ptr()) }
			}
			pub fn set_attachment_point(&self, value: CFrame) {
				unsafe { prop_set_accoutrement_attachment_point(self.to_ptr(), value) }
			}
			pub fn attachment_pos(&self) -> Vector3 {
				unsafe { prop_accoutrement_attachment_pos(self.to_ptr()) }
			}
			pub fn set_attachment_pos(&self, value: Vector3) {
				unsafe { prop_set_accoutrement_attachment_pos(self.to_ptr(), value) }
			}
			pub fn attachment_right(&self) -> Vector3 {
				unsafe { prop_accoutrement_attachment_right(self.to_ptr()) }
			}
			pub fn set_attachment_right(&self, value: Vector3) {
				unsafe { prop_set_accoutrement_attachment_right(self.to_ptr(), value) }
			}
			pub fn attachment_up(&self) -> Vector3 {
				unsafe { prop_accoutrement_attachment_up(self.to_ptr()) }
			}
			pub fn set_attachment_up(&self, value: Vector3) {
				unsafe { prop_set_accoutrement_attachment_up(self.to_ptr(), value) }
			}
		}
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
macro_rules! impl_analytics_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn animation_id(&self) -> Content {
				unsafe { prop_animation_animation_id(self.to_ptr()) }
			}
			pub fn set_animation_id(&self, value: Content) {
				unsafe { prop_set_animation_animation_id(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation_clip {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn r#loop(&self) -> bool {
				unsafe { prop_animation_clip_loop(self.to_ptr()) }
			}
			pub fn set_loop(&self, value: bool) {
				unsafe { prop_set_animation_clip_loop(self.to_ptr(), value) }
			}
		}
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
				unsafe { dyn_keyframe_sequence_add_keyframe(self.to_ptr(), keyframe) }
			}
			pub fn fn_get_keyframes(&self) -> Objects {
				unsafe { dyn_keyframe_sequence_get_keyframes(self.to_ptr()) }
			}
			pub fn fn_remove_keyframe(&self, keyframe: Option<Instance>) {
				unsafe { dyn_keyframe_sequence_remove_keyframe(self.to_ptr(), keyframe) }
			}
		}
		impl From<$name> for AnimationClip {
			fn from(value: $name) -> AnimationClip {
				AnimationClip(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation_clip_provider {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_register_active_animation_clip(&self, animation_clip: Option<AnimationClip>) -> Content {
				unsafe { dyn_animation_clip_provider_register_active_animation_clip(self.to_ptr(), animation_clip) }
			}
			pub fn fn_register_animation_clip(&self, animation_clip: Option<AnimationClip>) -> Content {
				unsafe { dyn_animation_clip_provider_register_animation_clip(self.to_ptr(), animation_clip) }
			}
			pub fn fn_get_animation_clip_async(&self, asset_id: Content) -> Option<AnimationClip> {
				unsafe { dyn_animation_clip_provider_get_animation_clip_async(self.to_ptr(), asset_id) }
			}
			pub fn fn_get_animations(&self, user_id: f64) -> Option<Instance> {
				unsafe { dyn_animation_clip_provider_get_animations(self.to_ptr(), user_id) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation_controller {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_playing_animation_tracks(&self) {
				unsafe { dyn_animation_controller_get_playing_animation_tracks(self.to_ptr()) }
			}
			pub fn fn_load_animation(&self, animation: Option<Animation>) -> Option<AnimationTrack> {
				unsafe { dyn_animation_controller_load_animation(self.to_ptr(), animation) }
			}
			pub fn on_animation_played<F: 'static + Fn(Option<AnimationTrack>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_animation_controller_animation_played(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation_from_video_creator_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation_from_video_creator_studio_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation_rig_data {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation_stream_track {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn is_playing(&self) -> bool {
				unsafe { prop_animation_stream_track_is_playing(self.to_ptr()) }
			}
			pub fn weight_current(&self) -> f64 {
				unsafe { prop_animation_stream_track_weight_current(self.to_ptr()) }
			}
			pub fn weight_target(&self) -> f64 {
				unsafe { prop_animation_stream_track_weight_target(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animation_track {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn animation(&self) -> Option<Animation> {
				unsafe { prop_animation_track_animation(self.to_ptr()) }
			}
			pub fn is_playing(&self) -> bool {
				unsafe { prop_animation_track_is_playing(self.to_ptr()) }
			}
			pub fn length(&self) -> f64 {
				unsafe { prop_animation_track_length(self.to_ptr()) }
			}
			pub fn looped(&self) -> bool {
				unsafe { prop_animation_track_looped(self.to_ptr()) }
			}
			pub fn set_looped(&self, value: bool) {
				unsafe { prop_set_animation_track_looped(self.to_ptr(), value) }
			}
			pub fn speed(&self) -> f64 {
				unsafe { prop_animation_track_speed(self.to_ptr()) }
			}
			pub fn time_position(&self) -> f64 {
				unsafe { prop_animation_track_time_position(self.to_ptr()) }
			}
			pub fn set_time_position(&self, value: f64) {
				unsafe { prop_set_animation_track_time_position(self.to_ptr(), value) }
			}
			pub fn weight_current(&self) -> f64 {
				unsafe { prop_animation_track_weight_current(self.to_ptr()) }
			}
			pub fn weight_target(&self) -> f64 {
				unsafe { prop_animation_track_weight_target(self.to_ptr()) }
			}
			pub fn fn_adjust_speed(&self, speed: f64) {
				unsafe { dyn_animation_track_adjust_speed(self.to_ptr(), speed) }
			}
			pub fn fn_adjust_weight(&self, weight: f64, fade_time: f64) {
				unsafe { dyn_animation_track_adjust_weight(self.to_ptr(), weight, fade_time) }
			}
			pub fn fn_get_marker_reached_signal(&self, name: &str) -> RbxScriptSignal {
				unsafe { dyn_animation_track_get_marker_reached_signal(self.to_ptr(), name) }
			}
			pub fn fn_get_time_of_keyframe(&self, keyframe_name: &str) -> f64 {
				unsafe { dyn_animation_track_get_time_of_keyframe(self.to_ptr(), keyframe_name) }
			}
			pub fn fn_play(&self, fade_time: f64, weight: f64, speed: f64) {
				unsafe { dyn_animation_track_play(self.to_ptr(), fade_time, weight, speed) }
			}
			pub fn fn_stop(&self, fade_time: f64) {
				unsafe { dyn_animation_track_stop(self.to_ptr(), fade_time) }
			}
			pub fn on_did_loop<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_animation_track_did_loop(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_keyframe_reached<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_animation_track_keyframe_reached(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_stopped<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_animation_track_stopped(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_animator {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_playing_animation_tracks(&self) {
				unsafe { dyn_animator_get_playing_animation_tracks(self.to_ptr()) }
			}
			pub fn fn_load_animation(&self, animation: Option<Animation>) -> Option<AnimationTrack> {
				unsafe { dyn_animator_load_animation(self.to_ptr(), animation) }
			}
			pub fn on_animation_played<F: 'static + Fn(Option<AnimationTrack>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_animator_animation_played(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_app_update_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_asset_counter_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_asset_delivery_proxy {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_asset_import_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn on_progress_update<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_asset_import_service_progress_update(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_upload_finished<F: 'static + Fn(bool, ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_asset_import_service_upload_finished(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_asset_manager_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_asset_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_create_place_async(&self, place_name: &str, template_place_id: f64, description: &str) -> f64 {
				unsafe { dyn_asset_service_create_place_async(self.to_ptr(), place_name, template_place_id, description) }
			}
			pub fn fn_create_place_in_player_inventory_async(&self, player: Option<Instance>, place_name: &str, template_place_id: f64, description: &str) -> f64 {
				unsafe { dyn_asset_service_create_place_in_player_inventory_async(self.to_ptr(), player, place_name, template_place_id, description) }
			}
			pub fn fn_get_asset_ids_for_package(&self, package_asset_id: f64) {
				unsafe { dyn_asset_service_get_asset_ids_for_package(self.to_ptr(), package_asset_id) }
			}
			pub fn fn_get_bundle_details_async(&self, bundle_id: f64) {
				unsafe { dyn_asset_service_get_bundle_details_async(self.to_ptr(), bundle_id) }
			}
			pub fn fn_get_creator_asset_id(&self, creation_id: f64) -> f64 {
				unsafe { dyn_asset_service_get_creator_asset_id(self.to_ptr(), creation_id) }
			}
			pub fn fn_get_game_places_async(&self) -> Option<Instance> {
				unsafe { dyn_asset_service_get_game_places_async(self.to_ptr()) }
			}
			pub fn fn_save_place_async(&self) {
				unsafe { dyn_asset_service_save_place_async(self.to_ptr()) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_atmosphere {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn color(&self) -> Color3 {
				unsafe { prop_atmosphere_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { prop_set_atmosphere_color(self.to_ptr(), value) }
			}
			pub fn decay(&self) -> Color3 {
				unsafe { prop_atmosphere_decay(self.to_ptr()) }
			}
			pub fn set_decay(&self, value: Color3) {
				unsafe { prop_set_atmosphere_decay(self.to_ptr(), value) }
			}
			pub fn density(&self) -> f64 {
				unsafe { prop_atmosphere_density(self.to_ptr()) }
			}
			pub fn set_density(&self, value: f64) {
				unsafe { prop_set_atmosphere_density(self.to_ptr(), value) }
			}
			pub fn glare(&self) -> f64 {
				unsafe { prop_atmosphere_glare(self.to_ptr()) }
			}
			pub fn set_glare(&self, value: f64) {
				unsafe { prop_set_atmosphere_glare(self.to_ptr(), value) }
			}
			pub fn haze(&self) -> f64 {
				unsafe { prop_atmosphere_haze(self.to_ptr()) }
			}
			pub fn set_haze(&self, value: f64) {
				unsafe { prop_set_atmosphere_haze(self.to_ptr(), value) }
			}
			pub fn offset(&self) -> f64 {
				unsafe { prop_atmosphere_offset(self.to_ptr()) }
			}
			pub fn set_offset(&self, value: f64) {
				unsafe { prop_set_atmosphere_offset(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_attachment {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn axis(&self) -> Vector3 {
				unsafe { prop_attachment_axis(self.to_ptr()) }
			}
			pub fn set_axis(&self, value: Vector3) {
				unsafe { prop_set_attachment_axis(self.to_ptr(), value) }
			}
			pub fn c_frame(&self) -> CFrame {
				unsafe { prop_attachment_c_frame(self.to_ptr()) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { prop_set_attachment_c_frame(self.to_ptr(), value) }
			}
			pub fn orientation(&self) -> Vector3 {
				unsafe { prop_attachment_orientation(self.to_ptr()) }
			}
			pub fn set_orientation(&self, value: Vector3) {
				unsafe { prop_set_attachment_orientation(self.to_ptr(), value) }
			}
			pub fn position(&self) -> Vector3 {
				unsafe { prop_attachment_position(self.to_ptr()) }
			}
			pub fn set_position(&self, value: Vector3) {
				unsafe { prop_set_attachment_position(self.to_ptr(), value) }
			}
			pub fn rotation(&self) -> Vector3 {
				unsafe { prop_attachment_rotation(self.to_ptr()) }
			}
			pub fn set_rotation(&self, value: Vector3) {
				unsafe { prop_set_attachment_rotation(self.to_ptr(), value) }
			}
			pub fn secondary_axis(&self) -> Vector3 {
				unsafe { prop_attachment_secondary_axis(self.to_ptr()) }
			}
			pub fn set_secondary_axis(&self, value: Vector3) {
				unsafe { prop_set_attachment_secondary_axis(self.to_ptr(), value) }
			}
			pub fn visible(&self) -> bool {
				unsafe { prop_attachment_visible(self.to_ptr()) }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { prop_set_attachment_visible(self.to_ptr(), value) }
			}
			pub fn world_axis(&self) -> Vector3 {
				unsafe { prop_attachment_world_axis(self.to_ptr()) }
			}
			pub fn set_world_axis(&self, value: Vector3) {
				unsafe { prop_set_attachment_world_axis(self.to_ptr(), value) }
			}
			pub fn world_c_frame(&self) -> CFrame {
				unsafe { prop_attachment_world_c_frame(self.to_ptr()) }
			}
			pub fn set_world_c_frame(&self, value: CFrame) {
				unsafe { prop_set_attachment_world_c_frame(self.to_ptr(), value) }
			}
			pub fn world_orientation(&self) -> Vector3 {
				unsafe { prop_attachment_world_orientation(self.to_ptr()) }
			}
			pub fn set_world_orientation(&self, value: Vector3) {
				unsafe { prop_set_attachment_world_orientation(self.to_ptr(), value) }
			}
			pub fn world_position(&self) -> Vector3 {
				unsafe { prop_attachment_world_position(self.to_ptr()) }
			}
			pub fn set_world_position(&self, value: Vector3) {
				unsafe { prop_set_attachment_world_position(self.to_ptr(), value) }
			}
			pub fn world_rotation(&self) -> Vector3 {
				unsafe { prop_attachment_world_rotation(self.to_ptr()) }
			}
			pub fn world_secondary_axis(&self) -> Vector3 {
				unsafe { prop_attachment_world_secondary_axis(self.to_ptr()) }
			}
			pub fn set_world_secondary_axis(&self, value: Vector3) {
				unsafe { prop_set_attachment_world_secondary_axis(self.to_ptr(), value) }
			}
			pub fn fn_get_axis(&self) -> Vector3 {
				unsafe { dyn_attachment_get_axis(self.to_ptr()) }
			}
			pub fn fn_get_secondary_axis(&self) -> Vector3 {
				unsafe { dyn_attachment_get_secondary_axis(self.to_ptr()) }
			}
			pub fn fn_set_axis(&self, axis: Vector3) {
				unsafe { dyn_attachment_set_axis(self.to_ptr(), axis) }
			}
			pub fn fn_set_secondary_axis(&self, axis: Vector3) {
				unsafe { dyn_attachment_set_secondary_axis(self.to_ptr(), axis) }
			}
		}
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
				unsafe { prop_bone_transform(self.to_ptr()) }
			}
			pub fn set_transform(&self, value: CFrame) {
				unsafe { prop_set_bone_transform(self.to_ptr(), value) }
			}
			pub fn transformed_c_frame(&self) -> CFrame {
				unsafe { prop_bone_transformed_c_frame(self.to_ptr()) }
			}
			pub fn transformed_world_c_frame(&self) -> CFrame {
				unsafe { prop_bone_transformed_world_c_frame(self.to_ptr()) }
			}
		}
		impl From<$name> for Attachment {
			fn from(value: $name) -> Attachment {
				Attachment(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_avatar_editor_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_prompt_allow_inventory_read_access(&self) {
				unsafe { dyn_avatar_editor_service_prompt_allow_inventory_read_access(self.to_ptr()) }
			}
			pub fn fn_prompt_delete_outfit(&self, outfit_id: f64) {
				unsafe { dyn_avatar_editor_service_prompt_delete_outfit(self.to_ptr(), outfit_id) }
			}
			pub fn fn_prompt_rename_outfit(&self, outfit_id: f64) {
				unsafe { dyn_avatar_editor_service_prompt_rename_outfit(self.to_ptr(), outfit_id) }
			}
			pub fn fn_check_apply_default_clothing(&self, humanoid_description: Option<HumanoidDescription>) -> Option<HumanoidDescription> {
				unsafe { dyn_avatar_editor_service_check_apply_default_clothing(self.to_ptr(), humanoid_description) }
			}
			pub fn fn_get_avatar_rules(&self) {
				unsafe { dyn_avatar_editor_service_get_avatar_rules(self.to_ptr()) }
			}
			pub fn fn_get_recommended_bundles(&self, bundle_id: f64) {
				unsafe { dyn_avatar_editor_service_get_recommended_bundles(self.to_ptr(), bundle_id) }
			}
			pub fn fn_search_catalog(&self, search_parameters: CatalogSearchParams) -> Option<CatalogPages> {
				unsafe { dyn_avatar_editor_service_search_catalog(self.to_ptr(), search_parameters) }
			}
			pub fn on_prompt_allow_inventory_read_access_completed<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_avatar_editor_service_prompt_allow_inventory_read_access_completed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_create_outfit_completed<F: 'static + Fn((), ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_avatar_editor_service_prompt_create_outfit_completed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_delete_outfit_completed<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_avatar_editor_service_prompt_delete_outfit_completed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_rename_outfit_completed<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_avatar_editor_service_prompt_rename_outfit_completed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_save_avatar_completed<F: 'static + Fn((), Option<HumanoidDescription>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_avatar_editor_service_prompt_save_avatar_completed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_set_favorite_completed<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_avatar_editor_service_prompt_set_favorite_completed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_update_outfit_completed<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_avatar_editor_service_prompt_update_outfit_completed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_avatar_import_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_backpack {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_backpack_item {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn texture_id(&self) -> Content {
				unsafe { prop_backpack_item_texture_id(self.to_ptr()) }
			}
			pub fn set_texture_id(&self, value: Content) {
				unsafe { prop_set_backpack_item_texture_id(self.to_ptr(), value) }
			}
		}
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
				unsafe { prop_tool_can_be_dropped(self.to_ptr()) }
			}
			pub fn set_can_be_dropped(&self, value: bool) {
				unsafe { prop_set_tool_can_be_dropped(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_tool_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_tool_enabled(self.to_ptr(), value) }
			}
			pub fn grip(&self) -> CFrame {
				unsafe { prop_tool_grip(self.to_ptr()) }
			}
			pub fn set_grip(&self, value: CFrame) {
				unsafe { prop_set_tool_grip(self.to_ptr(), value) }
			}
			pub fn grip_forward(&self) -> Vector3 {
				unsafe { prop_tool_grip_forward(self.to_ptr()) }
			}
			pub fn set_grip_forward(&self, value: Vector3) {
				unsafe { prop_set_tool_grip_forward(self.to_ptr(), value) }
			}
			pub fn grip_pos(&self) -> Vector3 {
				unsafe { prop_tool_grip_pos(self.to_ptr()) }
			}
			pub fn set_grip_pos(&self, value: Vector3) {
				unsafe { prop_set_tool_grip_pos(self.to_ptr(), value) }
			}
			pub fn grip_right(&self) -> Vector3 {
				unsafe { prop_tool_grip_right(self.to_ptr()) }
			}
			pub fn set_grip_right(&self, value: Vector3) {
				unsafe { prop_set_tool_grip_right(self.to_ptr(), value) }
			}
			pub fn grip_up(&self) -> Vector3 {
				unsafe { prop_tool_grip_up(self.to_ptr()) }
			}
			pub fn set_grip_up(&self, value: Vector3) {
				unsafe { prop_set_tool_grip_up(self.to_ptr(), value) }
			}
			pub fn manual_activation_only(&self) -> bool {
				unsafe { prop_tool_manual_activation_only(self.to_ptr()) }
			}
			pub fn set_manual_activation_only(&self, value: bool) {
				unsafe { prop_set_tool_manual_activation_only(self.to_ptr(), value) }
			}
			pub fn requires_handle(&self) -> bool {
				unsafe { prop_tool_requires_handle(self.to_ptr()) }
			}
			pub fn set_requires_handle(&self, value: bool) {
				unsafe { prop_set_tool_requires_handle(self.to_ptr(), value) }
			}
			pub fn tool_tip(&self) -> String {
				unsafe { prop_tool_tool_tip(self.to_ptr()) }
			}
			pub fn set_tool_tip(&self, value: &str) {
				unsafe { prop_set_tool_tool_tip(self.to_ptr(), value) }
			}
			pub fn fn_activate(&self) {
				unsafe { dyn_tool_activate(self.to_ptr()) }
			}
			pub fn fn_deactivate(&self) {
				unsafe { dyn_tool_deactivate(self.to_ptr()) }
			}
			pub fn on_activated<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_tool_activated(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_deactivated<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_tool_deactivated(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_equipped<F: 'static + Fn(Option<Mouse>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_tool_equipped(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_unequipped<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_tool_unequipped(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for BackpackItem {
			fn from(value: $name) -> BackpackItem {
				BackpackItem(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_badge_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_award_badge(&self, user_id: f64, badge_id: f64) -> bool {
				unsafe { dyn_badge_service_award_badge(self.to_ptr(), user_id, badge_id) }
			}
			pub fn fn_get_badge_info_async(&self, badge_id: f64) {
				unsafe { dyn_badge_service_get_badge_info_async(self.to_ptr(), badge_id) }
			}
			pub fn fn_is_disabled(&self, badge_id: f64) -> bool {
				unsafe { dyn_badge_service_is_disabled(self.to_ptr(), badge_id) }
			}
			pub fn fn_is_legal(&self, badge_id: f64) -> bool {
				unsafe { dyn_badge_service_is_legal(self.to_ptr(), badge_id) }
			}
			pub fn fn_user_has_badge(&self, user_id: f64, badge_id: f64) -> bool {
				unsafe { dyn_badge_service_user_has_badge(self.to_ptr(), user_id, badge_id) }
			}
			pub fn fn_user_has_badge_async(&self, user_id: f64, badge_id: f64) -> bool {
				unsafe { dyn_badge_service_user_has_badge_async(self.to_ptr(), user_id, badge_id) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_base_player_gui {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_gui_objects_at_position(&self, x: f64, y: f64) -> Objects {
				unsafe { dyn_base_player_gui_get_gui_objects_at_position(self.to_ptr(), x, y) }
			}
		}
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
				unsafe { prop_player_gui_selection_image_object(self.to_ptr()) }
			}
			pub fn set_selection_image_object(&self, value: Option<GuiObject>) {
				unsafe { prop_set_player_gui_selection_image_object(self.to_ptr(), value) }
			}
			pub fn fn_get_topbar_transparency(&self) -> f64 {
				unsafe { dyn_player_gui_get_topbar_transparency(self.to_ptr()) }
			}
			pub fn fn_set_topbar_transparency(&self, transparency: f64) {
				unsafe { dyn_player_gui_set_topbar_transparency(self.to_ptr(), transparency) }
			}
			pub fn on_topbar_transparency_changed_signal<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_player_gui_topbar_transparency_changed_signal(self.to_ptr(), Box::new(callback))) }
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
			pub fn reset_player_gui_on_spawn(&self) -> bool {
				unsafe { prop_starter_gui_reset_player_gui_on_spawn(self.to_ptr()) }
			}
			pub fn set_reset_player_gui_on_spawn(&self, value: bool) {
				unsafe { prop_set_starter_gui_reset_player_gui_on_spawn(self.to_ptr(), value) }
			}
			pub fn show_development_gui(&self) -> bool {
				unsafe { prop_starter_gui_show_development_gui(self.to_ptr()) }
			}
			pub fn set_show_development_gui(&self, value: bool) {
				unsafe { prop_set_starter_gui_show_development_gui(self.to_ptr(), value) }
			}
			pub fn fn_get_core(&self, parameter_name: &str) {
				unsafe { dyn_starter_gui_get_core(self.to_ptr(), parameter_name) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn cage_mesh_id(&self) -> Content {
				unsafe { prop_base_wrap_cage_mesh_id(self.to_ptr()) }
			}
			pub fn cage_origin(&self) -> CFrame {
				unsafe { prop_base_wrap_cage_origin(self.to_ptr()) }
			}
			pub fn cage_origin_world(&self) -> CFrame {
				unsafe { prop_base_wrap_cage_origin_world(self.to_ptr()) }
			}
			pub fn import_origin(&self) -> CFrame {
				unsafe { prop_base_wrap_import_origin(self.to_ptr()) }
			}
			pub fn import_origin_world(&self) -> CFrame {
				unsafe { prop_base_wrap_import_origin_world(self.to_ptr()) }
			}
		}
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
				unsafe { prop_wrap_layer_bind_offset(self.to_ptr()) }
			}
			pub fn color(&self) -> Color3 {
				unsafe { prop_wrap_layer_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { prop_set_wrap_layer_color(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_wrap_layer_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_wrap_layer_enabled(self.to_ptr(), value) }
			}
			pub fn order(&self) -> f64 {
				unsafe { prop_wrap_layer_order(self.to_ptr()) }
			}
			pub fn set_order(&self, value: f64) {
				unsafe { prop_set_wrap_layer_order(self.to_ptr(), value) }
			}
			pub fn puffiness(&self) -> f64 {
				unsafe { prop_wrap_layer_puffiness(self.to_ptr()) }
			}
			pub fn set_puffiness(&self, value: f64) {
				unsafe { prop_set_wrap_layer_puffiness(self.to_ptr(), value) }
			}
			pub fn reference_mesh_id(&self) -> Content {
				unsafe { prop_wrap_layer_reference_mesh_id(self.to_ptr()) }
			}
			pub fn reference_origin(&self) -> CFrame {
				unsafe { prop_wrap_layer_reference_origin(self.to_ptr()) }
			}
			pub fn reference_origin_world(&self) -> CFrame {
				unsafe { prop_wrap_layer_reference_origin_world(self.to_ptr()) }
			}
			pub fn shrink_factor(&self) -> f64 {
				unsafe { prop_wrap_layer_shrink_factor(self.to_ptr()) }
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
			pub fn color(&self) -> Color3 {
				unsafe { prop_wrap_target_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { prop_set_wrap_target_color(self.to_ptr(), value) }
			}
			pub fn stiffness(&self) -> f64 {
				unsafe { prop_wrap_target_stiffness(self.to_ptr()) }
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn attachment_0(&self) -> Option<Attachment> {
				unsafe { prop_beam_attachment_0(self.to_ptr()) }
			}
			pub fn set_attachment_0(&self, value: Option<Attachment>) {
				unsafe { prop_set_beam_attachment_0(self.to_ptr(), value) }
			}
			pub fn attachment_1(&self) -> Option<Attachment> {
				unsafe { prop_beam_attachment_1(self.to_ptr()) }
			}
			pub fn set_attachment_1(&self, value: Option<Attachment>) {
				unsafe { prop_set_beam_attachment_1(self.to_ptr(), value) }
			}
			pub fn brightness(&self) -> f64 {
				unsafe { prop_beam_brightness(self.to_ptr()) }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { prop_set_beam_brightness(self.to_ptr(), value) }
			}
			pub fn color(&self) -> ColorSequence {
				unsafe { prop_beam_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: ColorSequence) {
				unsafe { prop_set_beam_color(self.to_ptr(), value) }
			}
			pub fn curve_size_0(&self) -> f64 {
				unsafe { prop_beam_curve_size_0(self.to_ptr()) }
			}
			pub fn set_curve_size_0(&self, value: f64) {
				unsafe { prop_set_beam_curve_size_0(self.to_ptr(), value) }
			}
			pub fn curve_size_1(&self) -> f64 {
				unsafe { prop_beam_curve_size_1(self.to_ptr()) }
			}
			pub fn set_curve_size_1(&self, value: f64) {
				unsafe { prop_set_beam_curve_size_1(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_beam_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_beam_enabled(self.to_ptr(), value) }
			}
			pub fn face_camera(&self) -> bool {
				unsafe { prop_beam_face_camera(self.to_ptr()) }
			}
			pub fn set_face_camera(&self, value: bool) {
				unsafe { prop_set_beam_face_camera(self.to_ptr(), value) }
			}
			pub fn light_emission(&self) -> f64 {
				unsafe { prop_beam_light_emission(self.to_ptr()) }
			}
			pub fn set_light_emission(&self, value: f64) {
				unsafe { prop_set_beam_light_emission(self.to_ptr(), value) }
			}
			pub fn light_influence(&self) -> f64 {
				unsafe { prop_beam_light_influence(self.to_ptr()) }
			}
			pub fn set_light_influence(&self, value: f64) {
				unsafe { prop_set_beam_light_influence(self.to_ptr(), value) }
			}
			pub fn segments(&self) -> f64 {
				unsafe { prop_beam_segments(self.to_ptr()) }
			}
			pub fn set_segments(&self, value: f64) {
				unsafe { prop_set_beam_segments(self.to_ptr(), value) }
			}
			pub fn texture(&self) -> Content {
				unsafe { prop_beam_texture(self.to_ptr()) }
			}
			pub fn set_texture(&self, value: Content) {
				unsafe { prop_set_beam_texture(self.to_ptr(), value) }
			}
			pub fn texture_length(&self) -> f64 {
				unsafe { prop_beam_texture_length(self.to_ptr()) }
			}
			pub fn set_texture_length(&self, value: f64) {
				unsafe { prop_set_beam_texture_length(self.to_ptr(), value) }
			}
			pub fn texture_speed(&self) -> f64 {
				unsafe { prop_beam_texture_speed(self.to_ptr()) }
			}
			pub fn set_texture_speed(&self, value: f64) {
				unsafe { prop_set_beam_texture_speed(self.to_ptr(), value) }
			}
			pub fn transparency(&self) -> NumberSequence {
				unsafe { prop_beam_transparency(self.to_ptr()) }
			}
			pub fn set_transparency(&self, value: NumberSequence) {
				unsafe { prop_set_beam_transparency(self.to_ptr(), value) }
			}
			pub fn width_0(&self) -> f64 {
				unsafe { prop_beam_width_0(self.to_ptr()) }
			}
			pub fn set_width_0(&self, value: f64) {
				unsafe { prop_set_beam_width_0(self.to_ptr(), value) }
			}
			pub fn width_1(&self) -> f64 {
				unsafe { prop_beam_width_1(self.to_ptr()) }
			}
			pub fn set_width_1(&self, value: f64) {
				unsafe { prop_set_beam_width_1(self.to_ptr(), value) }
			}
			pub fn z_offset(&self) -> f64 {
				unsafe { prop_beam_z_offset(self.to_ptr()) }
			}
			pub fn set_z_offset(&self, value: f64) {
				unsafe { prop_set_beam_z_offset(self.to_ptr(), value) }
			}
			pub fn fn_set_texture_offset(&self, offset: f64) {
				unsafe { dyn_beam_set_texture_offset(self.to_ptr(), offset) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_bindable_event {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn on_event<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_bindable_event_event(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_bindable_function {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_body_mover {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
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
				unsafe { prop_body_angular_velocity_angular_velocity(self.to_ptr()) }
			}
			pub fn set_angular_velocity(&self, value: Vector3) {
				unsafe { prop_set_body_angular_velocity_angular_velocity(self.to_ptr(), value) }
			}
			pub fn max_torque(&self) -> Vector3 {
				unsafe { prop_body_angular_velocity_max_torque(self.to_ptr()) }
			}
			pub fn set_max_torque(&self, value: Vector3) {
				unsafe { prop_set_body_angular_velocity_max_torque(self.to_ptr(), value) }
			}
			pub fn p(&self) -> f64 {
				unsafe { prop_body_angular_velocity_p(self.to_ptr()) }
			}
			pub fn set_p(&self, value: f64) {
				unsafe { prop_set_body_angular_velocity_p(self.to_ptr(), value) }
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
				unsafe { prop_body_force_force(self.to_ptr()) }
			}
			pub fn set_force(&self, value: Vector3) {
				unsafe { prop_set_body_force_force(self.to_ptr(), value) }
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
				unsafe { prop_body_gyro_c_frame(self.to_ptr()) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { prop_set_body_gyro_c_frame(self.to_ptr(), value) }
			}
			pub fn d(&self) -> f64 {
				unsafe { prop_body_gyro_d(self.to_ptr()) }
			}
			pub fn set_d(&self, value: f64) {
				unsafe { prop_set_body_gyro_d(self.to_ptr(), value) }
			}
			pub fn max_torque(&self) -> Vector3 {
				unsafe { prop_body_gyro_max_torque(self.to_ptr()) }
			}
			pub fn set_max_torque(&self, value: Vector3) {
				unsafe { prop_set_body_gyro_max_torque(self.to_ptr(), value) }
			}
			pub fn p(&self) -> f64 {
				unsafe { prop_body_gyro_p(self.to_ptr()) }
			}
			pub fn set_p(&self, value: f64) {
				unsafe { prop_set_body_gyro_p(self.to_ptr(), value) }
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
				unsafe { prop_body_position_d(self.to_ptr()) }
			}
			pub fn set_d(&self, value: f64) {
				unsafe { prop_set_body_position_d(self.to_ptr(), value) }
			}
			pub fn max_force(&self) -> Vector3 {
				unsafe { prop_body_position_max_force(self.to_ptr()) }
			}
			pub fn set_max_force(&self, value: Vector3) {
				unsafe { prop_set_body_position_max_force(self.to_ptr(), value) }
			}
			pub fn p(&self) -> f64 {
				unsafe { prop_body_position_p(self.to_ptr()) }
			}
			pub fn set_p(&self, value: f64) {
				unsafe { prop_set_body_position_p(self.to_ptr(), value) }
			}
			pub fn position(&self) -> Vector3 {
				unsafe { prop_body_position_position(self.to_ptr()) }
			}
			pub fn set_position(&self, value: Vector3) {
				unsafe { prop_set_body_position_position(self.to_ptr(), value) }
			}
			pub fn on_reached_target<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_body_position_reached_target(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_body_thrust_force(self.to_ptr()) }
			}
			pub fn set_force(&self, value: Vector3) {
				unsafe { prop_set_body_thrust_force(self.to_ptr(), value) }
			}
			pub fn location(&self) -> Vector3 {
				unsafe { prop_body_thrust_location(self.to_ptr()) }
			}
			pub fn set_location(&self, value: Vector3) {
				unsafe { prop_set_body_thrust_location(self.to_ptr(), value) }
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
				unsafe { prop_body_velocity_max_force(self.to_ptr()) }
			}
			pub fn set_max_force(&self, value: Vector3) {
				unsafe { prop_set_body_velocity_max_force(self.to_ptr(), value) }
			}
			pub fn p(&self) -> f64 {
				unsafe { prop_body_velocity_p(self.to_ptr()) }
			}
			pub fn set_p(&self, value: f64) {
				unsafe { prop_set_body_velocity_p(self.to_ptr(), value) }
			}
			pub fn velocity(&self) -> Vector3 {
				unsafe { prop_body_velocity_velocity(self.to_ptr()) }
			}
			pub fn set_velocity(&self, value: Vector3) {
				unsafe { prop_set_body_velocity_velocity(self.to_ptr(), value) }
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
				unsafe { prop_rocket_propulsion_cartoon_factor(self.to_ptr()) }
			}
			pub fn set_cartoon_factor(&self, value: f64) {
				unsafe { prop_set_rocket_propulsion_cartoon_factor(self.to_ptr(), value) }
			}
			pub fn max_speed(&self) -> f64 {
				unsafe { prop_rocket_propulsion_max_speed(self.to_ptr()) }
			}
			pub fn set_max_speed(&self, value: f64) {
				unsafe { prop_set_rocket_propulsion_max_speed(self.to_ptr(), value) }
			}
			pub fn max_thrust(&self) -> f64 {
				unsafe { prop_rocket_propulsion_max_thrust(self.to_ptr()) }
			}
			pub fn set_max_thrust(&self, value: f64) {
				unsafe { prop_set_rocket_propulsion_max_thrust(self.to_ptr(), value) }
			}
			pub fn max_torque(&self) -> Vector3 {
				unsafe { prop_rocket_propulsion_max_torque(self.to_ptr()) }
			}
			pub fn set_max_torque(&self, value: Vector3) {
				unsafe { prop_set_rocket_propulsion_max_torque(self.to_ptr(), value) }
			}
			pub fn target(&self) -> Option<BasePart> {
				unsafe { prop_rocket_propulsion_target(self.to_ptr()) }
			}
			pub fn set_target(&self, value: Option<BasePart>) {
				unsafe { prop_set_rocket_propulsion_target(self.to_ptr(), value) }
			}
			pub fn target_offset(&self) -> Vector3 {
				unsafe { prop_rocket_propulsion_target_offset(self.to_ptr()) }
			}
			pub fn set_target_offset(&self, value: Vector3) {
				unsafe { prop_set_rocket_propulsion_target_offset(self.to_ptr(), value) }
			}
			pub fn target_radius(&self) -> f64 {
				unsafe { prop_rocket_propulsion_target_radius(self.to_ptr()) }
			}
			pub fn set_target_radius(&self, value: f64) {
				unsafe { prop_set_rocket_propulsion_target_radius(self.to_ptr(), value) }
			}
			pub fn thrust_d(&self) -> f64 {
				unsafe { prop_rocket_propulsion_thrust_d(self.to_ptr()) }
			}
			pub fn set_thrust_d(&self, value: f64) {
				unsafe { prop_set_rocket_propulsion_thrust_d(self.to_ptr(), value) }
			}
			pub fn thrust_p(&self) -> f64 {
				unsafe { prop_rocket_propulsion_thrust_p(self.to_ptr()) }
			}
			pub fn set_thrust_p(&self, value: f64) {
				unsafe { prop_set_rocket_propulsion_thrust_p(self.to_ptr(), value) }
			}
			pub fn turn_d(&self) -> f64 {
				unsafe { prop_rocket_propulsion_turn_d(self.to_ptr()) }
			}
			pub fn set_turn_d(&self, value: f64) {
				unsafe { prop_set_rocket_propulsion_turn_d(self.to_ptr(), value) }
			}
			pub fn turn_p(&self) -> f64 {
				unsafe { prop_rocket_propulsion_turn_p(self.to_ptr()) }
			}
			pub fn set_turn_p(&self, value: f64) {
				unsafe { prop_set_rocket_propulsion_turn_p(self.to_ptr(), value) }
			}
			pub fn on_reached_target<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_rocket_propulsion_reached_target(self.to_ptr(), Box::new(callback))) }
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn c_frame(&self) -> CFrame {
				unsafe { prop_camera_c_frame(self.to_ptr()) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { prop_set_camera_c_frame(self.to_ptr(), value) }
			}
			pub fn camera_subject(&self) -> Option<Instance> {
				unsafe { prop_camera_camera_subject(self.to_ptr()) }
			}
			pub fn set_camera_subject(&self, value: Option<Instance>) {
				unsafe { prop_set_camera_camera_subject(self.to_ptr(), value) }
			}
			pub fn coordinate_frame(&self) -> CFrame {
				unsafe { prop_camera_coordinate_frame(self.to_ptr()) }
			}
			pub fn set_coordinate_frame(&self, value: CFrame) {
				unsafe { prop_set_camera_coordinate_frame(self.to_ptr(), value) }
			}
			pub fn diagonal_field_of_view(&self) -> f64 {
				unsafe { prop_camera_diagonal_field_of_view(self.to_ptr()) }
			}
			pub fn set_diagonal_field_of_view(&self, value: f64) {
				unsafe { prop_set_camera_diagonal_field_of_view(self.to_ptr(), value) }
			}
			pub fn field_of_view(&self) -> f64 {
				unsafe { prop_camera_field_of_view(self.to_ptr()) }
			}
			pub fn set_field_of_view(&self, value: f64) {
				unsafe { prop_set_camera_field_of_view(self.to_ptr(), value) }
			}
			pub fn focus(&self) -> CFrame {
				unsafe { prop_camera_focus(self.to_ptr()) }
			}
			pub fn set_focus(&self, value: CFrame) {
				unsafe { prop_set_camera_focus(self.to_ptr(), value) }
			}
			pub fn head_locked(&self) -> bool {
				unsafe { prop_camera_head_locked(self.to_ptr()) }
			}
			pub fn set_head_locked(&self, value: bool) {
				unsafe { prop_set_camera_head_locked(self.to_ptr(), value) }
			}
			pub fn head_scale(&self) -> f64 {
				unsafe { prop_camera_head_scale(self.to_ptr()) }
			}
			pub fn set_head_scale(&self, value: f64) {
				unsafe { prop_set_camera_head_scale(self.to_ptr(), value) }
			}
			pub fn max_axis_field_of_view(&self) -> f64 {
				unsafe { prop_camera_max_axis_field_of_view(self.to_ptr()) }
			}
			pub fn set_max_axis_field_of_view(&self, value: f64) {
				unsafe { prop_set_camera_max_axis_field_of_view(self.to_ptr(), value) }
			}
			pub fn near_plane_z(&self) -> f64 {
				unsafe { prop_camera_near_plane_z(self.to_ptr()) }
			}
			pub fn viewport_size(&self) -> Vector2 {
				unsafe { prop_camera_viewport_size(self.to_ptr()) }
			}
			pub fn fn_get_largest_cutoff_distance(&self, ignore_list: Objects) -> f64 {
				unsafe { dyn_camera_get_largest_cutoff_distance(self.to_ptr(), ignore_list) }
			}
			pub fn fn_get_pan_speed(&self) -> f64 {
				unsafe { dyn_camera_get_pan_speed(self.to_ptr()) }
			}
			pub fn fn_get_render_c_frame(&self) -> CFrame {
				unsafe { dyn_camera_get_render_c_frame(self.to_ptr()) }
			}
			pub fn fn_get_roll(&self) -> f64 {
				unsafe { dyn_camera_get_roll(self.to_ptr()) }
			}
			pub fn fn_get_tilt_speed(&self) -> f64 {
				unsafe { dyn_camera_get_tilt_speed(self.to_ptr()) }
			}
			pub fn fn_interpolate(&self, end_pos: CFrame, end_focus: CFrame, duration: f64) {
				unsafe { dyn_camera_interpolate(self.to_ptr(), end_pos, end_focus, duration) }
			}
			pub fn fn_pan_units(&self, units: f64) {
				unsafe { dyn_camera_pan_units(self.to_ptr(), units) }
			}
			pub fn fn_screen_point_to_ray(&self, x: f64, y: f64, depth: f64) -> Ray {
				unsafe { dyn_camera_screen_point_to_ray(self.to_ptr(), x, y, depth) }
			}
			pub fn fn_set_roll(&self, roll_angle: f64) {
				unsafe { dyn_camera_set_roll(self.to_ptr(), roll_angle) }
			}
			pub fn fn_tilt_units(&self, units: f64) -> bool {
				unsafe { dyn_camera_tilt_units(self.to_ptr(), units) }
			}
			pub fn fn_viewport_point_to_ray(&self, x: f64, y: f64, depth: f64) -> Ray {
				unsafe { dyn_camera_viewport_point_to_ray(self.to_ptr(), x, y, depth) }
			}
			pub fn fn_world_to_screen_point(&self, world_point: Vector3) {
				unsafe { dyn_camera_world_to_screen_point(self.to_ptr(), world_point) }
			}
			pub fn fn_world_to_viewport_point(&self, world_point: Vector3) {
				unsafe { dyn_camera_world_to_viewport_point(self.to_ptr(), world_point) }
			}
			pub fn on_interpolation_finished<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_camera_interpolation_finished(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_character_appearance {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
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
				unsafe { prop_body_colors_head_color(self.to_ptr()) }
			}
			pub fn set_head_color(&self, value: BrickColor) {
				unsafe { prop_set_body_colors_head_color(self.to_ptr(), value) }
			}
			pub fn head_color_3(&self) -> Color3 {
				unsafe { prop_body_colors_head_color_3(self.to_ptr()) }
			}
			pub fn set_head_color_3(&self, value: Color3) {
				unsafe { prop_set_body_colors_head_color_3(self.to_ptr(), value) }
			}
			pub fn left_arm_color(&self) -> BrickColor {
				unsafe { prop_body_colors_left_arm_color(self.to_ptr()) }
			}
			pub fn set_left_arm_color(&self, value: BrickColor) {
				unsafe { prop_set_body_colors_left_arm_color(self.to_ptr(), value) }
			}
			pub fn left_arm_color_3(&self) -> Color3 {
				unsafe { prop_body_colors_left_arm_color_3(self.to_ptr()) }
			}
			pub fn set_left_arm_color_3(&self, value: Color3) {
				unsafe { prop_set_body_colors_left_arm_color_3(self.to_ptr(), value) }
			}
			pub fn left_leg_color(&self) -> BrickColor {
				unsafe { prop_body_colors_left_leg_color(self.to_ptr()) }
			}
			pub fn set_left_leg_color(&self, value: BrickColor) {
				unsafe { prop_set_body_colors_left_leg_color(self.to_ptr(), value) }
			}
			pub fn left_leg_color_3(&self) -> Color3 {
				unsafe { prop_body_colors_left_leg_color_3(self.to_ptr()) }
			}
			pub fn set_left_leg_color_3(&self, value: Color3) {
				unsafe { prop_set_body_colors_left_leg_color_3(self.to_ptr(), value) }
			}
			pub fn right_arm_color(&self) -> BrickColor {
				unsafe { prop_body_colors_right_arm_color(self.to_ptr()) }
			}
			pub fn set_right_arm_color(&self, value: BrickColor) {
				unsafe { prop_set_body_colors_right_arm_color(self.to_ptr(), value) }
			}
			pub fn right_arm_color_3(&self) -> Color3 {
				unsafe { prop_body_colors_right_arm_color_3(self.to_ptr()) }
			}
			pub fn set_right_arm_color_3(&self, value: Color3) {
				unsafe { prop_set_body_colors_right_arm_color_3(self.to_ptr(), value) }
			}
			pub fn right_leg_color(&self) -> BrickColor {
				unsafe { prop_body_colors_right_leg_color(self.to_ptr()) }
			}
			pub fn set_right_leg_color(&self, value: BrickColor) {
				unsafe { prop_set_body_colors_right_leg_color(self.to_ptr(), value) }
			}
			pub fn right_leg_color_3(&self) -> Color3 {
				unsafe { prop_body_colors_right_leg_color_3(self.to_ptr()) }
			}
			pub fn set_right_leg_color_3(&self, value: Color3) {
				unsafe { prop_set_body_colors_right_leg_color_3(self.to_ptr(), value) }
			}
			pub fn torso_color(&self) -> BrickColor {
				unsafe { prop_body_colors_torso_color(self.to_ptr()) }
			}
			pub fn set_torso_color(&self, value: BrickColor) {
				unsafe { prop_set_body_colors_torso_color(self.to_ptr(), value) }
			}
			pub fn torso_color_3(&self) -> Color3 {
				unsafe { prop_body_colors_torso_color_3(self.to_ptr()) }
			}
			pub fn set_torso_color_3(&self, value: Color3) {
				unsafe { prop_set_body_colors_torso_color_3(self.to_ptr(), value) }
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
				unsafe { prop_character_mesh_base_texture_id(self.to_ptr()) }
			}
			pub fn set_base_texture_id(&self, value: f64) {
				unsafe { prop_set_character_mesh_base_texture_id(self.to_ptr(), value) }
			}
			pub fn mesh_id(&self) -> f64 {
				unsafe { prop_character_mesh_mesh_id(self.to_ptr()) }
			}
			pub fn set_mesh_id(&self, value: f64) {
				unsafe { prop_set_character_mesh_mesh_id(self.to_ptr(), value) }
			}
			pub fn overlay_texture_id(&self) -> f64 {
				unsafe { prop_character_mesh_overlay_texture_id(self.to_ptr()) }
			}
			pub fn set_overlay_texture_id(&self, value: f64) {
				unsafe { prop_set_character_mesh_overlay_texture_id(self.to_ptr(), value) }
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
				unsafe { prop_clothing_color_3(self.to_ptr()) }
			}
			pub fn set_color_3(&self, value: Color3) {
				unsafe { prop_set_clothing_color_3(self.to_ptr(), value) }
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
				unsafe { prop_pants_pants_template(self.to_ptr()) }
			}
			pub fn set_pants_template(&self, value: Content) {
				unsafe { prop_set_pants_pants_template(self.to_ptr(), value) }
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
				unsafe { prop_shirt_shirt_template(self.to_ptr()) }
			}
			pub fn set_shirt_template(&self, value: Content) {
				unsafe { prop_set_shirt_shirt_template(self.to_ptr(), value) }
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
				unsafe { prop_shirt_graphic_color_3(self.to_ptr()) }
			}
			pub fn set_color_3(&self, value: Color3) {
				unsafe { prop_set_shirt_graphic_color_3(self.to_ptr(), value) }
			}
			pub fn graphic(&self) -> Content {
				unsafe { prop_shirt_graphic_graphic(self.to_ptr()) }
			}
			pub fn set_graphic(&self, value: Content) {
				unsafe { prop_set_shirt_graphic_graphic(self.to_ptr(), value) }
			}
		}
		impl From<$name> for CharacterAppearance {
			fn from(value: $name) -> CharacterAppearance {
				CharacterAppearance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_chat {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn bubble_chat_enabled(&self) -> bool {
				unsafe { prop_chat_bubble_chat_enabled(self.to_ptr()) }
			}
			pub fn set_bubble_chat_enabled(&self, value: bool) {
				unsafe { prop_set_chat_bubble_chat_enabled(self.to_ptr(), value) }
			}
			pub fn load_default_chat(&self) -> bool {
				unsafe { prop_chat_load_default_chat(self.to_ptr()) }
			}
			pub fn fn_can_user_chat_async(&self, user_id: f64) -> bool {
				unsafe { dyn_chat_can_user_chat_async(self.to_ptr(), user_id) }
			}
			pub fn fn_can_users_chat_async(&self, user_id_from: f64, user_id_to: f64) -> bool {
				unsafe { dyn_chat_can_users_chat_async(self.to_ptr(), user_id_from, user_id_to) }
			}
			pub fn fn_filter_string_async(&self, string_to_filter: &str, player_from: Option<Player>, player_to: Option<Player>) -> String {
				unsafe { dyn_chat_filter_string_async(self.to_ptr(), string_to_filter, player_from, player_to) }
			}
			pub fn fn_filter_string_for_broadcast(&self, string_to_filter: &str, player_from: Option<Player>) -> String {
				unsafe { dyn_chat_filter_string_for_broadcast(self.to_ptr(), string_to_filter, player_from) }
			}
			pub fn fn_filter_string_for_player_async(&self, string_to_filter: &str, player_to_filter_for: Option<Player>) -> String {
				unsafe { dyn_chat_filter_string_for_player_async(self.to_ptr(), string_to_filter, player_to_filter_for) }
			}
			pub fn on_chatted<F: 'static + Fn(Option<Instance>, String, ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_chat_chatted(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_click_detector {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn cursor_icon(&self) -> Content {
				unsafe { prop_click_detector_cursor_icon(self.to_ptr()) }
			}
			pub fn set_cursor_icon(&self, value: Content) {
				unsafe { prop_set_click_detector_cursor_icon(self.to_ptr(), value) }
			}
			pub fn max_activation_distance(&self) -> f64 {
				unsafe { prop_click_detector_max_activation_distance(self.to_ptr()) }
			}
			pub fn set_max_activation_distance(&self, value: f64) {
				unsafe { prop_set_click_detector_max_activation_distance(self.to_ptr(), value) }
			}
			pub fn on_mouse_click<F: 'static + Fn(Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_click_detector_mouse_click(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_hover_enter<F: 'static + Fn(Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_click_detector_mouse_hover_enter(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_hover_leave<F: 'static + Fn(Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_click_detector_mouse_hover_leave(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_right_mouse_click<F: 'static + Fn(Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_click_detector_right_mouse_click(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_clouds {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn color(&self) -> Color3 {
				unsafe { prop_clouds_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { prop_set_clouds_color(self.to_ptr(), value) }
			}
			pub fn cover(&self) -> f64 {
				unsafe { prop_clouds_cover(self.to_ptr()) }
			}
			pub fn set_cover(&self, value: f64) {
				unsafe { prop_set_clouds_cover(self.to_ptr(), value) }
			}
			pub fn density(&self) -> f64 {
				unsafe { prop_clouds_density(self.to_ptr()) }
			}
			pub fn set_density(&self, value: f64) {
				unsafe { prop_set_clouds_density(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_clouds_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_clouds_enabled(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_collection_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_add_tag(&self, instance: Option<Instance>, tag: &str) {
				unsafe { dyn_collection_service_add_tag(self.to_ptr(), instance, tag) }
			}
			pub fn fn_get_all_tags(&self) {
				unsafe { dyn_collection_service_get_all_tags(self.to_ptr()) }
			}
			pub fn fn_get_collection(&self, class: &str) -> Objects {
				unsafe { dyn_collection_service_get_collection(self.to_ptr(), class) }
			}
			pub fn fn_get_instance_added_signal(&self, tag: &str) -> RbxScriptSignal {
				unsafe { dyn_collection_service_get_instance_added_signal(self.to_ptr(), tag) }
			}
			pub fn fn_get_instance_removed_signal(&self, tag: &str) -> RbxScriptSignal {
				unsafe { dyn_collection_service_get_instance_removed_signal(self.to_ptr(), tag) }
			}
			pub fn fn_get_tagged(&self, tag: &str) -> Objects {
				unsafe { dyn_collection_service_get_tagged(self.to_ptr(), tag) }
			}
			pub fn fn_get_tags(&self, instance: Option<Instance>) {
				unsafe { dyn_collection_service_get_tags(self.to_ptr(), instance) }
			}
			pub fn fn_has_tag(&self, instance: Option<Instance>, tag: &str) -> bool {
				unsafe { dyn_collection_service_has_tag(self.to_ptr(), instance, tag) }
			}
			pub fn fn_remove_tag(&self, instance: Option<Instance>, tag: &str) {
				unsafe { dyn_collection_service_remove_tag(self.to_ptr(), instance, tag) }
			}
			pub fn on_item_added<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_collection_service_item_added(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_item_removed<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_collection_service_item_removed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_tag_added<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_collection_service_tag_added(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_tag_removed<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_collection_service_tag_removed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_command_instance {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn allow_gui_access_points(&self) -> bool {
				unsafe { prop_command_instance_allow_gui_access_points(self.to_ptr()) }
			}
			pub fn display_name(&self) -> String {
				unsafe { prop_command_instance_display_name(self.to_ptr()) }
			}
			pub fn set_display_name(&self, value: &str) {
				unsafe { prop_set_command_instance_display_name(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_command_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_constraint {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn active(&self) -> bool {
				unsafe { prop_constraint_active(self.to_ptr()) }
			}
			pub fn attachment_0(&self) -> Option<Attachment> {
				unsafe { prop_constraint_attachment_0(self.to_ptr()) }
			}
			pub fn set_attachment_0(&self, value: Option<Attachment>) {
				unsafe { prop_set_constraint_attachment_0(self.to_ptr(), value) }
			}
			pub fn attachment_1(&self) -> Option<Attachment> {
				unsafe { prop_constraint_attachment_1(self.to_ptr()) }
			}
			pub fn set_attachment_1(&self, value: Option<Attachment>) {
				unsafe { prop_set_constraint_attachment_1(self.to_ptr(), value) }
			}
			pub fn color(&self) -> BrickColor {
				unsafe { prop_constraint_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: BrickColor) {
				unsafe { prop_set_constraint_color(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_constraint_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_constraint_enabled(self.to_ptr(), value) }
			}
			pub fn visible(&self) -> bool {
				unsafe { prop_constraint_visible(self.to_ptr()) }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { prop_set_constraint_visible(self.to_ptr(), value) }
			}
		}
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
				unsafe { prop_align_orientation_c_frame(self.to_ptr()) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { prop_set_align_orientation_c_frame(self.to_ptr(), value) }
			}
			pub fn max_angular_velocity(&self) -> f64 {
				unsafe { prop_align_orientation_max_angular_velocity(self.to_ptr()) }
			}
			pub fn set_max_angular_velocity(&self, value: f64) {
				unsafe { prop_set_align_orientation_max_angular_velocity(self.to_ptr(), value) }
			}
			pub fn max_torque(&self) -> f64 {
				unsafe { prop_align_orientation_max_torque(self.to_ptr()) }
			}
			pub fn set_max_torque(&self, value: f64) {
				unsafe { prop_set_align_orientation_max_torque(self.to_ptr(), value) }
			}
			pub fn primary_axis(&self) -> Vector3 {
				unsafe { prop_align_orientation_primary_axis(self.to_ptr()) }
			}
			pub fn set_primary_axis(&self, value: Vector3) {
				unsafe { prop_set_align_orientation_primary_axis(self.to_ptr(), value) }
			}
			pub fn primary_axis_only(&self) -> bool {
				unsafe { prop_align_orientation_primary_axis_only(self.to_ptr()) }
			}
			pub fn set_primary_axis_only(&self, value: bool) {
				unsafe { prop_set_align_orientation_primary_axis_only(self.to_ptr(), value) }
			}
			pub fn reaction_torque_enabled(&self) -> bool {
				unsafe { prop_align_orientation_reaction_torque_enabled(self.to_ptr()) }
			}
			pub fn set_reaction_torque_enabled(&self, value: bool) {
				unsafe { prop_set_align_orientation_reaction_torque_enabled(self.to_ptr(), value) }
			}
			pub fn responsiveness(&self) -> f64 {
				unsafe { prop_align_orientation_responsiveness(self.to_ptr()) }
			}
			pub fn set_responsiveness(&self, value: f64) {
				unsafe { prop_set_align_orientation_responsiveness(self.to_ptr(), value) }
			}
			pub fn rigidity_enabled(&self) -> bool {
				unsafe { prop_align_orientation_rigidity_enabled(self.to_ptr()) }
			}
			pub fn set_rigidity_enabled(&self, value: bool) {
				unsafe { prop_set_align_orientation_rigidity_enabled(self.to_ptr(), value) }
			}
			pub fn secondary_axis(&self) -> Vector3 {
				unsafe { prop_align_orientation_secondary_axis(self.to_ptr()) }
			}
			pub fn set_secondary_axis(&self, value: Vector3) {
				unsafe { prop_set_align_orientation_secondary_axis(self.to_ptr(), value) }
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
				unsafe { prop_align_position_apply_at_center_of_mass(self.to_ptr()) }
			}
			pub fn set_apply_at_center_of_mass(&self, value: bool) {
				unsafe { prop_set_align_position_apply_at_center_of_mass(self.to_ptr(), value) }
			}
			pub fn max_force(&self) -> f64 {
				unsafe { prop_align_position_max_force(self.to_ptr()) }
			}
			pub fn set_max_force(&self, value: f64) {
				unsafe { prop_set_align_position_max_force(self.to_ptr(), value) }
			}
			pub fn max_velocity(&self) -> f64 {
				unsafe { prop_align_position_max_velocity(self.to_ptr()) }
			}
			pub fn set_max_velocity(&self, value: f64) {
				unsafe { prop_set_align_position_max_velocity(self.to_ptr(), value) }
			}
			pub fn position(&self) -> Vector3 {
				unsafe { prop_align_position_position(self.to_ptr()) }
			}
			pub fn set_position(&self, value: Vector3) {
				unsafe { prop_set_align_position_position(self.to_ptr(), value) }
			}
			pub fn reaction_force_enabled(&self) -> bool {
				unsafe { prop_align_position_reaction_force_enabled(self.to_ptr()) }
			}
			pub fn set_reaction_force_enabled(&self, value: bool) {
				unsafe { prop_set_align_position_reaction_force_enabled(self.to_ptr(), value) }
			}
			pub fn responsiveness(&self) -> f64 {
				unsafe { prop_align_position_responsiveness(self.to_ptr()) }
			}
			pub fn set_responsiveness(&self, value: f64) {
				unsafe { prop_set_align_position_responsiveness(self.to_ptr(), value) }
			}
			pub fn rigidity_enabled(&self) -> bool {
				unsafe { prop_align_position_rigidity_enabled(self.to_ptr()) }
			}
			pub fn set_rigidity_enabled(&self, value: bool) {
				unsafe { prop_set_align_position_rigidity_enabled(self.to_ptr(), value) }
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
				unsafe { prop_angular_velocity_angular_velocity(self.to_ptr()) }
			}
			pub fn set_angular_velocity(&self, value: Vector3) {
				unsafe { prop_set_angular_velocity_angular_velocity(self.to_ptr(), value) }
			}
			pub fn max_torque(&self) -> f64 {
				unsafe { prop_angular_velocity_max_torque(self.to_ptr()) }
			}
			pub fn set_max_torque(&self, value: f64) {
				unsafe { prop_set_angular_velocity_max_torque(self.to_ptr(), value) }
			}
			pub fn reaction_torque_enabled(&self) -> bool {
				unsafe { prop_angular_velocity_reaction_torque_enabled(self.to_ptr()) }
			}
			pub fn set_reaction_torque_enabled(&self, value: bool) {
				unsafe { prop_set_angular_velocity_reaction_torque_enabled(self.to_ptr(), value) }
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
				unsafe { prop_ball_socket_constraint_limits_enabled(self.to_ptr()) }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { prop_set_ball_socket_constraint_limits_enabled(self.to_ptr(), value) }
			}
			pub fn max_friction_torque(&self) -> f64 {
				unsafe { prop_ball_socket_constraint_max_friction_torque(self.to_ptr()) }
			}
			pub fn set_max_friction_torque(&self, value: f64) {
				unsafe { prop_set_ball_socket_constraint_max_friction_torque(self.to_ptr(), value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { prop_ball_socket_constraint_radius(self.to_ptr()) }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { prop_set_ball_socket_constraint_radius(self.to_ptr(), value) }
			}
			pub fn restitution(&self) -> f64 {
				unsafe { prop_ball_socket_constraint_restitution(self.to_ptr()) }
			}
			pub fn set_restitution(&self, value: f64) {
				unsafe { prop_set_ball_socket_constraint_restitution(self.to_ptr(), value) }
			}
			pub fn twist_limits_enabled(&self) -> bool {
				unsafe { prop_ball_socket_constraint_twist_limits_enabled(self.to_ptr()) }
			}
			pub fn set_twist_limits_enabled(&self, value: bool) {
				unsafe { prop_set_ball_socket_constraint_twist_limits_enabled(self.to_ptr(), value) }
			}
			pub fn twist_lower_angle(&self) -> f64 {
				unsafe { prop_ball_socket_constraint_twist_lower_angle(self.to_ptr()) }
			}
			pub fn set_twist_lower_angle(&self, value: f64) {
				unsafe { prop_set_ball_socket_constraint_twist_lower_angle(self.to_ptr(), value) }
			}
			pub fn twist_upper_angle(&self) -> f64 {
				unsafe { prop_ball_socket_constraint_twist_upper_angle(self.to_ptr()) }
			}
			pub fn set_twist_upper_angle(&self, value: f64) {
				unsafe { prop_set_ball_socket_constraint_twist_upper_angle(self.to_ptr(), value) }
			}
			pub fn upper_angle(&self) -> f64 {
				unsafe { prop_ball_socket_constraint_upper_angle(self.to_ptr()) }
			}
			pub fn set_upper_angle(&self, value: f64) {
				unsafe { prop_set_ball_socket_constraint_upper_angle(self.to_ptr(), value) }
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
				unsafe { prop_hinge_constraint_angular_responsiveness(self.to_ptr()) }
			}
			pub fn set_angular_responsiveness(&self, value: f64) {
				unsafe { prop_set_hinge_constraint_angular_responsiveness(self.to_ptr(), value) }
			}
			pub fn angular_speed(&self) -> f64 {
				unsafe { prop_hinge_constraint_angular_speed(self.to_ptr()) }
			}
			pub fn set_angular_speed(&self, value: f64) {
				unsafe { prop_set_hinge_constraint_angular_speed(self.to_ptr(), value) }
			}
			pub fn angular_velocity(&self) -> f64 {
				unsafe { prop_hinge_constraint_angular_velocity(self.to_ptr()) }
			}
			pub fn set_angular_velocity(&self, value: f64) {
				unsafe { prop_set_hinge_constraint_angular_velocity(self.to_ptr(), value) }
			}
			pub fn current_angle(&self) -> f64 {
				unsafe { prop_hinge_constraint_current_angle(self.to_ptr()) }
			}
			pub fn limits_enabled(&self) -> bool {
				unsafe { prop_hinge_constraint_limits_enabled(self.to_ptr()) }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { prop_set_hinge_constraint_limits_enabled(self.to_ptr(), value) }
			}
			pub fn lower_angle(&self) -> f64 {
				unsafe { prop_hinge_constraint_lower_angle(self.to_ptr()) }
			}
			pub fn set_lower_angle(&self, value: f64) {
				unsafe { prop_set_hinge_constraint_lower_angle(self.to_ptr(), value) }
			}
			pub fn motor_max_acceleration(&self) -> f64 {
				unsafe { prop_hinge_constraint_motor_max_acceleration(self.to_ptr()) }
			}
			pub fn set_motor_max_acceleration(&self, value: f64) {
				unsafe { prop_set_hinge_constraint_motor_max_acceleration(self.to_ptr(), value) }
			}
			pub fn motor_max_torque(&self) -> f64 {
				unsafe { prop_hinge_constraint_motor_max_torque(self.to_ptr()) }
			}
			pub fn set_motor_max_torque(&self, value: f64) {
				unsafe { prop_set_hinge_constraint_motor_max_torque(self.to_ptr(), value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { prop_hinge_constraint_radius(self.to_ptr()) }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { prop_set_hinge_constraint_radius(self.to_ptr(), value) }
			}
			pub fn restitution(&self) -> f64 {
				unsafe { prop_hinge_constraint_restitution(self.to_ptr()) }
			}
			pub fn set_restitution(&self, value: f64) {
				unsafe { prop_set_hinge_constraint_restitution(self.to_ptr(), value) }
			}
			pub fn servo_max_torque(&self) -> f64 {
				unsafe { prop_hinge_constraint_servo_max_torque(self.to_ptr()) }
			}
			pub fn set_servo_max_torque(&self, value: f64) {
				unsafe { prop_set_hinge_constraint_servo_max_torque(self.to_ptr(), value) }
			}
			pub fn target_angle(&self) -> f64 {
				unsafe { prop_hinge_constraint_target_angle(self.to_ptr()) }
			}
			pub fn set_target_angle(&self, value: f64) {
				unsafe { prop_set_hinge_constraint_target_angle(self.to_ptr(), value) }
			}
			pub fn upper_angle(&self) -> f64 {
				unsafe { prop_hinge_constraint_upper_angle(self.to_ptr()) }
			}
			pub fn set_upper_angle(&self, value: f64) {
				unsafe { prop_set_hinge_constraint_upper_angle(self.to_ptr(), value) }
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
				unsafe { prop_line_force_apply_at_center_of_mass(self.to_ptr()) }
			}
			pub fn set_apply_at_center_of_mass(&self, value: bool) {
				unsafe { prop_set_line_force_apply_at_center_of_mass(self.to_ptr(), value) }
			}
			pub fn inverse_square_law(&self) -> bool {
				unsafe { prop_line_force_inverse_square_law(self.to_ptr()) }
			}
			pub fn set_inverse_square_law(&self, value: bool) {
				unsafe { prop_set_line_force_inverse_square_law(self.to_ptr(), value) }
			}
			pub fn magnitude(&self) -> f64 {
				unsafe { prop_line_force_magnitude(self.to_ptr()) }
			}
			pub fn set_magnitude(&self, value: f64) {
				unsafe { prop_set_line_force_magnitude(self.to_ptr(), value) }
			}
			pub fn max_force(&self) -> f64 {
				unsafe { prop_line_force_max_force(self.to_ptr()) }
			}
			pub fn set_max_force(&self, value: f64) {
				unsafe { prop_set_line_force_max_force(self.to_ptr(), value) }
			}
			pub fn reaction_force_enabled(&self) -> bool {
				unsafe { prop_line_force_reaction_force_enabled(self.to_ptr()) }
			}
			pub fn set_reaction_force_enabled(&self, value: bool) {
				unsafe { prop_set_line_force_reaction_force_enabled(self.to_ptr(), value) }
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
				unsafe { prop_linear_velocity_line_direction(self.to_ptr()) }
			}
			pub fn set_line_direction(&self, value: Vector3) {
				unsafe { prop_set_linear_velocity_line_direction(self.to_ptr(), value) }
			}
			pub fn line_velocity(&self) -> f64 {
				unsafe { prop_linear_velocity_line_velocity(self.to_ptr()) }
			}
			pub fn set_line_velocity(&self, value: f64) {
				unsafe { prop_set_linear_velocity_line_velocity(self.to_ptr(), value) }
			}
			pub fn max_force(&self) -> f64 {
				unsafe { prop_linear_velocity_max_force(self.to_ptr()) }
			}
			pub fn set_max_force(&self, value: f64) {
				unsafe { prop_set_linear_velocity_max_force(self.to_ptr(), value) }
			}
			pub fn plane_velocity(&self) -> Vector2 {
				unsafe { prop_linear_velocity_plane_velocity(self.to_ptr()) }
			}
			pub fn set_plane_velocity(&self, value: Vector2) {
				unsafe { prop_set_linear_velocity_plane_velocity(self.to_ptr(), value) }
			}
			pub fn primary_tangent_axis(&self) -> Vector3 {
				unsafe { prop_linear_velocity_primary_tangent_axis(self.to_ptr()) }
			}
			pub fn set_primary_tangent_axis(&self, value: Vector3) {
				unsafe { prop_set_linear_velocity_primary_tangent_axis(self.to_ptr(), value) }
			}
			pub fn secondary_tangent_axis(&self) -> Vector3 {
				unsafe { prop_linear_velocity_secondary_tangent_axis(self.to_ptr()) }
			}
			pub fn set_secondary_tangent_axis(&self, value: Vector3) {
				unsafe { prop_set_linear_velocity_secondary_tangent_axis(self.to_ptr(), value) }
			}
			pub fn vector_velocity(&self) -> Vector3 {
				unsafe { prop_linear_velocity_vector_velocity(self.to_ptr()) }
			}
			pub fn set_vector_velocity(&self, value: Vector3) {
				unsafe { prop_set_linear_velocity_vector_velocity(self.to_ptr(), value) }
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
				unsafe { prop_rigid_constraint_broken(self.to_ptr()) }
			}
			pub fn destruction_enabled(&self) -> bool {
				unsafe { prop_rigid_constraint_destruction_enabled(self.to_ptr()) }
			}
			pub fn set_destruction_enabled(&self, value: bool) {
				unsafe { prop_set_rigid_constraint_destruction_enabled(self.to_ptr(), value) }
			}
			pub fn destruction_force(&self) -> f64 {
				unsafe { prop_rigid_constraint_destruction_force(self.to_ptr()) }
			}
			pub fn set_destruction_force(&self, value: f64) {
				unsafe { prop_set_rigid_constraint_destruction_force(self.to_ptr(), value) }
			}
			pub fn destruction_torque(&self) -> f64 {
				unsafe { prop_rigid_constraint_destruction_torque(self.to_ptr()) }
			}
			pub fn set_destruction_torque(&self, value: f64) {
				unsafe { prop_set_rigid_constraint_destruction_torque(self.to_ptr(), value) }
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
				unsafe { prop_rod_constraint_current_distance(self.to_ptr()) }
			}
			pub fn length(&self) -> f64 {
				unsafe { prop_rod_constraint_length(self.to_ptr()) }
			}
			pub fn set_length(&self, value: f64) {
				unsafe { prop_set_rod_constraint_length(self.to_ptr(), value) }
			}
			pub fn limit_angle_0(&self) -> f64 {
				unsafe { prop_rod_constraint_limit_angle_0(self.to_ptr()) }
			}
			pub fn set_limit_angle_0(&self, value: f64) {
				unsafe { prop_set_rod_constraint_limit_angle_0(self.to_ptr(), value) }
			}
			pub fn limit_angle_1(&self) -> f64 {
				unsafe { prop_rod_constraint_limit_angle_1(self.to_ptr()) }
			}
			pub fn set_limit_angle_1(&self, value: f64) {
				unsafe { prop_set_rod_constraint_limit_angle_1(self.to_ptr(), value) }
			}
			pub fn limits_enabled(&self) -> bool {
				unsafe { prop_rod_constraint_limits_enabled(self.to_ptr()) }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { prop_set_rod_constraint_limits_enabled(self.to_ptr(), value) }
			}
			pub fn thickness(&self) -> f64 {
				unsafe { prop_rod_constraint_thickness(self.to_ptr()) }
			}
			pub fn set_thickness(&self, value: f64) {
				unsafe { prop_set_rod_constraint_thickness(self.to_ptr(), value) }
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
				unsafe { prop_rope_constraint_current_distance(self.to_ptr()) }
			}
			pub fn length(&self) -> f64 {
				unsafe { prop_rope_constraint_length(self.to_ptr()) }
			}
			pub fn set_length(&self, value: f64) {
				unsafe { prop_set_rope_constraint_length(self.to_ptr(), value) }
			}
			pub fn restitution(&self) -> f64 {
				unsafe { prop_rope_constraint_restitution(self.to_ptr()) }
			}
			pub fn set_restitution(&self, value: f64) {
				unsafe { prop_set_rope_constraint_restitution(self.to_ptr(), value) }
			}
			pub fn thickness(&self) -> f64 {
				unsafe { prop_rope_constraint_thickness(self.to_ptr()) }
			}
			pub fn set_thickness(&self, value: f64) {
				unsafe { prop_set_rope_constraint_thickness(self.to_ptr(), value) }
			}
			pub fn winch_enabled(&self) -> bool {
				unsafe { prop_rope_constraint_winch_enabled(self.to_ptr()) }
			}
			pub fn set_winch_enabled(&self, value: bool) {
				unsafe { prop_set_rope_constraint_winch_enabled(self.to_ptr(), value) }
			}
			pub fn winch_force(&self) -> f64 {
				unsafe { prop_rope_constraint_winch_force(self.to_ptr()) }
			}
			pub fn set_winch_force(&self, value: f64) {
				unsafe { prop_set_rope_constraint_winch_force(self.to_ptr(), value) }
			}
			pub fn winch_responsiveness(&self) -> f64 {
				unsafe { prop_rope_constraint_winch_responsiveness(self.to_ptr()) }
			}
			pub fn set_winch_responsiveness(&self, value: f64) {
				unsafe { prop_set_rope_constraint_winch_responsiveness(self.to_ptr(), value) }
			}
			pub fn winch_speed(&self) -> f64 {
				unsafe { prop_rope_constraint_winch_speed(self.to_ptr()) }
			}
			pub fn set_winch_speed(&self, value: f64) {
				unsafe { prop_set_rope_constraint_winch_speed(self.to_ptr(), value) }
			}
			pub fn winch_target(&self) -> f64 {
				unsafe { prop_rope_constraint_winch_target(self.to_ptr()) }
			}
			pub fn set_winch_target(&self, value: f64) {
				unsafe { prop_set_rope_constraint_winch_target(self.to_ptr(), value) }
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
				unsafe { prop_sliding_ball_constraint_current_position(self.to_ptr()) }
			}
			pub fn limits_enabled(&self) -> bool {
				unsafe { prop_sliding_ball_constraint_limits_enabled(self.to_ptr()) }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { prop_set_sliding_ball_constraint_limits_enabled(self.to_ptr(), value) }
			}
			pub fn linear_responsiveness(&self) -> f64 {
				unsafe { prop_sliding_ball_constraint_linear_responsiveness(self.to_ptr()) }
			}
			pub fn set_linear_responsiveness(&self, value: f64) {
				unsafe { prop_set_sliding_ball_constraint_linear_responsiveness(self.to_ptr(), value) }
			}
			pub fn lower_limit(&self) -> f64 {
				unsafe { prop_sliding_ball_constraint_lower_limit(self.to_ptr()) }
			}
			pub fn set_lower_limit(&self, value: f64) {
				unsafe { prop_set_sliding_ball_constraint_lower_limit(self.to_ptr(), value) }
			}
			pub fn motor_max_acceleration(&self) -> f64 {
				unsafe { prop_sliding_ball_constraint_motor_max_acceleration(self.to_ptr()) }
			}
			pub fn set_motor_max_acceleration(&self, value: f64) {
				unsafe { prop_set_sliding_ball_constraint_motor_max_acceleration(self.to_ptr(), value) }
			}
			pub fn motor_max_force(&self) -> f64 {
				unsafe { prop_sliding_ball_constraint_motor_max_force(self.to_ptr()) }
			}
			pub fn set_motor_max_force(&self, value: f64) {
				unsafe { prop_set_sliding_ball_constraint_motor_max_force(self.to_ptr(), value) }
			}
			pub fn restitution(&self) -> f64 {
				unsafe { prop_sliding_ball_constraint_restitution(self.to_ptr()) }
			}
			pub fn set_restitution(&self, value: f64) {
				unsafe { prop_set_sliding_ball_constraint_restitution(self.to_ptr(), value) }
			}
			pub fn servo_max_force(&self) -> f64 {
				unsafe { prop_sliding_ball_constraint_servo_max_force(self.to_ptr()) }
			}
			pub fn set_servo_max_force(&self, value: f64) {
				unsafe { prop_set_sliding_ball_constraint_servo_max_force(self.to_ptr(), value) }
			}
			pub fn size(&self) -> f64 {
				unsafe { prop_sliding_ball_constraint_size(self.to_ptr()) }
			}
			pub fn set_size(&self, value: f64) {
				unsafe { prop_set_sliding_ball_constraint_size(self.to_ptr(), value) }
			}
			pub fn speed(&self) -> f64 {
				unsafe { prop_sliding_ball_constraint_speed(self.to_ptr()) }
			}
			pub fn set_speed(&self, value: f64) {
				unsafe { prop_set_sliding_ball_constraint_speed(self.to_ptr(), value) }
			}
			pub fn target_position(&self) -> f64 {
				unsafe { prop_sliding_ball_constraint_target_position(self.to_ptr()) }
			}
			pub fn set_target_position(&self, value: f64) {
				unsafe { prop_set_sliding_ball_constraint_target_position(self.to_ptr(), value) }
			}
			pub fn upper_limit(&self) -> f64 {
				unsafe { prop_sliding_ball_constraint_upper_limit(self.to_ptr()) }
			}
			pub fn set_upper_limit(&self, value: f64) {
				unsafe { prop_set_sliding_ball_constraint_upper_limit(self.to_ptr(), value) }
			}
			pub fn velocity(&self) -> f64 {
				unsafe { prop_sliding_ball_constraint_velocity(self.to_ptr()) }
			}
			pub fn set_velocity(&self, value: f64) {
				unsafe { prop_set_sliding_ball_constraint_velocity(self.to_ptr(), value) }
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
				unsafe { prop_cylindrical_constraint_angular_limits_enabled(self.to_ptr()) }
			}
			pub fn set_angular_limits_enabled(&self, value: bool) {
				unsafe { prop_set_cylindrical_constraint_angular_limits_enabled(self.to_ptr(), value) }
			}
			pub fn angular_responsiveness(&self) -> f64 {
				unsafe { prop_cylindrical_constraint_angular_responsiveness(self.to_ptr()) }
			}
			pub fn set_angular_responsiveness(&self, value: f64) {
				unsafe { prop_set_cylindrical_constraint_angular_responsiveness(self.to_ptr(), value) }
			}
			pub fn angular_restitution(&self) -> f64 {
				unsafe { prop_cylindrical_constraint_angular_restitution(self.to_ptr()) }
			}
			pub fn set_angular_restitution(&self, value: f64) {
				unsafe { prop_set_cylindrical_constraint_angular_restitution(self.to_ptr(), value) }
			}
			pub fn angular_speed(&self) -> f64 {
				unsafe { prop_cylindrical_constraint_angular_speed(self.to_ptr()) }
			}
			pub fn set_angular_speed(&self, value: f64) {
				unsafe { prop_set_cylindrical_constraint_angular_speed(self.to_ptr(), value) }
			}
			pub fn angular_velocity(&self) -> f64 {
				unsafe { prop_cylindrical_constraint_angular_velocity(self.to_ptr()) }
			}
			pub fn set_angular_velocity(&self, value: f64) {
				unsafe { prop_set_cylindrical_constraint_angular_velocity(self.to_ptr(), value) }
			}
			pub fn current_angle(&self) -> f64 {
				unsafe { prop_cylindrical_constraint_current_angle(self.to_ptr()) }
			}
			pub fn inclination_angle(&self) -> f64 {
				unsafe { prop_cylindrical_constraint_inclination_angle(self.to_ptr()) }
			}
			pub fn set_inclination_angle(&self, value: f64) {
				unsafe { prop_set_cylindrical_constraint_inclination_angle(self.to_ptr(), value) }
			}
			pub fn lower_angle(&self) -> f64 {
				unsafe { prop_cylindrical_constraint_lower_angle(self.to_ptr()) }
			}
			pub fn set_lower_angle(&self, value: f64) {
				unsafe { prop_set_cylindrical_constraint_lower_angle(self.to_ptr(), value) }
			}
			pub fn motor_max_angular_acceleration(&self) -> f64 {
				unsafe { prop_cylindrical_constraint_motor_max_angular_acceleration(self.to_ptr()) }
			}
			pub fn set_motor_max_angular_acceleration(&self, value: f64) {
				unsafe { prop_set_cylindrical_constraint_motor_max_angular_acceleration(self.to_ptr(), value) }
			}
			pub fn motor_max_torque(&self) -> f64 {
				unsafe { prop_cylindrical_constraint_motor_max_torque(self.to_ptr()) }
			}
			pub fn set_motor_max_torque(&self, value: f64) {
				unsafe { prop_set_cylindrical_constraint_motor_max_torque(self.to_ptr(), value) }
			}
			pub fn rotation_axis_visible(&self) -> bool {
				unsafe { prop_cylindrical_constraint_rotation_axis_visible(self.to_ptr()) }
			}
			pub fn set_rotation_axis_visible(&self, value: bool) {
				unsafe { prop_set_cylindrical_constraint_rotation_axis_visible(self.to_ptr(), value) }
			}
			pub fn servo_max_torque(&self) -> f64 {
				unsafe { prop_cylindrical_constraint_servo_max_torque(self.to_ptr()) }
			}
			pub fn set_servo_max_torque(&self, value: f64) {
				unsafe { prop_set_cylindrical_constraint_servo_max_torque(self.to_ptr(), value) }
			}
			pub fn target_angle(&self) -> f64 {
				unsafe { prop_cylindrical_constraint_target_angle(self.to_ptr()) }
			}
			pub fn set_target_angle(&self, value: f64) {
				unsafe { prop_set_cylindrical_constraint_target_angle(self.to_ptr(), value) }
			}
			pub fn upper_angle(&self) -> f64 {
				unsafe { prop_cylindrical_constraint_upper_angle(self.to_ptr()) }
			}
			pub fn set_upper_angle(&self, value: f64) {
				unsafe { prop_set_cylindrical_constraint_upper_angle(self.to_ptr(), value) }
			}
			pub fn world_rotation_axis(&self) -> Vector3 {
				unsafe { prop_cylindrical_constraint_world_rotation_axis(self.to_ptr()) }
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
				unsafe { prop_spring_constraint_coils(self.to_ptr()) }
			}
			pub fn set_coils(&self, value: f64) {
				unsafe { prop_set_spring_constraint_coils(self.to_ptr(), value) }
			}
			pub fn current_length(&self) -> f64 {
				unsafe { prop_spring_constraint_current_length(self.to_ptr()) }
			}
			pub fn damping(&self) -> f64 {
				unsafe { prop_spring_constraint_damping(self.to_ptr()) }
			}
			pub fn set_damping(&self, value: f64) {
				unsafe { prop_set_spring_constraint_damping(self.to_ptr(), value) }
			}
			pub fn free_length(&self) -> f64 {
				unsafe { prop_spring_constraint_free_length(self.to_ptr()) }
			}
			pub fn set_free_length(&self, value: f64) {
				unsafe { prop_set_spring_constraint_free_length(self.to_ptr(), value) }
			}
			pub fn limits_enabled(&self) -> bool {
				unsafe { prop_spring_constraint_limits_enabled(self.to_ptr()) }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { prop_set_spring_constraint_limits_enabled(self.to_ptr(), value) }
			}
			pub fn max_force(&self) -> f64 {
				unsafe { prop_spring_constraint_max_force(self.to_ptr()) }
			}
			pub fn set_max_force(&self, value: f64) {
				unsafe { prop_set_spring_constraint_max_force(self.to_ptr(), value) }
			}
			pub fn max_length(&self) -> f64 {
				unsafe { prop_spring_constraint_max_length(self.to_ptr()) }
			}
			pub fn set_max_length(&self, value: f64) {
				unsafe { prop_set_spring_constraint_max_length(self.to_ptr(), value) }
			}
			pub fn min_length(&self) -> f64 {
				unsafe { prop_spring_constraint_min_length(self.to_ptr()) }
			}
			pub fn set_min_length(&self, value: f64) {
				unsafe { prop_set_spring_constraint_min_length(self.to_ptr(), value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { prop_spring_constraint_radius(self.to_ptr()) }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { prop_set_spring_constraint_radius(self.to_ptr(), value) }
			}
			pub fn stiffness(&self) -> f64 {
				unsafe { prop_spring_constraint_stiffness(self.to_ptr()) }
			}
			pub fn set_stiffness(&self, value: f64) {
				unsafe { prop_set_spring_constraint_stiffness(self.to_ptr(), value) }
			}
			pub fn thickness(&self) -> f64 {
				unsafe { prop_spring_constraint_thickness(self.to_ptr()) }
			}
			pub fn set_thickness(&self, value: f64) {
				unsafe { prop_set_spring_constraint_thickness(self.to_ptr(), value) }
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
				unsafe { prop_torque_torque(self.to_ptr()) }
			}
			pub fn set_torque(&self, value: Vector3) {
				unsafe { prop_set_torque_torque(self.to_ptr(), value) }
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
				unsafe { prop_torsion_spring_constraint_coils(self.to_ptr()) }
			}
			pub fn set_coils(&self, value: f64) {
				unsafe { prop_set_torsion_spring_constraint_coils(self.to_ptr(), value) }
			}
			pub fn current_angle(&self) -> f64 {
				unsafe { prop_torsion_spring_constraint_current_angle(self.to_ptr()) }
			}
			pub fn damping(&self) -> f64 {
				unsafe { prop_torsion_spring_constraint_damping(self.to_ptr()) }
			}
			pub fn set_damping(&self, value: f64) {
				unsafe { prop_set_torsion_spring_constraint_damping(self.to_ptr(), value) }
			}
			pub fn limit_enabled(&self) -> bool {
				unsafe { prop_torsion_spring_constraint_limit_enabled(self.to_ptr()) }
			}
			pub fn set_limit_enabled(&self, value: bool) {
				unsafe { prop_set_torsion_spring_constraint_limit_enabled(self.to_ptr(), value) }
			}
			pub fn limits_enabled(&self) -> bool {
				unsafe { prop_torsion_spring_constraint_limits_enabled(self.to_ptr()) }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { prop_set_torsion_spring_constraint_limits_enabled(self.to_ptr(), value) }
			}
			pub fn max_angle(&self) -> f64 {
				unsafe { prop_torsion_spring_constraint_max_angle(self.to_ptr()) }
			}
			pub fn set_max_angle(&self, value: f64) {
				unsafe { prop_set_torsion_spring_constraint_max_angle(self.to_ptr(), value) }
			}
			pub fn max_torque(&self) -> f64 {
				unsafe { prop_torsion_spring_constraint_max_torque(self.to_ptr()) }
			}
			pub fn set_max_torque(&self, value: f64) {
				unsafe { prop_set_torsion_spring_constraint_max_torque(self.to_ptr(), value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { prop_torsion_spring_constraint_radius(self.to_ptr()) }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { prop_set_torsion_spring_constraint_radius(self.to_ptr(), value) }
			}
			pub fn restitution(&self) -> f64 {
				unsafe { prop_torsion_spring_constraint_restitution(self.to_ptr()) }
			}
			pub fn set_restitution(&self, value: f64) {
				unsafe { prop_set_torsion_spring_constraint_restitution(self.to_ptr(), value) }
			}
			pub fn stiffness(&self) -> f64 {
				unsafe { prop_torsion_spring_constraint_stiffness(self.to_ptr()) }
			}
			pub fn set_stiffness(&self, value: f64) {
				unsafe { prop_set_torsion_spring_constraint_stiffness(self.to_ptr(), value) }
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
				unsafe { prop_universal_constraint_limits_enabled(self.to_ptr()) }
			}
			pub fn set_limits_enabled(&self, value: bool) {
				unsafe { prop_set_universal_constraint_limits_enabled(self.to_ptr(), value) }
			}
			pub fn max_angle(&self) -> f64 {
				unsafe { prop_universal_constraint_max_angle(self.to_ptr()) }
			}
			pub fn set_max_angle(&self, value: f64) {
				unsafe { prop_set_universal_constraint_max_angle(self.to_ptr(), value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { prop_universal_constraint_radius(self.to_ptr()) }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { prop_set_universal_constraint_radius(self.to_ptr(), value) }
			}
			pub fn restitution(&self) -> f64 {
				unsafe { prop_universal_constraint_restitution(self.to_ptr()) }
			}
			pub fn set_restitution(&self, value: f64) {
				unsafe { prop_set_universal_constraint_restitution(self.to_ptr(), value) }
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
				unsafe { prop_vector_force_apply_at_center_of_mass(self.to_ptr()) }
			}
			pub fn set_apply_at_center_of_mass(&self, value: bool) {
				unsafe { prop_set_vector_force_apply_at_center_of_mass(self.to_ptr(), value) }
			}
			pub fn force(&self) -> Vector3 {
				unsafe { prop_vector_force_force(self.to_ptr()) }
			}
			pub fn set_force(&self, value: Vector3) {
				unsafe { prop_set_vector_force_force(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Constraint {
			fn from(value: $name) -> Constraint {
				Constraint(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_content_provider {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn base_url(&self) -> String {
				unsafe { prop_content_provider_base_url(self.to_ptr()) }
			}
			pub fn request_queue_size(&self) -> f64 {
				unsafe { prop_content_provider_request_queue_size(self.to_ptr()) }
			}
			pub fn fn_list_encrypted_assets(&self) {
				unsafe { dyn_content_provider_list_encrypted_assets(self.to_ptr()) }
			}
			pub fn fn_preload(&self, content_id: Content) {
				unsafe { dyn_content_provider_preload(self.to_ptr(), content_id) }
			}
			pub fn fn_register_default_encryption_key(&self, encryption_key: &str) {
				unsafe { dyn_content_provider_register_default_encryption_key(self.to_ptr(), encryption_key) }
			}
			pub fn fn_register_default_session_key(&self, session_key: &str) {
				unsafe { dyn_content_provider_register_default_session_key(self.to_ptr(), session_key) }
			}
			pub fn fn_register_encrypted_asset(&self, asset_id: Content, encryption_key: &str) {
				unsafe { dyn_content_provider_register_encrypted_asset(self.to_ptr(), asset_id, encryption_key) }
			}
			pub fn fn_register_session_encrypted_asset(&self, content_id: Content, session_key: &str) {
				unsafe { dyn_content_provider_register_session_encrypted_asset(self.to_ptr(), content_id, session_key) }
			}
			pub fn fn_unregister_default_encryption_key(&self) {
				unsafe { dyn_content_provider_unregister_default_encryption_key(self.to_ptr()) }
			}
			pub fn fn_unregister_encrypted_asset(&self, asset_id: Content) {
				unsafe { dyn_content_provider_unregister_encrypted_asset(self.to_ptr(), asset_id) }
			}
			pub fn on_asset_fetch_failed<F: 'static + Fn(Content)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_content_provider_asset_fetch_failed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_context_action_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_all_bound_action_info(&self) {
				unsafe { dyn_context_action_service_get_all_bound_action_info(self.to_ptr()) }
			}
			pub fn fn_get_bound_action_info(&self, action_name: &str) {
				unsafe { dyn_context_action_service_get_bound_action_info(self.to_ptr(), action_name) }
			}
			pub fn fn_get_current_local_tool_icon(&self) -> String {
				unsafe { dyn_context_action_service_get_current_local_tool_icon(self.to_ptr()) }
			}
			pub fn fn_set_description(&self, action_name: &str, description: &str) {
				unsafe { dyn_context_action_service_set_description(self.to_ptr(), action_name, description) }
			}
			pub fn fn_set_image(&self, action_name: &str, image: &str) {
				unsafe { dyn_context_action_service_set_image(self.to_ptr(), action_name, image) }
			}
			pub fn fn_set_position(&self, action_name: &str, position: UDim2) {
				unsafe { dyn_context_action_service_set_position(self.to_ptr(), action_name, position) }
			}
			pub fn fn_set_title(&self, action_name: &str, title: &str) {
				unsafe { dyn_context_action_service_set_title(self.to_ptr(), action_name, title) }
			}
			pub fn fn_unbind_action(&self, action_name: &str) {
				unsafe { dyn_context_action_service_unbind_action(self.to_ptr(), action_name) }
			}
			pub fn fn_unbind_all_actions(&self) {
				unsafe { dyn_context_action_service_unbind_all_actions(self.to_ptr()) }
			}
			pub fn fn_get_button(&self, action_name: &str) -> Option<Instance> {
				unsafe { dyn_context_action_service_get_button(self.to_ptr(), action_name) }
			}
			pub fn on_local_tool_equipped<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_context_action_service_local_tool_equipped(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_local_tool_unequipped<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_context_action_service_local_tool_unequipped(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_controller {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn on_button_changed<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_controller_button_changed(self.to_ptr(), Box::new(callback))) }
			}
		}
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
				unsafe { prop_skateboard_controller_steer(self.to_ptr()) }
			}
			pub fn throttle(&self) -> f64 {
				unsafe { prop_skateboard_controller_throttle(self.to_ptr()) }
			}
			pub fn on_axis_changed<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_skateboard_controller_axis_changed(self.to_ptr(), Box::new(callback))) }
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
macro_rules! impl_controller_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_model_mesh {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn offset(&self) -> Vector3 {
				unsafe { prop_data_model_mesh_offset(self.to_ptr()) }
			}
			pub fn set_offset(&self, value: Vector3) {
				unsafe { prop_set_data_model_mesh_offset(self.to_ptr(), value) }
			}
			pub fn scale(&self) -> Vector3 {
				unsafe { prop_data_model_mesh_scale(self.to_ptr()) }
			}
			pub fn set_scale(&self, value: Vector3) {
				unsafe { prop_set_data_model_mesh_scale(self.to_ptr(), value) }
			}
			pub fn vertex_color(&self) -> Vector3 {
				unsafe { prop_data_model_mesh_vertex_color(self.to_ptr()) }
			}
			pub fn set_vertex_color(&self, value: Vector3) {
				unsafe { prop_set_data_model_mesh_vertex_color(self.to_ptr(), value) }
			}
		}
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
				unsafe { prop_file_mesh_mesh_id(self.to_ptr()) }
			}
			pub fn set_mesh_id(&self, value: Content) {
				unsafe { prop_set_file_mesh_mesh_id(self.to_ptr(), value) }
			}
			pub fn texture_id(&self) -> Content {
				unsafe { prop_file_mesh_texture_id(self.to_ptr()) }
			}
			pub fn set_texture_id(&self, value: Content) {
				unsafe { prop_set_file_mesh_texture_id(self.to_ptr(), value) }
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_metadata(&self) {
				unsafe { dyn_data_store_increment_options_get_metadata(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_info {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn created_time(&self) -> f64 {
				unsafe { prop_data_store_info_created_time(self.to_ptr()) }
			}
			pub fn data_store_name(&self) -> String {
				unsafe { prop_data_store_info_data_store_name(self.to_ptr()) }
			}
			pub fn updated_time(&self) -> f64 {
				unsafe { prop_data_store_info_updated_time(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_key {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn key_name(&self) -> String {
				unsafe { prop_data_store_key_key_name(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_key_info {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn created_time(&self) -> f64 {
				unsafe { prop_data_store_key_info_created_time(self.to_ptr()) }
			}
			pub fn updated_time(&self) -> f64 {
				unsafe { prop_data_store_key_info_updated_time(self.to_ptr()) }
			}
			pub fn version(&self) -> String {
				unsafe { prop_data_store_key_info_version(self.to_ptr()) }
			}
			pub fn fn_get_metadata(&self) {
				unsafe { dyn_data_store_key_info_get_metadata(self.to_ptr()) }
			}
			pub fn fn_get_user_ids(&self) {
				unsafe { dyn_data_store_key_info_get_user_ids(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_object_version_info {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn created_time(&self) -> f64 {
				unsafe { prop_data_store_object_version_info_created_time(self.to_ptr()) }
			}
			pub fn is_deleted(&self) -> bool {
				unsafe { prop_data_store_object_version_info_is_deleted(self.to_ptr()) }
			}
			pub fn version(&self) -> String {
				unsafe { prop_data_store_object_version_info_version(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_options {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn all_scopes(&self) -> bool {
				unsafe { prop_data_store_options_all_scopes(self.to_ptr()) }
			}
			pub fn set_all_scopes(&self, value: bool) {
				unsafe { prop_set_data_store_options_all_scopes(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_data_store(&self, name: &str, scope: &str, options: Option<Instance>) -> Option<GlobalDataStore> {
				unsafe { dyn_data_store_service_get_data_store(self.to_ptr(), name, scope, options) }
			}
			pub fn fn_get_global_data_store(&self) -> Option<GlobalDataStore> {
				unsafe { dyn_data_store_service_get_global_data_store(self.to_ptr()) }
			}
			pub fn fn_get_ordered_data_store(&self, name: &str, scope: &str) -> Option<OrderedDataStore> {
				unsafe { dyn_data_store_service_get_ordered_data_store(self.to_ptr(), name, scope) }
			}
			pub fn fn_list_data_stores_async(&self, prefix: &str, page_size: f64) -> Option<DataStoreListingPages> {
				unsafe { dyn_data_store_service_list_data_stores_async(self.to_ptr(), prefix, page_size) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_data_store_set_options {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_metadata(&self) {
				unsafe { dyn_data_store_set_options_get_metadata(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_debris {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn max_items(&self) -> f64 {
				unsafe { prop_debris_max_items(self.to_ptr()) }
			}
			pub fn set_max_items(&self, value: f64) {
				unsafe { prop_set_debris_max_items(self.to_ptr(), value) }
			}
			pub fn fn_add_item(&self, item: Option<Instance>, lifetime: f64) {
				unsafe { dyn_debris_add_item(self.to_ptr(), item, lifetime) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_dialog {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn conversation_distance(&self) -> f64 {
				unsafe { prop_dialog_conversation_distance(self.to_ptr()) }
			}
			pub fn set_conversation_distance(&self, value: f64) {
				unsafe { prop_set_dialog_conversation_distance(self.to_ptr(), value) }
			}
			pub fn goodbye_choice_active(&self) -> bool {
				unsafe { prop_dialog_goodbye_choice_active(self.to_ptr()) }
			}
			pub fn set_goodbye_choice_active(&self, value: bool) {
				unsafe { prop_set_dialog_goodbye_choice_active(self.to_ptr(), value) }
			}
			pub fn goodbye_dialog(&self) -> String {
				unsafe { prop_dialog_goodbye_dialog(self.to_ptr()) }
			}
			pub fn set_goodbye_dialog(&self, value: &str) {
				unsafe { prop_set_dialog_goodbye_dialog(self.to_ptr(), value) }
			}
			pub fn in_use(&self) -> bool {
				unsafe { prop_dialog_in_use(self.to_ptr()) }
			}
			pub fn set_in_use(&self, value: bool) {
				unsafe { prop_set_dialog_in_use(self.to_ptr(), value) }
			}
			pub fn initial_prompt(&self) -> String {
				unsafe { prop_dialog_initial_prompt(self.to_ptr()) }
			}
			pub fn set_initial_prompt(&self, value: &str) {
				unsafe { prop_set_dialog_initial_prompt(self.to_ptr(), value) }
			}
			pub fn trigger_distance(&self) -> f64 {
				unsafe { prop_dialog_trigger_distance(self.to_ptr()) }
			}
			pub fn set_trigger_distance(&self, value: f64) {
				unsafe { prop_set_dialog_trigger_distance(self.to_ptr(), value) }
			}
			pub fn trigger_offset(&self) -> Vector3 {
				unsafe { prop_dialog_trigger_offset(self.to_ptr()) }
			}
			pub fn set_trigger_offset(&self, value: Vector3) {
				unsafe { prop_set_dialog_trigger_offset(self.to_ptr(), value) }
			}
			pub fn fn_get_current_players(&self) -> Objects {
				unsafe { dyn_dialog_get_current_players(self.to_ptr()) }
			}
			pub fn on_dialog_choice_selected<F: 'static + Fn(Option<Instance>, Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_dialog_dialog_choice_selected(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_dialog_choice {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn goodbye_choice_active(&self) -> bool {
				unsafe { prop_dialog_choice_goodbye_choice_active(self.to_ptr()) }
			}
			pub fn set_goodbye_choice_active(&self, value: bool) {
				unsafe { prop_set_dialog_choice_goodbye_choice_active(self.to_ptr(), value) }
			}
			pub fn goodbye_dialog(&self) -> String {
				unsafe { prop_dialog_choice_goodbye_dialog(self.to_ptr()) }
			}
			pub fn set_goodbye_dialog(&self, value: &str) {
				unsafe { prop_set_dialog_choice_goodbye_dialog(self.to_ptr(), value) }
			}
			pub fn response_dialog(&self) -> String {
				unsafe { prop_dialog_choice_response_dialog(self.to_ptr()) }
			}
			pub fn set_response_dialog(&self, value: &str) {
				unsafe { prop_set_dialog_choice_response_dialog(self.to_ptr(), value) }
			}
			pub fn user_dialog(&self) -> String {
				unsafe { prop_dialog_choice_user_dialog(self.to_ptr()) }
			}
			pub fn set_user_dialog(&self, value: &str) {
				unsafe { prop_set_dialog_choice_user_dialog(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_dragger {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_mouse_down(&self, mouse_part: Option<Instance>, point_on_mouse_part: Vector3, parts: Objects) {
				unsafe { dyn_dragger_mouse_down(self.to_ptr(), mouse_part, point_on_mouse_part, parts) }
			}
			pub fn fn_mouse_move(&self, mouse_ray: Ray) {
				unsafe { dyn_dragger_mouse_move(self.to_ptr(), mouse_ray) }
			}
			pub fn fn_mouse_up(&self) {
				unsafe { dyn_dragger_mouse_up(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_dragger_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn align_dragged_objects(&self) -> bool {
				unsafe { prop_dragger_service_align_dragged_objects(self.to_ptr()) }
			}
			pub fn angle_snap_enabled(&self) -> bool {
				unsafe { prop_dragger_service_angle_snap_enabled(self.to_ptr()) }
			}
			pub fn angle_snap_increment(&self) -> f64 {
				unsafe { prop_dragger_service_angle_snap_increment(self.to_ptr()) }
			}
			pub fn animate_hover(&self) -> bool {
				unsafe { prop_dragger_service_animate_hover(self.to_ptr()) }
			}
			pub fn collisions_enabled(&self) -> bool {
				unsafe { prop_dragger_service_collisions_enabled(self.to_ptr()) }
			}
			pub fn geometry_snap_color(&self) -> Color3 {
				unsafe { prop_dragger_service_geometry_snap_color(self.to_ptr()) }
			}
			pub fn hover_animate_frequency(&self) -> f64 {
				unsafe { prop_dragger_service_hover_animate_frequency(self.to_ptr()) }
			}
			pub fn hover_thickness(&self) -> f64 {
				unsafe { prop_dragger_service_hover_thickness(self.to_ptr()) }
			}
			pub fn joints_enabled(&self) -> bool {
				unsafe { prop_dragger_service_joints_enabled(self.to_ptr()) }
			}
			pub fn linear_snap_enabled(&self) -> bool {
				unsafe { prop_dragger_service_linear_snap_enabled(self.to_ptr()) }
			}
			pub fn linear_snap_increment(&self) -> f64 {
				unsafe { prop_dragger_service_linear_snap_increment(self.to_ptr()) }
			}
			pub fn show_hover(&self) -> bool {
				unsafe { prop_dragger_service_show_hover(self.to_ptr()) }
			}
			pub fn show_pivot_indicator(&self) -> bool {
				unsafe { prop_dragger_service_show_pivot_indicator(self.to_ptr()) }
			}
			pub fn set_show_pivot_indicator(&self, value: bool) {
				unsafe { prop_set_dragger_service_show_pivot_indicator(self.to_ptr(), value) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_euler_rotation_curve {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_angles_at_time(&self, time: f64) {
				unsafe { dyn_euler_rotation_curve_get_angles_at_time(self.to_ptr(), time) }
			}
			pub fn fn_get_rotation_at_time(&self, time: f64) -> CFrame {
				unsafe { dyn_euler_rotation_curve_get_rotation_at_time(self.to_ptr(), time) }
			}
			pub fn fn_x(&self) -> Option<FloatCurve> {
				unsafe { dyn_euler_rotation_curve_x(self.to_ptr()) }
			}
			pub fn fn_y(&self) -> Option<FloatCurve> {
				unsafe { dyn_euler_rotation_curve_y(self.to_ptr()) }
			}
			pub fn fn_z(&self) -> Option<FloatCurve> {
				unsafe { dyn_euler_rotation_curve_z(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_explosion {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn blast_pressure(&self) -> f64 {
				unsafe { prop_explosion_blast_pressure(self.to_ptr()) }
			}
			pub fn set_blast_pressure(&self, value: f64) {
				unsafe { prop_set_explosion_blast_pressure(self.to_ptr(), value) }
			}
			pub fn blast_radius(&self) -> f64 {
				unsafe { prop_explosion_blast_radius(self.to_ptr()) }
			}
			pub fn set_blast_radius(&self, value: f64) {
				unsafe { prop_set_explosion_blast_radius(self.to_ptr(), value) }
			}
			pub fn destroy_joint_radius_percent(&self) -> f64 {
				unsafe { prop_explosion_destroy_joint_radius_percent(self.to_ptr()) }
			}
			pub fn set_destroy_joint_radius_percent(&self, value: f64) {
				unsafe { prop_set_explosion_destroy_joint_radius_percent(self.to_ptr(), value) }
			}
			pub fn position(&self) -> Vector3 {
				unsafe { prop_explosion_position(self.to_ptr()) }
			}
			pub fn set_position(&self, value: Vector3) {
				unsafe { prop_set_explosion_position(self.to_ptr(), value) }
			}
			pub fn time_scale(&self) -> f64 {
				unsafe { prop_explosion_time_scale(self.to_ptr()) }
			}
			pub fn set_time_scale(&self, value: f64) {
				unsafe { prop_set_explosion_time_scale(self.to_ptr(), value) }
			}
			pub fn visible(&self) -> bool {
				unsafe { prop_explosion_visible(self.to_ptr()) }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { prop_set_explosion_visible(self.to_ptr(), value) }
			}
			pub fn on_hit<F: 'static + Fn(Option<BasePart>, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_explosion_hit(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_face_instance {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
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
				unsafe { prop_decal_color_3(self.to_ptr()) }
			}
			pub fn set_color_3(&self, value: Color3) {
				unsafe { prop_set_decal_color_3(self.to_ptr(), value) }
			}
			pub fn local_transparency_modifier(&self) -> f64 {
				unsafe { prop_decal_local_transparency_modifier(self.to_ptr()) }
			}
			pub fn set_local_transparency_modifier(&self, value: f64) {
				unsafe { prop_set_decal_local_transparency_modifier(self.to_ptr(), value) }
			}
			pub fn shiny(&self) -> f64 {
				unsafe { prop_decal_shiny(self.to_ptr()) }
			}
			pub fn set_shiny(&self, value: f64) {
				unsafe { prop_set_decal_shiny(self.to_ptr(), value) }
			}
			pub fn specular(&self) -> f64 {
				unsafe { prop_decal_specular(self.to_ptr()) }
			}
			pub fn set_specular(&self, value: f64) {
				unsafe { prop_set_decal_specular(self.to_ptr(), value) }
			}
			pub fn texture(&self) -> Content {
				unsafe { prop_decal_texture(self.to_ptr()) }
			}
			pub fn set_texture(&self, value: Content) {
				unsafe { prop_set_decal_texture(self.to_ptr(), value) }
			}
			pub fn transparency(&self) -> f64 {
				unsafe { prop_decal_transparency(self.to_ptr()) }
			}
			pub fn set_transparency(&self, value: f64) {
				unsafe { prop_set_decal_transparency(self.to_ptr(), value) }
			}
			pub fn z_index(&self) -> f64 {
				unsafe { prop_decal_z_index(self.to_ptr()) }
			}
			pub fn set_z_index(&self, value: f64) {
				unsafe { prop_set_decal_z_index(self.to_ptr(), value) }
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
				unsafe { prop_texture_offset_studs_u(self.to_ptr()) }
			}
			pub fn set_offset_studs_u(&self, value: f64) {
				unsafe { prop_set_texture_offset_studs_u(self.to_ptr(), value) }
			}
			pub fn offset_studs_v(&self) -> f64 {
				unsafe { prop_texture_offset_studs_v(self.to_ptr()) }
			}
			pub fn set_offset_studs_v(&self, value: f64) {
				unsafe { prop_set_texture_offset_studs_v(self.to_ptr(), value) }
			}
			pub fn studs_per_tile_u(&self) -> f64 {
				unsafe { prop_texture_studs_per_tile_u(self.to_ptr()) }
			}
			pub fn set_studs_per_tile_u(&self, value: f64) {
				unsafe { prop_set_texture_studs_per_tile_u(self.to_ptr(), value) }
			}
			pub fn studs_per_tile_v(&self) -> f64 {
				unsafe { prop_texture_studs_per_tile_v(self.to_ptr()) }
			}
			pub fn set_studs_per_tile_v(&self, value: f64) {
				unsafe { prop_set_texture_studs_per_tile_v(self.to_ptr(), value) }
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
		impl_instance_exclusive!($name);
		impl $name {
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_fire {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn color(&self) -> Color3 {
				unsafe { prop_fire_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { prop_set_fire_color(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_fire_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_fire_enabled(self.to_ptr(), value) }
			}
			pub fn heat(&self) -> f64 {
				unsafe { prop_fire_heat(self.to_ptr()) }
			}
			pub fn set_heat(&self, value: f64) {
				unsafe { prop_set_fire_heat(self.to_ptr(), value) }
			}
			pub fn secondary_color(&self) -> Color3 {
				unsafe { prop_fire_secondary_color(self.to_ptr()) }
			}
			pub fn set_secondary_color(&self, value: Color3) {
				unsafe { prop_set_fire_secondary_color(self.to_ptr(), value) }
			}
			pub fn size(&self) -> f64 {
				unsafe { prop_fire_size(self.to_ptr()) }
			}
			pub fn set_size(&self, value: f64) {
				unsafe { prop_set_fire_size(self.to_ptr(), value) }
			}
			pub fn time_scale(&self) -> f64 {
				unsafe { prop_fire_time_scale(self.to_ptr()) }
			}
			pub fn set_time_scale(&self, value: f64) {
				unsafe { prop_set_fire_time_scale(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_float_curve {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn length(&self) -> f64 {
				unsafe { prop_float_curve_length(self.to_ptr()) }
			}
			pub fn fn_get_key_at_index(&self, index: f64) -> FloatCurveKey {
				unsafe { dyn_float_curve_get_key_at_index(self.to_ptr(), index) }
			}
			pub fn fn_get_key_indices_at_time(&self, time: f64) {
				unsafe { dyn_float_curve_get_key_indices_at_time(self.to_ptr(), time) }
			}
			pub fn fn_get_keys(&self) {
				unsafe { dyn_float_curve_get_keys(self.to_ptr()) }
			}
			pub fn fn_get_value_at_time(&self, time: f64) {
				unsafe { dyn_float_curve_get_value_at_time(self.to_ptr(), time) }
			}
			pub fn fn_insert_key(&self, key: FloatCurveKey) {
				unsafe { dyn_float_curve_insert_key(self.to_ptr(), key) }
			}
			pub fn fn_remove_key_at_index(&self, starting_index: f64, count: f64) -> f64 {
				unsafe { dyn_float_curve_remove_key_at_index(self.to_ptr(), starting_index, count) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_folder {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_force_field {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn visible(&self) -> bool {
				unsafe { prop_force_field_visible(self.to_ptr()) }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { prop_set_force_field_visible(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_game_pass_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_player_has_pass(&self, player: Option<Player>, game_pass_id: f64) -> bool {
				unsafe { dyn_game_pass_service_player_has_pass(self.to_ptr(), player, game_pass_id) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_get_text_bounds_params {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn font(&self) -> Font {
				unsafe { prop_get_text_bounds_params_font(self.to_ptr()) }
			}
			pub fn set_font(&self, value: Font) {
				unsafe { prop_set_get_text_bounds_params_font(self.to_ptr(), value) }
			}
			pub fn size(&self) -> f64 {
				unsafe { prop_get_text_bounds_params_size(self.to_ptr()) }
			}
			pub fn set_size(&self, value: f64) {
				unsafe { prop_set_get_text_bounds_params_size(self.to_ptr(), value) }
			}
			pub fn text(&self) -> String {
				unsafe { prop_get_text_bounds_params_text(self.to_ptr()) }
			}
			pub fn set_text(&self, value: &str) {
				unsafe { prop_set_get_text_bounds_params_text(self.to_ptr(), value) }
			}
			pub fn width(&self) -> f64 {
				unsafe { prop_get_text_bounds_params_width(self.to_ptr()) }
			}
			pub fn set_width(&self, value: f64) {
				unsafe { prop_set_get_text_bounds_params_width(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_global_data_store {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_on_update(&self, key: &str, callback: Function) -> RbxScriptConnection {
				unsafe { dyn_global_data_store_on_update(self.to_ptr(), key, callback) }
			}
			pub fn fn_get_async(&self, key: &str) {
				unsafe { dyn_global_data_store_get_async(self.to_ptr(), key) }
			}
			pub fn fn_remove_async(&self, key: &str) {
				unsafe { dyn_global_data_store_remove_async(self.to_ptr(), key) }
			}
			pub fn fn_update_async(&self, key: &str, transform_function: Function) {
				unsafe { dyn_global_data_store_update_async(self.to_ptr(), key, transform_function) }
			}
		}
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
				unsafe { dyn_data_store_get_version_async(self.to_ptr(), key, version) }
			}
			pub fn fn_list_keys_async(&self, prefix: &str, page_size: f64) -> Option<DataStoreKeyPages> {
				unsafe { dyn_data_store_list_keys_async(self.to_ptr(), prefix, page_size) }
			}
			pub fn fn_remove_version_async(&self, key: &str, version: &str) {
				unsafe { dyn_data_store_remove_version_async(self.to_ptr(), key, version) }
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
macro_rules! impl_group_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_allies_async(&self, group_id: f64) -> Option<StandardPages> {
				unsafe { dyn_group_service_get_allies_async(self.to_ptr(), group_id) }
			}
			pub fn fn_get_enemies_async(&self, group_id: f64) -> Option<StandardPages> {
				unsafe { dyn_group_service_get_enemies_async(self.to_ptr(), group_id) }
			}
			pub fn fn_get_group_info_async(&self, group_id: f64) {
				unsafe { dyn_group_service_get_group_info_async(self.to_ptr(), group_id) }
			}
			pub fn fn_get_groups_async(&self, user_id: f64) {
				unsafe { dyn_group_service_get_groups_async(self.to_ptr(), user_id) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_gui_base {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
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
				unsafe { prop_gui_base_2_d_absolute_position(self.to_ptr()) }
			}
			pub fn absolute_rotation(&self) -> f64 {
				unsafe { prop_gui_base_2_d_absolute_rotation(self.to_ptr()) }
			}
			pub fn absolute_size(&self) -> Vector2 {
				unsafe { prop_gui_base_2_d_absolute_size(self.to_ptr()) }
			}
			pub fn auto_localize(&self) -> bool {
				unsafe { prop_gui_base_2_d_auto_localize(self.to_ptr()) }
			}
			pub fn set_auto_localize(&self, value: bool) {
				unsafe { prop_set_gui_base_2_d_auto_localize(self.to_ptr(), value) }
			}
			pub fn localize(&self) -> bool {
				unsafe { prop_gui_base_2_d_localize(self.to_ptr()) }
			}
			pub fn set_localize(&self, value: bool) {
				unsafe { prop_set_gui_base_2_d_localize(self.to_ptr(), value) }
			}
			pub fn root_localization_table(&self) -> Option<LocalizationTable> {
				unsafe { prop_gui_base_2_d_root_localization_table(self.to_ptr()) }
			}
			pub fn set_root_localization_table(&self, value: Option<LocalizationTable>) {
				unsafe { prop_set_gui_base_2_d_root_localization_table(self.to_ptr(), value) }
			}
			pub fn selection_group(&self) -> bool {
				unsafe { prop_gui_base_2_d_selection_group(self.to_ptr()) }
			}
			pub fn set_selection_group(&self, value: bool) {
				unsafe { prop_set_gui_base_2_d_selection_group(self.to_ptr(), value) }
			}
			pub fn on_selection_changed<F: 'static + Fn(bool, Option<GuiObject>, Option<GuiObject>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_base_2_d_selection_changed(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_gui_object_active(self.to_ptr()) }
			}
			pub fn set_active(&self, value: bool) {
				unsafe { prop_set_gui_object_active(self.to_ptr(), value) }
			}
			pub fn anchor_point(&self) -> Vector2 {
				unsafe { prop_gui_object_anchor_point(self.to_ptr()) }
			}
			pub fn set_anchor_point(&self, value: Vector2) {
				unsafe { prop_set_gui_object_anchor_point(self.to_ptr(), value) }
			}
			pub fn background_color(&self) -> BrickColor {
				unsafe { prop_gui_object_background_color(self.to_ptr()) }
			}
			pub fn set_background_color(&self, value: BrickColor) {
				unsafe { prop_set_gui_object_background_color(self.to_ptr(), value) }
			}
			pub fn background_color_3(&self) -> Color3 {
				unsafe { prop_gui_object_background_color_3(self.to_ptr()) }
			}
			pub fn set_background_color_3(&self, value: Color3) {
				unsafe { prop_set_gui_object_background_color_3(self.to_ptr(), value) }
			}
			pub fn background_transparency(&self) -> f64 {
				unsafe { prop_gui_object_background_transparency(self.to_ptr()) }
			}
			pub fn set_background_transparency(&self, value: f64) {
				unsafe { prop_set_gui_object_background_transparency(self.to_ptr(), value) }
			}
			pub fn border_color(&self) -> BrickColor {
				unsafe { prop_gui_object_border_color(self.to_ptr()) }
			}
			pub fn set_border_color(&self, value: BrickColor) {
				unsafe { prop_set_gui_object_border_color(self.to_ptr(), value) }
			}
			pub fn border_color_3(&self) -> Color3 {
				unsafe { prop_gui_object_border_color_3(self.to_ptr()) }
			}
			pub fn set_border_color_3(&self, value: Color3) {
				unsafe { prop_set_gui_object_border_color_3(self.to_ptr(), value) }
			}
			pub fn border_size_pixel(&self) -> f64 {
				unsafe { prop_gui_object_border_size_pixel(self.to_ptr()) }
			}
			pub fn set_border_size_pixel(&self, value: f64) {
				unsafe { prop_set_gui_object_border_size_pixel(self.to_ptr(), value) }
			}
			pub fn clips_descendants(&self) -> bool {
				unsafe { prop_gui_object_clips_descendants(self.to_ptr()) }
			}
			pub fn set_clips_descendants(&self, value: bool) {
				unsafe { prop_set_gui_object_clips_descendants(self.to_ptr(), value) }
			}
			pub fn draggable(&self) -> bool {
				unsafe { prop_gui_object_draggable(self.to_ptr()) }
			}
			pub fn set_draggable(&self, value: bool) {
				unsafe { prop_set_gui_object_draggable(self.to_ptr(), value) }
			}
			pub fn layout_order(&self) -> f64 {
				unsafe { prop_gui_object_layout_order(self.to_ptr()) }
			}
			pub fn set_layout_order(&self, value: f64) {
				unsafe { prop_set_gui_object_layout_order(self.to_ptr(), value) }
			}
			pub fn next_selection_down(&self) -> Option<GuiObject> {
				unsafe { prop_gui_object_next_selection_down(self.to_ptr()) }
			}
			pub fn set_next_selection_down(&self, value: Option<GuiObject>) {
				unsafe { prop_set_gui_object_next_selection_down(self.to_ptr(), value) }
			}
			pub fn next_selection_left(&self) -> Option<GuiObject> {
				unsafe { prop_gui_object_next_selection_left(self.to_ptr()) }
			}
			pub fn set_next_selection_left(&self, value: Option<GuiObject>) {
				unsafe { prop_set_gui_object_next_selection_left(self.to_ptr(), value) }
			}
			pub fn next_selection_right(&self) -> Option<GuiObject> {
				unsafe { prop_gui_object_next_selection_right(self.to_ptr()) }
			}
			pub fn set_next_selection_right(&self, value: Option<GuiObject>) {
				unsafe { prop_set_gui_object_next_selection_right(self.to_ptr(), value) }
			}
			pub fn next_selection_up(&self) -> Option<GuiObject> {
				unsafe { prop_gui_object_next_selection_up(self.to_ptr()) }
			}
			pub fn set_next_selection_up(&self, value: Option<GuiObject>) {
				unsafe { prop_set_gui_object_next_selection_up(self.to_ptr(), value) }
			}
			pub fn position(&self) -> UDim2 {
				unsafe { prop_gui_object_position(self.to_ptr()) }
			}
			pub fn set_position(&self, value: UDim2) {
				unsafe { prop_set_gui_object_position(self.to_ptr(), value) }
			}
			pub fn rotation(&self) -> f64 {
				unsafe { prop_gui_object_rotation(self.to_ptr()) }
			}
			pub fn set_rotation(&self, value: f64) {
				unsafe { prop_set_gui_object_rotation(self.to_ptr(), value) }
			}
			pub fn selectable(&self) -> bool {
				unsafe { prop_gui_object_selectable(self.to_ptr()) }
			}
			pub fn set_selectable(&self, value: bool) {
				unsafe { prop_set_gui_object_selectable(self.to_ptr(), value) }
			}
			pub fn selection_image_object(&self) -> Option<GuiObject> {
				unsafe { prop_gui_object_selection_image_object(self.to_ptr()) }
			}
			pub fn set_selection_image_object(&self, value: Option<GuiObject>) {
				unsafe { prop_set_gui_object_selection_image_object(self.to_ptr(), value) }
			}
			pub fn selection_order(&self) -> f64 {
				unsafe { prop_gui_object_selection_order(self.to_ptr()) }
			}
			pub fn set_selection_order(&self, value: f64) {
				unsafe { prop_set_gui_object_selection_order(self.to_ptr(), value) }
			}
			pub fn size(&self) -> UDim2 {
				unsafe { prop_gui_object_size(self.to_ptr()) }
			}
			pub fn set_size(&self, value: UDim2) {
				unsafe { prop_set_gui_object_size(self.to_ptr(), value) }
			}
			pub fn transparency(&self) -> f64 {
				unsafe { prop_gui_object_transparency(self.to_ptr()) }
			}
			pub fn set_transparency(&self, value: f64) {
				unsafe { prop_set_gui_object_transparency(self.to_ptr(), value) }
			}
			pub fn visible(&self) -> bool {
				unsafe { prop_gui_object_visible(self.to_ptr()) }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { prop_set_gui_object_visible(self.to_ptr(), value) }
			}
			pub fn z_index(&self) -> f64 {
				unsafe { prop_gui_object_z_index(self.to_ptr()) }
			}
			pub fn set_z_index(&self, value: f64) {
				unsafe { prop_set_gui_object_z_index(self.to_ptr(), value) }
			}
			pub fn on_drag_begin<F: 'static + Fn(UDim2)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_drag_begin(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_drag_stopped<F: 'static + Fn(f64, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_drag_stopped(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_input_began<F: 'static + Fn(Option<InputObject>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_input_began(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_input_changed<F: 'static + Fn(Option<InputObject>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_input_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_input_ended<F: 'static + Fn(Option<InputObject>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_input_ended(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_enter<F: 'static + Fn(f64, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_mouse_enter(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_leave<F: 'static + Fn(f64, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_mouse_leave(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_moved<F: 'static + Fn(f64, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_mouse_moved(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_wheel_backward<F: 'static + Fn(f64, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_mouse_wheel_backward(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_wheel_forward<F: 'static + Fn(f64, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_mouse_wheel_forward(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_selection_gained<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_selection_gained(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_selection_lost<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_selection_lost(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_long_press<F: 'static + Fn((), ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_touch_long_press(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_pan<F: 'static + Fn((), Vector2, Vector2, ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_touch_pan(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_pinch<F: 'static + Fn((), f64, f64, ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_touch_pinch(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_rotate<F: 'static + Fn((), f64, f64, ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_touch_rotate(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_swipe<F: 'static + Fn((), f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_touch_swipe(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_tap<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_object_touch_tap(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_canvas_group_group_color_3(self.to_ptr()) }
			}
			pub fn set_group_color_3(&self, value: Color3) {
				unsafe { prop_set_canvas_group_group_color_3(self.to_ptr(), value) }
			}
			pub fn group_transparency(&self) -> f64 {
				unsafe { prop_canvas_group_group_transparency(self.to_ptr()) }
			}
			pub fn set_group_transparency(&self, value: f64) {
				unsafe { prop_set_canvas_group_group_transparency(self.to_ptr(), value) }
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
				unsafe { prop_gui_button_auto_button_color(self.to_ptr()) }
			}
			pub fn set_auto_button_color(&self, value: bool) {
				unsafe { prop_set_gui_button_auto_button_color(self.to_ptr(), value) }
			}
			pub fn modal(&self) -> bool {
				unsafe { prop_gui_button_modal(self.to_ptr()) }
			}
			pub fn set_modal(&self, value: bool) {
				unsafe { prop_set_gui_button_modal(self.to_ptr(), value) }
			}
			pub fn selected(&self) -> bool {
				unsafe { prop_gui_button_selected(self.to_ptr()) }
			}
			pub fn set_selected(&self, value: bool) {
				unsafe { prop_set_gui_button_selected(self.to_ptr(), value) }
			}
			pub fn on_activated<F: 'static + Fn(Option<InputObject>, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_button_activated(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_button_1_click<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_button_mouse_button_1_click(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_button_1_down<F: 'static + Fn(f64, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_button_mouse_button_1_down(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_button_1_up<F: 'static + Fn(f64, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_button_mouse_button_1_up(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_button_2_click<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_button_mouse_button_2_click(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_button_2_down<F: 'static + Fn(f64, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_button_mouse_button_2_down(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_button_2_up<F: 'static + Fn(f64, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_button_mouse_button_2_up(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_image_button_hover_image(self.to_ptr()) }
			}
			pub fn set_hover_image(&self, value: Content) {
				unsafe { prop_set_image_button_hover_image(self.to_ptr(), value) }
			}
			pub fn image(&self) -> Content {
				unsafe { prop_image_button_image(self.to_ptr()) }
			}
			pub fn set_image(&self, value: Content) {
				unsafe { prop_set_image_button_image(self.to_ptr(), value) }
			}
			pub fn image_color_3(&self) -> Color3 {
				unsafe { prop_image_button_image_color_3(self.to_ptr()) }
			}
			pub fn set_image_color_3(&self, value: Color3) {
				unsafe { prop_set_image_button_image_color_3(self.to_ptr(), value) }
			}
			pub fn image_rect_offset(&self) -> Vector2 {
				unsafe { prop_image_button_image_rect_offset(self.to_ptr()) }
			}
			pub fn set_image_rect_offset(&self, value: Vector2) {
				unsafe { prop_set_image_button_image_rect_offset(self.to_ptr(), value) }
			}
			pub fn image_rect_size(&self) -> Vector2 {
				unsafe { prop_image_button_image_rect_size(self.to_ptr()) }
			}
			pub fn set_image_rect_size(&self, value: Vector2) {
				unsafe { prop_set_image_button_image_rect_size(self.to_ptr(), value) }
			}
			pub fn image_transparency(&self) -> f64 {
				unsafe { prop_image_button_image_transparency(self.to_ptr()) }
			}
			pub fn set_image_transparency(&self, value: f64) {
				unsafe { prop_set_image_button_image_transparency(self.to_ptr(), value) }
			}
			pub fn is_loaded(&self) -> bool {
				unsafe { prop_image_button_is_loaded(self.to_ptr()) }
			}
			pub fn pressed_image(&self) -> Content {
				unsafe { prop_image_button_pressed_image(self.to_ptr()) }
			}
			pub fn set_pressed_image(&self, value: Content) {
				unsafe { prop_set_image_button_pressed_image(self.to_ptr(), value) }
			}
			pub fn slice_center(&self) -> Rect {
				unsafe { prop_image_button_slice_center(self.to_ptr()) }
			}
			pub fn set_slice_center(&self, value: Rect) {
				unsafe { prop_set_image_button_slice_center(self.to_ptr(), value) }
			}
			pub fn slice_scale(&self) -> f64 {
				unsafe { prop_image_button_slice_scale(self.to_ptr()) }
			}
			pub fn set_slice_scale(&self, value: f64) {
				unsafe { prop_set_image_button_slice_scale(self.to_ptr(), value) }
			}
			pub fn tile_size(&self) -> UDim2 {
				unsafe { prop_image_button_tile_size(self.to_ptr()) }
			}
			pub fn set_tile_size(&self, value: UDim2) {
				unsafe { prop_set_image_button_tile_size(self.to_ptr(), value) }
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
				unsafe { prop_text_button_content_text(self.to_ptr()) }
			}
			pub fn font_face(&self) -> Font {
				unsafe { prop_text_button_font_face(self.to_ptr()) }
			}
			pub fn set_font_face(&self, value: Font) {
				unsafe { prop_set_text_button_font_face(self.to_ptr(), value) }
			}
			pub fn line_height(&self) -> f64 {
				unsafe { prop_text_button_line_height(self.to_ptr()) }
			}
			pub fn set_line_height(&self, value: f64) {
				unsafe { prop_set_text_button_line_height(self.to_ptr(), value) }
			}
			pub fn localized_text(&self) -> String {
				unsafe { prop_text_button_localized_text(self.to_ptr()) }
			}
			pub fn max_visible_graphemes(&self) -> f64 {
				unsafe { prop_text_button_max_visible_graphemes(self.to_ptr()) }
			}
			pub fn set_max_visible_graphemes(&self, value: f64) {
				unsafe { prop_set_text_button_max_visible_graphemes(self.to_ptr(), value) }
			}
			pub fn rich_text(&self) -> bool {
				unsafe { prop_text_button_rich_text(self.to_ptr()) }
			}
			pub fn set_rich_text(&self, value: bool) {
				unsafe { prop_set_text_button_rich_text(self.to_ptr(), value) }
			}
			pub fn text(&self) -> String {
				unsafe { prop_text_button_text(self.to_ptr()) }
			}
			pub fn set_text(&self, value: &str) {
				unsafe { prop_set_text_button_text(self.to_ptr(), value) }
			}
			pub fn text_bounds(&self) -> Vector2 {
				unsafe { prop_text_button_text_bounds(self.to_ptr()) }
			}
			pub fn text_color(&self) -> BrickColor {
				unsafe { prop_text_button_text_color(self.to_ptr()) }
			}
			pub fn set_text_color(&self, value: BrickColor) {
				unsafe { prop_set_text_button_text_color(self.to_ptr(), value) }
			}
			pub fn text_color_3(&self) -> Color3 {
				unsafe { prop_text_button_text_color_3(self.to_ptr()) }
			}
			pub fn set_text_color_3(&self, value: Color3) {
				unsafe { prop_set_text_button_text_color_3(self.to_ptr(), value) }
			}
			pub fn text_fits(&self) -> bool {
				unsafe { prop_text_button_text_fits(self.to_ptr()) }
			}
			pub fn text_scaled(&self) -> bool {
				unsafe { prop_text_button_text_scaled(self.to_ptr()) }
			}
			pub fn set_text_scaled(&self, value: bool) {
				unsafe { prop_set_text_button_text_scaled(self.to_ptr(), value) }
			}
			pub fn text_size(&self) -> f64 {
				unsafe { prop_text_button_text_size(self.to_ptr()) }
			}
			pub fn set_text_size(&self, value: f64) {
				unsafe { prop_set_text_button_text_size(self.to_ptr(), value) }
			}
			pub fn text_stroke_color_3(&self) -> Color3 {
				unsafe { prop_text_button_text_stroke_color_3(self.to_ptr()) }
			}
			pub fn set_text_stroke_color_3(&self, value: Color3) {
				unsafe { prop_set_text_button_text_stroke_color_3(self.to_ptr(), value) }
			}
			pub fn text_stroke_transparency(&self) -> f64 {
				unsafe { prop_text_button_text_stroke_transparency(self.to_ptr()) }
			}
			pub fn set_text_stroke_transparency(&self, value: f64) {
				unsafe { prop_set_text_button_text_stroke_transparency(self.to_ptr(), value) }
			}
			pub fn text_transparency(&self) -> f64 {
				unsafe { prop_text_button_text_transparency(self.to_ptr()) }
			}
			pub fn set_text_transparency(&self, value: f64) {
				unsafe { prop_set_text_button_text_transparency(self.to_ptr(), value) }
			}
			pub fn text_wrap(&self) -> bool {
				unsafe { prop_text_button_text_wrap(self.to_ptr()) }
			}
			pub fn set_text_wrap(&self, value: bool) {
				unsafe { prop_set_text_button_text_wrap(self.to_ptr(), value) }
			}
			pub fn text_wrapped(&self) -> bool {
				unsafe { prop_text_button_text_wrapped(self.to_ptr()) }
			}
			pub fn set_text_wrapped(&self, value: bool) {
				unsafe { prop_set_text_button_text_wrapped(self.to_ptr(), value) }
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
				unsafe { prop_image_label_image(self.to_ptr()) }
			}
			pub fn set_image(&self, value: Content) {
				unsafe { prop_set_image_label_image(self.to_ptr(), value) }
			}
			pub fn image_color_3(&self) -> Color3 {
				unsafe { prop_image_label_image_color_3(self.to_ptr()) }
			}
			pub fn set_image_color_3(&self, value: Color3) {
				unsafe { prop_set_image_label_image_color_3(self.to_ptr(), value) }
			}
			pub fn image_rect_offset(&self) -> Vector2 {
				unsafe { prop_image_label_image_rect_offset(self.to_ptr()) }
			}
			pub fn set_image_rect_offset(&self, value: Vector2) {
				unsafe { prop_set_image_label_image_rect_offset(self.to_ptr(), value) }
			}
			pub fn image_rect_size(&self) -> Vector2 {
				unsafe { prop_image_label_image_rect_size(self.to_ptr()) }
			}
			pub fn set_image_rect_size(&self, value: Vector2) {
				unsafe { prop_set_image_label_image_rect_size(self.to_ptr(), value) }
			}
			pub fn image_transparency(&self) -> f64 {
				unsafe { prop_image_label_image_transparency(self.to_ptr()) }
			}
			pub fn set_image_transparency(&self, value: f64) {
				unsafe { prop_set_image_label_image_transparency(self.to_ptr(), value) }
			}
			pub fn is_loaded(&self) -> bool {
				unsafe { prop_image_label_is_loaded(self.to_ptr()) }
			}
			pub fn slice_center(&self) -> Rect {
				unsafe { prop_image_label_slice_center(self.to_ptr()) }
			}
			pub fn set_slice_center(&self, value: Rect) {
				unsafe { prop_set_image_label_slice_center(self.to_ptr(), value) }
			}
			pub fn slice_scale(&self) -> f64 {
				unsafe { prop_image_label_slice_scale(self.to_ptr()) }
			}
			pub fn set_slice_scale(&self, value: f64) {
				unsafe { prop_set_image_label_slice_scale(self.to_ptr(), value) }
			}
			pub fn tile_size(&self) -> UDim2 {
				unsafe { prop_image_label_tile_size(self.to_ptr()) }
			}
			pub fn set_tile_size(&self, value: UDim2) {
				unsafe { prop_set_image_label_tile_size(self.to_ptr(), value) }
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
				unsafe { prop_text_label_content_text(self.to_ptr()) }
			}
			pub fn font_face(&self) -> Font {
				unsafe { prop_text_label_font_face(self.to_ptr()) }
			}
			pub fn set_font_face(&self, value: Font) {
				unsafe { prop_set_text_label_font_face(self.to_ptr(), value) }
			}
			pub fn line_height(&self) -> f64 {
				unsafe { prop_text_label_line_height(self.to_ptr()) }
			}
			pub fn set_line_height(&self, value: f64) {
				unsafe { prop_set_text_label_line_height(self.to_ptr(), value) }
			}
			pub fn localized_text(&self) -> String {
				unsafe { prop_text_label_localized_text(self.to_ptr()) }
			}
			pub fn max_visible_graphemes(&self) -> f64 {
				unsafe { prop_text_label_max_visible_graphemes(self.to_ptr()) }
			}
			pub fn set_max_visible_graphemes(&self, value: f64) {
				unsafe { prop_set_text_label_max_visible_graphemes(self.to_ptr(), value) }
			}
			pub fn rich_text(&self) -> bool {
				unsafe { prop_text_label_rich_text(self.to_ptr()) }
			}
			pub fn set_rich_text(&self, value: bool) {
				unsafe { prop_set_text_label_rich_text(self.to_ptr(), value) }
			}
			pub fn text(&self) -> String {
				unsafe { prop_text_label_text(self.to_ptr()) }
			}
			pub fn set_text(&self, value: &str) {
				unsafe { prop_set_text_label_text(self.to_ptr(), value) }
			}
			pub fn text_bounds(&self) -> Vector2 {
				unsafe { prop_text_label_text_bounds(self.to_ptr()) }
			}
			pub fn text_color(&self) -> BrickColor {
				unsafe { prop_text_label_text_color(self.to_ptr()) }
			}
			pub fn set_text_color(&self, value: BrickColor) {
				unsafe { prop_set_text_label_text_color(self.to_ptr(), value) }
			}
			pub fn text_color_3(&self) -> Color3 {
				unsafe { prop_text_label_text_color_3(self.to_ptr()) }
			}
			pub fn set_text_color_3(&self, value: Color3) {
				unsafe { prop_set_text_label_text_color_3(self.to_ptr(), value) }
			}
			pub fn text_fits(&self) -> bool {
				unsafe { prop_text_label_text_fits(self.to_ptr()) }
			}
			pub fn text_scaled(&self) -> bool {
				unsafe { prop_text_label_text_scaled(self.to_ptr()) }
			}
			pub fn set_text_scaled(&self, value: bool) {
				unsafe { prop_set_text_label_text_scaled(self.to_ptr(), value) }
			}
			pub fn text_size(&self) -> f64 {
				unsafe { prop_text_label_text_size(self.to_ptr()) }
			}
			pub fn set_text_size(&self, value: f64) {
				unsafe { prop_set_text_label_text_size(self.to_ptr(), value) }
			}
			pub fn text_stroke_color_3(&self) -> Color3 {
				unsafe { prop_text_label_text_stroke_color_3(self.to_ptr()) }
			}
			pub fn set_text_stroke_color_3(&self, value: Color3) {
				unsafe { prop_set_text_label_text_stroke_color_3(self.to_ptr(), value) }
			}
			pub fn text_stroke_transparency(&self) -> f64 {
				unsafe { prop_text_label_text_stroke_transparency(self.to_ptr()) }
			}
			pub fn set_text_stroke_transparency(&self, value: f64) {
				unsafe { prop_set_text_label_text_stroke_transparency(self.to_ptr(), value) }
			}
			pub fn text_transparency(&self) -> f64 {
				unsafe { prop_text_label_text_transparency(self.to_ptr()) }
			}
			pub fn set_text_transparency(&self, value: f64) {
				unsafe { prop_set_text_label_text_transparency(self.to_ptr(), value) }
			}
			pub fn text_wrap(&self) -> bool {
				unsafe { prop_text_label_text_wrap(self.to_ptr()) }
			}
			pub fn set_text_wrap(&self, value: bool) {
				unsafe { prop_set_text_label_text_wrap(self.to_ptr(), value) }
			}
			pub fn text_wrapped(&self) -> bool {
				unsafe { prop_text_label_text_wrapped(self.to_ptr()) }
			}
			pub fn set_text_wrapped(&self, value: bool) {
				unsafe { prop_set_text_label_text_wrapped(self.to_ptr(), value) }
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
				unsafe { prop_scrolling_frame_absolute_canvas_size(self.to_ptr()) }
			}
			pub fn absolute_window_size(&self) -> Vector2 {
				unsafe { prop_scrolling_frame_absolute_window_size(self.to_ptr()) }
			}
			pub fn bottom_image(&self) -> Content {
				unsafe { prop_scrolling_frame_bottom_image(self.to_ptr()) }
			}
			pub fn set_bottom_image(&self, value: Content) {
				unsafe { prop_set_scrolling_frame_bottom_image(self.to_ptr(), value) }
			}
			pub fn canvas_position(&self) -> Vector2 {
				unsafe { prop_scrolling_frame_canvas_position(self.to_ptr()) }
			}
			pub fn set_canvas_position(&self, value: Vector2) {
				unsafe { prop_set_scrolling_frame_canvas_position(self.to_ptr(), value) }
			}
			pub fn canvas_size(&self) -> UDim2 {
				unsafe { prop_scrolling_frame_canvas_size(self.to_ptr()) }
			}
			pub fn set_canvas_size(&self, value: UDim2) {
				unsafe { prop_set_scrolling_frame_canvas_size(self.to_ptr(), value) }
			}
			pub fn mid_image(&self) -> Content {
				unsafe { prop_scrolling_frame_mid_image(self.to_ptr()) }
			}
			pub fn set_mid_image(&self, value: Content) {
				unsafe { prop_set_scrolling_frame_mid_image(self.to_ptr(), value) }
			}
			pub fn scroll_bar_image_color_3(&self) -> Color3 {
				unsafe { prop_scrolling_frame_scroll_bar_image_color_3(self.to_ptr()) }
			}
			pub fn set_scroll_bar_image_color_3(&self, value: Color3) {
				unsafe { prop_set_scrolling_frame_scroll_bar_image_color_3(self.to_ptr(), value) }
			}
			pub fn scroll_bar_image_transparency(&self) -> f64 {
				unsafe { prop_scrolling_frame_scroll_bar_image_transparency(self.to_ptr()) }
			}
			pub fn set_scroll_bar_image_transparency(&self, value: f64) {
				unsafe { prop_set_scrolling_frame_scroll_bar_image_transparency(self.to_ptr(), value) }
			}
			pub fn scroll_bar_thickness(&self) -> f64 {
				unsafe { prop_scrolling_frame_scroll_bar_thickness(self.to_ptr()) }
			}
			pub fn set_scroll_bar_thickness(&self, value: f64) {
				unsafe { prop_set_scrolling_frame_scroll_bar_thickness(self.to_ptr(), value) }
			}
			pub fn scrolling_enabled(&self) -> bool {
				unsafe { prop_scrolling_frame_scrolling_enabled(self.to_ptr()) }
			}
			pub fn set_scrolling_enabled(&self, value: bool) {
				unsafe { prop_set_scrolling_frame_scrolling_enabled(self.to_ptr(), value) }
			}
			pub fn top_image(&self) -> Content {
				unsafe { prop_scrolling_frame_top_image(self.to_ptr()) }
			}
			pub fn set_top_image(&self, value: Content) {
				unsafe { prop_set_scrolling_frame_top_image(self.to_ptr(), value) }
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
				unsafe { prop_text_box_clear_text_on_focus(self.to_ptr()) }
			}
			pub fn set_clear_text_on_focus(&self, value: bool) {
				unsafe { prop_set_text_box_clear_text_on_focus(self.to_ptr(), value) }
			}
			pub fn content_text(&self) -> String {
				unsafe { prop_text_box_content_text(self.to_ptr()) }
			}
			pub fn cursor_position(&self) -> f64 {
				unsafe { prop_text_box_cursor_position(self.to_ptr()) }
			}
			pub fn set_cursor_position(&self, value: f64) {
				unsafe { prop_set_text_box_cursor_position(self.to_ptr(), value) }
			}
			pub fn font_face(&self) -> Font {
				unsafe { prop_text_box_font_face(self.to_ptr()) }
			}
			pub fn set_font_face(&self, value: Font) {
				unsafe { prop_set_text_box_font_face(self.to_ptr(), value) }
			}
			pub fn line_height(&self) -> f64 {
				unsafe { prop_text_box_line_height(self.to_ptr()) }
			}
			pub fn set_line_height(&self, value: f64) {
				unsafe { prop_set_text_box_line_height(self.to_ptr(), value) }
			}
			pub fn max_visible_graphemes(&self) -> f64 {
				unsafe { prop_text_box_max_visible_graphemes(self.to_ptr()) }
			}
			pub fn set_max_visible_graphemes(&self, value: f64) {
				unsafe { prop_set_text_box_max_visible_graphemes(self.to_ptr(), value) }
			}
			pub fn multi_line(&self) -> bool {
				unsafe { prop_text_box_multi_line(self.to_ptr()) }
			}
			pub fn set_multi_line(&self, value: bool) {
				unsafe { prop_set_text_box_multi_line(self.to_ptr(), value) }
			}
			pub fn placeholder_color_3(&self) -> Color3 {
				unsafe { prop_text_box_placeholder_color_3(self.to_ptr()) }
			}
			pub fn set_placeholder_color_3(&self, value: Color3) {
				unsafe { prop_set_text_box_placeholder_color_3(self.to_ptr(), value) }
			}
			pub fn placeholder_text(&self) -> String {
				unsafe { prop_text_box_placeholder_text(self.to_ptr()) }
			}
			pub fn set_placeholder_text(&self, value: &str) {
				unsafe { prop_set_text_box_placeholder_text(self.to_ptr(), value) }
			}
			pub fn rich_text(&self) -> bool {
				unsafe { prop_text_box_rich_text(self.to_ptr()) }
			}
			pub fn set_rich_text(&self, value: bool) {
				unsafe { prop_set_text_box_rich_text(self.to_ptr(), value) }
			}
			pub fn selection_start(&self) -> f64 {
				unsafe { prop_text_box_selection_start(self.to_ptr()) }
			}
			pub fn set_selection_start(&self, value: f64) {
				unsafe { prop_set_text_box_selection_start(self.to_ptr(), value) }
			}
			pub fn show_native_input(&self) -> bool {
				unsafe { prop_text_box_show_native_input(self.to_ptr()) }
			}
			pub fn set_show_native_input(&self, value: bool) {
				unsafe { prop_set_text_box_show_native_input(self.to_ptr(), value) }
			}
			pub fn text(&self) -> String {
				unsafe { prop_text_box_text(self.to_ptr()) }
			}
			pub fn set_text(&self, value: &str) {
				unsafe { prop_set_text_box_text(self.to_ptr(), value) }
			}
			pub fn text_bounds(&self) -> Vector2 {
				unsafe { prop_text_box_text_bounds(self.to_ptr()) }
			}
			pub fn text_color(&self) -> BrickColor {
				unsafe { prop_text_box_text_color(self.to_ptr()) }
			}
			pub fn set_text_color(&self, value: BrickColor) {
				unsafe { prop_set_text_box_text_color(self.to_ptr(), value) }
			}
			pub fn text_color_3(&self) -> Color3 {
				unsafe { prop_text_box_text_color_3(self.to_ptr()) }
			}
			pub fn set_text_color_3(&self, value: Color3) {
				unsafe { prop_set_text_box_text_color_3(self.to_ptr(), value) }
			}
			pub fn text_editable(&self) -> bool {
				unsafe { prop_text_box_text_editable(self.to_ptr()) }
			}
			pub fn set_text_editable(&self, value: bool) {
				unsafe { prop_set_text_box_text_editable(self.to_ptr(), value) }
			}
			pub fn text_fits(&self) -> bool {
				unsafe { prop_text_box_text_fits(self.to_ptr()) }
			}
			pub fn text_scaled(&self) -> bool {
				unsafe { prop_text_box_text_scaled(self.to_ptr()) }
			}
			pub fn set_text_scaled(&self, value: bool) {
				unsafe { prop_set_text_box_text_scaled(self.to_ptr(), value) }
			}
			pub fn text_size(&self) -> f64 {
				unsafe { prop_text_box_text_size(self.to_ptr()) }
			}
			pub fn set_text_size(&self, value: f64) {
				unsafe { prop_set_text_box_text_size(self.to_ptr(), value) }
			}
			pub fn text_stroke_color_3(&self) -> Color3 {
				unsafe { prop_text_box_text_stroke_color_3(self.to_ptr()) }
			}
			pub fn set_text_stroke_color_3(&self, value: Color3) {
				unsafe { prop_set_text_box_text_stroke_color_3(self.to_ptr(), value) }
			}
			pub fn text_stroke_transparency(&self) -> f64 {
				unsafe { prop_text_box_text_stroke_transparency(self.to_ptr()) }
			}
			pub fn set_text_stroke_transparency(&self, value: f64) {
				unsafe { prop_set_text_box_text_stroke_transparency(self.to_ptr(), value) }
			}
			pub fn text_transparency(&self) -> f64 {
				unsafe { prop_text_box_text_transparency(self.to_ptr()) }
			}
			pub fn set_text_transparency(&self, value: f64) {
				unsafe { prop_set_text_box_text_transparency(self.to_ptr(), value) }
			}
			pub fn text_wrap(&self) -> bool {
				unsafe { prop_text_box_text_wrap(self.to_ptr()) }
			}
			pub fn set_text_wrap(&self, value: bool) {
				unsafe { prop_set_text_box_text_wrap(self.to_ptr(), value) }
			}
			pub fn text_wrapped(&self) -> bool {
				unsafe { prop_text_box_text_wrapped(self.to_ptr()) }
			}
			pub fn set_text_wrapped(&self, value: bool) {
				unsafe { prop_set_text_box_text_wrapped(self.to_ptr(), value) }
			}
			pub fn fn_capture_focus(&self) {
				unsafe { dyn_text_box_capture_focus(self.to_ptr()) }
			}
			pub fn fn_is_focused(&self) -> bool {
				unsafe { dyn_text_box_is_focused(self.to_ptr()) }
			}
			pub fn fn_release_focus(&self, submitted: bool) {
				unsafe { dyn_text_box_release_focus(self.to_ptr(), submitted) }
			}
			pub fn on_focus_lost<F: 'static + Fn(bool, Option<InputObject>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_text_box_focus_lost(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_focused<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_text_box_focused(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_return_pressed_from_on_screen_keyboard<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_text_box_return_pressed_from_on_screen_keyboard(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_video_frame_is_loaded(self.to_ptr()) }
			}
			pub fn looped(&self) -> bool {
				unsafe { prop_video_frame_looped(self.to_ptr()) }
			}
			pub fn set_looped(&self, value: bool) {
				unsafe { prop_set_video_frame_looped(self.to_ptr(), value) }
			}
			pub fn playing(&self) -> bool {
				unsafe { prop_video_frame_playing(self.to_ptr()) }
			}
			pub fn set_playing(&self, value: bool) {
				unsafe { prop_set_video_frame_playing(self.to_ptr(), value) }
			}
			pub fn resolution(&self) -> Vector2 {
				unsafe { prop_video_frame_resolution(self.to_ptr()) }
			}
			pub fn time_length(&self) -> f64 {
				unsafe { prop_video_frame_time_length(self.to_ptr()) }
			}
			pub fn time_position(&self) -> f64 {
				unsafe { prop_video_frame_time_position(self.to_ptr()) }
			}
			pub fn set_time_position(&self, value: f64) {
				unsafe { prop_set_video_frame_time_position(self.to_ptr(), value) }
			}
			pub fn video(&self) -> Content {
				unsafe { prop_video_frame_video(self.to_ptr()) }
			}
			pub fn set_video(&self, value: Content) {
				unsafe { prop_set_video_frame_video(self.to_ptr(), value) }
			}
			pub fn volume(&self) -> f64 {
				unsafe { prop_video_frame_volume(self.to_ptr()) }
			}
			pub fn set_volume(&self, value: f64) {
				unsafe { prop_set_video_frame_volume(self.to_ptr(), value) }
			}
			pub fn fn_pause(&self) {
				unsafe { dyn_video_frame_pause(self.to_ptr()) }
			}
			pub fn fn_play(&self) {
				unsafe { dyn_video_frame_play(self.to_ptr()) }
			}
			pub fn on_did_loop<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_video_frame_did_loop(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_ended<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_video_frame_ended(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_loaded<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_video_frame_loaded(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_paused<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_video_frame_paused(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_played<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_video_frame_played(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_viewport_frame_ambient(self.to_ptr()) }
			}
			pub fn set_ambient(&self, value: Color3) {
				unsafe { prop_set_viewport_frame_ambient(self.to_ptr(), value) }
			}
			pub fn current_camera(&self) -> Option<Camera> {
				unsafe { prop_viewport_frame_current_camera(self.to_ptr()) }
			}
			pub fn set_current_camera(&self, value: Option<Camera>) {
				unsafe { prop_set_viewport_frame_current_camera(self.to_ptr(), value) }
			}
			pub fn image_color_3(&self) -> Color3 {
				unsafe { prop_viewport_frame_image_color_3(self.to_ptr()) }
			}
			pub fn set_image_color_3(&self, value: Color3) {
				unsafe { prop_set_viewport_frame_image_color_3(self.to_ptr(), value) }
			}
			pub fn image_transparency(&self) -> f64 {
				unsafe { prop_viewport_frame_image_transparency(self.to_ptr()) }
			}
			pub fn set_image_transparency(&self, value: f64) {
				unsafe { prop_set_viewport_frame_image_transparency(self.to_ptr(), value) }
			}
			pub fn light_color(&self) -> Color3 {
				unsafe { prop_viewport_frame_light_color(self.to_ptr()) }
			}
			pub fn set_light_color(&self, value: Color3) {
				unsafe { prop_set_viewport_frame_light_color(self.to_ptr(), value) }
			}
			pub fn light_direction(&self) -> Vector3 {
				unsafe { prop_viewport_frame_light_direction(self.to_ptr()) }
			}
			pub fn set_light_direction(&self, value: Vector3) {
				unsafe { prop_set_viewport_frame_light_direction(self.to_ptr(), value) }
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
				unsafe { prop_layer_collector_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_layer_collector_enabled(self.to_ptr(), value) }
			}
			pub fn reset_on_spawn(&self) -> bool {
				unsafe { prop_layer_collector_reset_on_spawn(self.to_ptr()) }
			}
			pub fn set_reset_on_spawn(&self, value: bool) {
				unsafe { prop_set_layer_collector_reset_on_spawn(self.to_ptr(), value) }
			}
			pub fn fn_get_layout_node_tree(&self) {
				unsafe { dyn_layer_collector_get_layout_node_tree(self.to_ptr()) }
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
				unsafe { prop_billboard_gui_active(self.to_ptr()) }
			}
			pub fn set_active(&self, value: bool) {
				unsafe { prop_set_billboard_gui_active(self.to_ptr(), value) }
			}
			pub fn adornee(&self) -> Option<Instance> {
				unsafe { prop_billboard_gui_adornee(self.to_ptr()) }
			}
			pub fn set_adornee(&self, value: Option<Instance>) {
				unsafe { prop_set_billboard_gui_adornee(self.to_ptr(), value) }
			}
			pub fn always_on_top(&self) -> bool {
				unsafe { prop_billboard_gui_always_on_top(self.to_ptr()) }
			}
			pub fn set_always_on_top(&self, value: bool) {
				unsafe { prop_set_billboard_gui_always_on_top(self.to_ptr(), value) }
			}
			pub fn brightness(&self) -> f64 {
				unsafe { prop_billboard_gui_brightness(self.to_ptr()) }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { prop_set_billboard_gui_brightness(self.to_ptr(), value) }
			}
			pub fn clips_descendants(&self) -> bool {
				unsafe { prop_billboard_gui_clips_descendants(self.to_ptr()) }
			}
			pub fn set_clips_descendants(&self, value: bool) {
				unsafe { prop_set_billboard_gui_clips_descendants(self.to_ptr(), value) }
			}
			pub fn current_distance(&self) -> f64 {
				unsafe { prop_billboard_gui_current_distance(self.to_ptr()) }
			}
			pub fn distance_lower_limit(&self) -> f64 {
				unsafe { prop_billboard_gui_distance_lower_limit(self.to_ptr()) }
			}
			pub fn set_distance_lower_limit(&self, value: f64) {
				unsafe { prop_set_billboard_gui_distance_lower_limit(self.to_ptr(), value) }
			}
			pub fn distance_step(&self) -> f64 {
				unsafe { prop_billboard_gui_distance_step(self.to_ptr()) }
			}
			pub fn set_distance_step(&self, value: f64) {
				unsafe { prop_set_billboard_gui_distance_step(self.to_ptr(), value) }
			}
			pub fn distance_upper_limit(&self) -> f64 {
				unsafe { prop_billboard_gui_distance_upper_limit(self.to_ptr()) }
			}
			pub fn set_distance_upper_limit(&self, value: f64) {
				unsafe { prop_set_billboard_gui_distance_upper_limit(self.to_ptr(), value) }
			}
			pub fn extents_offset(&self) -> Vector3 {
				unsafe { prop_billboard_gui_extents_offset(self.to_ptr()) }
			}
			pub fn set_extents_offset(&self, value: Vector3) {
				unsafe { prop_set_billboard_gui_extents_offset(self.to_ptr(), value) }
			}
			pub fn extents_offset_world_space(&self) -> Vector3 {
				unsafe { prop_billboard_gui_extents_offset_world_space(self.to_ptr()) }
			}
			pub fn set_extents_offset_world_space(&self, value: Vector3) {
				unsafe { prop_set_billboard_gui_extents_offset_world_space(self.to_ptr(), value) }
			}
			pub fn light_influence(&self) -> f64 {
				unsafe { prop_billboard_gui_light_influence(self.to_ptr()) }
			}
			pub fn set_light_influence(&self, value: f64) {
				unsafe { prop_set_billboard_gui_light_influence(self.to_ptr(), value) }
			}
			pub fn max_distance(&self) -> f64 {
				unsafe { prop_billboard_gui_max_distance(self.to_ptr()) }
			}
			pub fn set_max_distance(&self, value: f64) {
				unsafe { prop_set_billboard_gui_max_distance(self.to_ptr(), value) }
			}
			pub fn player_to_hide_from(&self) -> Option<Instance> {
				unsafe { prop_billboard_gui_player_to_hide_from(self.to_ptr()) }
			}
			pub fn set_player_to_hide_from(&self, value: Option<Instance>) {
				unsafe { prop_set_billboard_gui_player_to_hide_from(self.to_ptr(), value) }
			}
			pub fn size(&self) -> UDim2 {
				unsafe { prop_billboard_gui_size(self.to_ptr()) }
			}
			pub fn set_size(&self, value: UDim2) {
				unsafe { prop_set_billboard_gui_size(self.to_ptr(), value) }
			}
			pub fn size_offset(&self) -> Vector2 {
				unsafe { prop_billboard_gui_size_offset(self.to_ptr()) }
			}
			pub fn set_size_offset(&self, value: Vector2) {
				unsafe { prop_set_billboard_gui_size_offset(self.to_ptr(), value) }
			}
			pub fn studs_offset(&self) -> Vector3 {
				unsafe { prop_billboard_gui_studs_offset(self.to_ptr()) }
			}
			pub fn set_studs_offset(&self, value: Vector3) {
				unsafe { prop_set_billboard_gui_studs_offset(self.to_ptr(), value) }
			}
			pub fn studs_offset_world_space(&self) -> Vector3 {
				unsafe { prop_billboard_gui_studs_offset_world_space(self.to_ptr()) }
			}
			pub fn set_studs_offset_world_space(&self, value: Vector3) {
				unsafe { prop_set_billboard_gui_studs_offset_world_space(self.to_ptr(), value) }
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
				unsafe { prop_screen_gui_display_order(self.to_ptr()) }
			}
			pub fn set_display_order(&self, value: f64) {
				unsafe { prop_set_screen_gui_display_order(self.to_ptr(), value) }
			}
			pub fn ignore_gui_inset(&self) -> bool {
				unsafe { prop_screen_gui_ignore_gui_inset(self.to_ptr()) }
			}
			pub fn set_ignore_gui_inset(&self, value: bool) {
				unsafe { prop_set_screen_gui_ignore_gui_inset(self.to_ptr(), value) }
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
				unsafe { prop_surface_gui_active(self.to_ptr()) }
			}
			pub fn set_active(&self, value: bool) {
				unsafe { prop_set_surface_gui_active(self.to_ptr(), value) }
			}
			pub fn adornee(&self) -> Option<Instance> {
				unsafe { prop_surface_gui_adornee(self.to_ptr()) }
			}
			pub fn set_adornee(&self, value: Option<Instance>) {
				unsafe { prop_set_surface_gui_adornee(self.to_ptr(), value) }
			}
			pub fn always_on_top(&self) -> bool {
				unsafe { prop_surface_gui_always_on_top(self.to_ptr()) }
			}
			pub fn set_always_on_top(&self, value: bool) {
				unsafe { prop_set_surface_gui_always_on_top(self.to_ptr(), value) }
			}
			pub fn brightness(&self) -> f64 {
				unsafe { prop_surface_gui_brightness(self.to_ptr()) }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { prop_set_surface_gui_brightness(self.to_ptr(), value) }
			}
			pub fn canvas_size(&self) -> Vector2 {
				unsafe { prop_surface_gui_canvas_size(self.to_ptr()) }
			}
			pub fn set_canvas_size(&self, value: Vector2) {
				unsafe { prop_set_surface_gui_canvas_size(self.to_ptr(), value) }
			}
			pub fn clips_descendants(&self) -> bool {
				unsafe { prop_surface_gui_clips_descendants(self.to_ptr()) }
			}
			pub fn set_clips_descendants(&self, value: bool) {
				unsafe { prop_set_surface_gui_clips_descendants(self.to_ptr(), value) }
			}
			pub fn light_influence(&self) -> f64 {
				unsafe { prop_surface_gui_light_influence(self.to_ptr()) }
			}
			pub fn set_light_influence(&self, value: f64) {
				unsafe { prop_set_surface_gui_light_influence(self.to_ptr(), value) }
			}
			pub fn pixels_per_stud(&self) -> f64 {
				unsafe { prop_surface_gui_pixels_per_stud(self.to_ptr()) }
			}
			pub fn set_pixels_per_stud(&self, value: f64) {
				unsafe { prop_set_surface_gui_pixels_per_stud(self.to_ptr(), value) }
			}
			pub fn tool_punch_through_distance(&self) -> f64 {
				unsafe { prop_surface_gui_tool_punch_through_distance(self.to_ptr()) }
			}
			pub fn set_tool_punch_through_distance(&self, value: f64) {
				unsafe { prop_set_surface_gui_tool_punch_through_distance(self.to_ptr(), value) }
			}
			pub fn z_offset(&self) -> f64 {
				unsafe { prop_surface_gui_z_offset(self.to_ptr()) }
			}
			pub fn set_z_offset(&self, value: f64) {
				unsafe { prop_set_surface_gui_z_offset(self.to_ptr(), value) }
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
			pub fn color(&self) -> BrickColor {
				unsafe { prop_gui_base_3_d_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: BrickColor) {
				unsafe { prop_set_gui_base_3_d_color(self.to_ptr(), value) }
			}
			pub fn color_3(&self) -> Color3 {
				unsafe { prop_gui_base_3_d_color_3(self.to_ptr()) }
			}
			pub fn set_color_3(&self, value: Color3) {
				unsafe { prop_set_gui_base_3_d_color_3(self.to_ptr(), value) }
			}
			pub fn transparency(&self) -> f64 {
				unsafe { prop_gui_base_3_d_transparency(self.to_ptr()) }
			}
			pub fn set_transparency(&self, value: f64) {
				unsafe { prop_set_gui_base_3_d_transparency(self.to_ptr(), value) }
			}
			pub fn visible(&self) -> bool {
				unsafe { prop_gui_base_3_d_visible(self.to_ptr()) }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { prop_set_gui_base_3_d_visible(self.to_ptr(), value) }
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
				unsafe { prop_floor_wire_cycle_offset(self.to_ptr()) }
			}
			pub fn set_cycle_offset(&self, value: f64) {
				unsafe { prop_set_floor_wire_cycle_offset(self.to_ptr(), value) }
			}
			pub fn from(&self) -> Option<BasePart> {
				unsafe { prop_floor_wire_from(self.to_ptr()) }
			}
			pub fn set_from(&self, value: Option<BasePart>) {
				unsafe { prop_set_floor_wire_from(self.to_ptr(), value) }
			}
			pub fn studs_between_textures(&self) -> f64 {
				unsafe { prop_floor_wire_studs_between_textures(self.to_ptr()) }
			}
			pub fn set_studs_between_textures(&self, value: f64) {
				unsafe { prop_set_floor_wire_studs_between_textures(self.to_ptr(), value) }
			}
			pub fn texture(&self) -> Content {
				unsafe { prop_floor_wire_texture(self.to_ptr()) }
			}
			pub fn set_texture(&self, value: Content) {
				unsafe { prop_set_floor_wire_texture(self.to_ptr(), value) }
			}
			pub fn texture_size(&self) -> Vector2 {
				unsafe { prop_floor_wire_texture_size(self.to_ptr()) }
			}
			pub fn set_texture_size(&self, value: Vector2) {
				unsafe { prop_set_floor_wire_texture_size(self.to_ptr(), value) }
			}
			pub fn to(&self) -> Option<BasePart> {
				unsafe { prop_floor_wire_to(self.to_ptr()) }
			}
			pub fn set_to(&self, value: Option<BasePart>) {
				unsafe { prop_set_floor_wire_to(self.to_ptr(), value) }
			}
			pub fn velocity(&self) -> f64 {
				unsafe { prop_floor_wire_velocity(self.to_ptr()) }
			}
			pub fn set_velocity(&self, value: f64) {
				unsafe { prop_set_floor_wire_velocity(self.to_ptr(), value) }
			}
			pub fn wire_radius(&self) -> f64 {
				unsafe { prop_floor_wire_wire_radius(self.to_ptr()) }
			}
			pub fn set_wire_radius(&self, value: f64) {
				unsafe { prop_set_floor_wire_wire_radius(self.to_ptr(), value) }
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
				unsafe { prop_instance_adornment_adornee(self.to_ptr()) }
			}
			pub fn set_adornee(&self, value: Option<Instance>) {
				unsafe { prop_set_instance_adornment_adornee(self.to_ptr(), value) }
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
				unsafe { prop_selection_box_line_thickness(self.to_ptr()) }
			}
			pub fn set_line_thickness(&self, value: f64) {
				unsafe { prop_set_selection_box_line_thickness(self.to_ptr(), value) }
			}
			pub fn surface_color(&self) -> BrickColor {
				unsafe { prop_selection_box_surface_color(self.to_ptr()) }
			}
			pub fn set_surface_color(&self, value: BrickColor) {
				unsafe { prop_set_selection_box_surface_color(self.to_ptr(), value) }
			}
			pub fn surface_color_3(&self) -> Color3 {
				unsafe { prop_selection_box_surface_color_3(self.to_ptr()) }
			}
			pub fn set_surface_color_3(&self, value: Color3) {
				unsafe { prop_set_selection_box_surface_color_3(self.to_ptr(), value) }
			}
			pub fn surface_transparency(&self) -> f64 {
				unsafe { prop_selection_box_surface_transparency(self.to_ptr()) }
			}
			pub fn set_surface_transparency(&self, value: f64) {
				unsafe { prop_set_selection_box_surface_transparency(self.to_ptr(), value) }
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
				unsafe { prop_pv_adornment_adornee(self.to_ptr()) }
			}
			pub fn set_adornee(&self, value: Option<PVInstance>) {
				unsafe { prop_set_pv_adornment_adornee(self.to_ptr(), value) }
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
				unsafe { prop_handle_adornment_always_on_top(self.to_ptr()) }
			}
			pub fn set_always_on_top(&self, value: bool) {
				unsafe { prop_set_handle_adornment_always_on_top(self.to_ptr(), value) }
			}
			pub fn c_frame(&self) -> CFrame {
				unsafe { prop_handle_adornment_c_frame(self.to_ptr()) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { prop_set_handle_adornment_c_frame(self.to_ptr(), value) }
			}
			pub fn size_relative_offset(&self) -> Vector3 {
				unsafe { prop_handle_adornment_size_relative_offset(self.to_ptr()) }
			}
			pub fn set_size_relative_offset(&self, value: Vector3) {
				unsafe { prop_set_handle_adornment_size_relative_offset(self.to_ptr(), value) }
			}
			pub fn z_index(&self) -> f64 {
				unsafe { prop_handle_adornment_z_index(self.to_ptr()) }
			}
			pub fn set_z_index(&self, value: f64) {
				unsafe { prop_set_handle_adornment_z_index(self.to_ptr(), value) }
			}
			pub fn on_mouse_button_1_down<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_handle_adornment_mouse_button_1_down(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_button_1_up<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_handle_adornment_mouse_button_1_up(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_enter<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_handle_adornment_mouse_enter(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_leave<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_handle_adornment_mouse_leave(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_box_handle_adornment_size(self.to_ptr()) }
			}
			pub fn set_size(&self, value: Vector3) {
				unsafe { prop_set_box_handle_adornment_size(self.to_ptr(), value) }
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
				unsafe { prop_cone_handle_adornment_height(self.to_ptr()) }
			}
			pub fn set_height(&self, value: f64) {
				unsafe { prop_set_cone_handle_adornment_height(self.to_ptr(), value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { prop_cone_handle_adornment_radius(self.to_ptr()) }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { prop_set_cone_handle_adornment_radius(self.to_ptr(), value) }
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
				unsafe { prop_cylinder_handle_adornment_angle(self.to_ptr()) }
			}
			pub fn set_angle(&self, value: f64) {
				unsafe { prop_set_cylinder_handle_adornment_angle(self.to_ptr(), value) }
			}
			pub fn height(&self) -> f64 {
				unsafe { prop_cylinder_handle_adornment_height(self.to_ptr()) }
			}
			pub fn set_height(&self, value: f64) {
				unsafe { prop_set_cylinder_handle_adornment_height(self.to_ptr(), value) }
			}
			pub fn inner_radius(&self) -> f64 {
				unsafe { prop_cylinder_handle_adornment_inner_radius(self.to_ptr()) }
			}
			pub fn set_inner_radius(&self, value: f64) {
				unsafe { prop_set_cylinder_handle_adornment_inner_radius(self.to_ptr(), value) }
			}
			pub fn radius(&self) -> f64 {
				unsafe { prop_cylinder_handle_adornment_radius(self.to_ptr()) }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { prop_set_cylinder_handle_adornment_radius(self.to_ptr(), value) }
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
				unsafe { prop_image_handle_adornment_image(self.to_ptr()) }
			}
			pub fn set_image(&self, value: Content) {
				unsafe { prop_set_image_handle_adornment_image(self.to_ptr(), value) }
			}
			pub fn size(&self) -> Vector2 {
				unsafe { prop_image_handle_adornment_size(self.to_ptr()) }
			}
			pub fn set_size(&self, value: Vector2) {
				unsafe { prop_set_image_handle_adornment_size(self.to_ptr(), value) }
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
				unsafe { prop_line_handle_adornment_length(self.to_ptr()) }
			}
			pub fn set_length(&self, value: f64) {
				unsafe { prop_set_line_handle_adornment_length(self.to_ptr(), value) }
			}
			pub fn thickness(&self) -> f64 {
				unsafe { prop_line_handle_adornment_thickness(self.to_ptr()) }
			}
			pub fn set_thickness(&self, value: f64) {
				unsafe { prop_set_line_handle_adornment_thickness(self.to_ptr(), value) }
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
				unsafe { prop_sphere_handle_adornment_radius(self.to_ptr()) }
			}
			pub fn set_radius(&self, value: f64) {
				unsafe { prop_set_sphere_handle_adornment_radius(self.to_ptr(), value) }
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
			pub fn surface_color(&self) -> BrickColor {
				unsafe { prop_selection_sphere_surface_color(self.to_ptr()) }
			}
			pub fn set_surface_color(&self, value: BrickColor) {
				unsafe { prop_set_selection_sphere_surface_color(self.to_ptr(), value) }
			}
			pub fn surface_color_3(&self) -> Color3 {
				unsafe { prop_selection_sphere_surface_color_3(self.to_ptr()) }
			}
			pub fn set_surface_color_3(&self, value: Color3) {
				unsafe { prop_set_selection_sphere_surface_color_3(self.to_ptr(), value) }
			}
			pub fn surface_transparency(&self) -> f64 {
				unsafe { prop_selection_sphere_surface_transparency(self.to_ptr()) }
			}
			pub fn set_surface_transparency(&self, value: f64) {
				unsafe { prop_set_selection_sphere_surface_transparency(self.to_ptr(), value) }
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
				unsafe { prop_part_adornment_adornee(self.to_ptr()) }
			}
			pub fn set_adornee(&self, value: Option<BasePart>) {
				unsafe { prop_set_part_adornment_adornee(self.to_ptr(), value) }
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
				unsafe { prop_arc_handles_axes(self.to_ptr()) }
			}
			pub fn set_axes(&self, value: Axes) {
				unsafe { prop_set_arc_handles_axes(self.to_ptr(), value) }
			}
			pub fn on_mouse_button_1_down<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_arc_handles_mouse_button_1_down(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_button_1_up<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_arc_handles_mouse_button_1_up(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_drag<F: 'static + Fn((), f64, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_arc_handles_mouse_drag(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_enter<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_arc_handles_mouse_enter(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_leave<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_arc_handles_mouse_leave(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_handles_faces(self.to_ptr()) }
			}
			pub fn set_faces(&self, value: Faces) {
				unsafe { prop_set_handles_faces(self.to_ptr(), value) }
			}
			pub fn on_mouse_button_1_down<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_handles_mouse_button_1_down(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_button_1_up<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_handles_mouse_button_1_up(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_drag<F: 'static + Fn((), f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_handles_mouse_drag(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_enter<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_handles_mouse_enter(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_mouse_leave<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_handles_mouse_leave(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_selection_lasso_humanoid(self.to_ptr()) }
			}
			pub fn set_humanoid(&self, value: Option<Humanoid>) {
				unsafe { prop_set_selection_lasso_humanoid(self.to_ptr(), value) }
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
				unsafe { prop_selection_part_lasso_part(self.to_ptr()) }
			}
			pub fn set_part(&self, value: Option<BasePart>) {
				unsafe { prop_set_selection_part_lasso_part(self.to_ptr(), value) }
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
				unsafe { prop_selection_point_lasso_point(self.to_ptr()) }
			}
			pub fn set_point(&self, value: Vector3) {
				unsafe { prop_set_selection_point_lasso_point(self.to_ptr(), value) }
			}
		}
		impl From<$name> for SelectionLasso {
			fn from(value: $name) -> SelectionLasso {
				SelectionLasso(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_gui_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn auto_select_gui_enabled(&self) -> bool {
				unsafe { prop_gui_service_auto_select_gui_enabled(self.to_ptr()) }
			}
			pub fn set_auto_select_gui_enabled(&self, value: bool) {
				unsafe { prop_set_gui_service_auto_select_gui_enabled(self.to_ptr(), value) }
			}
			pub fn core_gui_navigation_enabled(&self) -> bool {
				unsafe { prop_gui_service_core_gui_navigation_enabled(self.to_ptr()) }
			}
			pub fn set_core_gui_navigation_enabled(&self, value: bool) {
				unsafe { prop_set_gui_service_core_gui_navigation_enabled(self.to_ptr(), value) }
			}
			pub fn gui_navigation_enabled(&self) -> bool {
				unsafe { prop_gui_service_gui_navigation_enabled(self.to_ptr()) }
			}
			pub fn set_gui_navigation_enabled(&self, value: bool) {
				unsafe { prop_set_gui_service_gui_navigation_enabled(self.to_ptr(), value) }
			}
			pub fn is_modal_dialog(&self) -> bool {
				unsafe { prop_gui_service_is_modal_dialog(self.to_ptr()) }
			}
			pub fn is_windows(&self) -> bool {
				unsafe { prop_gui_service_is_windows(self.to_ptr()) }
			}
			pub fn menu_is_open(&self) -> bool {
				unsafe { prop_gui_service_menu_is_open(self.to_ptr()) }
			}
			pub fn selected_object(&self) -> Option<GuiObject> {
				unsafe { prop_gui_service_selected_object(self.to_ptr()) }
			}
			pub fn set_selected_object(&self, value: Option<GuiObject>) {
				unsafe { prop_set_gui_service_selected_object(self.to_ptr(), value) }
			}
			pub fn touch_controls_enabled(&self) -> bool {
				unsafe { prop_gui_service_touch_controls_enabled(self.to_ptr()) }
			}
			pub fn set_touch_controls_enabled(&self, value: bool) {
				unsafe { prop_set_gui_service_touch_controls_enabled(self.to_ptr(), value) }
			}
			pub fn fn_add_selection_parent(&self, selection_name: &str, selection_parent: Option<Instance>) {
				unsafe { dyn_gui_service_add_selection_parent(self.to_ptr(), selection_name, selection_parent) }
			}
			pub fn fn_close_inspect_menu(&self) {
				unsafe { dyn_gui_service_close_inspect_menu(self.to_ptr()) }
			}
			pub fn fn_get_emotes_menu_open(&self) -> bool {
				unsafe { dyn_gui_service_get_emotes_menu_open(self.to_ptr()) }
			}
			pub fn fn_get_gameplay_paused_notification_enabled(&self) -> bool {
				unsafe { dyn_gui_service_get_gameplay_paused_notification_enabled(self.to_ptr()) }
			}
			pub fn fn_get_gui_inset(&self) {
				unsafe { dyn_gui_service_get_gui_inset(self.to_ptr()) }
			}
			pub fn fn_get_inspect_menu_enabled(&self) -> bool {
				unsafe { dyn_gui_service_get_inspect_menu_enabled(self.to_ptr()) }
			}
			pub fn fn_inspect_player_from_humanoid_description(&self, humanoid_description: Option<Instance>, name: &str) {
				unsafe { dyn_gui_service_inspect_player_from_humanoid_description(self.to_ptr(), humanoid_description, name) }
			}
			pub fn fn_inspect_player_from_user_id(&self, user_id: f64) {
				unsafe { dyn_gui_service_inspect_player_from_user_id(self.to_ptr(), user_id) }
			}
			pub fn fn_is_ten_foot_interface(&self) -> bool {
				unsafe { dyn_gui_service_is_ten_foot_interface(self.to_ptr()) }
			}
			pub fn fn_remove_selection_group(&self, selection_name: &str) {
				unsafe { dyn_gui_service_remove_selection_group(self.to_ptr(), selection_name) }
			}
			pub fn fn_select(&self, selection_parent: Option<Instance>) {
				unsafe { dyn_gui_service_select(self.to_ptr(), selection_parent) }
			}
			pub fn fn_set_emotes_menu_open(&self, is_open: bool) {
				unsafe { dyn_gui_service_set_emotes_menu_open(self.to_ptr(), is_open) }
			}
			pub fn fn_set_gameplay_paused_notification_enabled(&self, enabled: bool) {
				unsafe { dyn_gui_service_set_gameplay_paused_notification_enabled(self.to_ptr(), enabled) }
			}
			pub fn fn_set_inspect_menu_enabled(&self, enabled: bool) {
				unsafe { dyn_gui_service_set_inspect_menu_enabled(self.to_ptr(), enabled) }
			}
			pub fn on_menu_closed<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_service_menu_closed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_menu_opened<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_gui_service_menu_opened(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_haptic_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_highlight {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn adornee(&self) -> Option<Instance> {
				unsafe { prop_highlight_adornee(self.to_ptr()) }
			}
			pub fn set_adornee(&self, value: Option<Instance>) {
				unsafe { prop_set_highlight_adornee(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_highlight_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_highlight_enabled(self.to_ptr(), value) }
			}
			pub fn fill_color(&self) -> Color3 {
				unsafe { prop_highlight_fill_color(self.to_ptr()) }
			}
			pub fn set_fill_color(&self, value: Color3) {
				unsafe { prop_set_highlight_fill_color(self.to_ptr(), value) }
			}
			pub fn fill_transparency(&self) -> f64 {
				unsafe { prop_highlight_fill_transparency(self.to_ptr()) }
			}
			pub fn set_fill_transparency(&self, value: f64) {
				unsafe { prop_set_highlight_fill_transparency(self.to_ptr(), value) }
			}
			pub fn outline_color(&self) -> Color3 {
				unsafe { prop_highlight_outline_color(self.to_ptr()) }
			}
			pub fn set_outline_color(&self, value: Color3) {
				unsafe { prop_set_highlight_outline_color(self.to_ptr(), value) }
			}
			pub fn outline_transparency(&self) -> f64 {
				unsafe { prop_highlight_outline_transparency(self.to_ptr()) }
			}
			pub fn set_outline_transparency(&self, value: f64) {
				unsafe { prop_set_highlight_outline_transparency(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_http_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_generate_guid(&self, wrap_in_curly_braces: bool) -> String {
				unsafe { dyn_http_service_generate_guid(self.to_ptr(), wrap_in_curly_braces) }
			}
			pub fn fn_json_decode(&self, input: &str) {
				unsafe { dyn_http_service_json_decode(self.to_ptr(), input) }
			}
			pub fn fn_url_encode(&self, input: &str) -> String {
				unsafe { dyn_http_service_url_encode(self.to_ptr(), input) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_humanoid {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn auto_jump_enabled(&self) -> bool {
				unsafe { prop_humanoid_auto_jump_enabled(self.to_ptr()) }
			}
			pub fn set_auto_jump_enabled(&self, value: bool) {
				unsafe { prop_set_humanoid_auto_jump_enabled(self.to_ptr(), value) }
			}
			pub fn auto_rotate(&self) -> bool {
				unsafe { prop_humanoid_auto_rotate(self.to_ptr()) }
			}
			pub fn set_auto_rotate(&self, value: bool) {
				unsafe { prop_set_humanoid_auto_rotate(self.to_ptr(), value) }
			}
			pub fn automatic_scaling_enabled(&self) -> bool {
				unsafe { prop_humanoid_automatic_scaling_enabled(self.to_ptr()) }
			}
			pub fn set_automatic_scaling_enabled(&self, value: bool) {
				unsafe { prop_set_humanoid_automatic_scaling_enabled(self.to_ptr(), value) }
			}
			pub fn break_joints_on_death(&self) -> bool {
				unsafe { prop_humanoid_break_joints_on_death(self.to_ptr()) }
			}
			pub fn set_break_joints_on_death(&self, value: bool) {
				unsafe { prop_set_humanoid_break_joints_on_death(self.to_ptr(), value) }
			}
			pub fn camera_offset(&self) -> Vector3 {
				unsafe { prop_humanoid_camera_offset(self.to_ptr()) }
			}
			pub fn set_camera_offset(&self, value: Vector3) {
				unsafe { prop_set_humanoid_camera_offset(self.to_ptr(), value) }
			}
			pub fn display_name(&self) -> String {
				unsafe { prop_humanoid_display_name(self.to_ptr()) }
			}
			pub fn set_display_name(&self, value: &str) {
				unsafe { prop_set_humanoid_display_name(self.to_ptr(), value) }
			}
			pub fn health(&self) -> f64 {
				unsafe { prop_humanoid_health(self.to_ptr()) }
			}
			pub fn set_health(&self, value: f64) {
				unsafe { prop_set_humanoid_health(self.to_ptr(), value) }
			}
			pub fn health_display_distance(&self) -> f64 {
				unsafe { prop_humanoid_health_display_distance(self.to_ptr()) }
			}
			pub fn set_health_display_distance(&self, value: f64) {
				unsafe { prop_set_humanoid_health_display_distance(self.to_ptr(), value) }
			}
			pub fn hip_height(&self) -> f64 {
				unsafe { prop_humanoid_hip_height(self.to_ptr()) }
			}
			pub fn set_hip_height(&self, value: f64) {
				unsafe { prop_set_humanoid_hip_height(self.to_ptr(), value) }
			}
			pub fn jump(&self) -> bool {
				unsafe { prop_humanoid_jump(self.to_ptr()) }
			}
			pub fn set_jump(&self, value: bool) {
				unsafe { prop_set_humanoid_jump(self.to_ptr(), value) }
			}
			pub fn jump_height(&self) -> f64 {
				unsafe { prop_humanoid_jump_height(self.to_ptr()) }
			}
			pub fn set_jump_height(&self, value: f64) {
				unsafe { prop_set_humanoid_jump_height(self.to_ptr(), value) }
			}
			pub fn jump_power(&self) -> f64 {
				unsafe { prop_humanoid_jump_power(self.to_ptr()) }
			}
			pub fn set_jump_power(&self, value: f64) {
				unsafe { prop_set_humanoid_jump_power(self.to_ptr(), value) }
			}
			pub fn left_leg(&self) -> Option<BasePart> {
				unsafe { prop_humanoid_left_leg(self.to_ptr()) }
			}
			pub fn set_left_leg(&self, value: Option<BasePart>) {
				unsafe { prop_set_humanoid_left_leg(self.to_ptr(), value) }
			}
			pub fn max_health(&self) -> f64 {
				unsafe { prop_humanoid_max_health(self.to_ptr()) }
			}
			pub fn set_max_health(&self, value: f64) {
				unsafe { prop_set_humanoid_max_health(self.to_ptr(), value) }
			}
			pub fn max_slope_angle(&self) -> f64 {
				unsafe { prop_humanoid_max_slope_angle(self.to_ptr()) }
			}
			pub fn set_max_slope_angle(&self, value: f64) {
				unsafe { prop_set_humanoid_max_slope_angle(self.to_ptr(), value) }
			}
			pub fn move_direction(&self) -> Vector3 {
				unsafe { prop_humanoid_move_direction(self.to_ptr()) }
			}
			pub fn name_display_distance(&self) -> f64 {
				unsafe { prop_humanoid_name_display_distance(self.to_ptr()) }
			}
			pub fn set_name_display_distance(&self, value: f64) {
				unsafe { prop_set_humanoid_name_display_distance(self.to_ptr(), value) }
			}
			pub fn platform_stand(&self) -> bool {
				unsafe { prop_humanoid_platform_stand(self.to_ptr()) }
			}
			pub fn set_platform_stand(&self, value: bool) {
				unsafe { prop_set_humanoid_platform_stand(self.to_ptr(), value) }
			}
			pub fn requires_neck(&self) -> bool {
				unsafe { prop_humanoid_requires_neck(self.to_ptr()) }
			}
			pub fn set_requires_neck(&self, value: bool) {
				unsafe { prop_set_humanoid_requires_neck(self.to_ptr(), value) }
			}
			pub fn right_leg(&self) -> Option<BasePart> {
				unsafe { prop_humanoid_right_leg(self.to_ptr()) }
			}
			pub fn set_right_leg(&self, value: Option<BasePart>) {
				unsafe { prop_set_humanoid_right_leg(self.to_ptr(), value) }
			}
			pub fn root_part(&self) -> Option<BasePart> {
				unsafe { prop_humanoid_root_part(self.to_ptr()) }
			}
			pub fn seat_part(&self) -> Option<BasePart> {
				unsafe { prop_humanoid_seat_part(self.to_ptr()) }
			}
			pub fn sit(&self) -> bool {
				unsafe { prop_humanoid_sit(self.to_ptr()) }
			}
			pub fn set_sit(&self, value: bool) {
				unsafe { prop_set_humanoid_sit(self.to_ptr(), value) }
			}
			pub fn target_point(&self) -> Vector3 {
				unsafe { prop_humanoid_target_point(self.to_ptr()) }
			}
			pub fn set_target_point(&self, value: Vector3) {
				unsafe { prop_set_humanoid_target_point(self.to_ptr(), value) }
			}
			pub fn torso(&self) -> Option<BasePart> {
				unsafe { prop_humanoid_torso(self.to_ptr()) }
			}
			pub fn set_torso(&self, value: Option<BasePart>) {
				unsafe { prop_set_humanoid_torso(self.to_ptr(), value) }
			}
			pub fn use_jump_power(&self) -> bool {
				unsafe { prop_humanoid_use_jump_power(self.to_ptr()) }
			}
			pub fn set_use_jump_power(&self, value: bool) {
				unsafe { prop_set_humanoid_use_jump_power(self.to_ptr(), value) }
			}
			pub fn walk_speed(&self) -> f64 {
				unsafe { prop_humanoid_walk_speed(self.to_ptr()) }
			}
			pub fn set_walk_speed(&self, value: f64) {
				unsafe { prop_set_humanoid_walk_speed(self.to_ptr(), value) }
			}
			pub fn walk_to_part(&self) -> Option<BasePart> {
				unsafe { prop_humanoid_walk_to_part(self.to_ptr()) }
			}
			pub fn set_walk_to_part(&self, value: Option<BasePart>) {
				unsafe { prop_set_humanoid_walk_to_part(self.to_ptr(), value) }
			}
			pub fn walk_to_point(&self) -> Vector3 {
				unsafe { prop_humanoid_walk_to_point(self.to_ptr()) }
			}
			pub fn set_walk_to_point(&self, value: Vector3) {
				unsafe { prop_set_humanoid_walk_to_point(self.to_ptr(), value) }
			}
			pub fn fn_add_accessory(&self, accessory: Option<Instance>) {
				unsafe { dyn_humanoid_add_accessory(self.to_ptr(), accessory) }
			}
			pub fn fn_add_custom_status(&self, status: &str) -> bool {
				unsafe { dyn_humanoid_add_custom_status(self.to_ptr(), status) }
			}
			pub fn fn_build_rig_from_attachments(&self) {
				unsafe { dyn_humanoid_build_rig_from_attachments(self.to_ptr()) }
			}
			pub fn fn_equip_tool(&self, tool: Option<Instance>) {
				unsafe { dyn_humanoid_equip_tool(self.to_ptr(), tool) }
			}
			pub fn fn_get_accessories(&self) {
				unsafe { dyn_humanoid_get_accessories(self.to_ptr()) }
			}
			pub fn fn_get_applied_description(&self) -> Option<HumanoidDescription> {
				unsafe { dyn_humanoid_get_applied_description(self.to_ptr()) }
			}
			pub fn fn_get_body_part_r_15(&self, part: Option<Instance>) {
				unsafe { dyn_humanoid_get_body_part_r_15(self.to_ptr(), part) }
			}
			pub fn fn_get_limb(&self, part: Option<Instance>) {
				unsafe { dyn_humanoid_get_limb(self.to_ptr(), part) }
			}
			pub fn fn_get_playing_animation_tracks(&self) {
				unsafe { dyn_humanoid_get_playing_animation_tracks(self.to_ptr()) }
			}
			pub fn fn_get_state(&self) {
				unsafe { dyn_humanoid_get_state(self.to_ptr()) }
			}
			pub fn fn_get_statuses(&self) {
				unsafe { dyn_humanoid_get_statuses(self.to_ptr()) }
			}
			pub fn fn_has_custom_status(&self, status: &str) -> bool {
				unsafe { dyn_humanoid_has_custom_status(self.to_ptr(), status) }
			}
			pub fn fn_load_animation(&self, animation: Option<Animation>) -> Option<AnimationTrack> {
				unsafe { dyn_humanoid_load_animation(self.to_ptr(), animation) }
			}
			pub fn fn_move(&self, move_direction: Vector3, relative_to_camera: bool) {
				unsafe { dyn_humanoid_move(self.to_ptr(), move_direction, relative_to_camera) }
			}
			pub fn fn_move_to(&self, location: Vector3, part: Option<Instance>) {
				unsafe { dyn_humanoid_move_to(self.to_ptr(), location, part) }
			}
			pub fn fn_remove_accessories(&self) {
				unsafe { dyn_humanoid_remove_accessories(self.to_ptr()) }
			}
			pub fn fn_remove_custom_status(&self, status: &str) -> bool {
				unsafe { dyn_humanoid_remove_custom_status(self.to_ptr(), status) }
			}
			pub fn fn_take_damage(&self, amount: f64) {
				unsafe { dyn_humanoid_take_damage(self.to_ptr(), amount) }
			}
			pub fn fn_unequip_tools(&self) {
				unsafe { dyn_humanoid_unequip_tools(self.to_ptr()) }
			}
			pub fn fn_play_emote(&self, emote_name: &str) -> bool {
				unsafe { dyn_humanoid_play_emote(self.to_ptr(), emote_name) }
			}
			pub fn on_animation_played<F: 'static + Fn(Option<AnimationTrack>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_animation_played(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_climbing<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_climbing(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_custom_status_added<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_custom_status_added(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_custom_status_removed<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_custom_status_removed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_died<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_died(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_falling_down<F: 'static + Fn(bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_falling_down(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_free_falling<F: 'static + Fn(bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_free_falling(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_getting_up<F: 'static + Fn(bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_getting_up(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_health_changed<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_health_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_jumping<F: 'static + Fn(bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_jumping(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_move_to_finished<F: 'static + Fn(bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_move_to_finished(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_platform_standing<F: 'static + Fn(bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_platform_standing(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_ragdoll<F: 'static + Fn(bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_ragdoll(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_running<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_running(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_seated<F: 'static + Fn(bool, Option<BasePart>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_seated(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_state_changed<F: 'static + Fn((), ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_state_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_state_enabled_changed<F: 'static + Fn((), bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_state_enabled_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_status_added<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_status_added(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_status_removed<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_status_removed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_strafing<F: 'static + Fn(bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_strafing(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_swimming<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_swimming(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touched<F: 'static + Fn(Option<BasePart>, Option<BasePart>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_touched(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_humanoid_description {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn accessory_blob(&self) -> String {
				unsafe { prop_humanoid_description_accessory_blob(self.to_ptr()) }
			}
			pub fn set_accessory_blob(&self, value: &str) {
				unsafe { prop_set_humanoid_description_accessory_blob(self.to_ptr(), value) }
			}
			pub fn back_accessory(&self) -> String {
				unsafe { prop_humanoid_description_back_accessory(self.to_ptr()) }
			}
			pub fn set_back_accessory(&self, value: &str) {
				unsafe { prop_set_humanoid_description_back_accessory(self.to_ptr(), value) }
			}
			pub fn body_type_scale(&self) -> f64 {
				unsafe { prop_humanoid_description_body_type_scale(self.to_ptr()) }
			}
			pub fn set_body_type_scale(&self, value: f64) {
				unsafe { prop_set_humanoid_description_body_type_scale(self.to_ptr(), value) }
			}
			pub fn climb_animation(&self) -> f64 {
				unsafe { prop_humanoid_description_climb_animation(self.to_ptr()) }
			}
			pub fn set_climb_animation(&self, value: f64) {
				unsafe { prop_set_humanoid_description_climb_animation(self.to_ptr(), value) }
			}
			pub fn depth_scale(&self) -> f64 {
				unsafe { prop_humanoid_description_depth_scale(self.to_ptr()) }
			}
			pub fn set_depth_scale(&self, value: f64) {
				unsafe { prop_set_humanoid_description_depth_scale(self.to_ptr(), value) }
			}
			pub fn face(&self) -> f64 {
				unsafe { prop_humanoid_description_face(self.to_ptr()) }
			}
			pub fn set_face(&self, value: f64) {
				unsafe { prop_set_humanoid_description_face(self.to_ptr(), value) }
			}
			pub fn face_accessory(&self) -> String {
				unsafe { prop_humanoid_description_face_accessory(self.to_ptr()) }
			}
			pub fn set_face_accessory(&self, value: &str) {
				unsafe { prop_set_humanoid_description_face_accessory(self.to_ptr(), value) }
			}
			pub fn fall_animation(&self) -> f64 {
				unsafe { prop_humanoid_description_fall_animation(self.to_ptr()) }
			}
			pub fn set_fall_animation(&self, value: f64) {
				unsafe { prop_set_humanoid_description_fall_animation(self.to_ptr(), value) }
			}
			pub fn front_accessory(&self) -> String {
				unsafe { prop_humanoid_description_front_accessory(self.to_ptr()) }
			}
			pub fn set_front_accessory(&self, value: &str) {
				unsafe { prop_set_humanoid_description_front_accessory(self.to_ptr(), value) }
			}
			pub fn graphic_t_shirt(&self) -> f64 {
				unsafe { prop_humanoid_description_graphic_t_shirt(self.to_ptr()) }
			}
			pub fn set_graphic_t_shirt(&self, value: f64) {
				unsafe { prop_set_humanoid_description_graphic_t_shirt(self.to_ptr(), value) }
			}
			pub fn hair_accessory(&self) -> String {
				unsafe { prop_humanoid_description_hair_accessory(self.to_ptr()) }
			}
			pub fn set_hair_accessory(&self, value: &str) {
				unsafe { prop_set_humanoid_description_hair_accessory(self.to_ptr(), value) }
			}
			pub fn hat_accessory(&self) -> String {
				unsafe { prop_humanoid_description_hat_accessory(self.to_ptr()) }
			}
			pub fn set_hat_accessory(&self, value: &str) {
				unsafe { prop_set_humanoid_description_hat_accessory(self.to_ptr(), value) }
			}
			pub fn head(&self) -> f64 {
				unsafe { prop_humanoid_description_head(self.to_ptr()) }
			}
			pub fn set_head(&self, value: f64) {
				unsafe { prop_set_humanoid_description_head(self.to_ptr(), value) }
			}
			pub fn head_color(&self) -> Color3 {
				unsafe { prop_humanoid_description_head_color(self.to_ptr()) }
			}
			pub fn set_head_color(&self, value: Color3) {
				unsafe { prop_set_humanoid_description_head_color(self.to_ptr(), value) }
			}
			pub fn head_scale(&self) -> f64 {
				unsafe { prop_humanoid_description_head_scale(self.to_ptr()) }
			}
			pub fn set_head_scale(&self, value: f64) {
				unsafe { prop_set_humanoid_description_head_scale(self.to_ptr(), value) }
			}
			pub fn height_scale(&self) -> f64 {
				unsafe { prop_humanoid_description_height_scale(self.to_ptr()) }
			}
			pub fn set_height_scale(&self, value: f64) {
				unsafe { prop_set_humanoid_description_height_scale(self.to_ptr(), value) }
			}
			pub fn idle_animation(&self) -> f64 {
				unsafe { prop_humanoid_description_idle_animation(self.to_ptr()) }
			}
			pub fn set_idle_animation(&self, value: f64) {
				unsafe { prop_set_humanoid_description_idle_animation(self.to_ptr(), value) }
			}
			pub fn jump_animation(&self) -> f64 {
				unsafe { prop_humanoid_description_jump_animation(self.to_ptr()) }
			}
			pub fn set_jump_animation(&self, value: f64) {
				unsafe { prop_set_humanoid_description_jump_animation(self.to_ptr(), value) }
			}
			pub fn left_arm(&self) -> f64 {
				unsafe { prop_humanoid_description_left_arm(self.to_ptr()) }
			}
			pub fn set_left_arm(&self, value: f64) {
				unsafe { prop_set_humanoid_description_left_arm(self.to_ptr(), value) }
			}
			pub fn left_arm_color(&self) -> Color3 {
				unsafe { prop_humanoid_description_left_arm_color(self.to_ptr()) }
			}
			pub fn set_left_arm_color(&self, value: Color3) {
				unsafe { prop_set_humanoid_description_left_arm_color(self.to_ptr(), value) }
			}
			pub fn left_leg(&self) -> f64 {
				unsafe { prop_humanoid_description_left_leg(self.to_ptr()) }
			}
			pub fn set_left_leg(&self, value: f64) {
				unsafe { prop_set_humanoid_description_left_leg(self.to_ptr(), value) }
			}
			pub fn left_leg_color(&self) -> Color3 {
				unsafe { prop_humanoid_description_left_leg_color(self.to_ptr()) }
			}
			pub fn set_left_leg_color(&self, value: Color3) {
				unsafe { prop_set_humanoid_description_left_leg_color(self.to_ptr(), value) }
			}
			pub fn neck_accessory(&self) -> String {
				unsafe { prop_humanoid_description_neck_accessory(self.to_ptr()) }
			}
			pub fn set_neck_accessory(&self, value: &str) {
				unsafe { prop_set_humanoid_description_neck_accessory(self.to_ptr(), value) }
			}
			pub fn pants(&self) -> f64 {
				unsafe { prop_humanoid_description_pants(self.to_ptr()) }
			}
			pub fn set_pants(&self, value: f64) {
				unsafe { prop_set_humanoid_description_pants(self.to_ptr(), value) }
			}
			pub fn proportion_scale(&self) -> f64 {
				unsafe { prop_humanoid_description_proportion_scale(self.to_ptr()) }
			}
			pub fn set_proportion_scale(&self, value: f64) {
				unsafe { prop_set_humanoid_description_proportion_scale(self.to_ptr(), value) }
			}
			pub fn right_arm(&self) -> f64 {
				unsafe { prop_humanoid_description_right_arm(self.to_ptr()) }
			}
			pub fn set_right_arm(&self, value: f64) {
				unsafe { prop_set_humanoid_description_right_arm(self.to_ptr(), value) }
			}
			pub fn right_arm_color(&self) -> Color3 {
				unsafe { prop_humanoid_description_right_arm_color(self.to_ptr()) }
			}
			pub fn set_right_arm_color(&self, value: Color3) {
				unsafe { prop_set_humanoid_description_right_arm_color(self.to_ptr(), value) }
			}
			pub fn right_leg(&self) -> f64 {
				unsafe { prop_humanoid_description_right_leg(self.to_ptr()) }
			}
			pub fn set_right_leg(&self, value: f64) {
				unsafe { prop_set_humanoid_description_right_leg(self.to_ptr(), value) }
			}
			pub fn right_leg_color(&self) -> Color3 {
				unsafe { prop_humanoid_description_right_leg_color(self.to_ptr()) }
			}
			pub fn set_right_leg_color(&self, value: Color3) {
				unsafe { prop_set_humanoid_description_right_leg_color(self.to_ptr(), value) }
			}
			pub fn run_animation(&self) -> f64 {
				unsafe { prop_humanoid_description_run_animation(self.to_ptr()) }
			}
			pub fn set_run_animation(&self, value: f64) {
				unsafe { prop_set_humanoid_description_run_animation(self.to_ptr(), value) }
			}
			pub fn shirt(&self) -> f64 {
				unsafe { prop_humanoid_description_shirt(self.to_ptr()) }
			}
			pub fn set_shirt(&self, value: f64) {
				unsafe { prop_set_humanoid_description_shirt(self.to_ptr(), value) }
			}
			pub fn shoulders_accessory(&self) -> String {
				unsafe { prop_humanoid_description_shoulders_accessory(self.to_ptr()) }
			}
			pub fn set_shoulders_accessory(&self, value: &str) {
				unsafe { prop_set_humanoid_description_shoulders_accessory(self.to_ptr(), value) }
			}
			pub fn swim_animation(&self) -> f64 {
				unsafe { prop_humanoid_description_swim_animation(self.to_ptr()) }
			}
			pub fn set_swim_animation(&self, value: f64) {
				unsafe { prop_set_humanoid_description_swim_animation(self.to_ptr(), value) }
			}
			pub fn torso(&self) -> f64 {
				unsafe { prop_humanoid_description_torso(self.to_ptr()) }
			}
			pub fn set_torso(&self, value: f64) {
				unsafe { prop_set_humanoid_description_torso(self.to_ptr(), value) }
			}
			pub fn torso_color(&self) -> Color3 {
				unsafe { prop_humanoid_description_torso_color(self.to_ptr()) }
			}
			pub fn set_torso_color(&self, value: Color3) {
				unsafe { prop_set_humanoid_description_torso_color(self.to_ptr(), value) }
			}
			pub fn waist_accessory(&self) -> String {
				unsafe { prop_humanoid_description_waist_accessory(self.to_ptr()) }
			}
			pub fn set_waist_accessory(&self, value: &str) {
				unsafe { prop_set_humanoid_description_waist_accessory(self.to_ptr(), value) }
			}
			pub fn walk_animation(&self) -> f64 {
				unsafe { prop_humanoid_description_walk_animation(self.to_ptr()) }
			}
			pub fn set_walk_animation(&self, value: f64) {
				unsafe { prop_set_humanoid_description_walk_animation(self.to_ptr(), value) }
			}
			pub fn width_scale(&self) -> f64 {
				unsafe { prop_humanoid_description_width_scale(self.to_ptr()) }
			}
			pub fn set_width_scale(&self, value: f64) {
				unsafe { prop_set_humanoid_description_width_scale(self.to_ptr(), value) }
			}
			pub fn fn_add_emote(&self, name: &str, asset_id: f64) {
				unsafe { dyn_humanoid_description_add_emote(self.to_ptr(), name, asset_id) }
			}
			pub fn fn_get_accessories(&self, include_rigid_accessories: bool) {
				unsafe { dyn_humanoid_description_get_accessories(self.to_ptr(), include_rigid_accessories) }
			}
			pub fn fn_get_emotes(&self) {
				unsafe { dyn_humanoid_description_get_emotes(self.to_ptr()) }
			}
			pub fn fn_get_equipped_emotes(&self) {
				unsafe { dyn_humanoid_description_get_equipped_emotes(self.to_ptr()) }
			}
			pub fn fn_remove_emote(&self, name: &str) {
				unsafe { dyn_humanoid_description_remove_emote(self.to_ptr(), name) }
			}
			pub fn on_emotes_changed<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_description_emotes_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_equipped_emotes_changed<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_humanoid_description_equipped_emotes_changed(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_importer_base_settings {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn id(&self) -> String {
				unsafe { prop_importer_base_settings_id(self.to_ptr()) }
			}
			pub fn import_name(&self) -> String {
				unsafe { prop_importer_base_settings_import_name(self.to_ptr()) }
			}
			pub fn set_import_name(&self, value: &str) {
				unsafe { prop_set_importer_base_settings_import_name(self.to_ptr(), value) }
			}
			pub fn should_import(&self) -> bool {
				unsafe { prop_importer_base_settings_should_import(self.to_ptr()) }
			}
			pub fn set_should_import(&self, value: bool) {
				unsafe { prop_set_importer_base_settings_should_import(self.to_ptr(), value) }
			}
		}
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
				unsafe { prop_importer_group_settings_anchored(self.to_ptr()) }
			}
			pub fn set_anchored(&self, value: bool) {
				unsafe { prop_set_importer_group_settings_anchored(self.to_ptr(), value) }
			}
			pub fn import_as_model_asset(&self) -> bool {
				unsafe { prop_importer_group_settings_import_as_model_asset(self.to_ptr()) }
			}
			pub fn set_import_as_model_asset(&self, value: bool) {
				unsafe { prop_set_importer_group_settings_import_as_model_asset(self.to_ptr(), value) }
			}
			pub fn insert_in_workspace(&self) -> bool {
				unsafe { prop_importer_group_settings_insert_in_workspace(self.to_ptr()) }
			}
			pub fn set_insert_in_workspace(&self, value: bool) {
				unsafe { prop_set_importer_group_settings_insert_in_workspace(self.to_ptr(), value) }
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
				unsafe { prop_importer_material_settings_diffuse_file_path(self.to_ptr()) }
			}
			pub fn is_pbr(&self) -> bool {
				unsafe { prop_importer_material_settings_is_pbr(self.to_ptr()) }
			}
			pub fn metalness_file_path(&self) -> String {
				unsafe { prop_importer_material_settings_metalness_file_path(self.to_ptr()) }
			}
			pub fn normal_file_path(&self) -> String {
				unsafe { prop_importer_material_settings_normal_file_path(self.to_ptr()) }
			}
			pub fn roughness_file_path(&self) -> String {
				unsafe { prop_importer_material_settings_roughness_file_path(self.to_ptr()) }
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
				unsafe { prop_importer_mesh_settings_anchored(self.to_ptr()) }
			}
			pub fn set_anchored(&self, value: bool) {
				unsafe { prop_set_importer_mesh_settings_anchored(self.to_ptr(), value) }
			}
			pub fn cage_manifold(&self) -> bool {
				unsafe { prop_importer_mesh_settings_cage_manifold(self.to_ptr()) }
			}
			pub fn cage_manifold_preview(&self) -> bool {
				unsafe { prop_importer_mesh_settings_cage_manifold_preview(self.to_ptr()) }
			}
			pub fn set_cage_manifold_preview(&self, value: bool) {
				unsafe { prop_set_importer_mesh_settings_cage_manifold_preview(self.to_ptr(), value) }
			}
			pub fn cage_no_overlapping_vertices(&self) -> bool {
				unsafe { prop_importer_mesh_settings_cage_no_overlapping_vertices(self.to_ptr()) }
			}
			pub fn cage_no_overlapping_vertices_preview(&self) -> bool {
				unsafe { prop_importer_mesh_settings_cage_no_overlapping_vertices_preview(self.to_ptr()) }
			}
			pub fn set_cage_no_overlapping_vertices_preview(&self, value: bool) {
				unsafe { prop_set_importer_mesh_settings_cage_no_overlapping_vertices_preview(self.to_ptr(), value) }
			}
			pub fn cage_uv_matched(&self) -> bool {
				unsafe { prop_importer_mesh_settings_cage_uv_matched(self.to_ptr()) }
			}
			pub fn cage_uv_matched_preview(&self) -> bool {
				unsafe { prop_importer_mesh_settings_cage_uv_matched_preview(self.to_ptr()) }
			}
			pub fn set_cage_uv_matched_preview(&self, value: bool) {
				unsafe { prop_set_importer_mesh_settings_cage_uv_matched_preview(self.to_ptr(), value) }
			}
			pub fn dimensions(&self) -> Vector3 {
				unsafe { prop_importer_mesh_settings_dimensions(self.to_ptr()) }
			}
			pub fn double_sided(&self) -> bool {
				unsafe { prop_importer_mesh_settings_double_sided(self.to_ptr()) }
			}
			pub fn set_double_sided(&self, value: bool) {
				unsafe { prop_set_importer_mesh_settings_double_sided(self.to_ptr(), value) }
			}
			pub fn ignore_vertex_colors(&self) -> bool {
				unsafe { prop_importer_mesh_settings_ignore_vertex_colors(self.to_ptr()) }
			}
			pub fn set_ignore_vertex_colors(&self, value: bool) {
				unsafe { prop_set_importer_mesh_settings_ignore_vertex_colors(self.to_ptr(), value) }
			}
			pub fn polygon_count(&self) -> f64 {
				unsafe { prop_importer_mesh_settings_polygon_count(self.to_ptr()) }
			}
			pub fn use_imported_pivot(&self) -> bool {
				unsafe { prop_importer_mesh_settings_use_imported_pivot(self.to_ptr()) }
			}
			pub fn set_use_imported_pivot(&self, value: bool) {
				unsafe { prop_set_importer_mesh_settings_use_imported_pivot(self.to_ptr(), value) }
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
				unsafe { prop_importer_root_settings_anchored(self.to_ptr()) }
			}
			pub fn set_anchored(&self, value: bool) {
				unsafe { prop_set_importer_root_settings_anchored(self.to_ptr(), value) }
			}
			pub fn file_dimensions(&self) -> Vector3 {
				unsafe { prop_importer_root_settings_file_dimensions(self.to_ptr()) }
			}
			pub fn import_as_model_asset(&self) -> bool {
				unsafe { prop_importer_root_settings_import_as_model_asset(self.to_ptr()) }
			}
			pub fn set_import_as_model_asset(&self, value: bool) {
				unsafe { prop_set_importer_root_settings_import_as_model_asset(self.to_ptr(), value) }
			}
			pub fn insert_in_workspace(&self) -> bool {
				unsafe { prop_importer_root_settings_insert_in_workspace(self.to_ptr()) }
			}
			pub fn set_insert_in_workspace(&self, value: bool) {
				unsafe { prop_set_importer_root_settings_insert_in_workspace(self.to_ptr(), value) }
			}
			pub fn insert_with_scene_position(&self) -> bool {
				unsafe { prop_importer_root_settings_insert_with_scene_position(self.to_ptr()) }
			}
			pub fn set_insert_with_scene_position(&self, value: bool) {
				unsafe { prop_set_importer_root_settings_insert_with_scene_position(self.to_ptr(), value) }
			}
			pub fn invert_negative_faces(&self) -> bool {
				unsafe { prop_importer_root_settings_invert_negative_faces(self.to_ptr()) }
			}
			pub fn set_invert_negative_faces(&self, value: bool) {
				unsafe { prop_set_importer_root_settings_invert_negative_faces(self.to_ptr(), value) }
			}
			pub fn merge_meshes(&self) -> bool {
				unsafe { prop_importer_root_settings_merge_meshes(self.to_ptr()) }
			}
			pub fn set_merge_meshes(&self, value: bool) {
				unsafe { prop_set_importer_root_settings_merge_meshes(self.to_ptr(), value) }
			}
			pub fn polygon_count(&self) -> f64 {
				unsafe { prop_importer_root_settings_polygon_count(self.to_ptr()) }
			}
			pub fn use_scene_origin_as_pivot(&self) -> bool {
				unsafe { prop_importer_root_settings_use_scene_origin_as_pivot(self.to_ptr()) }
			}
			pub fn set_use_scene_origin_as_pivot(&self, value: bool) {
				unsafe { prop_set_importer_root_settings_use_scene_origin_as_pivot(self.to_ptr(), value) }
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn delta(&self) -> Vector3 {
				unsafe { prop_input_object_delta(self.to_ptr()) }
			}
			pub fn set_delta(&self, value: Vector3) {
				unsafe { prop_set_input_object_delta(self.to_ptr(), value) }
			}
			pub fn position(&self) -> Vector3 {
				unsafe { prop_input_object_position(self.to_ptr()) }
			}
			pub fn set_position(&self, value: Vector3) {
				unsafe { prop_set_input_object_position(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_insert_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn allow_client_insert_models(&self) -> bool {
				unsafe { prop_insert_service_allow_client_insert_models(self.to_ptr()) }
			}
			pub fn set_allow_client_insert_models(&self, value: bool) {
				unsafe { prop_set_insert_service_allow_client_insert_models(self.to_ptr(), value) }
			}
			pub fn allow_insert_free_models(&self) -> bool {
				unsafe { prop_insert_service_allow_insert_free_models(self.to_ptr()) }
			}
			pub fn set_allow_insert_free_models(&self, value: bool) {
				unsafe { prop_set_insert_service_allow_insert_free_models(self.to_ptr(), value) }
			}
			pub fn fn_approve_asset_id(&self, asset_id: f64) {
				unsafe { dyn_insert_service_approve_asset_id(self.to_ptr(), asset_id) }
			}
			pub fn fn_approve_asset_version_id(&self, asset_version_id: f64) {
				unsafe { dyn_insert_service_approve_asset_version_id(self.to_ptr(), asset_version_id) }
			}
			pub fn fn_insert(&self, instance: Option<Instance>) {
				unsafe { dyn_insert_service_insert(self.to_ptr(), instance) }
			}
			pub fn fn_get_base_categories(&self) {
				unsafe { dyn_insert_service_get_base_categories(self.to_ptr()) }
			}
			pub fn fn_get_base_sets(&self) {
				unsafe { dyn_insert_service_get_base_sets(self.to_ptr()) }
			}
			pub fn fn_get_collection(&self, category_id: f64) {
				unsafe { dyn_insert_service_get_collection(self.to_ptr(), category_id) }
			}
			pub fn fn_get_free_decals(&self, search_text: &str, page_num: f64) {
				unsafe { dyn_insert_service_get_free_decals(self.to_ptr(), search_text, page_num) }
			}
			pub fn fn_get_free_models(&self, search_text: &str, page_num: f64) {
				unsafe { dyn_insert_service_get_free_models(self.to_ptr(), search_text, page_num) }
			}
			pub fn fn_get_latest_asset_version_async(&self, asset_id: f64) -> f64 {
				unsafe { dyn_insert_service_get_latest_asset_version_async(self.to_ptr(), asset_id) }
			}
			pub fn fn_get_user_categories(&self, user_id: f64) {
				unsafe { dyn_insert_service_get_user_categories(self.to_ptr(), user_id) }
			}
			pub fn fn_get_user_sets(&self, user_id: f64) {
				unsafe { dyn_insert_service_get_user_sets(self.to_ptr(), user_id) }
			}
			pub fn fn_load_asset(&self, asset_id: f64) -> Option<Instance> {
				unsafe { dyn_insert_service_load_asset(self.to_ptr(), asset_id) }
			}
			pub fn fn_load_asset_version(&self, asset_version_id: f64) -> Option<Instance> {
				unsafe { dyn_insert_service_load_asset_version(self.to_ptr(), asset_version_id) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_joint_instance {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn active(&self) -> bool {
				unsafe { prop_joint_instance_active(self.to_ptr()) }
			}
			pub fn c_0(&self) -> CFrame {
				unsafe { prop_joint_instance_c_0(self.to_ptr()) }
			}
			pub fn set_c_0(&self, value: CFrame) {
				unsafe { prop_set_joint_instance_c_0(self.to_ptr(), value) }
			}
			pub fn c_1(&self) -> CFrame {
				unsafe { prop_joint_instance_c_1(self.to_ptr()) }
			}
			pub fn set_c_1(&self, value: CFrame) {
				unsafe { prop_set_joint_instance_c_1(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_joint_instance_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_joint_instance_enabled(self.to_ptr(), value) }
			}
			pub fn part_0(&self) -> Option<BasePart> {
				unsafe { prop_joint_instance_part_0(self.to_ptr()) }
			}
			pub fn set_part_0(&self, value: Option<BasePart>) {
				unsafe { prop_set_joint_instance_part_0(self.to_ptr(), value) }
			}
			pub fn part_1(&self) -> Option<BasePart> {
				unsafe { prop_joint_instance_part_1(self.to_ptr()) }
			}
			pub fn set_part_1(&self, value: Option<BasePart>) {
				unsafe { prop_set_joint_instance_part_1(self.to_ptr(), value) }
			}
		}
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
				unsafe { prop_dynamic_rotate_base_angle(self.to_ptr()) }
			}
			pub fn set_base_angle(&self, value: f64) {
				unsafe { prop_set_dynamic_rotate_base_angle(self.to_ptr(), value) }
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
				unsafe { prop_glue_f_0(self.to_ptr()) }
			}
			pub fn set_f_0(&self, value: Vector3) {
				unsafe { prop_set_glue_f_0(self.to_ptr(), value) }
			}
			pub fn f_1(&self) -> Vector3 {
				unsafe { prop_glue_f_1(self.to_ptr()) }
			}
			pub fn set_f_1(&self, value: Vector3) {
				unsafe { prop_set_glue_f_1(self.to_ptr(), value) }
			}
			pub fn f_2(&self) -> Vector3 {
				unsafe { prop_glue_f_2(self.to_ptr()) }
			}
			pub fn set_f_2(&self, value: Vector3) {
				unsafe { prop_set_glue_f_2(self.to_ptr(), value) }
			}
			pub fn f_3(&self) -> Vector3 {
				unsafe { prop_glue_f_3(self.to_ptr()) }
			}
			pub fn set_f_3(&self, value: Vector3) {
				unsafe { prop_set_glue_f_3(self.to_ptr(), value) }
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
				unsafe { prop_motor_current_angle(self.to_ptr()) }
			}
			pub fn set_current_angle(&self, value: f64) {
				unsafe { prop_set_motor_current_angle(self.to_ptr(), value) }
			}
			pub fn desired_angle(&self) -> f64 {
				unsafe { prop_motor_desired_angle(self.to_ptr()) }
			}
			pub fn set_desired_angle(&self, value: f64) {
				unsafe { prop_set_motor_desired_angle(self.to_ptr(), value) }
			}
			pub fn max_velocity(&self) -> f64 {
				unsafe { prop_motor_max_velocity(self.to_ptr()) }
			}
			pub fn set_max_velocity(&self, value: f64) {
				unsafe { prop_set_motor_max_velocity(self.to_ptr(), value) }
			}
			pub fn fn_set_desired_angle(&self, value: f64) {
				unsafe { dyn_motor_set_desired_angle(self.to_ptr(), value) }
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
			pub fn child_name(&self) -> String {
				unsafe { prop_motor_6_d_child_name(self.to_ptr()) }
			}
			pub fn parent_name(&self) -> String {
				unsafe { prop_motor_6_d_parent_name(self.to_ptr()) }
			}
			pub fn transform(&self) -> CFrame {
				unsafe { prop_motor_6_d_transform(self.to_ptr()) }
			}
			pub fn set_transform(&self, value: CFrame) {
				unsafe { prop_set_motor_6_d_transform(self.to_ptr(), value) }
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
				unsafe { prop_velocity_motor_current_angle(self.to_ptr()) }
			}
			pub fn set_current_angle(&self, value: f64) {
				unsafe { prop_set_velocity_motor_current_angle(self.to_ptr(), value) }
			}
			pub fn desired_angle(&self) -> f64 {
				unsafe { prop_velocity_motor_desired_angle(self.to_ptr()) }
			}
			pub fn set_desired_angle(&self, value: f64) {
				unsafe { prop_set_velocity_motor_desired_angle(self.to_ptr(), value) }
			}
			pub fn max_velocity(&self) -> f64 {
				unsafe { prop_velocity_motor_max_velocity(self.to_ptr()) }
			}
			pub fn set_max_velocity(&self, value: f64) {
				unsafe { prop_set_velocity_motor_max_velocity(self.to_ptr(), value) }
			}
		}
		impl From<$name> for JointInstance {
			fn from(value: $name) -> JointInstance {
				JointInstance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_joints_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_keyframe {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn time(&self) -> f64 {
				unsafe { prop_keyframe_time(self.to_ptr()) }
			}
			pub fn set_time(&self, value: f64) {
				unsafe { prop_set_keyframe_time(self.to_ptr(), value) }
			}
			pub fn fn_add_marker(&self, marker: Option<Instance>) {
				unsafe { dyn_keyframe_add_marker(self.to_ptr(), marker) }
			}
			pub fn fn_add_pose(&self, pose: Option<Instance>) {
				unsafe { dyn_keyframe_add_pose(self.to_ptr(), pose) }
			}
			pub fn fn_get_markers(&self) -> Objects {
				unsafe { dyn_keyframe_get_markers(self.to_ptr()) }
			}
			pub fn fn_get_poses(&self) -> Objects {
				unsafe { dyn_keyframe_get_poses(self.to_ptr()) }
			}
			pub fn fn_remove_marker(&self, marker: Option<Instance>) {
				unsafe { dyn_keyframe_remove_marker(self.to_ptr(), marker) }
			}
			pub fn fn_remove_pose(&self, pose: Option<Instance>) {
				unsafe { dyn_keyframe_remove_pose(self.to_ptr(), pose) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_keyframe_marker {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn value(&self) -> String {
				unsafe { prop_keyframe_marker_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: &str) {
				unsafe { prop_set_keyframe_marker_value(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_keyframe_sequence_provider {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_register_active_keyframe_sequence(&self, keyframe_sequence: Option<Instance>) -> Content {
				unsafe { dyn_keyframe_sequence_provider_register_active_keyframe_sequence(self.to_ptr(), keyframe_sequence) }
			}
			pub fn fn_register_keyframe_sequence(&self, keyframe_sequence: Option<Instance>) -> Content {
				unsafe { dyn_keyframe_sequence_provider_register_keyframe_sequence(self.to_ptr(), keyframe_sequence) }
			}
			pub fn fn_get_animations(&self, user_id: f64) -> Option<Instance> {
				unsafe { dyn_keyframe_sequence_provider_get_animations(self.to_ptr(), user_id) }
			}
			pub fn fn_get_keyframe_sequence_async(&self, asset_id: Content) -> Option<Instance> {
				unsafe { dyn_keyframe_sequence_provider_get_keyframe_sequence_async(self.to_ptr(), asset_id) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_light {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn brightness(&self) -> f64 {
				unsafe { prop_light_brightness(self.to_ptr()) }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { prop_set_light_brightness(self.to_ptr(), value) }
			}
			pub fn color(&self) -> Color3 {
				unsafe { prop_light_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { prop_set_light_color(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_light_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_light_enabled(self.to_ptr(), value) }
			}
			pub fn shadows(&self) -> bool {
				unsafe { prop_light_shadows(self.to_ptr()) }
			}
			pub fn set_shadows(&self, value: bool) {
				unsafe { prop_set_light_shadows(self.to_ptr(), value) }
			}
		}
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
				unsafe { prop_point_light_range(self.to_ptr()) }
			}
			pub fn set_range(&self, value: f64) {
				unsafe { prop_set_point_light_range(self.to_ptr(), value) }
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
				unsafe { prop_spot_light_angle(self.to_ptr()) }
			}
			pub fn set_angle(&self, value: f64) {
				unsafe { prop_set_spot_light_angle(self.to_ptr(), value) }
			}
			pub fn range(&self) -> f64 {
				unsafe { prop_spot_light_range(self.to_ptr()) }
			}
			pub fn set_range(&self, value: f64) {
				unsafe { prop_set_spot_light_range(self.to_ptr(), value) }
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
				unsafe { prop_surface_light_angle(self.to_ptr()) }
			}
			pub fn set_angle(&self, value: f64) {
				unsafe { prop_set_surface_light_angle(self.to_ptr(), value) }
			}
			pub fn range(&self) -> f64 {
				unsafe { prop_surface_light_range(self.to_ptr()) }
			}
			pub fn set_range(&self, value: f64) {
				unsafe { prop_set_surface_light_range(self.to_ptr(), value) }
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn ambient(&self) -> Color3 {
				unsafe { prop_lighting_ambient(self.to_ptr()) }
			}
			pub fn set_ambient(&self, value: Color3) {
				unsafe { prop_set_lighting_ambient(self.to_ptr(), value) }
			}
			pub fn brightness(&self) -> f64 {
				unsafe { prop_lighting_brightness(self.to_ptr()) }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { prop_set_lighting_brightness(self.to_ptr(), value) }
			}
			pub fn clock_time(&self) -> f64 {
				unsafe { prop_lighting_clock_time(self.to_ptr()) }
			}
			pub fn set_clock_time(&self, value: f64) {
				unsafe { prop_set_lighting_clock_time(self.to_ptr(), value) }
			}
			pub fn color_shift_bottom(&self) -> Color3 {
				unsafe { prop_lighting_color_shift_bottom(self.to_ptr()) }
			}
			pub fn set_color_shift_bottom(&self, value: Color3) {
				unsafe { prop_set_lighting_color_shift_bottom(self.to_ptr(), value) }
			}
			pub fn color_shift_top(&self) -> Color3 {
				unsafe { prop_lighting_color_shift_top(self.to_ptr()) }
			}
			pub fn set_color_shift_top(&self, value: Color3) {
				unsafe { prop_set_lighting_color_shift_top(self.to_ptr(), value) }
			}
			pub fn environment_diffuse_scale(&self) -> f64 {
				unsafe { prop_lighting_environment_diffuse_scale(self.to_ptr()) }
			}
			pub fn set_environment_diffuse_scale(&self, value: f64) {
				unsafe { prop_set_lighting_environment_diffuse_scale(self.to_ptr(), value) }
			}
			pub fn environment_specular_scale(&self) -> f64 {
				unsafe { prop_lighting_environment_specular_scale(self.to_ptr()) }
			}
			pub fn set_environment_specular_scale(&self, value: f64) {
				unsafe { prop_set_lighting_environment_specular_scale(self.to_ptr(), value) }
			}
			pub fn exposure_compensation(&self) -> f64 {
				unsafe { prop_lighting_exposure_compensation(self.to_ptr()) }
			}
			pub fn set_exposure_compensation(&self, value: f64) {
				unsafe { prop_set_lighting_exposure_compensation(self.to_ptr(), value) }
			}
			pub fn fog_color(&self) -> Color3 {
				unsafe { prop_lighting_fog_color(self.to_ptr()) }
			}
			pub fn set_fog_color(&self, value: Color3) {
				unsafe { prop_set_lighting_fog_color(self.to_ptr(), value) }
			}
			pub fn fog_end(&self) -> f64 {
				unsafe { prop_lighting_fog_end(self.to_ptr()) }
			}
			pub fn set_fog_end(&self, value: f64) {
				unsafe { prop_set_lighting_fog_end(self.to_ptr(), value) }
			}
			pub fn fog_start(&self) -> f64 {
				unsafe { prop_lighting_fog_start(self.to_ptr()) }
			}
			pub fn set_fog_start(&self, value: f64) {
				unsafe { prop_set_lighting_fog_start(self.to_ptr(), value) }
			}
			pub fn geographic_latitude(&self) -> f64 {
				unsafe { prop_lighting_geographic_latitude(self.to_ptr()) }
			}
			pub fn set_geographic_latitude(&self, value: f64) {
				unsafe { prop_set_lighting_geographic_latitude(self.to_ptr(), value) }
			}
			pub fn global_shadows(&self) -> bool {
				unsafe { prop_lighting_global_shadows(self.to_ptr()) }
			}
			pub fn set_global_shadows(&self, value: bool) {
				unsafe { prop_set_lighting_global_shadows(self.to_ptr(), value) }
			}
			pub fn outdoor_ambient(&self) -> Color3 {
				unsafe { prop_lighting_outdoor_ambient(self.to_ptr()) }
			}
			pub fn set_outdoor_ambient(&self, value: Color3) {
				unsafe { prop_set_lighting_outdoor_ambient(self.to_ptr(), value) }
			}
			pub fn outlines(&self) -> bool {
				unsafe { prop_lighting_outlines(self.to_ptr()) }
			}
			pub fn set_outlines(&self, value: bool) {
				unsafe { prop_set_lighting_outlines(self.to_ptr(), value) }
			}
			pub fn shadow_color(&self) -> Color3 {
				unsafe { prop_lighting_shadow_color(self.to_ptr()) }
			}
			pub fn set_shadow_color(&self, value: Color3) {
				unsafe { prop_set_lighting_shadow_color(self.to_ptr(), value) }
			}
			pub fn shadow_softness(&self) -> f64 {
				unsafe { prop_lighting_shadow_softness(self.to_ptr()) }
			}
			pub fn set_shadow_softness(&self, value: f64) {
				unsafe { prop_set_lighting_shadow_softness(self.to_ptr(), value) }
			}
			pub fn time_of_day(&self) -> String {
				unsafe { prop_lighting_time_of_day(self.to_ptr()) }
			}
			pub fn set_time_of_day(&self, value: &str) {
				unsafe { prop_set_lighting_time_of_day(self.to_ptr(), value) }
			}
			pub fn fn_get_minutes_after_midnight(&self) -> f64 {
				unsafe { dyn_lighting_get_minutes_after_midnight(self.to_ptr()) }
			}
			pub fn fn_get_moon_direction(&self) -> Vector3 {
				unsafe { dyn_lighting_get_moon_direction(self.to_ptr()) }
			}
			pub fn fn_get_moon_phase(&self) -> f64 {
				unsafe { dyn_lighting_get_moon_phase(self.to_ptr()) }
			}
			pub fn fn_get_sun_direction(&self) -> Vector3 {
				unsafe { dyn_lighting_get_sun_direction(self.to_ptr()) }
			}
			pub fn fn_set_minutes_after_midnight(&self, minutes: f64) {
				unsafe { dyn_lighting_set_minutes_after_midnight(self.to_ptr(), minutes) }
			}
			pub fn on_lighting_changed<F: 'static + Fn(bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_lighting_lighting_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_localization_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn roblox_locale_id(&self) -> String {
				unsafe { prop_localization_service_roblox_locale_id(self.to_ptr()) }
			}
			pub fn system_locale_id(&self) -> String {
				unsafe { prop_localization_service_system_locale_id(self.to_ptr()) }
			}
			pub fn fn_get_corescript_localizations(&self) -> Objects {
				unsafe { dyn_localization_service_get_corescript_localizations(self.to_ptr()) }
			}
			pub fn fn_get_table_entries(&self, instance: Option<Instance>) {
				unsafe { dyn_localization_service_get_table_entries(self.to_ptr(), instance) }
			}
			pub fn fn_get_translator_for_player(&self, player: Option<Instance>) -> Option<Instance> {
				unsafe { dyn_localization_service_get_translator_for_player(self.to_ptr(), player) }
			}
			pub fn fn_get_country_region_for_player_async(&self, player: Option<Instance>) -> String {
				unsafe { dyn_localization_service_get_country_region_for_player_async(self.to_ptr(), player) }
			}
			pub fn fn_get_translator_for_locale_async(&self, locale: &str) -> Option<Instance> {
				unsafe { dyn_localization_service_get_translator_for_locale_async(self.to_ptr(), locale) }
			}
			pub fn fn_get_translator_for_player_async(&self, player: Option<Instance>) -> Option<Instance> {
				unsafe { dyn_localization_service_get_translator_for_player_async(self.to_ptr(), player) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_localization_table {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn development_language(&self) -> String {
				unsafe { prop_localization_table_development_language(self.to_ptr()) }
			}
			pub fn set_development_language(&self, value: &str) {
				unsafe { prop_set_localization_table_development_language(self.to_ptr(), value) }
			}
			pub fn root(&self) -> Option<Instance> {
				unsafe { prop_localization_table_root(self.to_ptr()) }
			}
			pub fn set_root(&self, value: Option<Instance>) {
				unsafe { prop_set_localization_table_root(self.to_ptr(), value) }
			}
			pub fn source_locale_id(&self) -> String {
				unsafe { prop_localization_table_source_locale_id(self.to_ptr()) }
			}
			pub fn set_source_locale_id(&self, value: &str) {
				unsafe { prop_set_localization_table_source_locale_id(self.to_ptr(), value) }
			}
			pub fn fn_get_contents(&self) -> String {
				unsafe { dyn_localization_table_get_contents(self.to_ptr()) }
			}
			pub fn fn_get_entries(&self) {
				unsafe { dyn_localization_table_get_entries(self.to_ptr()) }
			}
			pub fn fn_get_string(&self, target_locale_id: &str, key: &str) -> String {
				unsafe { dyn_localization_table_get_string(self.to_ptr(), target_locale_id, key) }
			}
			pub fn fn_get_translator(&self, locale_id: &str) -> Option<Instance> {
				unsafe { dyn_localization_table_get_translator(self.to_ptr(), locale_id) }
			}
			pub fn fn_remove_entry(&self, key: &str, source: &str, context: &str) {
				unsafe { dyn_localization_table_remove_entry(self.to_ptr(), key, source, context) }
			}
			pub fn fn_remove_entry_value(&self, key: &str, source: &str, context: &str, locale_id: &str) {
				unsafe { dyn_localization_table_remove_entry_value(self.to_ptr(), key, source, context, locale_id) }
			}
			pub fn fn_remove_key(&self, key: &str) {
				unsafe { dyn_localization_table_remove_key(self.to_ptr(), key) }
			}
			pub fn fn_remove_target_locale(&self, locale_id: &str) {
				unsafe { dyn_localization_table_remove_target_locale(self.to_ptr(), locale_id) }
			}
			pub fn fn_set_contents(&self, contents: &str) {
				unsafe { dyn_localization_table_set_contents(self.to_ptr(), contents) }
			}
			pub fn fn_set_entry(&self, key: &str, target_locale_id: &str, text: &str) {
				unsafe { dyn_localization_table_set_entry(self.to_ptr(), key, target_locale_id, text) }
			}
			pub fn fn_set_entry_context(&self, key: &str, source: &str, context: &str, new_context: &str) {
				unsafe { dyn_localization_table_set_entry_context(self.to_ptr(), key, source, context, new_context) }
			}
			pub fn fn_set_entry_example(&self, key: &str, source: &str, context: &str, example: &str) {
				unsafe { dyn_localization_table_set_entry_example(self.to_ptr(), key, source, context, example) }
			}
			pub fn fn_set_entry_key(&self, key: &str, source: &str, context: &str, new_key: &str) {
				unsafe { dyn_localization_table_set_entry_key(self.to_ptr(), key, source, context, new_key) }
			}
			pub fn fn_set_entry_source(&self, key: &str, source: &str, context: &str, new_source: &str) {
				unsafe { dyn_localization_table_set_entry_source(self.to_ptr(), key, source, context, new_source) }
			}
			pub fn fn_set_entry_value(&self, key: &str, source: &str, context: &str, locale_id: &str, text: &str) {
				unsafe { dyn_localization_table_set_entry_value(self.to_ptr(), key, source, context, locale_id, text) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_lod_data_entity {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn entity_lod_enabled(&self) -> bool {
				unsafe { prop_lod_data_entity_entity_lod_enabled(self.to_ptr()) }
			}
			pub fn set_entity_lod_enabled(&self, value: bool) {
				unsafe { prop_set_lod_data_entity_entity_lod_enabled(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_log_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_log_history(&self) {
				unsafe { dyn_log_service_get_log_history(self.to_ptr()) }
			}
			pub fn on_message_out<F: 'static + Fn(String, ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_log_service_message_out(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_lua_source_container {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn current_editor(&self) -> Option<Instance> {
				unsafe { prop_lua_source_container_current_editor(self.to_ptr()) }
			}
			pub fn set_current_editor(&self, value: Option<Instance>) {
				unsafe { prop_set_lua_source_container_current_editor(self.to_ptr(), value) }
			}
		}
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
				unsafe { prop_base_script_disabled(self.to_ptr()) }
			}
			pub fn set_disabled(&self, value: bool) {
				unsafe { prop_set_base_script_disabled(self.to_ptr(), value) }
			}
			pub fn linked_source(&self) -> Content {
				unsafe { prop_base_script_linked_source(self.to_ptr()) }
			}
			pub fn set_linked_source(&self, value: Content) {
				unsafe { prop_set_base_script_linked_source(self.to_ptr(), value) }
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
				unsafe { prop_module_script_linked_source(self.to_ptr()) }
			}
			pub fn set_linked_source(&self, value: Content) {
				unsafe { prop_set_module_script_linked_source(self.to_ptr(), value) }
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn length(&self) -> f64 {
				unsafe { prop_marker_curve_length(self.to_ptr()) }
			}
			pub fn fn_get_marker_at_index(&self, index: f64) {
				unsafe { dyn_marker_curve_get_marker_at_index(self.to_ptr(), index) }
			}
			pub fn fn_get_markers(&self) {
				unsafe { dyn_marker_curve_get_markers(self.to_ptr()) }
			}
			pub fn fn_insert_marker_at_time(&self, time: f64, marker: &str) {
				unsafe { dyn_marker_curve_insert_marker_at_time(self.to_ptr(), time, marker) }
			}
			pub fn fn_remove_marker_at_index(&self, starting_index: f64, count: f64) -> f64 {
				unsafe { dyn_marker_curve_remove_marker_at_index(self.to_ptr(), starting_index, count) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_marketplace_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_prompt_bundle_purchase(&self, player: Option<Instance>, bundle_id: f64) {
				unsafe { dyn_marketplace_service_prompt_bundle_purchase(self.to_ptr(), player, bundle_id) }
			}
			pub fn fn_prompt_game_pass_purchase(&self, player: Option<Instance>, game_pass_id: f64) {
				unsafe { dyn_marketplace_service_prompt_game_pass_purchase(self.to_ptr(), player, game_pass_id) }
			}
			pub fn fn_prompt_premium_purchase(&self, player: Option<Instance>) {
				unsafe { dyn_marketplace_service_prompt_premium_purchase(self.to_ptr(), player) }
			}
			pub fn fn_prompt_subscription_cancellation(&self, player: Option<Instance>, subscription_id: f64) {
				unsafe { dyn_marketplace_service_prompt_subscription_cancellation(self.to_ptr(), player, subscription_id) }
			}
			pub fn fn_prompt_subscription_purchase(&self, player: Option<Instance>, subscription_id: f64) {
				unsafe { dyn_marketplace_service_prompt_subscription_purchase(self.to_ptr(), player, subscription_id) }
			}
			pub fn fn_get_developer_products_async(&self) -> Option<Instance> {
				unsafe { dyn_marketplace_service_get_developer_products_async(self.to_ptr()) }
			}
			pub fn fn_is_player_subscribed(&self, player: Option<Instance>, subscription_id: f64) -> bool {
				unsafe { dyn_marketplace_service_is_player_subscribed(self.to_ptr(), player, subscription_id) }
			}
			pub fn fn_player_owns_asset(&self, player: Option<Instance>, asset_id: f64) -> bool {
				unsafe { dyn_marketplace_service_player_owns_asset(self.to_ptr(), player, asset_id) }
			}
			pub fn fn_player_owns_bundle(&self, player: Option<Player>, bundle_id: f64) -> bool {
				unsafe { dyn_marketplace_service_player_owns_bundle(self.to_ptr(), player, bundle_id) }
			}
			pub fn fn_user_owns_game_pass_async(&self, user_id: f64, game_pass_id: f64) -> bool {
				unsafe { dyn_marketplace_service_user_owns_game_pass_async(self.to_ptr(), user_id, game_pass_id) }
			}
			pub fn on_prompt_bundle_purchase_finished<F: 'static + Fn(Option<Instance>, f64, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_marketplace_service_prompt_bundle_purchase_finished(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_game_pass_purchase_finished<F: 'static + Fn(Option<Instance>, f64, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_marketplace_service_prompt_game_pass_purchase_finished(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_premium_purchase_finished<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_marketplace_service_prompt_premium_purchase_finished(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_product_purchase_finished<F: 'static + Fn(f64, f64, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_marketplace_service_prompt_product_purchase_finished(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_purchase_finished<F: 'static + Fn(Option<Instance>, f64, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_marketplace_service_prompt_purchase_finished(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_subscription_cancellation_finished<F: 'static + Fn(Option<Instance>, f64, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_marketplace_service_prompt_subscription_cancellation_finished(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_subscription_purchase_finished<F: 'static + Fn(Option<Instance>, f64, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_marketplace_service_prompt_subscription_purchase_finished(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_material_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_material_variant {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn studs_per_tile(&self) -> f64 {
				unsafe { prop_material_variant_studs_per_tile(self.to_ptr()) }
			}
			pub fn set_studs_per_tile(&self, value: f64) {
				unsafe { prop_set_material_variant_studs_per_tile(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_memory_store_queue {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_read_async(&self, count: f64, all_or_nothing: bool, wait_timeout: f64) {
				unsafe { dyn_memory_store_queue_read_async(self.to_ptr(), count, all_or_nothing, wait_timeout) }
			}
			pub fn fn_remove_async(&self, id: &str) {
				unsafe { dyn_memory_store_queue_remove_async(self.to_ptr(), id) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_memory_store_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_queue(&self, name: &str, invisibility_timeout: f64) -> Option<MemoryStoreQueue> {
				unsafe { dyn_memory_store_service_get_queue(self.to_ptr(), name, invisibility_timeout) }
			}
			pub fn fn_get_sorted_map(&self, name: &str) -> Option<MemoryStoreSortedMap> {
				unsafe { dyn_memory_store_service_get_sorted_map(self.to_ptr(), name) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_memory_store_sorted_map {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_async(&self, key: &str) {
				unsafe { dyn_memory_store_sorted_map_get_async(self.to_ptr(), key) }
			}
			pub fn fn_remove_async(&self, key: &str) {
				unsafe { dyn_memory_store_sorted_map_remove_async(self.to_ptr(), key) }
			}
			pub fn fn_update_async(&self, key: &str, transform_function: Function, expiration: f64) {
				unsafe { dyn_memory_store_sorted_map_update_async(self.to_ptr(), key, transform_function, expiration) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_messaging_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_subscribe_async(&self, topic: &str, callback: Function) -> RbxScriptConnection {
				unsafe { dyn_messaging_service_subscribe_async(self.to_ptr(), topic, callback) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_mouse {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn hit(&self) -> CFrame {
				unsafe { prop_mouse_hit(self.to_ptr()) }
			}
			pub fn icon(&self) -> Content {
				unsafe { prop_mouse_icon(self.to_ptr()) }
			}
			pub fn set_icon(&self, value: Content) {
				unsafe { prop_set_mouse_icon(self.to_ptr(), value) }
			}
			pub fn origin(&self) -> CFrame {
				unsafe { prop_mouse_origin(self.to_ptr()) }
			}
			pub fn target(&self) -> Option<BasePart> {
				unsafe { prop_mouse_target(self.to_ptr()) }
			}
			pub fn target_filter(&self) -> Option<Instance> {
				unsafe { prop_mouse_target_filter(self.to_ptr()) }
			}
			pub fn set_target_filter(&self, value: Option<Instance>) {
				unsafe { prop_set_mouse_target_filter(self.to_ptr(), value) }
			}
			pub fn unit_ray(&self) -> Ray {
				unsafe { prop_mouse_unit_ray(self.to_ptr()) }
			}
			pub fn view_size_x(&self) -> f64 {
				unsafe { prop_mouse_view_size_x(self.to_ptr()) }
			}
			pub fn view_size_y(&self) -> f64 {
				unsafe { prop_mouse_view_size_y(self.to_ptr()) }
			}
			pub fn x(&self) -> f64 {
				unsafe { prop_mouse_x(self.to_ptr()) }
			}
			pub fn y(&self) -> f64 {
				unsafe { prop_mouse_y(self.to_ptr()) }
			}
			pub fn on_button_1_down<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_mouse_button_1_down(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_button_1_up<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_mouse_button_1_up(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_button_2_down<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_mouse_button_2_down(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_button_2_up<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_mouse_button_2_up(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_idle<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_mouse_idle(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_key_down<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_mouse_key_down(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_key_up<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_mouse_key_up(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_move<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_mouse_move(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_wheel_backward<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_mouse_wheel_backward(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_wheel_forward<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_mouse_wheel_forward(self.to_ptr(), Box::new(callback))) }
			}
		}
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn on_received<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_network_marker_received(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_no_collision_constraint {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn enabled(&self) -> bool {
				unsafe { prop_no_collision_constraint_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_no_collision_constraint_enabled(self.to_ptr(), value) }
			}
			pub fn part_0(&self) -> Option<BasePart> {
				unsafe { prop_no_collision_constraint_part_0(self.to_ptr()) }
			}
			pub fn set_part_0(&self, value: Option<BasePart>) {
				unsafe { prop_set_no_collision_constraint_part_0(self.to_ptr(), value) }
			}
			pub fn part_1(&self) -> Option<BasePart> {
				unsafe { prop_no_collision_constraint_part_1(self.to_ptr()) }
			}
			pub fn set_part_1(&self, value: Option<BasePart>) {
				unsafe { prop_set_no_collision_constraint_part_1(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pv_instance {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_pivot(&self) -> CFrame {
				unsafe { dyn_pv_instance_get_pivot(self.to_ptr()) }
			}
			pub fn fn_pivot_to(&self, target_c_frame: CFrame) {
				unsafe { dyn_pv_instance_pivot_to(self.to_ptr(), target_c_frame) }
			}
		}
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
				unsafe { prop_base_part_anchored(self.to_ptr()) }
			}
			pub fn set_anchored(&self, value: bool) {
				unsafe { prop_set_base_part_anchored(self.to_ptr(), value) }
			}
			pub fn assembly_angular_velocity(&self) -> Vector3 {
				unsafe { prop_base_part_assembly_angular_velocity(self.to_ptr()) }
			}
			pub fn set_assembly_angular_velocity(&self, value: Vector3) {
				unsafe { prop_set_base_part_assembly_angular_velocity(self.to_ptr(), value) }
			}
			pub fn assembly_center_of_mass(&self) -> Vector3 {
				unsafe { prop_base_part_assembly_center_of_mass(self.to_ptr()) }
			}
			pub fn assembly_linear_velocity(&self) -> Vector3 {
				unsafe { prop_base_part_assembly_linear_velocity(self.to_ptr()) }
			}
			pub fn set_assembly_linear_velocity(&self, value: Vector3) {
				unsafe { prop_set_base_part_assembly_linear_velocity(self.to_ptr(), value) }
			}
			pub fn assembly_mass(&self) -> f64 {
				unsafe { prop_base_part_assembly_mass(self.to_ptr()) }
			}
			pub fn assembly_root_part(&self) -> Option<BasePart> {
				unsafe { prop_base_part_assembly_root_part(self.to_ptr()) }
			}
			pub fn back_param_a(&self) -> f64 {
				unsafe { prop_base_part_back_param_a(self.to_ptr()) }
			}
			pub fn set_back_param_a(&self, value: f64) {
				unsafe { prop_set_base_part_back_param_a(self.to_ptr(), value) }
			}
			pub fn back_param_b(&self) -> f64 {
				unsafe { prop_base_part_back_param_b(self.to_ptr()) }
			}
			pub fn set_back_param_b(&self, value: f64) {
				unsafe { prop_set_base_part_back_param_b(self.to_ptr(), value) }
			}
			pub fn bottom_param_a(&self) -> f64 {
				unsafe { prop_base_part_bottom_param_a(self.to_ptr()) }
			}
			pub fn set_bottom_param_a(&self, value: f64) {
				unsafe { prop_set_base_part_bottom_param_a(self.to_ptr(), value) }
			}
			pub fn bottom_param_b(&self) -> f64 {
				unsafe { prop_base_part_bottom_param_b(self.to_ptr()) }
			}
			pub fn set_bottom_param_b(&self, value: f64) {
				unsafe { prop_set_base_part_bottom_param_b(self.to_ptr(), value) }
			}
			pub fn brick_color(&self) -> BrickColor {
				unsafe { prop_base_part_brick_color(self.to_ptr()) }
			}
			pub fn set_brick_color(&self, value: BrickColor) {
				unsafe { prop_set_base_part_brick_color(self.to_ptr(), value) }
			}
			pub fn c_frame(&self) -> CFrame {
				unsafe { prop_base_part_c_frame(self.to_ptr()) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { prop_set_base_part_c_frame(self.to_ptr(), value) }
			}
			pub fn can_collide(&self) -> bool {
				unsafe { prop_base_part_can_collide(self.to_ptr()) }
			}
			pub fn set_can_collide(&self, value: bool) {
				unsafe { prop_set_base_part_can_collide(self.to_ptr(), value) }
			}
			pub fn can_query(&self) -> bool {
				unsafe { prop_base_part_can_query(self.to_ptr()) }
			}
			pub fn set_can_query(&self, value: bool) {
				unsafe { prop_set_base_part_can_query(self.to_ptr(), value) }
			}
			pub fn can_touch(&self) -> bool {
				unsafe { prop_base_part_can_touch(self.to_ptr()) }
			}
			pub fn set_can_touch(&self, value: bool) {
				unsafe { prop_set_base_part_can_touch(self.to_ptr(), value) }
			}
			pub fn cast_shadow(&self) -> bool {
				unsafe { prop_base_part_cast_shadow(self.to_ptr()) }
			}
			pub fn set_cast_shadow(&self, value: bool) {
				unsafe { prop_set_base_part_cast_shadow(self.to_ptr(), value) }
			}
			pub fn center_of_mass(&self) -> Vector3 {
				unsafe { prop_base_part_center_of_mass(self.to_ptr()) }
			}
			pub fn collision_group_id(&self) -> f64 {
				unsafe { prop_base_part_collision_group_id(self.to_ptr()) }
			}
			pub fn set_collision_group_id(&self, value: f64) {
				unsafe { prop_set_base_part_collision_group_id(self.to_ptr(), value) }
			}
			pub fn color(&self) -> Color3 {
				unsafe { prop_base_part_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { prop_set_base_part_color(self.to_ptr(), value) }
			}
			pub fn custom_physical_properties(&self) -> PhysicalProperties {
				unsafe { prop_base_part_custom_physical_properties(self.to_ptr()) }
			}
			pub fn set_custom_physical_properties(&self, value: PhysicalProperties) {
				unsafe { prop_set_base_part_custom_physical_properties(self.to_ptr(), value) }
			}
			pub fn elasticity(&self) -> f64 {
				unsafe { prop_base_part_elasticity(self.to_ptr()) }
			}
			pub fn set_elasticity(&self, value: f64) {
				unsafe { prop_set_base_part_elasticity(self.to_ptr(), value) }
			}
			pub fn friction(&self) -> f64 {
				unsafe { prop_base_part_friction(self.to_ptr()) }
			}
			pub fn set_friction(&self, value: f64) {
				unsafe { prop_set_base_part_friction(self.to_ptr(), value) }
			}
			pub fn front_param_a(&self) -> f64 {
				unsafe { prop_base_part_front_param_a(self.to_ptr()) }
			}
			pub fn set_front_param_a(&self, value: f64) {
				unsafe { prop_set_base_part_front_param_a(self.to_ptr(), value) }
			}
			pub fn front_param_b(&self) -> f64 {
				unsafe { prop_base_part_front_param_b(self.to_ptr()) }
			}
			pub fn set_front_param_b(&self, value: f64) {
				unsafe { prop_set_base_part_front_param_b(self.to_ptr(), value) }
			}
			pub fn left_param_a(&self) -> f64 {
				unsafe { prop_base_part_left_param_a(self.to_ptr()) }
			}
			pub fn set_left_param_a(&self, value: f64) {
				unsafe { prop_set_base_part_left_param_a(self.to_ptr(), value) }
			}
			pub fn left_param_b(&self) -> f64 {
				unsafe { prop_base_part_left_param_b(self.to_ptr()) }
			}
			pub fn set_left_param_b(&self, value: f64) {
				unsafe { prop_set_base_part_left_param_b(self.to_ptr(), value) }
			}
			pub fn local_transparency_modifier(&self) -> f64 {
				unsafe { prop_base_part_local_transparency_modifier(self.to_ptr()) }
			}
			pub fn set_local_transparency_modifier(&self, value: f64) {
				unsafe { prop_set_base_part_local_transparency_modifier(self.to_ptr(), value) }
			}
			pub fn locked(&self) -> bool {
				unsafe { prop_base_part_locked(self.to_ptr()) }
			}
			pub fn set_locked(&self, value: bool) {
				unsafe { prop_set_base_part_locked(self.to_ptr(), value) }
			}
			pub fn mass(&self) -> f64 {
				unsafe { prop_base_part_mass(self.to_ptr()) }
			}
			pub fn massless(&self) -> bool {
				unsafe { prop_base_part_massless(self.to_ptr()) }
			}
			pub fn set_massless(&self, value: bool) {
				unsafe { prop_set_base_part_massless(self.to_ptr(), value) }
			}
			pub fn material_variant(&self) -> String {
				unsafe { prop_base_part_material_variant(self.to_ptr()) }
			}
			pub fn set_material_variant(&self, value: &str) {
				unsafe { prop_set_base_part_material_variant(self.to_ptr(), value) }
			}
			pub fn orientation(&self) -> Vector3 {
				unsafe { prop_base_part_orientation(self.to_ptr()) }
			}
			pub fn set_orientation(&self, value: Vector3) {
				unsafe { prop_set_base_part_orientation(self.to_ptr(), value) }
			}
			pub fn pivot_offset(&self) -> CFrame {
				unsafe { prop_base_part_pivot_offset(self.to_ptr()) }
			}
			pub fn set_pivot_offset(&self, value: CFrame) {
				unsafe { prop_set_base_part_pivot_offset(self.to_ptr(), value) }
			}
			pub fn position(&self) -> Vector3 {
				unsafe { prop_base_part_position(self.to_ptr()) }
			}
			pub fn set_position(&self, value: Vector3) {
				unsafe { prop_set_base_part_position(self.to_ptr(), value) }
			}
			pub fn receive_age(&self) -> f64 {
				unsafe { prop_base_part_receive_age(self.to_ptr()) }
			}
			pub fn reflectance(&self) -> f64 {
				unsafe { prop_base_part_reflectance(self.to_ptr()) }
			}
			pub fn set_reflectance(&self, value: f64) {
				unsafe { prop_set_base_part_reflectance(self.to_ptr(), value) }
			}
			pub fn resize_increment(&self) -> f64 {
				unsafe { prop_base_part_resize_increment(self.to_ptr()) }
			}
			pub fn resizeable_faces(&self) -> Faces {
				unsafe { prop_base_part_resizeable_faces(self.to_ptr()) }
			}
			pub fn right_param_a(&self) -> f64 {
				unsafe { prop_base_part_right_param_a(self.to_ptr()) }
			}
			pub fn set_right_param_a(&self, value: f64) {
				unsafe { prop_set_base_part_right_param_a(self.to_ptr(), value) }
			}
			pub fn right_param_b(&self) -> f64 {
				unsafe { prop_base_part_right_param_b(self.to_ptr()) }
			}
			pub fn set_right_param_b(&self, value: f64) {
				unsafe { prop_set_base_part_right_param_b(self.to_ptr(), value) }
			}
			pub fn root_priority(&self) -> f64 {
				unsafe { prop_base_part_root_priority(self.to_ptr()) }
			}
			pub fn set_root_priority(&self, value: f64) {
				unsafe { prop_set_base_part_root_priority(self.to_ptr(), value) }
			}
			pub fn rot_velocity(&self) -> Vector3 {
				unsafe { prop_base_part_rot_velocity(self.to_ptr()) }
			}
			pub fn set_rot_velocity(&self, value: Vector3) {
				unsafe { prop_set_base_part_rot_velocity(self.to_ptr(), value) }
			}
			pub fn rotation(&self) -> Vector3 {
				unsafe { prop_base_part_rotation(self.to_ptr()) }
			}
			pub fn set_rotation(&self, value: Vector3) {
				unsafe { prop_set_base_part_rotation(self.to_ptr(), value) }
			}
			pub fn size(&self) -> Vector3 {
				unsafe { prop_base_part_size(self.to_ptr()) }
			}
			pub fn set_size(&self, value: Vector3) {
				unsafe { prop_set_base_part_size(self.to_ptr(), value) }
			}
			pub fn specific_gravity(&self) -> f64 {
				unsafe { prop_base_part_specific_gravity(self.to_ptr()) }
			}
			pub fn top_param_a(&self) -> f64 {
				unsafe { prop_base_part_top_param_a(self.to_ptr()) }
			}
			pub fn set_top_param_a(&self, value: f64) {
				unsafe { prop_set_base_part_top_param_a(self.to_ptr(), value) }
			}
			pub fn top_param_b(&self) -> f64 {
				unsafe { prop_base_part_top_param_b(self.to_ptr()) }
			}
			pub fn set_top_param_b(&self, value: f64) {
				unsafe { prop_set_base_part_top_param_b(self.to_ptr(), value) }
			}
			pub fn transparency(&self) -> f64 {
				unsafe { prop_base_part_transparency(self.to_ptr()) }
			}
			pub fn set_transparency(&self, value: f64) {
				unsafe { prop_set_base_part_transparency(self.to_ptr(), value) }
			}
			pub fn velocity(&self) -> Vector3 {
				unsafe { prop_base_part_velocity(self.to_ptr()) }
			}
			pub fn set_velocity(&self, value: Vector3) {
				unsafe { prop_set_base_part_velocity(self.to_ptr(), value) }
			}
			pub fn fn_apply_angular_impulse(&self, impulse: Vector3) {
				unsafe { dyn_base_part_apply_angular_impulse(self.to_ptr(), impulse) }
			}
			pub fn fn_apply_impulse(&self, impulse: Vector3) {
				unsafe { dyn_base_part_apply_impulse(self.to_ptr(), impulse) }
			}
			pub fn fn_apply_impulse_at_position(&self, impulse: Vector3, position: Vector3) {
				unsafe { dyn_base_part_apply_impulse_at_position(self.to_ptr(), impulse, position) }
			}
			pub fn fn_break_joints(&self) {
				unsafe { dyn_base_part_break_joints(self.to_ptr()) }
			}
			pub fn fn_can_collide_with(&self, part: Option<BasePart>) -> bool {
				unsafe { dyn_base_part_can_collide_with(self.to_ptr(), part) }
			}
			pub fn fn_can_set_network_ownership(&self) {
				unsafe { dyn_base_part_can_set_network_ownership(self.to_ptr()) }
			}
			pub fn fn_get_connected_parts(&self, recursive: bool) -> Objects {
				unsafe { dyn_base_part_get_connected_parts(self.to_ptr(), recursive) }
			}
			pub fn fn_get_joints(&self) -> Objects {
				unsafe { dyn_base_part_get_joints(self.to_ptr()) }
			}
			pub fn fn_get_mass(&self) -> f64 {
				unsafe { dyn_base_part_get_mass(self.to_ptr()) }
			}
			pub fn fn_get_network_owner(&self) -> Option<Instance> {
				unsafe { dyn_base_part_get_network_owner(self.to_ptr()) }
			}
			pub fn fn_get_network_ownership_auto(&self) -> bool {
				unsafe { dyn_base_part_get_network_ownership_auto(self.to_ptr()) }
			}
			pub fn fn_get_render_c_frame(&self) -> CFrame {
				unsafe { dyn_base_part_get_render_c_frame(self.to_ptr()) }
			}
			pub fn fn_get_root_part(&self) -> Option<Instance> {
				unsafe { dyn_base_part_get_root_part(self.to_ptr()) }
			}
			pub fn fn_get_touching_parts(&self) -> Objects {
				unsafe { dyn_base_part_get_touching_parts(self.to_ptr()) }
			}
			pub fn fn_get_velocity_at_position(&self, position: Vector3) -> Vector3 {
				unsafe { dyn_base_part_get_velocity_at_position(self.to_ptr(), position) }
			}
			pub fn fn_is_grounded(&self) -> bool {
				unsafe { dyn_base_part_is_grounded(self.to_ptr()) }
			}
			pub fn fn_make_joints(&self) {
				unsafe { dyn_base_part_make_joints(self.to_ptr()) }
			}
			pub fn fn_set_network_owner(&self, player_instance: Option<Player>) {
				unsafe { dyn_base_part_set_network_owner(self.to_ptr(), player_instance) }
			}
			pub fn fn_set_network_ownership_auto(&self) {
				unsafe { dyn_base_part_set_network_ownership_auto(self.to_ptr()) }
			}
			pub fn on_local_simulation_touched<F: 'static + Fn(Option<BasePart>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_base_part_local_simulation_touched(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_outfit_changed<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_base_part_outfit_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_stopped_touching<F: 'static + Fn(Option<BasePart>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_base_part_stopped_touching(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_ended<F: 'static + Fn(Option<BasePart>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_base_part_touch_ended(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touched<F: 'static + Fn(Option<BasePart>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_base_part_touched(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_seat_disabled(self.to_ptr()) }
			}
			pub fn set_disabled(&self, value: bool) {
				unsafe { prop_set_seat_disabled(self.to_ptr(), value) }
			}
			pub fn occupant(&self) -> Option<Humanoid> {
				unsafe { prop_seat_occupant(self.to_ptr()) }
			}
			pub fn fn_sit(&self, humanoid: Option<Instance>) {
				unsafe { dyn_seat_sit(self.to_ptr(), humanoid) }
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
				unsafe { prop_skateboard_platform_controller(self.to_ptr()) }
			}
			pub fn controlling_humanoid(&self) -> Option<Humanoid> {
				unsafe { prop_skateboard_platform_controlling_humanoid(self.to_ptr()) }
			}
			pub fn steer(&self) -> f64 {
				unsafe { prop_skateboard_platform_steer(self.to_ptr()) }
			}
			pub fn set_steer(&self, value: f64) {
				unsafe { prop_set_skateboard_platform_steer(self.to_ptr(), value) }
			}
			pub fn sticky_wheels(&self) -> bool {
				unsafe { prop_skateboard_platform_sticky_wheels(self.to_ptr()) }
			}
			pub fn set_sticky_wheels(&self, value: bool) {
				unsafe { prop_set_skateboard_platform_sticky_wheels(self.to_ptr(), value) }
			}
			pub fn throttle(&self) -> f64 {
				unsafe { prop_skateboard_platform_throttle(self.to_ptr()) }
			}
			pub fn set_throttle(&self, value: f64) {
				unsafe { prop_set_skateboard_platform_throttle(self.to_ptr(), value) }
			}
			pub fn on_equipped<F: 'static + Fn(Option<Instance>, Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_skateboard_platform_equipped(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_move_state_changed<F: 'static + Fn((), ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_skateboard_platform_move_state_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_unequipped<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_skateboard_platform_unequipped(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_spawn_location_allow_team_change_on_touch(self.to_ptr()) }
			}
			pub fn set_allow_team_change_on_touch(&self, value: bool) {
				unsafe { prop_set_spawn_location_allow_team_change_on_touch(self.to_ptr(), value) }
			}
			pub fn duration(&self) -> f64 {
				unsafe { prop_spawn_location_duration(self.to_ptr()) }
			}
			pub fn set_duration(&self, value: f64) {
				unsafe { prop_set_spawn_location_duration(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_spawn_location_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_spawn_location_enabled(self.to_ptr(), value) }
			}
			pub fn neutral(&self) -> bool {
				unsafe { prop_spawn_location_neutral(self.to_ptr()) }
			}
			pub fn set_neutral(&self, value: bool) {
				unsafe { prop_set_spawn_location_neutral(self.to_ptr(), value) }
			}
			pub fn team_color(&self) -> BrickColor {
				unsafe { prop_spawn_location_team_color(self.to_ptr()) }
			}
			pub fn set_team_color(&self, value: BrickColor) {
				unsafe { prop_set_spawn_location_team_color(self.to_ptr(), value) }
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
			pub fn decoration(&self) -> bool {
				unsafe { prop_terrain_decoration(self.to_ptr()) }
			}
			pub fn set_decoration(&self, value: bool) {
				unsafe { prop_set_terrain_decoration(self.to_ptr(), value) }
			}
			pub fn is_smooth(&self) -> bool {
				unsafe { prop_terrain_is_smooth(self.to_ptr()) }
			}
			pub fn material_colors(&self) -> BinaryString {
				unsafe { prop_terrain_material_colors(self.to_ptr()) }
			}
			pub fn set_material_colors(&self, value: BinaryString) {
				unsafe { prop_set_terrain_material_colors(self.to_ptr(), value) }
			}
			pub fn max_extents(&self) -> Region3int16 {
				unsafe { prop_terrain_max_extents(self.to_ptr()) }
			}
			pub fn water_color(&self) -> Color3 {
				unsafe { prop_terrain_water_color(self.to_ptr()) }
			}
			pub fn set_water_color(&self, value: Color3) {
				unsafe { prop_set_terrain_water_color(self.to_ptr(), value) }
			}
			pub fn water_reflectance(&self) -> f64 {
				unsafe { prop_terrain_water_reflectance(self.to_ptr()) }
			}
			pub fn set_water_reflectance(&self, value: f64) {
				unsafe { prop_set_terrain_water_reflectance(self.to_ptr(), value) }
			}
			pub fn water_transparency(&self) -> f64 {
				unsafe { prop_terrain_water_transparency(self.to_ptr()) }
			}
			pub fn set_water_transparency(&self, value: f64) {
				unsafe { prop_set_terrain_water_transparency(self.to_ptr(), value) }
			}
			pub fn water_wave_size(&self) -> f64 {
				unsafe { prop_terrain_water_wave_size(self.to_ptr()) }
			}
			pub fn set_water_wave_size(&self, value: f64) {
				unsafe { prop_set_terrain_water_wave_size(self.to_ptr(), value) }
			}
			pub fn water_wave_speed(&self) -> f64 {
				unsafe { prop_terrain_water_wave_speed(self.to_ptr()) }
			}
			pub fn set_water_wave_speed(&self, value: f64) {
				unsafe { prop_set_terrain_water_wave_speed(self.to_ptr(), value) }
			}
			pub fn fn_autowedge_cell(&self, x: f64, y: f64, z: f64) -> bool {
				unsafe { dyn_terrain_autowedge_cell(self.to_ptr(), x, y, z) }
			}
			pub fn fn_autowedge_cells(&self, region: Region3int16) {
				unsafe { dyn_terrain_autowedge_cells(self.to_ptr(), region) }
			}
			pub fn fn_cell_center_to_world(&self, x: f64, y: f64, z: f64) -> Vector3 {
				unsafe { dyn_terrain_cell_center_to_world(self.to_ptr(), x, y, z) }
			}
			pub fn fn_cell_corner_to_world(&self, x: f64, y: f64, z: f64) -> Vector3 {
				unsafe { dyn_terrain_cell_corner_to_world(self.to_ptr(), x, y, z) }
			}
			pub fn fn_clear(&self) {
				unsafe { dyn_terrain_clear(self.to_ptr()) }
			}
			pub fn fn_copy_region(&self, region: Region3int16) -> Option<TerrainRegion> {
				unsafe { dyn_terrain_copy_region(self.to_ptr(), region) }
			}
			pub fn fn_count_cells(&self) -> f64 {
				unsafe { dyn_terrain_count_cells(self.to_ptr()) }
			}
			pub fn fn_get_cell(&self, x: f64, y: f64, z: f64) {
				unsafe { dyn_terrain_get_cell(self.to_ptr(), x, y, z) }
			}
			pub fn fn_get_water_cell(&self, x: f64, y: f64, z: f64) {
				unsafe { dyn_terrain_get_water_cell(self.to_ptr(), x, y, z) }
			}
			pub fn fn_paste_region(&self, region: Option<TerrainRegion>, corner: Vector3int16, paste_empty_cells: bool) {
				unsafe { dyn_terrain_paste_region(self.to_ptr(), region, corner, paste_empty_cells) }
			}
			pub fn fn_read_voxels(&self, region: Region3, resolution: f64) {
				unsafe { dyn_terrain_read_voxels(self.to_ptr(), region, resolution) }
			}
			pub fn fn_world_to_cell(&self, position: Vector3) -> Vector3 {
				unsafe { dyn_terrain_world_to_cell(self.to_ptr(), position) }
			}
			pub fn fn_world_to_cell_prefer_empty(&self, position: Vector3) -> Vector3 {
				unsafe { dyn_terrain_world_to_cell_prefer_empty(self.to_ptr(), position) }
			}
			pub fn fn_world_to_cell_prefer_solid(&self, position: Vector3) -> Vector3 {
				unsafe { dyn_terrain_world_to_cell_prefer_solid(self.to_ptr(), position) }
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
				unsafe { prop_mesh_part_double_sided(self.to_ptr()) }
			}
			pub fn has_joint_offset(&self) -> bool {
				unsafe { prop_mesh_part_has_joint_offset(self.to_ptr()) }
			}
			pub fn has_skinned_mesh(&self) -> bool {
				unsafe { prop_mesh_part_has_skinned_mesh(self.to_ptr()) }
			}
			pub fn joint_offset(&self) -> Vector3 {
				unsafe { prop_mesh_part_joint_offset(self.to_ptr()) }
			}
			pub fn mesh_id(&self) -> Content {
				unsafe { prop_mesh_part_mesh_id(self.to_ptr()) }
			}
			pub fn mesh_size(&self) -> Vector3 {
				unsafe { prop_mesh_part_mesh_size(self.to_ptr()) }
			}
			pub fn texture_id(&self) -> Content {
				unsafe { prop_mesh_part_texture_id(self.to_ptr()) }
			}
			pub fn set_texture_id(&self, value: Content) {
				unsafe { prop_set_mesh_part_texture_id(self.to_ptr(), value) }
			}
			pub fn fn_apply_mesh(&self, mesh_part: Option<Instance>) {
				unsafe { dyn_mesh_part_apply_mesh(self.to_ptr(), mesh_part) }
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
				unsafe { prop_part_operation_smoothing_angle(self.to_ptr()) }
			}
			pub fn triangle_count(&self) -> f64 {
				unsafe { prop_part_operation_triangle_count(self.to_ptr()) }
			}
			pub fn use_part_color(&self) -> bool {
				unsafe { prop_part_operation_use_part_color(self.to_ptr()) }
			}
			pub fn set_use_part_color(&self, value: bool) {
				unsafe { prop_set_part_operation_use_part_color(self.to_ptr(), value) }
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
				unsafe { prop_vehicle_seat_are_hinges_detected(self.to_ptr()) }
			}
			pub fn disabled(&self) -> bool {
				unsafe { prop_vehicle_seat_disabled(self.to_ptr()) }
			}
			pub fn set_disabled(&self, value: bool) {
				unsafe { prop_set_vehicle_seat_disabled(self.to_ptr(), value) }
			}
			pub fn heads_up_display(&self) -> bool {
				unsafe { prop_vehicle_seat_heads_up_display(self.to_ptr()) }
			}
			pub fn set_heads_up_display(&self, value: bool) {
				unsafe { prop_set_vehicle_seat_heads_up_display(self.to_ptr(), value) }
			}
			pub fn max_speed(&self) -> f64 {
				unsafe { prop_vehicle_seat_max_speed(self.to_ptr()) }
			}
			pub fn set_max_speed(&self, value: f64) {
				unsafe { prop_set_vehicle_seat_max_speed(self.to_ptr(), value) }
			}
			pub fn occupant(&self) -> Option<Humanoid> {
				unsafe { prop_vehicle_seat_occupant(self.to_ptr()) }
			}
			pub fn steer(&self) -> f64 {
				unsafe { prop_vehicle_seat_steer(self.to_ptr()) }
			}
			pub fn set_steer(&self, value: f64) {
				unsafe { prop_set_vehicle_seat_steer(self.to_ptr(), value) }
			}
			pub fn steer_float(&self) -> f64 {
				unsafe { prop_vehicle_seat_steer_float(self.to_ptr()) }
			}
			pub fn set_steer_float(&self, value: f64) {
				unsafe { prop_set_vehicle_seat_steer_float(self.to_ptr(), value) }
			}
			pub fn throttle(&self) -> f64 {
				unsafe { prop_vehicle_seat_throttle(self.to_ptr()) }
			}
			pub fn set_throttle(&self, value: f64) {
				unsafe { prop_set_vehicle_seat_throttle(self.to_ptr(), value) }
			}
			pub fn throttle_float(&self) -> f64 {
				unsafe { prop_vehicle_seat_throttle_float(self.to_ptr()) }
			}
			pub fn set_throttle_float(&self, value: f64) {
				unsafe { prop_set_vehicle_seat_throttle_float(self.to_ptr(), value) }
			}
			pub fn torque(&self) -> f64 {
				unsafe { prop_vehicle_seat_torque(self.to_ptr()) }
			}
			pub fn set_torque(&self, value: f64) {
				unsafe { prop_set_vehicle_seat_torque(self.to_ptr(), value) }
			}
			pub fn turn_speed(&self) -> f64 {
				unsafe { prop_vehicle_seat_turn_speed(self.to_ptr()) }
			}
			pub fn set_turn_speed(&self, value: f64) {
				unsafe { prop_set_vehicle_seat_turn_speed(self.to_ptr(), value) }
			}
			pub fn fn_sit(&self, humanoid: Option<Instance>) {
				unsafe { dyn_vehicle_seat_sit(self.to_ptr(), humanoid) }
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
				unsafe { prop_model_primary_part(self.to_ptr()) }
			}
			pub fn set_primary_part(&self, value: Option<BasePart>) {
				unsafe { prop_set_model_primary_part(self.to_ptr(), value) }
			}
			pub fn world_pivot(&self) -> CFrame {
				unsafe { prop_model_world_pivot(self.to_ptr()) }
			}
			pub fn set_world_pivot(&self, value: CFrame) {
				unsafe { prop_set_model_world_pivot(self.to_ptr(), value) }
			}
			pub fn fn_break_joints(&self) {
				unsafe { dyn_model_break_joints(self.to_ptr()) }
			}
			pub fn fn_get_bounding_box(&self) {
				unsafe { dyn_model_get_bounding_box(self.to_ptr()) }
			}
			pub fn fn_get_extents_size(&self) -> Vector3 {
				unsafe { dyn_model_get_extents_size(self.to_ptr()) }
			}
			pub fn fn_get_model_c_frame(&self) -> CFrame {
				unsafe { dyn_model_get_model_c_frame(self.to_ptr()) }
			}
			pub fn fn_get_model_size(&self) -> Vector3 {
				unsafe { dyn_model_get_model_size(self.to_ptr()) }
			}
			pub fn fn_get_primary_part_c_frame(&self) -> CFrame {
				unsafe { dyn_model_get_primary_part_c_frame(self.to_ptr()) }
			}
			pub fn fn_make_joints(&self) {
				unsafe { dyn_model_make_joints(self.to_ptr()) }
			}
			pub fn fn_move_to(&self, position: Vector3) {
				unsafe { dyn_model_move_to(self.to_ptr(), position) }
			}
			pub fn fn_reset_orientation_to_identity(&self) {
				unsafe { dyn_model_reset_orientation_to_identity(self.to_ptr()) }
			}
			pub fn fn_set_identity_orientation(&self) {
				unsafe { dyn_model_set_identity_orientation(self.to_ptr()) }
			}
			pub fn fn_set_primary_part_c_frame(&self, cframe: CFrame) {
				unsafe { dyn_model_set_primary_part_c_frame(self.to_ptr(), cframe) }
			}
			pub fn fn_translate_by(&self, delta: Vector3) {
				unsafe { dyn_model_translate_by(self.to_ptr(), delta) }
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
				unsafe { dyn_world_root_are_parts_touching_others(self.to_ptr(), part_list, overlap_ignored) }
			}
			pub fn fn_find_part_on_ray(&self, ray: Ray, ignore_descendants_instance: Option<Instance>, terrain_cells_are_cubes: bool, ignore_water: bool) {
				unsafe { dyn_world_root_find_part_on_ray(self.to_ptr(), ray, ignore_descendants_instance, terrain_cells_are_cubes, ignore_water) }
			}
			pub fn fn_find_part_on_ray_with_ignore_list(&self, ray: Ray, ignore_descendants_table: Objects, terrain_cells_are_cubes: bool, ignore_water: bool) {
				unsafe { dyn_world_root_find_part_on_ray_with_ignore_list(self.to_ptr(), ray, ignore_descendants_table, terrain_cells_are_cubes, ignore_water) }
			}
			pub fn fn_find_part_on_ray_with_whitelist(&self, ray: Ray, whitelist_descendants_table: Objects, ignore_water: bool) {
				unsafe { dyn_world_root_find_part_on_ray_with_whitelist(self.to_ptr(), ray, whitelist_descendants_table, ignore_water) }
			}
			pub fn fn_find_parts_in_region_3(&self, region: Region3, ignore_descendants_instance: Option<Instance>, max_parts: f64) -> Objects {
				unsafe { dyn_world_root_find_parts_in_region_3(self.to_ptr(), region, ignore_descendants_instance, max_parts) }
			}
			pub fn fn_find_parts_in_region_3_with_ignore_list(&self, region: Region3, ignore_descendants_table: Objects, max_parts: f64) -> Objects {
				unsafe { dyn_world_root_find_parts_in_region_3_with_ignore_list(self.to_ptr(), region, ignore_descendants_table, max_parts) }
			}
			pub fn fn_find_parts_in_region_3_with_white_list(&self, region: Region3, whitelist_descendants_table: Objects, max_parts: f64) -> Objects {
				unsafe { dyn_world_root_find_parts_in_region_3_with_white_list(self.to_ptr(), region, whitelist_descendants_table, max_parts) }
			}
			pub fn fn_get_part_bounds_in_box(&self, cframe: CFrame, size: Vector3, overlap_params: OverlapParams) -> Objects {
				unsafe { dyn_world_root_get_part_bounds_in_box(self.to_ptr(), cframe, size, overlap_params) }
			}
			pub fn fn_get_part_bounds_in_radius(&self, position: Vector3, radius: f64, overlap_params: OverlapParams) -> Objects {
				unsafe { dyn_world_root_get_part_bounds_in_radius(self.to_ptr(), position, radius, overlap_params) }
			}
			pub fn fn_get_parts_in_part(&self, part: Option<BasePart>, overlap_params: OverlapParams) -> Objects {
				unsafe { dyn_world_root_get_parts_in_part(self.to_ptr(), part, overlap_params) }
			}
			pub fn fn_is_region_3_empty(&self, region: Region3, ignore_descendents_instance: Option<Instance>) -> bool {
				unsafe { dyn_world_root_is_region_3_empty(self.to_ptr(), region, ignore_descendents_instance) }
			}
			pub fn fn_is_region_3_empty_with_ignore_list(&self, region: Region3, ignore_descendents_table: Objects) -> bool {
				unsafe { dyn_world_root_is_region_3_empty_with_ignore_list(self.to_ptr(), region, ignore_descendents_table) }
			}
			pub fn fn_raycast(&self, origin: Vector3, direction: Vector3, raycast_params: RaycastParams) -> RaycastResult {
				unsafe { dyn_world_root_raycast(self.to_ptr(), origin, direction, raycast_params) }
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
				unsafe { prop_workspace_allow_third_party_sales(self.to_ptr()) }
			}
			pub fn set_allow_third_party_sales(&self, value: bool) {
				unsafe { prop_set_workspace_allow_third_party_sales(self.to_ptr(), value) }
			}
			pub fn current_camera(&self) -> Option<Camera> {
				unsafe { prop_workspace_current_camera(self.to_ptr()) }
			}
			pub fn set_current_camera(&self, value: Option<Camera>) {
				unsafe { prop_set_workspace_current_camera(self.to_ptr(), value) }
			}
			pub fn distributed_game_time(&self) -> f64 {
				unsafe { prop_workspace_distributed_game_time(self.to_ptr()) }
			}
			pub fn set_distributed_game_time(&self, value: f64) {
				unsafe { prop_set_workspace_distributed_game_time(self.to_ptr(), value) }
			}
			pub fn fallen_parts_destroy_height(&self) -> f64 {
				unsafe { prop_workspace_fallen_parts_destroy_height(self.to_ptr()) }
			}
			pub fn filtering_enabled(&self) -> bool {
				unsafe { prop_workspace_filtering_enabled(self.to_ptr()) }
			}
			pub fn global_wind(&self) -> Vector3 {
				unsafe { prop_workspace_global_wind(self.to_ptr()) }
			}
			pub fn set_global_wind(&self, value: Vector3) {
				unsafe { prop_set_workspace_global_wind(self.to_ptr(), value) }
			}
			pub fn gravity(&self) -> f64 {
				unsafe { prop_workspace_gravity(self.to_ptr()) }
			}
			pub fn set_gravity(&self, value: f64) {
				unsafe { prop_set_workspace_gravity(self.to_ptr(), value) }
			}
			pub fn streaming_enabled(&self) -> bool {
				unsafe { prop_workspace_streaming_enabled(self.to_ptr()) }
			}
			pub fn streaming_min_radius(&self) -> f64 {
				unsafe { prop_workspace_streaming_min_radius(self.to_ptr()) }
			}
			pub fn set_streaming_min_radius(&self, value: f64) {
				unsafe { prop_set_workspace_streaming_min_radius(self.to_ptr(), value) }
			}
			pub fn streaming_target_radius(&self) -> f64 {
				unsafe { prop_workspace_streaming_target_radius(self.to_ptr()) }
			}
			pub fn set_streaming_target_radius(&self, value: f64) {
				unsafe { prop_set_workspace_streaming_target_radius(self.to_ptr(), value) }
			}
			pub fn terrain(&self) -> Option<Terrain> {
				unsafe { prop_workspace_terrain(self.to_ptr()) }
			}
			pub fn touches_use_collision_groups(&self) -> bool {
				unsafe { prop_workspace_touches_use_collision_groups(self.to_ptr()) }
			}
			pub fn set_touches_use_collision_groups(&self, value: bool) {
				unsafe { prop_set_workspace_touches_use_collision_groups(self.to_ptr(), value) }
			}
			pub fn fn_get_num_awake_parts(&self) -> f64 {
				unsafe { dyn_workspace_get_num_awake_parts(self.to_ptr()) }
			}
			pub fn fn_get_physics_throttling(&self) -> f64 {
				unsafe { dyn_workspace_get_physics_throttling(self.to_ptr()) }
			}
			pub fn fn_get_real_physics_fps(&self) -> f64 {
				unsafe { dyn_workspace_get_real_physics_fps(self.to_ptr()) }
			}
			pub fn fn_get_server_time_now(&self) -> f64 {
				unsafe { dyn_workspace_get_server_time_now(self.to_ptr()) }
			}
			pub fn fn_pgs_is_enabled(&self) -> bool {
				unsafe { dyn_workspace_pgs_is_enabled(self.to_ptr()) }
			}
			pub fn fn_unjoin_from_outsiders(&self, objects: Objects) {
				unsafe { dyn_workspace_unjoin_from_outsiders(self.to_ptr(), objects) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn auto_update(&self) -> bool {
				unsafe { prop_package_link_auto_update(self.to_ptr()) }
			}
			pub fn set_auto_update(&self, value: bool) {
				unsafe { prop_set_package_link_auto_update(self.to_ptr(), value) }
			}
			pub fn creator(&self) -> String {
				unsafe { prop_package_link_creator(self.to_ptr()) }
			}
			pub fn package_asset_name(&self) -> String {
				unsafe { prop_package_link_package_asset_name(self.to_ptr()) }
			}
			pub fn package_id(&self) -> Content {
				unsafe { prop_package_link_package_id(self.to_ptr()) }
			}
			pub fn version_number(&self) -> f64 {
				unsafe { prop_package_link_version_number(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pages {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn is_finished(&self) -> bool {
				unsafe { prop_pages_is_finished(self.to_ptr()) }
			}
			pub fn fn_get_current_page(&self) {
				unsafe { dyn_pages_get_current_page(self.to_ptr()) }
			}
			pub fn fn_advance_to_next_page_async(&self) {
				unsafe { dyn_pages_advance_to_next_page_async(self.to_ptr()) }
			}
		}
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn acceleration(&self) -> Vector3 {
				unsafe { prop_particle_emitter_acceleration(self.to_ptr()) }
			}
			pub fn set_acceleration(&self, value: Vector3) {
				unsafe { prop_set_particle_emitter_acceleration(self.to_ptr(), value) }
			}
			pub fn brightness(&self) -> f64 {
				unsafe { prop_particle_emitter_brightness(self.to_ptr()) }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { prop_set_particle_emitter_brightness(self.to_ptr(), value) }
			}
			pub fn color(&self) -> ColorSequence {
				unsafe { prop_particle_emitter_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: ColorSequence) {
				unsafe { prop_set_particle_emitter_color(self.to_ptr(), value) }
			}
			pub fn drag(&self) -> f64 {
				unsafe { prop_particle_emitter_drag(self.to_ptr()) }
			}
			pub fn set_drag(&self, value: f64) {
				unsafe { prop_set_particle_emitter_drag(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_particle_emitter_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_particle_emitter_enabled(self.to_ptr(), value) }
			}
			pub fn flipbook_framerate(&self) -> NumberRange {
				unsafe { prop_particle_emitter_flipbook_framerate(self.to_ptr()) }
			}
			pub fn set_flipbook_framerate(&self, value: NumberRange) {
				unsafe { prop_set_particle_emitter_flipbook_framerate(self.to_ptr(), value) }
			}
			pub fn flipbook_incompatible(&self) -> String {
				unsafe { prop_particle_emitter_flipbook_incompatible(self.to_ptr()) }
			}
			pub fn set_flipbook_incompatible(&self, value: &str) {
				unsafe { prop_set_particle_emitter_flipbook_incompatible(self.to_ptr(), value) }
			}
			pub fn flipbook_start_random(&self) -> bool {
				unsafe { prop_particle_emitter_flipbook_start_random(self.to_ptr()) }
			}
			pub fn set_flipbook_start_random(&self, value: bool) {
				unsafe { prop_set_particle_emitter_flipbook_start_random(self.to_ptr(), value) }
			}
			pub fn lifetime(&self) -> NumberRange {
				unsafe { prop_particle_emitter_lifetime(self.to_ptr()) }
			}
			pub fn set_lifetime(&self, value: NumberRange) {
				unsafe { prop_set_particle_emitter_lifetime(self.to_ptr(), value) }
			}
			pub fn light_emission(&self) -> f64 {
				unsafe { prop_particle_emitter_light_emission(self.to_ptr()) }
			}
			pub fn set_light_emission(&self, value: f64) {
				unsafe { prop_set_particle_emitter_light_emission(self.to_ptr(), value) }
			}
			pub fn light_influence(&self) -> f64 {
				unsafe { prop_particle_emitter_light_influence(self.to_ptr()) }
			}
			pub fn set_light_influence(&self, value: f64) {
				unsafe { prop_set_particle_emitter_light_influence(self.to_ptr(), value) }
			}
			pub fn locked_to_part(&self) -> bool {
				unsafe { prop_particle_emitter_locked_to_part(self.to_ptr()) }
			}
			pub fn set_locked_to_part(&self, value: bool) {
				unsafe { prop_set_particle_emitter_locked_to_part(self.to_ptr(), value) }
			}
			pub fn rate(&self) -> f64 {
				unsafe { prop_particle_emitter_rate(self.to_ptr()) }
			}
			pub fn set_rate(&self, value: f64) {
				unsafe { prop_set_particle_emitter_rate(self.to_ptr(), value) }
			}
			pub fn rot_speed(&self) -> NumberRange {
				unsafe { prop_particle_emitter_rot_speed(self.to_ptr()) }
			}
			pub fn set_rot_speed(&self, value: NumberRange) {
				unsafe { prop_set_particle_emitter_rot_speed(self.to_ptr(), value) }
			}
			pub fn rotation(&self) -> NumberRange {
				unsafe { prop_particle_emitter_rotation(self.to_ptr()) }
			}
			pub fn set_rotation(&self, value: NumberRange) {
				unsafe { prop_set_particle_emitter_rotation(self.to_ptr(), value) }
			}
			pub fn shape_partial(&self) -> f64 {
				unsafe { prop_particle_emitter_shape_partial(self.to_ptr()) }
			}
			pub fn set_shape_partial(&self, value: f64) {
				unsafe { prop_set_particle_emitter_shape_partial(self.to_ptr(), value) }
			}
			pub fn size(&self) -> NumberSequence {
				unsafe { prop_particle_emitter_size(self.to_ptr()) }
			}
			pub fn set_size(&self, value: NumberSequence) {
				unsafe { prop_set_particle_emitter_size(self.to_ptr(), value) }
			}
			pub fn speed(&self) -> NumberRange {
				unsafe { prop_particle_emitter_speed(self.to_ptr()) }
			}
			pub fn set_speed(&self, value: NumberRange) {
				unsafe { prop_set_particle_emitter_speed(self.to_ptr(), value) }
			}
			pub fn spread_angle(&self) -> Vector2 {
				unsafe { prop_particle_emitter_spread_angle(self.to_ptr()) }
			}
			pub fn set_spread_angle(&self, value: Vector2) {
				unsafe { prop_set_particle_emitter_spread_angle(self.to_ptr(), value) }
			}
			pub fn squash(&self) -> NumberSequence {
				unsafe { prop_particle_emitter_squash(self.to_ptr()) }
			}
			pub fn set_squash(&self, value: NumberSequence) {
				unsafe { prop_set_particle_emitter_squash(self.to_ptr(), value) }
			}
			pub fn texture(&self) -> Content {
				unsafe { prop_particle_emitter_texture(self.to_ptr()) }
			}
			pub fn set_texture(&self, value: Content) {
				unsafe { prop_set_particle_emitter_texture(self.to_ptr(), value) }
			}
			pub fn time_scale(&self) -> f64 {
				unsafe { prop_particle_emitter_time_scale(self.to_ptr()) }
			}
			pub fn set_time_scale(&self, value: f64) {
				unsafe { prop_set_particle_emitter_time_scale(self.to_ptr(), value) }
			}
			pub fn transparency(&self) -> NumberSequence {
				unsafe { prop_particle_emitter_transparency(self.to_ptr()) }
			}
			pub fn set_transparency(&self, value: NumberSequence) {
				unsafe { prop_set_particle_emitter_transparency(self.to_ptr(), value) }
			}
			pub fn velocity_inheritance(&self) -> f64 {
				unsafe { prop_particle_emitter_velocity_inheritance(self.to_ptr()) }
			}
			pub fn set_velocity_inheritance(&self, value: f64) {
				unsafe { prop_set_particle_emitter_velocity_inheritance(self.to_ptr(), value) }
			}
			pub fn velocity_spread(&self) -> f64 {
				unsafe { prop_particle_emitter_velocity_spread(self.to_ptr()) }
			}
			pub fn set_velocity_spread(&self, value: f64) {
				unsafe { prop_set_particle_emitter_velocity_spread(self.to_ptr(), value) }
			}
			pub fn z_offset(&self) -> f64 {
				unsafe { prop_particle_emitter_z_offset(self.to_ptr()) }
			}
			pub fn set_z_offset(&self, value: f64) {
				unsafe { prop_set_particle_emitter_z_offset(self.to_ptr(), value) }
			}
			pub fn fn_clear(&self) {
				unsafe { dyn_particle_emitter_clear(self.to_ptr()) }
			}
			pub fn fn_emit(&self, particle_count: f64) {
				unsafe { dyn_particle_emitter_emit(self.to_ptr(), particle_count) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_path {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_point_coordinates(&self) {
				unsafe { dyn_path_get_point_coordinates(self.to_ptr()) }
			}
			pub fn fn_get_waypoints(&self) {
				unsafe { dyn_path_get_waypoints(self.to_ptr()) }
			}
			pub fn fn_check_occlusion_async(&self, start: f64) -> f64 {
				unsafe { dyn_path_check_occlusion_async(self.to_ptr(), start) }
			}
			pub fn fn_compute_async(&self, start: Vector3, finish: Vector3) {
				unsafe { dyn_path_compute_async(self.to_ptr(), start, finish) }
			}
			pub fn on_blocked<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_path_blocked(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_unblocked<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_path_unblocked(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pathfinding_link {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn attachment_0(&self) -> Option<Attachment> {
				unsafe { prop_pathfinding_link_attachment_0(self.to_ptr()) }
			}
			pub fn set_attachment_0(&self, value: Option<Attachment>) {
				unsafe { prop_set_pathfinding_link_attachment_0(self.to_ptr(), value) }
			}
			pub fn attachment_1(&self) -> Option<Attachment> {
				unsafe { prop_pathfinding_link_attachment_1(self.to_ptr()) }
			}
			pub fn set_attachment_1(&self, value: Option<Attachment>) {
				unsafe { prop_set_pathfinding_link_attachment_1(self.to_ptr(), value) }
			}
			pub fn is_bidirectional(&self) -> bool {
				unsafe { prop_pathfinding_link_is_bidirectional(self.to_ptr()) }
			}
			pub fn set_is_bidirectional(&self, value: bool) {
				unsafe { prop_set_pathfinding_link_is_bidirectional(self.to_ptr(), value) }
			}
			pub fn label(&self) -> String {
				unsafe { prop_pathfinding_link_label(self.to_ptr()) }
			}
			pub fn set_label(&self, value: &str) {
				unsafe { prop_set_pathfinding_link_label(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pathfinding_modifier {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn label(&self) -> String {
				unsafe { prop_pathfinding_modifier_label(self.to_ptr()) }
			}
			pub fn set_label(&self, value: &str) {
				unsafe { prop_set_pathfinding_modifier_label(self.to_ptr(), value) }
			}
			pub fn pass_through(&self) -> bool {
				unsafe { prop_pathfinding_modifier_pass_through(self.to_ptr()) }
			}
			pub fn set_pass_through(&self, value: bool) {
				unsafe { prop_set_pathfinding_modifier_pass_through(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pathfinding_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn empty_cutoff(&self) -> f64 {
				unsafe { prop_pathfinding_service_empty_cutoff(self.to_ptr()) }
			}
			pub fn set_empty_cutoff(&self, value: f64) {
				unsafe { prop_set_pathfinding_service_empty_cutoff(self.to_ptr(), value) }
			}
			pub fn fn_compute_raw_path_async(&self, start: Vector3, finish: Vector3, max_distance: f64) -> Option<Instance> {
				unsafe { dyn_pathfinding_service_compute_raw_path_async(self.to_ptr(), start, finish, max_distance) }
			}
			pub fn fn_compute_smooth_path_async(&self, start: Vector3, finish: Vector3, max_distance: f64) -> Option<Instance> {
				unsafe { dyn_pathfinding_service_compute_smooth_path_async(self.to_ptr(), start, finish, max_distance) }
			}
			pub fn fn_find_path_async(&self, start: Vector3, finish: Vector3) -> Option<Instance> {
				unsafe { dyn_pathfinding_service_find_path_async(self.to_ptr(), start, finish) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_physics_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_collision_group_contains_part(&self, name: &str, part: Option<BasePart>) -> bool {
				unsafe { dyn_physics_service_collision_group_contains_part(self.to_ptr(), name, part) }
			}
			pub fn fn_collision_group_set_collidable(&self, name_1: &str, name_2: &str, collidable: bool) {
				unsafe { dyn_physics_service_collision_group_set_collidable(self.to_ptr(), name_1, name_2, collidable) }
			}
			pub fn fn_collision_groups_are_collidable(&self, name_1: &str, name_2: &str) -> bool {
				unsafe { dyn_physics_service_collision_groups_are_collidable(self.to_ptr(), name_1, name_2) }
			}
			pub fn fn_create_collision_group(&self, name: &str) -> f64 {
				unsafe { dyn_physics_service_create_collision_group(self.to_ptr(), name) }
			}
			pub fn fn_get_collision_group_id(&self, name: &str) -> f64 {
				unsafe { dyn_physics_service_get_collision_group_id(self.to_ptr(), name) }
			}
			pub fn fn_get_collision_group_name(&self, name: f64) -> String {
				unsafe { dyn_physics_service_get_collision_group_name(self.to_ptr(), name) }
			}
			pub fn fn_get_collision_groups(&self) {
				unsafe { dyn_physics_service_get_collision_groups(self.to_ptr()) }
			}
			pub fn fn_get_max_collision_groups(&self) -> f64 {
				unsafe { dyn_physics_service_get_max_collision_groups(self.to_ptr()) }
			}
			pub fn fn_remove_collision_group(&self, name: &str) {
				unsafe { dyn_physics_service_remove_collision_group(self.to_ptr(), name) }
			}
			pub fn fn_rename_collision_group(&self, from: &str, to: &str) {
				unsafe { dyn_physics_service_rename_collision_group(self.to_ptr(), from, to) }
			}
			pub fn fn_set_part_collision_group(&self, part: Option<BasePart>, name: &str) {
				unsafe { dyn_physics_service_set_part_collision_group(self.to_ptr(), part, name) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_player {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn account_age(&self) -> f64 {
				unsafe { prop_player_account_age(self.to_ptr()) }
			}
			pub fn auto_jump_enabled(&self) -> bool {
				unsafe { prop_player_auto_jump_enabled(self.to_ptr()) }
			}
			pub fn set_auto_jump_enabled(&self, value: bool) {
				unsafe { prop_set_player_auto_jump_enabled(self.to_ptr(), value) }
			}
			pub fn camera_max_zoom_distance(&self) -> f64 {
				unsafe { prop_player_camera_max_zoom_distance(self.to_ptr()) }
			}
			pub fn set_camera_max_zoom_distance(&self, value: f64) {
				unsafe { prop_set_player_camera_max_zoom_distance(self.to_ptr(), value) }
			}
			pub fn camera_min_zoom_distance(&self) -> f64 {
				unsafe { prop_player_camera_min_zoom_distance(self.to_ptr()) }
			}
			pub fn set_camera_min_zoom_distance(&self, value: f64) {
				unsafe { prop_set_player_camera_min_zoom_distance(self.to_ptr(), value) }
			}
			pub fn can_load_character_appearance(&self) -> bool {
				unsafe { prop_player_can_load_character_appearance(self.to_ptr()) }
			}
			pub fn set_can_load_character_appearance(&self, value: bool) {
				unsafe { prop_set_player_can_load_character_appearance(self.to_ptr(), value) }
			}
			pub fn character(&self) -> Option<Model> {
				unsafe { prop_player_character(self.to_ptr()) }
			}
			pub fn set_character(&self, value: Option<Model>) {
				unsafe { prop_set_player_character(self.to_ptr(), value) }
			}
			pub fn character_appearance(&self) -> String {
				unsafe { prop_player_character_appearance(self.to_ptr()) }
			}
			pub fn set_character_appearance(&self, value: &str) {
				unsafe { prop_set_player_character_appearance(self.to_ptr(), value) }
			}
			pub fn character_appearance_id(&self) -> f64 {
				unsafe { prop_player_character_appearance_id(self.to_ptr()) }
			}
			pub fn set_character_appearance_id(&self, value: f64) {
				unsafe { prop_set_player_character_appearance_id(self.to_ptr(), value) }
			}
			pub fn data_complexity(&self) -> f64 {
				unsafe { prop_player_data_complexity(self.to_ptr()) }
			}
			pub fn data_ready(&self) -> bool {
				unsafe { prop_player_data_ready(self.to_ptr()) }
			}
			pub fn dev_enable_mouse_lock(&self) -> bool {
				unsafe { prop_player_dev_enable_mouse_lock(self.to_ptr()) }
			}
			pub fn set_dev_enable_mouse_lock(&self, value: bool) {
				unsafe { prop_set_player_dev_enable_mouse_lock(self.to_ptr(), value) }
			}
			pub fn display_name(&self) -> String {
				unsafe { prop_player_display_name(self.to_ptr()) }
			}
			pub fn set_display_name(&self, value: &str) {
				unsafe { prop_set_player_display_name(self.to_ptr(), value) }
			}
			pub fn follow_user_id(&self) -> f64 {
				unsafe { prop_player_follow_user_id(self.to_ptr()) }
			}
			pub fn gameplay_paused(&self) -> bool {
				unsafe { prop_player_gameplay_paused(self.to_ptr()) }
			}
			pub fn health_display_distance(&self) -> f64 {
				unsafe { prop_player_health_display_distance(self.to_ptr()) }
			}
			pub fn set_health_display_distance(&self, value: f64) {
				unsafe { prop_set_player_health_display_distance(self.to_ptr(), value) }
			}
			pub fn locale_id(&self) -> String {
				unsafe { prop_player_locale_id(self.to_ptr()) }
			}
			pub fn name_display_distance(&self) -> f64 {
				unsafe { prop_player_name_display_distance(self.to_ptr()) }
			}
			pub fn set_name_display_distance(&self, value: f64) {
				unsafe { prop_set_player_name_display_distance(self.to_ptr(), value) }
			}
			pub fn neutral(&self) -> bool {
				unsafe { prop_player_neutral(self.to_ptr()) }
			}
			pub fn set_neutral(&self, value: bool) {
				unsafe { prop_set_player_neutral(self.to_ptr(), value) }
			}
			pub fn replication_focus(&self) -> Option<Instance> {
				unsafe { prop_player_replication_focus(self.to_ptr()) }
			}
			pub fn set_replication_focus(&self, value: Option<Instance>) {
				unsafe { prop_set_player_replication_focus(self.to_ptr(), value) }
			}
			pub fn respawn_location(&self) -> Option<SpawnLocation> {
				unsafe { prop_player_respawn_location(self.to_ptr()) }
			}
			pub fn set_respawn_location(&self, value: Option<SpawnLocation>) {
				unsafe { prop_set_player_respawn_location(self.to_ptr(), value) }
			}
			pub fn team(&self) -> Option<Team> {
				unsafe { prop_player_team(self.to_ptr()) }
			}
			pub fn set_team(&self, value: Option<Team>) {
				unsafe { prop_set_player_team(self.to_ptr(), value) }
			}
			pub fn team_color(&self) -> BrickColor {
				unsafe { prop_player_team_color(self.to_ptr()) }
			}
			pub fn set_team_color(&self, value: BrickColor) {
				unsafe { prop_set_player_team_color(self.to_ptr(), value) }
			}
			pub fn user_id(&self) -> f64 {
				unsafe { prop_player_user_id(self.to_ptr()) }
			}
			pub fn set_user_id(&self, value: f64) {
				unsafe { prop_set_player_user_id(self.to_ptr(), value) }
			}
			pub fn fn_clear_character_appearance(&self) {
				unsafe { dyn_player_clear_character_appearance(self.to_ptr()) }
			}
			pub fn fn_distance_from_character(&self, point: Vector3) -> f64 {
				unsafe { dyn_player_distance_from_character(self.to_ptr(), point) }
			}
			pub fn fn_get_join_data(&self) {
				unsafe { dyn_player_get_join_data(self.to_ptr()) }
			}
			pub fn fn_get_mouse(&self) -> Option<Mouse> {
				unsafe { dyn_player_get_mouse(self.to_ptr()) }
			}
			pub fn fn_get_network_ping(&self) -> f64 {
				unsafe { dyn_player_get_network_ping(self.to_ptr()) }
			}
			pub fn fn_has_appearance_loaded(&self) -> bool {
				unsafe { dyn_player_has_appearance_loaded(self.to_ptr()) }
			}
			pub fn fn_is_user_available_for_experiment(&self) -> bool {
				unsafe { dyn_player_is_user_available_for_experiment(self.to_ptr()) }
			}
			pub fn fn_kick(&self, message: &str) {
				unsafe { dyn_player_kick(self.to_ptr(), message) }
			}
			pub fn fn_load_boolean(&self, key: &str) -> bool {
				unsafe { dyn_player_load_boolean(self.to_ptr(), key) }
			}
			pub fn fn_load_character_appearance(&self, asset_instance: Option<Instance>) {
				unsafe { dyn_player_load_character_appearance(self.to_ptr(), asset_instance) }
			}
			pub fn fn_load_instance(&self, key: &str) -> Option<Instance> {
				unsafe { dyn_player_load_instance(self.to_ptr(), key) }
			}
			pub fn fn_load_number(&self, key: &str) -> f64 {
				unsafe { dyn_player_load_number(self.to_ptr(), key) }
			}
			pub fn fn_load_string(&self, key: &str) -> String {
				unsafe { dyn_player_load_string(self.to_ptr(), key) }
			}
			pub fn fn_move(&self, walk_direction: Vector3, relative_to_camera: bool) {
				unsafe { dyn_player_move(self.to_ptr(), walk_direction, relative_to_camera) }
			}
			pub fn fn_save_boolean(&self, key: &str, value: bool) {
				unsafe { dyn_player_save_boolean(self.to_ptr(), key, value) }
			}
			pub fn fn_save_instance(&self, key: &str, value: Option<Instance>) {
				unsafe { dyn_player_save_instance(self.to_ptr(), key, value) }
			}
			pub fn fn_save_number(&self, key: &str, value: f64) {
				unsafe { dyn_player_save_number(self.to_ptr(), key, value) }
			}
			pub fn fn_save_string(&self, key: &str, value: &str) {
				unsafe { dyn_player_save_string(self.to_ptr(), key, value) }
			}
			pub fn fn_get_friends_online(&self, max_friends: f64) {
				unsafe { dyn_player_get_friends_online(self.to_ptr(), max_friends) }
			}
			pub fn fn_get_rank_in_group(&self, group_id: f64) -> f64 {
				unsafe { dyn_player_get_rank_in_group(self.to_ptr(), group_id) }
			}
			pub fn fn_get_role_in_group(&self, group_id: f64) -> String {
				unsafe { dyn_player_get_role_in_group(self.to_ptr(), group_id) }
			}
			pub fn fn_is_best_friends_with(&self, user_id: f64) -> bool {
				unsafe { dyn_player_is_best_friends_with(self.to_ptr(), user_id) }
			}
			pub fn fn_is_friends_with(&self, user_id: f64) -> bool {
				unsafe { dyn_player_is_friends_with(self.to_ptr(), user_id) }
			}
			pub fn fn_is_in_group(&self, group_id: f64) -> bool {
				unsafe { dyn_player_is_in_group(self.to_ptr(), group_id) }
			}
			pub fn fn_load_character(&self) {
				unsafe { dyn_player_load_character(self.to_ptr()) }
			}
			pub fn fn_load_character_with_humanoid_description(&self, humanoid_description: Option<HumanoidDescription>) {
				unsafe { dyn_player_load_character_with_humanoid_description(self.to_ptr(), humanoid_description) }
			}
			pub fn fn_request_stream_around_async(&self, position: Vector3, time_out: f64) {
				unsafe { dyn_player_request_stream_around_async(self.to_ptr(), position, time_out) }
			}
			pub fn fn_wait_for_data_ready(&self) -> bool {
				unsafe { dyn_player_wait_for_data_ready(self.to_ptr()) }
			}
			pub fn on_character_added<F: 'static + Fn(Option<Model>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_player_character_added(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_character_appearance_loaded<F: 'static + Fn(Option<Model>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_player_character_appearance_loaded(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_character_removing<F: 'static + Fn(Option<Model>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_player_character_removing(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_chatted<F: 'static + Fn(String, Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_player_chatted(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_idled<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_player_idled(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_on_teleport<F: 'static + Fn((), f64, String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_player_on_teleport(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_player_scripts {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_clear_computer_camera_movement_modes(&self) {
				unsafe { dyn_player_scripts_clear_computer_camera_movement_modes(self.to_ptr()) }
			}
			pub fn fn_clear_computer_movement_modes(&self) {
				unsafe { dyn_player_scripts_clear_computer_movement_modes(self.to_ptr()) }
			}
			pub fn fn_clear_touch_camera_movement_modes(&self) {
				unsafe { dyn_player_scripts_clear_touch_camera_movement_modes(self.to_ptr()) }
			}
			pub fn fn_clear_touch_movement_modes(&self) {
				unsafe { dyn_player_scripts_clear_touch_movement_modes(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_players {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn bubble_chat(&self) -> bool {
				unsafe { prop_players_bubble_chat(self.to_ptr()) }
			}
			pub fn character_auto_loads(&self) -> bool {
				unsafe { prop_players_character_auto_loads(self.to_ptr()) }
			}
			pub fn set_character_auto_loads(&self, value: bool) {
				unsafe { prop_set_players_character_auto_loads(self.to_ptr(), value) }
			}
			pub fn classic_chat(&self) -> bool {
				unsafe { prop_players_classic_chat(self.to_ptr()) }
			}
			pub fn local_player(&self) -> Option<Player> {
				unsafe { prop_players_local_player(self.to_ptr()) }
			}
			pub fn max_players(&self) -> f64 {
				unsafe { prop_players_max_players(self.to_ptr()) }
			}
			pub fn num_players(&self) -> f64 {
				unsafe { prop_players_num_players(self.to_ptr()) }
			}
			pub fn preferred_players(&self) -> f64 {
				unsafe { prop_players_preferred_players(self.to_ptr()) }
			}
			pub fn respawn_time(&self) -> f64 {
				unsafe { prop_players_respawn_time(self.to_ptr()) }
			}
			pub fn set_respawn_time(&self, value: f64) {
				unsafe { prop_set_players_respawn_time(self.to_ptr(), value) }
			}
			pub fn fn_get_player_by_user_id(&self, user_id: f64) -> Option<Player> {
				unsafe { dyn_players_get_player_by_user_id(self.to_ptr(), user_id) }
			}
			pub fn fn_get_player_from_character(&self, character: Option<Model>) -> Option<Player> {
				unsafe { dyn_players_get_player_from_character(self.to_ptr(), character) }
			}
			pub fn fn_get_players(&self) -> Objects {
				unsafe { dyn_players_get_players(self.to_ptr()) }
			}
			pub fn fn_create_humanoid_model_from_user_id(&self, user_id: f64) -> Option<Model> {
				unsafe { dyn_players_create_humanoid_model_from_user_id(self.to_ptr(), user_id) }
			}
			pub fn fn_get_character_appearance_async(&self, user_id: f64) -> Option<Model> {
				unsafe { dyn_players_get_character_appearance_async(self.to_ptr(), user_id) }
			}
			pub fn fn_get_character_appearance_info_async(&self, user_id: f64) {
				unsafe { dyn_players_get_character_appearance_info_async(self.to_ptr(), user_id) }
			}
			pub fn fn_get_friends_async(&self, user_id: f64) -> Option<FriendPages> {
				unsafe { dyn_players_get_friends_async(self.to_ptr(), user_id) }
			}
			pub fn fn_get_humanoid_description_from_outfit_id(&self, outfit_id: f64) -> Option<HumanoidDescription> {
				unsafe { dyn_players_get_humanoid_description_from_outfit_id(self.to_ptr(), outfit_id) }
			}
			pub fn fn_get_humanoid_description_from_user_id(&self, user_id: f64) -> Option<HumanoidDescription> {
				unsafe { dyn_players_get_humanoid_description_from_user_id(self.to_ptr(), user_id) }
			}
			pub fn fn_get_name_from_user_id_async(&self, user_id: f64) -> String {
				unsafe { dyn_players_get_name_from_user_id_async(self.to_ptr(), user_id) }
			}
			pub fn fn_get_user_id_from_name_async(&self, user_name: &str) -> f64 {
				unsafe { dyn_players_get_user_id_from_name_async(self.to_ptr(), user_name) }
			}
			pub fn on_player_added<F: 'static + Fn(Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_players_player_added(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_player_membership_changed<F: 'static + Fn(Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_players_player_membership_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_player_removing<F: 'static + Fn(Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_players_player_removing(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_policy_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_policy_info_for_player_async(&self, player: Option<Instance>) {
				unsafe { dyn_policy_service_get_policy_info_for_player_async(self.to_ptr(), player) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_pose_base {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn weight(&self) -> f64 {
				unsafe { prop_pose_base_weight(self.to_ptr()) }
			}
			pub fn set_weight(&self, value: f64) {
				unsafe { prop_set_pose_base_weight(self.to_ptr(), value) }
			}
		}
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
				unsafe { prop_number_pose_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: f64) {
				unsafe { prop_set_number_pose_value(self.to_ptr(), value) }
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
				unsafe { prop_pose_c_frame(self.to_ptr()) }
			}
			pub fn set_c_frame(&self, value: CFrame) {
				unsafe { prop_set_pose_c_frame(self.to_ptr(), value) }
			}
			pub fn mask_weight(&self) -> f64 {
				unsafe { prop_pose_mask_weight(self.to_ptr()) }
			}
			pub fn set_mask_weight(&self, value: f64) {
				unsafe { prop_set_pose_mask_weight(self.to_ptr(), value) }
			}
			pub fn fn_add_sub_pose(&self, pose: Option<Instance>) {
				unsafe { dyn_pose_add_sub_pose(self.to_ptr(), pose) }
			}
			pub fn fn_get_sub_poses(&self) -> Objects {
				unsafe { dyn_pose_get_sub_poses(self.to_ptr()) }
			}
			pub fn fn_remove_sub_pose(&self, pose: Option<Instance>) {
				unsafe { dyn_pose_remove_sub_pose(self.to_ptr(), pose) }
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn enabled(&self) -> bool {
				unsafe { prop_post_effect_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_post_effect_enabled(self.to_ptr(), value) }
			}
		}
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
				unsafe { prop_bloom_effect_intensity(self.to_ptr()) }
			}
			pub fn set_intensity(&self, value: f64) {
				unsafe { prop_set_bloom_effect_intensity(self.to_ptr(), value) }
			}
			pub fn size(&self) -> f64 {
				unsafe { prop_bloom_effect_size(self.to_ptr()) }
			}
			pub fn set_size(&self, value: f64) {
				unsafe { prop_set_bloom_effect_size(self.to_ptr(), value) }
			}
			pub fn threshold(&self) -> f64 {
				unsafe { prop_bloom_effect_threshold(self.to_ptr()) }
			}
			pub fn set_threshold(&self, value: f64) {
				unsafe { prop_set_bloom_effect_threshold(self.to_ptr(), value) }
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
				unsafe { prop_blur_effect_size(self.to_ptr()) }
			}
			pub fn set_size(&self, value: f64) {
				unsafe { prop_set_blur_effect_size(self.to_ptr(), value) }
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
				unsafe { prop_color_correction_effect_brightness(self.to_ptr()) }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { prop_set_color_correction_effect_brightness(self.to_ptr(), value) }
			}
			pub fn contrast(&self) -> f64 {
				unsafe { prop_color_correction_effect_contrast(self.to_ptr()) }
			}
			pub fn set_contrast(&self, value: f64) {
				unsafe { prop_set_color_correction_effect_contrast(self.to_ptr(), value) }
			}
			pub fn saturation(&self) -> f64 {
				unsafe { prop_color_correction_effect_saturation(self.to_ptr()) }
			}
			pub fn set_saturation(&self, value: f64) {
				unsafe { prop_set_color_correction_effect_saturation(self.to_ptr(), value) }
			}
			pub fn tint_color(&self) -> Color3 {
				unsafe { prop_color_correction_effect_tint_color(self.to_ptr()) }
			}
			pub fn set_tint_color(&self, value: Color3) {
				unsafe { prop_set_color_correction_effect_tint_color(self.to_ptr(), value) }
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
				unsafe { prop_depth_of_field_effect_far_intensity(self.to_ptr()) }
			}
			pub fn set_far_intensity(&self, value: f64) {
				unsafe { prop_set_depth_of_field_effect_far_intensity(self.to_ptr(), value) }
			}
			pub fn focus_distance(&self) -> f64 {
				unsafe { prop_depth_of_field_effect_focus_distance(self.to_ptr()) }
			}
			pub fn set_focus_distance(&self, value: f64) {
				unsafe { prop_set_depth_of_field_effect_focus_distance(self.to_ptr(), value) }
			}
			pub fn in_focus_radius(&self) -> f64 {
				unsafe { prop_depth_of_field_effect_in_focus_radius(self.to_ptr()) }
			}
			pub fn set_in_focus_radius(&self, value: f64) {
				unsafe { prop_set_depth_of_field_effect_in_focus_radius(self.to_ptr(), value) }
			}
			pub fn near_intensity(&self) -> f64 {
				unsafe { prop_depth_of_field_effect_near_intensity(self.to_ptr()) }
			}
			pub fn set_near_intensity(&self, value: f64) {
				unsafe { prop_set_depth_of_field_effect_near_intensity(self.to_ptr(), value) }
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
				unsafe { prop_sun_rays_effect_intensity(self.to_ptr()) }
			}
			pub fn set_intensity(&self, value: f64) {
				unsafe { prop_set_sun_rays_effect_intensity(self.to_ptr(), value) }
			}
			pub fn spread(&self) -> f64 {
				unsafe { prop_sun_rays_effect_spread(self.to_ptr()) }
			}
			pub fn set_spread(&self, value: f64) {
				unsafe { prop_set_sun_rays_effect_spread(self.to_ptr(), value) }
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn action_text(&self) -> String {
				unsafe { prop_proximity_prompt_action_text(self.to_ptr()) }
			}
			pub fn set_action_text(&self, value: &str) {
				unsafe { prop_set_proximity_prompt_action_text(self.to_ptr(), value) }
			}
			pub fn auto_localize(&self) -> bool {
				unsafe { prop_proximity_prompt_auto_localize(self.to_ptr()) }
			}
			pub fn set_auto_localize(&self, value: bool) {
				unsafe { prop_set_proximity_prompt_auto_localize(self.to_ptr(), value) }
			}
			pub fn clickable_prompt(&self) -> bool {
				unsafe { prop_proximity_prompt_clickable_prompt(self.to_ptr()) }
			}
			pub fn set_clickable_prompt(&self, value: bool) {
				unsafe { prop_set_proximity_prompt_clickable_prompt(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_proximity_prompt_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_proximity_prompt_enabled(self.to_ptr(), value) }
			}
			pub fn hold_duration(&self) -> f64 {
				unsafe { prop_proximity_prompt_hold_duration(self.to_ptr()) }
			}
			pub fn set_hold_duration(&self, value: f64) {
				unsafe { prop_set_proximity_prompt_hold_duration(self.to_ptr(), value) }
			}
			pub fn max_activation_distance(&self) -> f64 {
				unsafe { prop_proximity_prompt_max_activation_distance(self.to_ptr()) }
			}
			pub fn set_max_activation_distance(&self, value: f64) {
				unsafe { prop_set_proximity_prompt_max_activation_distance(self.to_ptr(), value) }
			}
			pub fn object_text(&self) -> String {
				unsafe { prop_proximity_prompt_object_text(self.to_ptr()) }
			}
			pub fn set_object_text(&self, value: &str) {
				unsafe { prop_set_proximity_prompt_object_text(self.to_ptr(), value) }
			}
			pub fn requires_line_of_sight(&self) -> bool {
				unsafe { prop_proximity_prompt_requires_line_of_sight(self.to_ptr()) }
			}
			pub fn set_requires_line_of_sight(&self, value: bool) {
				unsafe { prop_set_proximity_prompt_requires_line_of_sight(self.to_ptr(), value) }
			}
			pub fn root_localization_table(&self) -> Option<LocalizationTable> {
				unsafe { prop_proximity_prompt_root_localization_table(self.to_ptr()) }
			}
			pub fn set_root_localization_table(&self, value: Option<LocalizationTable>) {
				unsafe { prop_set_proximity_prompt_root_localization_table(self.to_ptr(), value) }
			}
			pub fn ui_offset(&self) -> Vector2 {
				unsafe { prop_proximity_prompt_ui_offset(self.to_ptr()) }
			}
			pub fn set_ui_offset(&self, value: Vector2) {
				unsafe { prop_set_proximity_prompt_ui_offset(self.to_ptr(), value) }
			}
			pub fn fn_input_hold_begin(&self) {
				unsafe { dyn_proximity_prompt_input_hold_begin(self.to_ptr()) }
			}
			pub fn fn_input_hold_end(&self) {
				unsafe { dyn_proximity_prompt_input_hold_end(self.to_ptr()) }
			}
			pub fn on_prompt_button_hold_began<F: 'static + Fn(Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_proximity_prompt_prompt_button_hold_began(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_button_hold_ended<F: 'static + Fn(Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_proximity_prompt_prompt_button_hold_ended(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_hidden<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_proximity_prompt_prompt_hidden(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_shown<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_proximity_prompt_prompt_shown(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_trigger_ended<F: 'static + Fn(Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_proximity_prompt_trigger_ended(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_triggered<F: 'static + Fn(Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_proximity_prompt_triggered(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_proximity_prompt_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn enabled(&self) -> bool {
				unsafe { prop_proximity_prompt_service_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_proximity_prompt_service_enabled(self.to_ptr(), value) }
			}
			pub fn max_prompts_visible(&self) -> f64 {
				unsafe { prop_proximity_prompt_service_max_prompts_visible(self.to_ptr()) }
			}
			pub fn set_max_prompts_visible(&self, value: f64) {
				unsafe { prop_set_proximity_prompt_service_max_prompts_visible(self.to_ptr(), value) }
			}
			pub fn on_prompt_button_hold_began<F: 'static + Fn(Option<ProximityPrompt>, Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_proximity_prompt_service_prompt_button_hold_began(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_button_hold_ended<F: 'static + Fn(Option<ProximityPrompt>, Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_proximity_prompt_service_prompt_button_hold_ended(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_hidden<F: 'static + Fn(Option<ProximityPrompt>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_proximity_prompt_service_prompt_hidden(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_shown<F: 'static + Fn(Option<ProximityPrompt>, ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_proximity_prompt_service_prompt_shown(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_trigger_ended<F: 'static + Fn(Option<ProximityPrompt>, Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_proximity_prompt_service_prompt_trigger_ended(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_prompt_triggered<F: 'static + Fn(Option<ProximityPrompt>, Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_proximity_prompt_service_prompt_triggered(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_remote_event {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn on_on_client_event<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_remote_event_on_client_event(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_on_server_event<F: 'static + Fn(Option<Player>, ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_remote_event_on_server_event(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_remote_function {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_replicated_first {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_remove_default_loading_screen(&self) {
				unsafe { dyn_replicated_first_remove_default_loading_screen(self.to_ptr()) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_replicated_storage {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_rotation_curve {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn length(&self) -> f64 {
				unsafe { prop_rotation_curve_length(self.to_ptr()) }
			}
			pub fn fn_get_key_at_index(&self, index: f64) -> RotationCurveKey {
				unsafe { dyn_rotation_curve_get_key_at_index(self.to_ptr(), index) }
			}
			pub fn fn_get_key_indices_at_time(&self, time: f64) {
				unsafe { dyn_rotation_curve_get_key_indices_at_time(self.to_ptr(), time) }
			}
			pub fn fn_get_keys(&self) {
				unsafe { dyn_rotation_curve_get_keys(self.to_ptr()) }
			}
			pub fn fn_get_value_at_time(&self, time: f64) -> Option<CFrame> {
				unsafe { dyn_rotation_curve_get_value_at_time(self.to_ptr(), time) }
			}
			pub fn fn_insert_key(&self, key: RotationCurveKey) {
				unsafe { dyn_rotation_curve_insert_key(self.to_ptr(), key) }
			}
			pub fn fn_remove_key_at_index(&self, starting_index: f64, count: f64) -> f64 {
				unsafe { dyn_rotation_curve_remove_key_at_index(self.to_ptr(), starting_index, count) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_run_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_bind_to_render_step(&self, name: &str, priority: f64, function: Function) {
				unsafe { dyn_run_service_bind_to_render_step(self.to_ptr(), name, priority, function) }
			}
			pub fn fn_is_client(&self) -> bool {
				unsafe { dyn_run_service_is_client(self.to_ptr()) }
			}
			pub fn fn_is_run_mode(&self) -> bool {
				unsafe { dyn_run_service_is_run_mode(self.to_ptr()) }
			}
			pub fn fn_is_running(&self) -> bool {
				unsafe { dyn_run_service_is_running(self.to_ptr()) }
			}
			pub fn fn_is_server(&self) -> bool {
				unsafe { dyn_run_service_is_server(self.to_ptr()) }
			}
			pub fn fn_is_studio(&self) -> bool {
				unsafe { dyn_run_service_is_studio(self.to_ptr()) }
			}
			pub fn fn_unbind_from_render_step(&self, name: &str) {
				unsafe { dyn_run_service_unbind_from_render_step(self.to_ptr(), name) }
			}
			pub fn on_heartbeat<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_run_service_heartbeat(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_post_simulation<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_run_service_post_simulation(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_pre_animation<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_run_service_pre_animation(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_pre_render<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_run_service_pre_render(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_pre_simulation<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_run_service_pre_simulation(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_render_stepped<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_run_service_render_stepped(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_stepped<F: 'static + Fn(f64, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_run_service_stepped(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_screenshot_hud {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn camera_button_icon(&self) -> Content {
				unsafe { prop_screenshot_hud_camera_button_icon(self.to_ptr()) }
			}
			pub fn set_camera_button_icon(&self, value: Content) {
				unsafe { prop_set_screenshot_hud_camera_button_icon(self.to_ptr(), value) }
			}
			pub fn camera_button_position(&self) -> UDim2 {
				unsafe { prop_screenshot_hud_camera_button_position(self.to_ptr()) }
			}
			pub fn set_camera_button_position(&self, value: UDim2) {
				unsafe { prop_set_screenshot_hud_camera_button_position(self.to_ptr(), value) }
			}
			pub fn close_button_position(&self) -> UDim2 {
				unsafe { prop_screenshot_hud_close_button_position(self.to_ptr()) }
			}
			pub fn set_close_button_position(&self, value: UDim2) {
				unsafe { prop_set_screenshot_hud_close_button_position(self.to_ptr(), value) }
			}
			pub fn close_when_screenshot_taken(&self) -> bool {
				unsafe { prop_screenshot_hud_close_when_screenshot_taken(self.to_ptr()) }
			}
			pub fn set_close_when_screenshot_taken(&self, value: bool) {
				unsafe { prop_set_screenshot_hud_close_when_screenshot_taken(self.to_ptr(), value) }
			}
			pub fn experience_name_overlay_enabled(&self) -> bool {
				unsafe { prop_screenshot_hud_experience_name_overlay_enabled(self.to_ptr()) }
			}
			pub fn set_experience_name_overlay_enabled(&self, value: bool) {
				unsafe { prop_set_screenshot_hud_experience_name_overlay_enabled(self.to_ptr(), value) }
			}
			pub fn username_overlay_enabled(&self) -> bool {
				unsafe { prop_screenshot_hud_username_overlay_enabled(self.to_ptr()) }
			}
			pub fn set_username_overlay_enabled(&self, value: bool) {
				unsafe { prop_set_screenshot_hud_username_overlay_enabled(self.to_ptr(), value) }
			}
			pub fn visible(&self) -> bool {
				unsafe { prop_screenshot_hud_visible(self.to_ptr()) }
			}
			pub fn set_visible(&self, value: bool) {
				unsafe { prop_set_screenshot_hud_visible(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_script_context {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn on_error<F: 'static + Fn(String, String, Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_script_context_error(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_server_script_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn load_string_enabled(&self) -> bool {
				unsafe { prop_server_script_service_load_string_enabled(self.to_ptr()) }
			}
			pub fn set_load_string_enabled(&self, value: bool) {
				unsafe { prop_set_server_script_service_load_string_enabled(self.to_ptr(), value) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_server_storage {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_service_provider {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_find_service(&self, class_name: &str) -> Option<Instance> {
				unsafe { dyn_service_provider_find_service(self.to_ptr(), class_name) }
			}
			pub fn fn_get_service(&self, class_name: &str) -> Option<Instance> {
				unsafe { dyn_service_provider_get_service(self.to_ptr(), class_name) }
			}
			pub fn on_close<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_service_provider_close(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_service_added<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_service_provider_service_added(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_service_removing<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_service_provider_service_removing(self.to_ptr(), Box::new(callback))) }
			}
		}
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
				unsafe { prop_data_model_creator_id(self.to_ptr()) }
			}
			pub fn game_id(&self) -> f64 {
				unsafe { prop_data_model_game_id(self.to_ptr()) }
			}
			pub fn job_id(&self) -> String {
				unsafe { prop_data_model_job_id(self.to_ptr()) }
			}
			pub fn place_id(&self) -> f64 {
				unsafe { prop_data_model_place_id(self.to_ptr()) }
			}
			pub fn place_version(&self) -> f64 {
				unsafe { prop_data_model_place_version(self.to_ptr()) }
			}
			pub fn private_server_id(&self) -> String {
				unsafe { prop_data_model_private_server_id(self.to_ptr()) }
			}
			pub fn private_server_owner_id(&self) -> f64 {
				unsafe { prop_data_model_private_server_owner_id(self.to_ptr()) }
			}
			pub fn vip_server_id(&self) -> String {
				unsafe { prop_data_model_vip_server_id(self.to_ptr()) }
			}
			pub fn vip_server_owner_id(&self) -> f64 {
				unsafe { prop_data_model_vip_server_owner_id(self.to_ptr()) }
			}
			pub fn workspace(&self) -> Option<Workspace> {
				unsafe { prop_data_model_workspace(self.to_ptr()) }
			}
			pub fn fn_bind_to_close(&self, function: Function) {
				unsafe { dyn_data_model_bind_to_close(self.to_ptr(), function) }
			}
			pub fn fn_get_message(&self) -> String {
				unsafe { dyn_data_model_get_message(self.to_ptr()) }
			}
			pub fn fn_get_remote_build_mode(&self) -> bool {
				unsafe { dyn_data_model_get_remote_build_mode(self.to_ptr()) }
			}
			pub fn fn_is_loaded(&self) -> bool {
				unsafe { dyn_data_model_is_loaded(self.to_ptr()) }
			}
			pub fn on_allowed_gear_type_changed<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_data_model_allowed_gear_type_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_graphics_quality_change_request<F: 'static + Fn(bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_data_model_graphics_quality_change_request(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_item_changed<F: 'static + Fn(Option<Instance>, String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_data_model_item_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_loaded<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_data_model_loaded(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				unsafe { get_game() }
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
				unsafe { dyn_user_settings_is_user_feature_enabled(self.to_ptr(), name) }
			}
			pub fn fn_reset(&self) {
				unsafe { dyn_user_settings_reset(self.to_ptr()) }
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn celestial_bodies_shown(&self) -> bool {
				unsafe { prop_sky_celestial_bodies_shown(self.to_ptr()) }
			}
			pub fn set_celestial_bodies_shown(&self, value: bool) {
				unsafe { prop_set_sky_celestial_bodies_shown(self.to_ptr(), value) }
			}
			pub fn moon_angular_size(&self) -> f64 {
				unsafe { prop_sky_moon_angular_size(self.to_ptr()) }
			}
			pub fn set_moon_angular_size(&self, value: f64) {
				unsafe { prop_set_sky_moon_angular_size(self.to_ptr(), value) }
			}
			pub fn moon_texture_id(&self) -> Content {
				unsafe { prop_sky_moon_texture_id(self.to_ptr()) }
			}
			pub fn set_moon_texture_id(&self, value: Content) {
				unsafe { prop_set_sky_moon_texture_id(self.to_ptr(), value) }
			}
			pub fn skybox_bk(&self) -> Content {
				unsafe { prop_sky_skybox_bk(self.to_ptr()) }
			}
			pub fn set_skybox_bk(&self, value: Content) {
				unsafe { prop_set_sky_skybox_bk(self.to_ptr(), value) }
			}
			pub fn skybox_dn(&self) -> Content {
				unsafe { prop_sky_skybox_dn(self.to_ptr()) }
			}
			pub fn set_skybox_dn(&self, value: Content) {
				unsafe { prop_set_sky_skybox_dn(self.to_ptr(), value) }
			}
			pub fn skybox_ft(&self) -> Content {
				unsafe { prop_sky_skybox_ft(self.to_ptr()) }
			}
			pub fn set_skybox_ft(&self, value: Content) {
				unsafe { prop_set_sky_skybox_ft(self.to_ptr(), value) }
			}
			pub fn skybox_lf(&self) -> Content {
				unsafe { prop_sky_skybox_lf(self.to_ptr()) }
			}
			pub fn set_skybox_lf(&self, value: Content) {
				unsafe { prop_set_sky_skybox_lf(self.to_ptr(), value) }
			}
			pub fn skybox_rt(&self) -> Content {
				unsafe { prop_sky_skybox_rt(self.to_ptr()) }
			}
			pub fn set_skybox_rt(&self, value: Content) {
				unsafe { prop_set_sky_skybox_rt(self.to_ptr(), value) }
			}
			pub fn skybox_up(&self) -> Content {
				unsafe { prop_sky_skybox_up(self.to_ptr()) }
			}
			pub fn set_skybox_up(&self, value: Content) {
				unsafe { prop_set_sky_skybox_up(self.to_ptr(), value) }
			}
			pub fn star_count(&self) -> f64 {
				unsafe { prop_sky_star_count(self.to_ptr()) }
			}
			pub fn set_star_count(&self, value: f64) {
				unsafe { prop_set_sky_star_count(self.to_ptr(), value) }
			}
			pub fn sun_angular_size(&self) -> f64 {
				unsafe { prop_sky_sun_angular_size(self.to_ptr()) }
			}
			pub fn set_sun_angular_size(&self, value: f64) {
				unsafe { prop_set_sky_sun_angular_size(self.to_ptr(), value) }
			}
			pub fn sun_texture_id(&self) -> Content {
				unsafe { prop_sky_sun_texture_id(self.to_ptr()) }
			}
			pub fn set_sun_texture_id(&self, value: Content) {
				unsafe { prop_set_sky_sun_texture_id(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_smoke {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn color(&self) -> Color3 {
				unsafe { prop_smoke_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { prop_set_smoke_color(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_smoke_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_smoke_enabled(self.to_ptr(), value) }
			}
			pub fn opacity(&self) -> f64 {
				unsafe { prop_smoke_opacity(self.to_ptr()) }
			}
			pub fn set_opacity(&self, value: f64) {
				unsafe { prop_set_smoke_opacity(self.to_ptr(), value) }
			}
			pub fn rise_velocity(&self) -> f64 {
				unsafe { prop_smoke_rise_velocity(self.to_ptr()) }
			}
			pub fn set_rise_velocity(&self, value: f64) {
				unsafe { prop_set_smoke_rise_velocity(self.to_ptr(), value) }
			}
			pub fn size(&self) -> f64 {
				unsafe { prop_smoke_size(self.to_ptr()) }
			}
			pub fn set_size(&self, value: f64) {
				unsafe { prop_set_smoke_size(self.to_ptr(), value) }
			}
			pub fn time_scale(&self) -> f64 {
				unsafe { prop_smoke_time_scale(self.to_ptr()) }
			}
			pub fn set_time_scale(&self, value: f64) {
				unsafe { prop_set_smoke_time_scale(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_social_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_prompt_game_invite(&self, player: Option<Instance>) {
				unsafe { dyn_social_service_prompt_game_invite(self.to_ptr(), player) }
			}
			pub fn fn_can_send_game_invite_async(&self, player: Option<Instance>) -> bool {
				unsafe { dyn_social_service_can_send_game_invite_async(self.to_ptr(), player) }
			}
			pub fn on_game_invite_prompt_closed<F: 'static + Fn(Option<Instance>, ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_social_service_game_invite_prompt_closed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_sound {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn emitter_size(&self) -> f64 {
				unsafe { prop_sound_emitter_size(self.to_ptr()) }
			}
			pub fn set_emitter_size(&self, value: f64) {
				unsafe { prop_set_sound_emitter_size(self.to_ptr(), value) }
			}
			pub fn is_loaded(&self) -> bool {
				unsafe { prop_sound_is_loaded(self.to_ptr()) }
			}
			pub fn is_paused(&self) -> bool {
				unsafe { prop_sound_is_paused(self.to_ptr()) }
			}
			pub fn is_playing(&self) -> bool {
				unsafe { prop_sound_is_playing(self.to_ptr()) }
			}
			pub fn looped(&self) -> bool {
				unsafe { prop_sound_looped(self.to_ptr()) }
			}
			pub fn set_looped(&self, value: bool) {
				unsafe { prop_set_sound_looped(self.to_ptr(), value) }
			}
			pub fn max_distance(&self) -> f64 {
				unsafe { prop_sound_max_distance(self.to_ptr()) }
			}
			pub fn set_max_distance(&self, value: f64) {
				unsafe { prop_set_sound_max_distance(self.to_ptr(), value) }
			}
			pub fn min_distance(&self) -> f64 {
				unsafe { prop_sound_min_distance(self.to_ptr()) }
			}
			pub fn set_min_distance(&self, value: f64) {
				unsafe { prop_set_sound_min_distance(self.to_ptr(), value) }
			}
			pub fn pitch(&self) -> f64 {
				unsafe { prop_sound_pitch(self.to_ptr()) }
			}
			pub fn set_pitch(&self, value: f64) {
				unsafe { prop_set_sound_pitch(self.to_ptr(), value) }
			}
			pub fn play_on_remove(&self) -> bool {
				unsafe { prop_sound_play_on_remove(self.to_ptr()) }
			}
			pub fn set_play_on_remove(&self, value: bool) {
				unsafe { prop_set_sound_play_on_remove(self.to_ptr(), value) }
			}
			pub fn playback_loudness(&self) -> f64 {
				unsafe { prop_sound_playback_loudness(self.to_ptr()) }
			}
			pub fn playback_speed(&self) -> f64 {
				unsafe { prop_sound_playback_speed(self.to_ptr()) }
			}
			pub fn set_playback_speed(&self, value: f64) {
				unsafe { prop_set_sound_playback_speed(self.to_ptr(), value) }
			}
			pub fn playing(&self) -> bool {
				unsafe { prop_sound_playing(self.to_ptr()) }
			}
			pub fn set_playing(&self, value: bool) {
				unsafe { prop_set_sound_playing(self.to_ptr(), value) }
			}
			pub fn roll_off_max_distance(&self) -> f64 {
				unsafe { prop_sound_roll_off_max_distance(self.to_ptr()) }
			}
			pub fn set_roll_off_max_distance(&self, value: f64) {
				unsafe { prop_set_sound_roll_off_max_distance(self.to_ptr(), value) }
			}
			pub fn roll_off_min_distance(&self) -> f64 {
				unsafe { prop_sound_roll_off_min_distance(self.to_ptr()) }
			}
			pub fn set_roll_off_min_distance(&self, value: f64) {
				unsafe { prop_set_sound_roll_off_min_distance(self.to_ptr(), value) }
			}
			pub fn sound_group(&self) -> Option<SoundGroup> {
				unsafe { prop_sound_sound_group(self.to_ptr()) }
			}
			pub fn set_sound_group(&self, value: Option<SoundGroup>) {
				unsafe { prop_set_sound_sound_group(self.to_ptr(), value) }
			}
			pub fn sound_id(&self) -> Content {
				unsafe { prop_sound_sound_id(self.to_ptr()) }
			}
			pub fn set_sound_id(&self, value: Content) {
				unsafe { prop_set_sound_sound_id(self.to_ptr(), value) }
			}
			pub fn time_length(&self) -> f64 {
				unsafe { prop_sound_time_length(self.to_ptr()) }
			}
			pub fn time_position(&self) -> f64 {
				unsafe { prop_sound_time_position(self.to_ptr()) }
			}
			pub fn set_time_position(&self, value: f64) {
				unsafe { prop_set_sound_time_position(self.to_ptr(), value) }
			}
			pub fn volume(&self) -> f64 {
				unsafe { prop_sound_volume(self.to_ptr()) }
			}
			pub fn set_volume(&self, value: f64) {
				unsafe { prop_set_sound_volume(self.to_ptr(), value) }
			}
			pub fn fn_pause(&self) {
				unsafe { dyn_sound_pause(self.to_ptr()) }
			}
			pub fn fn_play(&self) {
				unsafe { dyn_sound_play(self.to_ptr()) }
			}
			pub fn fn_resume(&self) {
				unsafe { dyn_sound_resume(self.to_ptr()) }
			}
			pub fn fn_stop(&self) {
				unsafe { dyn_sound_stop(self.to_ptr()) }
			}
			pub fn on_did_loop<F: 'static + Fn(String, f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_sound_did_loop(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_ended<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_sound_ended(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_loaded<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_sound_loaded(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_paused<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_sound_paused(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_played<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_sound_played(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_resumed<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_sound_resumed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_stopped<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_sound_stopped(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_sound_effect {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn enabled(&self) -> bool {
				unsafe { prop_sound_effect_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_sound_effect_enabled(self.to_ptr(), value) }
			}
			pub fn priority(&self) -> f64 {
				unsafe { prop_sound_effect_priority(self.to_ptr()) }
			}
			pub fn set_priority(&self, value: f64) {
				unsafe { prop_set_sound_effect_priority(self.to_ptr(), value) }
			}
		}
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
				unsafe { prop_chorus_sound_effect_depth(self.to_ptr()) }
			}
			pub fn set_depth(&self, value: f64) {
				unsafe { prop_set_chorus_sound_effect_depth(self.to_ptr(), value) }
			}
			pub fn mix(&self) -> f64 {
				unsafe { prop_chorus_sound_effect_mix(self.to_ptr()) }
			}
			pub fn set_mix(&self, value: f64) {
				unsafe { prop_set_chorus_sound_effect_mix(self.to_ptr(), value) }
			}
			pub fn rate(&self) -> f64 {
				unsafe { prop_chorus_sound_effect_rate(self.to_ptr()) }
			}
			pub fn set_rate(&self, value: f64) {
				unsafe { prop_set_chorus_sound_effect_rate(self.to_ptr(), value) }
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
				unsafe { prop_compressor_sound_effect_attack(self.to_ptr()) }
			}
			pub fn set_attack(&self, value: f64) {
				unsafe { prop_set_compressor_sound_effect_attack(self.to_ptr(), value) }
			}
			pub fn gain_makeup(&self) -> f64 {
				unsafe { prop_compressor_sound_effect_gain_makeup(self.to_ptr()) }
			}
			pub fn set_gain_makeup(&self, value: f64) {
				unsafe { prop_set_compressor_sound_effect_gain_makeup(self.to_ptr(), value) }
			}
			pub fn ratio(&self) -> f64 {
				unsafe { prop_compressor_sound_effect_ratio(self.to_ptr()) }
			}
			pub fn set_ratio(&self, value: f64) {
				unsafe { prop_set_compressor_sound_effect_ratio(self.to_ptr(), value) }
			}
			pub fn release(&self) -> f64 {
				unsafe { prop_compressor_sound_effect_release(self.to_ptr()) }
			}
			pub fn set_release(&self, value: f64) {
				unsafe { prop_set_compressor_sound_effect_release(self.to_ptr(), value) }
			}
			pub fn side_chain(&self) -> Option<Instance> {
				unsafe { prop_compressor_sound_effect_side_chain(self.to_ptr()) }
			}
			pub fn set_side_chain(&self, value: Option<Instance>) {
				unsafe { prop_set_compressor_sound_effect_side_chain(self.to_ptr(), value) }
			}
			pub fn threshold(&self) -> f64 {
				unsafe { prop_compressor_sound_effect_threshold(self.to_ptr()) }
			}
			pub fn set_threshold(&self, value: f64) {
				unsafe { prop_set_compressor_sound_effect_threshold(self.to_ptr(), value) }
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
				unsafe { prop_channel_selector_sound_effect_channel(self.to_ptr()) }
			}
			pub fn set_channel(&self, value: f64) {
				unsafe { prop_set_channel_selector_sound_effect_channel(self.to_ptr(), value) }
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
				unsafe { prop_distortion_sound_effect_level(self.to_ptr()) }
			}
			pub fn set_level(&self, value: f64) {
				unsafe { prop_set_distortion_sound_effect_level(self.to_ptr(), value) }
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
				unsafe { prop_echo_sound_effect_delay(self.to_ptr()) }
			}
			pub fn set_delay(&self, value: f64) {
				unsafe { prop_set_echo_sound_effect_delay(self.to_ptr(), value) }
			}
			pub fn dry_level(&self) -> f64 {
				unsafe { prop_echo_sound_effect_dry_level(self.to_ptr()) }
			}
			pub fn set_dry_level(&self, value: f64) {
				unsafe { prop_set_echo_sound_effect_dry_level(self.to_ptr(), value) }
			}
			pub fn feedback(&self) -> f64 {
				unsafe { prop_echo_sound_effect_feedback(self.to_ptr()) }
			}
			pub fn set_feedback(&self, value: f64) {
				unsafe { prop_set_echo_sound_effect_feedback(self.to_ptr(), value) }
			}
			pub fn wet_level(&self) -> f64 {
				unsafe { prop_echo_sound_effect_wet_level(self.to_ptr()) }
			}
			pub fn set_wet_level(&self, value: f64) {
				unsafe { prop_set_echo_sound_effect_wet_level(self.to_ptr(), value) }
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
				unsafe { prop_equalizer_sound_effect_high_gain(self.to_ptr()) }
			}
			pub fn set_high_gain(&self, value: f64) {
				unsafe { prop_set_equalizer_sound_effect_high_gain(self.to_ptr(), value) }
			}
			pub fn low_gain(&self) -> f64 {
				unsafe { prop_equalizer_sound_effect_low_gain(self.to_ptr()) }
			}
			pub fn set_low_gain(&self, value: f64) {
				unsafe { prop_set_equalizer_sound_effect_low_gain(self.to_ptr(), value) }
			}
			pub fn mid_gain(&self) -> f64 {
				unsafe { prop_equalizer_sound_effect_mid_gain(self.to_ptr()) }
			}
			pub fn set_mid_gain(&self, value: f64) {
				unsafe { prop_set_equalizer_sound_effect_mid_gain(self.to_ptr(), value) }
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
				unsafe { prop_flange_sound_effect_depth(self.to_ptr()) }
			}
			pub fn set_depth(&self, value: f64) {
				unsafe { prop_set_flange_sound_effect_depth(self.to_ptr(), value) }
			}
			pub fn mix(&self) -> f64 {
				unsafe { prop_flange_sound_effect_mix(self.to_ptr()) }
			}
			pub fn set_mix(&self, value: f64) {
				unsafe { prop_set_flange_sound_effect_mix(self.to_ptr(), value) }
			}
			pub fn rate(&self) -> f64 {
				unsafe { prop_flange_sound_effect_rate(self.to_ptr()) }
			}
			pub fn set_rate(&self, value: f64) {
				unsafe { prop_set_flange_sound_effect_rate(self.to_ptr(), value) }
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
				unsafe { prop_pitch_shift_sound_effect_octave(self.to_ptr()) }
			}
			pub fn set_octave(&self, value: f64) {
				unsafe { prop_set_pitch_shift_sound_effect_octave(self.to_ptr(), value) }
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
				unsafe { prop_reverb_sound_effect_decay_time(self.to_ptr()) }
			}
			pub fn set_decay_time(&self, value: f64) {
				unsafe { prop_set_reverb_sound_effect_decay_time(self.to_ptr(), value) }
			}
			pub fn density(&self) -> f64 {
				unsafe { prop_reverb_sound_effect_density(self.to_ptr()) }
			}
			pub fn set_density(&self, value: f64) {
				unsafe { prop_set_reverb_sound_effect_density(self.to_ptr(), value) }
			}
			pub fn diffusion(&self) -> f64 {
				unsafe { prop_reverb_sound_effect_diffusion(self.to_ptr()) }
			}
			pub fn set_diffusion(&self, value: f64) {
				unsafe { prop_set_reverb_sound_effect_diffusion(self.to_ptr(), value) }
			}
			pub fn dry_level(&self) -> f64 {
				unsafe { prop_reverb_sound_effect_dry_level(self.to_ptr()) }
			}
			pub fn set_dry_level(&self, value: f64) {
				unsafe { prop_set_reverb_sound_effect_dry_level(self.to_ptr(), value) }
			}
			pub fn wet_level(&self) -> f64 {
				unsafe { prop_reverb_sound_effect_wet_level(self.to_ptr()) }
			}
			pub fn set_wet_level(&self, value: f64) {
				unsafe { prop_set_reverb_sound_effect_wet_level(self.to_ptr(), value) }
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
				unsafe { prop_tremolo_sound_effect_depth(self.to_ptr()) }
			}
			pub fn set_depth(&self, value: f64) {
				unsafe { prop_set_tremolo_sound_effect_depth(self.to_ptr(), value) }
			}
			pub fn duty(&self) -> f64 {
				unsafe { prop_tremolo_sound_effect_duty(self.to_ptr()) }
			}
			pub fn set_duty(&self, value: f64) {
				unsafe { prop_set_tremolo_sound_effect_duty(self.to_ptr(), value) }
			}
			pub fn frequency(&self) -> f64 {
				unsafe { prop_tremolo_sound_effect_frequency(self.to_ptr()) }
			}
			pub fn set_frequency(&self, value: f64) {
				unsafe { prop_set_tremolo_sound_effect_frequency(self.to_ptr(), value) }
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn volume(&self) -> f64 {
				unsafe { prop_sound_group_volume(self.to_ptr()) }
			}
			pub fn set_volume(&self, value: f64) {
				unsafe { prop_set_sound_group_volume(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_sound_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn distance_factor(&self) -> f64 {
				unsafe { prop_sound_service_distance_factor(self.to_ptr()) }
			}
			pub fn set_distance_factor(&self, value: f64) {
				unsafe { prop_set_sound_service_distance_factor(self.to_ptr(), value) }
			}
			pub fn doppler_scale(&self) -> f64 {
				unsafe { prop_sound_service_doppler_scale(self.to_ptr()) }
			}
			pub fn set_doppler_scale(&self, value: f64) {
				unsafe { prop_set_sound_service_doppler_scale(self.to_ptr(), value) }
			}
			pub fn respect_filtering_enabled(&self) -> bool {
				unsafe { prop_sound_service_respect_filtering_enabled(self.to_ptr()) }
			}
			pub fn set_respect_filtering_enabled(&self, value: bool) {
				unsafe { prop_set_sound_service_respect_filtering_enabled(self.to_ptr(), value) }
			}
			pub fn rolloff_scale(&self) -> f64 {
				unsafe { prop_sound_service_rolloff_scale(self.to_ptr()) }
			}
			pub fn set_rolloff_scale(&self, value: f64) {
				unsafe { prop_set_sound_service_rolloff_scale(self.to_ptr(), value) }
			}
			pub fn fn_get_listener(&self) {
				unsafe { dyn_sound_service_get_listener(self.to_ptr()) }
			}
			pub fn fn_play_local_sound(&self, sound: Option<Instance>) {
				unsafe { dyn_sound_service_play_local_sound(self.to_ptr(), sound) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_sparkles {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn color(&self) -> Color3 {
				unsafe { prop_sparkles_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { prop_set_sparkles_color(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_sparkles_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_sparkles_enabled(self.to_ptr(), value) }
			}
			pub fn sparkle_color(&self) -> Color3 {
				unsafe { prop_sparkles_sparkle_color(self.to_ptr()) }
			}
			pub fn set_sparkle_color(&self, value: Color3) {
				unsafe { prop_set_sparkles_sparkle_color(self.to_ptr(), value) }
			}
			pub fn time_scale(&self) -> f64 {
				unsafe { prop_sparkles_time_scale(self.to_ptr()) }
			}
			pub fn set_time_scale(&self, value: f64) {
				unsafe { prop_set_sparkles_time_scale(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_speaker {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn channel_count(&self) -> f64 {
				unsafe { prop_speaker_channel_count(self.to_ptr()) }
			}
			pub fn playback_loudness(&self) -> f64 {
				unsafe { prop_speaker_playback_loudness(self.to_ptr()) }
			}
			pub fn roll_off_max_distance(&self) -> f64 {
				unsafe { prop_speaker_roll_off_max_distance(self.to_ptr()) }
			}
			pub fn set_roll_off_max_distance(&self, value: f64) {
				unsafe { prop_set_speaker_roll_off_max_distance(self.to_ptr(), value) }
			}
			pub fn roll_off_min_distance(&self) -> f64 {
				unsafe { prop_speaker_roll_off_min_distance(self.to_ptr()) }
			}
			pub fn set_roll_off_min_distance(&self, value: f64) {
				unsafe { prop_set_speaker_roll_off_min_distance(self.to_ptr(), value) }
			}
			pub fn sound_group(&self) -> Option<SoundGroup> {
				unsafe { prop_speaker_sound_group(self.to_ptr()) }
			}
			pub fn set_sound_group(&self, value: Option<SoundGroup>) {
				unsafe { prop_set_speaker_sound_group(self.to_ptr(), value) }
			}
			pub fn source(&self) -> Option<Instance> {
				unsafe { prop_speaker_source(self.to_ptr()) }
			}
			pub fn set_source(&self, value: Option<Instance>) {
				unsafe { prop_set_speaker_source(self.to_ptr(), value) }
			}
			pub fn volume(&self) -> f64 {
				unsafe { prop_speaker_volume(self.to_ptr()) }
			}
			pub fn set_volume(&self, value: f64) {
				unsafe { prop_set_speaker_volume(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_starter_gear {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_starter_pack {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_starter_player {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn allow_custom_animations(&self) -> bool {
				unsafe { prop_starter_player_allow_custom_animations(self.to_ptr()) }
			}
			pub fn auto_jump_enabled(&self) -> bool {
				unsafe { prop_starter_player_auto_jump_enabled(self.to_ptr()) }
			}
			pub fn set_auto_jump_enabled(&self, value: bool) {
				unsafe { prop_set_starter_player_auto_jump_enabled(self.to_ptr(), value) }
			}
			pub fn camera_max_zoom_distance(&self) -> f64 {
				unsafe { prop_starter_player_camera_max_zoom_distance(self.to_ptr()) }
			}
			pub fn set_camera_max_zoom_distance(&self, value: f64) {
				unsafe { prop_set_starter_player_camera_max_zoom_distance(self.to_ptr(), value) }
			}
			pub fn camera_min_zoom_distance(&self) -> f64 {
				unsafe { prop_starter_player_camera_min_zoom_distance(self.to_ptr()) }
			}
			pub fn set_camera_min_zoom_distance(&self, value: f64) {
				unsafe { prop_set_starter_player_camera_min_zoom_distance(self.to_ptr(), value) }
			}
			pub fn character_jump_height(&self) -> f64 {
				unsafe { prop_starter_player_character_jump_height(self.to_ptr()) }
			}
			pub fn set_character_jump_height(&self, value: f64) {
				unsafe { prop_set_starter_player_character_jump_height(self.to_ptr(), value) }
			}
			pub fn character_jump_power(&self) -> f64 {
				unsafe { prop_starter_player_character_jump_power(self.to_ptr()) }
			}
			pub fn set_character_jump_power(&self, value: f64) {
				unsafe { prop_set_starter_player_character_jump_power(self.to_ptr(), value) }
			}
			pub fn character_max_slope_angle(&self) -> f64 {
				unsafe { prop_starter_player_character_max_slope_angle(self.to_ptr()) }
			}
			pub fn set_character_max_slope_angle(&self, value: f64) {
				unsafe { prop_set_starter_player_character_max_slope_angle(self.to_ptr(), value) }
			}
			pub fn character_use_jump_power(&self) -> bool {
				unsafe { prop_starter_player_character_use_jump_power(self.to_ptr()) }
			}
			pub fn set_character_use_jump_power(&self, value: bool) {
				unsafe { prop_set_starter_player_character_use_jump_power(self.to_ptr(), value) }
			}
			pub fn character_walk_speed(&self) -> f64 {
				unsafe { prop_starter_player_character_walk_speed(self.to_ptr()) }
			}
			pub fn set_character_walk_speed(&self, value: f64) {
				unsafe { prop_set_starter_player_character_walk_speed(self.to_ptr(), value) }
			}
			pub fn enable_mouse_lock_option(&self) -> bool {
				unsafe { prop_starter_player_enable_mouse_lock_option(self.to_ptr()) }
			}
			pub fn set_enable_mouse_lock_option(&self, value: bool) {
				unsafe { prop_set_starter_player_enable_mouse_lock_option(self.to_ptr(), value) }
			}
			pub fn health_display_distance(&self) -> f64 {
				unsafe { prop_starter_player_health_display_distance(self.to_ptr()) }
			}
			pub fn set_health_display_distance(&self, value: f64) {
				unsafe { prop_set_starter_player_health_display_distance(self.to_ptr(), value) }
			}
			pub fn load_character_appearance(&self) -> bool {
				unsafe { prop_starter_player_load_character_appearance(self.to_ptr()) }
			}
			pub fn set_load_character_appearance(&self, value: bool) {
				unsafe { prop_set_starter_player_load_character_appearance(self.to_ptr(), value) }
			}
			pub fn name_display_distance(&self) -> f64 {
				unsafe { prop_starter_player_name_display_distance(self.to_ptr()) }
			}
			pub fn set_name_display_distance(&self, value: f64) {
				unsafe { prop_set_starter_player_name_display_distance(self.to_ptr(), value) }
			}
			pub fn user_emotes_enabled(&self) -> bool {
				unsafe { prop_starter_player_user_emotes_enabled(self.to_ptr()) }
			}
			pub fn set_user_emotes_enabled(&self, value: bool) {
				unsafe { prop_set_starter_player_user_emotes_enabled(self.to_ptr(), value) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_starter_player_scripts {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
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
macro_rules! impl_stats {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn contacts_count(&self) -> f64 {
				unsafe { prop_stats_contacts_count(self.to_ptr()) }
			}
			pub fn data_receive_kbps(&self) -> f64 {
				unsafe { prop_stats_data_receive_kbps(self.to_ptr()) }
			}
			pub fn data_send_kbps(&self) -> f64 {
				unsafe { prop_stats_data_send_kbps(self.to_ptr()) }
			}
			pub fn heartbeat_time_ms(&self) -> f64 {
				unsafe { prop_stats_heartbeat_time_ms(self.to_ptr()) }
			}
			pub fn instance_count(&self) -> f64 {
				unsafe { prop_stats_instance_count(self.to_ptr()) }
			}
			pub fn moving_primitives_count(&self) -> f64 {
				unsafe { prop_stats_moving_primitives_count(self.to_ptr()) }
			}
			pub fn physics_receive_kbps(&self) -> f64 {
				unsafe { prop_stats_physics_receive_kbps(self.to_ptr()) }
			}
			pub fn physics_send_kbps(&self) -> f64 {
				unsafe { prop_stats_physics_send_kbps(self.to_ptr()) }
			}
			pub fn physics_step_time_ms(&self) -> f64 {
				unsafe { prop_stats_physics_step_time_ms(self.to_ptr()) }
			}
			pub fn primitives_count(&self) -> f64 {
				unsafe { prop_stats_primitives_count(self.to_ptr()) }
			}
			pub fn fn_get_total_memory_usage_mb(&self) -> f64 {
				unsafe { dyn_stats_get_total_memory_usage_mb(self.to_ptr()) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_surface_appearance {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_team {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn auto_assignable(&self) -> bool {
				unsafe { prop_team_auto_assignable(self.to_ptr()) }
			}
			pub fn set_auto_assignable(&self, value: bool) {
				unsafe { prop_set_team_auto_assignable(self.to_ptr(), value) }
			}
			pub fn auto_color_characters(&self) -> bool {
				unsafe { prop_team_auto_color_characters(self.to_ptr()) }
			}
			pub fn set_auto_color_characters(&self, value: bool) {
				unsafe { prop_set_team_auto_color_characters(self.to_ptr(), value) }
			}
			pub fn score(&self) -> f64 {
				unsafe { prop_team_score(self.to_ptr()) }
			}
			pub fn set_score(&self, value: f64) {
				unsafe { prop_set_team_score(self.to_ptr(), value) }
			}
			pub fn team_color(&self) -> BrickColor {
				unsafe { prop_team_team_color(self.to_ptr()) }
			}
			pub fn set_team_color(&self, value: BrickColor) {
				unsafe { prop_set_team_team_color(self.to_ptr(), value) }
			}
			pub fn fn_get_players(&self) -> Objects {
				unsafe { dyn_team_get_players(self.to_ptr()) }
			}
			pub fn on_player_added<F: 'static + Fn(Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_team_player_added(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_player_removed<F: 'static + Fn(Option<Player>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_team_player_removed(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_teams {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_teams(&self) -> Objects {
				unsafe { dyn_teams_get_teams(self.to_ptr()) }
			}
			pub fn fn_rebalance_teams(&self) {
				unsafe { dyn_teams_rebalance_teams(self.to_ptr()) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_teleport_async_result {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn private_server_id(&self) -> String {
				unsafe { prop_teleport_async_result_private_server_id(self.to_ptr()) }
			}
			pub fn reserved_server_access_code(&self) -> String {
				unsafe { prop_teleport_async_result_reserved_server_access_code(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_teleport_options {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn reserved_server_access_code(&self) -> String {
				unsafe { prop_teleport_options_reserved_server_access_code(self.to_ptr()) }
			}
			pub fn set_reserved_server_access_code(&self, value: &str) {
				unsafe { prop_set_teleport_options_reserved_server_access_code(self.to_ptr(), value) }
			}
			pub fn server_instance_id(&self) -> String {
				unsafe { prop_teleport_options_server_instance_id(self.to_ptr()) }
			}
			pub fn set_server_instance_id(&self, value: &str) {
				unsafe { prop_set_teleport_options_server_instance_id(self.to_ptr(), value) }
			}
			pub fn should_reserve_server(&self) -> bool {
				unsafe { prop_teleport_options_should_reserve_server(self.to_ptr()) }
			}
			pub fn set_should_reserve_server(&self, value: bool) {
				unsafe { prop_set_teleport_options_should_reserve_server(self.to_ptr(), value) }
			}
			pub fn fn_get_teleport_data(&self) {
				unsafe { dyn_teleport_options_get_teleport_data(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_teleport_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn customized_teleport_ui(&self) -> bool {
				unsafe { prop_teleport_service_customized_teleport_ui(self.to_ptr()) }
			}
			pub fn set_customized_teleport_ui(&self, value: bool) {
				unsafe { prop_set_teleport_service_customized_teleport_ui(self.to_ptr(), value) }
			}
			pub fn fn_get_arriving_teleport_gui(&self) -> Option<Instance> {
				unsafe { dyn_teleport_service_get_arriving_teleport_gui(self.to_ptr()) }
			}
			pub fn fn_get_local_player_teleport_data(&self) {
				unsafe { dyn_teleport_service_get_local_player_teleport_data(self.to_ptr()) }
			}
			pub fn fn_get_teleport_setting(&self, setting: &str) {
				unsafe { dyn_teleport_service_get_teleport_setting(self.to_ptr(), setting) }
			}
			pub fn fn_set_teleport_gui(&self, gui: Option<Instance>) {
				unsafe { dyn_teleport_service_set_teleport_gui(self.to_ptr(), gui) }
			}
			pub fn fn_get_player_place_instance_async(&self, user_id: f64) {
				unsafe { dyn_teleport_service_get_player_place_instance_async(self.to_ptr(), user_id) }
			}
			pub fn fn_reserve_server(&self, place_id: f64) {
				unsafe { dyn_teleport_service_reserve_server(self.to_ptr(), place_id) }
			}
			pub fn fn_teleport_async(&self, place_id: f64, players: Objects, teleport_options: Option<Instance>) -> Option<Instance> {
				unsafe { dyn_teleport_service_teleport_async(self.to_ptr(), place_id, players, teleport_options) }
			}
			pub fn on_local_player_arrived_from_teleport<F: 'static + Fn(Option<Instance>, ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_teleport_service_local_player_arrived_from_teleport(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_teleport_init_failed<F: 'static + Fn(Option<Instance>, (), String, f64, Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_teleport_service_teleport_init_failed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_temporary_cage_mesh_provider {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_temporary_script_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_terrain_detail {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn studs_per_tile(&self) -> f64 {
				unsafe { prop_terrain_detail_studs_per_tile(self.to_ptr()) }
			}
			pub fn set_studs_per_tile(&self, value: f64) {
				unsafe { prop_set_terrain_detail_studs_per_tile(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_terrain_region {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn is_smooth(&self) -> bool {
				unsafe { prop_terrain_region_is_smooth(self.to_ptr()) }
			}
			pub fn size_in_cells(&self) -> Vector3 {
				unsafe { prop_terrain_region_size_in_cells(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_box_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_channel {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_display_system_message(&self, system_message: &str, metadata: &str) -> Option<TextChatMessage> {
				unsafe { dyn_text_channel_display_system_message(self.to_ptr(), system_message, metadata) }
			}
			pub fn fn_add_user_async(&self, user_id: f64) {
				unsafe { dyn_text_channel_add_user_async(self.to_ptr(), user_id) }
			}
			pub fn fn_send_async(&self, message: &str, metadata: &str) -> Option<TextChatMessage> {
				unsafe { dyn_text_channel_send_async(self.to_ptr(), message, metadata) }
			}
			pub fn on_message_received<F: 'static + Fn(Option<TextChatMessage>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_text_channel_message_received(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_chat_command {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn enabled(&self) -> bool {
				unsafe { prop_text_chat_command_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_text_chat_command_enabled(self.to_ptr(), value) }
			}
			pub fn primary_alias(&self) -> String {
				unsafe { prop_text_chat_command_primary_alias(self.to_ptr()) }
			}
			pub fn set_primary_alias(&self, value: &str) {
				unsafe { prop_set_text_chat_command_primary_alias(self.to_ptr(), value) }
			}
			pub fn secondary_alias(&self) -> String {
				unsafe { prop_text_chat_command_secondary_alias(self.to_ptr()) }
			}
			pub fn set_secondary_alias(&self, value: &str) {
				unsafe { prop_set_text_chat_command_secondary_alias(self.to_ptr(), value) }
			}
			pub fn on_triggered<F: 'static + Fn(Option<TextSource>, String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_text_chat_command_triggered(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_chat_configurations {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
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
				unsafe { prop_chat_input_bar_configuration_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_chat_input_bar_configuration_enabled(self.to_ptr(), value) }
			}
			pub fn target_text_channel(&self) -> Option<TextChannel> {
				unsafe { prop_chat_input_bar_configuration_target_text_channel(self.to_ptr()) }
			}
			pub fn set_target_text_channel(&self, value: Option<TextChannel>) {
				unsafe { prop_set_chat_input_bar_configuration_target_text_channel(self.to_ptr(), value) }
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
				unsafe { prop_chat_window_configuration_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_chat_window_configuration_enabled(self.to_ptr(), value) }
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn message_id(&self) -> String {
				unsafe { prop_text_chat_message_message_id(self.to_ptr()) }
			}
			pub fn set_message_id(&self, value: &str) {
				unsafe { prop_set_text_chat_message_message_id(self.to_ptr(), value) }
			}
			pub fn metadata(&self) -> String {
				unsafe { prop_text_chat_message_metadata(self.to_ptr()) }
			}
			pub fn set_metadata(&self, value: &str) {
				unsafe { prop_set_text_chat_message_metadata(self.to_ptr(), value) }
			}
			pub fn prefix_text(&self) -> String {
				unsafe { prop_text_chat_message_prefix_text(self.to_ptr()) }
			}
			pub fn set_prefix_text(&self, value: &str) {
				unsafe { prop_set_text_chat_message_prefix_text(self.to_ptr(), value) }
			}
			pub fn text(&self) -> String {
				unsafe { prop_text_chat_message_text(self.to_ptr()) }
			}
			pub fn set_text(&self, value: &str) {
				unsafe { prop_set_text_chat_message_text(self.to_ptr(), value) }
			}
			pub fn text_channel(&self) -> Option<TextChannel> {
				unsafe { prop_text_chat_message_text_channel(self.to_ptr()) }
			}
			pub fn set_text_channel(&self, value: Option<TextChannel>) {
				unsafe { prop_set_text_chat_message_text_channel(self.to_ptr(), value) }
			}
			pub fn text_source(&self) -> Option<TextSource> {
				unsafe { prop_text_chat_message_text_source(self.to_ptr()) }
			}
			pub fn set_text_source(&self, value: Option<TextSource>) {
				unsafe { prop_set_text_chat_message_text_source(self.to_ptr(), value) }
			}
			pub fn timestamp(&self) -> DateTime {
				unsafe { prop_text_chat_message_timestamp(self.to_ptr()) }
			}
			pub fn set_timestamp(&self, value: DateTime) {
				unsafe { prop_set_text_chat_message_timestamp(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_chat_message_properties {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn prefix_text(&self) -> String {
				unsafe { prop_text_chat_message_properties_prefix_text(self.to_ptr()) }
			}
			pub fn set_prefix_text(&self, value: &str) {
				unsafe { prop_set_text_chat_message_properties_prefix_text(self.to_ptr(), value) }
			}
			pub fn text(&self) -> String {
				unsafe { prop_text_chat_message_properties_text(self.to_ptr()) }
			}
			pub fn set_text(&self, value: &str) {
				unsafe { prop_set_text_chat_message_properties_text(self.to_ptr(), value) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_chat_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn create_default_commands(&self) -> bool {
				unsafe { prop_text_chat_service_create_default_commands(self.to_ptr()) }
			}
			pub fn create_default_text_channels(&self) -> bool {
				unsafe { prop_text_chat_service_create_default_text_channels(self.to_ptr()) }
			}
			pub fn on_message_received<F: 'static + Fn(Option<TextChatMessage>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_text_chat_service_message_received(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_sending_message<F: 'static + Fn(Option<TextChatMessage>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_text_chat_service_sending_message(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_filter_result {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_chat_for_user_async(&self, to_user_id: f64) -> String {
				unsafe { dyn_text_filter_result_get_chat_for_user_async(self.to_ptr(), to_user_id) }
			}
			pub fn fn_get_non_chat_string_for_broadcast_async(&self) -> String {
				unsafe { dyn_text_filter_result_get_non_chat_string_for_broadcast_async(self.to_ptr()) }
			}
			pub fn fn_get_non_chat_string_for_user_async(&self, to_user_id: f64) -> String {
				unsafe { dyn_text_filter_result_get_non_chat_string_for_user_async(self.to_ptr(), to_user_id) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_family_info_async(&self, asset_id: Content) {
				unsafe { dyn_text_service_get_family_info_async(self.to_ptr(), asset_id) }
			}
			pub fn fn_get_text_bounds_async(&self, params: Option<GetTextBoundsParams>) -> Vector2 {
				unsafe { dyn_text_service_get_text_bounds_async(self.to_ptr(), params) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_text_source {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn can_send(&self) -> bool {
				unsafe { prop_text_source_can_send(self.to_ptr()) }
			}
			pub fn set_can_send(&self, value: bool) {
				unsafe { prop_set_text_source_can_send(self.to_ptr(), value) }
			}
			pub fn user_id(&self) -> f64 {
				unsafe { prop_text_source_user_id(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_touch_transmitter {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_trail {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn attachment_0(&self) -> Option<Attachment> {
				unsafe { prop_trail_attachment_0(self.to_ptr()) }
			}
			pub fn set_attachment_0(&self, value: Option<Attachment>) {
				unsafe { prop_set_trail_attachment_0(self.to_ptr(), value) }
			}
			pub fn attachment_1(&self) -> Option<Attachment> {
				unsafe { prop_trail_attachment_1(self.to_ptr()) }
			}
			pub fn set_attachment_1(&self, value: Option<Attachment>) {
				unsafe { prop_set_trail_attachment_1(self.to_ptr(), value) }
			}
			pub fn brightness(&self) -> f64 {
				unsafe { prop_trail_brightness(self.to_ptr()) }
			}
			pub fn set_brightness(&self, value: f64) {
				unsafe { prop_set_trail_brightness(self.to_ptr(), value) }
			}
			pub fn color(&self) -> ColorSequence {
				unsafe { prop_trail_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: ColorSequence) {
				unsafe { prop_set_trail_color(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_trail_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_trail_enabled(self.to_ptr(), value) }
			}
			pub fn face_camera(&self) -> bool {
				unsafe { prop_trail_face_camera(self.to_ptr()) }
			}
			pub fn set_face_camera(&self, value: bool) {
				unsafe { prop_set_trail_face_camera(self.to_ptr(), value) }
			}
			pub fn lifetime(&self) -> f64 {
				unsafe { prop_trail_lifetime(self.to_ptr()) }
			}
			pub fn set_lifetime(&self, value: f64) {
				unsafe { prop_set_trail_lifetime(self.to_ptr(), value) }
			}
			pub fn light_emission(&self) -> f64 {
				unsafe { prop_trail_light_emission(self.to_ptr()) }
			}
			pub fn set_light_emission(&self, value: f64) {
				unsafe { prop_set_trail_light_emission(self.to_ptr(), value) }
			}
			pub fn light_influence(&self) -> f64 {
				unsafe { prop_trail_light_influence(self.to_ptr()) }
			}
			pub fn set_light_influence(&self, value: f64) {
				unsafe { prop_set_trail_light_influence(self.to_ptr(), value) }
			}
			pub fn max_length(&self) -> f64 {
				unsafe { prop_trail_max_length(self.to_ptr()) }
			}
			pub fn set_max_length(&self, value: f64) {
				unsafe { prop_set_trail_max_length(self.to_ptr(), value) }
			}
			pub fn min_length(&self) -> f64 {
				unsafe { prop_trail_min_length(self.to_ptr()) }
			}
			pub fn set_min_length(&self, value: f64) {
				unsafe { prop_set_trail_min_length(self.to_ptr(), value) }
			}
			pub fn texture(&self) -> Content {
				unsafe { prop_trail_texture(self.to_ptr()) }
			}
			pub fn set_texture(&self, value: Content) {
				unsafe { prop_set_trail_texture(self.to_ptr(), value) }
			}
			pub fn texture_length(&self) -> f64 {
				unsafe { prop_trail_texture_length(self.to_ptr()) }
			}
			pub fn set_texture_length(&self, value: f64) {
				unsafe { prop_set_trail_texture_length(self.to_ptr(), value) }
			}
			pub fn transparency(&self) -> NumberSequence {
				unsafe { prop_trail_transparency(self.to_ptr()) }
			}
			pub fn set_transparency(&self, value: NumberSequence) {
				unsafe { prop_set_trail_transparency(self.to_ptr(), value) }
			}
			pub fn width_scale(&self) -> NumberSequence {
				unsafe { prop_trail_width_scale(self.to_ptr()) }
			}
			pub fn set_width_scale(&self, value: NumberSequence) {
				unsafe { prop_set_trail_width_scale(self.to_ptr(), value) }
			}
			pub fn fn_clear(&self) {
				unsafe { dyn_trail_clear(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_translator {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn locale_id(&self) -> String {
				unsafe { prop_translator_locale_id(self.to_ptr()) }
			}
			pub fn fn_translate(&self, context: Option<Instance>, text: &str) -> String {
				unsafe { dyn_translator_translate(self.to_ptr(), context, text) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_tween_base {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_cancel(&self) {
				unsafe { dyn_tween_base_cancel(self.to_ptr()) }
			}
			pub fn fn_pause(&self) {
				unsafe { dyn_tween_base_pause(self.to_ptr()) }
			}
			pub fn fn_play(&self) {
				unsafe { dyn_tween_base_play(self.to_ptr()) }
			}
			pub fn on_completed<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_tween_base_completed(self.to_ptr(), Box::new(callback))) }
			}
		}
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
				unsafe { prop_tween_instance(self.to_ptr()) }
			}
			pub fn tween_info(&self) -> TweenInfo {
				unsafe { prop_tween_tween_info(self.to_ptr()) }
			}
		}
		impl From<$name> for TweenBase {
			fn from(value: $name) -> TweenBase {
				TweenBase(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_tween_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_ui_base {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
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
				unsafe { prop_ui_aspect_ratio_constraint_aspect_ratio(self.to_ptr()) }
			}
			pub fn set_aspect_ratio(&self, value: f64) {
				unsafe { prop_set_ui_aspect_ratio_constraint_aspect_ratio(self.to_ptr(), value) }
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
				unsafe { prop_ui_size_constraint_max_size(self.to_ptr()) }
			}
			pub fn set_max_size(&self, value: Vector2) {
				unsafe { prop_set_ui_size_constraint_max_size(self.to_ptr(), value) }
			}
			pub fn min_size(&self) -> Vector2 {
				unsafe { prop_ui_size_constraint_min_size(self.to_ptr()) }
			}
			pub fn set_min_size(&self, value: Vector2) {
				unsafe { prop_set_ui_size_constraint_min_size(self.to_ptr(), value) }
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
				unsafe { prop_ui_text_size_constraint_max_text_size(self.to_ptr()) }
			}
			pub fn set_max_text_size(&self, value: f64) {
				unsafe { prop_set_ui_text_size_constraint_max_text_size(self.to_ptr(), value) }
			}
			pub fn min_text_size(&self) -> f64 {
				unsafe { prop_ui_text_size_constraint_min_text_size(self.to_ptr()) }
			}
			pub fn set_min_text_size(&self, value: f64) {
				unsafe { prop_set_ui_text_size_constraint_min_text_size(self.to_ptr(), value) }
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
				unsafe { prop_ui_corner_corner_radius(self.to_ptr()) }
			}
			pub fn set_corner_radius(&self, value: UDim) {
				unsafe { prop_set_ui_corner_corner_radius(self.to_ptr(), value) }
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
				unsafe { prop_ui_gradient_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: ColorSequence) {
				unsafe { prop_set_ui_gradient_color(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_ui_gradient_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_ui_gradient_enabled(self.to_ptr(), value) }
			}
			pub fn offset(&self) -> Vector2 {
				unsafe { prop_ui_gradient_offset(self.to_ptr()) }
			}
			pub fn set_offset(&self, value: Vector2) {
				unsafe { prop_set_ui_gradient_offset(self.to_ptr(), value) }
			}
			pub fn rotation(&self) -> f64 {
				unsafe { prop_ui_gradient_rotation(self.to_ptr()) }
			}
			pub fn set_rotation(&self, value: f64) {
				unsafe { prop_set_ui_gradient_rotation(self.to_ptr(), value) }
			}
			pub fn transparency(&self) -> NumberSequence {
				unsafe { prop_ui_gradient_transparency(self.to_ptr()) }
			}
			pub fn set_transparency(&self, value: NumberSequence) {
				unsafe { prop_set_ui_gradient_transparency(self.to_ptr(), value) }
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
				unsafe { prop_ui_grid_style_layout_absolute_content_size(self.to_ptr()) }
			}
			pub fn fn_apply_layout(&self) {
				unsafe { dyn_ui_grid_style_layout_apply_layout(self.to_ptr()) }
			}
			pub fn fn_set_custom_sort_function(&self, function: Function) {
				unsafe { dyn_ui_grid_style_layout_set_custom_sort_function(self.to_ptr(), function) }
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
				unsafe { prop_ui_grid_layout_absolute_cell_count(self.to_ptr()) }
			}
			pub fn absolute_cell_size(&self) -> Vector2 {
				unsafe { prop_ui_grid_layout_absolute_cell_size(self.to_ptr()) }
			}
			pub fn cell_padding(&self) -> UDim2 {
				unsafe { prop_ui_grid_layout_cell_padding(self.to_ptr()) }
			}
			pub fn set_cell_padding(&self, value: UDim2) {
				unsafe { prop_set_ui_grid_layout_cell_padding(self.to_ptr(), value) }
			}
			pub fn cell_size(&self) -> UDim2 {
				unsafe { prop_ui_grid_layout_cell_size(self.to_ptr()) }
			}
			pub fn set_cell_size(&self, value: UDim2) {
				unsafe { prop_set_ui_grid_layout_cell_size(self.to_ptr(), value) }
			}
			pub fn fill_direction_max_cells(&self) -> f64 {
				unsafe { prop_ui_grid_layout_fill_direction_max_cells(self.to_ptr()) }
			}
			pub fn set_fill_direction_max_cells(&self, value: f64) {
				unsafe { prop_set_ui_grid_layout_fill_direction_max_cells(self.to_ptr(), value) }
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
				unsafe { prop_ui_list_layout_padding(self.to_ptr()) }
			}
			pub fn set_padding(&self, value: UDim) {
				unsafe { prop_set_ui_list_layout_padding(self.to_ptr(), value) }
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
				unsafe { prop_ui_page_layout_animated(self.to_ptr()) }
			}
			pub fn set_animated(&self, value: bool) {
				unsafe { prop_set_ui_page_layout_animated(self.to_ptr(), value) }
			}
			pub fn circular(&self) -> bool {
				unsafe { prop_ui_page_layout_circular(self.to_ptr()) }
			}
			pub fn set_circular(&self, value: bool) {
				unsafe { prop_set_ui_page_layout_circular(self.to_ptr(), value) }
			}
			pub fn current_page(&self) -> Option<GuiObject> {
				unsafe { prop_ui_page_layout_current_page(self.to_ptr()) }
			}
			pub fn gamepad_input_enabled(&self) -> bool {
				unsafe { prop_ui_page_layout_gamepad_input_enabled(self.to_ptr()) }
			}
			pub fn set_gamepad_input_enabled(&self, value: bool) {
				unsafe { prop_set_ui_page_layout_gamepad_input_enabled(self.to_ptr(), value) }
			}
			pub fn padding(&self) -> UDim {
				unsafe { prop_ui_page_layout_padding(self.to_ptr()) }
			}
			pub fn set_padding(&self, value: UDim) {
				unsafe { prop_set_ui_page_layout_padding(self.to_ptr(), value) }
			}
			pub fn scroll_wheel_input_enabled(&self) -> bool {
				unsafe { prop_ui_page_layout_scroll_wheel_input_enabled(self.to_ptr()) }
			}
			pub fn set_scroll_wheel_input_enabled(&self, value: bool) {
				unsafe { prop_set_ui_page_layout_scroll_wheel_input_enabled(self.to_ptr(), value) }
			}
			pub fn touch_input_enabled(&self) -> bool {
				unsafe { prop_ui_page_layout_touch_input_enabled(self.to_ptr()) }
			}
			pub fn set_touch_input_enabled(&self, value: bool) {
				unsafe { prop_set_ui_page_layout_touch_input_enabled(self.to_ptr(), value) }
			}
			pub fn tween_time(&self) -> f64 {
				unsafe { prop_ui_page_layout_tween_time(self.to_ptr()) }
			}
			pub fn set_tween_time(&self, value: f64) {
				unsafe { prop_set_ui_page_layout_tween_time(self.to_ptr(), value) }
			}
			pub fn fn_jump_to(&self, page: Option<Instance>) {
				unsafe { dyn_ui_page_layout_jump_to(self.to_ptr(), page) }
			}
			pub fn fn_jump_to_index(&self, index: f64) {
				unsafe { dyn_ui_page_layout_jump_to_index(self.to_ptr(), index) }
			}
			pub fn fn_next(&self) {
				unsafe { dyn_ui_page_layout_next(self.to_ptr()) }
			}
			pub fn fn_previous(&self) {
				unsafe { dyn_ui_page_layout_previous(self.to_ptr()) }
			}
			pub fn on_page_enter<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_ui_page_layout_page_enter(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_page_leave<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_ui_page_layout_page_leave(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_stopped<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_ui_page_layout_stopped(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_ui_table_layout_fill_empty_space_columns(self.to_ptr()) }
			}
			pub fn set_fill_empty_space_columns(&self, value: bool) {
				unsafe { prop_set_ui_table_layout_fill_empty_space_columns(self.to_ptr(), value) }
			}
			pub fn fill_empty_space_rows(&self) -> bool {
				unsafe { prop_ui_table_layout_fill_empty_space_rows(self.to_ptr()) }
			}
			pub fn set_fill_empty_space_rows(&self, value: bool) {
				unsafe { prop_set_ui_table_layout_fill_empty_space_rows(self.to_ptr(), value) }
			}
			pub fn padding(&self) -> UDim2 {
				unsafe { prop_ui_table_layout_padding(self.to_ptr()) }
			}
			pub fn set_padding(&self, value: UDim2) {
				unsafe { prop_set_ui_table_layout_padding(self.to_ptr(), value) }
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
				unsafe { prop_ui_padding_padding_bottom(self.to_ptr()) }
			}
			pub fn set_padding_bottom(&self, value: UDim) {
				unsafe { prop_set_ui_padding_padding_bottom(self.to_ptr(), value) }
			}
			pub fn padding_left(&self) -> UDim {
				unsafe { prop_ui_padding_padding_left(self.to_ptr()) }
			}
			pub fn set_padding_left(&self, value: UDim) {
				unsafe { prop_set_ui_padding_padding_left(self.to_ptr(), value) }
			}
			pub fn padding_right(&self) -> UDim {
				unsafe { prop_ui_padding_padding_right(self.to_ptr()) }
			}
			pub fn set_padding_right(&self, value: UDim) {
				unsafe { prop_set_ui_padding_padding_right(self.to_ptr(), value) }
			}
			pub fn padding_top(&self) -> UDim {
				unsafe { prop_ui_padding_padding_top(self.to_ptr()) }
			}
			pub fn set_padding_top(&self, value: UDim) {
				unsafe { prop_set_ui_padding_padding_top(self.to_ptr(), value) }
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
				unsafe { prop_ui_scale_scale(self.to_ptr()) }
			}
			pub fn set_scale(&self, value: f64) {
				unsafe { prop_set_ui_scale_scale(self.to_ptr(), value) }
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
				unsafe { prop_ui_stroke_color(self.to_ptr()) }
			}
			pub fn set_color(&self, value: Color3) {
				unsafe { prop_set_ui_stroke_color(self.to_ptr(), value) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_ui_stroke_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_ui_stroke_enabled(self.to_ptr(), value) }
			}
			pub fn thickness(&self) -> f64 {
				unsafe { prop_ui_stroke_thickness(self.to_ptr()) }
			}
			pub fn set_thickness(&self, value: f64) {
				unsafe { prop_set_ui_stroke_thickness(self.to_ptr(), value) }
			}
			pub fn transparency(&self) -> f64 {
				unsafe { prop_ui_stroke_transparency(self.to_ptr()) }
			}
			pub fn set_transparency(&self, value: f64) {
				unsafe { prop_set_ui_stroke_transparency(self.to_ptr(), value) }
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn gamepad_camera_sensitivity(&self) -> f64 {
				unsafe { prop_user_game_settings_gamepad_camera_sensitivity(self.to_ptr()) }
			}
			pub fn set_gamepad_camera_sensitivity(&self, value: f64) {
				unsafe { prop_set_user_game_settings_gamepad_camera_sensitivity(self.to_ptr(), value) }
			}
			pub fn mouse_sensitivity(&self) -> f64 {
				unsafe { prop_user_game_settings_mouse_sensitivity(self.to_ptr()) }
			}
			pub fn set_mouse_sensitivity(&self, value: f64) {
				unsafe { prop_set_user_game_settings_mouse_sensitivity(self.to_ptr(), value) }
			}
			pub fn rcc_profiler_record_frame_rate(&self) -> f64 {
				unsafe { prop_user_game_settings_rcc_profiler_record_frame_rate(self.to_ptr()) }
			}
			pub fn set_rcc_profiler_record_frame_rate(&self, value: f64) {
				unsafe { prop_set_user_game_settings_rcc_profiler_record_frame_rate(self.to_ptr(), value) }
			}
			pub fn rcc_profiler_record_time_frame(&self) -> f64 {
				unsafe { prop_user_game_settings_rcc_profiler_record_time_frame(self.to_ptr()) }
			}
			pub fn set_rcc_profiler_record_time_frame(&self, value: f64) {
				unsafe { prop_set_user_game_settings_rcc_profiler_record_time_frame(self.to_ptr(), value) }
			}
			pub fn vignette_enabled(&self) -> bool {
				unsafe { prop_user_game_settings_vignette_enabled(self.to_ptr()) }
			}
			pub fn fn_get_camera_y_invert_value(&self) -> f64 {
				unsafe { dyn_user_game_settings_get_camera_y_invert_value(self.to_ptr()) }
			}
			pub fn fn_get_onboarding_completed(&self, onboarding_id: &str) -> bool {
				unsafe { dyn_user_game_settings_get_onboarding_completed(self.to_ptr(), onboarding_id) }
			}
			pub fn fn_in_full_screen(&self) -> bool {
				unsafe { dyn_user_game_settings_in_full_screen(self.to_ptr()) }
			}
			pub fn fn_in_studio_mode(&self) -> bool {
				unsafe { dyn_user_game_settings_in_studio_mode(self.to_ptr()) }
			}
			pub fn fn_set_camera_y_invert_visible(&self) {
				unsafe { dyn_user_game_settings_set_camera_y_invert_visible(self.to_ptr()) }
			}
			pub fn fn_set_gamepad_camera_sensitivity_visible(&self) {
				unsafe { dyn_user_game_settings_set_gamepad_camera_sensitivity_visible(self.to_ptr()) }
			}
			pub fn fn_set_onboarding_completed(&self, onboarding_id: &str) {
				unsafe { dyn_user_game_settings_set_onboarding_completed(self.to_ptr(), onboarding_id) }
			}
			pub fn on_fullscreen_changed<F: 'static + Fn(bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_game_settings_fullscreen_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_studio_mode_changed<F: 'static + Fn(bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_game_settings_studio_mode_changed(self.to_ptr(), Box::new(callback))) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_user_input_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn accelerometer_enabled(&self) -> bool {
				unsafe { prop_user_input_service_accelerometer_enabled(self.to_ptr()) }
			}
			pub fn gamepad_enabled(&self) -> bool {
				unsafe { prop_user_input_service_gamepad_enabled(self.to_ptr()) }
			}
			pub fn gyroscope_enabled(&self) -> bool {
				unsafe { prop_user_input_service_gyroscope_enabled(self.to_ptr()) }
			}
			pub fn keyboard_enabled(&self) -> bool {
				unsafe { prop_user_input_service_keyboard_enabled(self.to_ptr()) }
			}
			pub fn modal_enabled(&self) -> bool {
				unsafe { prop_user_input_service_modal_enabled(self.to_ptr()) }
			}
			pub fn set_modal_enabled(&self, value: bool) {
				unsafe { prop_set_user_input_service_modal_enabled(self.to_ptr(), value) }
			}
			pub fn mouse_delta_sensitivity(&self) -> f64 {
				unsafe { prop_user_input_service_mouse_delta_sensitivity(self.to_ptr()) }
			}
			pub fn set_mouse_delta_sensitivity(&self, value: f64) {
				unsafe { prop_set_user_input_service_mouse_delta_sensitivity(self.to_ptr(), value) }
			}
			pub fn mouse_enabled(&self) -> bool {
				unsafe { prop_user_input_service_mouse_enabled(self.to_ptr()) }
			}
			pub fn mouse_icon_enabled(&self) -> bool {
				unsafe { prop_user_input_service_mouse_icon_enabled(self.to_ptr()) }
			}
			pub fn set_mouse_icon_enabled(&self, value: bool) {
				unsafe { prop_set_user_input_service_mouse_icon_enabled(self.to_ptr(), value) }
			}
			pub fn on_screen_keyboard_position(&self) -> Vector2 {
				unsafe { prop_user_input_service_on_screen_keyboard_position(self.to_ptr()) }
			}
			pub fn on_screen_keyboard_size(&self) -> Vector2 {
				unsafe { prop_user_input_service_on_screen_keyboard_size(self.to_ptr()) }
			}
			pub fn on_screen_keyboard_visible(&self) -> bool {
				unsafe { prop_user_input_service_on_screen_keyboard_visible(self.to_ptr()) }
			}
			pub fn touch_enabled(&self) -> bool {
				unsafe { prop_user_input_service_touch_enabled(self.to_ptr()) }
			}
			pub fn user_head_c_frame(&self) -> CFrame {
				unsafe { prop_user_input_service_user_head_c_frame(self.to_ptr()) }
			}
			pub fn vr_enabled(&self) -> bool {
				unsafe { prop_user_input_service_vr_enabled(self.to_ptr()) }
			}
			pub fn fn_get_connected_gamepads(&self) {
				unsafe { dyn_user_input_service_get_connected_gamepads(self.to_ptr()) }
			}
			pub fn fn_get_device_acceleration(&self) -> Option<InputObject> {
				unsafe { dyn_user_input_service_get_device_acceleration(self.to_ptr()) }
			}
			pub fn fn_get_device_gravity(&self) -> Option<InputObject> {
				unsafe { dyn_user_input_service_get_device_gravity(self.to_ptr()) }
			}
			pub fn fn_get_device_rotation(&self) {
				unsafe { dyn_user_input_service_get_device_rotation(self.to_ptr()) }
			}
			pub fn fn_get_focused_text_box(&self) -> Option<TextBox> {
				unsafe { dyn_user_input_service_get_focused_text_box(self.to_ptr()) }
			}
			pub fn fn_get_keys_pressed(&self) {
				unsafe { dyn_user_input_service_get_keys_pressed(self.to_ptr()) }
			}
			pub fn fn_get_last_input_type(&self) {
				unsafe { dyn_user_input_service_get_last_input_type(self.to_ptr()) }
			}
			pub fn fn_get_mouse_buttons_pressed(&self) {
				unsafe { dyn_user_input_service_get_mouse_buttons_pressed(self.to_ptr()) }
			}
			pub fn fn_get_mouse_delta(&self) -> Vector2 {
				unsafe { dyn_user_input_service_get_mouse_delta(self.to_ptr()) }
			}
			pub fn fn_get_mouse_location(&self) -> Vector2 {
				unsafe { dyn_user_input_service_get_mouse_location(self.to_ptr()) }
			}
			pub fn fn_get_navigation_gamepads(&self) {
				unsafe { dyn_user_input_service_get_navigation_gamepads(self.to_ptr()) }
			}
			pub fn fn_recenter_user_head_c_frame(&self) {
				unsafe { dyn_user_input_service_recenter_user_head_c_frame(self.to_ptr()) }
			}
			pub fn on_device_acceleration_changed<F: 'static + Fn(Option<InputObject>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_device_acceleration_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_device_gravity_changed<F: 'static + Fn(Option<InputObject>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_device_gravity_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_device_rotation_changed<F: 'static + Fn(Option<InputObject>, CFrame)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_device_rotation_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_gamepad_connected<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_gamepad_connected(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_gamepad_disconnected<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_gamepad_disconnected(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_input_began<F: 'static + Fn(Option<InputObject>, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_input_began(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_input_changed<F: 'static + Fn(Option<InputObject>, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_input_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_input_ended<F: 'static + Fn(Option<InputObject>, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_input_ended(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_jump_request<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_jump_request(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_last_input_type_changed<F: 'static + Fn(())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_last_input_type_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_pointer_action<F: 'static + Fn(f64, Vector2, f64, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_pointer_action(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_text_box_focus_released<F: 'static + Fn(Option<TextBox>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_text_box_focus_released(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_text_box_focused<F: 'static + Fn(Option<TextBox>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_text_box_focused(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_ended<F: 'static + Fn(Option<InputObject>, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_touch_ended(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_long_press<F: 'static + Fn((), (), bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_touch_long_press(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_moved<F: 'static + Fn(Option<InputObject>, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_touch_moved(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_pan<F: 'static + Fn((), Vector2, Vector2, (), bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_touch_pan(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_pinch<F: 'static + Fn((), f64, f64, (), bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_touch_pinch(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_rotate<F: 'static + Fn((), f64, f64, (), bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_touch_rotate(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_started<F: 'static + Fn(Option<InputObject>, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_touch_started(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_swipe<F: 'static + Fn((), f64, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_touch_swipe(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_tap<F: 'static + Fn((), bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_touch_tap(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touch_tap_in_world<F: 'static + Fn(Vector2, bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_touch_tap_in_world(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_user_c_frame_changed<F: 'static + Fn((), CFrame)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_user_c_frame_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_window_focus_released<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_window_focus_released(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_window_focused<F: 'static + Fn()>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_user_input_service_window_focused(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_user_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_vr_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn vr_enabled(&self) -> bool {
				unsafe { prop_vr_service_vr_enabled(self.to_ptr()) }
			}
			pub fn fn_recenter_user_head_c_frame(&self) {
				unsafe { dyn_vr_service_recenter_user_head_c_frame(self.to_ptr()) }
			}
			pub fn on_navigation_requested<F: 'static + Fn(CFrame, ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_vr_service_navigation_requested(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_touchpad_mode_changed<F: 'static + Fn((), ())>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_vr_service_touchpad_mode_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_user_c_frame_changed<F: 'static + Fn((), CFrame)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_vr_service_user_c_frame_changed(self.to_ptr(), Box::new(callback))) }
			}
			pub fn on_user_c_frame_enabled<F: 'static + Fn((), bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_vr_service_user_c_frame_enabled(self.to_ptr(), Box::new(callback))) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_value_base {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
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
				unsafe { prop_bool_value_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: bool) {
				unsafe { prop_set_bool_value_value(self.to_ptr(), value) }
			}
			pub fn on_changed<F: 'static + Fn(bool)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_bool_value_changed(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_brick_color_value_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: BrickColor) {
				unsafe { prop_set_brick_color_value_value(self.to_ptr(), value) }
			}
			pub fn on_changed<F: 'static + Fn(BrickColor)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_brick_color_value_changed(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_c_frame_value_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: CFrame) {
				unsafe { prop_set_c_frame_value_value(self.to_ptr(), value) }
			}
			pub fn on_changed<F: 'static + Fn(CFrame)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_c_frame_value_changed(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_color_3_value_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: Color3) {
				unsafe { prop_set_color_3_value_value(self.to_ptr(), value) }
			}
			pub fn on_changed<F: 'static + Fn(Color3)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_color_3_value_changed(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_double_constrained_value_constrained_value(self.to_ptr()) }
			}
			pub fn set_constrained_value(&self, value: f64) {
				unsafe { prop_set_double_constrained_value_constrained_value(self.to_ptr(), value) }
			}
			pub fn max_value(&self) -> f64 {
				unsafe { prop_double_constrained_value_max_value(self.to_ptr()) }
			}
			pub fn set_max_value(&self, value: f64) {
				unsafe { prop_set_double_constrained_value_max_value(self.to_ptr(), value) }
			}
			pub fn min_value(&self) -> f64 {
				unsafe { prop_double_constrained_value_min_value(self.to_ptr()) }
			}
			pub fn set_min_value(&self, value: f64) {
				unsafe { prop_set_double_constrained_value_min_value(self.to_ptr(), value) }
			}
			pub fn value(&self) -> f64 {
				unsafe { prop_double_constrained_value_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: f64) {
				unsafe { prop_set_double_constrained_value_value(self.to_ptr(), value) }
			}
			pub fn on_changed<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_double_constrained_value_changed(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_int_constrained_value_constrained_value(self.to_ptr()) }
			}
			pub fn set_constrained_value(&self, value: f64) {
				unsafe { prop_set_int_constrained_value_constrained_value(self.to_ptr(), value) }
			}
			pub fn max_value(&self) -> f64 {
				unsafe { prop_int_constrained_value_max_value(self.to_ptr()) }
			}
			pub fn set_max_value(&self, value: f64) {
				unsafe { prop_set_int_constrained_value_max_value(self.to_ptr(), value) }
			}
			pub fn min_value(&self) -> f64 {
				unsafe { prop_int_constrained_value_min_value(self.to_ptr()) }
			}
			pub fn set_min_value(&self, value: f64) {
				unsafe { prop_set_int_constrained_value_min_value(self.to_ptr(), value) }
			}
			pub fn value(&self) -> f64 {
				unsafe { prop_int_constrained_value_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: f64) {
				unsafe { prop_set_int_constrained_value_value(self.to_ptr(), value) }
			}
			pub fn on_changed<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_int_constrained_value_changed(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_int_value_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: f64) {
				unsafe { prop_set_int_value_value(self.to_ptr(), value) }
			}
			pub fn on_changed<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_int_value_changed(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_number_value_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: f64) {
				unsafe { prop_set_number_value_value(self.to_ptr(), value) }
			}
			pub fn on_changed<F: 'static + Fn(f64)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_number_value_changed(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_object_value_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: Option<Instance>) {
				unsafe { prop_set_object_value_value(self.to_ptr(), value) }
			}
			pub fn on_changed<F: 'static + Fn(Option<Instance>)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_object_value_changed(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_ray_value_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: Ray) {
				unsafe { prop_set_ray_value_value(self.to_ptr(), value) }
			}
			pub fn on_changed<F: 'static + Fn(Ray)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_ray_value_changed(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_string_value_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: &str) {
				unsafe { prop_set_string_value_value(self.to_ptr(), value) }
			}
			pub fn on_changed<F: 'static + Fn(String)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_string_value_changed(self.to_ptr(), Box::new(callback))) }
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
				unsafe { prop_vector_3_value_value(self.to_ptr()) }
			}
			pub fn set_value(&self, value: Vector3) {
				unsafe { prop_set_vector_3_value_value(self.to_ptr(), value) }
			}
			pub fn on_changed<F: 'static + Fn(Vector3)>(&self, callback: F) -> RbxScriptConnection {
				unsafe { RbxScriptConnection(connect_vector_3_value_changed(self.to_ptr(), Box::new(callback))) }
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
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_value_at_time(&self, time: f64) {
				unsafe { dyn_vector_3_curve_get_value_at_time(self.to_ptr(), time) }
			}
			pub fn fn_x(&self) -> Option<FloatCurve> {
				unsafe { dyn_vector_3_curve_x(self.to_ptr()) }
			}
			pub fn fn_y(&self) -> Option<FloatCurve> {
				unsafe { dyn_vector_3_curve_y(self.to_ptr()) }
			}
			pub fn fn_z(&self) -> Option<FloatCurve> {
				unsafe { dyn_vector_3_curve_z(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_voice_channel {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_voice_chat_internal {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_get_audio_processing_settings(&self) {
				unsafe { dyn_voice_chat_internal_get_audio_processing_settings(self.to_ptr()) }
			}
			pub fn fn_get_mic_devices(&self) {
				unsafe { dyn_voice_chat_internal_get_mic_devices(self.to_ptr()) }
			}
			pub fn fn_get_participants(&self) {
				unsafe { dyn_voice_chat_internal_get_participants(self.to_ptr()) }
			}
			pub fn fn_get_speaker_devices(&self) {
				unsafe { dyn_voice_chat_internal_get_speaker_devices(self.to_ptr()) }
			}
			pub fn fn_get_voice_chat_api_version(&self) -> f64 {
				unsafe { dyn_voice_chat_internal_get_voice_chat_api_version(self.to_ptr()) }
			}
			pub fn fn_get_voice_chat_available(&self) -> f64 {
				unsafe { dyn_voice_chat_internal_get_voice_chat_available(self.to_ptr()) }
			}
			pub fn fn_is_publish_paused(&self) -> bool {
				unsafe { dyn_voice_chat_internal_is_publish_paused(self.to_ptr()) }
			}
			pub fn fn_is_subscribe_paused(&self, user_id: f64) -> bool {
				unsafe { dyn_voice_chat_internal_is_subscribe_paused(self.to_ptr(), user_id) }
			}
			pub fn fn_join_by_group_id(&self, group_id: &str, is_mic_muted: bool) -> bool {
				unsafe { dyn_voice_chat_internal_join_by_group_id(self.to_ptr(), group_id, is_mic_muted) }
			}
			pub fn fn_join_by_group_id_token(&self, group_id: &str, is_mic_muted: bool) -> bool {
				unsafe { dyn_voice_chat_internal_join_by_group_id_token(self.to_ptr(), group_id, is_mic_muted) }
			}
			pub fn fn_leave(&self) {
				unsafe { dyn_voice_chat_internal_leave(self.to_ptr()) }
			}
			pub fn fn_publish_pause(&self, paused: bool) -> bool {
				unsafe { dyn_voice_chat_internal_publish_pause(self.to_ptr(), paused) }
			}
			pub fn fn_set_mic_device(&self, mic_device_name: &str, mic_device_guid: &str) {
				unsafe { dyn_voice_chat_internal_set_mic_device(self.to_ptr(), mic_device_name, mic_device_guid) }
			}
			pub fn fn_set_speaker_device(&self, speaker_device_name: &str, speaker_device_guid: &str) {
				unsafe { dyn_voice_chat_internal_set_speaker_device(self.to_ptr(), speaker_device_name, speaker_device_guid) }
			}
			pub fn fn_subscribe_pause(&self, user_id: f64, paused: bool) -> bool {
				unsafe { dyn_voice_chat_internal_subscribe_pause(self.to_ptr(), user_id, paused) }
			}
			pub fn fn_subscribe_pause_all(&self, paused: bool) -> bool {
				unsafe { dyn_voice_chat_internal_subscribe_pause_all(self.to_ptr(), paused) }
			}
			pub fn fn_is_voice_enabled_for_user_id_async(&self, user_id: f64) -> bool {
				unsafe { dyn_voice_chat_internal_is_voice_enabled_for_user_id_async(self.to_ptr(), user_id) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_voice_chat_service {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn fn_is_voice_enabled_for_user_id_async(&self, user_id: f64) -> bool {
				unsafe { dyn_voice_chat_service_is_voice_enabled_for_user_id_async(self.to_ptr(), user_id) }
			}
			pub fn instance() -> $name {
				$name(DataModel::instance().fn_get_service(stringify!($name)).unwrap().to_ptr())
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_voice_source {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn user_id(&self) -> f64 {
				unsafe { prop_voice_source_user_id(self.to_ptr()) }
			}
		}
		impl From<$name> for Instance {
			fn from(value: $name) -> Instance {
				Instance(value.to_ptr())
			}
		}
	}
}
macro_rules! impl_weld_constraint {
	($name:ident) => {
		impl_instance_exclusive!($name);
		impl $name {
			pub fn active(&self) -> bool {
				unsafe { prop_weld_constraint_active(self.to_ptr()) }
			}
			pub fn enabled(&self) -> bool {
				unsafe { prop_weld_constraint_enabled(self.to_ptr()) }
			}
			pub fn set_enabled(&self, value: bool) {
				unsafe { prop_set_weld_constraint_enabled(self.to_ptr(), value) }
			}
			pub fn part_0(&self) -> Option<BasePart> {
				unsafe { prop_weld_constraint_part_0(self.to_ptr()) }
			}
			pub fn set_part_0(&self, value: Option<BasePart>) {
				unsafe { prop_set_weld_constraint_part_0(self.to_ptr(), value) }
			}
			pub fn part_1(&self) -> Option<BasePart> {
				unsafe { prop_weld_constraint_part_1(self.to_ptr()) }
			}
			pub fn set_part_1(&self, value: Option<BasePart>) {
				unsafe { prop_set_weld_constraint_part_1(self.to_ptr(), value) }
			}
		}
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
impl_analytics_service!(AnalyticsService);
impl_animation!(Animation);
impl_animation_clip!(AnimationClip);
impl_curve_animation!(CurveAnimation);
impl_keyframe_sequence!(KeyframeSequence);
impl_animation_clip_provider!(AnimationClipProvider);
impl_animation_controller!(AnimationController);
impl_animation_from_video_creator_service!(AnimationFromVideoCreatorService);
impl_animation_from_video_creator_studio_service!(AnimationFromVideoCreatorStudioService);
impl_animation_rig_data!(AnimationRigData);
impl_animation_stream_track!(AnimationStreamTrack);
impl_animation_track!(AnimationTrack);
impl_animator!(Animator);
impl_app_update_service!(AppUpdateService);
impl_asset_counter_service!(AssetCounterService);
impl_asset_delivery_proxy!(AssetDeliveryProxy);
impl_asset_import_service!(AssetImportService);
impl_asset_manager_service!(AssetManagerService);
impl_asset_service!(AssetService);
impl_atmosphere!(Atmosphere);
impl_attachment!(Attachment);
impl_bone!(Bone);
impl_avatar_editor_service!(AvatarEditorService);
impl_avatar_import_service!(AvatarImportService);
impl_backpack!(Backpack);
impl_backpack_item!(BackpackItem);
impl_tool!(Tool);
impl_badge_service!(BadgeService);
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
impl_chat!(Chat);
impl_click_detector!(ClickDetector);
impl_clouds!(Clouds);
impl_collection_service!(CollectionService);
impl_command_instance!(CommandInstance);
impl_command_service!(CommandService);
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
impl_content_provider!(ContentProvider);
impl_context_action_service!(ContextActionService);
impl_controller!(Controller);
impl_humanoid_controller!(HumanoidController);
impl_skateboard_controller!(SkateboardController);
impl_vehicle_controller!(VehicleController);
impl_controller_service!(ControllerService);
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
impl_data_store_service!(DataStoreService);
impl_data_store_set_options!(DataStoreSetOptions);
impl_debris!(Debris);
impl_dialog!(Dialog);
impl_dialog_choice!(DialogChoice);
impl_dragger!(Dragger);
impl_dragger_service!(DraggerService);
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
impl_game_pass_service!(GamePassService);
impl_get_text_bounds_params!(GetTextBoundsParams);
impl_global_data_store!(GlobalDataStore);
impl_data_store!(DataStore);
impl_ordered_data_store!(OrderedDataStore);
impl_group_service!(GroupService);
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
impl_gui_service!(GuiService);
impl_haptic_service!(HapticService);
impl_highlight!(Highlight);
impl_http_service!(HttpService);
impl_humanoid!(Humanoid);
impl_humanoid_description!(HumanoidDescription);
impl_importer_base_settings!(ImporterBaseSettings);
impl_importer_group_settings!(ImporterGroupSettings);
impl_importer_material_settings!(ImporterMaterialSettings);
impl_importer_mesh_settings!(ImporterMeshSettings);
impl_importer_root_settings!(ImporterRootSettings);
impl_input_object!(InputObject);
impl_insert_service!(InsertService);
impl_joint_instance!(JointInstance);
impl_dynamic_rotate!(DynamicRotate);
impl_glue!(Glue);
impl_motor!(Motor);
impl_motor_6_d!(Motor6D);
impl_velocity_motor!(VelocityMotor);
impl_joints_service!(JointsService);
impl_keyframe!(Keyframe);
impl_keyframe_marker!(KeyframeMarker);
impl_keyframe_sequence_provider!(KeyframeSequenceProvider);
impl_light!(Light);
impl_point_light!(PointLight);
impl_spot_light!(SpotLight);
impl_surface_light!(SurfaceLight);
impl_lighting!(Lighting);
impl_localization_service!(LocalizationService);
impl_localization_table!(LocalizationTable);
impl_lod_data_entity!(LodDataEntity);
impl_log_service!(LogService);
impl_lua_source_container!(LuaSourceContainer);
impl_base_script!(BaseScript);
impl_script!(Script);
impl_local_script!(LocalScript);
impl_module_script!(ModuleScript);
impl_marker_curve!(MarkerCurve);
impl_marketplace_service!(MarketplaceService);
impl_material_service!(MaterialService);
impl_material_variant!(MaterialVariant);
impl_memory_store_queue!(MemoryStoreQueue);
impl_memory_store_service!(MemoryStoreService);
impl_memory_store_sorted_map!(MemoryStoreSortedMap);
impl_messaging_service!(MessagingService);
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
impl_pathfinding_service!(PathfindingService);
impl_physics_service!(PhysicsService);
impl_player!(Player);
impl_player_scripts!(PlayerScripts);
impl_players!(Players);
impl_policy_service!(PolicyService);
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
impl_proximity_prompt_service!(ProximityPromptService);
impl_remote_event!(RemoteEvent);
impl_remote_function!(RemoteFunction);
impl_replicated_first!(ReplicatedFirst);
impl_replicated_storage!(ReplicatedStorage);
impl_rotation_curve!(RotationCurve);
impl_run_service!(RunService);
impl_screenshot_hud!(ScreenshotHud);
impl_script_context!(ScriptContext);
impl_server_script_service!(ServerScriptService);
impl_server_storage!(ServerStorage);
impl_service_provider!(ServiceProvider);
impl_data_model!(DataModel);
impl_generic_settings!(GenericSettings);
impl_user_settings!(UserSettings);
impl_sky!(Sky);
impl_smoke!(Smoke);
impl_social_service!(SocialService);
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
impl_sound_service!(SoundService);
impl_sparkles!(Sparkles);
impl_speaker!(Speaker);
impl_starter_gear!(StarterGear);
impl_starter_pack!(StarterPack);
impl_starter_player!(StarterPlayer);
impl_starter_player_scripts!(StarterPlayerScripts);
impl_starter_character_scripts!(StarterCharacterScripts);
impl_stats!(Stats);
impl_surface_appearance!(SurfaceAppearance);
impl_team!(Team);
impl_teams!(Teams);
impl_teleport_async_result!(TeleportAsyncResult);
impl_teleport_options!(TeleportOptions);
impl_teleport_service!(TeleportService);
impl_temporary_cage_mesh_provider!(TemporaryCageMeshProvider);
impl_temporary_script_service!(TemporaryScriptService);
impl_terrain_detail!(TerrainDetail);
impl_terrain_region!(TerrainRegion);
impl_text_box_service!(TextBoxService);
impl_text_channel!(TextChannel);
impl_text_chat_command!(TextChatCommand);
impl_text_chat_configurations!(TextChatConfigurations);
impl_chat_input_bar_configuration!(ChatInputBarConfiguration);
impl_chat_window_configuration!(ChatWindowConfiguration);
impl_text_chat_message!(TextChatMessage);
impl_text_chat_message_properties!(TextChatMessageProperties);
impl_text_chat_service!(TextChatService);
impl_text_filter_result!(TextFilterResult);
impl_text_service!(TextService);
impl_text_source!(TextSource);
impl_touch_transmitter!(TouchTransmitter);
impl_trail!(Trail);
impl_translator!(Translator);
impl_tween_base!(TweenBase);
impl_tween!(Tween);
impl_tween_service!(TweenService);
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
impl_user_input_service!(UserInputService);
impl_user_service!(UserService);
impl_vr_service!(VRService);
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
impl_voice_chat_internal!(VoiceChatInternal);
impl_voice_chat_service!(VoiceChatService);
impl_voice_source!(VoiceSource);
impl_weld_constraint!(WeldConstraint);
creatable!(Accoutrement Accessory Hat AnalyticsService Animation CurveAnimation KeyframeSequence AnimationController AnimationRigData Animator Atmosphere Attachment Bone Backpack Tool WrapLayer WrapTarget Beam BindableEvent BindableFunction BodyAngularVelocity BodyForce BodyGyro BodyPosition BodyThrust BodyVelocity RocketPropulsion Camera BodyColors CharacterMesh Pants Shirt ShirtGraphic ClickDetector Clouds AlignOrientation AlignPosition AngularVelocity BallSocketConstraint HingeConstraint LineForce LinearVelocity PlaneConstraint Plane RigidConstraint RodConstraint RopeConstraint CylindricalConstraint PrismaticConstraint SpringConstraint Torque TorsionSpringConstraint UniversalConstraint VectorForce HumanoidController SkateboardController VehicleController BlockMesh CylinderMesh FileMesh SpecialMesh DataStoreIncrementOptions DataStoreOptions DataStoreSetOptions Dialog DialogChoice Dragger EulerRotationCurve Explosion Decal Texture Fire FloatCurve Folder ForceField GetTextBoundsParams CanvasGroup Frame ImageButton TextButton ImageLabel TextLabel ScrollingFrame TextBox VideoFrame ViewportFrame BillboardGui ScreenGui SurfaceGui FloorWire SelectionBox BoxHandleAdornment ConeHandleAdornment CylinderHandleAdornment ImageHandleAdornment LineHandleAdornment SphereHandleAdornment ParabolaAdornment SelectionSphere ArcHandles Handles SurfaceSelection SelectionPartLasso SelectionPointLasso Highlight Humanoid HumanoidDescription Glue Motor Motor6D VelocityMotor Keyframe KeyframeMarker PointLight SpotLight SurfaceLight LocalizationTable Script LocalScript ModuleScript MarkerCurve MaterialVariant MemoryStoreService NoCollisionConstraint CornerWedgePart Part Seat SkateboardPlatform SpawnLocation WedgePart MeshPart PartOperation NegateOperation UnionOperation TrussPart VehicleSeat Model Actor WorldModel ParticleEmitter PathfindingLink PathfindingModifier Player NumberPose Pose BloomEffect BlurEffect ColorCorrectionEffect DepthOfFieldEffect SunRaysEffect ProximityPrompt ProximityPromptService RemoteEvent RemoteFunction RotationCurve Sky Smoke Sound ChorusSoundEffect CompressorSoundEffect ChannelSelectorSoundEffect DistortionSoundEffect EchoSoundEffect EqualizerSoundEffect FlangeSoundEffect PitchShiftSoundEffect ReverbSoundEffect TremoloSoundEffect SoundGroup Sparkles Speaker StarterGear SurfaceAppearance Team TeleportOptions TerrainDetail TerrainRegion TextChannel TextChatCommand TextChatMessageProperties Trail Tween UIAspectRatioConstraint UISizeConstraint UITextSizeConstraint UICorner UIGradient UIGridLayout UIListLayout UIPageLayout UITableLayout UIPadding UIScale UIStroke BoolValue BrickColorValue CFrameValue Color3Value DoubleConstrainedValue IntConstrainedValue IntValue NumberValue ObjectValue RayValue StringValue Vector3Value Vector3Curve VoiceChannel WeldConstraint);
