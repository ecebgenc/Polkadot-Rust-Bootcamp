fn concatenate_strings(string1: &str, string2: &str) -> String {
    let mut result = String::new();
    result.push_str(string1);
    result.push_str(string2);
    result
}

fn main() {
    let string1 = String::from("Hello, all! ");
    // println!("{}", &string1);
    let string2 = String::from("This is the first assignment.");
    // println!("{}", &string2);
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_string);
}
