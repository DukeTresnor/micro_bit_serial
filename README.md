For Microbit v2  
Target: thumbv7em-none-eabihf  
Chip Type: nrf52833_xxAA  
Rust-lld error: https://ferrous-systems.com/blog/defmt-rtt-linker-error/  
To flash onto microbit: cargo embed --features v2 --target thumbv7em-none-eabihf  
When not using features, omit the featiure command: cargo embed --target thumbv7em-none-eabihf  
Also run minicom in another terminal: minicom -D /dev/path/to/device/ -b (baud_rate)  
minicom -d /dev/tty.usbmodem143202 -b 115200  
If you want to save data to a text file, follow minicom command with | tee /path/to/file/filename.text  
My specific command is: minicom -D /dev/tty.usbmodem143202 -b 115200 | tee ~/Projects/rust_space/embedded_projects/micro_bit_serial/src/data/minicom_output_4.txt  
Right now I receive the data in a format that is difficult to loop through. I have been using gsed to cut and replace certain phrases within the data folder: gsed -i ’s/in_stuff/out_stuff/g’ file_name.txt. Example: gsed -i 's/33;1H/ /g' minicom_output_7.txt
