#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Intid {
    _RESERVED_0 = 0x0,
    #[doc = "TX Holding Register Empty"]
    TX_HOLDING_REGISTER_EMPTY = 0x01,
    #[doc = "Receive Data Available"]
    RX_DATA_AVAILABLE = 0x02,
    #[doc = "Receive Line Status"]
    RX_LINE_STATUS = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Receive FIFO Character Timeout"]
    RX_FIFO_TIMEOUT = 0x06,
    _RESERVED_7 = 0x07,
}
impl Intid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intid {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intid {
    #[inline(always)]
    fn from(val: u8) -> Intid {
        Intid::from_bits(val)
    }
}
impl From<Intid> for u8 {
    #[inline(always)]
    fn from(val: Intid) -> u8 {
        Intid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Psel {
    #[doc = "Odd parity"]
    ODD_PARITY = 0x0,
    #[doc = "Even parity"]
    EVEN_PARITY = 0x01,
    #[doc = "Forced 1 sticky parity"]
    FORCED_1_STICKY_PARITY = 0x02,
    #[doc = "Forced 0 sticky parity"]
    FORCED_0_STICKY_PARITY = 0x03,
}
impl Psel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psel {
    #[inline(always)]
    fn from(val: u8) -> Psel {
        Psel::from_bits(val)
    }
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(val: Psel) -> u8 {
        Psel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rxtl {
    #[doc = "1 character"]
    FIFO1 = 0x0,
    #[doc = "4 characters"]
    FIFO4 = 0x01,
    #[doc = "8 characters"]
    FIFO8 = 0x02,
    #[doc = "14 characters"]
    FIFO14 = 0x03,
}
impl Rxtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxtl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxtl {
    #[inline(always)]
    fn from(val: u8) -> Rxtl {
        Rxtl::from_bits(val)
    }
}
impl From<Rxtl> for u8 {
    #[inline(always)]
    fn from(val: Rxtl) -> u8 {
        Rxtl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sbs {
    #[doc = "1 stop bit"]
    ONE = 0x0,
    #[doc = "2 stop bits (1.5 if LCR.WLS = 0)"]
    TWO = 0x01,
}
impl Sbs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbs {
    #[inline(always)]
    fn from(val: u8) -> Sbs {
        Sbs::from_bits(val)
    }
}
impl From<Sbs> for u8 {
    #[inline(always)]
    fn from(val: Sbs) -> u8 {
        Sbs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Txtl {
    #[doc = "1 character"]
    FIFO1 = 0x0,
    #[doc = "4 characters"]
    FIFO4 = 0x01,
    #[doc = "8 characters"]
    FIFO8 = 0x02,
    #[doc = "14 characters"]
    FIFO14 = 0x03,
}
impl Txtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txtl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txtl {
    #[inline(always)]
    fn from(val: u8) -> Txtl {
        Txtl::from_bits(val)
    }
}
impl From<Txtl> for u8 {
    #[inline(always)]
    fn from(val: Txtl) -> u8 {
        Txtl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wls {
    #[doc = "5 bits"]
    WL5 = 0x0,
    #[doc = "6 bits"]
    WL6 = 0x01,
    #[doc = "7 bits"]
    WL7 = 0x02,
    #[doc = "8 bits"]
    WL8 = 0x03,
}
impl Wls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wls {
    #[inline(always)]
    fn from(val: u8) -> Wls {
        Wls::from_bits(val)
    }
}
impl From<Wls> for u8 {
    #[inline(always)]
    fn from(val: Wls) -> u8 {
        Wls::to_bits(val)
    }
}
