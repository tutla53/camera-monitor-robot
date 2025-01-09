//! Servo PIO Task with state machine 0 and 1 

use {
    core::time::Duration,
    rp2040_servo_pio::ServoPioBuilder,
    crate::resources::gpio_list::{
        Irqs,
        HeadServoResources,
        BodyServoResources,
    },
    embassy_rp::{
        pio::Pio,
        pio_programs::{
            pwm::{PioPwm, PioPwmProgram},
        },
    },
    embassy_sync::{
        signal::Signal,
        blocking_mutex::raw::CriticalSectionRawMutex,
    },
    embassy_time::Timer,
    {defmt_rtt as _, panic_probe as _},
};

const REFRESH_INTERVAL: u64 = 20000;
const BODY_SERVO_INIT_POS: u64 = 90;
const HEAD_SERVO_INIT_POS: u64 = 90;

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
    let Pio { mut common, sm0, .. } = Pio::new(r.BODY_SERVO_PIO_CH, Irqs);
    let prg = PioPwmProgram::new(&mut common);

    let body_pwm_pio = PioPwm::new(&mut common, sm0, r.BODY_SERVO_PIN, &prg);
    
    let mut body_servo = ServoPioBuilder::new(body_pwm_pio)
        .set_period(Duration::from_micros(REFRESH_INTERVAL))
        .set_max_degree_rotation(180)
        .set_min_pulse_width(Duration::from_micros(1000))
        .set_max_pulse_width(Duration::from_micros(2000))
        .set_initial_position(BODY_SERVO_INIT_POS)
        .build();

    body_servo.start();
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

        body_servo.rotate(body_degree as u64);
    }
}

#[embassy_executor::task]
pub async fn head_servo_task(r: HeadServoResources) {
    let Pio { mut common, sm0, .. } = Pio::new(r.HEAD_SERVO_PIO_CH, Irqs);
    let prg = PioPwmProgram::new(&mut common);

    let head_pwm_pio = PioPwm::new(&mut common, sm0, r.HEAD_SERVO_PIN, &prg);
    
    let mut head_servo = ServoPioBuilder::new(head_pwm_pio)
        .set_period(Duration::from_micros(REFRESH_INTERVAL))
        .set_max_degree_rotation(180)
        .set_min_pulse_width(Duration::from_micros(1000))
        .set_max_pulse_width(Duration::from_micros(2000))
        .set_initial_position(HEAD_SERVO_INIT_POS)
        .build();

    head_servo.start();
    Timer::after_secs(1).await;

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

        head_servo.rotate(head_degree as u64);
    }
}