#[doc = "FLASH Erase"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flasherase(pub u32);
impl Flasherase {
    #[doc = "Key for erase operations"]
    #[inline(always)]
    pub const fn key(&self) -> super::vals::Key {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Key::from_bits(val as u32)
    }
    #[doc = "Key for erase operations"]
    #[inline(always)]
    pub fn set_key(&mut self, val: super::vals::Key) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Flasherase {
    #[inline(always)]
    fn default() -> Flasherase {
        Flasherase(0)
    }
}
#[doc = "FLASH Lock Access"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashlock(pub u32);
impl Flashlock {
    #[doc = "Flash Lock Value for Memory Controller Operations"]
    #[inline(always)]
    pub const fn flashlock(&self) -> super::vals::Flashlock {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Flashlock::from_bits(val as u32)
    }
    #[doc = "Flash Lock Value for Memory Controller Operations"]
    #[inline(always)]
    pub fn set_flashlock(&mut self, val: super::vals::Flashlock) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Flashlock {
    #[inline(always)]
    fn default() -> Flashlock {
        Flashlock(0)
    }
}
#[doc = "FLASH Page"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashpage(pub u32);
impl Flashpage {
    #[doc = "Flash page selection"]
    #[inline(always)]
    pub const fn page(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Flash page selection"]
    #[inline(always)]
    pub fn set_page(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Flashpage {
    #[inline(always)]
    fn default() -> Flashpage {
        Flashpage(0)
    }
}
#[doc = "Memory Controller Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memctl(pub u32);
impl Memctl {
    #[doc = "FLASH Access Wait States"]
    #[inline(always)]
    pub const fn wstate(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "FLASH Access Wait States"]
    #[inline(always)]
    pub fn set_wstate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "MCLK Divider"]
    #[inline(always)]
    pub const fn mclkdiv(&self) -> super::vals::Mclkdiv {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Mclkdiv::from_bits(val as u8)
    }
    #[doc = "MCLK Divider"]
    #[inline(always)]
    pub fn set_mclkdiv(&mut self, val: super::vals::Mclkdiv) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Write Word Data Count"]
    #[inline(always)]
    pub const fn writewordcnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Write Word Data Count"]
    #[inline(always)]
    pub fn set_writewordcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "ECC Single bit Error Interrupt Enable"]
    #[inline(always)]
    pub const fn seie(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ECC Single bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_seie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ECC Dual bit Error Interrupt Enable"]
    #[inline(always)]
    pub const fn deie(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ECC Dual bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_deie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Invalid Memory Address Access Interrupt Enable"]
    #[inline(always)]
    pub const fn invaddrie(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid Memory Address Access Interrupt Enable"]
    #[inline(always)]
    pub fn set_invaddrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "FLASH Standby Mode"]
    #[inline(always)]
    pub const fn stdby(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "FLASH Standby Mode"]
    #[inline(always)]
    pub fn set_stdby(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SRAM ECC Disable"]
    #[inline(always)]
    pub const fn eccdis(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM ECC Disable"]
    #[inline(always)]
    pub fn set_eccdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "FLASH Read Cache Disable"]
    #[inline(always)]
    pub const fn cachedis(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "FLASH Read Cache Disable"]
    #[inline(always)]
    pub fn set_cachedis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "MCLK Mux Select"]
    #[inline(always)]
    pub const fn mclksel(&self) -> super::vals::Mclksel {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Mclksel::from_bits(val as u8)
    }
    #[doc = "MCLK Mux Select"]
    #[inline(always)]
    pub fn set_mclksel(&mut self, val: super::vals::Mclksel) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
}
impl Default for Memctl {
    #[inline(always)]
    fn default() -> Memctl {
        Memctl(0)
    }
}
#[doc = "Memory Controller Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memstatus(pub u32);
impl Memstatus {
    #[doc = "FLASH Write Busy"]
    #[inline(always)]
    pub const fn wbusy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLASH Write Busy"]
    #[inline(always)]
    pub fn set_wbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FLASH Erase Busy"]
    #[inline(always)]
    pub const fn ebusy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FLASH Erase Busy"]
    #[inline(always)]
    pub fn set_ebusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Number of bytes written to FLASH for the write data buffer"]
    #[inline(always)]
    pub const fn writewordcnt(&self) -> super::vals::Writewordcnt {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Writewordcnt::from_bits(val as u8)
    }
    #[doc = "Number of bytes written to FLASH for the write data buffer"]
    #[inline(always)]
    pub fn set_writewordcnt(&mut self, val: super::vals::Writewordcnt) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "ECC Single bit Error Detection Flag"]
    #[inline(always)]
    pub const fn se(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ECC Single bit Error Detection Flag"]
    #[inline(always)]
    pub fn set_se(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ECC Dual bit Error Detection Flag"]
    #[inline(always)]
    pub const fn de(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ECC Dual bit Error Detection Flag"]
    #[inline(always)]
    pub fn set_de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Invalid Memory Address Access Flag"]
    #[inline(always)]
    pub const fn invaddr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid Memory Address Access Flag"]
    #[inline(always)]
    pub fn set_invaddr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Memstatus {
    #[inline(always)]
    fn default() -> Memstatus {
        Memstatus(0)
    }
}
#[doc = "SWD Unlock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdunlock(pub u32);
impl Swdunlock {
    #[doc = "Key for SWD unlock"]
    #[inline(always)]
    pub const fn key_swdunlock(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Key for SWD unlock"]
    #[inline(always)]
    pub fn set_key_swdunlock(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Swdunlock {
    #[inline(always)]
    fn default() -> Swdunlock {
        Swdunlock(0)
    }
}
