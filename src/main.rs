use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = args.get(1).expect("Invalid url");
    let body = reqwest::blocking::get(url)
        .unwrap()
        .bytes()
        .unwrap()
        .to_vec();
    let model = feed_rs::parser::parse(&mut &body[..]).expect("Could not parse feed file");
    for entry in model.entries.iter() {
        if let Some(title) = &entry.title {
            print!("{} ", title.content);
        }
        for link in &entry.links {
            print!("{} ", link.href);
        }
        println!();
    }
}
