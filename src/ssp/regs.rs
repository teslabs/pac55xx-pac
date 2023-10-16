#[doc = "SSP Clock Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clk(pub u32);
impl Clk {
    #[doc = "N Value"]
    #[inline(always)]
    pub const fn n(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "N Value"]
    #[inline(always)]
    pub fn set_n(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "M Value"]
    #[inline(always)]
    pub const fn m(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "M Value"]
    #[inline(always)]
    pub fn set_m(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Clk {
    #[inline(always)]
    fn default() -> Clk {
        Clk(0)
    }
}
#[doc = "SSP Interrupt Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clr(pub u32);
impl Clr {
    #[doc = "Receive FIFO Overrun Interrupt Clear"]
    #[inline(always)]
    pub const fn roic(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Overrun Interrupt Clear"]
    #[inline(always)]
    pub fn set_roic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive FIFO Timeout Interrupt Clear"]
    #[inline(always)]
    pub const fn rtic(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Timeout Interrupt Clear"]
    #[inline(always)]
    pub fn set_rtic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Clr {
    #[inline(always)]
    fn default() -> Clr {
        Clr(0)
    }
}
#[doc = "SSP Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Con(pub u32);
impl Con {
    #[doc = "Data Size Select"]
    #[inline(always)]
    pub const fn dss(&self) -> super::vals::Dss {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Dss::from_bits(val as u8)
    }
    #[doc = "Data Size Select"]
    #[inline(always)]
    pub fn set_dss(&mut self, val: super::vals::Dss) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Frame Format"]
    #[inline(always)]
    pub const fn frf(&self) -> super::vals::Frf {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Frf::from_bits(val as u8)
    }
    #[doc = "Frame Format"]
    #[inline(always)]
    pub fn set_frf(&mut self, val: super::vals::Frf) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Clock Out Polarity"]
    #[inline(always)]
    pub const fn cpo(&self) -> super::vals::Cpo {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cpo::from_bits(val as u8)
    }
    #[doc = "Clock Out Polarity"]
    #[inline(always)]
    pub fn set_cpo(&mut self, val: super::vals::Cpo) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Clock Phase"]
    #[inline(always)]
    pub const fn cph(&self) -> super::vals::Cph {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cph::from_bits(val as u8)
    }
    #[doc = "Clock Phase"]
    #[inline(always)]
    pub fn set_cph(&mut self, val: super::vals::Cph) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Slave Output Disable"]
    #[inline(always)]
    pub const fn sod(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Output Disable"]
    #[inline(always)]
    pub fn set_sod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Master/Slave Mode"]
    #[inline(always)]
    pub const fn ms(&self) -> super::vals::Ms {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Ms::from_bits(val as u8)
    }
    #[doc = "Master/Slave Mode"]
    #[inline(always)]
    pub fn set_ms(&mut self, val: super::vals::Ms) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "SSP Enable"]
    #[inline(always)]
    pub const fn sspen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SSP Enable"]
    #[inline(always)]
    pub fn set_sspen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Loopback Mode"]
    #[inline(always)]
    pub const fn lbm(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback Mode"]
    #[inline(always)]
    pub fn set_lbm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "LSB First"]
    #[inline(always)]
    pub const fn lsbfirst(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "LSB First"]
    #[inline(always)]
    pub fn set_lsbfirst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Con {
    #[inline(always)]
    fn default() -> Con {
        Con(0)
    }
}
#[doc = "SSP Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dat(pub u32);
impl Dat {
    #[doc = "Data"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dat {
    #[inline(always)]
    fn default() -> Dat {
        Dat(0)
    }
}
#[doc = "SSP Interrupt Mask Set/Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imsc(pub u32);
impl Imsc {
    #[doc = "Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub const fn rorim(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn set_rorim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Timeout Interrupt Mask"]
    #[inline(always)]
    pub const fn rtim(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Timeout Interrupt Mask"]
    #[inline(always)]
    pub fn set_rtim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO Interrupt Mask"]
    #[inline(always)]
    pub const fn rxim(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn set_rxim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit FIFO Interrupt Mask"]
    #[inline(always)]
    pub const fn txim(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn set_txim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Imsc {
    #[inline(always)]
    fn default() -> Imsc {
        Imsc(0)
    }
}
#[doc = "SSP Masked Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mis(pub u32);
impl Mis {
    #[doc = "Receive Overrun Masked Interrupt Status"]
    #[inline(always)]
    pub const fn rormis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Overrun Masked Interrupt Status"]
    #[inline(always)]
    pub fn set_rormis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Timeout Masked Interrupt Status"]
    #[inline(always)]
    pub const fn rtmis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Timeout Masked Interrupt Status"]
    #[inline(always)]
    pub fn set_rtmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub const fn rxmis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn set_rxmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub const fn txmis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn set_txmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Mis {
    #[inline(always)]
    fn default() -> Mis {
        Mis(0)
    }
}
#[doc = "SSP Raw Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ris(pub u32);
impl Ris {
    #[doc = "Receive Overrun Raw Interrupt Status"]
    #[inline(always)]
    pub const fn rorris(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Overrun Raw Interrupt Status"]
    #[inline(always)]
    pub fn set_rorris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Timeout Raw Interrupt Status"]
    #[inline(always)]
    pub const fn rtris(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Timeout Raw Interrupt Status"]
    #[inline(always)]
    pub fn set_rtris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub const fn rxris(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn set_rxris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub const fn txris(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn set_txris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Ris {
    #[inline(always)]
    fn default() -> Ris {
        Ris(0)
    }
}
#[doc = "SSP Slave Select Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sscr(pub u32);
impl Sscr {
    #[doc = "Slave Select"]
    #[inline(always)]
    pub const fn selss(&self) -> super::vals::Selss {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Selss::from_bits(val as u8)
    }
    #[doc = "Slave Select"]
    #[inline(always)]
    pub fn set_selss(&mut self, val: super::vals::Selss) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Slave Select Software Control"]
    #[inline(always)]
    pub const fn swsel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select Software Control"]
    #[inline(always)]
    pub fn set_swsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Slave Select State"]
    #[inline(always)]
    pub const fn swss(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select State"]
    #[inline(always)]
    pub fn set_swss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Slave Select Pull-high"]
    #[inline(always)]
    pub const fn sphdontcare(&self) -> super::vals::Sphdontcare {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sphdontcare::from_bits(val as u8)
    }
    #[doc = "Slave Select Pull-high"]
    #[inline(always)]
    pub fn set_sphdontcare(&mut self, val: super::vals::Sphdontcare) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for Sscr {
    #[inline(always)]
    fn default() -> Sscr {
        Sscr(0)
    }
}
#[doc = "SSP Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Transmit FIFO Empty"]
    #[inline(always)]
    pub const fn tfe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Empty"]
    #[inline(always)]
    pub fn set_tfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO Not Full"]
    #[inline(always)]
    pub const fn tnf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Not Full"]
    #[inline(always)]
    pub fn set_tnf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO Not Empty"]
    #[inline(always)]
    pub const fn rne(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Not Empty"]
    #[inline(always)]
    pub fn set_rne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO Full"]
    #[inline(always)]
    pub const fn rff(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Full"]
    #[inline(always)]
    pub fn set_rff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub const fn bsy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn set_bsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
