//! Test driver
use crate::utils::ParseError;
use std::io::stdout;
use std::io::Write;
use std::str::FromStr;
extern crate structopt;
use structopt::StructOpt;
pub mod basic;
pub mod pfm;

/// Test Struct
pub struct Test {
    pub name: String,
    pub description: String,
    pub call: fn() -> bool,
}

/// TestResult
pub enum TestResult {
    Passed,
    Failed,
    Skipped,
}

/// TestEvents
#[derive(Debug)]
pub enum TestEvent {
    RunAll,
    RunSome,
    List,
}

/// Match on each supported event to parse from command line
impl FromStr for TestEvent {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(TestEvent::RunAll),
            "list" => Ok(TestEvent::List),
            "-s" => Ok(TestEvent::RunSome),
            "--skip" => Ok(TestEvent::RunSome),
            _ => Err(ParseError::InvalidEvent),
        }
    }
}

/// Configuration settings for running test
#[derive(Debug, StructOpt)]
pub struct TestOptions {
    #[structopt(short, long, help = "Event to collect", number_of_values = 1)]
    pub event: Vec<TestEvent>,

    // Allows multiple arguments to be passed, collects everything remaining on
    // the command line
    #[structopt(required = false, help = "Command to run")]
    pub command: Vec<String>,
}

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

/// Handles the running of the "test" command.
pub fn run_test(options: &TestOptions) {
    let tests = make_tests();
    let mut events = Vec::new();
    let mut to_skip: Vec<usize> = Vec::new();
    if options.command.is_empty() {
        events.push(TestEvent::RunAll);
    }
    for (index, command) in options.command.iter().enumerate() {
        let possible_event = TestEvent::from_str(command);
        let event = match possible_event {
            Ok(e) => e,
            Err(_) => {
                println!("Incorrect parameter");
                //TODO print usage
                return;
            }
        };
        match event {
            TestEvent::RunSome => {
                for test_to_skip in &options.command[index + 1..] {
                    let parsed = test_to_skip.trim().parse();
                    match parsed {
                        Ok(number) => to_skip.push(number),
                        Err(_) => {}
                    }
                }
                events.push(TestEvent::RunSome);
                break;
            }
            _ => events.push(event),
        }
    }
    for event in &events {
        match event {
            TestEvent::RunAll | TestEvent::RunSome => run_all_tests(&tests, &to_skip),
            TestEvent::List => list_all_tests(&tests),
        }
    }
}
