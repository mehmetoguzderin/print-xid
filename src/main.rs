use std::env;
use std::fs;

use unicode_xid::UnicodeXID;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        println!("No file supplied");
        return;
    }
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for (i, ch) in contents.chars().enumerate() {
        if i == 0 {
            if !(UnicodeXID::is_xid_start(ch) || ch == '_') {
                println!("Invalid start");
                return;
            }
        } else {
            if !(UnicodeXID::is_xid_continue(ch)) {
                println!("Invalid continue");
                return;
            }
        }
    }
    println!("With XID-verified text:\n{}", contents);
}
