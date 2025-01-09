// Button Task

use {
    crate::resources::gpio_list::{
        BodyButtonResources,
        HeadButtonResources,
    },
    crate::tasks::{
        servo_pio::{
            send_body_command,
            send_head_command,
            BodyCommand,
            HeadCommand,
        },
    },
    embassy_rp::gpio::{Input, Pull},
    embassy_time::Timer,
    {defmt_rtt as _, panic_probe as _},
};

#[embassy_executor::task]
pub async fn body_button_task(r: BodyButtonResources) {
    
    let left_input = Input::new(r.LEFT_PIN, Pull::Down);
    let right_input = Input::new(r.RIGHT_PIN, Pull::Down);
    
    loop {
        while left_input.is_high() {
            send_body_command(BodyCommand::Left);
            Timer::after_millis(50).await;
        }

        while right_input.is_high() {
            send_body_command(BodyCommand::Right);
            Timer::after_millis(50).await;
        }
        Timer::after_millis(10).await;
    }
}

#[embassy_executor::task]
pub async fn head_button_task(r: HeadButtonResources) {
    
    let up_input = Input::new(r.UP_PIN, Pull::Down);
    let down_input = Input::new(r.DOWN_PIN, Pull::Down);
    
    loop {
        while up_input.is_high() {
            send_head_command(HeadCommand::Up);
            Timer::after_millis(50).await;
        }

        while down_input.is_high() {
            send_head_command(HeadCommand::Down);
            Timer::after_millis(50).await;
        }
        Timer::after_millis(10).await;
    }
}