# Change log

## 1.0.0 - The new import

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

## 0.1.5

The problem fixed: Misdeclared description in Cargo.toml.

## 0.1.4

Additional documentation has been added, including the proc macros `import_pub_modules`, `import_modules`, `import_scope_modules`.

## 0.1.3

The problem fixed: String parameters now support escaping characters with backslashes, resolving compatibility with Windows.

## 0.1.2

The problem fixed: The macro import_pub_modules did not make the modules public.
The problem fixed: Directories are already included as modules.

## 0.1.1
The problem fixed: If the last slash was not included in the directory path, it would cause an error.

## 0.1
First release

