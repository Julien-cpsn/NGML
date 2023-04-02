use std::fs::{OpenOptions};
use std::io::Write;
use std::path::Path;
use fancy_regex::Regex;
use html5ever::{LocalName};
use kuchiki::NodeData::{Element};
use kuchiki::{NodeRef};
use kuchiki::traits::TendrilSink;

fn main() {
    //const CSS_KEYWORDS: [&str; 2] = ["color", "font-size"];

    let ngml_path = std::env::args().nth(1).expect("No NGML input file path given");
    let html_path = std::env::args().nth(2).expect("No HTML output file path given");

    println!("NGML input :{ngml_path}, HTML output: {html_path}");

    let mut html_file =
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(html_path)
            .expect("The given HTML path was not found");

    let parser = kuchiki::parse_html();
    let parsed_file = parser.from_utf8().from_file(Path::new(&ngml_path)).unwrap();

    let final_tree = manage_node(parsed_file.first_child().unwrap());


    /* Comments */
    let mut final_content = final_tree.to_string();
    let final_content_iterator = final_content.clone();

    let comment_regex = Regex::new(r#"&lt;\*(?s)(.*?)\*&gt;"#).unwrap();
    for cap in comment_regex.captures_iter(final_content_iterator.as_str())  {
        let capture = cap.unwrap();

        let new_comment= "<!--".to_owned() + &capture[1] + "-->";
        final_content = final_content.replace(&capture[0], new_comment.as_str());
    }
    /* ----- */

    html_file.write_all(final_content.as_bytes()).expect("Cannot write in HTML file");
}


fn manage_node(node: NodeRef) -> NodeRef {
    let new_node: NodeRef;
    match &node.data() {
        Element(data) => {
            let mut new_node_data = data.clone();
            match new_node_data.name.local.to_string().as_str() {
                "text" => {
                    new_node_data.name.local = LocalName::from("p");

                    let h_regex = Regex::new(r"h\d").unwrap();
                    let attributes = new_node_data.attributes.clone().into_inner().map;
                    for (name, attribute) in attributes {
                        println!("{:?} {:?}", name.local.to_string(), attribute.value);

                        if h_regex.is_match(name.local.to_string().as_str()).unwrap() {
                            new_node_data.name.local = LocalName::from(name.local.to_string());
                            new_node_data.attributes.get_mut().map.remove(&name);
                        }
                    }
                    println!()
                }
                _ => {}
            }
            new_node = NodeRef::new(Element(new_node_data));
        }
        _ => {
            new_node = node.clone()
        }
    }


    for child in node.children() {
        new_node.append(manage_node(child));
    }

    return new_node;
}