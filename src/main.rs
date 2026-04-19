#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

use panic_halt as _;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let mut gpioc = dp.GPIOC.split(&mut rcc);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut timer = Timer::syst(cp.SYST, &rcc.clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    loop {
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}
