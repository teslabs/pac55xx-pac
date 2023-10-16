#[doc = "General Purpose Input/Output"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO Pin Mode Select"]
    #[inline(always)]
    pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPIO Data Output Write Mask"]
    #[inline(always)]
    pub const fn outmask(self) -> crate::common::Reg<regs::Outmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "GPIO Data Output Value"]
    #[inline(always)]
    pub const fn out(self) -> crate::common::Reg<regs::Out, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "GPIO Data Input Value"]
    #[inline(always)]
    pub const fn in_(self) -> crate::common::Reg<regs::In, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "GPIO Interrupt Enable"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "GPIO Interrupt Flag Raw"]
    #[inline(always)]
    pub const fn intflagraw(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "GPIO Interrupt Flag Masked"]
    #[inline(always)]
    pub const fn intflagmasked(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "GPIO Interrupt Clear"]
    #[inline(always)]
    pub const fn intclear(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "GPIO Interrupt Type"]
    #[inline(always)]
    pub const fn inttype(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "GPIO Interrupt Value"]
    #[inline(always)]
    pub const fn intvalue(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "GPIO Interrupt Edge Both"]
    #[inline(always)]
    pub const fn intedgeboth(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "GPIO De-bounce Filter"]
    #[inline(always)]
    pub const fn debounce(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "GPIO Data Output Set"]
    #[inline(always)]
    pub const fn doset(self) -> crate::common::Reg<regs::Doset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "GPIO Data Output Clear"]
    #[inline(always)]
    pub const fn doclear(self) -> crate::common::Reg<regs::Doclear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
}
pub mod regs;
pub mod vals;
