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

    Test {
        name: "fd_sanity".to_string(),
        description: "File descriptor sanity tests".to_string(),
        call: dummy,
        subtests: vec![test_perf_event_open()],
        is_subtest: false,
    }
}
