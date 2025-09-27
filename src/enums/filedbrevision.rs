use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u32)]
#[derive(Debug, Copy, Clone, IntoPrimitive, TryFromPrimitive, PartialEq)]
pub enum FileDBRevision {
    Unknown,
    LBP1Or2 = 256,
    LBPV = 936,
    LBP3 = 21496064,
}