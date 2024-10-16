use std::num::ParseIntError;
use thiserror::Error;

#[allow(dead_code)]
fn run_error_machine(input1: &str, input2: u32, input3: u32) -> Result<(), ()> {
    consume_error(task_one_parse_number(input1));
    consume_error(task_two_divide_150_by_number(input2));
    consume_error(task_three_check_result_is_even(input3));
    Err(())
}

fn task_one_parse_number(input: &str) -> Result<u32, MachineError> {
    let number: Result<u32, ParseIntError> = input.parse();
    number.map_err(MachineError::ParseError)
}

fn task_two_divide_150_by_number(divisor: u32) -> Result<u32, MachineError> {
    150u32.checked_div(divisor).ok_or(MachineError::DivideByZeroError)
}

fn task_three_check_result_is_even(value: u32) -> Result<(), MachineError> {
    if value % 2 == 0 {
        Ok(())
    } else {
        Err(MachineError::ValidationError("Result is not even.".to_string()))
    }
}

fn consume_error<T>(input: Result<T, MachineError>) {
    match input {
        Ok(_) => panic!("I only eat errors!"),
        Err(e) => println!("Error consumed: {}", e),
    }
}

#[derive(Debug, Error)]
enum MachineError {
    #[error(transparent)]
    ParseError(#[from] ParseIntError),
    #[error("Divide by zero error")]
    DivideByZeroError,
    #[error("Validation error: {0}")]
    ValidationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_machine_with_errors() {
        // Provide inputs that will cause errors
        let result = run_error_machine("abc", 0, 3);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_machine_with_success() {
        // Provide inputs that should not cause errors
        let result = std::panic::catch_unwind(|| {
            run_error_machine("42", 1, 2)
        });
        // Since tasks succeed, consume_error should panic, and the catch_unwind should return Err
        assert!(result.is_err(), "consume_error did not panic when tasks succeeded");
    }

    #[test]
    #[should_panic(expected = "I only eat errors!")]
    fn test_consume_error_with_ok() {
        // Test that consume_error panics when given an Ok value
        consume_error(Ok(()));
    }
}