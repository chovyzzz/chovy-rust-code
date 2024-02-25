#[macro_use] extern crate nickel;
use nickel::Nickel;
use std::fs::File;

use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
// use std::collections::HashMap;
// use nickel::{Nickel, HttpRouter};

fn say_hello() -> &'static str {
    "Hello dear world!"
}

fn main() {
    let mut server = Nickel::new();
    match File::create("foo.txt") {
        Ok(..) => println!("File created!"),
        Err(..) => println!("Error: could not create file.")
    }
    server.utilize(router! {
        get "**" => |_req, _res| {
            say_hello()
        }
    });

    server.listen("127.0.0.1:6767");
}

// fn main() {
//     let mut server = Nickel::new();

//     server.get("/", middleware! { |_, response|
//         let mut data = HashMap::new();
//         data.insert("name", "user");
//         return response.render("site/assets/template.tpl", &data);
//     });

//     server.listen("127.0.0.1:6767");
// } 