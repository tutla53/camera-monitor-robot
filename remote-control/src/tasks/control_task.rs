//! Servo PIO Task with state machine 0 and 1 

use {
    crate::resources::gpio_list::{
        Irqs,
        ControlResources,
    },
    embassy_rp::{
        gpio::Pull,
        pio::Pio,
        pio_programs::{
            uart::{
                PioUartTx,
                PioUartTxProgram,
            },
        },
        adc::{
            Adc, 
            Config,
            Channel,
        },
    },
    embedded_io_async::Write,
    embassy_time::Timer,
    {defmt_rtt as _, panic_probe as _},
};

#[embassy_executor::task]
pub async fn control_task(r: ControlResources) {
    let Pio { mut common, sm0, .. } = Pio::new(r.UART_PIO_CH, Irqs);
    let mut adc = Adc::new(r.ADC_PERIPHERAL, Irqs, Config::default());

    let mut head_pin = Channel::new_pin(r.ADC_HEAD_PIN, Pull::None);
    let mut body_pin = Channel::new_pin(r.ADC_BODY_PIN, Pull::None);

    let tx_program = PioUartTxProgram::new(&mut common);
    let mut uart_tx = PioUartTx::new(9600, &mut common, sm0, r.UART_TX_PIN, &tx_program);

    Timer::after_secs(2).await;

    loop {
        let head_val = adc.read(&mut head_pin).await.unwrap();
        let body_val = adc.read(&mut body_pin).await.unwrap();

        if head_val > 3500 { uart_tx.write("w".as_bytes()).await.unwrap(); }
        else if head_val < 500{ uart_tx.write("s".as_bytes()).await.unwrap(); }

        if body_val > 3500 { uart_tx.write("a".as_bytes()).await.unwrap(); }
        else if body_val < 500{ uart_tx.write("d".as_bytes()).await.unwrap(); }

        Timer::after_millis(100).await;
    }
}