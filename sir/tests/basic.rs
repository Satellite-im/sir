use sir::{css, DEFAULT_CSS_COLLECTION};

#[test]
fn sanity_check() {
    assert_eq!(DEFAULT_CSS_COLLECTION.get_css().len(), 0);
    let _class: &str = css!("color: red;");
    assert!(DEFAULT_CSS_COLLECTION.get_css().len() > 0);
}
