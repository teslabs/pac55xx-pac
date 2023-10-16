#[doc = "WWDT Load Counter Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdctl(pub u32);
impl Cdctl {
    #[doc = "WWDT count-down value"]
    #[inline(always)]
    pub const fn cdv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "WWDT count-down value"]
    #[inline(always)]
    pub fn set_cdv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "WWDT window value"]
    #[inline(always)]
    pub const fn window(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "WWDT window value"]
    #[inline(always)]
    pub fn set_window(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cdctl {
    #[inline(always)]
    fn default() -> Cdctl {
        Cdctl(0)
    }
}
#[doc = "WWDT Interrupt Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clear(pub u32);
impl Clear {
    #[doc = "WWDT clear"]
    #[inline(always)]
    pub const fn value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "WWDT clear"]
    #[inline(always)]
    pub fn set_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Clear {
    #[inline(always)]
    fn default() -> Clear {
        Clear(0)
    }
}
#[doc = "WWDT Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "Enable WWDT"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable WWDT"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable WWDT interrupt"]
    #[inline(always)]
    pub const fn inten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable WWDT interrupt"]
    #[inline(always)]
    pub fn set_inten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable WWDT POR reset"]
    #[inline(always)]
    pub const fn rsten(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable WWDT POR reset"]
    #[inline(always)]
    pub fn set_rsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "WWDT input clock source"]
    #[inline(always)]
    pub const fn clksel(&self) -> super::vals::Clksel {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Clksel::from_bits(val as u8)
    }
    #[doc = "WWDT input clock source"]
    #[inline(always)]
    pub fn set_clksel(&mut self, val: super::vals::Clksel) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "WWDT input clock divider"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> super::vals::Clkdiv {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Clkdiv::from_bits(val as u8)
    }
    #[doc = "WWDT input clock divider"]
    #[inline(always)]
    pub fn set_clkdiv(&mut self, val: super::vals::Clkdiv) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
#[doc = "WWDT Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctr(pub u32);
impl Ctr {
    #[doc = "WWDT counter value"]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "WWDT counter value"]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ctr {
    #[inline(always)]
    fn default() -> Ctr {
        Ctr(0)
    }
}
#[doc = "WWDT Interrupt Flag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flag(pub u32);
impl Flag {
    #[doc = "WWDT interrupt flag"]
    #[inline(always)]
    pub const fn if_(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT interrupt flag"]
    #[inline(always)]
    pub fn set_if_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "WWDT reset flag"]
    #[inline(always)]
    pub const fn rstf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT reset flag"]
    #[inline(always)]
    pub fn set_rstf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Flag {
    #[inline(always)]
    fn default() -> Flag {
        Flag(0)
    }
}
#[doc = "WWDT Lock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "WWDT lock"]
    #[inline(always)]
    pub const fn value(&self) -> super::vals::Value {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Value::from_bits(val as u32)
    }
    #[doc = "WWDT lock"]
    #[inline(always)]
    pub fn set_value(&mut self, val: super::vals::Value) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Lock {
    #[inline(always)]
    fn default() -> Lock {
        Lock(0)
    }
}
