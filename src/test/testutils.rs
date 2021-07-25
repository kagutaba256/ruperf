use crate::test::basic;
use crate::test::pfm;
use crate::test::RunSettings;
use crate::test::Test;
use crate::test::TestResult;
use serde_json::json;
use serde_json::to_string_pretty;
use std::io::stdout;
use std::io::Write;
/// Gathers all tests and returns a Vec with them all
pub fn make_tests() -> Vec<Test> {
    let tests = vec![
        basic::test_always_passes(),
        basic::test_always_fails(),
        basic::test_passes_after_1sec(),
        basic::test_with_pointless_subtests(),
        pfm::test_check_for_libpfm4(),
    ];
    tests
}

/// Runs all tests and outputs results to stdout
pub fn run_all_tests(tests: &[Test], to_skip: &[String], settings: &RunSettings) {
    let mut should_skip;
    let mut tests_passed = 0;
    let mut tests_failed = 0;
    let mut tests_skipped = 0;
    let mut results_as_json: Vec<serde_json::Value> = Vec::new();
    if settings.json {}
    for (index, test) in tests.iter().enumerate() {
        should_skip = to_skip.iter().any(|i| *i == index.to_string());
        let result = run_single_test(&test, index, should_skip, "".to_string(), &settings);
        if settings.json {
            let result_string: String;
            match result {
                TestResult::Passed => {
                    tests_passed += 1;
                    result_string = String::from("passed");
                }
                TestResult::Failed(_) => {
                    tests_failed += 1;
                    result_string = String::from("failed");
                }
                TestResult::Skipped => {
                    tests_skipped += 1;
                    result_string = String::from("skipped");
                }
            }
            results_as_json.push(json!({
                "name": test.name,
                "description": test.description,
                "result": result_string,
                "number": index as u32
            }));
        }
    }
    if settings.json {
        let result_object = json!({
            "tests_available": results_as_json.len(),
            "tests_ran": results_as_json.len() - tests_skipped,
            "tests_passed": tests_passed,
            "tests_failed": tests_failed,
            "tests_skipped": tests_skipped,
            "results": results_as_json,
        });
        println!("{}", to_string_pretty(&result_object).unwrap());
    }
}

/// Runs a single test (or subtest)
pub fn run_single_test(
    test: &Test,
    index: usize,
    should_skip: bool,
    parent_index_string: String,
    settings: &RunSettings,
) -> TestResult {
    if !settings.json {
        print!(
            "{:>2}{}: {:<60} : ",
            parent_index_string, index, test.description
        );
        stdout().flush().unwrap();
    }
    let result_type: TestResult;
    if should_skip {
        result_type = TestResult::Skipped;
    } else if test.subtests.is_empty() {
        result_type = (test.call)(&settings);
    } else {
        if !settings.json {
            println!();
        }
        let mut overall_result_type: TestResult = TestResult::Passed;
        for (i, subtest) in test.subtests.iter().enumerate() {
            let result = run_single_test(subtest, i, false, index.to_string() + ".", settings);
            if let TestResult::Failed(_) = result {
                overall_result_type = TestResult::Failed(String::new());
            }
        }
        result_type = overall_result_type;
        return result_type;
    }
    if !settings.json {
        let result_text: String = match &result_type {
            TestResult::Skipped => "\x1b[0;33mSkip\x1b[0m".to_string(),
            TestResult::Passed => "Ok".to_string(),
            TestResult::Failed(s) => format!("\x1b[0;31mFAILED!\x1b[0m {}", s),
        };
        println!("{}", result_text);
    }
    result_type
}

/// Lists all tests and outputs results to stdout
pub fn list_all_tests(tests: &[Test]) {
    for (index, test) in tests.iter().enumerate() {
        println!("{:>2}: {:<60}", index, test.description);
    }
}
