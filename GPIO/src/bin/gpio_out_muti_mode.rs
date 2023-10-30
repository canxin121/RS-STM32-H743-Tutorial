//! Here we use push_pull mode of gpio_out for example.
#![no_std]
#![no_main]

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
    let mut led_green = gpiob.pb0.into_dynamic();
    let mut led_red = gpiob.pb14.into_dynamic();

    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        // light up

        led_green.make_push_pull_output_in_state(gpio::PinState::High);
        delay.delay_ms(500u32);
        led_green.set_low().unwrap();
        delay.delay_ms(500u32);
        led_red.make_push_pull_output_in_state(gpio::PinState::High);
        delay.delay_ms(500u32);
        led_red.set_low().unwrap();
        delay.delay_ms(500u32);
    }
}
