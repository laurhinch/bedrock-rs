use crate::version::v662::enums::{CodeBuilderStorageCategory, CodeBuilderStorageOperation};
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v766::enums::CodeBuilderExecutionState;

#[gamepacket(id = 178)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CodeBuilderSourcePacket {
    pub operation: CodeBuilderStorageOperation,
    pub category: CodeBuilderStorageCategory,
    pub code_status: CodeBuilderExecutionState,
}