use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct CurrentStructureFeaturePacket {
    pub current_structure_feature: String,
}