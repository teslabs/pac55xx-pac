#[doc = "CRC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "CRC Polynomial Select"]
    #[inline(always)]
    pub const fn polysel(&self) -> super::vals::Polysel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Polysel::from_bits(val as u8)
    }
    #[doc = "CRC Polynomial Select"]
    #[inline(always)]
    pub fn set_polysel(&mut self, val: super::vals::Polysel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CRC Data Width"]
    #[inline(always)]
    pub const fn datawidth(&self) -> super::vals::Datawidth {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Datawidth::from_bits(val as u8)
    }
    #[doc = "CRC Data Width"]
    #[inline(always)]
    pub fn set_datawidth(&mut self, val: super::vals::Datawidth) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Reflect DATAOUT output data from CRC engine"]
    #[inline(always)]
    pub const fn outref(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reflect DATAOUT output data from CRC engine"]
    #[inline(always)]
    pub fn set_outref(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reflect DATAIN input data to CRC engine"]
    #[inline(always)]
    pub const fn inref(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reflect DATAIN input data to CRC engine"]
    #[inline(always)]
    pub fn set_inref(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
#[doc = "CRC Data Output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out(pub u32);
impl Out {
    #[doc = "CRC data output value"]
    #[inline(always)]
    pub const fn crcout(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CRC data output value"]
    #[inline(always)]
    pub fn set_crcout(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Out {
    #[inline(always)]
    fn default() -> Out {
        Out(0)
    }
}
#[doc = "CRC Seed Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seed(pub u32);
impl Seed {
    #[doc = "CRC Seed Value"]
    #[inline(always)]
    pub const fn crcseed(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CRC Seed Value"]
    #[inline(always)]
    pub fn set_crcseed(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Seed {
    #[inline(always)]
    fn default() -> Seed {
        Seed(0)
    }
}
