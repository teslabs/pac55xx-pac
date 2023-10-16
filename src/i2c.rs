#[doc = "Inter-Integrated Circuit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c {
    ptr: *mut u8,
}
unsafe impl Send for I2c {}
unsafe impl Sync for I2c {}
impl I2c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "I2C Control Set"]
    #[inline(always)]
    pub const fn conset(self) -> crate::common::Reg<regs::Conset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "I2C Control Clear"]
    #[inline(always)]
    pub const fn conclr(self) -> crate::common::Reg<regs::Conclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "I2C Status"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "I2C Data"]
    #[inline(always)]
    pub const fn dat(self) -> crate::common::Reg<regs::Dat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "I2C Clock Control"]
    #[inline(always)]
    pub const fn clk(self) -> crate::common::Reg<regs::Clk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "I2C Slave Address"]
    #[inline(always)]
    pub const fn adr0(self) -> crate::common::Reg<regs::Adr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "I2C Slave Address Mask"]
    #[inline(always)]
    pub const fn adrm0(self) -> crate::common::Reg<regs::Adrm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "I2C Extended Slave Address 0"]
    #[inline(always)]
    pub const fn xadr0(self) -> crate::common::Reg<regs::Xadr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "I2C Extended Slave Address Mask 0"]
    #[inline(always)]
    pub const fn xadrm0(self) -> crate::common::Reg<regs::Xadrm0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "I2C Software Reset"]
    #[inline(always)]
    pub const fn rst(self) -> crate::common::Reg<regs::Rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "I2C Slave Address"]
    #[inline(always)]
    pub const fn adr1(self) -> crate::common::Reg<regs::Adr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "I2C Slave Address Mask"]
    #[inline(always)]
    pub const fn adrm1(self) -> crate::common::Reg<regs::Adrm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "I2C Slave Address"]
    #[inline(always)]
    pub const fn adr2(self) -> crate::common::Reg<regs::Adr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "I2C Slave Address Mask"]
    #[inline(always)]
    pub const fn adrm2(self) -> crate::common::Reg<regs::Adrm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "I2C Slave Address"]
    #[inline(always)]
    pub const fn adr3(self) -> crate::common::Reg<regs::Adr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "I2C Slave Address Mask"]
    #[inline(always)]
    pub const fn adrm3(self) -> crate::common::Reg<regs::Adrm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
