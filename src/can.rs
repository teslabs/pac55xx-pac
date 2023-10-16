#[doc = "Controller Area Network"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can {
    ptr: *mut u8,
}
unsafe impl Send for Can {}
unsafe impl Sync for Can {}
impl Can {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CAN Mode Register/Command Register/Status Register/Interrupt Status or ACK Register"]
    #[inline(always)]
    pub const fn mr_cmr_sr_isr(self) -> crate::common::Reg<regs::MrCmrSrIsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "CAN Interrupt Mask Register/Receive Message Counter Register/Bus Timing 0 Register/Bus Timing 1 Register"]
    #[inline(always)]
    pub const fn imr_rmc_btr0_btr1(
        self,
    ) -> crate::common::Reg<regs::ImrRmcBtr0Btr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "CAN Transmit Buffer register"]
    #[inline(always)]
    pub const fn txbuf(self) -> crate::common::Reg<regs::Txbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "CAN Receive Buffer register"]
    #[inline(always)]
    pub const fn rxbuf(self) -> crate::common::Reg<regs::Rxbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "CAN Acceptance Code register"]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "CAN Acceptance Mask register"]
    #[inline(always)]
    pub const fn amr(self) -> crate::common::Reg<regs::Amr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "CAN Error Code Capture register/Receive Error Counter Register/Transmit Error Counter Register/Arbitration Lost Code Capture Register"]
    #[inline(always)]
    pub const fn ecc_rxerr_txerr_alc(
        self,
    ) -> crate::common::Reg<regs::EccRxerrTxerrAlc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs;
pub mod vals;
