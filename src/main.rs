#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use nb::block;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

use defmt_rtt as _;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let core = cortex_m::Peripherals::take().unwrap();
    let device = pac::Peripherals::take().unwrap();
    info!("Hello, World!");

    let mut rcc = device.RCC.constrain();

    let mut gpioc = device.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut timer = Timer::syst(core.SYST, &rcc.clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    loop {
        block!(timer.wait()).unwrap();
        led.set_high();
        info!("High");
        block!(timer.wait()).unwrap();
        led.set_low();
        info!("Low");
    }
}
