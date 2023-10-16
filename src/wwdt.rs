#[doc = "Windowed Watchdog Timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wwdt {
    ptr: *mut u8,
}
unsafe impl Send for Wwdt {}
unsafe impl Sync for Wwdt {}
impl Wwdt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "WWDT Control"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "WWDT Load Counter Value"]
    #[inline(always)]
    pub const fn cdctl(self) -> crate::common::Reg<regs::Cdctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "WWDT Counter"]
    #[inline(always)]
    pub const fn ctr(self) -> crate::common::Reg<regs::Ctr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "WWDT Interrupt Flag"]
    #[inline(always)]
    pub const fn flag(self) -> crate::common::Reg<regs::Flag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "WWDT Interrupt Clear"]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::Clear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "WWDT Lock"]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
}
pub mod regs;
pub mod vals;
