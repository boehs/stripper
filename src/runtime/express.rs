use super::Runtime;
use bsp::hal::prelude::*;
use bsp::{
    hal::{clock::GenericClockController, delay::Delay, gpio, timer::TimerCounter},
    pac::{CorePeripherals, Peripherals},
};
use circuit_playground_express as bsp;
use smart_leds::{SmartLedsWrite, RGB, brightness};
use ws2812_timer_delay as ws2812;

pub struct Express {
    pub timer: Delay,
    np: ws2812::Ws2812<
        TimerCounter<atsamd21g::TC3>,
        gpio::v2::Pin<gpio::v2::PB23, gpio::v2::Output<gpio::v2::PushPull>>,
    >,
}

impl Runtime<()> for Express {
    fn new(input: ()) -> Self
    where
        Self: Sized,
    {
        let mut peripherals = Peripherals::take().unwrap();
        let core = CorePeripherals::take().unwrap();
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );

        let pins = bsp::Pins::new(peripherals.PORT);
        let mut delay = Delay::new(core.SYST, &mut clocks);

        let gclk0 = clocks.gclk0();
        let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
        let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
        timer.start(3.mhz());

        let neopixel_pin: bsp::NeoPixel = pins.d8.into();
        let neopixel = ws2812::Ws2812::new(timer, neopixel_pin);

        Self {
            timer: delay,
            np: neopixel,
        }
    }

    fn display(
        &mut self,
        pixels: &crate::Pixels,
    ) -> Result<(), alloc::boxed::Box<dyn core::error::Error>> {
        self.np.write(brightness(pixels.iter().map(|x| {
            RGB::new(
                (x.red * 255.0) as u8,
                (x.green * 255.0) as u8,
                (x.blue * 255.0) as u8,
            )
        }),100));
        Ok(())
    }

    fn get_number_of_pixels(&self) -> u16 {
        10
    }
}
