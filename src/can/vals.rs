#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Afm {
    #[doc = "Dual filter"]
    DUAL = 0x0,
    #[doc = "Single filter"]
    SINGLE = 0x01,
}
impl Afm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Afm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Afm {
    #[inline(always)]
    fn from(val: u8) -> Afm {
        Afm::from_bits(val)
    }
}
impl From<Afm> for u8 {
    #[inline(always)]
    fn from(val: Afm) -> u8 {
        Afm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Alc {
    #[doc = "Arbitration lost in ID28/10"]
    ID28_10 = 0x0,
    #[doc = "Arbitration lost in ID28/9"]
    ID28_9 = 0x01,
    #[doc = "Arbitration lost in ID28/8"]
    ID28_8 = 0x02,
    #[doc = "Arbitration lost in ID28/7"]
    ID28_7 = 0x03,
    #[doc = "Arbitration lost in ID28/6"]
    ID28_6 = 0x04,
    #[doc = "Arbitration lost in ID28/5"]
    ID28_5 = 0x05,
    #[doc = "Arbitration lost in ID28/4"]
    ID28_4 = 0x06,
    #[doc = "Arbitration lost in ID28/3"]
    ID28_3 = 0x07,
    #[doc = "Arbitration lost in ID28/2"]
    ID28_2 = 0x08,
    #[doc = "Arbitration lost in ID28/1"]
    ID28_1 = 0x09,
    #[doc = "Arbitration lost in ID28/0"]
    ID28_0 = 0x0a,
    #[doc = "Arbitration lost in SRR/RTR"]
    SRR_RTR = 0x0b,
    #[doc = "Arbitration lost in IDE bit"]
    IDE = 0x0c,
    #[doc = "Arbitration lost in ID17^24"]
    ID_17_24 = 0x0d,
    #[doc = "Arbitration lost in ID16^24"]
    ID_16_14 = 0x0e,
    #[doc = "Arbitration lost in ID15^24"]
    ID_15_24 = 0x0f,
    #[doc = "Arbitration lost in ID14^24"]
    ID_14_24 = 0x10,
    #[doc = "Arbitration lost in ID13^24"]
    ID_13_24 = 0x11,
    #[doc = "Arbitration lost in ID12^24"]
    ID_12_24 = 0x12,
    #[doc = "Arbitration lost in ID11^24"]
    ID_11_24 = 0x13,
    #[doc = "Arbitration lost in ID10^24"]
    ID_10_24 = 0x14,
    #[doc = "Arbitration lost in ID9^24"]
    ID_9_24 = 0x15,
    #[doc = "Arbitration lost in ID8^24"]
    ID_8_24 = 0x16,
    #[doc = "Arbitration lost in ID7^24"]
    ID_7_24 = 0x17,
    #[doc = "Arbitration lost in ID6^24"]
    ID_6_24 = 0x18,
    #[doc = "Arbitration lost in ID5^24"]
    ID_5_24 = 0x19,
    #[doc = "Arbitration lost in ID4^24"]
    ID_4_24 = 0x1a,
    #[doc = "Arbitration lost in ID3^24"]
    ID_3_24 = 0x1b,
    #[doc = "Arbitration lost in ID2^24"]
    ID_2_24 = 0x1c,
    #[doc = "Arbitration lost in ID1^24"]
    ID_1_24 = 0x1d,
    #[doc = "Arbitration lost in ID0^24"]
    ID_0_24 = 0x1e,
    #[doc = "Arbitration lost in RTR"]
    RTR = 0x1f,
}
impl Alc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Alc {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Alc {
    #[inline(always)]
    fn from(val: u8) -> Alc {
        Alc::from_bits(val)
    }
}
impl From<Alc> for u8 {
    #[inline(always)]
    fn from(val: Alc) -> u8 {
        Alc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Edir {
    #[doc = "Transmission"]
    TX = 0x0,
    #[doc = "Reception"]
    RX = 0x01,
}
impl Edir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edir {
    #[inline(always)]
    fn from(val: u8) -> Edir {
        Edir::from_bits(val)
    }
}
impl From<Edir> for u8 {
    #[inline(always)]
    fn from(val: Edir) -> u8 {
        Edir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sam {
    #[doc = "One sample"]
    ONE = 0x0,
    #[doc = "Three samples"]
    THREE = 0x01,
}
impl Sam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sam {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sam {
    #[inline(always)]
    fn from(val: u8) -> Sam {
        Sam::from_bits(val)
    }
}
impl From<Sam> for u8 {
    #[inline(always)]
    fn from(val: Sam) -> u8 {
        Sam::to_bits(val)
    }
}
