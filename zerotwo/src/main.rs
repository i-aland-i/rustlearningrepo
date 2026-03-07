fn main() {
    println!("App Started!");
    println!("");

    let y = 20;
    let x = 5;

    println!("{}", sum(x, y));

    println!("");
    println!("Finished!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
