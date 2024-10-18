// Eliminate all .unwrap() calls! Use appropriate handling instead.
// Some of the tests will currently FAIL due to the use of .unwrap() calls,
// as this makes the code panic when an error occurs.

// For simplicity, we'll use String as the error type throughout, so you'll
// see Result<(), String> and Result<String, String>.

/// Simulates hearing a knock at the door.
/// Returns a Result indicating whether a knock was heard.
pub fn hear_knock(knock_option: Option<bool>) -> Result<(), String> {
    let knock = knock_option.unwrap();
    if knock {
        Ok(())
    } else {
        Err("No knock heard".to_string())
    }
}

/// Simulates opening the door.
/// Returns a Result indicating if the door was successfully opened.
pub fn open_door(door_status_option: Option<&'static str>) -> Result<(), String> {
    let door_status = door_status_option.unwrap();
    if door_status == "open" {
        Ok(())
    } else {
        Err("The door is jammed!".to_string())
    }
}

/// Determines who is at the door.
/// Returns an Option with the visitor's name.
pub fn who_is_there(visitor_option: Option<String>) -> Option<String> {
    let visitor = visitor_option.unwrap();

    if visitor.is_empty() {
        None
    } else {
        Some(visitor)
    }
}

/// Poe handles the visitor.
/// Returns a Result<String, String> describing the outcome.
pub fn handle_visitor(
    knock_option: Option<bool>,
    door_status_option: Option<&'static str>,
    visitor_option: Option<String>,
) -> Result<String, String> {
    hear_knock(knock_option).unwrap();
    open_door(door_status_option).unwrap();

    let visitor = who_is_there(visitor_option).unwrap();

    if visitor == "the Raven" {
        Ok("'Quoth the Raven, 'Nevermore.''".to_string())
    } else {
        Ok(format!("You see {} at the door.", visitor))
    }
}

// You don't need to remove .unwrap() calls when they're used in tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lenore_at_the_door() {
        // Set up the scenario where Lenore is at the door.
        let result = handle_visitor(
            Some(true),
            Some("open"),
            Some("Lenore".to_string()),
        )
        .unwrap();
        assert_eq!(result, "You see Lenore at the door.");
    }

    #[test]
    fn test_raven_at_the_door() {
        // Simulate the Raven's arrival.
        let result = handle_visitor(
            Some(true),
            Some("open"),
            Some("the Raven".to_string()),
        )
        .unwrap();
        assert_eq!(result, "'Quoth the Raven, 'Nevermore.''");
    }

    #[test]
    fn test_no_one_at_the_door() {
        // Simulate no one at the door.
        let result = handle_visitor(
            Some(true),
            Some("open"),
            Some(String::new()),
        );
        assert_eq!(result.unwrap_err(), "No one is at the door.");
    }

    #[test]
    fn test_door_jammed() {
        // Simulate that the door is jammed.
        let result = handle_visitor(
            Some(true),
            Some("jammed"),
            Some("Lenore".to_string()),
        );
        assert_eq!(result.unwrap_err(), "The door is jammed!");
    }

    #[test]
    fn test_no_knock_heard() {
        // Simulate that no knock was heard.
        let result = handle_visitor(
            Some(false),
            Some("open"),
            Some("Lenore".to_string()),
        );
        assert_eq!(result.unwrap_err(), "No knock heard");
    }

    #[test]
    fn test_error_in_getting_knock_result() {
        let result = handle_visitor(
            None,
            Some("open"),
            Some("Lenore".to_string()),
        );
        assert_eq!(
            result.unwrap_err(),
            "Unable to determine if there was a knock."
        );
    }
}
