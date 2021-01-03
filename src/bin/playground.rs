

fn main () {
    let test = "test";
    let bits = test.as_bytes();
    let result = String::from_utf8(bits.to_vec()).unwrap();

    println!("{}", result);
}