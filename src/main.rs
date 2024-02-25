#[macro_use] extern crate nickel;
use nickel::Nickel;
use std::fs::File;
use std::io::{prelude::*, Bytes};
use std::prelude::v1::*;
use std::io;
use std::io::stdin;
use std::fs::OpenOptions;
extern crate chrono;
#[macro_use]
extern crate std as std;
use chrono::*;
// use std::collections::HashMap;
// use nickel::{Nickel, HttpRouter};

fn say_hello() -> &'static str {
    "Hello dear world!"
}
//fn log_something(filename: &'static str, string: &'static [u8; 12]) -> io::Result<()> {
    fn formatted_time_entry() -> String {
        let local: DateTime<Local> = Local::now();
        let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
        formatted
    }
    
    fn record_entry_in_log(filename: &str, bytes: &[u8]) -> io::Result<()> {
        let mut file = OpenOptions::new().
                            append(true).
                            write(true).
                            create(true).
                            open(filename)?;
        file.write_all(bytes)?;
        Ok(())
    }
    
    fn log_time(filename: &'static str) -> io::Result<()> {
        let entry = formatted_time_entry();
        let bytes = entry.as_bytes();
    
        record_entry_in_log(filename, &bytes)?;
        Ok(())
    }
fn main() {
    let mut server = Nickel::new();
    // server.utilize(router! {
    //     get "**" => |_req, _res| {
    //         say_hello()
    //     }
    // });

    // server.listen("127.0.0.1:6767");
    match log_time("log.txt") {
        Ok(..) => println!("File created!"),
        Err(e) => println!("Error: {}", e)
    }
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