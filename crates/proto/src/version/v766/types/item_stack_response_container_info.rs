use crate::v766::types::FullContainerName;
use crate::version::v662::types::ItemStackResponseSlotInfo;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemStackResponseContainerInfo {
    pub full_container_name: FullContainerName,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub slots: Vec<ItemStackResponseSlotInfo>,
}