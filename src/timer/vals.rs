#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CctlCcintedge {
    #[doc = "Rising Edge Interrupt"]
    RISING = 0x0,
    #[doc = "Falling Edge Interrupt"]
    FALLING = 0x01,
    #[doc = "Rising and Falling Edge Interrupt"]
    BOTH = 0x02,
    _RESERVED_3 = 0x03,
}
impl CctlCcintedge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CctlCcintedge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CctlCcintedge {
    #[inline(always)]
    fn from(val: u8) -> CctlCcintedge {
        CctlCcintedge::from_bits(val)
    }
}
impl From<CctlCcintedge> for u8 {
    #[inline(always)]
    fn from(val: CctlCcintedge) -> u8 {
        CctlCcintedge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CctlCcintskip {
    #[doc = "Don't skip CCR matches before interrupt"]
    NONE = 0x0,
    #[doc = "Skip 1 CCR match before interrupt"]
    SKIP1 = 0x01,
    #[doc = "Skip 2 CCR matches before interrupt"]
    SKIP2 = 0x02,
    #[doc = "Skip 3 CCR matches before interrupt"]
    SKIP3 = 0x03,
    #[doc = "Skip 4 CCR matches before interrupt"]
    SKIP4 = 0x04,
    #[doc = "Skip 5 CCR matches before interrupt"]
    SKIP5 = 0x05,
    #[doc = "Skip 6 CCR matches before interrupt"]
    SKIP6 = 0x06,
    #[doc = "Skip 7 CCR matches before interrupt"]
    SKIP7 = 0x07,
    #[doc = "Skip 8 CCR matches before interrupt"]
    SKIP8 = 0x08,
    #[doc = "Skip 9 CCR matches before interrupt"]
    SKIP9 = 0x09,
    #[doc = "Skip 10 CCR matches before interrupt"]
    SKIP10 = 0x0a,
    #[doc = "Skip 11 CCR matches before interrupt"]
    SKIP11 = 0x0b,
    #[doc = "Skip 12 CCR matches before interrupt"]
    SKIP12 = 0x0c,
    #[doc = "Skip 13 CCR matches before interrupt"]
    SKIP13 = 0x0d,
    #[doc = "Skip 14 CCR matches before interrupt"]
    SKIP14 = 0x0e,
    #[doc = "Skip 15 CCR matches before interrupt"]
    SKIP15 = 0x0f,
}
impl CctlCcintskip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CctlCcintskip {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CctlCcintskip {
    #[inline(always)]
    fn from(val: u8) -> CctlCcintskip {
        CctlCcintskip::from_bits(val)
    }
}
impl From<CctlCcintskip> for u8 {
    #[inline(always)]
    fn from(val: CctlCcintskip) -> u8 {
        CctlCcintskip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CctlCclatch {
    #[doc = "Compare(latch on CTR=0)/ Capture(latch on rising edge)"]
    CTR_ZERO_OR_RISING_EDGE = 0x0,
    #[doc = "Compare(latch on CTR=Period)/ Capture(latch on falling edge)"]
    CTR_PERIOD_OR_FALLING_EDGE = 0x01,
    #[doc = "Compare(latch immediately)/ Capture(latch on both edges)"]
    IMMEDIATE_OR_BOTH_EDGES = 0x02,
    _RESERVED_3 = 0x03,
}
impl CctlCclatch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CctlCclatch {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CctlCclatch {
    #[inline(always)]
    fn from(val: u8) -> CctlCclatch {
        CctlCclatch::from_bits(val)
    }
}
impl From<CctlCclatch> for u8 {
    #[inline(always)]
    fn from(val: CctlCclatch) -> u8 {
        CctlCclatch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CctlCcmode {
    #[doc = "Compare Mode"]
    COMPARE = 0x0,
    #[doc = "Capture Mode"]
    CAPTURE = 0x01,
}
impl CctlCcmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CctlCcmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CctlCcmode {
    #[inline(always)]
    fn from(val: u8) -> CctlCcmode {
        CctlCcmode::from_bits(val)
    }
}
impl From<CctlCcmode> for u8 {
    #[inline(always)]
    fn from(val: CctlCcmode) -> u8 {
        CctlCcmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clkdiv {
    #[doc = "Timer input clock /1"]
    DIV1 = 0x0,
    #[doc = "Timer input clock /2"]
    DIV2 = 0x01,
    #[doc = "Timer input clock /4"]
    DIV4 = 0x02,
    #[doc = "Timer input clock /8"]
    DIV8 = 0x03,
    #[doc = "Timer input clock /16"]
    DIV16 = 0x04,
    #[doc = "Timer input clock /32"]
    DIV32 = 0x05,
    #[doc = "Timer input clock /64"]
    DIV64 = 0x06,
    #[doc = "Timer input clock /128"]
    DIV128 = 0x07,
}
impl Clkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkdiv {
        unsafe { core::mem::transmute(val & 0x07) }
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
pub enum Clksrc {
    #[doc = "PCLK selected as input to timer"]
    PCLK = 0x0,
    #[doc = "ACLK selected as input to timer"]
    ACLK = 0x01,
}
impl Clksrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clksrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clksrc {
    #[inline(always)]
    fn from(val: u8) -> Clksrc {
        Clksrc::from_bits(val)
    }
}
impl From<Clksrc> for u8 {
    #[inline(always)]
    fn from(val: Clksrc) -> u8 {
        Clksrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dtgclk {
    #[doc = "Before input clock divider"]
    BEFORE_DIV = 0x0,
    #[doc = "After input clock divider"]
    AFTER_DIV = 0x01,
}
impl Dtgclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtgclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtgclk {
    #[inline(always)]
    fn from(val: u8) -> Dtgclk {
        Dtgclk::from_bits(val)
    }
}
impl From<Dtgclk> for u8 {
    #[inline(always)]
    fn from(val: Dtgclk) -> u8 {
        Dtgclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Up mode"]
    UP = 0x01,
    #[doc = "Up/Down mode"]
    UP_DOWN = 0x02,
    #[doc = "Asymmetric mode"]
    ASYMMETRIC = 0x03,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Prdlatch {
    #[doc = "Period is latched when CTR = 0"]
    CTR_0 = 0x0,
    #[doc = "Period is latched when CTR = Period (PRD)"]
    CTR_PERIOD = 0x01,
    #[doc = "Period is latched immediately upon register write"]
    IMMEDIATE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Prdlatch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prdlatch {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prdlatch {
    #[inline(always)]
    fn from(val: u8) -> Prdlatch {
        Prdlatch::from_bits(val)
    }
}
impl From<Prdlatch> for u8 {
    #[inline(always)]
    fn from(val: Prdlatch) -> u8 {
        Prdlatch::to_bits(val)
    }
}
