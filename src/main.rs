#![no_main]
#![no_std]



use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use core::fmt::Write;
use heapless::Vec;

use lsm303agr::{
    AccelOutputDataRate, Lsm303agr,
};

use microbit::hal::prelude::*;

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
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;




// Constants representing addresses to the slave devices -- accelerometer and 
//   manetometer. 
// These are 7 bits long -- "0b" implies that we are performing a write operation,
//   and are writing to the acceleraometer or the magnetometer. I think then, that
//   "1b" would direct communication to be a read operation, ie trying to read data
//   from the slave device.
// I am not sure here
const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

// hexidecimal ID registers -- 0x is the base to show it's hexidecimal
const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;


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

    // i2c block //

    // Defining i2c protocal instance
    //   Twim instance -- Two wite interface
    //   the M is because it's with the v2 microbit
    #[cfg(feature = "v2")]
    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut acc = [0];
    let mut mag = [0];

    // First write the address + register onto the bus, then read the chip's responses
    i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc).unwrap();
    i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag).unwrap();

    rprintln!("The accelerometer chip's id is: {:#b}", acc[0]);
    rprintln!("The magnetometer chip's id is: {:#b}", mag[0]);


    // setting up variables to hold accelerometer data
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();



    // i2c block //


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

        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            // print with RTT, not normal print
            // formatting -- :#.2f -- does what?
            rprintln!("Acceleration x: {} y: {} z: {}", data.x, data.y, data.z);
        }

        loop {
            // Assume that the receiving can't fail
            let byte = nb::block!(serial.read()).unwrap();
            rprintln!("{}", byte);

            // Push the current byte into the buffer
            // if pushing the current byte into buffer results in an error
            //   (ie if there's more than 32 bits)
            //   break the inner loop, which flushes your serial and clears your buffer
            if buffer.push(byte).is_err() {
                write!(serial, "Error: buffer full\r\n").unwrap();
                break;
            }

            // If you have the microbit read a carriage return character, for me --> enter (option + enter)?
            //   loop through buffer, then exit loop, then flush your serial and clear buffer
            // .rev() reverses the order of the iterator
            // .chain() combines two iterators to iterate over both in the for loop I think?
            if byte == 13 {
                for byte in buffer.iter().chain(&[b'\n', b'\r']) {
                    nb::block!(serial.write(*byte)).unwrap();
                }
                break;
            }
        }
        nb::block!(serial.flush()).unwrap();
        

    }


}
