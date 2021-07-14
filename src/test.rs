//! Test driver
use crate::utils::ParseError;
use std::str::FromStr;
extern crate structopt;
use structopt::StructOpt;
pub mod basic;
pub mod pfm;
pub mod testutils;

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
/// Handles the running of the "test" command.
pub fn run_test(options: &TestOptions) {
    let tests = testutils::make_tests();
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
            TestEvent::RunAll | TestEvent::RunSome => testutils::run_all_tests(&tests, &to_skip),
            TestEvent::List => testutils::list_all_tests(&tests),
        }
    }
}
