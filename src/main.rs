fn main() {
    if std::env::args().nth(1) == Some("-n".into()) {
        println!("{}", ulid::Ulid::new().0);
    } else {
        println!("{}", ulid::Ulid::new());
    }
}
