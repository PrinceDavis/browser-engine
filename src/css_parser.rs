use crate::css:: {Color, Declaration, Rule, Selector, SimpleSelector, Stylesheet, Unit, Value};

use std::iter::Peekable;
use std::str::Chars;


pub struct CssParser<'a> {
  chars: Peekable<Chars<'a>>,
}

impl <'a> CssParser<'a> {
  pub fn new(full_css: &str) -> CssParser {
    CssParser {
      chars: full_css.chars().peekable(),
    }
  }

  pub fn parse_stylesheet(&mut self) -> Stylesheet {
    let mut stylesheet = Stylesheet::default();

    while self.chars.peek().is_some() {
        let selector = self.parse_selectors();
        let styles = self.parse_declarations();
        let rule = Rule::new(selector, styles);

        stylesheet.rules.push(rule);
    }
    stylesheet
  }

  pub fn parse_selectors(&mut self) -> Vec<Selector> {
    let mut selectors = Vec::new();

    while self.chars.peek().map_or(false, |c| *c != '{') {
      let selector = self.parse_selector();

      if selector != Selector::default() {
        selectors.push(selector)
      }

      self.consume_where(char::is_whitespace);
      if self.chars.peek().map_or(false, |c| *c == ',') {
        self.chars.next();
      }
    }
    self.chars.next();
    selectors
  }

  pub fn parse_selector(&mut self) -> Selector {
    let mut sselector = SimpleSelector::default();
    let mut selector = Selector::default();

    self.consume_while(char::is_whitespace);

    sselector.tag_name = match self.chars.peek() {
        Some(&c) if is_valid_start_ident(c) => Some(self.parse_identifier()),
        _ => None,
    };
    let mut multiple_ids = false;
    while self.chars.peek().map_or(false, |c| *c != ',' && *c != '{' && !(*c).is_whitespace()) {
        match self.chars.peek() {
          Some(&c) if c == '#' => {
            self.chars.next();
            if sselector.id.is_some() || multiple_ids {
              sselector.id = None;
              multiple_ids = true;
              self.parse_id();
            }else {
              sselector.id = self.parse_id();
            }
          },
          Some(&c) if c == '.' => {
            self.chars.next();
            let class_name = self.parse_identifier();

            if class_name != String::from("") {
              sselector.classes.push(class_name);
            }
          },
          _ => {
            self.consume_while(|c| c!= ',' && c != '{');
          }
        }
    }
    if sselector != SimpleSelector::default() {
      selector.simple.push(sselector);
    }
    selector
  }

  pub fn parse_identifier(&mut self) -> String {
    let mut ident = String::new();

    match self.chars.peek() {
        Some(&c) =>  if is_valid_start_ident(c) {
          ident.push_str(&self.consume_while(is_valid_ident))
        },
        None => {}
    }
    ident.to_lowercase()
  }

  fn parse_id(&mut self) -> Option<String> {
    match &self.parse_identifier()[ ..] {
      "" => None,
      s @ _ => Some(s.to_string()),
    }
  }

  fn parse_declarations(&mut self) -> Vec<Declaration> {
    let mut declarations = Vect::<Declaration>::new();

    while self.chars.peek().map_or(false, |c| *c != '}') {
      self.consume_while(char::is_whitespace);

      let property = self.consume_while(|x| x != ':').to_lowercase();

      self.chars.next();
      slef.consume_whie(char::is_whitespace);

      let value = self.consume_while(|x| x != ';' && x != '\n' && x != '}').to_lowercase();

      let value_enum = match property.as_ref() {
          "backgrou-color" | "border-color" | "color" => {
            Value::Color(translate color(&value))
          }
          "margin-right" ! "margin-bottom" | "margin-left" | "margin-top" | "width" => translate_length(&value),
          _ => VAlue::Other(value),
      };

      let declaration = Declaration::new(property, value_enum);

      if self.chars.peek().map_or(false, |c| *c == ';') {
        declarations.push(declaration);
        self.chars.next();
      }else {
        self.consume_while(char::is_whitespace);
        if self.chars.peek().map_or(false, |c| *c == '}') {
          declarations.push(declaration);
        }
      }
      self.consume_while(char::is_whitespace)
    }
    self.chars.next();
    declarations
  }
}