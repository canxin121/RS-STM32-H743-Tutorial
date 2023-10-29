// Use this method of delay onlyif you dont't have access to SYST.
#![no_std]
#![no_main]
use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use stm32h7xx_hal::{delay::DelayFromCountDownTimer, prelude::*};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();
    let mut ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    let mut timer2 = dp
        .TIM2
        .timer(1.kHz(), ccdr.peripheral.TIM2, &mut ccdr.clocks);

    let mut delay = DelayFromCountDownTimer::new(timer2);

    loop {
        delay.delay_ms(1000u16);
        println!("After 1000ms")
    }
    // In actuall use, you need to free the used delay to relase the timer.
    // delay.free();
}
