use import_modules::import_modules;

#[cfg(test)]
import_modules!("tests/priv_modules", "^((?!mod.rs).)*$");

#[cfg(test)]
#[test]
fn test_import_pub_modules() {
    assert_eq!(a::test(), "The modules are imported!");
}
