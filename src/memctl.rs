#[doc = "Memory Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memctl {
    ptr: *mut u8,
}
unsafe impl Send for Memctl {}
unsafe impl Sync for Memctl {}
impl Memctl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Memory Controller Configuration"]
    #[inline(always)]
    pub const fn memctl(self) -> crate::common::Reg<regs::Memctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Memory Controller Status"]
    #[inline(always)]
    pub const fn memstatus(self) -> crate::common::Reg<regs::Memstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "FLASH Lock Access"]
    #[inline(always)]
    pub const fn flashlock(self) -> crate::common::Reg<regs::Flashlock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "FLASH Page"]
    #[inline(always)]
    pub const fn flashpage(self) -> crate::common::Reg<regs::Flashpage, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SWD Unlock"]
    #[inline(always)]
    pub const fn swdunlock(self) -> crate::common::Reg<regs::Swdunlock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "FLASH Erase"]
    #[inline(always)]
    pub const fn flasherase(self) -> crate::common::Reg<regs::Flasherase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs;
pub mod vals;
