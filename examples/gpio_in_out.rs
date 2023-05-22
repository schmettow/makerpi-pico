//! # Pico GPIO In/Out Example
//!
//! Toggles the LED based on GPIO input.
//!
//! This will control an LED on GP25 based on a button hooked up to GP15. The
//! button should cause the line to be grounded, as the input pin is pulled high
//! internally by this example. When the button is pressed, the LED will turn
//! off.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

use cytron_maker_pi_rp2040 as rp_pico;

use hal::gpio::Input;
// The macro for our start-up function
use rp_pico::entry;

// GPIO traits
use embedded_hal::digital::v2::{InputPin, OutputPin};

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal as hal;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then just reads the button
/// and sets the LED appropriately.
/// 

use rp2040_hal::gpio;
/// This pin will be our output - it will drive an LED if you run this on a Pico
type LedPin = gpio::Pin<gpio::bank0::Gpio25, gpio::PushPullOutput>;

/// It will trigger an interrupt if pulled to ground (via a switch or jumper wire)
type Button1Pin = gpio::Pin<gpio::bank0::Gpio20, gpio::PullUpInput>;
//type Button2Pin = gpio::Pin<gpio::bank0::Gpio21, gpio::PullUpInput>;
//type Button3Pin = gpio::Pin<gpio::bank0::Gpio22, gpio::PullUpInput>;

//type YBtnPin <P> = gpio::Pin<P, <P as gpio::PinId>::Reset>;

struct YBtn {
    state : bool,
    last_state : bool,
    pin : Button1Pin
}

impl YBtn{
    pub fn new(button_pin: Button1Pin) -> Self { 
        YBtn {
            state : false,
            last_state : false,
            pin: button_pin,
        } 
    }

    pub fn read(&mut self) -> bool{
        let this_value = self.pin.is_low().unwrap();
        self.last_state = self.state;
        self.state = this_value;
        return self.state;
    }

    pub fn event(&self) -> bool{
        self.state != self.last_state
    }

    pub fn pressed(&self) -> bool{
        self.event() & self.state
    }

    pub fn released(&self) -> bool{
        self.event() & self.state
    }
    
}

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Note - we don't do any clock set-up in this example. The RP2040 will run
    // at it's default clock speed.

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );


    // Our LED output
    let mut led_pin = pins.led.into_push_pull_output();

    // Our button input
    let button_pin = pins.button1.into_pull_up_input();

    let mut btn_1 = YBtn::new(button_pin);

    // Run forever, setting the LED according to the button
    
    let mut state = "off";

    loop {
        btn_1.read();
        if btn_1.pressed() {
            if state == "on" {
                state = "off";
                led_pin.set_low().unwrap();    
            } else {
                state = "on";
                led_pin.set_high().unwrap();    
            }
        }
/*         if btn_1.read() {
            led_pin.set_high().unwrap();
        } else {
            led_pin.set_low().unwrap();
        }
 */
        
    }
}

// End of file