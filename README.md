# Environment setup instructions
Install Rust + Cargo using rustup
```bash 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Or download the [Windows installer](https://win.rustup.rs/x86_64).

Install the thumbv6m-none-eabi target:
```bash
rustup target install thumbv6m-none-eabi
```

Install elf2uf2-rs:
```bash
cargo install --locked elf2uf2-rs
```

The rest of the configuration is set in the `.cargo` folder.

# Putting the Pico in bootloader mode
To put the Pico in bootloader mode, hold down the BOOTSEL button while plugging in the USB cable.
<img width="389" alt="image" src="https://github.com/user-attachments/assets/4f5b179f-2378-4bcf-9402-324859ec2c88">

An RP2 Boot device should appear, as well as a disk drive mount named `RPI-RP2`.
```bash
lsusb # check for RP2 Boot device on Linux
ioreg -p IOUSB # check for RP2 Boot device on macOS
Get-PnpDevice -Class USB # check for RP2 Boot device on Windows
```

# Flashing with cargo
elf2uf2-rs is used to convert the ELF file to a UF2 file and can also automatically flash it to the Pico.
Since it is defined as a cargo runner, you can just run:
```bash
cargo run
```
This will build the project, convert the ELF file to a UF2 file and flash it to the Pico.

# Manual flashing with mounted drive
The Pico can be flashed by mounting the Pico as a drive and copying the UF2 file to it.
To do this we manually we first have to convert the ELF file to a UF2 file:
```bash
elf2uf2-rs target/thumbv6m-none-eabi/debug/menorah-rs out.uf2
```
We can then copy it manually to the Pico drive.
```bash
cp out.uf2 /media/$USER/RPI-RP2 # Linux
cp out.uf2 /Volumes/RPI-RP2 # macOS
cp out.uf2 D: # Windows
``` 

# Manual flashing with picotool
Picotool is the Raspberry Pi Pico's official tool for interfacing with the RP2040 and can be used to flash the UF2 file to the Pico.
In my setup it is often a better way to flash uf2 files, but requires building picotool from source.
```bash
git clone https://github.com/raspberrypi/picotool
cd picotool
# clone the pico sdk
git clone https://github.com/raspberrypi/pico-sdk

# install libusb-1.0 if not yet installed
# build-essential pkg-config cmake are also necessary but are likely already installed
# on Ubuntu
# `sudo apt install libusb-1.0-0-dev`
# on macOS
# `brew install libusb`
# on Windows
# `choco install libusb`
# or `scoop install libusb` or `winget install libusb` or `vcpkg install libusb`, whatever is your package manager

# build
mkdir build
cd build
cmake .. -DPICO_SDK_PATH=../pico-sdk
make
```

Once built, you can flash the UF2 file to the Pico:
```bash
elf2uf2-rs target/thumbv6m-none-eabi/debug/menorah-rs out.uf2
./picotool load out.uf2
./picotool reboot # or reboot manually
```

# MicroPython
If you'd like to prototype with a convenient MicroPython REPL - you can download the UF2 file from the [MicroPython website](https://micropython.org/download/RPI_PICO/) and flash it to the Pico using the same methods as above.

You can then use a standard serial terminal to connect to the Pico and interact with the REPL.
```bash
python -m pip install pyserial
python -m serial.tools.miniterm 
# select the device called board in FS mode
# press enter to get the REPL
```

A few tips for using this mode - 
`Ctrl+D` causes a soft reset, `Ctrl+E` enters paste mode which allows convenient pasting of code into the terminal.

You can also use Thonny IDE if you prefer.

