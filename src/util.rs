use std::ops::{Add, Mul, MulAssign};

#[macro_export]
macro_rules! debug_with_rate {
    ($($arg:tt)+) => {
        {
            static mut LAST_REPORT_TS: Option<std::time::SystemTime> = None;
            static mut LAST_REPORT_ITER: usize = 0;
            static mut RATE_ESTIMATE: f64 = 10.;
            static mut ITER_COUNT: usize = 0;
            static mut NEXT_REPORT_AT_ITER: usize = 100;

            unsafe {
                #[allow(static_mut_refs)]
                if LAST_REPORT_TS.is_none() {
                    LAST_REPORT_TS = Some(std::time::SystemTime::now());
                }
                ITER_COUNT += 1;
                if ITER_COUNT >= NEXT_REPORT_AT_ITER {
                    let ts = std::time::SystemTime::now();
                    let new_rate_estimate = (RATE_ESTIMATE * 2.).min(
                        ((ITER_COUNT - LAST_REPORT_ITER) as f64) / ts.duration_since(LAST_REPORT_TS.unwrap()).unwrap().as_secs_f64());
                    NEXT_REPORT_AT_ITER = ITER_COUNT + (10. * new_rate_estimate + 1.) as usize;
                    LAST_REPORT_TS = Some(ts);
                    LAST_REPORT_ITER = ITER_COUNT;
                    RATE_ESTIMATE = new_rate_estimate;
                    let debug_msg = format!($($arg)+);
                    #[allow(static_mut_refs)]
                    {
                        log::debug!("[rate=={:.2}/s iter#{}] {}", new_rate_estimate, ITER_COUNT, debug_msg);
                    }
                }
            }
        }
    };
}

/**
 * Concatenate two integer numbers as decimal strings.
 *
 * E.g. 12 + 34 = 1234
 */
pub fn concatenate_numbers<T: Ord + Mul<Output = T> + Add<Output = T> + MulAssign + From<u8>>(
    a: T,
    b: T,
) -> T {
    let mut factor: T = 10.into();
    while factor <= b {
        factor *= 10.into();
    }
    a * factor + b
}
