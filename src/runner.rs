use crate::lang::{parser, interpreter, types};
use crate::challenges::TestCase;
use anyhow::{Result, anyhow};
use std::rc::Rc;
use std::time::{Duration, Instant};

const TIMEOUT_SECS: u64 = 2;

#[derive(Debug, Clone)]
pub struct TestResult {
    pub passed: bool,
    pub expected: String,
    pub actual: String,
    pub description: String,
    pub error: Option<String>,
}

pub struct Runner {
    timeout_duration: Duration,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            timeout_duration: Duration::from_secs(TIMEOUT_SECS),
        }
    }

    pub fn run_tests(&self, code: &str, test_cases: &[TestCase]) -> Vec<TestResult> {
        test_cases.iter().map(|tc| self.run_single_test(code, tc)).collect()
    }

    fn run_single_test(&self, code: &str, test_case: &TestCase) -> TestResult {
        let start = Instant::now();

        let result = self.execute_with_timeout(code, &test_case.input, start);

        match result {
            Ok(actual) => {
                let actual_str = actual.trim();
                let expected_str = test_case.expected.trim();
                let passed = actual_str == expected_str;

                TestResult {
                    passed,
                    expected: expected_str.to_string(),
                    actual: actual_str.to_string(),
                    description: test_case.description.clone(),
                    error: None,
                }
            }
            Err(e) => TestResult {
                passed: false,
                expected: test_case.expected.clone(),
                actual: "".to_string(),
                description: test_case.description.clone(),
                error: Some(e.to_string()),
            },
        }
    }

    fn execute_with_timeout(&self, code: &str, input: &str, start: Instant) -> Result<String> {
        // Check if we've already exceeded timeout
        if start.elapsed() > self.timeout_duration {
            return Err(anyhow!("Execution timeout exceeded"));
        }

        // Parse the user's code
        let user_expr = parser::parse(code)
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        // Type check
        let mut type_checker = types::TypeChecker::new();
        let mut type_env = types::get_builtin_env();
        type_checker.infer(&user_expr, &mut type_env)
            .map_err(|e| anyhow!("Type error: {}", e))?;

        // Create a function application with the input
        let full_code = if input.is_empty() {
            code.to_string()
        } else {
            format!("({}) {}", code, input)
        };

        // Parse and evaluate the full expression
        let expr = parser::parse(&full_code)
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        let mut interpreter = interpreter::Interpreter::new();
        let env = Rc::new(interpreter::get_builtin_env());

        // Simple timeout check - in a real implementation we'd use a separate thread
        let value = interpreter.eval(&expr, &env)
            .map_err(|e| {
                if e.to_string().contains("Maximum recursion depth") {
                    anyhow!("Infinite recursion detected")
                } else {
                    e
                }
            })?;

        if start.elapsed() > self.timeout_duration {
            return Err(anyhow!("Execution timeout exceeded"));
        }

        Ok(value.to_string_repr())
    }

    pub fn count_chars(&self, code: &str) -> usize {
        code.chars().filter(|c| !c.is_whitespace()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_execution() {
        let runner = Runner::new();
        let test_case = TestCase {
            input: "5".to_string(),
            expected: "10".to_string(),
            description: "double 5".to_string(),
        };

        let result = runner.run_single_test("\\x -> x * 2", &test_case);
        assert!(result.passed, "Expected pass but got: {:?}", result);
    }

    #[test]
    fn test_char_count() {
        let runner = Runner::new();
        assert_eq!(runner.count_chars("\\x -> x * 2"), 9);
        assert_eq!(runner.count_chars("  \\x  ->  x * 2  "), 9);
    }
}
