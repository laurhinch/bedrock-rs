use bedrockrs_macros::ProtoCodec;
use crate::version::v662::enums::ActorLinkType;
use crate::version::v662::types::ActorUniqueID;

#[derive(ProtoCodec)]
pub struct ActorLink {
    pub actor_unique_id_a: ActorUniqueID,
    pub actor_unique_id_b: ActorUniqueID, 
    pub link_type: ActorLinkType,
    pub immediate: bool,
    /// Whether the link was changed by the passenger
    pub passenger_initiated: bool, 
}
