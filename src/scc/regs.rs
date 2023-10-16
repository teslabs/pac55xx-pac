#[doc = "Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccsctl(pub u32);
impl Ccsctl {
    #[doc = "FRCLK MUX Select"]
    #[inline(always)]
    pub const fn frclkmuxsel(&self) -> super::vals::Frclkmuxsel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Frclkmuxsel::from_bits(val as u8)
    }
    #[doc = "FRCLK MUX Select"]
    #[inline(always)]
    pub fn set_frclkmuxsel(&mut self, val: super::vals::Frclkmuxsel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "ROSC Enable"]
    #[inline(always)]
    pub const fn roscen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ROSC Enable"]
    #[inline(always)]
    pub fn set_roscen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SCLK Mux Select"]
    #[inline(always)]
    pub const fn sclkmuxsel(&self) -> super::vals::Sclkmuxsel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sclkmuxsel::from_bits(val as u8)
    }
    #[doc = "SCLK Mux Select"]
    #[inline(always)]
    pub fn set_sclkmuxsel(&mut self, val: super::vals::Sclkmuxsel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Clock Fail Enable"]
    #[inline(always)]
    pub const fn clkfailen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Fail Enable"]
    #[inline(always)]
    pub fn set_clkfailen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock Fail Mux Select"]
    #[inline(always)]
    pub const fn clkfailmuxsel(&self) -> super::vals::Clkfailmuxsel {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Clkfailmuxsel::from_bits(val as u8)
    }
    #[doc = "Clock Fail Mux Select"]
    #[inline(always)]
    pub fn set_clkfailmuxsel(&mut self, val: super::vals::Clkfailmuxsel) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Clock Fail Interrupt Flag"]
    #[inline(always)]
    pub const fn clkfailif(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Fail Interrupt Flag"]
    #[inline(always)]
    pub fn set_clkfailif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "1.8V LDO Enable"]
    #[inline(always)]
    pub const fn ldoen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1.8V LDO Enable"]
    #[inline(always)]
    pub fn set_ldoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn swreset(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub fn set_swreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PCLK Enable"]
    #[inline(always)]
    pub const fn pclken(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PCLK Enable"]
    #[inline(always)]
    pub fn set_pclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "ACLK Enable"]
    #[inline(always)]
    pub const fn aclken(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "ACLK Enable"]
    #[inline(always)]
    pub fn set_aclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ADCCLK Enable"]
    #[inline(always)]
    pub const fn adclken(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ADCCLK Enable"]
    #[inline(always)]
    pub fn set_adclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SysTick Clock Sleep Enable"]
    #[inline(always)]
    pub const fn stclkslpen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SysTick Clock Sleep Enable"]
    #[inline(always)]
    pub fn set_stclkslpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PCLK Divider"]
    #[inline(always)]
    pub const fn pclkdiv(&self) -> super::vals::Pclkdiv {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Pclkdiv::from_bits(val as u8)
    }
    #[doc = "PCLK Divider"]
    #[inline(always)]
    pub fn set_pclkdiv(&mut self, val: super::vals::Pclkdiv) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "ACLK Divider"]
    #[inline(always)]
    pub const fn aclkdiv(&self) -> super::vals::Aclkdiv {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Aclkdiv::from_bits(val as u8)
    }
    #[doc = "ACLK Divider"]
    #[inline(always)]
    pub fn set_aclkdiv(&mut self, val: super::vals::Aclkdiv) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "HCLK Divider"]
    #[inline(always)]
    pub const fn hclkdiv(&self) -> super::vals::Hclkdiv {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Hclkdiv::from_bits(val as u8)
    }
    #[doc = "HCLK Divider"]
    #[inline(always)]
    pub fn set_hclkdiv(&mut self, val: super::vals::Hclkdiv) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "USART A Mode"]
    #[inline(always)]
    pub const fn usamode(&self) -> super::vals::Usmode {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Usmode::from_bits(val as u8)
    }
    #[doc = "USART A Mode"]
    #[inline(always)]
    pub fn set_usamode(&mut self, val: super::vals::Usmode) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "USART B Mode"]
    #[inline(always)]
    pub const fn usbmode(&self) -> super::vals::Usmode {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Usmode::from_bits(val as u8)
    }
    #[doc = "USART B Mode"]
    #[inline(always)]
    pub fn set_usbmode(&mut self, val: super::vals::Usmode) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "USART C Mode"]
    #[inline(always)]
    pub const fn uscmode(&self) -> super::vals::Usmode {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Usmode::from_bits(val as u8)
    }
    #[doc = "USART C Mode"]
    #[inline(always)]
    pub fn set_uscmode(&mut self, val: super::vals::Usmode) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "USART D Mode"]
    #[inline(always)]
    pub const fn usdmode(&self) -> super::vals::Usmode {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Usmode::from_bits(val as u8)
    }
    #[doc = "USART D Mode"]
    #[inline(always)]
    pub fn set_usdmode(&mut self, val: super::vals::Usmode) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ccsctl {
    #[inline(always)]
    fn default() -> Ccsctl {
        Ccsctl(0)
    }
}
#[doc = "PLL Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccspllctl(pub u32);
impl Ccspllctl {
    #[doc = "PLL Enable"]
    #[inline(always)]
    pub const fn pllen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Enable"]
    #[inline(always)]
    pub fn set_pllen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PLL Bypass"]
    #[inline(always)]
    pub const fn pllbp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Bypass"]
    #[inline(always)]
    pub fn set_pllbp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PLL Output Divider"]
    #[inline(always)]
    pub const fn plloutdiv(&self) -> super::vals::Plloutdiv {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Plloutdiv::from_bits(val as u8)
    }
    #[doc = "PLL Output Divider"]
    #[inline(always)]
    pub fn set_plloutdiv(&mut self, val: super::vals::Plloutdiv) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "PLL Input Divider"]
    #[inline(always)]
    pub const fn pllindiv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PLL Input Divider"]
    #[inline(always)]
    pub fn set_pllindiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "PLL Feedback Divider"]
    #[inline(always)]
    pub const fn pllfbdiv(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x3fff;
        val as u16
    }
    #[doc = "PLL Feedback Divider"]
    #[inline(always)]
    pub fn set_pllfbdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 8usize)) | (((val as u32) & 0x3fff) << 8usize);
    }
    #[doc = "PLL Lock Status"]
    #[inline(always)]
    pub const fn plllock(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Lock Status"]
    #[inline(always)]
    pub fn set_plllock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Ccspllctl {
    #[inline(always)]
    fn default() -> Ccspllctl {
        Ccspllctl(0)
    }
}
#[doc = "ROSC Trim Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccsrosctrim(pub u32);
impl Ccsrosctrim {
    #[doc = "ROSC Trim Value"]
    #[inline(always)]
    pub const fn trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "ROSC Trim Value"]
    #[inline(always)]
    pub fn set_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ccsrosctrim {
    #[inline(always)]
    fn default() -> Ccsrosctrim {
        Ccsrosctrim(0)
    }
}
#[doc = "PA Peripheral MUX Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pamuxsel(pub u32);
impl Pamuxsel {
    #[doc = "PA0 MUX Select"]
    #[inline(always)]
    pub const fn pa0(&self) -> super::vals::Pa0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Pa0::from_bits(val as u8)
    }
    #[doc = "PA0 MUX Select"]
    #[inline(always)]
    pub fn set_pa0(&mut self, val: super::vals::Pa0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "PA1 MUX Select"]
    #[inline(always)]
    pub const fn pa1(&self) -> super::vals::Pa1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Pa1::from_bits(val as u8)
    }
    #[doc = "PA1 MUX Select"]
    #[inline(always)]
    pub fn set_pa1(&mut self, val: super::vals::Pa1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "PA2 MUX Select"]
    #[inline(always)]
    pub const fn pa2(&self) -> super::vals::Pa2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Pa2::from_bits(val as u8)
    }
    #[doc = "PA2 MUX Select"]
    #[inline(always)]
    pub fn set_pa2(&mut self, val: super::vals::Pa2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "PA3 MUX Select"]
    #[inline(always)]
    pub const fn pa3(&self) -> super::vals::Pa3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Pa3::from_bits(val as u8)
    }
    #[doc = "PA3 MUX Select"]
    #[inline(always)]
    pub fn set_pa3(&mut self, val: super::vals::Pa3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "PA4 MUX Select"]
    #[inline(always)]
    pub const fn pa4(&self) -> super::vals::Pa4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Pa4::from_bits(val as u8)
    }
    #[doc = "PA4 MUX Select"]
    #[inline(always)]
    pub fn set_pa4(&mut self, val: super::vals::Pa4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "PA5 MUX Select"]
    #[inline(always)]
    pub const fn pa5(&self) -> super::vals::Pa5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Pa5::from_bits(val as u8)
    }
    #[doc = "PA5 MUX Select"]
    #[inline(always)]
    pub fn set_pa5(&mut self, val: super::vals::Pa5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "PA6 MUX Select"]
    #[inline(always)]
    pub const fn pa6(&self) -> super::vals::Pa6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Pa6::from_bits(val as u8)
    }
    #[doc = "PA6 MUX Select"]
    #[inline(always)]
    pub fn set_pa6(&mut self, val: super::vals::Pa6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "PA7 MUX Select"]
    #[inline(always)]
    pub const fn pa7(&self) -> super::vals::Pa7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Pa7::from_bits(val as u8)
    }
    #[doc = "PA7 MUX Select"]
    #[inline(always)]
    pub fn set_pa7(&mut self, val: super::vals::Pa7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for Pamuxsel {
    #[inline(always)]
    fn default() -> Pamuxsel {
        Pamuxsel(0)
    }
}
#[doc = "PB Peripheral MUX Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pbmuxsel(pub u32);
impl Pbmuxsel {
    #[doc = "PB0 MUX Select"]
    #[inline(always)]
    pub const fn pb0(&self) -> super::vals::Pb0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Pb0::from_bits(val as u8)
    }
    #[doc = "PB0 MUX Select"]
    #[inline(always)]
    pub fn set_pb0(&mut self, val: super::vals::Pb0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "PB1 MUX Select"]
    #[inline(always)]
    pub const fn pb1(&self) -> super::vals::Pb1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Pb1::from_bits(val as u8)
    }
    #[doc = "PB1 MUX Select"]
    #[inline(always)]
    pub fn set_pb1(&mut self, val: super::vals::Pb1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "PB2 MUX Select"]
    #[inline(always)]
    pub const fn pb2(&self) -> super::vals::Pb2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Pb2::from_bits(val as u8)
    }
    #[doc = "PB2 MUX Select"]
    #[inline(always)]
    pub fn set_pb2(&mut self, val: super::vals::Pb2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "PB3 MUX Select"]
    #[inline(always)]
    pub const fn pb3(&self) -> super::vals::Pb3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Pb3::from_bits(val as u8)
    }
    #[doc = "PB3 MUX Select"]
    #[inline(always)]
    pub fn set_pb3(&mut self, val: super::vals::Pb3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "PB4 MUX Select"]
    #[inline(always)]
    pub const fn pb4(&self) -> super::vals::Pb4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Pb4::from_bits(val as u8)
    }
    #[doc = "PB4 MUX Select"]
    #[inline(always)]
    pub fn set_pb4(&mut self, val: super::vals::Pb4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "PB5 MUX Select"]
    #[inline(always)]
    pub const fn pb5(&self) -> super::vals::Pb5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Pb5::from_bits(val as u8)
    }
    #[doc = "PB5 MUX Select"]
    #[inline(always)]
    pub fn set_pb5(&mut self, val: super::vals::Pb5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "PB6 MUX Select"]
    #[inline(always)]
    pub const fn pb6(&self) -> super::vals::Pb6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Pb6::from_bits(val as u8)
    }
    #[doc = "PB6 MUX Select"]
    #[inline(always)]
    pub fn set_pb6(&mut self, val: super::vals::Pb6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "PB7 MUX Select"]
    #[inline(always)]
    pub const fn pb7(&self) -> super::vals::Pb7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Pb7::from_bits(val as u8)
    }
    #[doc = "PB7 MUX Select"]
    #[inline(always)]
    pub fn set_pb7(&mut self, val: super::vals::Pb7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for Pbmuxsel {
    #[inline(always)]
    fn default() -> Pbmuxsel {
        Pbmuxsel(0)
    }
}
#[doc = "PC Peripheral MUX Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcmuxsel(pub u32);
impl Pcmuxsel {
    #[doc = "PC0 MUX Select"]
    #[inline(always)]
    pub const fn pc0(&self) -> super::vals::Pc0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Pc0::from_bits(val as u8)
    }
    #[doc = "PC0 MUX Select"]
    #[inline(always)]
    pub fn set_pc0(&mut self, val: super::vals::Pc0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "PC1 MUX Select"]
    #[inline(always)]
    pub const fn pc1(&self) -> super::vals::Pc1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Pc1::from_bits(val as u8)
    }
    #[doc = "PC1 MUX Select"]
    #[inline(always)]
    pub fn set_pc1(&mut self, val: super::vals::Pc1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "PC2 MUX Select"]
    #[inline(always)]
    pub const fn pc2(&self) -> super::vals::Pc2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Pc2::from_bits(val as u8)
    }
    #[doc = "PC2 MUX Select"]
    #[inline(always)]
    pub fn set_pc2(&mut self, val: super::vals::Pc2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "PC3 MUX Select"]
    #[inline(always)]
    pub const fn pc3(&self) -> super::vals::Pc3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Pc3::from_bits(val as u8)
    }
    #[doc = "PC3 MUX Select"]
    #[inline(always)]
    pub fn set_pc3(&mut self, val: super::vals::Pc3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "PC4 MUX Select"]
    #[inline(always)]
    pub const fn pc4(&self) -> super::vals::Pc4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Pc4::from_bits(val as u8)
    }
    #[doc = "PC4 MUX Select"]
    #[inline(always)]
    pub fn set_pc4(&mut self, val: super::vals::Pc4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "PC5 MUX Select"]
    #[inline(always)]
    pub const fn pc5(&self) -> super::vals::Pc5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Pc5::from_bits(val as u8)
    }
    #[doc = "PC5 MUX Select"]
    #[inline(always)]
    pub fn set_pc5(&mut self, val: super::vals::Pc5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "PC6 MUX Select"]
    #[inline(always)]
    pub const fn pc6(&self) -> super::vals::Pc6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Pc6::from_bits(val as u8)
    }
    #[doc = "PC6 MUX Select"]
    #[inline(always)]
    pub fn set_pc6(&mut self, val: super::vals::Pc6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "PC7 MUX Select"]
    #[inline(always)]
    pub const fn pc7(&self) -> super::vals::Pc7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Pc7::from_bits(val as u8)
    }
    #[doc = "PC7 MUX Select"]
    #[inline(always)]
    pub fn set_pc7(&mut self, val: super::vals::Pc7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for Pcmuxsel {
    #[inline(always)]
    fn default() -> Pcmuxsel {
        Pcmuxsel(0)
    }
}
#[doc = "PD Peripheral MUX Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdmuxsel(pub u32);
impl Pdmuxsel {
    #[doc = "PD0 MUX Select"]
    #[inline(always)]
    pub const fn pd0(&self) -> super::vals::Pd0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Pd0::from_bits(val as u8)
    }
    #[doc = "PD0 MUX Select"]
    #[inline(always)]
    pub fn set_pd0(&mut self, val: super::vals::Pd0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "PD1 MUX Select"]
    #[inline(always)]
    pub const fn pd1(&self) -> super::vals::Pd1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Pd1::from_bits(val as u8)
    }
    #[doc = "PD1 MUX Select"]
    #[inline(always)]
    pub fn set_pd1(&mut self, val: super::vals::Pd1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "PD2 MUX Select"]
    #[inline(always)]
    pub const fn pd2(&self) -> super::vals::Pd2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Pd2::from_bits(val as u8)
    }
    #[doc = "PD2 MUX Select"]
    #[inline(always)]
    pub fn set_pd2(&mut self, val: super::vals::Pd2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "PD3 MUX Select"]
    #[inline(always)]
    pub const fn pd3(&self) -> super::vals::Pd3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Pd3::from_bits(val as u8)
    }
    #[doc = "PD3 MUX Select"]
    #[inline(always)]
    pub fn set_pd3(&mut self, val: super::vals::Pd3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "PD4 MUX Select"]
    #[inline(always)]
    pub const fn pd4(&self) -> super::vals::Pd4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Pd4::from_bits(val as u8)
    }
    #[doc = "PD4 MUX Select"]
    #[inline(always)]
    pub fn set_pd4(&mut self, val: super::vals::Pd4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "PD5 MUX Select"]
    #[inline(always)]
    pub const fn pd5(&self) -> super::vals::Pd5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Pd5::from_bits(val as u8)
    }
    #[doc = "PD5 MUX Select"]
    #[inline(always)]
    pub fn set_pd5(&mut self, val: super::vals::Pd5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "PD6 MUX Select"]
    #[inline(always)]
    pub const fn pd6(&self) -> super::vals::Pd6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Pd6::from_bits(val as u8)
    }
    #[doc = "PD6 MUX Select"]
    #[inline(always)]
    pub fn set_pd6(&mut self, val: super::vals::Pd6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "PD7 MUX Select"]
    #[inline(always)]
    pub const fn pd7(&self) -> super::vals::Pd7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Pd7::from_bits(val as u8)
    }
    #[doc = "PD7 MUX Select"]
    #[inline(always)]
    pub fn set_pd7(&mut self, val: super::vals::Pd7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for Pdmuxsel {
    #[inline(always)]
    fn default() -> Pdmuxsel {
        Pdmuxsel(0)
    }
}
#[doc = "Drive Strength/Schmitt Trigger"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pds(pub u32);
impl Pds {
    #[doc = "Drive Strength"]
    #[inline(always)]
    pub const fn pds(&self, n: usize) -> super::vals::Pds {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x07;
        super::vals::Pds::from_bits(val as u8)
    }
    #[doc = "Drive Strength"]
    #[inline(always)]
    pub fn set_pds(&mut self, n: usize, val: super::vals::Pds) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
    }
    #[doc = "Schmitt Trigger Enable"]
    #[inline(always)]
    pub const fn pst(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Schmitt Trigger Enable"]
    #[inline(always)]
    pub fn set_pst(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pds {
    #[inline(always)]
    fn default() -> Pds {
        Pds(0)
    }
}
#[doc = "PE Peripheral MUX Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pemuxsel(pub u32);
impl Pemuxsel {
    #[doc = "PE0 MUX Select"]
    #[inline(always)]
    pub const fn pe0(&self) -> super::vals::Pe0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Pe0::from_bits(val as u8)
    }
    #[doc = "PE0 MUX Select"]
    #[inline(always)]
    pub fn set_pe0(&mut self, val: super::vals::Pe0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "PE1 MUX Select"]
    #[inline(always)]
    pub const fn pe1(&self) -> super::vals::Pe1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Pe1::from_bits(val as u8)
    }
    #[doc = "PE1 MUX Select"]
    #[inline(always)]
    pub fn set_pe1(&mut self, val: super::vals::Pe1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "PE2 MUX Select"]
    #[inline(always)]
    pub const fn pe2(&self) -> super::vals::Pe2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Pe2::from_bits(val as u8)
    }
    #[doc = "PE2 MUX Select"]
    #[inline(always)]
    pub fn set_pe2(&mut self, val: super::vals::Pe2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "PE3 MUX Select"]
    #[inline(always)]
    pub const fn pe3(&self) -> super::vals::Pe3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Pe3::from_bits(val as u8)
    }
    #[doc = "PE3 MUX Select"]
    #[inline(always)]
    pub fn set_pe3(&mut self, val: super::vals::Pe3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "PE4 MUX Select"]
    #[inline(always)]
    pub const fn pe4(&self) -> super::vals::Pe4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Pe4::from_bits(val as u8)
    }
    #[doc = "PE4 MUX Select"]
    #[inline(always)]
    pub fn set_pe4(&mut self, val: super::vals::Pe4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "PE5 MUX Select"]
    #[inline(always)]
    pub const fn pe5(&self) -> super::vals::Pe5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Pe5::from_bits(val as u8)
    }
    #[doc = "PE5 MUX Select"]
    #[inline(always)]
    pub fn set_pe5(&mut self, val: super::vals::Pe5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "PE6 MUX Select"]
    #[inline(always)]
    pub const fn pe6(&self) -> super::vals::Pe6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Pe6::from_bits(val as u8)
    }
    #[doc = "PE6 MUX Select"]
    #[inline(always)]
    pub fn set_pe6(&mut self, val: super::vals::Pe6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "PE7 MUX Select"]
    #[inline(always)]
    pub const fn pe7(&self) -> super::vals::Pe7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Pe7::from_bits(val as u8)
    }
    #[doc = "PE7 MUX Select"]
    #[inline(always)]
    pub fn set_pe7(&mut self, val: super::vals::Pe7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for Pemuxsel {
    #[inline(always)]
    fn default() -> Pemuxsel {
        Pemuxsel(0)
    }
}
#[doc = "PF Peripheral MUX Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfmuxsel(pub u32);
impl Pfmuxsel {
    #[doc = "PF0 MUX Select"]
    #[inline(always)]
    pub const fn pf0(&self) -> super::vals::Pf0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Pf0::from_bits(val as u8)
    }
    #[doc = "PF0 MUX Select"]
    #[inline(always)]
    pub fn set_pf0(&mut self, val: super::vals::Pf0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "PF1 MUX Select"]
    #[inline(always)]
    pub const fn pf1(&self) -> super::vals::Pf1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Pf1::from_bits(val as u8)
    }
    #[doc = "PF1 MUX Select"]
    #[inline(always)]
    pub fn set_pf1(&mut self, val: super::vals::Pf1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "PF2 MUX Select"]
    #[inline(always)]
    pub const fn pf2(&self) -> super::vals::Pf2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Pf2::from_bits(val as u8)
    }
    #[doc = "PF2 MUX Select"]
    #[inline(always)]
    pub fn set_pf2(&mut self, val: super::vals::Pf2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "PF3 MUX Select"]
    #[inline(always)]
    pub const fn pf3(&self) -> super::vals::Pf3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Pf3::from_bits(val as u8)
    }
    #[doc = "PF3 MUX Select"]
    #[inline(always)]
    pub fn set_pf3(&mut self, val: super::vals::Pf3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "PF4 MUX Select"]
    #[inline(always)]
    pub const fn pf4(&self) -> super::vals::Pf4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Pf4::from_bits(val as u8)
    }
    #[doc = "PF4 MUX Select"]
    #[inline(always)]
    pub fn set_pf4(&mut self, val: super::vals::Pf4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "PF5 MUX Select"]
    #[inline(always)]
    pub const fn pf5(&self) -> super::vals::Pf5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Pf5::from_bits(val as u8)
    }
    #[doc = "PF5 MUX Select"]
    #[inline(always)]
    pub fn set_pf5(&mut self, val: super::vals::Pf5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "PF6 MUX Select"]
    #[inline(always)]
    pub const fn pf6(&self) -> super::vals::Pf6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Pf6::from_bits(val as u8)
    }
    #[doc = "PF6 MUX Select"]
    #[inline(always)]
    pub fn set_pf6(&mut self, val: super::vals::Pf6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "PF7 MUX Select"]
    #[inline(always)]
    pub const fn pf7(&self) -> super::vals::Pf7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Pf7::from_bits(val as u8)
    }
    #[doc = "PF7 MUX Select"]
    #[inline(always)]
    pub fn set_pf7(&mut self, val: super::vals::Pf7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for Pfmuxsel {
    #[inline(always)]
    fn default() -> Pfmuxsel {
        Pfmuxsel(0)
    }
}
#[doc = "PG Peripheral MUX Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pgmuxsel(pub u32);
impl Pgmuxsel {
    #[doc = "PG0 MUX Select"]
    #[inline(always)]
    pub const fn pg0(&self) -> super::vals::Pg0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Pg0::from_bits(val as u8)
    }
    #[doc = "PG0 MUX Select"]
    #[inline(always)]
    pub fn set_pg0(&mut self, val: super::vals::Pg0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "PG1 MUX Select"]
    #[inline(always)]
    pub const fn pg1(&self) -> super::vals::Pg1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Pg1::from_bits(val as u8)
    }
    #[doc = "PG1 MUX Select"]
    #[inline(always)]
    pub fn set_pg1(&mut self, val: super::vals::Pg1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "PG2 MUX Select"]
    #[inline(always)]
    pub const fn pg2(&self) -> super::vals::Pg2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Pg2::from_bits(val as u8)
    }
    #[doc = "PG2 MUX Select"]
    #[inline(always)]
    pub fn set_pg2(&mut self, val: super::vals::Pg2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "PG3 MUX Select"]
    #[inline(always)]
    pub const fn pg3(&self) -> super::vals::Pg3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Pg3::from_bits(val as u8)
    }
    #[doc = "PG3 MUX Select"]
    #[inline(always)]
    pub fn set_pg3(&mut self, val: super::vals::Pg3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "PG4 MUX Select"]
    #[inline(always)]
    pub const fn pg4(&self) -> super::vals::Pg4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Pg4::from_bits(val as u8)
    }
    #[doc = "PG4 MUX Select"]
    #[inline(always)]
    pub fn set_pg4(&mut self, val: super::vals::Pg4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "PG5 MUX Select"]
    #[inline(always)]
    pub const fn pg5(&self) -> super::vals::Pg5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Pg5::from_bits(val as u8)
    }
    #[doc = "PG5 MUX Select"]
    #[inline(always)]
    pub fn set_pg5(&mut self, val: super::vals::Pg5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "PG6 MUX Select"]
    #[inline(always)]
    pub const fn pg6(&self) -> super::vals::Pg6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Pg6::from_bits(val as u8)
    }
    #[doc = "PG6 MUX Select"]
    #[inline(always)]
    pub fn set_pg6(&mut self, val: super::vals::Pg6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "PG7 MUX Select"]
    #[inline(always)]
    pub const fn pg7(&self) -> super::vals::Pg7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Pg7::from_bits(val as u8)
    }
    #[doc = "PG7 MUX Select"]
    #[inline(always)]
    pub fn set_pg7(&mut self, val: super::vals::Pg7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for Pgmuxsel {
    #[inline(always)]
    fn default() -> Pgmuxsel {
        Pgmuxsel(0)
    }
}
#[doc = "Weak Pull-down Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppden(pub u32);
impl Ppden {
    #[doc = "Weak Pull-down Enable"]
    #[inline(always)]
    pub const fn p(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Weak Pull-down Enable"]
    #[inline(always)]
    pub fn set_p(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Ppden {
    #[inline(always)]
    fn default() -> Ppden {
        Ppden(0)
    }
}
#[doc = "Weak Pull-up Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppuen(pub u32);
impl Ppuen {
    #[doc = "Weak Pull-up Enable"]
    #[inline(always)]
    pub const fn p(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Weak Pull-up Enable"]
    #[inline(always)]
    pub fn set_p(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Ppuen {
    #[inline(always)]
    fn default() -> Ppuen {
        Ppuen(0)
    }
}
