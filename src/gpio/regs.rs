#[doc = "GPIO Data Output Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doclear(pub u32);
impl Doclear {
    #[doc = "Pin 0 data output clear"]
    #[inline(always)]
    pub const fn p(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pin 0 data output clear"]
    #[inline(always)]
    pub fn set_p(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Doclear {
    #[inline(always)]
    fn default() -> Doclear {
        Doclear(0)
    }
}
#[doc = "GPIO Data Output Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doset(pub u32);
impl Doset {
    #[doc = "Pin 0 data output set"]
    #[inline(always)]
    pub const fn p(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pin 0 data output set"]
    #[inline(always)]
    pub fn set_p(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Doset {
    #[inline(always)]
    fn default() -> Doset {
        Doset(0)
    }
}
#[doc = "GPIO Data Input Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct In(pub u32);
impl In {
    #[doc = "Pin 0 input data value"]
    #[inline(always)]
    pub const fn p(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pin 0 input data value"]
    #[inline(always)]
    pub fn set_p(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for In {
    #[inline(always)]
    fn default() -> In {
        In(0)
    }
}
#[doc = "GPIO Pin Mode Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "Pin 0 mode"]
    #[inline(always)]
    pub const fn p(&self, n: usize) -> super::vals::P {
        assert!(n < 8usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        super::vals::P::from_bits(val as u8)
    }
    #[doc = "Pin 0 mode"]
    #[inline(always)]
    pub fn set_p(&mut self, n: usize, val: super::vals::P) {
        assert!(n < 8usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "GPIO Data Output Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out(pub u32);
impl Out {
    #[doc = "Pin 0 output data value"]
    #[inline(always)]
    pub const fn p(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pin 0 output data value"]
    #[inline(always)]
    pub fn set_p(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Out {
    #[inline(always)]
    fn default() -> Out {
        Out(0)
    }
}
#[doc = "GPIO Data Output Write Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outmask(pub u32);
impl Outmask {
    #[doc = "Pin 0 output data mask"]
    #[inline(always)]
    pub const fn p(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pin 0 output data mask"]
    #[inline(always)]
    pub fn set_p(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Outmask {
    #[inline(always)]
    fn default() -> Outmask {
        Outmask(0)
    }
}
