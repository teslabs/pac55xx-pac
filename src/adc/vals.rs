#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adcdiv {
    #[doc = "SCLK/1"]
    SCLK1 = 0x0,
    #[doc = "SCLK/2"]
    SCLK2 = 0x01,
    #[doc = "SCLK/3"]
    SCLK3 = 0x02,
    #[doc = "SCLK/4"]
    SCLK4 = 0x03,
    #[doc = "SCLK/5"]
    SCLK5 = 0x04,
    #[doc = "SCLK/6"]
    SCLK6 = 0x05,
    #[doc = "SCLK/7"]
    SCLK7 = 0x06,
    #[doc = "SCLK/8"]
    SCLK8 = 0x07,
    #[doc = "SCLK/9"]
    SCLK9 = 0x08,
    #[doc = "SCLK/10"]
    SCLK10 = 0x09,
    #[doc = "SCLK/11"]
    SCLK11 = 0x0a,
    #[doc = "SCLK/12"]
    SCLK12 = 0x0b,
    #[doc = "SCLK/13"]
    SCLK13 = 0x0c,
    #[doc = "SCLK/14"]
    SCLK14 = 0x0d,
    #[doc = "SCLK/15"]
    SCLK15 = 0x0e,
    #[doc = "SCLK/16"]
    SCLK16 = 0x0f,
}
impl Adcdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adcdiv {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adcdiv {
    #[inline(always)]
    fn from(val: u8) -> Adcdiv {
        Adcdiv::from_bits(val)
    }
}
impl From<Adcdiv> for u8 {
    #[inline(always)]
    fn from(val: Adcdiv) -> u8 {
        Adcdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DtseseqcfgEmuxc {
    #[doc = "Do not send EMUX command"]
    NO_EMUX_CMD = 0x0,
    #[doc = "Send EMUX command before sample and hold"]
    EMUX_CMD_BEFORE_SAMPLE = 0x01,
    #[doc = "Send EMUX command after conversion is complete"]
    EMUX_CMD_AFTER_CONVERSION = 0x02,
    _RESERVED_3 = 0x03,
}
impl DtseseqcfgEmuxc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DtseseqcfgEmuxc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DtseseqcfgEmuxc {
    #[inline(always)]
    fn from(val: u8) -> DtseseqcfgEmuxc {
        DtseseqcfgEmuxc::from_bits(val)
    }
}
impl From<DtseseqcfgEmuxc> for u8 {
    #[inline(always)]
    fn from(val: DtseseqcfgEmuxc) -> u8 {
        DtseseqcfgEmuxc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Emuxdiv {
    #[doc = "SCLK/1"]
    SCLK1 = 0x0,
    #[doc = "SCLK/2"]
    SCLK2 = 0x01,
    #[doc = "SCLK/3"]
    SCLK3 = 0x02,
    #[doc = "SCLK/4"]
    SCLK4 = 0x03,
    #[doc = "SCLK/5"]
    SCLK5 = 0x04,
    #[doc = "SCLK/6"]
    SCLK6 = 0x05,
    #[doc = "SCLK/7"]
    SCLK7 = 0x06,
    #[doc = "SCLK/8"]
    SCLK8 = 0x07,
    #[doc = "SCLK/9"]
    SCLK9 = 0x08,
    #[doc = "SCLK/10"]
    SCLK10 = 0x09,
    #[doc = "SCLK/11"]
    SCLK11 = 0x0a,
    #[doc = "SCLK/12"]
    SCLK12 = 0x0b,
    #[doc = "SCLK/13"]
    SCLK13 = 0x0c,
    #[doc = "SCLK/14"]
    SCLK14 = 0x0d,
    #[doc = "SCLK/15"]
    SCLK15 = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Emuxdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emuxdiv {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emuxdiv {
    #[inline(always)]
    fn from(val: u8) -> Emuxdiv {
        Emuxdiv::from_bits(val)
    }
}
impl From<Emuxdiv> for u8 {
    #[inline(always)]
    fn from(val: Emuxdiv) -> u8 {
        Emuxdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Emuxmode {
    #[doc = "Write EMUX data from EMUXDATA register"]
    DATA_FROM_EMUXDATA = 0x0,
    #[doc = "Write EMUX data from DTSE sequencer commands"]
    DATA_FROM_DTSE = 0x01,
}
impl Emuxmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emuxmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emuxmode {
    #[inline(always)]
    fn from(val: u8) -> Emuxmode {
        Emuxmode::from_bits(val)
    }
}
impl From<Emuxmode> for u8 {
    #[inline(always)]
    fn from(val: Emuxmode) -> u8 {
        Emuxmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "Manual mode"]
    MANUAL = 0x0,
    #[doc = "DTSE mode"]
    DTSE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
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
pub enum Trigedge {
    #[doc = "Unused"]
    UNUSED = 0x0,
    #[doc = "Rising edge"]
    RISING_EDGE = 0x01,
    #[doc = "Falling edge"]
    FALLING_EDGE = 0x02,
    #[doc = "Both edges"]
    BOTH_EDGES = 0x03,
}
impl Trigedge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigedge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigedge {
    #[inline(always)]
    fn from(val: u8) -> Trigedge {
        Trigedge::from_bits(val)
    }
}
impl From<Trigedge> for u8 {
    #[inline(always)]
    fn from(val: Trigedge) -> u8 {
        Trigedge::to_bits(val)
    }
}
