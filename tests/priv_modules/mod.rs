use import_modules::import_pub_modules;

#[cfg(test)]
import_pub_modules!("tests/priv_modules/", "^((?!mod.rs).)*$");

#[cfg(test)]
#[test]
fn test_import_pub_modules() {
    assert_eq!(a::test(), "The modules are imported!");
}
