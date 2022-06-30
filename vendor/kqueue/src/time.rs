use libc::timespec;
use std::time::Duration;

#[cfg(not(all(target_os = "freebsd", target_arch = "x86")))]
pub(crate) fn duration_to_timespec(d: Duration) -> timespec {
    let tv_sec = d.as_secs() as i64;
    let tv_nsec = d.subsec_nanos() as i64;

    if tv_sec.is_negative() {
        panic!("Duration seconds is negative");
    }

    if tv_nsec.is_negative() {
        panic!("Duration nsecs is negative");
    }

    timespec { tv_sec, tv_nsec }
}

#[cfg(all(target_os = "freebsd", target_arch = "x86"))]
pub(crate) fn duration_to_timespec(d: Duration) -> timespec {
    let tv_sec = d.as_secs() as i32;
    let tv_nsec = d.subsec_nanos() as i32;

    if tv_sec.is_negative() {
        panic!("Duration seconds is negative");
    }

    if tv_nsec.is_negative() {
        panic!("Duration nsecs is negative");
    }

    timespec { tv_sec, tv_nsec }
}

#[cfg(test)]
mod tests {
    use super::duration_to_timespec;
    use std::time::Duration;
    use std::{i64, u32};

    #[test]
    fn test_basic_duration_to_ts() {
        let d = Duration::new(4, 20);

        let ts = duration_to_timespec(d);

        assert_eq!(ts.tv_sec, 4);
        assert_eq!(ts.tv_nsec, 20);
    }

    #[test]
    #[should_panic]
    fn test_overflow() {
        let d = Duration::new(i64::MAX as u64 + 1, u32::MAX);
        let ts = duration_to_timespec(d);

        assert_eq!(ts.tv_sec, 1);
        assert_eq!(ts.tv_nsec, 1);
    }
}
