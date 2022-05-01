mod helpers;

#[cfg(test)]
pub mod tests {
    use crate::helpers;

    fn test() -> String {
        helpers::run_test("css-variables/variables")
    }

    #[test]
    fn it_keeps_primary_color() {
        let result = test();
        assert!(result.contains("--primary-color:"));
    }

    #[test]
    fn it_keeps_accent_color_used_color() {
        let result = test();
        assert!(result.contains("--accent-color:"));
        assert!(result.contains("--used-color:"));
    }

    #[test]
    #[ignore]
    fn it_removes_tertiary_color_unused_color_and_button_color() {
        let result = test();
        assert!(!result.contains("--tertiary-color"));
        assert!(!result.contains("--unused-color"));
        assert!(!result.contains("--button-color"));
    }

    #[test]
    fn it_keeps_color_first_wrong_order() {
        let result = test();
        assert!(result.contains("--color-first:"));
        assert!(result.contains("--wrong-order:"));
    }
}
