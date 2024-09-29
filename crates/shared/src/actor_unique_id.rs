use std::io::Cursor;
use varint_rs::{VarintReader, VarintWriter};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ActorUniqueID(pub i64);

// ProtoCodec
impl ProtoCodec for ActorUniqueID {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        Ok(stream.write_i64_varint(self.0)?)
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(Self(stream.read_i64_varint()?))
    }
}
