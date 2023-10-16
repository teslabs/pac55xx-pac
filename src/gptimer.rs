#[doc = "General Purpose Timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptimer {
    ptr: *mut u8,
}
unsafe impl Send for Gptimer {}
unsafe impl Sync for Gptimer {}
impl Gptimer {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPTimer Control"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPTimer Counter"]
    #[inline(always)]
    pub const fn ctr(self) -> crate::common::Reg<regs::Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs;
pub mod vals;
