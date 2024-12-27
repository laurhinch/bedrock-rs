use crate::version::v662::enums::ContainerID;
use crate::version::v662::types::NetworkItemStackDescriptor;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v766::types::FullContainerName;

#[gamepacket(id = 50)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct InventorySlotPacket {
    pub container_id: ContainerID,
    #[endianness(var)]
    pub slot: u32,
    pub full_container_name: FullContainerName,
    pub storage_item: NetworkItemStackDescriptor,
    pub item: NetworkItemStackDescriptor,
}