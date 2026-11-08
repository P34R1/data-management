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

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 64 * 1024 * 2);

    let mut rng = Rng::new(peripherals.RNG);
    let mut led = Output::new(peripherals.GPIO13, Level::Low, OutputConfig::default());

    // IGNORE EVERYTHING ABOVE THIS LINE. It's just setup that I'm too lazy to comment. The "meat" of the program is below.

    let length = 15000; // the length of the list

    let mut sorted: Vec<u32> = (0..length).map(|_| rng.random()).collect(); // makes random data once when the program starts. The numbers are reused for each sort.
    bubble(sorted.as_mut_slice()); // makes the original data set fully sorted (to be unsorted and resorted)

    loop { // the loop means everything from here to line 56 keeps repeating forever (or in reality until the device looses power)
        let k = rng.random() % length; // this is the "k" value. k is a positive whole number from 0 to length (15000)
        let mut list = unsort(sorted.as_slice(), k as usize); // this copies the list but with the last k elements reversed

        print!("{:0>2}%  k = {:<6}", k * 100 / length, k); // this was just for making the program, it didn't serve a purpose in measuring the data. it prints the % of the list that is unsorted and the k value.

        // these lines makes the program wait 7000ms (7 seconds) so that I have time to save the execution time from the last run into the spreadsheet.
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(7000) {}

        led.set_high(); // this turns the connected LED to "high" (aka. turns it on). I started my stopwatch when the LED turned on.
        bubble(list.as_mut_slice()); // this sorts the list.
        led.set_low(); // this turns the LED off. I stopped my stopwatch when the LED turned off.
        println!("|");
    } // line 56 is down here. once the program hits this line, it goes back up to the loop line and repeats by sorting a list with a different random "sortedness"
}

// This function returns a copy of "arr" with the last "n" elements in reverse order.
fn unsort(arr: &[u32], n: usize) -> Vec<u32> {
    let mut v = arr.to_vec();
    let start = v.len().saturating_sub(n);
    v[start..].reverse();
    v
}

// This function sorts "arr" in place using bubble sort.`
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
