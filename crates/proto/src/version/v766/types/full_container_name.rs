use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::ContainerName;

#[derive(ProtoCodec, Clone, Debug)]
pub struct FullContainerName {
    pub container_name: ContainerName,
    #[endianness(le)]
    pub dynamic_id: Option<u32>
}