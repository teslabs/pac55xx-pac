#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cph {
    #[doc = "First edge"]
    FIRST_EDGE = 0x0,
    #[doc = "Second edge"]
    SECOND_EDGE = 0x01,
}
impl Cph {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cph {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cph {
    #[inline(always)]
    fn from(val: u8) -> Cph {
        Cph::from_bits(val)
    }
}
impl From<Cph> for u8 {
    #[inline(always)]
    fn from(val: Cph) -> u8 {
        Cph::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cpo {
    #[doc = "Active High"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Active Low"]
    ACTIVE_LOW = 0x01,
}
impl Cpo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpo {
    #[inline(always)]
    fn from(val: u8) -> Cpo {
        Cpo::from_bits(val)
    }
}
impl From<Cpo> for u8 {
    #[inline(always)]
    fn from(val: Cpo) -> u8 {
        Cpo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dss {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "4-bit transfer"]
    BITS4 = 0x03,
    #[doc = "5-bit transfer"]
    BITS5 = 0x04,
    #[doc = "6-bit transfer"]
    BITS6 = 0x05,
    #[doc = "7-bit transfer"]
    BITS7 = 0x06,
    #[doc = "8-bit transfer"]
    BITS8 = 0x07,
    #[doc = "9-bit transfer"]
    BITS9 = 0x08,
    #[doc = "10-bit transfer"]
    BITS10 = 0x09,
    #[doc = "11-bit transfer"]
    BITS11 = 0x0a,
    #[doc = "12-bit transfer"]
    BITS12 = 0x0b,
    #[doc = "13-bit transfer"]
    BITS13 = 0x0c,
    #[doc = "14-bit transfer"]
    BITS14 = 0x0d,
    #[doc = "15-bit transfer"]
    BITS15 = 0x0e,
    #[doc = "16-bit transfer"]
    BITS16 = 0x0f,
    #[doc = "17-bit transfer"]
    BITS17 = 0x10,
    #[doc = "18-bit transfer"]
    BITS18 = 0x11,
    #[doc = "19-bit transfer"]
    BITS19 = 0x12,
    #[doc = "20-bit transfer"]
    BITS20 = 0x13,
    #[doc = "21-bit transfer"]
    BITS21 = 0x14,
    #[doc = "22-bit transfer"]
    BITS22 = 0x15,
    #[doc = "23-bit transfer"]
    BITS23 = 0x16,
    #[doc = "24-bit transfer"]
    BITS24 = 0x17,
    #[doc = "25-bit transfer"]
    BITS25 = 0x18,
    #[doc = "26-bit transfer"]
    BITS26 = 0x19,
    #[doc = "27-bit transfer"]
    BITS27 = 0x1a,
    #[doc = "28-bit transfer"]
    BITS28 = 0x1b,
    #[doc = "29-bit transfer"]
    BITS29 = 0x1c,
    #[doc = "30-bit transfer"]
    BITS30 = 0x1d,
    #[doc = "31-bit transfer"]
    BITS31 = 0x1e,
    #[doc = "32-bit transfer"]
    BITS32 = 0x1f,
}
impl Dss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dss {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dss {
    #[inline(always)]
    fn from(val: u8) -> Dss {
        Dss::from_bits(val)
    }
}
impl From<Dss> for u8 {
    #[inline(always)]
    fn from(val: Dss) -> u8 {
        Dss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Frf {
    #[doc = "SPI (Motorola)"]
    SPI = 0x0,
    #[doc = "Synchronous Serial Format (TI)"]
    TI = 0x01,
    #[doc = "Microwire (National Semiconductor)"]
    MICROWIRE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Frf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frf {
    #[inline(always)]
    fn from(val: u8) -> Frf {
        Frf::from_bits(val)
    }
}
impl From<Frf> for u8 {
    #[inline(always)]
    fn from(val: Frf) -> u8 {
        Frf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ms {
    #[doc = "Master"]
    MASTER = 0x0,
    #[doc = "Slave"]
    SLAVE = 0x01,
}
impl Ms {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ms {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ms {
    #[inline(always)]
    fn from(val: u8) -> Ms {
        Ms::from_bits(val)
    }
}
impl From<Ms> for u8 {
    #[inline(always)]
    fn from(val: Ms) -> u8 {
        Ms::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Selss {
    #[doc = "Slave Select Enabled"]
    SS_ENABLED = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Selss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Selss {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Selss {
    #[inline(always)]
    fn from(val: u8) -> Selss {
        Selss::from_bits(val)
    }
}
impl From<Selss> for u8 {
    #[inline(always)]
    fn from(val: Selss) -> u8 {
        Selss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sphdontcare {
    #[doc = "SS cannot be pulled high after transfer"]
    NO_PULL_HIGH = 0x0,
    #[doc = "SS must be pulled high after transfer"]
    PULL_HIGH = 0x01,
}
impl Sphdontcare {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sphdontcare {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sphdontcare {
    #[inline(always)]
    fn from(val: u8) -> Sphdontcare {
        Sphdontcare::from_bits(val)
    }
}
impl From<Sphdontcare> for u8 {
    #[inline(always)]
    fn from(val: Sphdontcare) -> u8 {
        Sphdontcare::to_bits(val)
    }
}
