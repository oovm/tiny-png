#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn main() {
    let before = include_bytes!("../iphone.test.png");
    let mut out = optimize_png(before).unwrap();
    println!("before: {}", out.before);
    println!("after: {}", out.after);
    println!("Reduce {:+.2}%", out.reduce);
}
