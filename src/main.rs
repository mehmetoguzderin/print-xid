use std::env;
use std::fs;

use unicode_xid::UnicodeXID;

#[test]
fn trojan_source() {
    let mut vulnerable = false;
    let paper_codepoints =
        "\u{202A}\u{202B}\u{202D}\u{202E}\u{2066}\u{2067}\u{2068}\u{202C}\u{2069}";
    for ch in paper_codepoints.chars() {
        if UnicodeXID::is_xid_start(ch) || UnicodeXID::is_xid_continue(ch) {
            vulnerable = true;
        }
    }
    assert_eq!(vulnerable, false);
}

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
