#[doc = "Timer CCR Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cctl(pub u32);
impl Cctl {
    #[doc = "CCR Mode"]
    #[inline(always)]
    pub const fn ccmode(&self) -> super::vals::CctlCcmode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CctlCcmode::from_bits(val as u8)
    }
    #[doc = "CCR Mode"]
    #[inline(always)]
    pub fn set_ccmode(&mut self, val: super::vals::CctlCcmode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CCR Interrupt Enable"]
    #[inline(always)]
    pub const fn ccinten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CCR Interrupt Enable"]
    #[inline(always)]
    pub fn set_ccinten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Capture Mode Interrupt Edge Setting"]
    #[inline(always)]
    pub const fn ccintedge(&self) -> super::vals::CctlCcintedge {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::CctlCcintedge::from_bits(val as u8)
    }
    #[doc = "Capture Mode Interrupt Edge Setting"]
    #[inline(always)]
    pub fn set_ccintedge(&mut self, val: super::vals::CctlCcintedge) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Invert CCR Output"]
    #[inline(always)]
    pub const fn ccoutinv(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Invert CCR Output"]
    #[inline(always)]
    pub fn set_ccoutinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CCR Register Latch Mode"]
    #[inline(always)]
    pub const fn cclatch(&self) -> super::vals::CctlCclatch {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CctlCclatch::from_bits(val as u8)
    }
    #[doc = "CCR Register Latch Mode"]
    #[inline(always)]
    pub fn set_cclatch(&mut self, val: super::vals::CctlCclatch) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Write 1 to force compare event (self-clearing)"]
    #[inline(always)]
    pub const fn ccforce(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to force compare event (self-clearing)"]
    #[inline(always)]
    pub fn set_ccforce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CC Interrupt Skip Counter"]
    #[inline(always)]
    pub const fn ccintskip(&self) -> super::vals::CctlCcintskip {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::CctlCcintskip::from_bits(val as u8)
    }
    #[doc = "CC Interrupt Skip Counter"]
    #[inline(always)]
    pub fn set_ccintskip(&mut self, val: super::vals::CctlCcintskip) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
}
impl Default for Cctl {
    #[inline(always)]
    fn default() -> Cctl {
        Cctl(0)
    }
}
#[doc = "Timer CCR Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cctr(pub u32);
impl Cctr {
    #[doc = "Capture/Compare Value"]
    #[inline(always)]
    pub const fn ctr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture/Compare Value"]
    #[inline(always)]
    pub fn set_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Delay count to use before compare events"]
    #[inline(always)]
    pub const fn delay(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay count to use before compare events"]
    #[inline(always)]
    pub fn set_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cctr {
    #[inline(always)]
    fn default() -> Cctr {
        Cctr(0)
    }
}
#[doc = "Timer Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Timer Period Latch Mode"]
    #[inline(always)]
    pub const fn prdlatch(&self) -> super::vals::Prdlatch {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Prdlatch::from_bits(val as u8)
    }
    #[doc = "Timer Period Latch Mode"]
    #[inline(always)]
    pub fn set_prdlatch(&mut self, val: super::vals::Prdlatch) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Timer Slave Synchronization"]
    #[inline(always)]
    pub const fn ssync(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Slave Synchronization"]
    #[inline(always)]
    pub fn set_ssync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Single Shot Timer"]
    #[inline(always)]
    pub const fn single(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Single Shot Timer"]
    #[inline(always)]
    pub fn set_single(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Timer Input Clock Divider"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> super::vals::Clkdiv {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Clkdiv::from_bits(val as u8)
    }
    #[doc = "Timer Input Clock Divider"]
    #[inline(always)]
    pub fn set_clkdiv(&mut self, val: super::vals::Clkdiv) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Timer Clock Source"]
    #[inline(always)]
    pub const fn clksrc(&self) -> super::vals::Clksrc {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Clksrc::from_bits(val as u8)
    }
    #[doc = "Timer Clock Source"]
    #[inline(always)]
    pub fn set_clksrc(&mut self, val: super::vals::Clksrc) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Dead-Time Generator Clock Source"]
    #[inline(always)]
    pub const fn dtgclk(&self) -> super::vals::Dtgclk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dtgclk::from_bits(val as u8)
    }
    #[doc = "Dead-Time Generator Clock Source"]
    #[inline(always)]
    pub fn set_dtgclk(&mut self, val: super::vals::Dtgclk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Write 1 to Latch Period and all CCTRs"]
    #[inline(always)]
    pub const fn latch(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to Latch Period and all CCTRs"]
    #[inline(always)]
    pub fn set_latch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Timer Clear"]
    #[inline(always)]
    pub const fn clr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Clear"]
    #[inline(always)]
    pub fn set_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Base timer interrupt enable"]
    #[inline(always)]
    pub const fn baseie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Base timer interrupt enable"]
    #[inline(always)]
    pub fn set_baseie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
#[doc = "Timer Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctr(pub u32);
impl Ctr {
    #[doc = "Timer Counter Value"]
    #[inline(always)]
    pub const fn counter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Counter Value"]
    #[inline(always)]
    pub fn set_counter(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ctr {
    #[inline(always)]
    fn default() -> Ctr {
        Ctr(0)
    }
}
#[doc = "Timer Dead-Time Generator Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtgctl(pub u32);
impl Dtgctl {
    #[doc = "Rising edge delay"]
    #[inline(always)]
    pub const fn red(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Rising edge delay"]
    #[inline(always)]
    pub fn set_red(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Falling edge delay"]
    #[inline(always)]
    pub const fn fed(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Falling edge delay"]
    #[inline(always)]
    pub fn set_fed(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "Dead-time generator enable"]
    #[inline(always)]
    pub const fn dten(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Dead-time generator enable"]
    #[inline(always)]
    pub fn set_dten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dtgctl {
    #[inline(always)]
    fn default() -> Dtgctl {
        Dtgctl(0)
    }
}
#[doc = "Timer Interrupt Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[doc = "CCR Interrupt Flag"]
    #[inline(always)]
    pub const fn ccrif(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "CCR Interrupt Flag"]
    #[inline(always)]
    pub fn set_ccrif(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Base Timer Interrupt Flag"]
    #[inline(always)]
    pub const fn baseif(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Base Timer Interrupt Flag"]
    #[inline(always)]
    pub fn set_baseif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Int {
    #[inline(always)]
    fn default() -> Int {
        Int(0)
    }
}
#[doc = "Timer Period"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prd(pub u32);
impl Prd {
    #[doc = "Timer Period Value"]
    #[inline(always)]
    pub const fn prd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Period Value"]
    #[inline(always)]
    pub fn set_prd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Prd {
    #[inline(always)]
    fn default() -> Prd {
        Prd(0)
    }
}
#[doc = "Timer QEP Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qepctl(pub u32);
impl Qepctl {
    #[doc = "QEP Enable"]
    #[inline(always)]
    pub const fn qepen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "QEP Enable"]
    #[inline(always)]
    pub fn set_qepen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Count on A/B"]
    #[inline(always)]
    pub const fn cntab(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Count on A/B"]
    #[inline(always)]
    pub fn set_cntab(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Count on edge"]
    #[inline(always)]
    pub const fn cntedge(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Count on edge"]
    #[inline(always)]
    pub fn set_cntedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Motor direction"]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Motor direction"]
    #[inline(always)]
    pub fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Index reset"]
    #[inline(always)]
    pub const fn idxrst(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Index reset"]
    #[inline(always)]
    pub fn set_idxrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "QEP input clock divider"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "QEP input clock divider"]
    #[inline(always)]
    pub fn set_clkdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Qepctl {
    #[inline(always)]
    fn default() -> Qepctl {
        Qepctl(0)
    }
}
#[doc = "Timer QEP Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qepctr(pub u32);
impl Qepctr {
    #[doc = "Counter value"]
    #[inline(always)]
    pub const fn ctr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Counter value"]
    #[inline(always)]
    pub fn set_ctr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Qepctr {
    #[inline(always)]
    fn default() -> Qepctr {
        Qepctr(0)
    }
}
#[doc = "Timer QEP Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qepier(pub u32);
impl Qepier {
    #[doc = "Direction change interrupt enable"]
    #[inline(always)]
    pub const fn dirie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Direction change interrupt enable"]
    #[inline(always)]
    pub fn set_dirie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Phase A rising edge interrupt enable"]
    #[inline(always)]
    pub const fn phaie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Phase A rising edge interrupt enable"]
    #[inline(always)]
    pub fn set_phaie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Phase B rising edge interrupt enable"]
    #[inline(always)]
    pub const fn phbie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Phase B rising edge interrupt enable"]
    #[inline(always)]
    pub fn set_phbie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Counter wrap interrupt enable"]
    #[inline(always)]
    pub const fn wrie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Counter wrap interrupt enable"]
    #[inline(always)]
    pub fn set_wrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Index event interrupt enable"]
    #[inline(always)]
    pub const fn idxevie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Index event interrupt enable"]
    #[inline(always)]
    pub fn set_idxevie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Qepier {
    #[inline(always)]
    fn default() -> Qepier {
        Qepier(0)
    }
}
#[doc = "Timer QEP Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qepif(pub u32);
impl Qepif {
    #[doc = "Direction change interrupt flag"]
    #[inline(always)]
    pub const fn dirif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Direction change interrupt flag"]
    #[inline(always)]
    pub fn set_dirif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Phase A rising edge interrupt flag"]
    #[inline(always)]
    pub const fn phaif(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Phase A rising edge interrupt flag"]
    #[inline(always)]
    pub fn set_phaif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Phase B rising edge interrupt flag"]
    #[inline(always)]
    pub const fn phbif(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Phase B rising edge interrupt flag"]
    #[inline(always)]
    pub fn set_phbif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Counter wrap interrupt flag"]
    #[inline(always)]
    pub const fn wrif(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Counter wrap interrupt flag"]
    #[inline(always)]
    pub fn set_wrif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Index event interrupt flag"]
    #[inline(always)]
    pub const fn idxevif(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Index event interrupt flag"]
    #[inline(always)]
    pub fn set_idxevif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Qepif {
    #[inline(always)]
    fn default() -> Qepif {
        Qepif(0)
    }
}
#[doc = "Timer QEP Maximum Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qepmax(pub u32);
impl Qepmax {
    #[doc = "Maximum Counter value"]
    #[inline(always)]
    pub const fn ctrmax(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Maximum Counter value"]
    #[inline(always)]
    pub fn set_ctrmax(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Qepmax {
    #[inline(always)]
    fn default() -> Qepmax {
        Qepmax(0)
    }
}
