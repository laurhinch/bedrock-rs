use crate::helper::ProtoHelper;
use crate::version::v729::gamepackets::GamePackets;

pub struct ProtoHelperV766;

impl ProtoHelper for ProtoHelperV766 {
    type GamePacketType = GamePackets;
}
