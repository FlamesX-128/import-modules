use import_modules::{import_modules, import_scope_modules};

import_modules!("tests/scope_modules/", "^((?!mod.rs).)*$");

#[cfg(test)]
#[test]
fn test_import_pub_modules() {
    import_scope_modules!("tests/scope_modules/", "^((?!mod.rs).)*$", "test()");
}
