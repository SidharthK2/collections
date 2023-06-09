fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("{}", third);
    let third = v.get(4);
    match third {
        Some(val) => println!("{}", val),
        None => println!("err"),
    }
}
