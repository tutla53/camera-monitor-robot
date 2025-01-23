//! Servo PIO Task with state machine 0 and 1 

use {
    crate::resources::gpio_list::{
        Irqs,
        ControlResources,
    },
    embassy_rp::{
        gpio::{Input, Output, Pull, Level},
        pio::Pio,
        pio_programs::{
            uart::{PioUartTx, PioUartTxProgram},
        },
        adc::{Adc, Config, Channel},
    },
    crate::tasks::{
        display::send_command as send_display,
        display::Command as DisplayCommand,
    },
    embedded_io_async::Write,
    embassy_time::{Timer, Instant, Duration, with_deadline},
    embassy_sync::{
        signal::Signal,
        blocking_mutex::raw::CriticalSectionRawMutex,
    },
    {defmt_rtt as _, panic_probe as _},
};

static MAIN_CONTROL: Signal<CriticalSectionRawMutex, Command> = Signal::new();

pub enum Command {
    Start,
}

pub fn send_command(command: Command) {
    MAIN_CONTROL.signal(command);
}

async fn wait_command() -> Command {
    MAIN_CONTROL.wait().await
}

#[embassy_executor::task]
pub async fn control_task(r: ControlResources) {
    let Pio { mut common, sm0, .. } = Pio::new(r.UART_PIO_CH, Irqs);
    let mut adc = Adc::new(r.ADC_PERIPHERAL, Irqs, Config::default());
    
    let mut hc_state_btn = Input::new(r.HC_STATE_PIN, Pull::Down);
    let mut hc_power = Output::new(r.HC_POWER, Level::Low);

    let mut head_pin = Channel::new_pin(r.ADC_HEAD_PIN, Pull::None);
    let mut body_pin = Channel::new_pin(r.ADC_BODY_PIN, Pull::None);

    let tx_program = PioUartTxProgram::new(&mut common);
    let mut uart_tx = PioUartTx::new(9600, &mut common, sm0, r.UART_TX_PIN, &tx_program);

    let task_timeout_s = 60;

    Timer::after_secs(1).await;

    loop {
        log::info!("Waiting for the Button...");
        send_display(DisplayCommand::Status(0));

        let _ = wait_command().await;

        let task_timeout = Duration::from_secs(task_timeout_s);
        let mut task_start = Instant::now();
        let mut task_deadline = task_start + task_timeout;

        hc_power.set_high();

        log::info!("Button is Up");
        log::info!("Waiting to connect to the Robot...");
        send_display(DisplayCommand::Status(1));
        
        let hc_status = match with_deadline(task_deadline, hc_state_btn.wait_for_high()).await{
            // Waiting for HC-05 to be paired with the robot
            Ok(_) => {
                send_display(DisplayCommand::Status(2));
                log::info!("HC-05 is connected");
                true
            }
            Err(_) => {
                send_display(DisplayCommand::Status(3));
                log::info!("HC-05 is not connected or paired");
                false
            }
        };

        task_start = Instant::now();
        task_deadline = task_start + task_timeout;

        if hc_status {
            loop{
                if Instant::now() >= task_deadline || hc_state_btn.is_low(){
                    hc_power.set_low();
                    log::info!("Stopping Control Task");
                    send_display(DisplayCommand::Status(5));
                    break;
                }
                log::info!("Running Control Task");
                send_display(DisplayCommand::Status(4));

                let head_val = adc.read(&mut head_pin).await.unwrap();
                let body_val = adc.read(&mut body_pin).await.unwrap();

                if head_val > 3500 { uart_tx.write("w".as_bytes()).await.unwrap(); }
                else if head_val < 500{ uart_tx.write("s".as_bytes()).await.unwrap(); }

                if body_val > 3500 { uart_tx.write("d".as_bytes()).await.unwrap(); }
                else if body_val < 500{ uart_tx.write("a".as_bytes()).await.unwrap(); }

                Timer::after_millis(300).await;
            }
        }
    }
}