#[macro_use] extern crate nickel;
use nickel::Nickel;
use std::fs::File;
use std::io::prelude::*;
use std::prelude::v1::*;
use std::io;
#[macro_use]
extern crate std as std;
// use std::collections::HashMap;
// use nickel::{Nickel, HttpRouter};

fn say_hello() -> &'static str {
    "Hello dear world!"
}
fn log_something(filename: &'static str, string: &'static [u8; 12]) -> io::Result<()> {
    let mut f = File::create(filename)?;
    f.write_all(string)?;
    Ok(())
}
fn main() {
    let mut server = Nickel::new();
    log_something("log.txt", b"ITS DEADS!!!");
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