use sir::{css, DEFAULT_CSS_COLLECTION};

#[test]
fn registered_once() {
    example_component();
    let content_after_first = DEFAULT_CSS_COLLECTION.get_css();

    example_component();
    let content_after_second = DEFAULT_CSS_COLLECTION.get_css();

    assert_eq!(content_after_first, content_after_second);
}

fn example_component() {
    let _class = css!("text-decoration: underline;");
}
