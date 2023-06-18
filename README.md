# import-modules

## Description

import-modules is a crate that enhances the ease of importing modules using regex patterns. It is based on the npm package require-all.

## Usage

```rs
use import_modules::import_pub_modules;

// import_pub_modules!(directory: String, pattern: Regex)
import_pub_modules!("tests/pub_modules/", "^((?!mod.rs).)*$");
// Equivalent to:
// pub mod a.rs;
//

```

## Authors

- [FlamesX-128](https://github.com/FlamesX-128/)

## Version History

- 0.1.3
    + The problem fixed: String parameters now support escaping characters with backslashes, resolving compatibility with Windows.

- 0.1.2
    + The problem fixed: The macro import_pub_modules did not make the modules public.
    + The problem fixed: Directories are already included as modules.

- 0.1.1
    + The problem fixed: If the last slash was not included in the directory path, it would cause an error.

- 0.1
    + First release

## License

This project is licensed under the [MIT](https://github.com/FlamesX-128/import-modules/blob/main/LICENSE) license.
