#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Datawidth {
    #[doc = "32 bits"]
    BITS32 = 0x0,
    #[doc = "8 bits"]
    BITS8 = 0x01,
}
impl Datawidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datawidth {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datawidth {
    #[inline(always)]
    fn from(val: u8) -> Datawidth {
        Datawidth::from_bits(val)
    }
}
impl From<Datawidth> for u8 {
    #[inline(always)]
    fn from(val: Datawidth) -> u8 {
        Datawidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Polysel {
    #[doc = "CRC-16-CCITT"]
    CRC_16_CCITT = 0x0,
    #[doc = "CRC-16-IBM"]
    CRC_16_IBM = 0x01,
    #[doc = "CRC-8-Dallas/Maxim"]
    CRC_8_DALLAS_MAXIM = 0x02,
    _RESERVED_3 = 0x03,
}
impl Polysel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Polysel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Polysel {
    #[inline(always)]
    fn from(val: u8) -> Polysel {
        Polysel::from_bits(val)
    }
}
impl From<Polysel> for u8 {
    #[inline(always)]
    fn from(val: Polysel) -> u8 {
        Polysel::to_bits(val)
    }
}
