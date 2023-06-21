use import_modules::import_pub_modules;

import_pub_modules!("tests/pub_modules/", "^((?!mod\\.rs).)*$");

#[test]
fn test_import_pub_modules() {
    assert_eq!(a::test(), "The public modules are imported!");
}
