#[doc = "ADC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcctl(pub u32);
impl Adcctl {
    #[doc = "ADC Clock Divider"]
    #[inline(always)]
    pub const fn adcdiv(&self) -> super::vals::Adcdiv {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Adcdiv::from_bits(val as u8)
    }
    #[doc = "ADC Clock Divider"]
    #[inline(always)]
    pub fn set_adcdiv(&mut self, val: super::vals::Adcdiv) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "ADC Channel Select"]
    #[inline(always)]
    pub const fn channel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "ADC Channel Select"]
    #[inline(always)]
    pub fn set_channel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "ADC Busy"]
    #[inline(always)]
    pub const fn adbusy(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Busy"]
    #[inline(always)]
    pub fn set_adbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "ADC Mode"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "ADC Mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "ADC Repeat"]
    #[inline(always)]
    pub const fn repeat(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Repeat"]
    #[inline(always)]
    pub fn set_repeat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "ADC Start"]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Start"]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ADC Enable"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Enable"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Manual Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn intenman(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Manual Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn set_intenman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EMUX Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn intenemux(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EMUX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn set_intenemux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Sequence Collision Interrupt Enable"]
    #[inline(always)]
    pub const fn intencol(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Collision Interrupt Enable"]
    #[inline(always)]
    pub fn set_intencol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC Accuracy Mode Enable"]
    #[inline(always)]
    pub const fn enacc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Accuracy Mode Enable"]
    #[inline(always)]
    pub fn set_enacc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Adcctl {
    #[inline(always)]
    fn default() -> Adcctl {
        Adcctl(0)
    }
}
#[doc = "ADC Interrupt Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcint(pub u32);
impl Adcint {
    #[doc = "DTSE Sequence Interrupt Flag"]
    #[inline(always)]
    pub const fn intf(&self, n: usize) -> bool {
        assert!(n < 24usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "DTSE Sequence Interrupt Flag"]
    #[inline(always)]
    pub fn set_intf(&mut self, n: usize, val: bool) {
        assert!(n < 24usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "DTSE Manual Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub const fn intfman(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DTSE Manual Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn set_intfman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DTSE EMUX Complete Interrupt Flag"]
    #[inline(always)]
    pub const fn intfemux(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DTSE EMUX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn set_intfemux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DTSE Sequence Collision Interrupt Flag"]
    #[inline(always)]
    pub const fn intfcol(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DTSE Sequence Collision Interrupt Flag"]
    #[inline(always)]
    pub fn set_intfcol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ADCIRQ Interrupt Flag"]
    #[inline(always)]
    pub const fn adcirqif(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 28usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "ADCIRQ Interrupt Flag"]
    #[inline(always)]
    pub fn set_adcirqif(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 28usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Adcint {
    #[inline(always)]
    fn default() -> Adcint {
        Adcint(0)
    }
}
#[doc = "ADC Result"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcres(pub u32);
impl Adcres {
    #[doc = "ADC Conversion Result (manual mode)"]
    #[inline(always)]
    pub const fn adcres(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "ADC Conversion Result (manual mode)"]
    #[inline(always)]
    pub fn set_adcres(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Adcres {
    #[inline(always)]
    fn default() -> Adcres {
        Adcres(0)
    }
}
#[doc = "DTSE Result"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtseres(pub u32);
impl Dtseres {
    #[doc = "DTSE Conversion Result"]
    #[inline(always)]
    pub const fn res(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "DTSE Conversion Result"]
    #[inline(always)]
    pub fn set_res(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Dtseres {
    #[inline(always)]
    fn default() -> Dtseres {
        Dtseres(0)
    }
}
#[doc = "DTSE Sequence Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtseseqcfg(pub u32);
impl Dtseseqcfg {
    #[doc = "EMUX Control"]
    #[inline(always)]
    pub const fn emuxc(&self) -> super::vals::DtseseqcfgEmuxc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DtseseqcfgEmuxc::from_bits(val as u8)
    }
    #[doc = "EMUX Control"]
    #[inline(always)]
    pub fn set_emuxc(&mut self, val: super::vals::DtseseqcfgEmuxc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Channel"]
    #[inline(always)]
    pub const fn channel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Channel"]
    #[inline(always)]
    pub fn set_channel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "Final sequence of series"]
    #[inline(always)]
    pub const fn seqdone(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Final sequence of series"]
    #[inline(always)]
    pub fn set_seqdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EMUX Data"]
    #[inline(always)]
    pub const fn emuxd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "EMUX Data"]
    #[inline(always)]
    pub fn set_emuxd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "IRQ Number to assert"]
    #[inline(always)]
    pub const fn irqnum(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "IRQ Number to assert"]
    #[inline(always)]
    pub fn set_irqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Assert IRQ after converting this sequence"]
    #[inline(always)]
    pub const fn irqen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Assert IRQ after converting this sequence"]
    #[inline(always)]
    pub fn set_irqen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "No ADC conversion"]
    #[inline(always)]
    pub const fn nocvt(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "No ADC conversion"]
    #[inline(always)]
    pub fn set_nocvt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Dtseseqcfg {
    #[inline(always)]
    fn default() -> Dtseseqcfg {
        Dtseseqcfg(0)
    }
}
#[doc = "DTSE Trigger Entry 0 to 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsetrigent0to3(pub u32);
impl Dtsetrigent0to3 {
    #[doc = "DTSE Trigger 0 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig0cfgidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 0 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig0cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "DTSE Trigger 0 Edge Configuration"]
    #[inline(always)]
    pub const fn trig0edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 0 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig0edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Force Trigger 0"]
    #[inline(always)]
    pub const fn force0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 0"]
    #[inline(always)]
    pub fn set_force0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DTSE Trigger 1 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig1cfgidx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 1 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig1cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "DTSE Trigger 1 Edge Configuration"]
    #[inline(always)]
    pub const fn trig1edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 1 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig1edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Force Trigger 1"]
    #[inline(always)]
    pub const fn force1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 1"]
    #[inline(always)]
    pub fn set_force1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DTSE Trigger 2 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig2cfgidx(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 2 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig2cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "DTSE Trigger 2 Edge Configuration"]
    #[inline(always)]
    pub const fn trig2edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 2 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig2edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Force Trigger 2"]
    #[inline(always)]
    pub const fn force2(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 2"]
    #[inline(always)]
    pub fn set_force2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DTSE Trigger 3 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig3cfgidx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 3 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig3cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "DTSE Trigger 3 Edge Configuration"]
    #[inline(always)]
    pub const fn trig3edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 3 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig3edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "Force Trigger 3"]
    #[inline(always)]
    pub const fn force3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 3"]
    #[inline(always)]
    pub fn set_force3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dtsetrigent0to3 {
    #[inline(always)]
    fn default() -> Dtsetrigent0to3 {
        Dtsetrigent0to3(0)
    }
}
#[doc = "DTSE Trigger Entry 12 to 15"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsetrigent12to15(pub u32);
impl Dtsetrigent12to15 {
    #[doc = "DTSE Trigger 12 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig12cfgidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 12 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig12cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "DTSE Trigger 12 Edge Configuration"]
    #[inline(always)]
    pub const fn trig12edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 12 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig12edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Force Trigger 12"]
    #[inline(always)]
    pub const fn force12(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 12"]
    #[inline(always)]
    pub fn set_force12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DTSE Trigger 13 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig13cfgidx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 13 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig13cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "DTSE Trigger 13 Edge Configuration"]
    #[inline(always)]
    pub const fn trig13edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 13 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig13edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Force Trigger 13"]
    #[inline(always)]
    pub const fn force13(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 13"]
    #[inline(always)]
    pub fn set_force13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DTSE Trigger 14 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig14cfgidx(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 14 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig14cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "DTSE Trigger 14 Edge Configuration"]
    #[inline(always)]
    pub const fn trig14edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 14 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig14edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Force Trigger 14"]
    #[inline(always)]
    pub const fn force14(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 14"]
    #[inline(always)]
    pub fn set_force14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DTSE Trigger 15 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig15cfgidx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 15 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig15cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "DTSE Trigger 15 Edge Configuration"]
    #[inline(always)]
    pub const fn trig15edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 15 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig15edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "Force Trigger 15"]
    #[inline(always)]
    pub const fn force15(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 15"]
    #[inline(always)]
    pub fn set_force15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dtsetrigent12to15 {
    #[inline(always)]
    fn default() -> Dtsetrigent12to15 {
        Dtsetrigent12to15(0)
    }
}
#[doc = "DTSE Trigger Entry 16 to 19"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsetrigent16to19(pub u32);
impl Dtsetrigent16to19 {
    #[doc = "DTSE Trigger 16 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig16cfgidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 16 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig16cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "DTSE Trigger 16 Edge Configuration"]
    #[inline(always)]
    pub const fn trig16edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 16 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig16edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Force Trigger 16"]
    #[inline(always)]
    pub const fn force16(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 16"]
    #[inline(always)]
    pub fn set_force16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DTSE Trigger 17 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig17cfgidx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 17 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig17cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "DTSE Trigger 17 Edge Configuration"]
    #[inline(always)]
    pub const fn trig17edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 17 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig17edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Force Trigger 17"]
    #[inline(always)]
    pub const fn force17(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 17"]
    #[inline(always)]
    pub fn set_force17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DTSE Trigger 18 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig18cfgidx(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 18 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig18cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "DTSE Trigger 18 Edge Configuration"]
    #[inline(always)]
    pub const fn trig18edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 18 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig18edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Force Trigger 18"]
    #[inline(always)]
    pub const fn force18(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 18"]
    #[inline(always)]
    pub fn set_force18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DTSE Trigger 19 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig19cfgidx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 19 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig19cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "DTSE Trigger 19 Edge Configuration"]
    #[inline(always)]
    pub const fn trig19edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 19 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig19edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "Force Trigger 19"]
    #[inline(always)]
    pub const fn force19(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 19"]
    #[inline(always)]
    pub fn set_force19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dtsetrigent16to19 {
    #[inline(always)]
    fn default() -> Dtsetrigent16to19 {
        Dtsetrigent16to19(0)
    }
}
#[doc = "DTSE Trigger Entry 20 to 23"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsetrigent20to23(pub u32);
impl Dtsetrigent20to23 {
    #[doc = "DTSE Trigger 20 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig20cfgidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 20 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig20cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "DTSE Trigger 20 Edge Configuration"]
    #[inline(always)]
    pub const fn trig20edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 20 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig20edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Force Trigger 20"]
    #[inline(always)]
    pub const fn force20(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 20"]
    #[inline(always)]
    pub fn set_force20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DTSE Trigger 21 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig21cfgidx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 21 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig21cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "DTSE Trigger 21 Edge Configuration"]
    #[inline(always)]
    pub const fn trig21edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 21 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig21edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Force Trigger 21"]
    #[inline(always)]
    pub const fn force21(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 21"]
    #[inline(always)]
    pub fn set_force21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DTSE Trigger 22 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig22cfgidx(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 22 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig22cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "DTSE Trigger 22 Edge Configuration"]
    #[inline(always)]
    pub const fn trig22edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 22 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig22edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Force Trigger 22"]
    #[inline(always)]
    pub const fn force22(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 22"]
    #[inline(always)]
    pub fn set_force22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DTSE Trigger 23 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig23cfgidx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 23 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig23cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "DTSE Trigger 23 Edge Configuration"]
    #[inline(always)]
    pub const fn trig23edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 23 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig23edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "Force Trigger 23"]
    #[inline(always)]
    pub const fn force23(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 23"]
    #[inline(always)]
    pub fn set_force23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dtsetrigent20to23 {
    #[inline(always)]
    fn default() -> Dtsetrigent20to23 {
        Dtsetrigent20to23(0)
    }
}
#[doc = "DTSE Trigger Entry 24 to 27"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsetrigent24to27(pub u32);
impl Dtsetrigent24to27 {
    #[doc = "DTSE Trigger 24 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig24cfgidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 24 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig24cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "DTSE Trigger 24 Edge Configuration"]
    #[inline(always)]
    pub const fn trig24edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 24 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig24edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Force Trigger 24"]
    #[inline(always)]
    pub const fn force24(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 24"]
    #[inline(always)]
    pub fn set_force24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DTSE Trigger 25 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig25cfgidx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 25 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig25cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "DTSE Trigger 25 Edge Configuration"]
    #[inline(always)]
    pub const fn trig25edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 25 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig25edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Force Trigger 25"]
    #[inline(always)]
    pub const fn force25(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 25"]
    #[inline(always)]
    pub fn set_force25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DTSE Trigger 26 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig26cfgidx(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 26 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig26cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "DTSE Trigger 26 Edge Configuration"]
    #[inline(always)]
    pub const fn trig26edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 26 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig26edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Force Trigger 26"]
    #[inline(always)]
    pub const fn force26(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 26"]
    #[inline(always)]
    pub fn set_force26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DTSE Trigger 27 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig27cfgidx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 27 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig27cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "DTSE Trigger 27 Edge Configuration"]
    #[inline(always)]
    pub const fn trig27edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 27 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig27edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "Force Trigger 27"]
    #[inline(always)]
    pub const fn force27(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 27"]
    #[inline(always)]
    pub fn set_force27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dtsetrigent24to27 {
    #[inline(always)]
    fn default() -> Dtsetrigent24to27 {
        Dtsetrigent24to27(0)
    }
}
#[doc = "DTSE Trigger Entry 28 to 31"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsetrigent28to31(pub u32);
impl Dtsetrigent28to31 {
    #[doc = "DTSE Trigger 28 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig28cfgidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 28 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig28cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "DTSE Trigger 28 Edge Configuration"]
    #[inline(always)]
    pub const fn trig28edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 28 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig28edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Force Trigger 28"]
    #[inline(always)]
    pub const fn force28(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 28"]
    #[inline(always)]
    pub fn set_force28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DTSE Trigger 29 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig29cfgidx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 29 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig29cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "DTSE Trigger 29 Edge Configuration"]
    #[inline(always)]
    pub const fn trig29edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 29 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig29edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Force Trigger 29"]
    #[inline(always)]
    pub const fn force29(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 29"]
    #[inline(always)]
    pub fn set_force29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DTSE Trigger 30 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig30cfgidx(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 30 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig30cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "DTSE Trigger 30 Edge Configuration"]
    #[inline(always)]
    pub const fn trig30edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 30 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig30edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Force Trigger 30"]
    #[inline(always)]
    pub const fn force30(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 30"]
    #[inline(always)]
    pub fn set_force30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DTSE Trigger 31 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig31cfgidx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 31 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig31cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "DTSE Trigger 31 Edge Configuration"]
    #[inline(always)]
    pub const fn trig31edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 31 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig31edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "Force Trigger 31"]
    #[inline(always)]
    pub const fn force31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 31"]
    #[inline(always)]
    pub fn set_force31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dtsetrigent28to31 {
    #[inline(always)]
    fn default() -> Dtsetrigent28to31 {
        Dtsetrigent28to31(0)
    }
}
#[doc = "DTSE Trigger Entry 4 to 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsetrigent4to7(pub u32);
impl Dtsetrigent4to7 {
    #[doc = "DTSE Trigger 4 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig4cfgidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 4 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig4cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "DTSE Trigger 4 Edge Configuration"]
    #[inline(always)]
    pub const fn trig4edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 4 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig4edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Force Trigger 4"]
    #[inline(always)]
    pub const fn force4(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 4"]
    #[inline(always)]
    pub fn set_force4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DTSE Trigger 5 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig5cfgidx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 5 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig5cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "DTSE Trigger 5 Edge Configuration"]
    #[inline(always)]
    pub const fn trig5edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 5 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig5edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Force Trigger 5"]
    #[inline(always)]
    pub const fn force5(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 5"]
    #[inline(always)]
    pub fn set_force5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DTSE Trigger 6 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig6cfgidx(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 6 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig6cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "DTSE Trigger 6 Edge Configuration"]
    #[inline(always)]
    pub const fn trig6edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 6 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig6edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Force Trigger 6"]
    #[inline(always)]
    pub const fn force6(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 6"]
    #[inline(always)]
    pub fn set_force6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DTSE Trigger 7 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig7cfgidx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 7 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig7cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "DTSE Trigger 7 Edge Configuration"]
    #[inline(always)]
    pub const fn trig7edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 7 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig7edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "Force Trigger 7"]
    #[inline(always)]
    pub const fn force7(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 7"]
    #[inline(always)]
    pub fn set_force7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dtsetrigent4to7 {
    #[inline(always)]
    fn default() -> Dtsetrigent4to7 {
        Dtsetrigent4to7(0)
    }
}
#[doc = "DTSE Trigger Entry 8 to 11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsetrigent8to11(pub u32);
impl Dtsetrigent8to11 {
    #[doc = "DTSE Trigger 8 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig8cfgidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 8 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig8cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "DTSE Trigger 8 Edge Configuration"]
    #[inline(always)]
    pub const fn trig8edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 8 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig8edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Force Trigger 8"]
    #[inline(always)]
    pub const fn force8(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 8"]
    #[inline(always)]
    pub fn set_force8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DTSE Trigger 9 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig9cfgidx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 9 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig9cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "DTSE Trigger 9 Edge Configuration"]
    #[inline(always)]
    pub const fn trig9edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 9 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig9edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Force Trigger 9"]
    #[inline(always)]
    pub const fn force9(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 9"]
    #[inline(always)]
    pub fn set_force9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DTSE Trigger 10 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig10cfgidx(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 10 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig10cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "DTSE Trigger 10 Edge Configuration"]
    #[inline(always)]
    pub const fn trig10edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 10 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig10edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Force Trigger 10"]
    #[inline(always)]
    pub const fn force10(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 10"]
    #[inline(always)]
    pub fn set_force10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DTSE Trigger 11 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub const fn trig11cfgidx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "DTSE Trigger 11 Sequence Configuration Entry Index"]
    #[inline(always)]
    pub fn set_trig11cfgidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "DTSE Trigger 11 Edge Configuration"]
    #[inline(always)]
    pub const fn trig11edge(&self) -> super::vals::Trigedge {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trigedge::from_bits(val as u8)
    }
    #[doc = "DTSE Trigger 11 Edge Configuration"]
    #[inline(always)]
    pub fn set_trig11edge(&mut self, val: super::vals::Trigedge) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "Force Trigger 11"]
    #[inline(always)]
    pub const fn force11(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Force Trigger 11"]
    #[inline(always)]
    pub fn set_force11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dtsetrigent8to11 {
    #[inline(always)]
    fn default() -> Dtsetrigent8to11 {
        Dtsetrigent8to11(0)
    }
}
#[doc = "EMUX Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emuxctl(pub u32);
impl Emuxctl {
    #[doc = "EMUX Clock Divider"]
    #[inline(always)]
    pub const fn emuxdiv(&self) -> super::vals::Emuxdiv {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Emuxdiv::from_bits(val as u8)
    }
    #[doc = "EMUX Clock Divider"]
    #[inline(always)]
    pub fn set_emuxdiv(&mut self, val: super::vals::Emuxdiv) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "EMUX Busy"]
    #[inline(always)]
    pub const fn emuxbusy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EMUX Busy"]
    #[inline(always)]
    pub fn set_emuxbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "EMUX Mode"]
    #[inline(always)]
    pub const fn emuxmode(&self) -> super::vals::Emuxmode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Emuxmode::from_bits(val as u8)
    }
    #[doc = "EMUX Mode"]
    #[inline(always)]
    pub fn set_emuxmode(&mut self, val: super::vals::Emuxmode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Emuxctl {
    #[inline(always)]
    fn default() -> Emuxctl {
        Emuxctl(0)
    }
}
#[doc = "EMUX Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emuxdata(pub u32);
impl Emuxdata {
    #[doc = "Write data onto the EMUX"]
    #[inline(always)]
    pub const fn emuxdata(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write data onto the EMUX"]
    #[inline(always)]
    pub fn set_emuxdata(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Emuxdata {
    #[inline(always)]
    fn default() -> Emuxdata {
        Emuxdata(0)
    }
}
