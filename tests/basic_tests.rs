extern crate haml;
mod common;

fn test(input_file: &str, expected_output_file: &str) {
    let haml = &common::read_file(input_file);
    let html = common::read_file(expected_output_file);
    let actual_html = haml::to_html(haml);
    assert_eq!(html, actual_html);
}

#[test]
fn test_basic_haml() {
    test("tests/inputs/basic.haml", "tests/outputs/basic.html");
}

#[test]
fn test_custom_elements() {
    test("tests/inputs/custom.haml", "tests/outputs/custom.html");
}

#[test]
fn test_ruby_attributes() {
    test(
        "tests/inputs/ruby_attributes.haml",
        "tests/outputs/ruby_attributes.html",
    );
}

#[test]
fn test_basic_01() {
    test("tests/inputs/01_basic.haml", "tests/outputs/01_basic.html")
}

#[test]
fn test_head_02() {
    test("tests/inputs/02_head.haml", "tests/outputs/02_head.html")
}

#[test]
fn test_comments_03() {
    test("tests/inputs/03_comments.haml", "tests/outputs/03_comments.html")
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;
    use test::Bencher;
    extern crate haml;

    #[bench]
    fn basic_01(b: &mut Bencher) {
        let haml = include_str!("inputs/01_basic.haml");
        b.iter(|| {
            haml::to_html(haml);
        });
    }

    #[bench]
    fn head_02(b: &mut Bencher) {
        let haml = include_str("inputs/02_head.haml");
        b.iter(|| {
            haml::to_html(haml)
        });
    }

}
