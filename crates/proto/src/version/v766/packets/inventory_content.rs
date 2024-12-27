use crate::version::v662::types::NetworkItemStackDescriptor;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v766::types::FullContainerName;

#[gamepacket(id = 49)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct InventoryContentPacket {
    #[endianness(var)]
    pub inventory_id: u32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub slots: Vec<NetworkItemStackDescriptor>,
    pub full_container_name: FullContainerName,
    pub storage_item: NetworkItemStackDescriptor,
}