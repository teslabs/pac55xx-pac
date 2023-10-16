#[doc = "GPTimer Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "Enable"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Flag"]
    #[inline(always)]
    pub const fn if_(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag"]
    #[inline(always)]
    pub fn set_if_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clock Divider"]
    #[inline(always)]
    pub const fn div(&self) -> super::vals::Div {
        let val = (self.0 >> 3usize) & 0x0f;
        super::vals::Div::from_bits(val as u8)
    }
    #[doc = "Clock Divider"]
    #[inline(always)]
    pub fn set_div(&mut self, val: super::vals::Div) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
    #[doc = "Count-down value"]
    #[inline(always)]
    pub const fn cdv(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Count-down value"]
    #[inline(always)]
    pub fn set_cdv(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
#[doc = "GPTimer Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctr(pub u32);
impl Ctr {
    #[doc = "Counter value"]
    #[inline(always)]
    pub const fn ctr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Counter value"]
    #[inline(always)]
    pub fn set_ctr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Ctr {
    #[inline(always)]
    fn default() -> Ctr {
        Ctr(0)
    }
}
