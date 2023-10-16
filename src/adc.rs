#[doc = "Analog to Digital Converter (ADC) and Dynamic Triggering and Sampling Engine (DTSE)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "EMUX Control"]
    #[inline(always)]
    pub const fn emuxctl(self) -> crate::common::Reg<regs::Emuxctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "EMUX Data"]
    #[inline(always)]
    pub const fn emuxdata(self) -> crate::common::Reg<regs::Emuxdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ADC Control"]
    #[inline(always)]
    pub const fn adcctl(self) -> crate::common::Reg<regs::Adcctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "ADC Result"]
    #[inline(always)]
    pub const fn adcres(self) -> crate::common::Reg<regs::Adcres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "ADC Interrupt Control"]
    #[inline(always)]
    pub const fn adcint(self) -> crate::common::Reg<regs::Adcint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "DTSE Trigger Entry 0 to 3"]
    #[inline(always)]
    pub const fn dtsetrigent0to3(
        self,
    ) -> crate::common::Reg<regs::Dtsetrigent0to3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "DTSE Trigger Entry 4 to 7"]
    #[inline(always)]
    pub const fn dtsetrigent4to7(
        self,
    ) -> crate::common::Reg<regs::Dtsetrigent4to7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "DTSE Trigger Entry 8 to 11"]
    #[inline(always)]
    pub const fn dtsetrigent8to11(
        self,
    ) -> crate::common::Reg<regs::Dtsetrigent8to11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "DTSE Trigger Entry 12 to 15"]
    #[inline(always)]
    pub const fn dtsetrigent12to15(
        self,
    ) -> crate::common::Reg<regs::Dtsetrigent12to15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "DTSE Trigger Entry 16 to 19"]
    #[inline(always)]
    pub const fn dtsetrigent16to19(
        self,
    ) -> crate::common::Reg<regs::Dtsetrigent16to19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "DTSE Trigger Entry 20 to 23"]
    #[inline(always)]
    pub const fn dtsetrigent20to23(
        self,
    ) -> crate::common::Reg<regs::Dtsetrigent20to23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "DTSE Trigger Entry 24 to 27"]
    #[inline(always)]
    pub const fn dtsetrigent24to27(
        self,
    ) -> crate::common::Reg<regs::Dtsetrigent24to27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "DTSE Trigger Entry 28 to 31"]
    #[inline(always)]
    pub const fn dtsetrigent28to31(
        self,
    ) -> crate::common::Reg<regs::Dtsetrigent28to31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "DTSE Sequence Configuration"]
    #[inline(always)]
    pub const fn dtseseqcfg(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Dtseseqcfg, crate::common::RW> {
        assert!(n < 24usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "DTSE Result"]
    #[inline(always)]
    pub const fn dtseres(self, n: usize) -> crate::common::Reg<regs::Dtseres, crate::common::RW> {
        assert!(n < 24usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
