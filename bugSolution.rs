fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 42; // Safe and proper way to modify vector elements
    println!("{:?}", v);
}