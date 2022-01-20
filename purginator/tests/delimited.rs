mod helpers;

#[cfg(test)]
pub mod tests {
    use crate::helpers;

    fn test() -> String {
        helpers::run_test("delimited/delimited")
    }

    #[test]
    fn it_keeps_h1() {
        let result = test();
        assert!(result.contains("h1"));
    }

    #[test]
    fn it_removes_p() {
        let result = test();
        assert!(!result.contains('p'));
    }

    #[test]
    fn it_removes_unused_class_name() {
        let result = test();
        assert!(!result.contains(".unused-class-name"));
    }
}
