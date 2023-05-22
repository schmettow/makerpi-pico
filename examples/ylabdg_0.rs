//! # YLab Edge
//!
//! Records Sensor Data
//!
//! Uses the `ws2812_pio` driver to control the LEDs, which in turns uses the
//! RP2040's PIO block.
//! 
//! 

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

// Make an alias for our board support package so copying examples to other boards is easier
use makerpi_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio::{FunctionPio0, Pin},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

use rp2040_hal::pio::PIOExt;
use ws2812_pio::{Ws2812Direct};
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;

mod yui{    
    use makerpi_pico as bsp;
    use bsp::hal::{
        clocks::{init_clocks_and_plls, Clock},
        gpio::{FunctionPio0, Pin},
        pac,
        sio::Sio,
        watchdog::Watchdog,
    };
    use rp2040_hal::{pio::PIOExt};
    use smart_leds::{brightness, SmartLedsWrite, RGB8};
    use ws2812_pio::{Ws2812Direct};
    use embedded_hal::digital::v2::InputPin;
    use embedded_hal::digital::v2::OutputPin;

    
    type RgbStatusLed = Ws2812Direct<pac::PIO0, 
                        bsp::hal::pio::SM0, 
                        bsp::hal::gpio::pin::bank0::Gpio28>;

    pub trait StaticOutput {
        fn write<T>(&mut self, value: T);
    }
                    


    pub struct RgbLed {
        led: RgbStatusLed,
    }

    impl RgbLed {
        pub fn new(led: RgbStatusLed) -> Self {
            RgbLed{led: led}
            }
        
        pub fn write(&mut self, value:RGB8){
            let col = [value,];
            self.led.write(brightness(col.iter().cloned(), 32)).unwrap();
        }

    }

    // Stateful button
    pub struct Button<T: InputPin> {
        pub pin: T,
        pub last_state: bool,
        pub state: bool,
    }

    impl<T: InputPin> Button<T> {
        pub fn new(pin: T) -> Self {
            Button { pin: pin, last_state: false, state: false}
        }

        pub fn update(&mut self) -> bool {
            let this_value = self.read();
            self.last_state = self.state;
            self.state = this_value;
            return self.state
        }

    }

    // Contact sensor trait
    pub trait ContactSensor {
        fn read(&self) -> bool;
    }

    impl<T: InputPin> ContactSensor for Button<T> {
        fn read(&self) -> bool {
            self.pin.is_low().unwrap_or(true)
        }
    }

/*     // Event trait
    pub trait EventButton {
        fn update(&mut self) -> bool;
        fn pressed(&self) -> bool;
    }

    impl<T: InputPin> EventButton for Button<T> {
        fn pressed(&self) -> bool {
            self.state
        }
    } */
}

#[entry]
fn main() -> ! {
    // Configure the RP2040 peripherals

    let mut pac: pac::Peripherals = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks: rp2040_hal::clocks::ClocksManager = init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let sio = Sio::new(pac.SIO);
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // INIT RGB
    let smartleds_pin: Pin<_, FunctionPio0> 
        = pins.smartleds.into_mode();
    // Configure the addressable LED
    let (mut pio, sm0, _, _, _) 
        = pac.PIO0.split(&mut pac.RESETS);
    let mut rgb
        = yui::RgbLed::new(Ws2812Direct::new(smartleds_pin,
                            &mut pio,
                            sm0,
                            clocks.peripheral_clock.freq()));

    let red: smart_leds::RGB<u8> = (255, 0, 0).into();
    //let yellow: smart_leds::RGB<u8> = (255, 255, 0).into();
    let green: smart_leds::RGB<u8> = (0, 255, 0).into();
    let _blue: smart_leds::RGB<u8> = (0, 0, 255).into();
    let white: smart_leds::RGB<u8> = (255, 255, 255).into();
    
    // Init Button
    let mut btn_1 = yui::Button::new(pins.button1.into_pull_up_input());
    
    // Init Led
    let mut led = pins.led.into_push_pull_output();

    // Init ADC
    use embedded_hal::adc::OneShot;
    use rp2040_hal::{adc::Adc};
    
    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    

    // Prepare interaction flow
    let mut state = "Stop";
    let mut trial: i8 = 0;
    let n_trials: i8 = 3;
    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);
    let mut adc_pin_0 = pins.grove_6_b.into_floating_input();
    let mut this_value: u16;

    loop{
        // Interaction
        btn_1.update();
        if btn_1.state {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap()
        }
        //sleep(&mut delay, 10); // waiting for user input
        if btn_1.state {
            if state == "Stop" {        
                trial = 0;
                rgb.write(white);
                //sleep(&mut delay, 2000); // waiting for user input
                state = "Pause";
                continue
            }
            if state == "Pause" {
                trial += 1;
                rgb.write(green);
                //sleep(&mut delay, 500);
                state = "Record";
                continue
            }
            if state == "Record" {
                rgb.write(red);        
                //sleep(&mut delay, 2000);
                if trial < n_trials {
                    state = "Pause";
                } else {
                    state = "Stop";
                }
                continue
        }

        // Continuous processing

        if state == "Pause" || state == "Record" {
            this_value = adc.read(&mut adc_pin_0).unwrap();
            delay.delay_ms(20);
        }

    }
  }
}
