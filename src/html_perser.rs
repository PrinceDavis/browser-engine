use std::iter::Peekable;
use  std::str::Chars;

use crate::dom::{Node, ElementData, AttrMap, NodeType};


pub struct HtmlParser<'a> {
  chars: Peekable<Chars<'a>>,
  node_q: Vec<String>,
}

impl <'a> HtmlParser<'a> {
  pub fn new(full_html: &str) -> HtmlParser {
    HtmlParser{
      chars: full_html.chars().peekable(),
      node_q: Vec::new(),
    }
  }

  pub fn parse_nodes(&mut self)-> Vec<Node> {
    let mut nodes = Vec::new();

    while self.chars.peek().is_some() {
        self.consume_while(char::is_whitespace);
        if self.chars.peek().map_or(false, |c| *c == '<'){
          self.chars.next();
          if self.chars.peek().map_or(false, |c| *c == '/') {
            self.chars.next();
            self.consume_while(char::is_whitespace);

            let close_tag_name = self.consume_while(is_valid_tag_name);

            self.consume_while(|x| x != '>');
            self.chars.next();

            self.node_q.push(close_tag_name);
            break;
          }else if self.chars.peek().map_or(false, |c| * == "!") {
            self.chars.next();
            nodes.push(self.parse_comment_node());
          }else {
            let mut node = self.parse_node();
            let insert_index = nodes.len();
            match node.node_type {
                NodeType::Element(ref e) => if self.node_q.len() > 0 {
                  let assumed_tag = self.node_q.remove(0);

                  if e.tag_name != assumed_tag {
                    nodes.append(&mut node.children);
                    self.node_q.insert(0, assumed_tag);
                  }
                },
                _ => {}
              }
              nodes.insert(insert_index, node);
          }

        }else {
          nodes.push(self.parse_text_node());
        }
    }
    nodes
  }
}