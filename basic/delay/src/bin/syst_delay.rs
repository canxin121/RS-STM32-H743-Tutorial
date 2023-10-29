// This method of delay is perferred and recommended.

#![no_std]
#![no_main]
use cortex_m::delay::Delay;
use defmt::{error, info, println, warn};
use defmt_rtt as _;
use panic_probe as _;

use stm32h7xx_hal::{prelude::*, timer::Timer};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let ccdr = rcc.sysclk(120.MHz()).freeze(pwrcfg, &dp.SYSCFG);
    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        delay.delay_ms(1000u32);
        println!("After 1000ms")
    }
    // In actuall use, you need to free the used delay to relase the timer.
    // delay.free();
;
}
