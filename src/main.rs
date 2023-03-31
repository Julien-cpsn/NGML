use std::fs::File;
use std::io::{Read, Write};
use regex::Regex;

enum CssAttribute {
    Color,
    FontSize,
}

fn main() {
    let ngml_path = std::env::args().nth(1).expect("No NGML input file path given");
    let html_path = std::env::args().nth(2).expect("No HTML output file path given");

    println!("NGML input :{ngml_path}, HTML output: {html_path}");

    let mut ngml_file = File::open(ngml_path).expect("The given NGML path was not found");
    let mut html_file = File::create(html_path).expect("The given HTML path was not found");

    /* Get content */

    let mut ngml_content= String::new();
    ngml_file.read_to_string(&mut ngml_content).expect("Cannot transform file content to string").to_string();

    /* Compile */

    println!("Content {ngml_content}");

    // text tag
    let text_h_regex = Regex::new(r#"<text (h\d)((\s([\w-]+)="(\w+)")*)>(.*)</text>"#).unwrap();
    let iterator_ngml_content = ngml_content.clone();

    for cap in text_h_regex.captures_iter(iterator_ngml_content.as_str())  {
        println!("{:?}", &cap);

        /*
        for key in (2..&cap.len() - 1).step_by(2) {
            println!("{}\t{}", &cap[key], &cap[key + 1])
        }*/

        let new_h_tag = "<".to_owned() + &cap[1] + ">" + &cap[cap.len() - 1] + "</" + &cap[1] + ">";
        ngml_content = ngml_content.replace(&cap[0], new_h_tag.as_str());
    }

    println!();
    println!("New content {ngml_content}");

    /* ----- */

    html_file.write_all(ngml_content.as_bytes()).expect("Cannot write in HTML file");
}
