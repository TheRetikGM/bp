use music_sheet_gen::ls::l_rule::*;

fn main() {
    let r = CSSLRule::from("a->b%1/2").unwrap();
    r.matches("abc");
    println!("Hello, world!");
}
