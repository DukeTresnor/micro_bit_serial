For Microbit v2  
Target: thumbv7em-none-eabihf  
Chip Type: nrf52833_xxAA  
Rust-lld error: https://ferrous-systems.com/blog/defmt-rtt-linker-error/  
To flash onto microbit: cargo embed --features v2 --target thumbv7em-none-eabihf  
Also run minicom in another terminal: minicom -D /dev/path/to/device/ -b (baud_rate)  
minicom -d /dev/tty.usbmodem143202 -b 115200  
