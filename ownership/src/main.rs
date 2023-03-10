// fn main() {
//     let passwd = String::from("Hello, world!");

//     // Introducing & references to allow us to pick the 
//     // value but not acquire ownership of it
//     // The action of creating a reference is what is called borrowing
//     let passwd_length:usize = calculate_length(&passwd);

//     println!("Password length: {passwd_length}");
// }

// fn calculate_length(str: &String) -> usize {
//     str.len()
// }

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let mut s = String::from("Hello");

    change(&mut s);

    println!("{s}")

    // One big restriction with mutables is that you can have only one 
    // mutable reference to a value, anything more than one leadeth to
    // E0499
}