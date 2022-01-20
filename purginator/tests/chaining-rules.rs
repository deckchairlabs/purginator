mod helpers;

#[cfg(test)]
pub mod tests {
    use crate::helpers;

    fn test() -> String {
        helpers::run_test("chaining-rules/index")
    }

    #[test]
    fn it_keeps_parent1_selector() {
        let result = test();
        assert!(result.contains("parent1"));
    }

    #[test]
    fn it_removes_parent3_d33ef1_parent2() {
        let result = test();
        assert!(!result.contains("parent3"));
        assert!(!result.contains("d33ef1"));
        assert!(!result.contains("parent2"));
    }
}
