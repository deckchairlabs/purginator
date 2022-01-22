mod helpers;

#[cfg(test)]
pub mod tests {
    use crate::helpers;

    fn test(test_name: &str) -> String {
        helpers::run_test(test_name)
    }

    #[test]
    fn it_keeps_not_pseudo_class() {
        let result = test("pseudo-class/not");

        assert!(result.contains(".foo-bar"));
        assert!(result.contains(".foo"));
    }

    #[test]
    fn it_keeps_pseudo_selectors() {
        let result = test("pseudo-class/pseudo_selector");

        assert!(result.contains(".some-item:nth-child(2n)"));
        assert!(result.contains(".some-item:nth-child(2n+1)"));
        assert!(result.contains(".some-item:nth-of-type(n+3)"));
        assert!(result.contains(".some-item:nth-of-type(-n+6)"));
        assert!(result.contains(".some-item:nth-of-type(-n+6)"));
        assert!(!result.contains(".unused:only-child"));
        assert!(!result.contains(".used:only-child"));
        assert!(result.contains(".odd-item:nth-child(2n+1)"));
    }

    #[test]
    fn it_keeps_nth_child_selectors() {
        let result = test("pseudo-class/nth_child");

        assert!(result.contains(".some-item:nth-child(2n)"));
        assert!(result.contains(".some-item:nth-child(2n+1)"));
        assert!(!result.contains("canvas"));
    }
}
