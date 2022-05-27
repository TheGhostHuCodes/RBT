#[derive(Debug)]
struct Parser<'a> {
    body: String,
    subtext: &'a str,
}

fn main() {
    let mut document = Parser {
        body: "Hello".to_string(),
        subtext: "",
    };
    document.subtext = &document.body;

    dbg!(&document);

    // This won't compile. We won't be able to move out of document because it
    // is a self-referential structure.
    // let b = document;
    // println!("{:?}", b);
}
