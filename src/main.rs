#![no_main]
#![no_std]



use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use core::fmt::Write;
use heapless::Vec;


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

    // Make a buffer with 32 bytes of capacity; a vector of u8's that can hold 32 u8's
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        // clearing the buffer
        buffer.clear();

        loop {
            // Assume that the receiving can't fail
            let byte = nb::block!(serial.read()).unwrap();
            rprintln!("{}", byte);

            // if pushing the current byte into buffer results in an error
            //   (ie if there's more than 32 bits)
            if buffer.push(byte).is_err() {
                write!(serial, "Error: buffer full\r\n").unwrap();
                break;
            }

            // If you have the microbit read a carriage return character,
            //   loop through buffer, then exit loop, then flush your serial and clear buffer
            if byte == 13 {
                for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    nb::block!(serial.write(*byte)).unwrap();
                }
                break;
            }
        }
        nb::block!(serial.flush()).unwrap();
        

    }


}
