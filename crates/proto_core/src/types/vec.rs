use std::io::Cursor;

use crate::byteorder::{ProtoCodecBE, ProtoCodecLE, ProtoCodecVAR};
use crate::error::ProtoCodecError;
use crate::ProtoCodec;
use bedrockrs_core::{Vec2, Vec3};

macro_rules! impl_proto_vec2 {
    ($name:ident) => {
        impl<T: $name> $name for Vec2<T> {
            fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>
            where
                Self: Sized,
            {
                T::proto_serialize(&self.x, stream)?;
                T::proto_serialize(&self.y, stream)?;

                Ok(())
            }

            fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
            where
                Self: Sized,
            {
                Ok(Self {
                    x: T::proto_deserialize(stream)?,
                    y: T::proto_deserialize(stream)?,
                })
            }
        }
    };
}

macro_rules! impl_proto_vec3 {
    ($name:ident) => {
        impl<T: $name> $name for Vec3<T> {
            fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>
            where
                Self: Sized,
            {
                T::proto_serialize(&self.x, stream)?;
                T::proto_serialize(&self.y, stream)?;
                T::proto_serialize(&self.z, stream)?;

                Ok(())
            }

            fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
            where
                Self: Sized,
            {
                Ok(Self {
                    x: T::proto_deserialize(stream)?,
                    y: T::proto_deserialize(stream)?,
                    z: T::proto_deserialize(stream)?,
                })
            }
        }
    };
}

impl_proto_vec2!(ProtoCodec);
impl_proto_vec2!(ProtoCodecLE);
impl_proto_vec2!(ProtoCodecBE);
impl_proto_vec2!(ProtoCodecVAR);

impl_proto_vec3!(ProtoCodec);
impl_proto_vec3!(ProtoCodecLE);
impl_proto_vec3!(ProtoCodecBE);
impl_proto_vec3!(ProtoCodecVAR);
