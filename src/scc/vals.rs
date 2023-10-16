#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Aclkdiv {
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
}
impl Aclkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aclkdiv {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aclkdiv {
    #[inline(always)]
    fn from(val: u8) -> Aclkdiv {
        Aclkdiv::from_bits(val)
    }
}
impl From<Aclkdiv> for u8 {
    #[inline(always)]
    fn from(val: Aclkdiv) -> u8 {
        Aclkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clkfailmuxsel {
    #[doc = "FRCLK selected as input to Clock Fail"]
    FRCLK = 0x0,
    #[doc = "PLLCLK selected as input to Clock Fail"]
    PLLCLK = 0x01,
}
impl Clkfailmuxsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkfailmuxsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkfailmuxsel {
    #[inline(always)]
    fn from(val: u8) -> Clkfailmuxsel {
        Clkfailmuxsel::from_bits(val)
    }
}
impl From<Clkfailmuxsel> for u8 {
    #[inline(always)]
    fn from(val: Clkfailmuxsel) -> u8 {
        Clkfailmuxsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Frclkmuxsel {
    #[doc = "ROSC selected as input to FRCLK"]
    ROSC = 0x0,
    #[doc = "CLKREF selected as input to FRCLK"]
    CLKREF = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "External Clock selected as input to FRCLK"]
    EXTCLK = 0x03,
}
impl Frclkmuxsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frclkmuxsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frclkmuxsel {
    #[inline(always)]
    fn from(val: u8) -> Frclkmuxsel {
        Frclkmuxsel::from_bits(val)
    }
}
impl From<Frclkmuxsel> for u8 {
    #[inline(always)]
    fn from(val: Frclkmuxsel) -> u8 {
        Frclkmuxsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hclkdiv {
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
}
impl Hclkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hclkdiv {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hclkdiv {
    #[inline(always)]
    fn from(val: u8) -> Hclkdiv {
        Hclkdiv::from_bits(val)
    }
}
impl From<Hclkdiv> for u8 {
    #[inline(always)]
    fn from(val: Hclkdiv) -> u8 {
        Hclkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pa0 {
    #[doc = "GPIO"]
    GPIOA0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pa0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pa0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pa0 {
    #[inline(always)]
    fn from(val: u8) -> Pa0 {
        Pa0::from_bits(val)
    }
}
impl From<Pa0> for u8 {
    #[inline(always)]
    fn from(val: Pa0) -> u8 {
        Pa0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pa1 {
    #[doc = "GPIO"]
    GPIOA1 = 0x0,
    #[doc = "EMUX Data"]
    EMUXD = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pa1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pa1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pa1 {
    #[inline(always)]
    fn from(val: u8) -> Pa1 {
        Pa1::from_bits(val)
    }
}
impl From<Pa1> for u8 {
    #[inline(always)]
    fn from(val: Pa1) -> u8 {
        Pa1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pa2 {
    #[doc = "GPIO"]
    GPIOA2 = 0x0,
    #[doc = "EMUX Clock"]
    EMUXC = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pa2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pa2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pa2 {
    #[inline(always)]
    fn from(val: u8) -> Pa2 {
        Pa2::from_bits(val)
    }
}
impl From<Pa2> for u8 {
    #[inline(always)]
    fn from(val: Pa2) -> u8 {
        Pa2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pa3 {
    #[doc = "GPIO"]
    GPIOA3 = 0x0,
    #[doc = "USART A SCLK"]
    USASCLK = 0x01,
    #[doc = "USART B SCLK"]
    USBSCLK = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pa3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pa3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pa3 {
    #[inline(always)]
    fn from(val: u8) -> Pa3 {
        Pa3::from_bits(val)
    }
}
impl From<Pa3> for u8 {
    #[inline(always)]
    fn from(val: Pa3) -> u8 {
        Pa3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pa4 {
    #[doc = "GPIO"]
    GPIOA4 = 0x0,
    #[doc = "USART A MOSI/TX"]
    USAMOSI = 0x01,
    #[doc = "USART B MOSI/TX"]
    USBMOSI = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pa4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pa4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pa4 {
    #[inline(always)]
    fn from(val: u8) -> Pa4 {
        Pa4::from_bits(val)
    }
}
impl From<Pa4> for u8 {
    #[inline(always)]
    fn from(val: Pa4) -> u8 {
        Pa4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pa5 {
    #[doc = "GPIO"]
    GPIOA5 = 0x0,
    #[doc = "USART A MISO/RX"]
    USAMISO = 0x01,
    #[doc = "USART B MISO/RX"]
    USBMISO = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pa5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pa5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pa5 {
    #[inline(always)]
    fn from(val: u8) -> Pa5 {
        Pa5::from_bits(val)
    }
}
impl From<Pa5> for u8 {
    #[inline(always)]
    fn from(val: Pa5) -> u8 {
        Pa5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pa6 {
    #[doc = "GPIO"]
    GPIOA6 = 0x0,
    #[doc = "USART A SPI Slave Select"]
    USASS = 0x01,
    #[doc = "USART B SPI Slave Select"]
    USBSS = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pa6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pa6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pa6 {
    #[inline(always)]
    fn from(val: u8) -> Pa6 {
        Pa6::from_bits(val)
    }
}
impl From<Pa6> for u8 {
    #[inline(always)]
    fn from(val: Pa6) -> u8 {
        Pa6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pa7 {
    #[doc = "GPIO"]
    GPIOA7 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pa7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pa7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pa7 {
    #[inline(always)]
    fn from(val: u8) -> Pa7 {
        Pa7::from_bits(val)
    }
}
impl From<Pa7> for u8 {
    #[inline(always)]
    fn from(val: Pa7) -> u8 {
        Pa7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pb0 {
    #[doc = "GPIO"]
    GPIOB0 = 0x0,
    #[doc = "Timer A PWM0"]
    TAPWM0 = 0x01,
    #[doc = "Timer B PWM0"]
    TBPWM0 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Timer C PWM0"]
    TCPWM0 = 0x04,
    #[doc = "Timer D PWM0"]
    TDPWM0 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pb0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pb0 {
    #[inline(always)]
    fn from(val: u8) -> Pb0 {
        Pb0::from_bits(val)
    }
}
impl From<Pb0> for u8 {
    #[inline(always)]
    fn from(val: Pb0) -> u8 {
        Pb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pb1 {
    #[doc = "GPIO"]
    GPIOB1 = 0x0,
    #[doc = "Timer A PWM1"]
    TAPWM1 = 0x01,
    #[doc = "Timer B PWM1"]
    TBPWM1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Timer C PWM1"]
    TCPWM1 = 0x04,
    #[doc = "Timer D PWM1"]
    TDPWM1 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pb1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pb1 {
    #[inline(always)]
    fn from(val: u8) -> Pb1 {
        Pb1::from_bits(val)
    }
}
impl From<Pb1> for u8 {
    #[inline(always)]
    fn from(val: Pb1) -> u8 {
        Pb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pb2 {
    #[doc = "GPIO"]
    GPIOB2 = 0x0,
    #[doc = "Timer A PWM2"]
    TAPWM2 = 0x01,
    #[doc = "Timer B PWM2"]
    TBPWM2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Timer C PWM2"]
    TCPWM2 = 0x04,
    #[doc = "Timer D PWM2"]
    TDPWM2 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pb2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pb2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pb2 {
    #[inline(always)]
    fn from(val: u8) -> Pb2 {
        Pb2::from_bits(val)
    }
}
impl From<Pb2> for u8 {
    #[inline(always)]
    fn from(val: Pb2) -> u8 {
        Pb2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pb3 {
    #[doc = "GPIO"]
    GPIOB3 = 0x0,
    #[doc = "Timer A PWM3"]
    TAPWM3 = 0x01,
    #[doc = "Timer B PWM3"]
    TBPWM3 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Timer C PWM3"]
    TCPWM3 = 0x04,
    #[doc = "Timer D PWM3"]
    TDPWM3 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pb3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pb3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pb3 {
    #[inline(always)]
    fn from(val: u8) -> Pb3 {
        Pb3::from_bits(val)
    }
}
impl From<Pb3> for u8 {
    #[inline(always)]
    fn from(val: Pb3) -> u8 {
        Pb3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pb4 {
    #[doc = "GPIO"]
    GPIOB4 = 0x0,
    #[doc = "Timer A PWM4"]
    TAPWM4 = 0x01,
    #[doc = "Timer B PWM4"]
    TBPWM4 = 0x02,
    #[doc = "Timer C PWM0"]
    TCPWM0 = 0x03,
    #[doc = "Timer C PWM4"]
    TCPWM4 = 0x04,
    #[doc = "Timer D PWM4"]
    TDPWM4 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pb4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pb4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pb4 {
    #[inline(always)]
    fn from(val: u8) -> Pb4 {
        Pb4::from_bits(val)
    }
}
impl From<Pb4> for u8 {
    #[inline(always)]
    fn from(val: Pb4) -> u8 {
        Pb4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pb5 {
    #[doc = "GPIO"]
    GPIOB5 = 0x0,
    #[doc = "Timer A PWM5"]
    TAPWM5 = 0x01,
    #[doc = "Timer B PWM5"]
    TBPWM5 = 0x02,
    #[doc = "Timer C PWM1"]
    TCPWM1 = 0x03,
    #[doc = "Timer C PWM5"]
    TCPWM5 = 0x04,
    #[doc = "Timer D PWM5"]
    TDPWM5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pb5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pb5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pb5 {
    #[inline(always)]
    fn from(val: u8) -> Pb5 {
        Pb5::from_bits(val)
    }
}
impl From<Pb5> for u8 {
    #[inline(always)]
    fn from(val: Pb5) -> u8 {
        Pb5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pb6 {
    #[doc = "GPIO"]
    GPIOB6 = 0x0,
    #[doc = "Timer A PWM6"]
    TAPWM6 = 0x01,
    #[doc = "Timer B PWM6"]
    TBPWM6 = 0x02,
    #[doc = "Timer C PWM2"]
    TCPWM2 = 0x03,
    #[doc = "Timer C PWM6"]
    TCPWM6 = 0x04,
    #[doc = "Timer D PWM6"]
    TDPWM6 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pb6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pb6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pb6 {
    #[inline(always)]
    fn from(val: u8) -> Pb6 {
        Pb6::from_bits(val)
    }
}
impl From<Pb6> for u8 {
    #[inline(always)]
    fn from(val: Pb6) -> u8 {
        Pb6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pb7 {
    #[doc = "GPIO"]
    GPIOB7 = 0x0,
    #[doc = "Timer A PWM7"]
    TAPWM7 = 0x01,
    #[doc = "Timer B PWM7"]
    TBPWM7 = 0x02,
    #[doc = "Timer C PWM3"]
    TCPWM3 = 0x03,
    #[doc = "Timer C PWM7"]
    TCPWM7 = 0x04,
    #[doc = "Timer D PWM7"]
    TDPWM7 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pb7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pb7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pb7 {
    #[inline(always)]
    fn from(val: u8) -> Pb7 {
        Pb7::from_bits(val)
    }
}
impl From<Pb7> for u8 {
    #[inline(always)]
    fn from(val: Pb7) -> u8 {
        Pb7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pc0 {
    #[doc = "GPIO"]
    GPIOC0 = 0x0,
    #[doc = "Timer B PWM0"]
    TBPWM0 = 0x01,
    #[doc = "Timer C PWM0"]
    TCPWM0 = 0x02,
    #[doc = "Timer B QEP Index"]
    TBQEPIDX = 0x03,
    #[doc = "USART B MOSI/TX"]
    USBMOSI = 0x04,
    #[doc = "USART C SPI Clock"]
    USCSCLK = 0x05,
    #[doc = "CAN Rx Data"]
    CANRXD = 0x06,
    #[doc = "I2C SCL"]
    I2CSCL = 0x07,
}
impl Pc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pc0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pc0 {
    #[inline(always)]
    fn from(val: u8) -> Pc0 {
        Pc0::from_bits(val)
    }
}
impl From<Pc0> for u8 {
    #[inline(always)]
    fn from(val: Pc0) -> u8 {
        Pc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pc1 {
    #[doc = "GPIO"]
    GPIOC1 = 0x0,
    #[doc = "Timer B PWM1"]
    TBPWM1 = 0x01,
    #[doc = "Timer C PWM1"]
    TCPWM1 = 0x02,
    #[doc = "Timer B QEP Phase A"]
    TBQEPPHA = 0x03,
    #[doc = "USART B MISO/RX"]
    USBMISO = 0x04,
    #[doc = "USART C Slave Select"]
    USCSS = 0x05,
    #[doc = "CAN Tx Data"]
    CANTXD = 0x06,
    #[doc = "I2C SDA"]
    I2CSDA = 0x07,
}
impl Pc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pc1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pc1 {
    #[inline(always)]
    fn from(val: u8) -> Pc1 {
        Pc1::from_bits(val)
    }
}
impl From<Pc1> for u8 {
    #[inline(always)]
    fn from(val: Pc1) -> u8 {
        Pc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pc2 {
    #[doc = "GPIO"]
    GPIOC2 = 0x0,
    #[doc = "Timer B PWM2"]
    TBPWM2 = 0x01,
    #[doc = "Timer C PWM2"]
    TCPWM2 = 0x02,
    #[doc = "Timer B QEP Phase B"]
    TBQEPPHB = 0x03,
    #[doc = "USART B SCLK"]
    USBSCLK = 0x04,
    #[doc = "USART C MOSI/TX"]
    USCMOSI = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "EMUX Data"]
    EMUXD = 0x07,
}
impl Pc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pc2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pc2 {
    #[inline(always)]
    fn from(val: u8) -> Pc2 {
        Pc2::from_bits(val)
    }
}
impl From<Pc2> for u8 {
    #[inline(always)]
    fn from(val: Pc2) -> u8 {
        Pc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pc3 {
    #[doc = "GPIO"]
    GPIOC3 = 0x0,
    #[doc = "Timer B PWM3"]
    TBPWM3 = 0x01,
    #[doc = "Timer C PWM3"]
    TCPWM3 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "USART B Slave Select"]
    USBSS = 0x04,
    #[doc = "USART C MISO/RX"]
    USCMISO = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "EMUX Clock"]
    EMUXC = 0x07,
}
impl Pc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pc3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pc3 {
    #[inline(always)]
    fn from(val: u8) -> Pc3 {
        Pc3::from_bits(val)
    }
}
impl From<Pc3> for u8 {
    #[inline(always)]
    fn from(val: Pc3) -> u8 {
        Pc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pc4 {
    #[doc = "GPIO"]
    GPIOC4 = 0x0,
    #[doc = "Timer B PWM4"]
    TBPWM4 = 0x01,
    #[doc = "Timer C PWM4"]
    TCPWM4 = 0x02,
    #[doc = "Timer C QEP Index"]
    TCQEPIDX = 0x03,
    #[doc = "USART B MOSI/TX"]
    USBMOSI = 0x04,
    #[doc = "USART C SPI Clock"]
    USCSCLK = 0x05,
    #[doc = "CAN Rx Data"]
    CANRXD = 0x06,
    #[doc = "I2C SCL"]
    I2CSCL = 0x07,
}
impl Pc4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pc4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pc4 {
    #[inline(always)]
    fn from(val: u8) -> Pc4 {
        Pc4::from_bits(val)
    }
}
impl From<Pc4> for u8 {
    #[inline(always)]
    fn from(val: Pc4) -> u8 {
        Pc4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pc5 {
    #[doc = "GPIO"]
    GPIOC5 = 0x0,
    #[doc = "Timer B PWM5"]
    TBPWM5 = 0x01,
    #[doc = "Timer C PWM5"]
    TCPWM5 = 0x02,
    #[doc = "Timer C QEP Phase A"]
    TCQEPPHA = 0x03,
    #[doc = "USART B MISO/RX"]
    USBMISO = 0x04,
    #[doc = "USART C Slave Select"]
    USCSS = 0x05,
    #[doc = "CAN Tx Data"]
    CANTXD = 0x06,
    #[doc = "I2C SDA"]
    I2CSDA = 0x07,
}
impl Pc5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pc5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pc5 {
    #[inline(always)]
    fn from(val: u8) -> Pc5 {
        Pc5::from_bits(val)
    }
}
impl From<Pc5> for u8 {
    #[inline(always)]
    fn from(val: Pc5) -> u8 {
        Pc5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pc6 {
    #[doc = "GPIO"]
    GPIOC6 = 0x0,
    #[doc = "Timer B PWM6"]
    TBPWM6 = 0x01,
    #[doc = "Timer C PWM6"]
    TCPWM6 = 0x02,
    #[doc = "Timer C QEP Phase B"]
    TCQEPPHB = 0x03,
    #[doc = "USART B SCLK"]
    USBSCLK = 0x04,
    #[doc = "USART C MOSI/TX"]
    USCMOSI = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "EMUX Data"]
    EMUXD = 0x07,
}
impl Pc6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pc6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pc6 {
    #[inline(always)]
    fn from(val: u8) -> Pc6 {
        Pc6::from_bits(val)
    }
}
impl From<Pc6> for u8 {
    #[inline(always)]
    fn from(val: Pc6) -> u8 {
        Pc6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pc7 {
    #[doc = "GPIO"]
    GPIOC7 = 0x0,
    #[doc = "Timer B PWM7"]
    TBPWM7 = 0x01,
    #[doc = "Timer C PWM7"]
    TCPWM7 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "USART B Slave Select"]
    USBSS = 0x04,
    #[doc = "USART C MISO/RX"]
    USCMISO = 0x05,
    #[doc = "FRCLK Output"]
    FRCLK = 0x06,
    #[doc = "EMUX Clock"]
    EMUXC = 0x07,
}
impl Pc7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pc7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pc7 {
    #[inline(always)]
    fn from(val: u8) -> Pc7 {
        Pc7::from_bits(val)
    }
}
impl From<Pc7> for u8 {
    #[inline(always)]
    fn from(val: Pc7) -> u8 {
        Pc7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pclkdiv {
    #[doc = "HCLK/1"]
    HCLK1 = 0x0,
    #[doc = "HCLK/2"]
    HCLK2 = 0x01,
    #[doc = "HCLK/3"]
    HCLK3 = 0x02,
    #[doc = "HCLK/4"]
    HCLK4 = 0x03,
    #[doc = "HCLK/5"]
    HCLK5 = 0x04,
    #[doc = "HCLK/6"]
    HCLK6 = 0x05,
    #[doc = "HCLK/7"]
    HCLK7 = 0x06,
    #[doc = "HCLK/8"]
    HCLK8 = 0x07,
}
impl Pclkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pclkdiv {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pclkdiv {
    #[inline(always)]
    fn from(val: u8) -> Pclkdiv {
        Pclkdiv::from_bits(val)
    }
}
impl From<Pclkdiv> for u8 {
    #[inline(always)]
    fn from(val: Pclkdiv) -> u8 {
        Pclkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pd0 {
    #[doc = "GPIO"]
    GPIOD0 = 0x0,
    #[doc = "Timer B PWM0"]
    TBPWM0 = 0x01,
    #[doc = "Timer C PWM0"]
    TCPWM0 = 0x02,
    #[doc = "Timer D QEP Index"]
    TDQEPIDX = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART C SPI Clock"]
    USCSCLK = 0x05,
    #[doc = "CAN Tx Data"]
    CANTXD = 0x06,
    #[doc = "EMUX Data"]
    EMUXD = 0x07,
}
impl Pd0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pd0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pd0 {
    #[inline(always)]
    fn from(val: u8) -> Pd0 {
        Pd0::from_bits(val)
    }
}
impl From<Pd0> for u8 {
    #[inline(always)]
    fn from(val: Pd0) -> u8 {
        Pd0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pd1 {
    #[doc = "GPIO"]
    GPIOD1 = 0x0,
    #[doc = "Timer B PWM1"]
    TBPWM1 = 0x01,
    #[doc = "Timer C PWM1"]
    TCPWM1 = 0x02,
    #[doc = "Timer D QEP Phase A"]
    TDQEPPHA = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART C Slave Select"]
    USCSS = 0x05,
    #[doc = "CAN Rx Data"]
    CANRXD = 0x06,
    #[doc = "EMUX Clock"]
    EMUXC = 0x07,
}
impl Pd1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pd1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pd1 {
    #[inline(always)]
    fn from(val: u8) -> Pd1 {
        Pd1::from_bits(val)
    }
}
impl From<Pd1> for u8 {
    #[inline(always)]
    fn from(val: Pd1) -> u8 {
        Pd1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pd2 {
    #[doc = "GPIO"]
    GPIOD2 = 0x0,
    #[doc = "Timer B PWM2"]
    TBPWM2 = 0x01,
    #[doc = "Timer C PWM2"]
    TCPWM2 = 0x02,
    #[doc = "Timer D QEP Phase B"]
    TDQEPPHB = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART C MOSI/TX"]
    USCMOSI = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pd2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pd2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pd2 {
    #[inline(always)]
    fn from(val: u8) -> Pd2 {
        Pd2::from_bits(val)
    }
}
impl From<Pd2> for u8 {
    #[inline(always)]
    fn from(val: Pd2) -> u8 {
        Pd2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pd3 {
    #[doc = "GPIO"]
    GPIOD3 = 0x0,
    #[doc = "Timer B PWM3"]
    TBPWM3 = 0x01,
    #[doc = "Timer C PWM3"]
    TCPWM3 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART C MISO/RX"]
    USCMISO = 0x05,
    #[doc = "FRCLK output"]
    FRCLK = 0x06,
    #[doc = "TRACE Data 3 output"]
    TRACED3 = 0x07,
}
impl Pd3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pd3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pd3 {
    #[inline(always)]
    fn from(val: u8) -> Pd3 {
        Pd3::from_bits(val)
    }
}
impl From<Pd3> for u8 {
    #[inline(always)]
    fn from(val: Pd3) -> u8 {
        Pd3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pd4 {
    #[doc = "GPIO"]
    GPIOD4 = 0x0,
    #[doc = "Timer B PWM4"]
    TBPWM4 = 0x01,
    #[doc = "Timer C PWM4"]
    TCPWM4 = 0x02,
    #[doc = "Timer D QEP Index"]
    TDQEPIDX = 0x03,
    #[doc = "Timer B QEP Index"]
    TBQEPIDX = 0x04,
    #[doc = "USART D SPI Clock"]
    USDSCLK = 0x05,
    #[doc = "TRACE Data 3 output"]
    TRACED3 = 0x06,
    #[doc = "USART D MOSI/TX"]
    USDMOSI = 0x07,
}
impl Pd4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pd4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pd4 {
    #[inline(always)]
    fn from(val: u8) -> Pd4 {
        Pd4::from_bits(val)
    }
}
impl From<Pd4> for u8 {
    #[inline(always)]
    fn from(val: Pd4) -> u8 {
        Pd4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pd5 {
    #[doc = "GPIO"]
    GPIOD5 = 0x0,
    #[doc = "Timer B PWM5"]
    TBPWM5 = 0x01,
    #[doc = "Timer C PWM5"]
    TCPWM5 = 0x02,
    #[doc = "Timer D QEP Phase A"]
    TDQEPPHA = 0x03,
    #[doc = "Timer D QEP Phase B"]
    TDQEPPHB = 0x04,
    #[doc = "USART D Slave Select"]
    USDSS = 0x05,
    #[doc = "CAN Rx Data"]
    CANRXD = 0x06,
    #[doc = "USART D MISO/RX"]
    USDMISO = 0x07,
}
impl Pd5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pd5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pd5 {
    #[inline(always)]
    fn from(val: u8) -> Pd5 {
        Pd5::from_bits(val)
    }
}
impl From<Pd5> for u8 {
    #[inline(always)]
    fn from(val: Pd5) -> u8 {
        Pd5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pd6 {
    #[doc = "GPIO"]
    GPIOD6 = 0x0,
    #[doc = "Timer B PWM6"]
    TBPWM6 = 0x01,
    #[doc = "Timer C PWM6"]
    TCPWM6 = 0x02,
    #[doc = "Timer D QEP Phase B"]
    TDQEPPHB = 0x03,
    #[doc = "Timer B QEP Phase B"]
    TBQEPPHB = 0x04,
    #[doc = "USART D MOSI/TX"]
    USDMOSI = 0x05,
    #[doc = "CAN Tx Data"]
    CANTXD = 0x06,
    #[doc = "I2C SDA"]
    I2CSDA = 0x07,
}
impl Pd6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pd6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pd6 {
    #[inline(always)]
    fn from(val: u8) -> Pd6 {
        Pd6::from_bits(val)
    }
}
impl From<Pd6> for u8 {
    #[inline(always)]
    fn from(val: Pd6) -> u8 {
        Pd6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pd7 {
    #[doc = "GPIO"]
    GPIOD7 = 0x0,
    #[doc = "Timer B PWM7"]
    TBPWM7 = 0x01,
    #[doc = "Timer C PWM7"]
    TCPWM7 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART C MISO/RX"]
    USDMISO = 0x05,
    #[doc = "CAN Rx Data"]
    CANRXD = 0x06,
    #[doc = "I2C SCL"]
    I2CSCL = 0x07,
}
impl Pd7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pd7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pd7 {
    #[inline(always)]
    fn from(val: u8) -> Pd7 {
        Pd7::from_bits(val)
    }
}
impl From<Pd7> for u8 {
    #[inline(always)]
    fn from(val: Pd7) -> u8 {
        Pd7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pds {
    #[doc = "6 mA"]
    DS_6MA = 0x0,
    #[doc = "8 mA"]
    DS_8MA = 0x01,
    #[doc = "11 mA"]
    DS_11MA = 0x02,
    #[doc = "14 mA"]
    DS_14MA = 0x03,
    #[doc = "17 mA"]
    DS_17MA = 0x04,
    #[doc = "20 mA"]
    DS_20MA = 0x05,
    #[doc = "22 mA"]
    DS_22MA = 0x06,
    #[doc = "25 mA"]
    DS_25MA = 0x07,
}
impl Pds {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pds {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pds {
    #[inline(always)]
    fn from(val: u8) -> Pds {
        Pds::from_bits(val)
    }
}
impl From<Pds> for u8 {
    #[inline(always)]
    fn from(val: Pds) -> u8 {
        Pds::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pe0 {
    #[doc = "GPIO"]
    GPIOE0 = 0x0,
    #[doc = "Timer C PWM4"]
    TCPWM4 = 0x01,
    #[doc = "Timer D PWM0"]
    TDPWM0 = 0x02,
    #[doc = "Timer A QEP Index"]
    TAQEPIDX = 0x03,
    #[doc = "Timer B QEP Index"]
    TBQEPIDX = 0x04,
    #[doc = "USART C SPI Clock"]
    USCSCLK = 0x05,
    #[doc = "I2C SCL"]
    I2CSCL = 0x06,
    #[doc = "EMUX Clock"]
    EMUXC = 0x07,
}
impl Pe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe0 {
    #[inline(always)]
    fn from(val: u8) -> Pe0 {
        Pe0::from_bits(val)
    }
}
impl From<Pe0> for u8 {
    #[inline(always)]
    fn from(val: Pe0) -> u8 {
        Pe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pe1 {
    #[doc = "GPIO"]
    GPIOE1 = 0x0,
    #[doc = "Timer C PWM5"]
    TCPWM5 = 0x01,
    #[doc = "Timer D PWM1"]
    TDPWM1 = 0x02,
    #[doc = "Timer A QEP Phase A"]
    TAQEPPHA = 0x03,
    #[doc = "Timer B QEP Phase A"]
    TBQEPPHA = 0x04,
    #[doc = "USART C Slave Select"]
    USCSS = 0x05,
    #[doc = "I2C SDA"]
    I2CSDA = 0x06,
    #[doc = "EMUX Data"]
    EMUXD = 0x07,
}
impl Pe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe1 {
    #[inline(always)]
    fn from(val: u8) -> Pe1 {
        Pe1::from_bits(val)
    }
}
impl From<Pe1> for u8 {
    #[inline(always)]
    fn from(val: Pe1) -> u8 {
        Pe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pe2 {
    #[doc = "GPIO"]
    GPIOE2 = 0x0,
    #[doc = "Timer C PWM6"]
    TCPWM6 = 0x01,
    #[doc = "Timer D PWM2"]
    TDPWM2 = 0x02,
    #[doc = "Timer A QEP Phase B"]
    TAQEPPHB = 0x03,
    #[doc = "Timer B QEP Phase B"]
    TBQEPPHB = 0x04,
    #[doc = "USART C MOSI/TX"]
    USCMOSI = 0x05,
    #[doc = "CAN Rx Data"]
    CANRXD = 0x06,
    #[doc = "External clock input"]
    EXTCLK = 0x07,
}
impl Pe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe2 {
    #[inline(always)]
    fn from(val: u8) -> Pe2 {
        Pe2::from_bits(val)
    }
}
impl From<Pe2> for u8 {
    #[inline(always)]
    fn from(val: Pe2) -> u8 {
        Pe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pe3 {
    #[doc = "GPIO"]
    GPIOE3 = 0x0,
    #[doc = "Timer C PWM7"]
    TCPWM7 = 0x01,
    #[doc = "Timer D PWM3"]
    TDPWM3 = 0x02,
    #[doc = "FRCLK Output"]
    FRCLK = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART C MISO/RX"]
    USCMISO = 0x05,
    #[doc = "CAN Tx Data"]
    CANTXD = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe3 {
    #[inline(always)]
    fn from(val: u8) -> Pe3 {
        Pe3::from_bits(val)
    }
}
impl From<Pe3> for u8 {
    #[inline(always)]
    fn from(val: Pe3) -> u8 {
        Pe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pe4 {
    #[doc = "GPIO"]
    GPIOE4 = 0x0,
    #[doc = "Timer C PWM4"]
    TCPWM4 = 0x01,
    #[doc = "Timer D PWM4"]
    TDPWM4 = 0x02,
    #[doc = "Timer D QEP Index"]
    TDQEPIDX = 0x03,
    #[doc = "USART B SCLK"]
    USBSCLK = 0x04,
    #[doc = "USART D MOSI/TX"]
    USDMOSI = 0x05,
    #[doc = "I2C SCL"]
    I2CSCL = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe4 {
    #[inline(always)]
    fn from(val: u8) -> Pe4 {
        Pe4::from_bits(val)
    }
}
impl From<Pe4> for u8 {
    #[inline(always)]
    fn from(val: Pe4) -> u8 {
        Pe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pe5 {
    #[doc = "GPIO"]
    GPIOE5 = 0x0,
    #[doc = "Timer C PWM5"]
    TCPWM5 = 0x01,
    #[doc = "Timer D PWM5"]
    TDPWM5 = 0x02,
    #[doc = "Timer D QEP Phase A"]
    TDQEPPHA = 0x03,
    #[doc = "USART B Slave Select"]
    USBSS = 0x04,
    #[doc = "USART D MISO/RX"]
    USDMISO = 0x05,
    #[doc = "I2C SDA"]
    I2CSDA = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe5 {
    #[inline(always)]
    fn from(val: u8) -> Pe5 {
        Pe5::from_bits(val)
    }
}
impl From<Pe5> for u8 {
    #[inline(always)]
    fn from(val: Pe5) -> u8 {
        Pe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pe6 {
    #[doc = "GPIO"]
    GPIOE6 = 0x0,
    #[doc = "Timer C PWM6"]
    TCPWM6 = 0x01,
    #[doc = "Timer D PWM6"]
    TDPWM6 = 0x02,
    #[doc = "Timer D QEP Phase B"]
    TDQEPPHB = 0x03,
    #[doc = "USART B MOSI/TX"]
    USBMOSI = 0x04,
    #[doc = "USART D SCLK"]
    USDSCLK = 0x05,
    #[doc = "CAN RX Data"]
    CANRXD = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pe6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe6 {
    #[inline(always)]
    fn from(val: u8) -> Pe6 {
        Pe6::from_bits(val)
    }
}
impl From<Pe6> for u8 {
    #[inline(always)]
    fn from(val: Pe6) -> u8 {
        Pe6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pe7 {
    #[doc = "GPIO"]
    GPIOE7 = 0x0,
    #[doc = "Timer C PWM7"]
    TCPWM7 = 0x01,
    #[doc = "Timer D PWM7"]
    TDPWM7 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "USART B MISO/RX"]
    USBMISO = 0x04,
    #[doc = "USART D Slave Select"]
    USDSS = 0x05,
    #[doc = "CAN TX Data"]
    CANTXD = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pe7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe7 {
    #[inline(always)]
    fn from(val: u8) -> Pe7 {
        Pe7::from_bits(val)
    }
}
impl From<Pe7> for u8 {
    #[inline(always)]
    fn from(val: Pe7) -> u8 {
        Pe7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pf0 {
    #[doc = "GPIO"]
    GPIOF0 = 0x0,
    #[doc = "Timer C PWM0"]
    TCPWM0 = 0x01,
    #[doc = "Timer D PWM0"]
    TDPWM0 = 0x02,
    #[doc = "TCK/SWDCLK"]
    TCK_SWDCLK = 0x03,
    #[doc = "Timer B QEP Index"]
    TBQEPIDX = 0x04,
    #[doc = "USART B SPI Clock"]
    USBSCLK = 0x05,
    #[doc = "Trace Data 2 output"]
    TRACED2 = 0x06,
    #[doc = "Trace Clock"]
    TRACECLK = 0x07,
}
impl Pf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pf0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pf0 {
    #[inline(always)]
    fn from(val: u8) -> Pf0 {
        Pf0::from_bits(val)
    }
}
impl From<Pf0> for u8 {
    #[inline(always)]
    fn from(val: Pf0) -> u8 {
        Pf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pf1 {
    #[doc = "GPIO"]
    GPIOF1 = 0x0,
    #[doc = "Timer C PWM1"]
    TCPWM1 = 0x01,
    #[doc = "Timer D PWM1"]
    TDPWM1 = 0x02,
    #[doc = "TMS/SWDIO"]
    TMS_SWDIO = 0x03,
    #[doc = "Timer B QEP Phase A"]
    TBQEPPHA = 0x04,
    #[doc = "USART B Slave Select"]
    USBSS = 0x05,
    #[doc = "Trace Data 1 output"]
    TRACED1 = 0x06,
    #[doc = "Trace Data 0 output"]
    TRACED0 = 0x07,
}
impl Pf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pf1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pf1 {
    #[inline(always)]
    fn from(val: u8) -> Pf1 {
        Pf1::from_bits(val)
    }
}
impl From<Pf1> for u8 {
    #[inline(always)]
    fn from(val: Pf1) -> u8 {
        Pf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pf2 {
    #[doc = "GPIO"]
    GPIOF2 = 0x0,
    #[doc = "Timer C PWM2"]
    TCPWM2 = 0x01,
    #[doc = "Timer D PWM2"]
    TDPWM2 = 0x02,
    #[doc = "Test Data In"]
    TDI = 0x03,
    #[doc = "Timer B QEP Phase B"]
    TBQEPPHB = 0x04,
    #[doc = "USART B MOSI/TX"]
    USBMOSI = 0x05,
    #[doc = "Trace Data 0 output"]
    TRACED0 = 0x06,
    #[doc = "Trace Data 1 output"]
    TRACED1 = 0x07,
}
impl Pf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pf2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pf2 {
    #[inline(always)]
    fn from(val: u8) -> Pf2 {
        Pf2::from_bits(val)
    }
}
impl From<Pf2> for u8 {
    #[inline(always)]
    fn from(val: Pf2) -> u8 {
        Pf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pf3 {
    #[doc = "GPIO"]
    GPIOF3 = 0x0,
    #[doc = "Timer C PWM3"]
    TCPWM3 = 0x01,
    #[doc = "Timer D PWM3"]
    TDPWM3 = 0x02,
    #[doc = "Test Data Out"]
    TDO = 0x03,
    #[doc = "FRCLK Output"]
    FRCLK = 0x04,
    #[doc = "USART B MISO/RX"]
    USBMISO = 0x05,
    #[doc = "Trace Clock"]
    TRACECLK = 0x06,
    #[doc = "Trace Data 2 output"]
    TRACED2 = 0x07,
}
impl Pf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pf3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pf3 {
    #[inline(always)]
    fn from(val: u8) -> Pf3 {
        Pf3::from_bits(val)
    }
}
impl From<Pf3> for u8 {
    #[inline(always)]
    fn from(val: Pf3) -> u8 {
        Pf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pf4 {
    #[doc = "GPIO"]
    GPIOF4 = 0x0,
    #[doc = "Timer C PWM4"]
    TCPWM4 = 0x01,
    #[doc = "Timer D PWM4"]
    TDPWM4 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Timer C QEP Index"]
    TCQEPIDX = 0x04,
    #[doc = "USART D SCLK"]
    USDSCLK = 0x05,
    #[doc = "Trace Data 3 output"]
    TRACED3 = 0x06,
    #[doc = "EMUX Clock"]
    EMUXC = 0x07,
}
impl Pf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pf4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pf4 {
    #[inline(always)]
    fn from(val: u8) -> Pf4 {
        Pf4::from_bits(val)
    }
}
impl From<Pf4> for u8 {
    #[inline(always)]
    fn from(val: Pf4) -> u8 {
        Pf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pf5 {
    #[doc = "GPIO"]
    GPIOF5 = 0x0,
    #[doc = "Timer C PWM5"]
    TCPWM5 = 0x01,
    #[doc = "Timer D PWM5"]
    TDPWM5 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Timer C QEP Phase A"]
    TCQEPPHA = 0x04,
    #[doc = "USART D Slave Select"]
    USDSS = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "EMUX Data"]
    EMUXD = 0x07,
}
impl Pf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pf5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pf5 {
    #[inline(always)]
    fn from(val: u8) -> Pf5 {
        Pf5::from_bits(val)
    }
}
impl From<Pf5> for u8 {
    #[inline(always)]
    fn from(val: Pf5) -> u8 {
        Pf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pf6 {
    #[doc = "GPIO"]
    GPIOF6 = 0x0,
    #[doc = "Timer C PWM6"]
    TCPWM6 = 0x01,
    #[doc = "Timer D PWM6"]
    TDPWM6 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Timer C QEP Phase B"]
    TCQEPPHB = 0x04,
    #[doc = "USART D MOSI/TX"]
    USDMOSI = 0x05,
    #[doc = "CAN RX Data"]
    CANRXD = 0x06,
    #[doc = "I2C SCL"]
    I2CSCL = 0x07,
}
impl Pf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pf6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pf6 {
    #[inline(always)]
    fn from(val: u8) -> Pf6 {
        Pf6::from_bits(val)
    }
}
impl From<Pf6> for u8 {
    #[inline(always)]
    fn from(val: Pf6) -> u8 {
        Pf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pf7 {
    #[doc = "GPIO"]
    GPIOF7 = 0x0,
    #[doc = "Timer C PWM7"]
    TCPWM7 = 0x01,
    #[doc = "Timer D PWM7"]
    TDPWM7 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART D MISO/RX"]
    USDMISO = 0x05,
    #[doc = "CAN TX Data"]
    CANTXD = 0x06,
    #[doc = "I2C SDA"]
    I2CSDA = 0x07,
}
impl Pf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pf7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pf7 {
    #[inline(always)]
    fn from(val: u8) -> Pf7 {
        Pf7::from_bits(val)
    }
}
impl From<Pf7> for u8 {
    #[inline(always)]
    fn from(val: Pf7) -> u8 {
        Pf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pg0 {
    #[doc = "GPIO"]
    GPIOG0 = 0x0,
    #[doc = "Timer C PWM0"]
    TCPWM0 = 0x01,
    #[doc = "Timer D PWM0"]
    TDPWM0 = 0x02,
    #[doc = "EMUX Clock"]
    EMUXC = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART D SPI Clock"]
    USDSCLK = 0x05,
    #[doc = "Trace Clock"]
    TRACECLK = 0x06,
    #[doc = "Timer C QEP Index"]
    TCQEPIDX = 0x07,
}
impl Pg0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pg0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pg0 {
    #[inline(always)]
    fn from(val: u8) -> Pg0 {
        Pg0::from_bits(val)
    }
}
impl From<Pg0> for u8 {
    #[inline(always)]
    fn from(val: Pg0) -> u8 {
        Pg0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pg1 {
    #[doc = "GPIO"]
    GPIOG1 = 0x0,
    #[doc = "Timer C PWM1"]
    TCPWM1 = 0x01,
    #[doc = "Timer D PWM1"]
    TDPWM1 = 0x02,
    #[doc = "EMUX Data"]
    EMUXD = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART D Slave Select"]
    USDSS = 0x05,
    #[doc = "Trace Data 0 output"]
    TRACED0 = 0x06,
    #[doc = "Timer C QEP Phase A"]
    TCQEPPHA = 0x07,
}
impl Pg1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pg1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pg1 {
    #[inline(always)]
    fn from(val: u8) -> Pg1 {
        Pg1::from_bits(val)
    }
}
impl From<Pg1> for u8 {
    #[inline(always)]
    fn from(val: Pg1) -> u8 {
        Pg1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pg2 {
    #[doc = "GPIO"]
    GPIOG2 = 0x0,
    #[doc = "Timer C PWM2"]
    TCPWM2 = 0x01,
    #[doc = "Timer D PWM2"]
    TDPWM2 = 0x02,
    #[doc = "FRCLK Output"]
    FRCLK = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART D MOSI/TX"]
    USDMOSI = 0x05,
    #[doc = "Trace Data 1 output"]
    TRACED1 = 0x06,
    #[doc = "Timer C QEP Phase B"]
    TCQEPPHB = 0x07,
}
impl Pg2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pg2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pg2 {
    #[inline(always)]
    fn from(val: u8) -> Pg2 {
        Pg2::from_bits(val)
    }
}
impl From<Pg2> for u8 {
    #[inline(always)]
    fn from(val: Pg2) -> u8 {
        Pg2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pg3 {
    #[doc = "GPIO"]
    GPIOG3 = 0x0,
    #[doc = "Timer C PWM3"]
    TCPWM3 = 0x01,
    #[doc = "Timer D PWM3"]
    TDPWM3 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART D MISO/RX"]
    USDMISO = 0x05,
    #[doc = "Trace Data 2 output"]
    TRACED2 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pg3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pg3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pg3 {
    #[inline(always)]
    fn from(val: u8) -> Pg3 {
        Pg3::from_bits(val)
    }
}
impl From<Pg3> for u8 {
    #[inline(always)]
    fn from(val: Pg3) -> u8 {
        Pg3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pg4 {
    #[doc = "GPIO"]
    GPIOG4 = 0x0,
    #[doc = "Timer C PWM4"]
    TCPWM4 = 0x01,
    #[doc = "Timer D PWM4"]
    TDPWM4 = 0x02,
    #[doc = "EMUX Data"]
    EMUXD = 0x03,
    #[doc = "I2C SCL"]
    I2CSCL = 0x04,
    #[doc = "USART D Slave Select"]
    USDSS = 0x05,
    #[doc = "Trace Data 3 output"]
    TRACED3 = 0x06,
    #[doc = "Timer D QEP Index"]
    TDQEPIDX = 0x07,
}
impl Pg4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pg4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pg4 {
    #[inline(always)]
    fn from(val: u8) -> Pg4 {
        Pg4::from_bits(val)
    }
}
impl From<Pg4> for u8 {
    #[inline(always)]
    fn from(val: Pg4) -> u8 {
        Pg4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pg5 {
    #[doc = "GPIO"]
    GPIOG5 = 0x0,
    #[doc = "Timer C PWM5"]
    TCPWM5 = 0x01,
    #[doc = "Timer D PWM5"]
    TDPWM5 = 0x02,
    #[doc = "EMUX Clock"]
    EMUXC = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART D MOSI/TX"]
    USDMOSI = 0x05,
    #[doc = "CAN RX Data"]
    CANRXD = 0x06,
    #[doc = "Timer D QEP Phase A"]
    TDQEPPHA = 0x07,
}
impl Pg5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pg5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pg5 {
    #[inline(always)]
    fn from(val: u8) -> Pg5 {
        Pg5::from_bits(val)
    }
}
impl From<Pg5> for u8 {
    #[inline(always)]
    fn from(val: Pg5) -> u8 {
        Pg5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pg6 {
    #[doc = "GPIO"]
    GPIOG6 = 0x0,
    #[doc = "Timer C PWM6"]
    TCPWM6 = 0x01,
    #[doc = "Timer D PWM6"]
    TDPWM6 = 0x02,
    #[doc = "I2C SDA"]
    I2CSDA = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART D MISO/RX"]
    USDMISO = 0x05,
    #[doc = "CAN TX Data"]
    CANTXD = 0x06,
    #[doc = "Timer D QEP Phase B"]
    TDQEPPHB = 0x07,
}
impl Pg6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pg6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pg6 {
    #[inline(always)]
    fn from(val: u8) -> Pg6 {
        Pg6::from_bits(val)
    }
}
impl From<Pg6> for u8 {
    #[inline(always)]
    fn from(val: Pg6) -> u8 {
        Pg6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pg7 {
    #[doc = "GPIO"]
    GPIOG7 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Timer D QEP Index"]
    TDQEPIDX = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "USART D SPI Clock"]
    USDSCLK = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pg7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pg7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pg7 {
    #[inline(always)]
    fn from(val: u8) -> Pg7 {
        Pg7::from_bits(val)
    }
}
impl From<Pg7> for u8 {
    #[inline(always)]
    fn from(val: Pg7) -> u8 {
        Pg7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Plloutdiv {
    #[doc = "/1"]
    DIV1 = 0x0,
    #[doc = "/2"]
    DIV2 = 0x01,
    #[doc = "/4"]
    DIV4 = 0x02,
    #[doc = "/8"]
    DIV8 = 0x03,
}
impl Plloutdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plloutdiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plloutdiv {
    #[inline(always)]
    fn from(val: u8) -> Plloutdiv {
        Plloutdiv::from_bits(val)
    }
}
impl From<Plloutdiv> for u8 {
    #[inline(always)]
    fn from(val: Plloutdiv) -> u8 {
        Plloutdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sclkmuxsel {
    #[doc = "FRCLK selected as input to SCLK"]
    FRCLK = 0x0,
    #[doc = "PLLCLK selected as input to SCLK"]
    PLLCLK = 0x01,
}
impl Sclkmuxsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sclkmuxsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sclkmuxsel {
    #[inline(always)]
    fn from(val: u8) -> Sclkmuxsel {
        Sclkmuxsel::from_bits(val)
    }
}
impl From<Sclkmuxsel> for u8 {
    #[inline(always)]
    fn from(val: Sclkmuxsel) -> u8 {
        Sclkmuxsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usmode {
    #[doc = "SSP Mode"]
    SSP = 0x0,
    #[doc = "UART Mode"]
    UART = 0x01,
}
impl Usmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usmode {
    #[inline(always)]
    fn from(val: u8) -> Usmode {
        Usmode::from_bits(val)
    }
}
impl From<Usmode> for u8 {
    #[inline(always)]
    fn from(val: Usmode) -> u8 {
        Usmode::to_bits(val)
    }
}
