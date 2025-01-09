# Simple 2 D.O.F Camera Monitor
Very simple 2 D.O.F camera monitor robot with two servo motors. 
This is my first project to use Rust + embassy-rs for microcontroller.

## Project Structure
```bash
.
├── Cargo.lock
├── Cargo.toml
├── build.rs
├── memory.x
└── src
    ├── main.rs
    ├── resources
    │   ├── gpio_list.rs
    │   └── mod.rs
    └── tasks
        ├── button.rs
        ├── mod.rs
        ├── servo_pio.rs
        └── uart_task.rs

4 directories, 11 files
```

## Hardware Components
|Component               | Description |
|------------------------|-------------|
|Raspberry Pi Pico RP2040| I use this because embassy-rs have many examples for Pico and also Pico W |
|JIYUE Baby Monitor|Existing baby monitor camera. This is for quick development, maybe I will develop embedded camera later|
|MG996R + Bracket|Servo Motor for yaw and pitch camera movement. Controlled by varying the PWM duty cycles via PIO. I have developed my own library for this project on [rp2040-servo-pio](https://github.com/tutla53/embassy-rp-library) |
|HC-05 Bluetooth         |Bluetooth module to drive the servo motor|
|IC2262/2272 RC Module   |RC transmitter and receiver module with 433 MHz variant. The output of this module is 5V, so we need the logic shifter to connect to Pico|
|CD4050BE|Logic shifter to drive the PWM from 3.3V to 5V and convert RC Module output from 5V to 3.3V|


