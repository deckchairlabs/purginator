mod helpers;

#[cfg(test)]
pub mod tests {
    use crate::helpers;

    fn test() -> String {
        helpers::run_test("pseudo-elements/pseudo-elements")
    }

    #[test]
    fn it_finds_root_pseudo_elements() {
        let result = test();
        assert!(result.contains("::-webkit-file-upload-button"));
        assert!(result.contains("::grammar-error"));
        assert!(result.contains("::-webkit-datetime-edit-fields-wrapper"));
        assert!(result.contains("::-moz-focus-inner"));
        assert!(result.contains("::file-selector-button"));
    }

    #[test]
    fn it_finds_pseudo_elements_on_used_class() {
        let result = test();
        assert!(result.contains(".used::grammar-error"));
    }

    #[test]
    #[ignore]
    fn it_removes_pseudo_elements_on_unused_class() {
        let result = test();
        assert!(!result.contains(".unused::grammar-error"));
    }
}
