// Define the Agent trait here
// TODO: The trait should have a function `act(&self) -> Action`

// Define the LoyalAgent and TraitorAgent structs
// TODO: Define structs `LoyalAgent` and `TraitorAgent`

// Define an Action enum: Uphold and Betray
// TODO: Define the Action enum with variants `Help` and `Betray`

// Implement the Agent trait for LoyalAgent and TraitorAgent
// TODO: Implement `Agent` for `LoyalAgent` and `TraitorAgent`

// Implement Debug and Display for Action
// TODO: Implement the `Debug` and `Display` traits for `Action`

// Define a function `process_agent` that takes a reference to an Agent.
// The `process_agent` function should call the agent's `act` method.
// Use a `where` clause to specify trait bounds.
// TODO: Implement `process_agent` function with appropriate trait bounds using `where` clause

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_display() {
        assert_eq!(format!("{}", Action::Help), "Help");
        assert_eq!(format!("{}", Action::Betray), "Betray");
    }

    #[test]
    fn test_loyal_agent() {
        let agent = LoyalAgent {};
        assert_eq!(agent.act(), Action::Help);
    }

    #[test]
    fn test_traitor_agent() {
        let agent = TraitorAgent {};
        assert_eq!(agent.act(), Action::Betray);
    }

    #[test]
    fn test_process_agent() {
        let loyal = LoyalAgent {};
        let traitor = TraitorAgent {};

        assert_eq!(process_agent(&loyal), Action::Help);
        assert_eq!(process_agent(&traitor), Action::Betray);
    }

    #[test]
    fn test_debug_printing() {
        let action = Action::Help;
        assert_eq!(format!("{:?}", action), "Help");
    }
}