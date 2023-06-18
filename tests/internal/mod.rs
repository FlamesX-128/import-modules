use import_modules::import_pub_modules;

import_pub_modules!("tests/internal/", "^((?!mod.rs).)*$");

#[test]
fn test() {
    sub_one::printer::printer();
}