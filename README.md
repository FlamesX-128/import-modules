# import-modules

## Description

`import-modules` is a library based on [require-all](https://www.npmjs.com/package/require-all)

## Examples

This is based on the [import-modules](https://github.com/FlamesX-128/import-modules) test directory.

### Inter Process and Post Process

Intermediary processes allow you to manipulate how the modules are processed.

#### Input

```rust, ignore
use import_macro::import;

// The {} is replaced by the module.
let functions = import!({
    "directory": "tests/math/",
    "inter_process": "math::{}::handler,",
    "post_process": "vec![{}]"
});
```

#### Output

```rust, ignore
let functions = vec![
    math::add::handler,
    math::sub::handler,
];
```

### Module

Similar to intermediate process, this imports it by default as Rust module.

#### Input

```rust, ignore
use import_macro::import;

import!({
    "directory": "tests",
});
```

#### Output

```rust, ignore
mod math;
```

## Authors

- [FlamesX-128](https://github.com/FlamesX-128/)

## Change log

### 1.0.0 - The new import

All macros have been eliminated. Instead, use the new `import` macro, which employs JSON configuration. The following valid configuration is available:

```rust, ignore
struct Config {
    /// The directory where the modules are located.
    pub directory: String,

    /// Recursive search for modules.
    pub recursive: bool,

    /// Intermediary processes allow you to manipulate how the modules are processed.
    /// Intermediary processes replaces {} with the module.
    pub inter_process: Option<String>,

    /// Post processes allow you to manipulate how the modules are processed.
    /// Post_process replaces the with the module.
    pub post_process: Option<String>,

    /// Similar to intermediate process, this imports it by default as a public Rust module.
    pub public_module: Option<bool>,

    /// Similar to intermediate process, this imports it by default as a Rust module.
    ///
    /// Default if you don't use inter_process: true
    /// Default if you use public_module: true
    pub module: Option<bool>,

    /// Exclude files from the module list.
    ///
    /// Default: ["lib.rs", "main.rs", "mod.rs"]
    pub exclude_files: Vec<Regex>,

    /// Exclude directories from the module list.
    ///
    /// Default: [".git", ".github", "lib", "src" "tests", "target"]
    /// Note: Only if ends by directory separator.
    pub exclude_dirs: Vec<Regex>,

    /// Include files from the module list.
    pub include_files: Vec<Regex>,

    /// Include directories from the module list.
    pub include_dirs: Vec<Regex>,
}
```

## License

This project is licensed under the [MIT](https://github.com/FlamesX-128/import-modules/blob/main/LICENSE) license.

