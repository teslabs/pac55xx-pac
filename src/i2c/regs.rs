#[doc = "I2C Slave Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adr(pub u32);
impl Adr {
    #[doc = "General Call Enable"]
    #[inline(always)]
    pub const fn gc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Enable"]
    #[inline(always)]
    pub fn set_gc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave Address"]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Slave Address"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for Adr {
    #[inline(always)]
    fn default() -> Adr {
        Adr(0)
    }
}
#[doc = "I2C Slave Address Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adrm(pub u32);
impl Adrm {
    #[doc = "Slave Address Mask"]
    #[inline(always)]
    pub const fn mask(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Slave Address Mask"]
    #[inline(always)]
    pub fn set_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for Adrm {
    #[inline(always)]
    fn default() -> Adrm {
        Adrm(0)
    }
}
#[doc = "I2C Clock Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clk(pub u32);
impl Clk {
    #[doc = "Fscl = PCLK / (2^M * (N+1) * 10)"]
    #[inline(always)]
    pub const fn n(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Fscl = PCLK / (2^M * (N+1) * 10)"]
    #[inline(always)]
    pub fn set_n(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Fsamp = PCLK / 2^M"]
    #[inline(always)]
    pub const fn m(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Fsamp = PCLK / 2^M"]
    #[inline(always)]
    pub fn set_m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for Clk {
    #[inline(always)]
    fn default() -> Clk {
        Clk(0)
    }
}
#[doc = "I2C Control Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Conclr(pub u32);
impl Conclr {
    #[doc = "Assert Acknowledge Clear"]
    #[inline(always)]
    pub const fn aac(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Assert Acknowledge Clear"]
    #[inline(always)]
    pub fn set_aac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "I2C Interrupt Clear"]
    #[inline(always)]
    pub const fn sic(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Interrupt Clear"]
    #[inline(always)]
    pub fn set_sic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "START Flag Clear"]
    #[inline(always)]
    pub const fn stac(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "START Flag Clear"]
    #[inline(always)]
    pub fn set_stac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "I2C Disable"]
    #[inline(always)]
    pub const fn i2cenc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Disable"]
    #[inline(always)]
    pub fn set_i2cenc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "I2C Interrupt Disable"]
    #[inline(always)]
    pub const fn i2ciec(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Interrupt Disable"]
    #[inline(always)]
    pub fn set_i2ciec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Conclr {
    #[inline(always)]
    fn default() -> Conclr {
        Conclr(0)
    }
}
#[doc = "I2C Control Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Conset(pub u32);
impl Conset {
    #[doc = "Slave Address Flag"]
    #[inline(always)]
    pub const fn adrf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Address Flag"]
    #[inline(always)]
    pub fn set_adrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Extended Slave Address Flag (10-bit addressing)"]
    #[inline(always)]
    pub const fn xadrf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Slave Address Flag (10-bit addressing)"]
    #[inline(always)]
    pub fn set_xadrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Assert Acknowledge Flag"]
    #[inline(always)]
    pub const fn aa(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Assert Acknowledge Flag"]
    #[inline(always)]
    pub fn set_aa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "I2C Interrupt Flag"]
    #[inline(always)]
    pub const fn si(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Interrupt Flag"]
    #[inline(always)]
    pub fn set_si(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "STOP Flag"]
    #[inline(always)]
    pub const fn sto(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Flag"]
    #[inline(always)]
    pub fn set_sto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "START Flag"]
    #[inline(always)]
    pub const fn sta(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "START Flag"]
    #[inline(always)]
    pub fn set_sta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "I2C Enable"]
    #[inline(always)]
    pub const fn i2cen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Enable"]
    #[inline(always)]
    pub fn set_i2cen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "I2C Interrupt Enable"]
    #[inline(always)]
    pub const fn i2cie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Interrupt Enable"]
    #[inline(always)]
    pub fn set_i2cie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "General Call Flag"]
    #[inline(always)]
    pub const fn gcf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Flag"]
    #[inline(always)]
    pub fn set_gcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Conset {
    #[inline(always)]
    fn default() -> Conset {
        Conset(0)
    }
}
#[doc = "I2C Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dat(pub u32);
impl Dat {
    #[doc = "Data values received or to be transmitted"]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data values received or to be transmitted"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Dat {
    #[inline(always)]
    fn default() -> Dat {
        Dat(0)
    }
}
#[doc = "I2C Software Reset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rst(pub u32);
impl Rst {
    #[doc = "I2C Software Reset"]
    #[inline(always)]
    pub const fn rst(&self) -> super::vals::Rst {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Rst::from_bits(val as u8)
    }
    #[doc = "I2C Software Reset"]
    #[inline(always)]
    pub fn set_rst(&mut self, val: super::vals::Rst) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Rst {
    #[inline(always)]
    fn default() -> Rst {
        Rst(0)
    }
}
#[doc = "I2C Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Status code for I2C engine"]
    #[inline(always)]
    pub const fn status(&self) -> super::vals::Status {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Status::from_bits(val as u8)
    }
    #[doc = "Status code for I2C engine"]
    #[inline(always)]
    pub fn set_status(&mut self, val: super::vals::Status) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
#[doc = "I2C Extended Slave Address 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xadr0(pub u32);
impl Xadr0 {
    #[doc = "Extended Slave Address"]
    #[inline(always)]
    pub const fn addr(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Extended Slave Address"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
}
impl Default for Xadr0 {
    #[inline(always)]
    fn default() -> Xadr0 {
        Xadr0(0)
    }
}
#[doc = "I2C Extended Slave Address Mask 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xadrm0(pub u32);
impl Xadrm0 {
    #[doc = "Extended Slave Address Mask"]
    #[inline(always)]
    pub const fn mask(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Extended Slave Address Mask"]
    #[inline(always)]
    pub fn set_mask(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
}
impl Default for Xadrm0 {
    #[inline(always)]
    fn default() -> Xadrm0 {
        Xadrm0(0)
    }
}
