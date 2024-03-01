fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    
    result.push_str(s1);
    result.push_str(s2);
    
    // Return the result string
    result
}

fn main() {
    // Initialize two String variables string1 and string2
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");

    // Call the concatenate_strings function with references to string1 and string2
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the result to the console
    println!("{}", concatenated_string);
}
