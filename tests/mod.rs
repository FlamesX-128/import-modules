use import_modules::import;

import!({
    "directory": "tests"
});

#[test]
fn test() {
    let data = import!({
        "directory": "tests/math/",
        "inter_process": "math::{}::handler,",
        "post_process": "vec![{}]"
    });

    // The first function is the sum of two numbers.
    assert!(data[0](16, 4) == 20);

    // The second function is the division of two numbers.
    assert!(data[1](16, 4) == 4);
}

