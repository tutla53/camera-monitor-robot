//! Servo PIO Task with state machine 0 and 1 

use {
    crate::resources::gpio_list::{
        Irqs,
        UartResources,
    },
    crate::tasks::{
        servo_pio::{
            send_body_command,
            send_head_command,
            BodyCommand,
            HeadCommand,
        },
    },
    embassy_rp::{
        uart::{
            Config,
            UartRx,
        },
    },
    {defmt_rtt as _, panic_probe as _},
};

#[embassy_executor::task]
pub async fn uart_task(r: UartResources) {
    let mut cfg = Config::default();
    cfg.baudrate = 9600;
    let mut uart_rx = UartRx::new(r.UART_CH, r.UART_RX_PIN, Irqs, r.UART_DMA_CH, cfg);
    
    loop {
        let mut buf = [0u8; 1];
        match uart_rx.read(&mut buf).await{
            Ok(_) =>{
                match buf[0] {
                    b'w' => {send_head_command(HeadCommand::Up);},
                    b's' => {send_head_command(HeadCommand::Down);},
                    b'a' => {send_body_command(BodyCommand::Left);},
                    b'd' => {send_body_command(BodyCommand::Right);},
                    _ => {},
                }
            },
            Err(e) => {log::info!("Error {:?}", e);}
        }           
    }
}