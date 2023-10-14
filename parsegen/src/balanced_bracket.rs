// This file is autogenerated
/*graph: 
        digraph {
  node [shape=circle];
  Q1 [style=invisible, height=0, width=0, fixedsize=true];
  node_type = "usize";
  item_type = "(core::option::Option<(usize, bool)>, core::option::Option<parsegen::tokenizer::Token>)";
  kind = nfa;
  Q1 -> "0";
0 -> 2 [label="Lp"];
2 -> 2 [label="Lp(1"];
0 -> 3 [label=")1"];
1 -> 3 [label=")1"];
3 -> 4 [label="Rp"];
4 -> 2 [label="Lp(2"];
0 -> 1 [label=")2"];
1 -> 1 [label=")2"];
  "0" [shape=doublecircle];
  "1" [shape=doublecircle];
}

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
Lp,
Rp,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Node {
AStart,
AEnd,
Lp(usize),
Rp(usize),
} use std::fmt::Display; impl Display for Token { fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self { Self::Lp => write!(fmt, "Lp"),
Self::Rp => write!(fmt, "Rp"),
} } }
 impl Display for Node { fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self { Self::Lp(i) => write!(fmt, "Lp({i})"),
Self::Rp(i) => write!(fmt, "Rp({i})"),
Self::AStart => write!(fmt, "AStart"),
Self::AEnd=> write!(fmt, "AEnd"),
} } }

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    state: usize,
    brackets: Vec<usize>,
    tokens_eaten: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self { state: 0, brackets: vec![0], tokens_eaten: 0 }
    }

    fn top_bracket(&self) -> usize { self.brackets.last().cloned().unwrap() }

    pub fn eat_token(&mut self, tok: Token) -> Vec<Node> {
        let top = self.top_bracket();
        let mut res = vec![];
        let mut consumed = false;
        while !consumed {
        match self.state {
            0 if tok == Token::Lp => { res.push(Node::AStart); res.push(Node::Lp(self.tokens_eaten)); self.tokens_eaten += 1; consumed = true; self.state = 2; }
0 if top == 1 => { self.brackets.pop(); res.push(Node::AStart); self.state = 3; }
0 if top == 2 => { self.brackets.pop(); res.push(Node::AStart); res.push(Node::AEnd); self.state = 1; }
1 if top == 1 => { self.brackets.pop(); self.state = 3; }
1 if top == 2 => { self.brackets.pop(); res.push(Node::AEnd); self.state = 1; }
2 if tok == Token::Lp => { self.brackets.push(1);res.push(Node::Lp(self.tokens_eaten)); self.tokens_eaten += 1; consumed = true; self.state = 2; }
3 if tok == Token::Rp => { res.push(Node::Rp(self.tokens_eaten)); self.tokens_eaten += 1; consumed = true; self.state = 4; }
4 if tok == Token::Lp => { self.brackets.push(2);res.push(Node::Lp(self.tokens_eaten)); self.tokens_eaten += 1; consumed = true; self.state = 2; }

            _ => panic!("Could not continue on {}, {top}, {tok}!", self.state) 
        } }
    return res;
    }

    pub fn is_in_end_state(&self) -> bool {
        if self.brackets.len() != 1 || self.brackets[0] != 0 { return false; }
        match self.state {
0 => true,
1 => true,
_ => false, } } }
