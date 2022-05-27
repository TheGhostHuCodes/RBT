fn main() {
    #[cfg(feature = "english")]
    let hello = || println!("Hello, world!");
    #[cfg(feature = "french")]
    let hello = || println!("Bonjour le monde");
    hello();
}
