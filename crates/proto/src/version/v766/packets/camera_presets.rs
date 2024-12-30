use crate::version::v766::types::CameraPresets;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 198)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CameraPresetsPacket {
    pub camera_presets: CameraPresets,
}