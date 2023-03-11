
fn main() {
    let name: String = String::from("Magere");

    let nombre = add_first_name(&name);

    println!("{}", nombre);
}


fn add_first_name(name: &String) -> String {
    let mut first_name = String::from("Lwanda");

    first_name.push_str(" ");
    first_name.push_str(&name);

    first_name
}
