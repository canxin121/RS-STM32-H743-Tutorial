//! Here we use push_pull mode of gpio_out for example.
#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use defmt::{error, info, println, warn};
use defmt_rtt as _;
use panic_probe as _;

use stm32h7xx_hal::{gpio, prelude::*};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let ccdr = rcc.sysclk(120.MHz()).freeze(pwrcfg, &dp.SYSCFG);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let mut led_green = gpiob.pb0.into_push_pull_output();
    let mut led_red = gpiob.pb14.into_push_pull_output();

    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        // light up
        println!("Yellow");
        led_green.set_high();
        delay.delay_ms(500u32);
        println!("Red");
        led_red.set_high();
        delay.delay_ms(500u16);
        // led out after 500ms
        led_green.set_low();
        led_red.set_low();
        delay.delay_ms(500u16);
    }
}
