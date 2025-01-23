
use {
    crate::resources::gpio_list::{ButtonResources},
    embassy_rp::gpio::{Input, Pull, Level},
    embassy_time::{Timer, Duration, with_deadline, Instant},
    {defmt_rtt as _, panic_probe as _},
    crate::tasks::{
        display::send_command as send_display,
        display::Command as DisplayCommand,
        control_task::send_command as send_main_command,
        control_task::Command as MainCommand,
        
    },
};

pub struct Debouncer<'a> {
    input: Input<'a>,
    debounce: Duration,
}

impl<'a> Debouncer<'a> {
    pub fn new(input: Input<'a>, debounce: Duration) -> Self {
        Self { input, debounce }
    }

    pub async fn debounce(&mut self) -> Level {
        loop {
            let l1 = self.input.get_level();

            self.input.wait_for_any_edge().await;

            Timer::after(self.debounce).await;

            let l2 = self.input.get_level();
            if l1 != l2 {
                break l2;
            }
        }
    }
}

#[embassy_executor::task]
pub async fn button_task(r: ButtonResources) {
    let mut brightness_status = true;
    let mut main_button = Debouncer::new(Input::new(r.MAIN_BUTTON, Pull::Down), Duration::from_millis(20));
    
    loop {
        // button pressed
        main_button.debounce().await;
        let start = Instant::now();

        match with_deadline(start + Duration::from_millis(500), main_button.debounce()).await {
            Ok(_) => {
                log::info!("Button Press");
                send_main_command(MainCommand::Start);
                continue;
            }
            Err(_) => {
                brightness_status = !brightness_status;
                send_display(DisplayCommand::Brightness(brightness_status));
                log::info!("Button Held");
            }
        }

        // wait for button release before handling another press
        main_button.debounce().await;
        log::info!("Button pressed for: {}ms", start.elapsed().as_millis());
    }
}
