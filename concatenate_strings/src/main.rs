fn main() {
    let first_string = String::from("Hello");
    let second_string = String::from(" world");

    let combo = concatenate_strings(&first_string, &second_string);
    println!("Combined string is {}", combo);
}

fn concatenate_strings(s1: &String, s2: &String) -> String {
    let mut combo = s1.clone();
    combo.push_str(s2);
    combo
}
