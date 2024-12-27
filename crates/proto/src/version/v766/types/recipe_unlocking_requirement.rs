use crate::version::v766::enums::RecipeUnlockingContext;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct RecipeUnlockingRequirement {
    pub unlocking_context: RecipeUnlockingContext,
}