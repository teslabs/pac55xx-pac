#[doc = "UART A Divisor Latch"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlr(pub u32);
impl Dlr {
    #[doc = "Divisor Latch Register (Baudrate = PCLK / (16 * DLR))"]
    #[inline(always)]
    pub const fn dlr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Divisor Latch Register (Baudrate = PCLK / (16 * DLR))"]
    #[inline(always)]
    pub fn set_dlr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dlr {
    #[inline(always)]
    fn default() -> Dlr {
        Dlr(0)
    }
}
#[doc = "Enhanced Feature"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Efr(pub u32);
impl Efr {
    #[doc = "Enable Enhanced Mode"]
    #[inline(always)]
    pub const fn erfen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Enhanced Mode"]
    #[inline(always)]
    pub fn set_erfen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Efr {
    #[inline(always)]
    fn default() -> Efr {
        Efr(0)
    }
}
#[doc = "UART A FIFO Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr(pub u32);
impl Fcr {
    #[doc = "FIFO Enable"]
    #[inline(always)]
    pub const fn fifoen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Enable"]
    #[inline(always)]
    pub fn set_fifoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX FIFO Reset"]
    #[inline(always)]
    pub const fn rxfiforst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO Reset"]
    #[inline(always)]
    pub fn set_rxfiforst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TX FIFO Reset"]
    #[inline(always)]
    pub const fn txfiforst(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO Reset"]
    #[inline(always)]
    pub fn set_txfiforst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TX FIFO Trigger Level"]
    #[inline(always)]
    pub const fn txtl(&self) -> super::vals::Txtl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Txtl::from_bits(val as u8)
    }
    #[doc = "TX FIFO Trigger Level"]
    #[inline(always)]
    pub fn set_txtl(&mut self, val: super::vals::Txtl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "RX FIFO Trigger Level"]
    #[inline(always)]
    pub const fn rxtl(&self) -> super::vals::Rxtl {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Rxtl::from_bits(val as u8)
    }
    #[doc = "RX FIFO Trigger Level"]
    #[inline(always)]
    pub fn set_rxtl(&mut self, val: super::vals::Rxtl) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for Fcr {
    #[inline(always)]
    fn default() -> Fcr {
        Fcr(0)
    }
}
#[doc = "UART A Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Enable the UART RX Buffer Full Interrupt"]
    #[inline(always)]
    pub const fn rbrie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the UART RX Buffer Full Interrupt"]
    #[inline(always)]
    pub fn set_rbrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the UART TX Holding Register Empty Interrupt"]
    #[inline(always)]
    pub const fn threie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the UART TX Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn set_threie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable the UART RX Line Status Interrupt"]
    #[inline(always)]
    pub const fn rlsie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the UART RX Line Status Interrupt"]
    #[inline(always)]
    pub fn set_rlsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
#[doc = "UART A Interrupt Identification"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iir(pub u32);
impl Iir {
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub const fn intstatus(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn set_intstatus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Identification"]
    #[inline(always)]
    pub const fn intid(&self) -> super::vals::Intid {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Intid::from_bits(val as u8)
    }
    #[doc = "Interrupt Identification"]
    #[inline(always)]
    pub fn set_intid(&mut self, val: super::vals::Intid) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
}
impl Default for Iir {
    #[inline(always)]
    fn default() -> Iir {
        Iir(0)
    }
}
#[doc = "UART A Line Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcr(pub u32);
impl Lcr {
    #[doc = "Word Length Select"]
    #[inline(always)]
    pub const fn wls(&self) -> super::vals::Wls {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Wls::from_bits(val as u8)
    }
    #[doc = "Word Length Select"]
    #[inline(always)]
    pub fn set_wls(&mut self, val: super::vals::Wls) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Stop Bit Select"]
    #[inline(always)]
    pub const fn sbs(&self) -> super::vals::Sbs {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sbs::from_bits(val as u8)
    }
    #[doc = "Stop Bit Select"]
    #[inline(always)]
    pub fn set_sbs(&mut self, val: super::vals::Sbs) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Parity Enable"]
    #[inline(always)]
    pub const fn pen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Enable"]
    #[inline(always)]
    pub fn set_pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Parity Select"]
    #[inline(always)]
    pub const fn psel(&self) -> super::vals::Psel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Psel::from_bits(val as u8)
    }
    #[doc = "Parity Select"]
    #[inline(always)]
    pub fn set_psel(&mut self, val: super::vals::Psel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Break Control"]
    #[inline(always)]
    pub const fn bcon(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Break Control"]
    #[inline(always)]
    pub fn set_bcon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Lcr {
    #[inline(always)]
    fn default() -> Lcr {
        Lcr(0)
    }
}
#[doc = "UART A Line Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lsr(pub u32);
impl Lsr {
    #[doc = "Receiver Data Ready"]
    #[inline(always)]
    pub const fn rdr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Data Ready"]
    #[inline(always)]
    pub fn set_rdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Overrun Error"]
    #[inline(always)]
    pub const fn oe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Error"]
    #[inline(always)]
    pub fn set_oe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Parity Error"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Framing Error"]
    #[inline(always)]
    pub const fn fe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error"]
    #[inline(always)]
    pub fn set_fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Break Interrupt"]
    #[inline(always)]
    pub const fn bi(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Break Interrupt"]
    #[inline(always)]
    pub fn set_bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmitter Holding Register Empty"]
    #[inline(always)]
    pub const fn thre(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Holding Register Empty"]
    #[inline(always)]
    pub fn set_thre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmitter Empty"]
    #[inline(always)]
    pub const fn temt(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Empty"]
    #[inline(always)]
    pub fn set_temt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Error in RX FIFO"]
    #[inline(always)]
    pub const fn rxfe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Error in RX FIFO"]
    #[inline(always)]
    pub fn set_rxfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Lsr {
    #[inline(always)]
    fn default() -> Lsr {
        Lsr(0)
    }
}
#[doc = "UART A Receive Buffer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbr(pub u32);
impl Rbr {
    #[doc = "Oldest received character in the UART RX FIFO"]
    #[inline(always)]
    pub const fn rbr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Oldest received character in the UART RX FIFO"]
    #[inline(always)]
    pub fn set_rbr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rbr {
    #[inline(always)]
    fn default() -> Rbr {
        Rbr(0)
    }
}
#[doc = "UART A Scratch Pad"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "Readable/writable byte"]
    #[inline(always)]
    pub const fn pad(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Readable/writable byte"]
    #[inline(always)]
    pub fn set_pad(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
#[doc = "UART A Transmit Holding"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Thr(pub u32);
impl Thr {
    #[doc = "Place a character in the UART TX FIFO"]
    #[inline(always)]
    pub const fn thr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Place a character in the UART TX FIFO"]
    #[inline(always)]
    pub fn set_thr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Thr {
    #[inline(always)]
    fn default() -> Thr {
        Thr(0)
    }
}
