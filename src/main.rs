mod core;
use crate::core::apps::parser::{parse_data};

// Main function that run parser
fn main()
{
    println!("This is example of parser it will be print all .desktop file and their icons!");
    let apps = parse_data();
    for entry in apps {
        println!("App {}, icon {:?}, comment {}, exec {}, type {}", entry.name, entry.icon_path, entry.description, entry.exec, entry.type_file);
    }
}
