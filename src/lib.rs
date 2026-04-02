pub const VERSION: &str = "0.1.0";
pub const NAME: &str = "cli-template-rust";

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
    }

    #[test]
    fn test_greet_custom() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
    }
}
