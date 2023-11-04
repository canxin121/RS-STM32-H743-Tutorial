#![no_main]
#![no_std]
// TIM1 CH1 PE9
// TIM1 CH1N PE8
// TIM1 CH2 PE11
// TIM1 CH2N PE10
// TIM1 CH3 PE13
// TIM1 CH3N PE12
// TIM1 CH4 PE14
extern crate alloc;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32h7xx_hal as hal;

use cortex_m_rt::entry;
use defmt::println;
use defmt_rtt as _;
use panic_probe as _;
use stm32h7xx_hal::delay::Delay;
use stm32h7xx_hal::{device, pac, prelude::*, pwm};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().expect("Cannot take peripherals");

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(8.MHz()).freeze(pwrcfg, &dp.SYSCFG);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let mut tim1_builder = dp.TIM1.pwm_advanced(
        (
            gpioe.pe9.into_alternate(),
            gpioe.pe11.into_alternate(),
            gpioe.pe13.into_alternate(),
            gpioe.pe14.into_alternate(),
        ),
        ccdr.peripheral.TIM1,
        &ccdr.clocks,
    );
    // two way to set frequency.

    // 1. set frequency mannuly
    // tim1_builder = tim1_builder.period(65535).prescaler(1000 - 1);
    // 2. set the PWM frequency automatically
    // Note that the automatically set frequency may have some error from the desired value

    tim1_builder = tim1_builder.frequency(1.kHz());

    let (mut _tim1_pwm_ctrl, (tim1_ch1_temp, tim1_ch2_temp, tim1_ch3_temp, _t1c4_pwm)) =
        tim1_builder.finalize();

    let mut delay = Delay::new(cp.SYST, ccdr.clocks);

    // Unfortunately, this time we need to manually add type annotations to the pwm structure,
    // because the rust compiler cannot automatically infer the type here.
    // Note that the ch in the comment starts from 0, different from the stm official starting from 1, so we need to subtract 1
    let mut t1c1_pwm: pwm::Pwm<device::TIM1, 0, pwm::ComplementaryEnabled> =
        tim1_ch1_temp.into_complementary(gpioe.pe8.into_alternate());

    let mut t1c2_pwm: pwm::Pwm<device::TIM1, 1, pwm::ComplementaryEnabled> =
        tim1_ch2_temp.into_complementary(gpioe.pe10.into_alternate());

    let mut t1c3_pwm: pwm::Pwm<device::TIM1, 2, pwm::ComplementaryEnabled> =
        tim1_ch3_temp.into_complementary(gpioe.pe12.into_alternate());
    let mut pwms: [&mut dyn _embedded_hal_PwmPin<Duty = u16>; 3] =
        [&mut t1c1_pwm, &mut t1c2_pwm, &mut t1c3_pwm];

    for pwm in pwms.iter_mut() {
        pwm.set_duty(pwm.get_max_duty());
        pwm.enable();
    }

    let mut crt = 100;
    let mut raise = false;

    loop {
        let mut duty = 0;
        for pwm in pwms.iter_mut() {
            duty = pwm.get_max_duty() / 100 * crt;
            pwm.set_duty(duty);
        }
        println!("set pwm: {}", duty);
        delay.delay_ms(10u16);

        if raise {
            crt += 1;
        } else {
            crt -= 1;
        }
        if crt <= 0 || crt >= 100 {
            raise = !raise;
        }
    }
}
