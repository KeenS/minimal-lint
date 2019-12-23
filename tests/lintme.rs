#![allow(deprecated)]
#![feature(plugin)]
#![plugin(minimal_lint)]

fn lintme() {}

#[test]
fn test() {
    lintme()
}
