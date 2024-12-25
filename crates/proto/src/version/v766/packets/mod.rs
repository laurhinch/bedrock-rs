macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(add_actor);
export!(add_player);
export!(award_achievement);
export!(boss_event);
export!(camera_aim_assist);
export!(camera_aim_assist_presets);
export!(camera_instruction);
export!(camera_presets);
export!(change_dimension);
export!(clientbound_close_form);
export!(clientbound_debug_renderer);
export!(clientbound_map_item_data);
export!(code_builder_source);
export!(container_close);
export!(container_registry_cleanup);
export!(correct_player_move_prediction);
export!(current_structure_feature);
