#[macro_use] extern crate nickel;
use nickel::Nickel;
use std::fs::File;
use std::io::{prelude::*, Bytes};
use std::prelude::v1::*;
use std::io;
use std::io::stdin;
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
fn log_something() -> io::Result<()>{
    let mut file_name = String::new();
    stdin()
        .read_line(&mut file_name)
        .expect("Failed to real line!");
    let file_name = file_name.trim();
    // 添加.txt扩展名
    let file_name = format!("{}.txt", file_name);
    let mut f = File::create(&file_name)?;
    let local: DateTime<Local> = Local::now();
    println!("{}", local);
    let bytes = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    let bytes = bytes.as_bytes();
    f.write_all(bytes)?;
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
    match log_something() {
        Ok(..) => println!("File created!"),
        Err(..) => println!("Error: could not create file.")
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