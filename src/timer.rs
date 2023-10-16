#[doc = "Timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer {
    ptr: *mut u8,
}
unsafe impl Send for Timer {}
unsafe impl Sync for Timer {}
impl Timer {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Timer Control"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Timer Interrupt Control"]
    #[inline(always)]
    pub const fn int(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Timer Period"]
    #[inline(always)]
    pub const fn prd(self) -> crate::common::Reg<regs::Prd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Timer Counter"]
    #[inline(always)]
    pub const fn ctr(self) -> crate::common::Reg<regs::Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Timer CCR Control"]
    #[inline(always)]
    pub const fn cctl(self, n: usize) -> crate::common::Reg<regs::Cctl, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 8usize) as _) }
    }
    #[doc = "Timer CCR Counter"]
    #[inline(always)]
    pub const fn cctr(self, n: usize) -> crate::common::Reg<regs::Cctr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize + n * 8usize) as _) }
    }
    #[doc = "Timer Dead-Time Generator Control"]
    #[inline(always)]
    pub const fn dtgctl(self, n: usize) -> crate::common::Reg<regs::Dtgctl, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[doc = "Timer QEP Control"]
    #[inline(always)]
    pub const fn qepctl(self) -> crate::common::Reg<regs::Qepctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "Timer QEP Counter"]
    #[inline(always)]
    pub const fn qepctr(self) -> crate::common::Reg<regs::Qepctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "Timer QEP Maximum Counter"]
    #[inline(always)]
    pub const fn qepmax(self) -> crate::common::Reg<regs::Qepmax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "Timer QEP Interrupt Enable"]
    #[inline(always)]
    pub const fn qepier(self) -> crate::common::Reg<regs::Qepier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "Timer QEP Interrupt Enable"]
    #[inline(always)]
    pub const fn qepif(self) -> crate::common::Reg<regs::Qepif, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
}
pub mod regs;
pub mod vals;
