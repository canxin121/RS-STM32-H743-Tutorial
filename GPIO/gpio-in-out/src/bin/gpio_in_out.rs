#![no_std]
#![no_main]
use defmt_rtt as _;
use panic_probe as _;

use stm32h7xx_hal::{gpio::PinState, prelude::*};

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
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);

    let mut led_green = gpiob.pb0.into_push_pull_output();
    let mut led_red = gpiob.pb14.into_push_pull_output();

    let button = gpioc.pc13.into_floating_input();
    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        // 2 different methods. the 2 in same loop should share the first state of button.
        // in this case, push down the button and the led light up,
        // relase the button and the led light out
        let state = button.is_high();
        if state {
            led_red.set_high();
        } else {
            led_red.set_low();
        }
        // in this case, push the button, the led toggle.
        if state {
            delay.delay_ms(10u16);
            if button.is_high() {
                led_green.toggle();
                while button.is_high() {}
            }
        }
    }
}
