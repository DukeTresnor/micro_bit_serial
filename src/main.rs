#![no_main]
#![no_std]



use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use core::fmt::Write;


// this cfg tag is to account for users with different microbit types
/* 
#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};
*/
#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
/*
    #[cfg(feature = "v1")]
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };
*/
    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };


    //let my_string = String::from("The quick brown fox jumps over the lazy dog.");
    //let my_string_slice = "The quick brown fox jumps over the lazy dog.";
    /* 
    let my_string_slice_as_bytes = "The quick brown fox jumps over the lazy dog.".as_bytes();


    for byte in my_string_slice_as_bytes {
        
        nb::block!(serial.write(*byte)).unwrap();
    }

    */

    //write!(serial, "The quick brown fox jumps over the lazy dog.\r\n").unwrap();   

    //nb::block!(serial.flush()).unwrap();

    loop {
        let byte = nb::block!(serial.read()).unwrap();
        rprintln!("{}", byte)
    }


}
