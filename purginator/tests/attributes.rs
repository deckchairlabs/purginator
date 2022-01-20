#[cfg(test)]
pub mod tests {
    use purginator::test_utils::run_test;

    fn test() -> String {
        run_test("attribute_selector")
    }

    #[test]
    fn it_always_keeps_attribute_when_attribute_is_value() {
        let result = test();
        assert!(result.contains("input[value=\"\"]"));
    }

    #[test]
    fn it_handles_attribute() {
        let result = test();

        // Keep used css
        assert!(result.contains("a[target]"));
        assert!(result.contains("input[checked]"));
        // Remove unused css
        assert!(!result.contains("a[invented]"));
    }

    #[test]
    fn it_handles_attribute_equals_value() {
        let result = test();

        // Keep used css
        assert!(result.contains("a[target=\"_blank\"]"));
        // Remove unused css
        assert!(!result.contains("a[target=\"no_blank\"]"));
    }

    #[test]
    fn it_handles_attribute_containing_value() {
        let result = test();

        // Keep used css
        assert!(result.contains("input[title~=\"flower\"]"));
        // Remove unused css
        assert!(!result.contains("input[title~=\"grass]"));
    }

    #[test]
    fn it_handles_attribute_equals_value_or_begins() {
        let result = test();

        // Keep used css
        assert!(result.contains("html[lang|=\"en\"]"));
        // Remove unused css
        assert!(!result.contains("html[lang|=\"fr\"]"));
    }

    #[test]
    fn it_handles_attribute_starts_with_value() {
        let result = test();

        // Keep used css
        assert!(result.contains("a[href^=\"http\"]"));
        // Remove unused css
        assert!(!result.contains("a[href^=\"ssl\"]"));
    }

    #[test]
    fn it_handles_attribute_ending_with_value() {
        let result = test();

        // Keep used css
        assert!(result.contains("a[href$=\"pdf\"]"));
        // Remove unused css
        assert!(!result.contains("a[href$=\"jpg\"]"));
        assert!(!result.contains("a[href$=\"http\"]"));
    }

    #[test]
    fn it_handles_attribute_containing_at_least_value() {
        let result = test();

        // Keep used css
        assert!(result.contains("a[title*=\"thin\"]"));
        // Remove unused css
        assert!(!result.contains("a[title*=\"fat\"]"));
    }

    #[test]
    fn it_handles_spaces_in_attribute_selector() {
        let result = test();

        // Keep used css
        assert!(result.contains("[class*=\" class2\"]"));
        assert!(result.contains("[class*=\"class1 class2 \"]"));
    }
}
