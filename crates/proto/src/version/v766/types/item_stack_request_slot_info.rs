use bedrockrs_macros::ProtoCodec;
use crate::v766::types::FullContainerName;
use crate::version::v662::enums::ContainerName;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemStackRequestSlotInfo {
    pub full_container_name: FullContainerName,
    pub slot: i8,
    #[endianness(var)]
    pub raw_id: i32,
}