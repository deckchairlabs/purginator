mod helpers;

#[cfg(test)]
pub mod tests {
    use crate::helpers;

    fn test() -> String {
        helpers::run_test("media-queries/media_queries")
    }

    #[test]
    fn it_finds_media_class() {
        let result = test();
        assert!(result.contains(".media-class"));
    }

    // #[test]
    // fn it_finds_alone() {
    //     let result = test();
    //     assert!(result.contains(".alone"));
    // }

    // #[test]
    // fn it_finds_id_in_media() {
    //     let result = test();
    //     assert!(result.contains("#id-in-media"));
    // }

    // #[test]
    // fn it_finds_body() {
    //     let result = test();
    //     assert!(result.contains("body"));
    // }

    // #[test]
    // fn it_removes_unused_class() {
    //     let result = test();
    //     assert!(!result.contains(".unused-class"));
    // }

    // #[test]
    // fn it_removes_the_empty_media_query() {
    //     let result = test();
    //     assert!(!result.contains("66666px"));
    // }
}
