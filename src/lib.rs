use std::{env,path::{self, Path}};

use fancy_regex::Regex;
use proc_macro::TokenStream;
use serde::Deserialize;

// - - -

#[derive(Clone, Debug, Deserialize)]
struct JSONConfig {
    pub directory: String,
    pub recursive: Option<bool>,

    pub inter_process: Option<String>,
    pub post_process: Option<String>,

    pub public_module: Option<bool>,
    pub module: Option<bool>,

    #[serde(default = "Vec::new")]
    pub exclude_files: Vec<String>,

    #[serde(default = "Vec::new")]
    pub exclude_dirs: Vec<String>,

    #[serde(default = "Vec::new")]
    pub include_files: Vec<String>,

    #[serde(default = "Vec::new")]
    pub include_dirs: Vec<String>,
}

// - - -

struct Config {
    /// The directory where the modules are located.
    pub directory: String,

    /// Recursive search for modules.
    pub recursive: bool,

    /// Intermediary processes allow you to manipulate how the modules are processed.
    /// Intermediary processes replaces {} with the module.
    pub inter_process: Option<String>,

    /// Post processes allow you to manipulate how the modules are processed.
    /// Post_process replaces {} with the module.
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

// - - -

fn files_from(path: &str, config: &Config) -> Vec<String> {
    let mut modules = Vec::new();

    for path in std::fs::read_dir(path).unwrap() {
        let path = path.unwrap().path();

        if path.is_dir() {
            let path = path.to_str().unwrap();

            if config.include_dirs.iter().any(|pattern| {
                pattern.is_match(&path).expect(&format!(
                    "Failed to match include_dirs pattern: {}.",
                    pattern
                ))
            }) {
                continue;
            }

            if config.exclude_dirs.iter().any(|pattern| {
                pattern.is_match(&path).expect(&format!(
                    "Failed to match exclude_dirs pattern: {}.",
                    pattern
                ))
            }) {
                continue;
            }

            if config.recursive {
                modules.extend(files_from(path, config));
            }

            modules.push(path.to_string());
        } else {
            let path = path.to_str().unwrap();

            if config.include_files.iter().any(|pattern| {
                !pattern.is_match(&path).expect(&format!(
                    "Failed to match include_files pattern: {}.",
                    pattern
                ))
            }) {
                continue;
            }

            if config.exclude_files.iter().any(|pattern| {
                pattern.is_match(&path).expect(&format!(
                    "Failed to match exclude_files pattern: {}.",
                    pattern
                ))
            }) {
                continue;
            }

            modules.push(path.to_string());
        }
    }

    modules
}

// - - -

fn parse_modules(base: &str, config: &Config, data: Vec<String>) -> Vec<String> {
    let mut modules = Vec::new();

    for module in data {
        let module = module.replace(base, "");

        // Maybe conflict with escape separators.
        let module = module.replace(path::MAIN_SEPARATOR_STR, "::");
        let module = module.replace(".rs", "");

        if module.len() == 0 {
            continue;
        }

        let mut module = module;

        if let Some(inter) = &config.inter_process {
            if let Some(state) = config.public_module {
                if state {
                    panic!("Cannot use public_module and process_as at the same time.");
                }

            }

            if let Some(state) = config.module {
                if state {
                    panic!("Cannot use module and process_as at the same time.");
                }
            }

            module = inter.replace("{}", &module);
        } else {
            let pub_module_state = config.public_module.unwrap_or(false);
            let module_state = config.module.unwrap_or(true);

            if pub_module_state && module_state {
                module.insert_str(0, "pub mod ");
            } else if module_state {
                module.insert_str(0, "mod ");
            } else {
                panic!("You must set the module to true to use public_module.");
            }

            module.push(';');
        }

        modules.push(module);
    }

    modules
}

// - - -

/// Returns a list of modules processed based on a configuration.
///
/// # Arguments
///
/// * `input` - A JSON containing the configuration.
///
/// # Example
///
/// This is based on the [import-modules](https://github.com/FlamesX-128/import-modules) test directory.
///
/// ## Inter Process and Post Process
///
/// Intermediary processes allow you to manipulate how the modules are processed.
///
/// ### Input
///
/// ```rust, ignore
/// use import_macro::import;
///
/// // The {} is replaced by the module.
/// let functions = import!({
///     "directory": "tests/math/",
///     "inter_process": "math::{}::handler,",
///     "post_process": "vec![{}]"
/// });
/// ```
///
/// ### Output
///
/// ```rust, ignore
/// let functions = vec![
///     math::add::handler,
///     math::sub::handler,
/// ];
/// ```
///
/// ## Module
///
/// Similar to intermediate process, this imports it by default as Rust module.
///
/// ### Input
///
/// ```rust, ignore
/// use import_macro::import;
///
/// import!({
///     "directory": "tests",
/// });
/// ```
///
/// ### Output
///
/// ```rust, ignore
/// mod math;
/// ```
///

#[proc_macro]
pub fn import(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let mut config = serde_json::from_str::<JSONConfig>(&input)
        .expect("Failed to parse config.");

    let exc = match env::consts::OS {
        "windows" => r"(\\)?$",
        _ => r"(/)?$",
    };

    // default files and directories excluded.
    config.exclude_files.push(r"(lib|main|mod).rs$".to_string());
    config.exclude_dirs
        .push(("(.git|.github|lib|src|target|tests)".to_string()) + exc);

    config.directory = config.directory.replace("/", path::MAIN_SEPARATOR_STR);

    if !config.directory.ends_with(path::MAIN_SEPARATOR) {
        config.directory.push(path::MAIN_SEPARATOR);
    }

    // Build regexes.
    let config = Config {
        directory: config.directory,
        recursive: config.recursive.unwrap_or(false),

        inter_process: config.inter_process,
        post_process: config.post_process,

        public_module: config.public_module,
        module: config.module,

        exclude_files: config
            .exclude_files.iter()
            .map(|pattern| {
                Regex::new(pattern).expect(&format!(
                    "Failed to parse exclude_files pattern: {}.",
                    pattern
                ))
            })
            .collect(),

        exclude_dirs: config
            .exclude_dirs.iter()
            .map(|pattern| {
                Regex::new(pattern).expect(&format!(
                    "Failed to parse exclude_dirs pattern: {}.",
                    pattern
                ))
            })
            .collect(),

        include_files: config
            .include_files.iter()
            .map(|pattern| {
                Regex::new(pattern).expect(&format!(
                    "Failed to parse include_files pattern: {}.",
                    pattern
                ))
            })
            .collect(),

        include_dirs: config
            .include_dirs.iter()
            .map(|pattern| {
                Regex::new(pattern).expect(&format!(
                    "Failed to parse include_dirs pattern: {}.",
                    pattern
                ))
            })
            .collect(),
    };

    // This is the directory of the manifest.
    let manifiest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to get CARGO_MANIFEST_DIR env variable.");

    let path = Path::new(&manifiest_dir)
        .join(&config.directory);

    let path = path.to_str()
        .unwrap();

    let output = files_from(&path, &config);
    let output = parse_modules(&path, &config, output);

    if let Some(post) = &config.post_process {
        post.replace("{}", &output.join("")).parse().unwrap()
    } else {
        output.join("").parse().unwrap()
    }

}

