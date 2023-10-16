extern "C" {
    fn MEMCTL();
    fn WWDT();
    fn RTC();
    fn ADC0();
    fn ADC1();
    fn ADC2();
    fn ADC3();
    fn TIMERA();
    fn TIMERB();
    fn TIMERC();
    fn TIMERD();
    fn TIMERAQEP();
    fn TIMERBQEP();
    fn TIMERCQEP();
    fn TIMERDQEP();
    fn GPIOA();
    fn GPIOB();
    fn GPIOC();
    fn GPIOD();
    fn GPIOE();
    fn GPIOF();
    fn GPIOG();
    fn I2C();
    fn USARTA_SSPA();
    fn USARTB_SSPB();
    fn USARTC_SSPC();
    fn USARTD_SSPD();
    fn CAN();
    fn GPTIMERA();
    fn GPTIMERB();
    fn SCC();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 31] = [
    Vector { _handler: MEMCTL },
    Vector { _handler: WWDT },
    Vector { _handler: RTC },
    Vector { _handler: ADC0 },
    Vector { _handler: ADC1 },
    Vector { _handler: ADC2 },
    Vector { _handler: ADC3 },
    Vector { _handler: TIMERA },
    Vector { _handler: TIMERB },
    Vector { _handler: TIMERC },
    Vector { _handler: TIMERD },
    Vector {
        _handler: TIMERAQEP,
    },
    Vector {
        _handler: TIMERBQEP,
    },
    Vector {
        _handler: TIMERCQEP,
    },
    Vector {
        _handler: TIMERDQEP,
    },
    Vector { _handler: GPIOA },
    Vector { _handler: GPIOB },
    Vector { _handler: GPIOC },
    Vector { _handler: GPIOD },
    Vector { _handler: GPIOE },
    Vector { _handler: GPIOF },
    Vector { _handler: GPIOG },
    Vector { _handler: I2C },
    Vector {
        _handler: USARTA_SSPA,
    },
    Vector {
        _handler: USARTB_SSPB,
    },
    Vector {
        _handler: USARTC_SSPC,
    },
    Vector {
        _handler: USARTD_SSPD,
    },
    Vector { _handler: CAN },
    Vector { _handler: GPTIMERA },
    Vector { _handler: GPTIMERB },
    Vector { _handler: SCC },
];
