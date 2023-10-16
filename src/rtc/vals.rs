#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clkdiv {
    #[doc = "FRCLK/1"]
    DIV1 = 0x0,
    #[doc = "FRCLK/2"]
    DIV2 = 0x01,
    #[doc = "FRCLK/3"]
    DIV3 = 0x02,
    #[doc = "FRCLK/4"]
    DIV4 = 0x03,
    #[doc = "FRCLK/5"]
    DIV5 = 0x04,
    #[doc = "FRCLK/6"]
    DIV6 = 0x05,
    #[doc = "FRCLK/7"]
    DIV7 = 0x06,
    #[doc = "FRCLK/8"]
    DIV8 = 0x07,
    #[doc = "FRCLK/9"]
    DIV9 = 0x08,
    #[doc = "FRCLK/10"]
    DIV10 = 0x09,
    #[doc = "FRCLK/11"]
    DIV11 = 0x0a,
    #[doc = "FRCLK/12"]
    DIV12 = 0x0b,
    #[doc = "FRCLK/13"]
    DIV13 = 0x0c,
    #[doc = "FRCLK/14"]
    DIV14 = 0x0d,
    #[doc = "FRCLK/15"]
    DIV15 = 0x0e,
    #[doc = "FRCLK/16"]
    DIV16 = 0x0f,
    #[doc = "FRCLK/17"]
    DIV17 = 0x10,
    #[doc = "FRCLK/18"]
    DIV18 = 0x11,
    #[doc = "FRCLK/19"]
    DIV19 = 0x12,
    #[doc = "FRCLK/20"]
    DIV20 = 0x13,
    #[doc = "FRCLK/21"]
    DIV21 = 0x14,
    #[doc = "FRCLK/22"]
    DIV22 = 0x15,
    #[doc = "FRCLK/23"]
    DIV23 = 0x16,
    #[doc = "FRCLK/24"]
    DIV24 = 0x17,
    #[doc = "FRCLK/25"]
    DIV25 = 0x18,
    #[doc = "FRCLK/26"]
    DIV26 = 0x19,
    #[doc = "FRCLK/27"]
    DIV27 = 0x1a,
    #[doc = "FRCLK/28"]
    DIV28 = 0x1b,
    #[doc = "FRCLK/29"]
    DIV29 = 0x1c,
    #[doc = "FRCLK/30"]
    DIV30 = 0x1d,
    #[doc = "FRCLK/31"]
    DIV31 = 0x1e,
    #[doc = "FRCLK/32"]
    DIV32 = 0x1f,
}
impl Clkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkdiv {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkdiv {
    #[inline(always)]
    fn from(val: u8) -> Clkdiv {
        Clkdiv::from_bits(val)
    }
}
impl From<Clkdiv> for u8 {
    #[inline(always)]
    fn from(val: Clkdiv) -> u8 {
        Clkdiv::to_bits(val)
    }
}
