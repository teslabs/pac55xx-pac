#![no_std]
#![allow(non_camel_case_types)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (2a63600 2024-11-04))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - MEMCTL Interrupt"]
    MEMCTL = 0,
    #[doc = "1 - WWDT Interrupt"]
    WWDT = 1,
    #[doc = "2 - RTC Interrupt"]
    RTC = 2,
    #[doc = "3 - ADC 0 Interrupt"]
    ADC0 = 3,
    #[doc = "4 - ADC 1 Interrupt"]
    ADC1 = 4,
    #[doc = "5 - ADC 2 Interrupt"]
    ADC2 = 5,
    #[doc = "6 - ADC 3 Interrupt"]
    ADC3 = 6,
    #[doc = "7 - Timer A Interrupt"]
    TIMERA = 7,
    #[doc = "8 - Timer B Interrupt"]
    TIMERB = 8,
    #[doc = "9 - Timer C Interrupt"]
    TIMERC = 9,
    #[doc = "10 - Timer D Interrupt"]
    TIMERD = 10,
    #[doc = "11 - Timer A QEP Interrupt"]
    TIMERAQEP = 11,
    #[doc = "12 - Timer B QEP Interrupt"]
    TIMERBQEP = 12,
    #[doc = "13 - Timer C QEP Interrupt"]
    TIMERCQEP = 13,
    #[doc = "14 - Timer D QEP Interrupt"]
    TIMERDQEP = 14,
    #[doc = "15 - GPIOA Interrupt"]
    GPIOA = 15,
    #[doc = "16 - GPIO B Interrupt"]
    GPIOB = 16,
    #[doc = "17 - GPIO C Interrupt"]
    GPIOC = 17,
    #[doc = "18 - GPIO D Interrupt"]
    GPIOD = 18,
    #[doc = "19 - GPIO E Interrupt"]
    GPIOE = 19,
    #[doc = "20 - GPIO F Interrupt"]
    GPIOF = 20,
    #[doc = "21 - GPIO G Interrupt"]
    GPIOG = 21,
    #[doc = "22 - I2C Interrupt"]
    I2C = 22,
    #[doc = "23 - USART/SSP A Interrupt"]
    USARTA_SSPA = 23,
    #[doc = "24 - USART/SSP B Interrupt"]
    USARTB_SSPB = 24,
    #[doc = "25 - USART/SSP C Interrupt"]
    USARTC_SSPC = 25,
    #[doc = "26 - USART/SSP D Interrupt"]
    USARTD_SSPD = 26,
    #[doc = "27 - CAN Interrupt"]
    CAN = 27,
    #[doc = "28 - GPTimer A Interrupt"]
    GPTIMERA = 28,
    #[doc = "29 - GPTimer B Interrupt"]
    GPTIMERB = 29,
    #[doc = "30 - SCC Interrupt"]
    SCC = 30,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "Analog to Digital Converter (ADC) and Dynamic Triggering and Sampling Engine (DTSE)"]
pub const ADC: adc::Adc = unsafe { adc::Adc::from_ptr(0x4000_0000usize as _) };
#[doc = "Inter-Integrated Circuit"]
pub const I2C: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4001_0000usize as _) };
#[doc = "Universal Synchronous Asynchronous Receive Transmit"]
pub const USARTA: usart::Usart = unsafe { usart::Usart::from_ptr(0x4002_0000usize as _) };
#[doc = "Synchronous Serial Peripheral"]
pub const SSPA: ssp::Ssp = unsafe { ssp::Ssp::from_ptr(0x4002_0000usize as _) };
#[doc = "Universal Synchronous Asynchronous Receive Transmit"]
pub const USARTB: usart::Usart = unsafe { usart::Usart::from_ptr(0x4003_0000usize as _) };
#[doc = "Synchronous Serial Peripheral"]
pub const SSPB: ssp::Ssp = unsafe { ssp::Ssp::from_ptr(0x4003_0000usize as _) };
#[doc = "Universal Synchronous Asynchronous Receive Transmit"]
pub const USARTC: usart::Usart = unsafe { usart::Usart::from_ptr(0x4004_0000usize as _) };
#[doc = "Synchronous Serial Peripheral"]
pub const SSPC: ssp::Ssp = unsafe { ssp::Ssp::from_ptr(0x4004_0000usize as _) };
#[doc = "Universal Synchronous Asynchronous Receive Transmit"]
pub const USARTD: usart::Usart = unsafe { usart::Usart::from_ptr(0x4005_0000usize as _) };
#[doc = "Synchronous Serial Peripheral"]
pub const SSPD: ssp::Ssp = unsafe { ssp::Ssp::from_ptr(0x4005_0000usize as _) };
#[doc = "Timer"]
pub const TIMERA: timer::Timer = unsafe { timer::Timer::from_ptr(0x4006_0000usize as _) };
#[doc = "Timer"]
pub const TIMERB: timer::Timer = unsafe { timer::Timer::from_ptr(0x4007_0000usize as _) };
#[doc = "Timer"]
pub const TIMERC: timer::Timer = unsafe { timer::Timer::from_ptr(0x4008_0000usize as _) };
#[doc = "Timer"]
pub const TIMERD: timer::Timer = unsafe { timer::Timer::from_ptr(0x4009_0000usize as _) };
#[doc = "Controller Area Network"]
pub const CAN: can::Can = unsafe { can::Can::from_ptr(0x400a_0000usize as _) };
#[doc = "General Purpose Timer"]
pub const GPTIMERA: gptimer::Gptimer = unsafe { gptimer::Gptimer::from_ptr(0x400b_0000usize as _) };
#[doc = "General Purpose Timer"]
pub const GPTIMERB: gptimer::Gptimer = unsafe { gptimer::Gptimer::from_ptr(0x400c_0000usize as _) };
#[doc = "Memory Controller"]
pub const MEMCTL: memctl::Memctl = unsafe { memctl::Memctl::from_ptr(0x400d_0000usize as _) };
#[doc = "System and Clock Control"]
pub const SCC: scc::Scc = unsafe { scc::Scc::from_ptr(0x400d_0400usize as _) };
#[doc = "Windowed Watchdog Timer"]
pub const WWDT: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x400d_0800usize as _) };
#[doc = "Real-Time Clock (RTC) with Calendar"]
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x400d_0c00usize as _) };
#[doc = "Cyclic Redundancy Check"]
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x400d_1000usize as _) };
#[doc = "General Purpose Input/Output"]
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x400d_1400usize as _) };
#[doc = "General Purpose Input/Output"]
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x400d_1800usize as _) };
#[doc = "General Purpose Input/Output"]
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x400d_1c00usize as _) };
#[doc = "General Purpose Input/Output"]
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x400d_2000usize as _) };
#[doc = "General Purpose Input/Output"]
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x400d_2400usize as _) };
#[doc = "General Purpose Input/Output"]
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x400d_2800usize as _) };
#[doc = "General Purpose Input/Output"]
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x400d_2c00usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod adc;
pub mod can;
pub mod common;
pub mod crc;
pub mod gpio;
pub mod gptimer;
pub mod i2c;
pub mod memctl;
pub mod rtc;
pub mod scc;
pub mod ssp;
pub mod timer;
pub mod usart;
pub mod wwdt;
