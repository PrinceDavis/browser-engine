use std::iter::Peekable;
use  std::str::Chars;

use crate::dom::{Node, ElementData, AttrMap, NodeType};


pub struct HtmlParser<'a> {
  chars: Peekable<Chars<'a>>,
  node_q: Vec<String>,
}