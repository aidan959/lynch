use std::time::{Instant, Duration};

use lazy_static::lazy_static;
use winapi::um::winnt::LARGE_INTEGER;

// Cached frequency.
lazy_static! {
    static ref FREQUENCY: u64 = {
        #[cfg(target_os = "windows")]
        {
            unsafe {
                let mut frequency : LARGE_INTEGER = std::mem::zeroed();
                winapi::um::profileapi::QueryPerformanceFrequency(&mut frequency as *mut _);
                *frequency.QuadPart() as u64
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            1_000_000_000
        }
    };
}

pub fn time_now() -> i64 {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let mut time : LARGE_INTEGER = std::mem::zeroed();
            winapi::um::profileapi::QueryPerformanceCounter(&mut time as *mut _);
            int64_mul_div(*time.QuadPart(), 1_000_000, *FREQUENCY as i64)
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let now = std::time::SystemTime::now();
        let duration = now.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
        duration.as_micros() as i64
    }
}

pub fn time_from(starting_time: i64) -> i64 {
    time_now() - starting_time
}

pub fn time_from_microseconds(starting_time: i64) -> f64 {
    time_microseconds(time_from(starting_time))
}

pub fn time_from_milliseconds(starting_time: i64) -> f64 {
    time_milliseconds(time_from(starting_time))
}

pub fn time_from_seconds(starting_time: i64) -> f64 {
    time_seconds(time_from(starting_time))
}

pub fn time_delta_seconds(starting_time: i64, ending_time: i64) -> f64 {
    time_seconds(ending_time - starting_time)
}

pub fn time_delta_milliseconds(starting_time: i64, ending_time: i64) -> f64 {
    time_milliseconds(ending_time - starting_time)
}

pub fn time_microseconds(time: i64) -> f64 {
    time as f64
}

pub fn time_milliseconds(time: i64) -> f64 {
    time as f64 / 1000.0
}

pub fn time_seconds(time: i64) -> f64 {
    time as f64 / 1_000_000.0
}

fn int64_mul_div(value: i64, numer: i64, denom: i64) -> i64 {
    let q = value / denom;
    let r = value % denom;

    q * numer + r * numer / denom
}