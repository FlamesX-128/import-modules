/*use import_modules::import;

import!(
    "tests/" where "" use as mod
);*/

use import_modules::import;

#[test]
fn test() {
    /*let current_file = file!();
    let current_directory = env!("PWD"); // This will retrieve the current working directory

    println!("Current File: {}", current_file);
    println!("Current Directory: {}", current_directory);*/

    // {module1{module2{module3,module4{module5,module6}},module7}}
    /*import!(
        {module1{module2{module3,module4{module5,module6}},module7}}
    );*/

    import!(
        use models::{a::{*}, b::{*}}
    );
}

