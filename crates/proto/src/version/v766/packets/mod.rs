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
