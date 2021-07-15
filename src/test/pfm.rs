use crate::test::Test;
use std::process::Command;

// TEST: Check for presence of libpfm4
pub fn test_check_for_libpfm4() -> Test {
    // This uses the linux command "ldconfig" and returns
    // based on whether or not the output contains "libpfm."
    fn check_for_libpfm4() -> bool {
        let output = Command::new("ldconfig")
            .args(&["-p"])
            .output()
            .expect("Issue running command.");
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            if text.contains("libpfm") {
                return true;
            }
        }
        false
    }

    Test {
        name: "has_libpfm4".to_string(),
        description: "Checks for presence of libpfm4".to_string(),
        call: check_for_libpfm4,
        subtests: Vec::new(),
        is_subtest: false,
    }
}
