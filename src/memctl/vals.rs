#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Flashlock(pub u32);
impl Flashlock {
    #[doc = "Reset value"]
    pub const RESET: Self = Self(0x0);
    #[doc = "Allow write and erase operations"]
    pub const ALLOW_WRITE_ERASE_FLASH: Self = Self(0x43df_140a);
    #[doc = "Allow write to INFO2.SWDFUSE register"]
    pub const ALLOW_WRITE_INFO2SWDFUSE: Self = Self(0x79b4_f762);
    #[doc = "Allow write to MEMCTRL register"]
    pub const ALLOW_WRITE_MEMCTL: Self = Self(0xd513_b490);
}
impl Flashlock {
    pub const fn from_bits(val: u32) -> Flashlock {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Flashlock {
    #[inline(always)]
    fn from(val: u32) -> Flashlock {
        Flashlock::from_bits(val)
    }
}
impl From<Flashlock> for u32 {
    #[inline(always)]
    fn from(val: Flashlock) -> u32 {
        Flashlock::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key(pub u32);
impl Key {
    #[doc = "Mass page erase"]
    pub const MASS_PAGE_ERASE: Self = Self(0x09ee_76c9);
    #[doc = "INFO-3 Erase"]
    pub const INFO_3_ERASE: Self = Self(0x1266_ff45);
    #[doc = "Page erase"]
    pub const PAGE_ERASE: Self = Self(0x8c79_9ca7);
}
impl Key {
    pub const fn from_bits(val: u32) -> Key {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Key {
    #[inline(always)]
    fn from(val: u32) -> Key {
        Key::from_bits(val)
    }
}
impl From<Key> for u32 {
    #[inline(always)]
    fn from(val: Key) -> u32 {
        Key::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mclkdiv {
    #[doc = "/1"]
    DIV1 = 0x0,
    #[doc = "/2"]
    DIV2 = 0x01,
    #[doc = "/3"]
    DIV3 = 0x02,
    #[doc = "/4"]
    DIV4 = 0x03,
    #[doc = "/5"]
    DIV5 = 0x04,
    #[doc = "/6"]
    DIV6 = 0x05,
    #[doc = "/7"]
    DIV7 = 0x06,
    #[doc = "/8"]
    DIV8 = 0x07,
    #[doc = "/9"]
    DIV9 = 0x08,
    #[doc = "/10"]
    DIV10 = 0x09,
    #[doc = "/11"]
    DIV11 = 0x0a,
    #[doc = "/12"]
    DIV12 = 0x0b,
    #[doc = "/13"]
    DIV13 = 0x0c,
    #[doc = "/14"]
    DIV14 = 0x0d,
    #[doc = "/15"]
    DIV15 = 0x0e,
    #[doc = "/16"]
    DIV16 = 0x0f,
}
impl Mclkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclkdiv {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclkdiv {
    #[inline(always)]
    fn from(val: u8) -> Mclkdiv {
        Mclkdiv::from_bits(val)
    }
}
impl From<Mclkdiv> for u8 {
    #[inline(always)]
    fn from(val: Mclkdiv) -> u8 {
        Mclkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mclksel {
    #[doc = "ROSCCLK as MCLK"]
    ROSCCLK = 0x0,
    #[doc = "HCLK/MCLKDIV as MCLK"]
    MCLK = 0x01,
}
impl Mclksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclksel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclksel {
    #[inline(always)]
    fn from(val: u8) -> Mclksel {
        Mclksel::from_bits(val)
    }
}
impl From<Mclksel> for u8 {
    #[inline(always)]
    fn from(val: Mclksel) -> u8 {
        Mclksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Writewordcnt {
    #[doc = "4 bytes"]
    BYTES4 = 0x0,
    #[doc = "8 bytes"]
    BYTES8 = 0x01,
    #[doc = "12 bytes"]
    BYTES12 = 0x02,
    #[doc = "16 bytes"]
    BYTES16 = 0x03,
}
impl Writewordcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Writewordcnt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Writewordcnt {
    #[inline(always)]
    fn from(val: u8) -> Writewordcnt {
        Writewordcnt::from_bits(val)
    }
}
impl From<Writewordcnt> for u8 {
    #[inline(always)]
    fn from(val: Writewordcnt) -> u8 {
        Writewordcnt::to_bits(val)
    }
}
