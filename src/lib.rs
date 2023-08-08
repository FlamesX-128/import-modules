use proc_macro::TokenStream;

#[derive(Clone, Debug)]
struct Module {
    pub submodules: Vec<Module>,
    pub name: String,
}

impl Module {
    pub fn new() -> Self {
        Self {
            submodules: Vec::new(),
            name: String::new(),
        }
    }
}

fn parse_modules(input: &str) -> Module {
    let stack = &mut Vec::new();

    let mut module = Module::new();

    for c in input.chars() {
        match c {
            '{' => {
                stack.push(module);
                module = Module::new();
            }
            '}' => {
                let mut parent = stack.pop().unwrap();
                parent.submodules.push(module);

                module = parent;
            }
            ',' => {
                let parent = stack.last_mut().unwrap();
                parent.submodules.push(module);

                module = Module::new();
            }
            '&' => {
                //stack.push(module.clone());

                let mut parent = stack.pop().unwrap();
                parent.submodules.push(module);

                module = parent;
            }
            _ => {
                module.name.push(c);
            }
        }
    }

    module
}

fn generate_import(path: String, module: Module) -> Vec<String> {
    let mut current = String::new();
    current.push_str(&path);
    current.push_str(&module.name);

    let mut data = Vec::new();
    data.push(current.clone());

    for submodule in module.submodules {
        data.append(
            &mut generate_import(current.clone() + "::", submodule)
        );
    }

   data 
}

#[proc_macro]
pub fn import(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    
    let args = input.trim().split_whitespace();

    let (prefix, path) = match args.clone().nth(0).unwrap_or_default() {
        "crate" | "mod" | "self" | "use" => {
            let prefix = args.clone().nth(0).unwrap_or_default();
            let path = args.clone().skip(1).collect::<Vec<_>>().join("");

            (prefix, path)
        }
        _ => {
            let path = args.clone().collect::<Vec<_>>().join("");

            ("", path)
        }
    };

    let path = path.replace("::", "");
    
    let module = parse_modules(&path);
    let imports = generate_import(String::new(), module.clone());

    println!("prefix: {}", prefix);
    println!("path: {}", path);
    println!("path: {:#?}", module);
    println!("{:#?}", imports);

    "".parse().unwrap()
}

