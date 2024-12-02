//! Servo PIO Task with state machine 0 and 1 

use {
    core::time::Duration,
    crate::builder::servo_pio_builder::ServoPioBuilder,
    crate::resources::gpio_list::{
        Irqs,
        ServoPioResources,
    },
    embassy_rp::{
        pio::Pio,
        pio_programs::{
            pwm::{
                PioPwm, 
                PioPwmProgram,
            },
            uart::{
                PioUartRx,
                PioUartRxProgram,
            },
        },
    },
    embassy_time::Timer,
    {defmt_rtt as _, panic_probe as _},
};

const REFRESH_INTERVAL: u64 = 20000;

#[embassy_executor::task]
pub async fn servo_pio(r: ServoPioResources) {
    let Pio { mut common, sm0, sm1, sm2, .. } = Pio::new(r.SERVO_PIO_CH, Irqs);
    let prg = PioPwmProgram::new(&mut common);
    let rx_program = PioUartRxProgram::new(&mut common);

    let mut uart_rx = PioUartRx::new(9600, &mut common, sm2, r.UART_RX_PIN, &rx_program);

    let body_pwm_pio = PioPwm::new(&mut common, sm0, r.SERVO_BODY_PIN, &prg);
    let head_pwm_pio = PioPwm::new(&mut common, sm1, r.SERVO_HEAD_PIN, &prg);

    let mut body_servo = ServoPioBuilder::new(body_pwm_pio)
        .set_period(Duration::from_micros(REFRESH_INTERVAL))
        .set_max_degree_rotation(180)
        .set_min_pulse_width(Duration::from_micros(1000))
        .set_max_pulse_width(Duration::from_micros(2000))
        .build();

    let mut head_servo = ServoPioBuilder::new(head_pwm_pio)
        .set_period(Duration::from_micros(REFRESH_INTERVAL))
        .set_max_degree_rotation(180)
        .set_min_pulse_width(Duration::from_micros(1000))
        .set_max_pulse_width(Duration::from_micros(2000))
        .build();

    body_servo.stop();
    head_servo.stop();
    Timer::after_secs(1).await;
    body_servo.start();
    head_servo.start();
    body_servo.rotate(0);
    head_servo.rotate(0);

    let mut head_degree: i16 = 0;
    let mut body_degree: i16 = 0;
    let inc: i16 = 1;

    loop {
        let n = uart_rx.read_u8().await;

        match n {
            b'w' => {head_degree = head_degree + inc;},
            b's' => {head_degree = head_degree - inc;},
            b'a' => {body_degree = body_degree + inc;},
            b'd' => {body_degree = body_degree - inc;},
            _ => {}
        }

        if head_degree<0 {head_degree = 0;}
        else if head_degree>180{head_degree = 180;}
        
        if body_degree<0 {body_degree = 0;}
        else if body_degree>180{body_degree = 180;}

        body_servo.rotate(body_degree as u64);
        head_servo.rotate(head_degree as u64);

        Timer::after_micros(100).await;
    }
}