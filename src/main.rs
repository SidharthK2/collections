fn main() {
    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let s2 = "world!";
    s1.push_str(&s2[..]);
    let s3 = format!("{s1}-{s2}");
    println!("s1 {} s3 {} ", s1, s3);
}
