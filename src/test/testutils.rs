use crate::test::basic;
use crate::test::pfm;
use crate::test::Test;
use crate::test::TestResult;
use std::io::stdout;
use std::io::Write;
/// Gathers all tests and returns a Vec with them all
pub fn make_tests() -> Vec<Test> {
    let mut tests: Vec<Test> = Vec::new();

    // from basic.rs
    tests.push(basic::test_always_passes());
    tests.push(basic::test_always_fails());
    tests.push(basic::test_passes_after_1sec());

    // from pfm.rs
    tests.push(pfm::test_check_for_libpfm4());

    tests
}

/// Runs all tests and outputs results to stdout
pub fn run_all_tests(tests: &Vec<Test>, to_skip: &Vec<usize>) {
    let mut should_skip;
    for (index, test) in tests.iter().enumerate() {
        should_skip = to_skip.iter().any(|&i| i == index);
        run_single_test(&test, index, should_skip);
    }
}

/// Runs a single test (or subtest)
pub fn run_single_test(test: &Test, index: usize, should_skip: bool) {
    print!("{:>2}: {:<60} : ", index, test.description);
    stdout().flush().unwrap();
    let result_type: TestResult;
    if should_skip {
        result_type = TestResult::Skipped;
    } else {
        let result = (test.call)();
        if result {
            result_type = TestResult::Passed;
        } else {
            result_type = TestResult::Failed;
        }
    }
    let result_text: String = match result_type {
        TestResult::Skipped => "\x1b[0;33mSkip\x1b[0m",
        TestResult::Passed => "Ok",
        TestResult::Failed => "\x1b[0;31mFAILED!\x1b[0m",
    }
    .to_string();
    println!("{}", result_text);
}

/// Lists all tests and outputs results to stdout
pub fn list_all_tests(tests: &Vec<Test>) {
    for (index, test) in tests.iter().enumerate() {
        println!("{:>2}: {:<60}", index, test.description);
    }
}
