fn main() {
    my_function().await;
}

async fn my_function() {
    println!("I am an async function");
    let s1: String = read_from_database().await;
    println!("First result: {s1}");
    let s2: String = read_from_database().await;
    println!("Second result: {s2}");

}

async fn read_from_database() -> String {
    "DB Result".to_owned()
}
