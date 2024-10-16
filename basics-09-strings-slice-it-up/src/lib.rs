// Slice It Up Exercise

/// Function to concatenate a `&str` and a `String`, returning a new `String`.
pub fn concatenate_str_and_string(s1: &str, s2: String) -> String {
    // TODO: Implement this function
    unimplemented!()
}

/// Function to return a slice of a `String`.
pub fn get_slice_of_string(s: &String, start: usize, end: usize) -> &str {
    // TODO: Implement this function
    unimplemented!()
}

/// Function to convert a `&str` to a `String`.
pub fn str_to_string(s: &str) -> String {
    // TODO: Implement this function
    unimplemented!()
}

/// Function to convert a `String` to a `&str`.
pub fn string_to_str(s: &String) -> &str {
    // TODO: Implement this function
    unimplemented!()
}

/// Function to check if a `&str` is contained within a `String`.
pub fn contains_substring(s: &String, substr: &str) -> bool {
    // TODO: Implement this function
    unimplemented!()
}

/// Function to replace a substring in a `String` with another substring, returning a new `String`.
pub fn replace_substring(s: &String, from: &str, to: &str) -> String {
    // TODO: Implement this function
    unimplemented!()
}

/// Function to split a `&str` by whitespace and return a `Vec<String>`.
pub fn split_into_words(s: &str) -> Vec<String> {
    // TODO: Implement this function
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate_str_and_string() {
        let s1 = "Hello, ";
        let s2 = String::from("world!");
        let result = concatenate_str_and_string(s1, s2);
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_get_slice_of_string() {
        let s = String::from("Hello, world!");
        let slice = get_slice_of_string(&s, 7, 12);
        assert_eq!(slice, "world");
    }

    #[test]
    fn test_str_to_string() {
        let s = "Hello, world!";
        let string = str_to_string(s);
        assert_eq!(string, String::from(s));
    }

    #[test]
    fn test_string_to_str() {
        let s = String::from("Hello, world!");
        let slice = string_to_str(&s);
        assert_eq!(slice, "Hello, world!");
    }

    #[test]
    fn test_contains_substring() {
        let s = String::from("The quick brown fox jumps over the lazy dog");
        assert!(contains_substring(&s, "brown fox"));
        assert!(!contains_substring(&s, "purple unicorn"));
    }

    #[test]
    fn test_replace_substring() {
        let s = String::from("I like apples");
        let result = replace_substring(&s, "apples", "oranges");
        assert_eq!(result, "I like oranges");
    }

    #[test]
    fn test_split_into_words() {
        let s = "The quick brown fox";
        let words = split_into_words(s);
        assert_eq!(words, vec!["The", "quick", "brown", "fox"]);
    }
}