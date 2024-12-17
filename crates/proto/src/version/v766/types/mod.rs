macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(actor_link);
export!(base_description);
export!(camera_instruction);
export!(camera_preset);
export!(camera_presets);
