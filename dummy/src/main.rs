fn main() {
    let a = "hello";
    let b = a.to_string();
    let c = b.as_str();
    println!("{:p}, {:p}, {:p}", a, &*b, c);
}
