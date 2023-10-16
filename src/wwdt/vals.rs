#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clkdiv {
    #[doc = "/1"]
    DIV1 = 0x0,
    #[doc = "/2"]
    DIV2 = 0x01,
    #[doc = "/4"]
    DIV4 = 0x02,
    #[doc = "/8"]
    DIV8 = 0x03,
    #[doc = "/16"]
    DIV16 = 0x04,
    #[doc = "/32"]
    DIV32 = 0x05,
    #[doc = "/64"]
    DIV64 = 0x06,
    #[doc = "/128"]
    DIV128 = 0x07,
    #[doc = "/256"]
    DIV256 = 0x08,
    #[doc = "/512"]
    DIV512 = 0x09,
    #[doc = "/1024"]
    DIV1024 = 0x0a,
    #[doc = "/2048"]
    DIV2048 = 0x0b,
    #[doc = "/4096"]
    DIV4096 = 0x0c,
    #[doc = "/8192"]
    DIV8192 = 0x0d,
    #[doc = "/16384"]
    DIV16384 = 0x0e,
    #[doc = "/32768"]
    DIV32768 = 0x0f,
}
impl Clkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkdiv {
        unsafe { core::mem::transmute(val & 0x0f) }
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
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clksel {
    #[doc = "FRCLK as WWDT clock"]
    FRCLK = 0x0,
    #[doc = "ROSCCLK as WWDT clock"]
    ROSCCLK = 0x01,
}
impl Clksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clksel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clksel {
    #[inline(always)]
    fn from(val: u8) -> Clksel {
        Clksel::from_bits(val)
    }
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(val: Clksel) -> u8 {
        Clksel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Value(pub u32);
impl Value {
    #[doc = "WWDT registers are read-only"]
    pub const READ_ONLY: Self = Self(0x55aa_6698);
    #[doc = "WWDT registers available for write"]
    pub const ALLOW_WRITE: Self = Self(0x55aa_6699);
}
impl Value {
    pub const fn from_bits(val: u32) -> Value {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Value {
    #[inline(always)]
    fn from(val: u32) -> Value {
        Value::from_bits(val)
    }
}
impl From<Value> for u32 {
    #[inline(always)]
    fn from(val: Value) -> u32 {
        Value::to_bits(val)
    }
}
