#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum P {
    #[doc = "Analog input"]
    ANALOG = 0x0,
    #[doc = "Push-pull output"]
    PUSH_PULL_OUTPUT = 0x01,
    #[doc = "Open-drain output"]
    OPEN_DRAIN_OUTPUT = 0x02,
    #[doc = "High-impedance input"]
    HIGH_IMPEDANCE_INPUT = 0x03,
}
impl P {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P {
    #[inline(always)]
    fn from(val: u8) -> P {
        P::from_bits(val)
    }
}
impl From<P> for u8 {
    #[inline(always)]
    fn from(val: P) -> u8 {
        P::to_bits(val)
    }
}
