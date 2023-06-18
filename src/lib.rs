use std::path;

extern crate proc_macro;

struct ModuleConfig {
    pub pub_keyword: bool,
    pub mod_keyword: bool,

    pub directory: String,
    pub entry: String,
}

fn parse_modules(config: ModuleConfig, modules: Vec<String>) -> Vec<String> {
    let mut parsed_modules = Vec::new();

    for module in modules {
        let mut parsed_module = String::new();

        let module = module.replace(&config.directory, "");
        let module = module.replace(".rs", "");

        let module = module.replace(path::MAIN_SEPARATOR_STR, "::");

        if module.len() == 0 {
            continue;
        }

        if config.pub_keyword == true {
            parsed_module.push_str("pub ");
        }

        if config.mod_keyword == true {
            parsed_module.push_str("mod ");
        }

        parsed_module.push_str(&module);

        if config.entry.len() > 0 {
            parsed_module.push_str("::");
            parsed_module.push_str(&config.entry);
        }

        parsed_module.push_str(";");

        parsed_modules.push(parsed_module);
    }

    parsed_modules
}

fn find_modules(
    directory: &String,
    pattern: &fancy_regex::Regex,
) -> Result<Vec<String>, fancy_regex::Error> {
    let mut modules = Vec::new();

    for path in std::fs::read_dir(directory).unwrap() {
        let path = path.unwrap().path();

        if path.is_dir() {
            continue;
        }

        let path = path.to_str().unwrap();

        if pattern.is_match(path).unwrap() {
            modules.push(path.to_string());
        }
    }

    Ok(modules)
}

fn parse_parameters(input: String) -> Vec<String> {
    let mut parameters = Vec::new();

    let mut parameter = String::new();
    let mut is_string = false;

    for c in input.chars() {
        if c == ',' && is_string == false {
            parameters.push(parameter.clone());
            parameter.clear();
        } else if c == ' ' && is_string == false {
            continue;
        } else if c == '"' {
            is_string = !is_string;
        } else {
            parameter.push(c);
        }
    }

    parameters.push(parameter);
    parameters
}

fn module_handler(
    pub_keyword: bool, mod_keyword: bool, mut directory: String,
    pattern: String, entry: String,
)
    -> proc_macro::TokenStream
{
    if directory.ends_with("/") == false {
        directory.push_str(path::MAIN_SEPARATOR_STR);
    }

    let pattern = fancy_regex::Regex::new(&pattern).unwrap();

    let modules = find_modules(&directory, &pattern).unwrap();

    let module_config = ModuleConfig {
        pub_keyword, mod_keyword,
        directory, entry,
    };

    let modules = parse_modules(module_config, modules);
    let output = modules.join("\n");

    output.parse().unwrap()
}

#[proc_macro]
pub fn import_pub_modules(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();

    let parameters = parse_parameters(input);

    if parameters.len() != 2 {
        panic!("Invalid number of parameters: {}", parameters.len());
    }

    module_handler(
        false, true, parameters[0].clone(), parameters[1].clone(), "".to_string(),
    )
}

#[proc_macro]
pub fn import_modules(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();

    let parameters = parse_parameters(input);

    if parameters.len() != 2 {
        panic!("Invalid number of parameters: {}", parameters.len());
    }

    module_handler(
        false, true, parameters[0].clone(), parameters[1].clone(), "".to_string(),
    )
}

#[proc_macro]
pub fn import_scope_modules(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();

    let parameters = parse_parameters(input);

    if parameters.len() != 3 {
        panic!("Invalid number of parameters: {}", parameters.len());
    }

    module_handler(
        false, false, parameters[0].clone(), parameters[1].clone(), parameters[2].clone(),
    )
}
