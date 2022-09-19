mod m5core2;

use std::time::Duration;

use embedded_svc::sys_time::SystemTime;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn sleep(duration: Duration) {
    let start = esp_idf_svc::systime::EspSystemTime.now();

    loop {
        let now = esp_idf_svc::systime::EspSystemTime.now();

        // println!("{:?}", esp_idf_hal::interrupt::task::current());
        esp_idf_hal::interrupt::task::do_yield();

        if now - start > duration {
            break;
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut m5 = m5core2::M5::default();

    let mut i = 0;
    let mut state = false;
    m5.set_vibration(state);

    let time = esp_idf_svc::systime::EspSystemTime;

    loop {
        println!("{}, {:?}", i, time.now());
        i += 1;
        sleep(Duration::from_millis(1000));
        // esp_idf_hal::delay::Ets.delay_ms(1000u32);

        m5.set_vibration(state);
        state = !state;
    }
}
