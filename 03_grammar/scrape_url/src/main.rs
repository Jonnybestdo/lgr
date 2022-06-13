use std::fs;

fn main() {
    let mut url = "https://www.rust-lang.org/";
    let mut output = "rust.md";

    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    match args.len() {
        2 => url = args[1],
        3 => {
            url = args[1];
            output = args[2];
        },
        _ => (),
    }

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}", output);
}
