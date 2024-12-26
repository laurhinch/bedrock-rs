use crate::version::v662::types::ItemStackResponseSlotInfo;
use bedrockrs_macros::ProtoCodec;
use crate::v766::types::FullContainerName;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemStackResponseContainerInfo {
    pub full_container_name: FullContainerName,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub slots: Vec<ItemStackResponseSlotInfo>
}