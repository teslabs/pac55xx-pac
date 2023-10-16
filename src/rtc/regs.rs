#[doc = "RTC Alarm Setting"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alarmset(pub u32);
impl Alarmset {
    #[doc = "Minute alarm"]
    #[inline(always)]
    pub const fn minutealarm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Minute alarm"]
    #[inline(always)]
    pub fn set_minutealarm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Minute alarm enable"]
    #[inline(always)]
    pub const fn minutealarmen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Minute alarm enable"]
    #[inline(always)]
    pub fn set_minutealarmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hour alarm"]
    #[inline(always)]
    pub const fn houralarm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Hour alarm"]
    #[inline(always)]
    pub fn set_houralarm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Hour alarm enable"]
    #[inline(always)]
    pub const fn houralarmen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Hour alarm enable"]
    #[inline(always)]
    pub fn set_houralarmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Day of week alarm"]
    #[inline(always)]
    pub const fn dayofweekalarm(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Day of week alarm"]
    #[inline(always)]
    pub fn set_dayofweekalarm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Day of week alarm enable"]
    #[inline(always)]
    pub const fn dayofweekalarmen(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Day of week alarm enable"]
    #[inline(always)]
    pub fn set_dayofweekalarmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Day alarm"]
    #[inline(always)]
    pub const fn dayalarm(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Day alarm"]
    #[inline(always)]
    pub fn set_dayalarm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "Day alarm enable"]
    #[inline(always)]
    pub const fn dayalarmen(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Day alarm enable"]
    #[inline(always)]
    pub fn set_dayalarmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Alarmset {
    #[inline(always)]
    fn default() -> Alarmset {
        Alarmset(0)
    }
}
#[doc = "RTC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "Enable RTC"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable RTC"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Alarm match interrupt enable"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm match interrupt enable"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Alarm match interrupt flag"]
    #[inline(always)]
    pub const fn if_(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm match interrupt flag"]
    #[inline(always)]
    pub fn set_if_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clock divider for the RTC input clock (FRCLK)"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> super::vals::Clkdiv {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Clkdiv::from_bits(val as u8)
    }
    #[doc = "Clock divider for the RTC input clock (FRCLK)"]
    #[inline(always)]
    pub fn set_clkdiv(&mut self, val: super::vals::Clkdiv) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
    #[doc = "Set the internal TIME/DATA with TIMESET/DATESET values"]
    #[inline(always)]
    pub const fn setcal(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set the internal TIME/DATA with TIMESET/DATESET values"]
    #[inline(always)]
    pub fn set_setcal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
#[doc = "RTC Date"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Date(pub u32);
impl Date {
    #[doc = "Day of week"]
    #[inline(always)]
    pub const fn dayofweek(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Day of week"]
    #[inline(always)]
    pub fn set_dayofweek(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Day"]
    #[inline(always)]
    pub const fn day(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Day"]
    #[inline(always)]
    pub fn set_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Month"]
    #[inline(always)]
    pub const fn month(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Month"]
    #[inline(always)]
    pub fn set_month(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Year"]
    #[inline(always)]
    pub const fn year(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Year"]
    #[inline(always)]
    pub fn set_year(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Date {
    #[inline(always)]
    fn default() -> Date {
        Date(0)
    }
}
#[doc = "RTC Date Setting"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dateset(pub u32);
impl Dateset {
    #[doc = "Day of week"]
    #[inline(always)]
    pub const fn dayofweek(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Day of week"]
    #[inline(always)]
    pub fn set_dayofweek(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Day"]
    #[inline(always)]
    pub const fn day(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Day"]
    #[inline(always)]
    pub fn set_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Month"]
    #[inline(always)]
    pub const fn month(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Month"]
    #[inline(always)]
    pub fn set_month(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Year"]
    #[inline(always)]
    pub const fn year(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Year"]
    #[inline(always)]
    pub fn set_year(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Dateset {
    #[inline(always)]
    fn default() -> Dateset {
        Dateset(0)
    }
}
#[doc = "RTC Time"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Time(pub u32);
impl Time {
    #[doc = "Seconds"]
    #[inline(always)]
    pub const fn seconds(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Seconds"]
    #[inline(always)]
    pub fn set_seconds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Minutes"]
    #[inline(always)]
    pub const fn minutes(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Minutes"]
    #[inline(always)]
    pub fn set_minutes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Hours"]
    #[inline(always)]
    pub const fn hours(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Hours"]
    #[inline(always)]
    pub fn set_hours(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Time {
    #[inline(always)]
    fn default() -> Time {
        Time(0)
    }
}
#[doc = "RTC Time Setting"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timeset(pub u32);
impl Timeset {
    #[doc = "Seconds"]
    #[inline(always)]
    pub const fn seconds(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Seconds"]
    #[inline(always)]
    pub fn set_seconds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Minutes"]
    #[inline(always)]
    pub const fn minutes(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Minutes"]
    #[inline(always)]
    pub fn set_minutes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Hours"]
    #[inline(always)]
    pub const fn hours(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Hours"]
    #[inline(always)]
    pub fn set_hours(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Timeset {
    #[inline(always)]
    fn default() -> Timeset {
        Timeset(0)
    }
}
