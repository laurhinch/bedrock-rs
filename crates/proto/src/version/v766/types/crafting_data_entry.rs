use bedrockrs_macros::ProtoCodec;
use crate::version::v766::enums::CraftingDataEntryType;

#[derive(ProtoCodec, Clone, Debug)]
pub struct CraftingDataEntry {
    pub crafting_type: CraftingDataEntryType,
}