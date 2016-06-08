#![feature(test)]

extern crate test;
extern crate syntect;
use test::Bencher;

use syntect::package_set::PackageSet;

#[bench]
fn bench_load_syntaxes(b: &mut Bencher) {
    b.iter(|| {
        let mut ps = PackageSet::new();
        ps.load_syntaxes("testdata/Packages", false).unwrap();
    });
}