
#![feature(test)]
extern crate haml;
extern crate test;
use test::Bencher;

#[bench]
fn basic_parsing(b: &mut Bencher) {
    let haml = include_str!("inputs/01_basic.haml");
    b.iter(|| {
        haml::to_html(haml);
    });
}
