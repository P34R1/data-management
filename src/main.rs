#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    main,
    rng::Rng,
    time::{Duration, Instant},
};

use esp_println::{print, println};

extern crate alloc;
use alloc::vec::Vec;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024 * 2);

    let length = 15000;

    let mut rng = Rng::new(peripherals.RNG);
    let mut led = Output::new(peripherals.GPIO13, Level::Low, OutputConfig::default());

    let mut sorted: Vec<u32> = (0..length).map(|_| rng.random()).collect();
    bubble(sorted.as_mut_slice());

    loop {
        let k = rng.random() % length;
        let mut list = unsort(sorted.as_slice(), k as usize);
        
        print!("{:0>2}%  k = {:<6}", k*100/length, k);

        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(5000) {}

        let t = Instant::now();
        led.set_high();
        bubble(list.as_mut_slice());
        led.set_low();
        let ms = t.elapsed().as_millis();
        println!("|{:>6}ms", ms);
    }
}

fn unsort(arr: &[u32], n: usize) -> Vec<u32> {
    let mut v = arr.to_vec();
    let start = v.len().saturating_sub(n);
    v[start..].reverse();
    v
}

fn bubble<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
