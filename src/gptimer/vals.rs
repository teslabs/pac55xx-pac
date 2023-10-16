#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Div {
    #[doc = "PCLK/1"]
    PCLK1 = 0x0,
    #[doc = "PCLK/2"]
    PCLK2 = 0x01,
    #[doc = "PCLK/4"]
    PCLK4 = 0x02,
    #[doc = "PCLK/8"]
    PCLK8 = 0x03,
    #[doc = "PCLK/16"]
    PCLK16 = 0x04,
    #[doc = "PCLK/32"]
    PCLK32 = 0x05,
    #[doc = "PCLK/64"]
    PCLK64 = 0x06,
    #[doc = "PCLK/128"]
    PCLK128 = 0x07,
    #[doc = "PCLK/256"]
    PCLK256 = 0x08,
    #[doc = "PCLK/512"]
    PCLK512 = 0x09,
    #[doc = "PCLK/1024"]
    PCLK1024 = 0x0a,
    #[doc = "PCLK/2048"]
    PCLK2048 = 0x0b,
    #[doc = "PCLK/4096"]
    PCLK4096 = 0x0c,
    #[doc = "PCLK/8192"]
    PCLK8192 = 0x0d,
    #[doc = "PCLK/16384"]
    PCLK16384 = 0x0e,
    #[doc = "PCLK/32768"]
    PCLK32768 = 0x0f,
}
impl Div {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Div {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Div {
    #[inline(always)]
    fn from(val: u8) -> Div {
        Div::from_bits(val)
    }
}
impl From<Div> for u8 {
    #[inline(always)]
    fn from(val: Div) -> u8 {
        Div::to_bits(val)
    }
}
