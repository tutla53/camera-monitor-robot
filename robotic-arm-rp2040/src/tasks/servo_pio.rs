//! Servo PIO Task with state machine 0 and 1 

use {
    rp2040_servo::ServoBuilder,
    crate::resources::gpio_list::{
        HeadServoResources,
        BodyServoResources,
    },
    embassy_rp::{
        pwm::{Config as PwmConfig, Pwm},
    },
    embassy_sync::{
        signal::Signal,
        blocking_mutex::raw::CriticalSectionRawMutex,
    },
    embassy_time::Timer,
    {defmt_rtt as _, panic_probe as _},
};

const BODY_SERVO_INIT_POS: u16 = 90;
const HEAD_SERVO_INIT_POS: u16 = 90;

static BODY_CONTROL: Signal<CriticalSectionRawMutex, BodyCommand> = Signal::new();
static HEAD_CONTROL: Signal<CriticalSectionRawMutex, HeadCommand> = Signal::new();

pub enum BodyCommand {Left, Right}
pub enum HeadCommand {Up, Down}

pub fn send_body_command(command: BodyCommand) {
    BODY_CONTROL.signal(command);
}

async fn wait_body_command() -> BodyCommand {
    BODY_CONTROL.wait().await
}

pub fn send_head_command(command: HeadCommand) {
    HEAD_CONTROL.signal(command);
}

async fn wait_head_command() -> HeadCommand {
    HEAD_CONTROL.wait().await
}

#[embassy_executor::task]
pub async fn body_servo_task(r: BodyServoResources) {
    let body_pwm_device = Pwm::new_output_a(
        r.BODY_SERVO_SLICE, 
        r.BODY_SERVO_PIN, 
        PwmConfig::default()
    );
    
    let mut body_servo = ServoBuilder::new(body_pwm_device)
        .set_servo_freq(50)
        .set_max_degree_rotation(180)
        .set_min_duty(1800)
        .set_max_duty(6600)
        .set_initial_position(0)
        .build();

    body_servo.enable();
    body_servo.rotate(BODY_SERVO_INIT_POS);
    Timer::after_secs(1).await;

    let mut body_degree: i16 = body_servo.get_current_pos() as i16;
    let body_inc: i16 = 1;

    loop {
        let command = wait_body_command().await;

        match command {
            BodyCommand::Left => {
                body_degree = body_degree + body_inc; 
            },
            BodyCommand::Right => {
                body_degree = body_degree - body_inc; 
            },
        }

        if body_degree<0 {body_degree = 0;}
        else if body_degree>180{body_degree = 180;}

        log::info!("Body Pos: {}", body_servo.get_current_pos());

        body_servo.rotate(body_degree as u16);
    }
}

#[embassy_executor::task]
pub async fn head_servo_task(r: HeadServoResources) {
    let head_pwm_device = Pwm::new_output_a(
        r.HEAD_SERVO_SLICE, 
        r.HEAD_SERVO_PIN, 
        PwmConfig::default()
    );
    
    let mut head_servo = ServoBuilder::new(head_pwm_device)
        .set_servo_freq(50)
        .set_max_degree_rotation(180)
        .set_min_duty(1800)
        .set_max_duty(6600)
        .set_initial_position(0)
        .build();

    head_servo.enable();
    Timer::after_secs(1).await;
    head_servo.rotate(HEAD_SERVO_INIT_POS);

    let mut head_degree: i16 = head_servo.get_current_pos() as i16;
    let head_inc: i16 = 1;

    loop {
        let command = wait_head_command().await;

        match command {
            HeadCommand::Up => {
                head_degree = head_degree + head_inc; 
            },
            HeadCommand::Down => {
                head_degree = head_degree - head_inc; 
            },
        }

        if head_degree<0 {head_degree = 0;}
        else if head_degree>180{head_degree = 180;}

        log::info!("Head Pos: {}", head_servo.get_current_pos());

        head_servo.rotate(head_degree as u16);
    }
}