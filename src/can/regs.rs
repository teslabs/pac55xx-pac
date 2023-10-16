#[doc = "CAN Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acr(pub u32);
impl Acr {
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn acr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub fn set_acr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Acr {
    #[inline(always)]
    fn default() -> Acr {
        Acr(0)
    }
}
#[doc = "CAN Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amr(pub u32);
impl Amr {
    #[doc = "Acceptance code mask"]
    #[inline(always)]
    pub const fn amr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Acceptance code mask"]
    #[inline(always)]
    pub fn set_amr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Amr {
    #[inline(always)]
    fn default() -> Amr {
        Amr(0)
    }
}
#[doc = "CAN Error Code Capture register/Receive Error Counter Register/Transmit Error Counter Register/Arbitration Lost Code Capture Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccRxerrTxerrAlc(pub u32);
impl EccRxerrTxerrAlc {
    #[doc = "Bit error occurred"]
    #[inline(always)]
    pub const fn ber(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit error occurred"]
    #[inline(always)]
    pub fn set_ber(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Stuff error occurred"]
    #[inline(always)]
    pub const fn stfer(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Stuff error occurred"]
    #[inline(always)]
    pub fn set_stfer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CRC error occurred"]
    #[inline(always)]
    pub const fn crcer(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CRC error occurred"]
    #[inline(always)]
    pub fn set_crcer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Form error occurred"]
    #[inline(always)]
    pub const fn frmer(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Form error occurred"]
    #[inline(always)]
    pub fn set_frmer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Acknowledge error occurred"]
    #[inline(always)]
    pub const fn acker(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Acknowledge error occurred"]
    #[inline(always)]
    pub fn set_acker(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Direction of transfer while error occurred"]
    #[inline(always)]
    pub const fn edir(&self) -> super::vals::Edir {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Edir::from_bits(val as u8)
    }
    #[doc = "Direction of transfer while error occurred"]
    #[inline(always)]
    pub fn set_edir(&mut self, val: super::vals::Edir) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Set when CANTXERR is major or equal to 96"]
    #[inline(always)]
    pub const fn txwrn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set when CANTXERR is major or equal to 96"]
    #[inline(always)]
    pub fn set_txwrn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set when CANRXERR is major or equal to 96"]
    #[inline(always)]
    pub const fn rxwrn(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set when CANRXERR is major or equal to 96"]
    #[inline(always)]
    pub fn set_rxwrn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive error counter"]
    #[inline(always)]
    pub const fn rxerr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Receive error counter"]
    #[inline(always)]
    pub fn set_rxerr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Transmit error counter"]
    #[inline(always)]
    pub const fn txerr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit error counter"]
    #[inline(always)]
    pub fn set_txerr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Arbitration lost capture"]
    #[inline(always)]
    pub const fn alc(&self) -> super::vals::Alc {
        let val = (self.0 >> 24usize) & 0x1f;
        super::vals::Alc::from_bits(val as u8)
    }
    #[doc = "Arbitration lost capture"]
    #[inline(always)]
    pub fn set_alc(&mut self, val: super::vals::Alc) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val.to_bits() as u32) & 0x1f) << 24usize);
    }
}
impl Default for EccRxerrTxerrAlc {
    #[inline(always)]
    fn default() -> EccRxerrTxerrAlc {
        EccRxerrTxerrAlc(0)
    }
}
#[doc = "CAN Interrupt Mask Register/Receive Message Counter Register/Bus Timing 0 Register/Bus Timing 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImrRmcBtr0Btr1(pub u32);
impl ImrRmcBtr0Btr1 {
    #[doc = "Data Overrun Interrupt Mask"]
    #[inline(always)]
    pub const fn doim(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Data Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn set_doim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bus Error Interrupt Mask"]
    #[inline(always)]
    pub const fn beim(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error Interrupt Mask"]
    #[inline(always)]
    pub fn set_beim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Interrupt Mask"]
    #[inline(always)]
    pub const fn tim(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn set_tim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive Interrupt Mask"]
    #[inline(always)]
    pub const fn rim(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt Mask"]
    #[inline(always)]
    pub fn set_rim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Error Passive Interrupt Mask"]
    #[inline(always)]
    pub const fn epim(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Error Passive Interrupt Mask"]
    #[inline(always)]
    pub fn set_epim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Error Warning Interrupt Mask"]
    #[inline(always)]
    pub const fn ewim(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Error Warning Interrupt Mask"]
    #[inline(always)]
    pub fn set_ewim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub const fn alim(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn set_alim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Number of frames stored in the receive FIFO"]
    #[inline(always)]
    pub const fn rmc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of frames stored in the receive FIFO"]
    #[inline(always)]
    pub fn set_rmc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Baud rate prescaler"]
    #[inline(always)]
    pub const fn brp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Baud rate prescaler"]
    #[inline(always)]
    pub fn set_brp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Synchronization jump width"]
    #[inline(always)]
    pub const fn sjw(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Synchronization jump width"]
    #[inline(always)]
    pub fn set_sjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Number of clock cycles per Time segment 1"]
    #[inline(always)]
    pub const fn tseg1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of clock cycles per Time segment 1"]
    #[inline(always)]
    pub fn set_tseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Number of clock cycles per Time segment 2"]
    #[inline(always)]
    pub const fn tseg2(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Number of clock cycles per Time segment 2"]
    #[inline(always)]
    pub fn set_tseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
    #[doc = "Number of bus level samples"]
    #[inline(always)]
    pub const fn sam(&self) -> super::vals::Sam {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Sam::from_bits(val as u8)
    }
    #[doc = "Number of bus level samples"]
    #[inline(always)]
    pub fn set_sam(&mut self, val: super::vals::Sam) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for ImrRmcBtr0Btr1 {
    #[inline(always)]
    fn default() -> ImrRmcBtr0Btr1 {
        ImrRmcBtr0Btr1(0)
    }
}
#[doc = "CAN Mode Register/Command Register/Status Register/Interrupt Status or ACK Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrCmrSrIsr(pub u32);
impl MrCmrSrIsr {
    #[doc = "Hardware filter acceptance scheme"]
    #[inline(always)]
    pub const fn afm(&self) -> super::vals::Afm {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Afm::from_bits(val as u8)
    }
    #[doc = "Hardware filter acceptance scheme"]
    #[inline(always)]
    pub fn set_afm(&mut self, val: super::vals::Afm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Listen-only mode"]
    #[inline(always)]
    pub const fn lom(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Listen-only mode"]
    #[inline(always)]
    pub fn set_lom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Reset mode"]
    #[inline(always)]
    pub const fn rm(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Reset mode"]
    #[inline(always)]
    pub fn set_rm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Abort Transmission"]
    #[inline(always)]
    pub const fn at(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Abort Transmission"]
    #[inline(always)]
    pub fn set_at(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmit request"]
    #[inline(always)]
    pub const fn tr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit request"]
    #[inline(always)]
    pub fn set_tr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Bus Off Status"]
    #[inline(always)]
    pub const fn bs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Off Status"]
    #[inline(always)]
    pub fn set_bs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Error Status"]
    #[inline(always)]
    pub const fn es(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Error Status"]
    #[inline(always)]
    pub fn set_es(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Transmit Status"]
    #[inline(always)]
    pub const fn ts(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Status"]
    #[inline(always)]
    pub fn set_ts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Receive Status"]
    #[inline(always)]
    pub const fn rs(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Status"]
    #[inline(always)]
    pub fn set_rs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Transmit Buffer Status"]
    #[inline(always)]
    pub const fn tbs(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer Status"]
    #[inline(always)]
    pub fn set_tbs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Data Overrun Status"]
    #[inline(always)]
    pub const fn dso(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Data Overrun Status"]
    #[inline(always)]
    pub fn set_dso(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Receive Buffer Status"]
    #[inline(always)]
    pub const fn rbs(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Status"]
    #[inline(always)]
    pub fn set_rbs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Data Overrun Interrupt"]
    #[inline(always)]
    pub const fn doi(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Data Overrun Interrupt"]
    #[inline(always)]
    pub fn set_doi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus Error Interrupt"]
    #[inline(always)]
    pub const fn bei(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error Interrupt"]
    #[inline(always)]
    pub fn set_bei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Transmit Interrupt"]
    #[inline(always)]
    pub const fn ti(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt"]
    #[inline(always)]
    pub fn set_ti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Receive Interrupt"]
    #[inline(always)]
    pub const fn ri(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt"]
    #[inline(always)]
    pub fn set_ri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Error Passive Interrupt"]
    #[inline(always)]
    pub const fn epi(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Error Passive Interrupt"]
    #[inline(always)]
    pub fn set_epi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Error Warning Interrupt"]
    #[inline(always)]
    pub const fn ewi(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Error Warning Interrupt"]
    #[inline(always)]
    pub fn set_ewi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Arbitration Lost Interrupt"]
    #[inline(always)]
    pub const fn ali(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn set_ali(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for MrCmrSrIsr {
    #[inline(always)]
    fn default() -> MrCmrSrIsr {
        MrCmrSrIsr(0)
    }
}
#[doc = "CAN Receive Buffer register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxbuf(pub u32);
impl Rxbuf {
    #[doc = "Receive buffer data"]
    #[inline(always)]
    pub const fn rxbuf(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive buffer data"]
    #[inline(always)]
    pub fn set_rxbuf(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rxbuf {
    #[inline(always)]
    fn default() -> Rxbuf {
        Rxbuf(0)
    }
}
#[doc = "CAN Transmit Buffer register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbuf(pub u32);
impl Txbuf {
    #[doc = "Transmit buffer data"]
    #[inline(always)]
    pub const fn txbuf(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit buffer data"]
    #[inline(always)]
    pub fn set_txbuf(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbuf {
    #[inline(always)]
    fn default() -> Txbuf {
        Txbuf(0)
    }
}
