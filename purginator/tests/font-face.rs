mod helpers;

#[cfg(test)]
pub mod tests {
    use crate::helpers;

    fn test() -> String {
        helpers::run_test("font-faces/font_face")
    }

    #[test]
    fn it_keeps_font_face_cerebri_bold() {
        let result = test();
        assert!(result.contains("src: url(../fonts/CerebriSans-Bold.eot?)"));
    }

    #[test]
    fn it_keeps_font_face_cerebri_sans() {
        let result = test();
        assert!(result.contains("src: url(../fonts/CerebriSans-Regular.eot?)"));
    }

    #[test]
    fn it_removes_font_face_other_font() {
        let result = test();
        assert!(!result.contains("src: url(xxx)"));
        assert!(!result.contains("OtherFont"));
    }
}
