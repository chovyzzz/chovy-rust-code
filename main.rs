use std::{io, num::ParseIntError};

fn come_on(num: i32){
    let mut num = num;
    while num > 0 {
        println!("{}come on!",num);
        num -= 1; 
    }

}
fn find_word(String: &str) -> &str{
    let byte = String.as_bytes();
    for (i,&item) in byte.iter().enumerate(){
        if item == b' '{
            return &String[0..i];
        }
    }
    return &String[..String.len()];
}
fn main() {
    println!("PLZ input a num");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let mut num: i32= input.trim().parse::<i32>().unwrap();
    come_on(num);
    let human = ["x","u","c","h","i"];
    for i in human.iter() {
        println!("{} come on",i);
    }
    println!("PLZ input a String");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2);
    println!("{}",find_word(&input2));
}
