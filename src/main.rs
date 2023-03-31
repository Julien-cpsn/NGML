use std::fs::File;
use std::io::{Read, Write};
use fancy_regex::Regex;

fn main() {
    const CSS_KEYWORDS: [&str; 2] = ["color", "font-size"];

    let ngml_path = std::env::args().nth(1).expect("No NGML input file path given");
    let html_path = std::env::args().nth(2).expect("No HTML output file path given");

    println!("NGML input :{ngml_path}, HTML output: {html_path}");

    let mut ngml_file = File::open(ngml_path).expect("The given NGML path was not found");
    let mut html_file = File::create(html_path).expect("The given HTML path was not found");

    /* Get content */

    let mut ngml_content= String::new();
    ngml_file.read_to_string(&mut ngml_content).expect("Cannot transform file content to string").to_string();

    /* Compile */

    // generic tag attributes regex
    let tag_regex = Regex::new(r#"<(\w+) (.*)>(.*)</(\w+)>"#).unwrap();
    let tag_attributes_regex = Regex::new(r#"(([\w-]+)=["']?((?:.(?!["']\s+(?:\S+)=|\s*/?[>"']))+.)["']|\w+)"#).unwrap();

    // text tag
    let h_regex = Regex::new(r"h\d").unwrap();

    let iterator_ngml_content = ngml_content.clone();

    for cap in tag_regex.captures_iter(iterator_ngml_content.as_str())  {
        let capture = cap.unwrap();

        // Creates the beginning of the new html tag
        let mut new_tag = String::from("<");
        let mut new_tag_name = String::new();

        /* --- ATTRIBUTES --- */
        // Count tag CSS attributes
        let mut css_attribute_count = 0;
        let mut style = String::from("style=\"");

        // Manages attributes
        for attribute_cap in tag_attributes_regex.captures_iter(&capture[2]) {
            let attribute_capture = attribute_cap.unwrap();

            if attribute_capture.get(2).is_some() && CSS_KEYWORDS.contains(&&attribute_capture[2]) {
                style = style + &attribute_capture[2] + ": " + &attribute_capture[3] + ";";
                css_attribute_count += 1;
            }
            else {
                // Adapts the html tag
                match &capture[1] {
                    "text" => {
                        // Handles h1 .. h6 case
                        if h_regex.is_match(&attribute_capture[1]).unwrap() {
                            new_tag_name = attribute_capture[1].to_string();
                        }
                        else if attribute_capture[1].eq("reverse") {
                            new_tag_name = "bdo".to_string();
                        }
                    }
                    _ => {
                        new_tag_name = capture[1].to_string();
                    }
                }
            }
        }

        // Appends tag name to tag
        new_tag = new_tag + &new_tag_name;

        // Do not append the style if there was no valid CSS attribute
        if css_attribute_count > 0 {
            new_tag = new_tag + " " + &style + "\"";
        }
        /* ----- */

        new_tag = new_tag + ">" + &capture[3] + "</" + &new_tag_name + ">";
        ngml_content = ngml_content.replace(&capture[0], new_tag.as_str());
    }

    println!();

    /* ----- */

    html_file.write_all(ngml_content.as_bytes()).expect("Cannot write in HTML file");
}
