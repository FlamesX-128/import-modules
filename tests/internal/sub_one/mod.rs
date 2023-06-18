use import_modules::import_pub_modules;

import_pub_modules!("tests/internal/sub_one/", "^((?!mod.rs).)*$");
