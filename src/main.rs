fn main() {
    std::env::args().for_each(|arg| {
        println!("{}", arg);
    });
}
