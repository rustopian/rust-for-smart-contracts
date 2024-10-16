// basics-05-unwrapping-at-my-chamber-door/src/lib.rs

/// Simulates hearing a knock at the door.
/// Returns a Result indicating whether a knock was heard.
pub fn hear_knock() -> Result<(), String> {
    // For this exercise, sometimes we hear a knock, sometimes not.
    // Simulate with an Option<bool>.
    let knock = get_knock_result().unwrap(); // TODO: Replace `.unwrap()` with appropriate handling.
    
    if knock {
        Ok(())
    } else {
        Err("No knock heard".to_string())
    }
}

/// Simulates opening the door.
/// Returns a Result indicating if the door was successfully opened.
pub fn open_door() -> Result<(), String> {
    // The door might be jammed.
    let door_status = check_door_status().unwrap(); // TODO: Replace `.unwrap()` with appropriate handling.
    
    if door_status == "open" {
        Ok(())
    } else {
        Err("The door is jammed!".to_string())
    }
}

/// Determines who is at the door.
/// Returns an Option with the visitor's name.
pub fn who_is_there() -> Option<String> {
    // There might be someone at the door.
    let visitor = get_visitor().unwrap(); // TODO: Replace `.unwrap()` with appropriate handling.
    
    if visitor.is_empty() {
        None
    } else {
        Some(visitor)
    }
}

/// Poe handles the visitor.
/// Returns a Result<String, String> describing the outcome.
// Use `?` in this function to handle errors.
pub fn handle_visitor() -> Result<String, String> {
    hear_knock()?; // TODO: This line is fine after handling `hear_knock()`.

    open_door()?; // TODO: This line is fine after handling `open_door()`.

    let visitor = who_is_there().unwrap(); // TODO: Replace `.unwrap()` with appropriate handling.

    if visitor == "the Raven" {
        Ok("'Quoth the Raven, 'Nevermore.''".to_string())
    } else {
        Ok(format!("You see {} at the door.", visitor))
    }
}

// Helper functions:

/// Simulates the result of hearing a knock.
/// Returns `Option<bool>`.
fn get_knock_result() -> Option<bool> {
    // Simulate randomness or potential failure.
    Some(true) // Change to `None` to simulate an error in getting the result.
}

/// Checks the status of the door.
/// Returns `Option<&'static str>`.
fn check_door_status() -> Option<&'static str> {
    Some("open") // Change to `None` to simulate an error.
}

/// Gets the visitor at the door.
/// Returns `Option<String>`.
fn get_visitor() -> Option<String> {
    Some("Lenore".to_string()) // Change to `None` or `Some(String::new())` for different scenarios.
}

// basics-05-unwrapping-at-my-chamber-door/tests/tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lenore_at_the_door() {
        // Set up the scenario where Lenore is at the door.
        let result = handle_visitor().unwrap();
        assert_eq!(result, "You see Lenore at the door.");
    }

    #[test]
    fn test_raven_at_the_door() {
        // Modify helper functions to simulate the Raven's arrival.
        fn get_visitor() -> Option<String> {
            Some("the Raven".to_string())
        }

        // Replace the original `get_visitor` with this one.
        // Since Rust doesn't support redefining functions in tests,
        // we can simulate this by adjusting the function to accept parameters.
        let result = handle_visitor().unwrap();
        assert_eq!(result, "'Quoth the Raven, 'Nevermore.''");
    }

    #[test]
    fn test_no_one_at_the_door() {
        // Simulate no one at the door.
        fn get_visitor() -> Option<String> {
            Some(String::new())
        }

        let result = handle_visitor().unwrap();
        assert_eq!(result, "You see  at the door.");
    }

    #[test]
    fn test_door_jammed() {
        // Simulate that the door is jammed.
        fn check_door_status() -> Option<&'static str> {
            Some("jammed")
        }

        let result = handle_visitor();
        assert_eq!(result.unwrap_err(), "The door is jammed!");
    }

    #[test]
    fn test_no_knock_heard() {
        // Simulate that no knock was heard.
        fn get_knock_result() -> Option<bool> {
            Some(false)
        }

        let result = handle_visitor();
        assert_eq!(result.unwrap_err(), "No knock heard");
    }

    #[test]
    fn test_error_in_getting_knock_result() {
        // Simulate an error in getting the knock result.
        fn get_knock_result() -> Option<bool> {
            None
        }

        let result = handle_visitor();
        assert_eq!(
            result.unwrap_err(),
            "Unable to determine if there was a knock."
        );
    }
}