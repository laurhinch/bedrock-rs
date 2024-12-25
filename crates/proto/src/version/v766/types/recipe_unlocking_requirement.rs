use bedrockrs_macros::ProtoCodec;
use crate::version::v766::enums::RecipeUnlockingContext;

#[derive(ProtoCodec, Clone, Debug)]
pub struct RecipeUnlockingRequirement {
    pub unlocking_context: RecipeUnlockingContext
}