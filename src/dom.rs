use core::fmt;
use std::{collections::{HashMap, HashSet}, f32::consts::E, fmt::{write, Formatter}};

struct Node {
  children: Vec<Node>,
  node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

struct ElementData {
  tag_name: String,
  attributes: AttrMap,
}

impl ElementData {
    fn new(tag_name: String, attributes:AttrMap) -> ElementData {
      ElementData{
        tag_name,
        attributes,
      }
    }

    fn get_id(&self) -> Option<&String> {
      self.attributes.get("id")
    }

    fn get_classes(&self) -> HashSet<&str> {
      match self.attributes.get("class") {
          Some(s) => s.split(' ').collect(),
          None => HashSet::new(),
      }
    }
}

type AttrMap = HashMap<String, String>;


impl Node {
    fn new(node_type: NodeType, children: Vec<Node>) -> Node {
      Node {
        node_type,
        children
      }
    }
}


impl fmt::Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}

impl fmt::Debug for NodeType {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      match *self {
          NodeType::Text(ref t) | NodeType::Comment(ref t) => write!(f, "{}", t),
          NodeType::Element(ref e) => write!(f, "{:?}", e),
      }
  }
}

impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut attributes_string = String::new();

        for (attr, value) in self.attributes.iter() {
          attributes_string.push_str(&format!(" {}=\"{}\"", attr, value));
        }

        write!(f, "<{}, {}", self.tag_name, attributes_string)
    }
}