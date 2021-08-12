use crate::bindings::*;
use crate::event::fd::perf_event_open;
use crate::event::sys::linux::*;
use crate::event::sys::wrapper::*;
use crate::event::utils::*;
use crate::test::RunSettings;
use crate::test::Test;
use crate::test::TestResult;

// Dummy function for parent test with subtests
fn dummy(_settings: &RunSettings) -> TestResult {
    TestResult::Passed
}

pub fn test_fd() -> Test {
    fn test_perf_event_open() -> Test {
        fn check_perf_event_open(settings: &RunSettings) -> TestResult {
            let event = &mut perf_event_attr {
                type_: perf_type_id_PERF_TYPE_HARDWARE,
                size: std::mem::size_of::<perf_event_attr>() as u32,
                config: perf_hw_id_PERF_COUNT_HW_INSTRUCTIONS as u64,
                ..Default::default()
            };
            event.set_disabled(1);
            event.set_exclude_hv(1);
            event.set_exclude_hv(1);
            let fd: isize;
            fd = perf_event_open(event, 0, -1, -1, 0);
            if fd == -1 {
                if settings.verbose {
                    return TestResult::Failed(format!(
                        "perf_event_open returned {}, which is an error code",
                        fd
                    ));
                }
                return TestResult::Failed(format!("{}", fd));
            }
            TestResult::Passed
        }
        Test {
            name: "perf_event_open".to_string(),
            description: "perf_event_open sanity check".to_string(),
            call: check_perf_event_open,
            subtests: Vec::new(),
            is_subtest: true,
        }
    }

    fn test_read() -> Test {
        fn check_read(settings: &RunSettings) -> TestResult {
            use libc::{ioctl, read};
            let event = &mut perf_event_attr {
                type_: perf_type_id_PERF_TYPE_HARDWARE,
                size: std::mem::size_of::<perf_event_attr>() as u32,
                config: perf_hw_id_PERF_COUNT_HW_CPU_CYCLES as u64,
                ..Default::default()
            };
            event.set_disabled(1);
            event.set_exclude_kernel(1);
            event.set_exclude_hv(1);
            let fd: isize;
            fd = perf_event_open(event, 0, -1, -1, 0);
            let mut cnt: u64 = 0;
            let buf: *mut libc::c_void = &mut cnt as *mut _ as *mut libc::c_void;
            unsafe {
                ioctl(fd as i32, ENABLE as u64, 0);
                read(fd as i32, buf, std::mem::size_of_val(&cnt));
            }
            if cnt == 0 {
                if settings.verbose {
                    return TestResult::Failed(format!(
                        "\nINFO\tThe CPU cycles count was {}. It should be bigger than that",
                        cnt
                    ));
                }
                return TestResult::Failed(format!("({})", cnt));
            }
            TestResult::Passed
        }
        Test {
            name: "test_read_ioctl".to_string(),
            description: "read / ioctl sanity check".to_string(),
            call: check_read,
            subtests: Vec::new(),
            is_subtest: true,
        }
    }

    Test {
        name: "fd_sanity".to_string(),
        description: "File descriptor sanity tests".to_string(),
        call: dummy,
        subtests: vec![test_perf_event_open(), test_read()],
        is_subtest: false,
    }
}
