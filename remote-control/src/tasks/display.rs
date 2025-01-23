/*
    OLED Display Task
*/

use {
    embassy_rp::i2c::{
        I2c,
        Config,
    },
    crate::resources::gpio_list::{
        Irqs,
        DisplayResources,
    },
    ssd1306::{
        I2CDisplayInterface,
        Ssd1306,
        mode::DisplayConfig,
        prelude::DisplayRotation,
        size::DisplaySize128x64,
    },
    embassy_sync::{
        signal::Signal,
        blocking_mutex::raw::CriticalSectionRawMutex,
    },
    embassy_time::Timer,
    core::fmt::Write,
};

static DISPLAY_CONTROL: Signal<CriticalSectionRawMutex, Command> = Signal::new();

pub enum Command {
    Brightness(bool),
    Status(usize),
}

pub fn send_command(command: Command) {
    DISPLAY_CONTROL.signal(command);
}

async fn wait_command() -> Command {
    DISPLAY_CONTROL.wait().await
}

const ENCODE_CODE: [&str; 6] = [
    "Sleep Mode      Press the Button               ", 
    "Pairing...                                     ",
    "Pairing: Connected                             ",
    "Pairing: Failed                                ",
    "Connected                                      ",
    "Sleep Mode                                     ",
];

#[embassy_executor::task]
pub async fn display(r: DisplayResources) {
    let i2c0 = I2c::new_async(r.I2C_CH, r.SCL_PIN, r.SDA_PIN, Irqs, Config::default());
    let interface = I2CDisplayInterface::new(i2c0);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();
    
    match display.init(){
        Ok(()) => {log::warn!("Display has been Initialized")}
        Err(e) => {
            log::warn!("Write Error: {:?}", e);
        }
    };

    display.clear().unwrap();
    display.set_position(0, 1).unwrap();
    let _ = display.write_str("Baby Monitor");
    
    display.set_position(0, 3).unwrap();
    let _ = display.write_str("Status:         ");

    loop{

        let command = wait_command().await;
        
        match command {
            Command::Brightness(value) => {
                display.set_display_on(value).unwrap();
                Timer::after_millis(200).await;
            },
            Command::Status(value) => {
                display.set_position(0, 4).unwrap();
                let _ = display.write_str(ENCODE_CODE[value]);
            },
        }
    }

}