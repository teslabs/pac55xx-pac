#[doc = "System and Clock Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scc {
    ptr: *mut u8,
}
unsafe impl Send for Scc {}
unsafe impl Sync for Scc {}
impl Scc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration"]
    #[inline(always)]
    pub const fn ccsctl(self) -> crate::common::Reg<regs::Ccsctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PLL Configuration"]
    #[inline(always)]
    pub const fn ccspllctl(self) -> crate::common::Reg<regs::Ccspllctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ROSC Trim Configuration"]
    #[inline(always)]
    pub const fn ccsrosctrim(self) -> crate::common::Reg<regs::Ccsrosctrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PA Peripheral MUX Select"]
    #[inline(always)]
    pub const fn pamuxsel(self) -> crate::common::Reg<regs::Pamuxsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PB Peripheral MUX Select"]
    #[inline(always)]
    pub const fn pbmuxsel(self) -> crate::common::Reg<regs::Pbmuxsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PC Peripheral MUX Select"]
    #[inline(always)]
    pub const fn pcmuxsel(self) -> crate::common::Reg<regs::Pcmuxsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "PD Peripheral MUX Select"]
    #[inline(always)]
    pub const fn pdmuxsel(self) -> crate::common::Reg<regs::Pdmuxsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "PE Peripheral MUX Select"]
    #[inline(always)]
    pub const fn pemuxsel(self) -> crate::common::Reg<regs::Pemuxsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "PF Peripheral MUX Select"]
    #[inline(always)]
    pub const fn pfmuxsel(self) -> crate::common::Reg<regs::Pfmuxsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "PG Peripheral MUX Select"]
    #[inline(always)]
    pub const fn pgmuxsel(self) -> crate::common::Reg<regs::Pgmuxsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Weak Pull-up Enable"]
    #[inline(always)]
    pub const fn papuen(self) -> crate::common::Reg<regs::Ppuen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Weak Pull-up Enable"]
    #[inline(always)]
    pub const fn pbpuen(self) -> crate::common::Reg<regs::Ppuen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Weak Pull-up Enable"]
    #[inline(always)]
    pub const fn pcpuen(self) -> crate::common::Reg<regs::Ppuen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Weak Pull-up Enable"]
    #[inline(always)]
    pub const fn pdpuen(self) -> crate::common::Reg<regs::Ppuen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Weak Pull-up Enable"]
    #[inline(always)]
    pub const fn pepuen(self) -> crate::common::Reg<regs::Ppuen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Weak Pull-up Enable"]
    #[inline(always)]
    pub const fn pfpuen(self) -> crate::common::Reg<regs::Ppuen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Weak Pull-up Enable"]
    #[inline(always)]
    pub const fn pgpuen(self) -> crate::common::Reg<regs::Ppuen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Weak Pull-down Enable"]
    #[inline(always)]
    pub const fn papden(self) -> crate::common::Reg<regs::Ppden, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Weak Pull-down Enable"]
    #[inline(always)]
    pub const fn pbpden(self) -> crate::common::Reg<regs::Ppden, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Weak Pull-down Enable"]
    #[inline(always)]
    pub const fn pcpden(self) -> crate::common::Reg<regs::Ppden, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Weak Pull-down Enable"]
    #[inline(always)]
    pub const fn pdpden(self) -> crate::common::Reg<regs::Ppden, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Weak Pull-down Enable"]
    #[inline(always)]
    pub const fn pepden(self) -> crate::common::Reg<regs::Ppden, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Weak Pull-down Enable"]
    #[inline(always)]
    pub const fn pfpden(self) -> crate::common::Reg<regs::Ppden, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Weak Pull-down Enable"]
    #[inline(always)]
    pub const fn pgpden(self) -> crate::common::Reg<regs::Ppden, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Drive Strength/Schmitt Trigger"]
    #[inline(always)]
    pub const fn pads(self) -> crate::common::Reg<regs::Pds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Drive Strength/Schmitt Trigger"]
    #[inline(always)]
    pub const fn pbds(self) -> crate::common::Reg<regs::Pds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Drive Strength/Schmitt Trigger"]
    #[inline(always)]
    pub const fn pcds(self) -> crate::common::Reg<regs::Pds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Drive Strength/Schmitt Trigger"]
    #[inline(always)]
    pub const fn pdds(self) -> crate::common::Reg<regs::Pds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "Drive Strength/Schmitt Trigger"]
    #[inline(always)]
    pub const fn peds(self) -> crate::common::Reg<regs::Pds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Drive Strength/Schmitt Trigger"]
    #[inline(always)]
    pub const fn pfds(self) -> crate::common::Reg<regs::Pds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Drive Strength/Schmitt Trigger"]
    #[inline(always)]
    pub const fn pgds(self) -> crate::common::Reg<regs::Pds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
}
pub mod regs;
pub mod vals;
