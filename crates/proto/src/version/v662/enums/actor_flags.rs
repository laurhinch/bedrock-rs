use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum ActorFlags {
    Unknown = -1,
    OnFire = 0,
    Sneaking = 1,
    Riding = 2,
    Sprinting = 3,
    UsingItem = 4,
    Invisible = 5,
    Tempted = 6,
    InLove = 7,
    Saddled = 8,
    Powered = 9,
    Ignited = 10,
    Baby = 11,
    Converting = 12,
    Critical = 13,
    CanShowName = 14,
    AlwaysShowName = 15,
    NoAI = 16,
    Silent = 17,
    WallClimbing = 18,
    CanClimb = 19,
    CanSwim = 20,
    CanFly = 21,
    CanWalk = 22,
    Resting = 23,
    Sitting = 24,
    Angry = 25,
    Interested = 26,
    Charged = 27,
    Tamed = 28,
    Orphaned = 29,
    Leashed = 30,
    Sheared = 31,
    Gliding = 32,
    Elder = 33,
    Moving = 34,
    Breathing = 35,
    Chested = 36,
    Stackable = 37,
    ShowBottom = 38,
    Standing = 39,
    Shaking = 40,
    Idling = 41,
    Casting = 42,
    Charging = 43,
    WasdControlled = 44,
    CanPowerJump = 45,
    CanDash = 46,
    Lingering = 47,
    HasCollision = 48,
    HasGravity = 49,
    FireImmune = 50,
    Dancing = 51,
    Enchanted = 52,
    ReturnTrident = 53,
    ContainerIsPrivate = 54,
    IsTransforming = 55,
    DamageNearbyMobs = 56,
    Swimming = 57,
    Bribed = 58,
    IsPregnant = 59,
    LayingEgg = 60,
    PassengerCanPick = 61,
    TransitionSitting = 62,
    Eating = 63,
    LayingDown = 64,
    Sneezing = 65,
    Trusting = 66,
    Rolling = 67,
    Scared  = 68,
    InScaffolding = 69,
    OverScaffolding = 70,
    DescendThroughBlock = 71,
    Blocking = 72,
    TransitionBlocking = 73,
    BlockedUsingShield = 74,
    BlockedUsingDamagedShield = 75,
    Sleeping = 76,
    WantsToWake = 77,
    TradeInterest = 78,
    DoorBreaker = 79,
    BreakingObstruction = 80,
    DoorOpener = 81,
    IsIllagerCaptain = 82,
    Stunned = 83,
    Roaring = 84,
    DelayedAttack = 85,
    IsAvoidingMobs = 86,
    IsAvoidingBlock = 87,
    FacingTargetToRangeAttack = 88,
    HiddenWhenInvisible = 89,
    IsInUi = 90,
    Stalking = 91,
    Emoting = 92,
    Celebrating = 93,
    Admiring = 94,
    CelebratingSpecial = 95,
    OutOfControl = 96,
    RamAttack = 97,
    PlayingDead = 98,
    InAscendableBlock = 99,
    OverDescendableBlock = 100,
    Croaking = 101,
    EatMob = 102,
    JumpGoalJump = 103,
    Emerging = 104,
    Sniffing = 105,
    Digging = 106,
    SonicBoom = 107,
    HasDashCooldown = 108,
    PushTowardsClosestSpace = 109,
    Scenting = 110,
    Rising = 111,
    FeelingHappy = 112,
    Searching = 113,
    Crawling = 114,
    TimerFlag1 = 115,
    TimerFlag2 = 116,
    TimerFlag3 = 117,
    Count = 118,
}