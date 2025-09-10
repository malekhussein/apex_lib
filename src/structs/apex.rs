pub fn apex(input: &str) {
    println!("label: {}", input);
}


pub fn apex_multiple(inputs: &[&str]) {
    for word in inputs {
        println!("{}", word);
    }
}

pub fn apex_upper(inputs: &[&str]) {
    for word in inputs {
        println!("{}", word.to_uppercase());
    }
}

pub fn apex_lowwer(inputs: &[&str]) {
    for word in inputs {
        println!("{}", word.to_lowercase());
    }
}
