#[doc = "Synchronous Serial Peripheral"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssp {
    ptr: *mut u8,
}
unsafe impl Send for Ssp {}
unsafe impl Sync for Ssp {}
impl Ssp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SSP Control Register"]
    #[inline(always)]
    pub const fn con(self) -> crate::common::Reg<regs::Con, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SSP Status Register"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SSP Data Register"]
    #[inline(always)]
    pub const fn dat(self) -> crate::common::Reg<regs::Dat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "SSP Clock Register"]
    #[inline(always)]
    pub const fn clk(self) -> crate::common::Reg<regs::Clk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SSP Interrupt Mask Set/Clear Register"]
    #[inline(always)]
    pub const fn imsc(self) -> crate::common::Reg<regs::Imsc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SSP Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn ris(self) -> crate::common::Reg<regs::Ris, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SSP Masked Interrupt Status Register"]
    #[inline(always)]
    pub const fn mis(self) -> crate::common::Reg<regs::Mis, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "SSP Interrupt Clear Register"]
    #[inline(always)]
    pub const fn clr(self) -> crate::common::Reg<regs::Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SSP Slave Select Control Register"]
    #[inline(always)]
    pub const fn sscr(self) -> crate::common::Reg<regs::Sscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
pub mod regs;
pub mod vals;
