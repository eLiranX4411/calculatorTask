fn main() {

    let string1 = String::from("Rust, ");
    let string2 = String::from("Art");

    fn concatenate_strings(string1: &str, string2: &str) -> String {
        let mut result = String::new();
        result.push_str(string1);
        result.push_str(string2);
        result
    }

    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_string); 
}