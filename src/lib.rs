#![no_std]

pub extern crate rp2040_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

//// The linker will place this boot block at the start of our program image. We
//// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

pub use hal::pac;

hal::bsp_pins!(
    Gpio0 { name: grove_1_a },
    Gpio1 { name: grove_1_b },
    Gpio2 { name: grove_2_a },
    Gpio3 { name: grove_2_b },
    Gpio4 { name: grove_3_a },
    Gpio5 { name: grove_3_b },
    Gpio6 { name: grove_4_a },
    Gpio7 { name: grove_4_b },
    Gpio8 { name: grove_5_a },
    Gpio9 { name: grove_5_b },
    Gpio10 { name: sd_clk },
    Gpio11 { name: sd_cmd },
    Gpio12 { name: sd_dat_0 },
    Gpio13 { name: sd_dat_1 },
    Gpio14 { name: sd_dat_2 },
    Gpio15 { name: sd_dat_3 },
    Gpio16 { name: wifi_tx }, // guessed
    Gpio17 { name: wifi_rx },
    Gpio18 { name: buzzer }, // also audio0
    Gpio19 { name: audio1 },
    Gpio20 { name: button1 },
    Gpio21 { name: button2 },
    Gpio22 { name: button3 },
    Gpio23 { name: bootsel },
    Gpio24 { name: vbus_detect },
    Gpio25 { name: led },
    Gpio26 { name: grove_6_a },
    Gpio27 { name: grove_6_b },
    Gpio28 { name: smartleds },
    Gpio29 { name: half_vbatt },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
