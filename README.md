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
```

## Hardware Components
|Component               | Description |
|------------------------|----         |
|Raspberry Pi Pico RP2040|             |
|JIYUE Baby Monitor||
|MG996R Servo Motor||
|Motor Bracket||
|Motor Base||
|HC-05 Bluetooth         ||
|IC2262/2272 RC Module   ||
|CD4050BE||
|Power Supply||


