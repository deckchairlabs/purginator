mod helpers;

#[cfg(test)]
pub mod tests {
    use crate::helpers;

    fn test() -> String {
        helpers::run_test("media-queries/media_queries")
    }

    #[test]
    fn it_retains_used_selectors() {
        let result = test();
        assert!(result.contains(".media-class"));
        assert!(result.contains("body"));
    }

    #[test]
    fn it_removes_unused_selectors() {
        let result = test();
        assert!(!result.contains(".alone"));
        assert!(!result.contains("#id-in-media"));
        assert!(!result.contains(".unused-class"));
        assert!(!result.contains(".unused-class2"));
    }

    #[test]
    fn it_removes_the_empty_media_query() {
        let result = test();
        assert!(!result.contains("66666px"));
    }
}
