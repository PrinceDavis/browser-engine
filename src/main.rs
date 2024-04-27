mod dom;
mod html_parser;
mod css;
mod css_parser;

use std::iter::Peekable;
use std::str::Chars;

fn main() {
    let  mychars = "helhhfhfe89hea8hha".chars().peekable();
    let me = mychars.peek();

    println!("{:?}", mychars.peek());
}