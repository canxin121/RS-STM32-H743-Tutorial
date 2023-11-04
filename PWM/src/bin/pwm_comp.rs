//! Here we use push_pull mode of gpio_out for example.
#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::{
    interrupt::{free, Mutex},
    peripheral::NVIC,
};
use defmt_rtt as _;

use cortex_m_rt::entry;
use panic_probe as _;
use stm32h7xx_hal::{
    delay::Delay,
    device::{self},
    gpio,
    prelude::*,
    pwm,
};
#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32h7xx_hal::pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let ccdr = rcc.sysclk(120.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // pe9 Tim1 ch1
    // pb8 Tim1 ch1n
    // in order to use pb8, we need to enable pe11 pwm, and set the pwm into complementary mode with pe9,
    // and then we can use the new pwm struct.
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let pe9: gpio::Pin<'E', 9, gpio::Alternate<1>> = gpioe.pe9.into_alternate();
    let pe8: gpio::Pin<'E', 8, gpio::Alternate<1>> = gpioe.pe8.into_alternate();
    let t1c1: pwm::Pwm<device::TIM1, 0, pwm::ComplementaryDisabled> =
        dp.TIM1
            .pwm(pe9, 10.kHz(), ccdr.peripheral.TIM1, &ccdr.clocks);
    let mut t1c1_comp: pwm::Pwm<device::TIM1, 0, pwm::ComplementaryEnabled> =
        t1c1.into_complementary(pe8);
    let mut delay = Delay::new(cp.SYST, ccdr.clocks);
    t1c1_comp.enable();
    t1c1_comp.set_duty(t1c1_comp.get_max_duty());
    // pwm of pe9:t1c1 : max
    // pwm of pb8:t1c1n : 0
    delay.delay_ms(1000u16);
    t1c1_comp.set_duty(0);
    // pwm of pe9:t1c1 : 0
    // pwm of pb8:t1c1n : max
    loop {}
}
