#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use stm32f0xx_hal as hal;
use ws2812_spi as ws2812;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::hal::spi::Spi;
use crate::hal::stm32;
use crate::ws2812::Ws2812;
use cortex_m::peripheral::Peripherals;

use smart_leds::{Color, SmartLedsWrite};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpioa = p.GPIOA.split();

        // Constrain clocking registers
        let rcc = p.RCC.constrain();

        // Configure clock to 8 MHz (i.e. the default) and freeze it
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        // Configure pins for SPI
        let sck = gpioa.pa5.into_alternate_af0();
        let miso = gpioa.pa6.into_alternate_af0();
        let mosi = gpioa.pa7.into_alternate_af0();

        // Configure SPI with 3Mhz rate
        let spi = Spi::spi1(
            p.SPI1,
            (sck, miso, mosi),
            ws2812::MODE,
            3_000_000.hz(),
            clocks,
        );

        let mut data: [Color; 3] = [Color::default(); 3];
        let empty: [Color; 3] = [Color::default(); 3];
        let mut ws = Ws2812::new(spi);
        loop {
            data[0] = Color {
                r: 0,
                g: 0,
                b: 0x10,
            };
            data[1] = Color {
                r: 0,
                g: 0x10,
                b: 0,
            };
            data[2] = Color {
                r: 0x10,
                g: 0,
                b: 0,
            };
            ws.write(data.iter().cloned()).unwrap();
            delay.delay_ms(1000 as u16);
            ws.write(empty.iter().cloned()).unwrap();
            delay.delay_ms(1000 as u16);
        }
    }
    loop {
        continue;
    }
}
