pub fn label(input: &str) {
    println!("label: {}", input);
}

#[macro_export]
macro_rules! labels {
    ($($word:expr),*) => {
        println!("labels: [{ }]", vec![$($word),*].join(", "));
    };
}


