#[macro_use] 

extern crate lazy_static;
extern crate regex;
extern crate rustc_serialize;

use regex::Regex;
use std::fs;
use rustc_serialize::json::Json;

// #[no_mangle]
pub extern "Rust" fn main() {

    lazy_static! {
        static ref RE: Regex = Regex::new(r#"class[ \t]*=[ \t]*"[^"]+""#).unwrap();
    }

    let mut content = fs::read_to_string("data.txt")
        .expect("Something went wrong reading the file");

    let data = fs::read_to_string("data.json").expect("Something went wrong reading the file");

    let json_string = Json::from_str(&data).unwrap();
    let data_json = json_string.as_object().unwrap();
    let regex_captured_content = RE.captures_iter(&mut content);

    for class_attribute in regex_captured_content {
        let attribute = &class_attribute[0].replace(r#"class=""#, "").replace(r#"""#, "");
        let inline_class_attribute: Vec<&str> = attribute.split(" ").collect();
        let mut minified_classes_in_attribute = vec![]; 
        let mut _lookup_classname;

        for class_name in inline_class_attribute {

            if data_json.get(class_name) == None {
                minified_classes_in_attribute.push(class_name);
            } else {
                _lookup_classname = data_json[class_name].to_string().by_ref();
                // println!("{:?}", &*lookup_classname);
                // add_class(&mut &*lookup_classname.clone(), &mut minified_classes_in_attribute);
                minified_classes_in_attribute.push("new");
            }

            // if data_json.get(class_name) == None { println!("{:?}", class_name) } else { println!("{:?}", data_json[class_name]) };
        }

        let new_attribute = r#"class=""#.to_string() + &minified_classes_in_attribute.join(" ") + r"'";

        // content = content.replace(&class_attribute[0], &new_attribute);
        // change_content(&mut content,&mut &class_attribute[0],&mut &*new_attribute);
        println!("{:?}", new_attribute);
    }
}

// fn add_class(lookup_classname: &mut &str, minified_classes_in_attribute: &mut Vec<&str>, ) {
//     minified_classes_in_attribute.push(lookup_classname);
//     println!("{:?}", minified_classes_in_attribute);
//     // println!("{:?}", lookup_classname);
// }

// fn change_content(content: &mut String, class_attribute: &mut &str, new_attribute: &mut &str) {
//     *content = content.replace("hello", "new");
// }


