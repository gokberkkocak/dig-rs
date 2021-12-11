use std::{collections::HashMap, fs};

use boolean_expression::Expr;

use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
    static ref REG: Regex = Regex::new(r"fromSolution\(([a-zA-Z0-9\[\]_]+)\)").unwrap();
}

pub(crate) struct Parser {
    dominance_line: String,
}

impl Parser {
    pub fn parse_file(filename: &str) -> (Expr<String>, HashMap<String, String>) {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        for line in contents.lines() {
            if line.starts_with("dominance_relation") {
                assert!(Parser::are_brackets_valid(line));
                let mut p = Parser {
                    dominance_line: line.to_string(),
                };
                return p.parse_dom_line();
            }
        }
        panic!("Couldn't find a dominance line")
    }

    fn are_brackets_valid(line: &str) -> bool {
        let mut stack = Vec::new();
        for i in line.chars() {
            match i {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '}' | ')' | ']' if Some(i) != stack.pop() => return false,
                _ => (),
            }
        }
        stack.is_empty()
    }

    fn parse_dom_line(&mut self) -> (Expr<String>, HashMap<String, String>) {
        fn parse_inner(line: &str, map: &mut HashMap<String, String>) -> (Expr<String>, usize) {
            fn next_identifier(map: &HashMap<String, String>) -> String {
                for c in 'a'..'z' {
                    if !map.contains_key(&c.to_string()) {
                        return c.to_string();
                    }
                }
                // wow all occupied
                for c in 'a'..'z' {
                    let mut s = c.to_string();
                    for i in 1..9 {
                        s.push_str(&i.to_string());
                        if !map.contains_key(&s.to_string()) {
                            return s.to_string();
                        }
                    }
                }
                unreachable!()
            }
            // println!("inner --- {}", line);
            let mut expr = Expr::Const(true);
            let mut parse_status = ParseStatus::Operator;
            let mut current_operator = Operator::And;
            let mut is_negated = false;
            let mut i = 0;
            while i < line.len() {
                // println!("actual --- {}", line);
                // println!("{} {}", i, line.len());
                let c = line.chars().nth(i).unwrap();
                match c {
                    '(' => {
                        let (mut sub_expr, ind) = parse_inner(&line[i + 1..], map);
                        if is_negated {
                            sub_expr = Expr::not(sub_expr);
                        }
                        expr = current_operator.apply(expr, sub_expr);
                        current_operator = Operator::And;
                        i += ind + 1;
                    }
                    ')' => {
                        match parse_status {
                            ParseStatus::Operator => {},
                            ParseStatus::Identifier(x, y) => {
                                // parse_status = ParseStatus::Operator;
                                let identifier = line[x..y].to_string();
                                let key = next_identifier(map);
                                let mut sub_expr = Expr::Terminal(key.clone());
                                map.insert(key, identifier);
                                if is_negated {
                                    sub_expr = Expr::not(sub_expr);
                                }
                                expr = current_operator.apply(expr, sub_expr);
                                // current_operator = Operator::And;
                            }
                        }
                        return (expr, i);
                    }
                    '+' => match parse_status {
                        ParseStatus::Operator => {
                            current_operator = Operator::Or;
                        }
                        ParseStatus::Identifier(x, y) => {
                            parse_status = ParseStatus::Operator;
                            let identifier = line[x..y].to_string();
                            let key = next_identifier(map);
                            let mut sub_expr = Expr::Terminal(key.clone());
                            map.insert(key, identifier);
                            if is_negated {
                                sub_expr = Expr::not(sub_expr);
                            }
                            expr = current_operator.apply(expr, sub_expr);
                            current_operator = Operator::And;
                        }
                    },
                    '*' => match parse_status {
                        ParseStatus::Operator => {
                            current_operator = Operator::And;
                        }
                        ParseStatus::Identifier(x, y) => {
                            parse_status = ParseStatus::Operator;
                            let identifier = line[x..y].to_string();
                            let key = next_identifier(map);
                            let mut sub_expr = Expr::Terminal(key.clone());
                            map.insert(key, identifier);
                            if is_negated {
                                sub_expr = Expr::not(sub_expr);
                            }
                            expr = current_operator.apply(expr, sub_expr);
                            current_operator = Operator::And;
                        }
                    },
                    '#' => match parse_status {
                        ParseStatus::Operator => {
                            current_operator = Operator::Imply;
                        }
                        ParseStatus::Identifier(x, y) => {
                            parse_status = ParseStatus::Operator;
                            let identifier = line[x..y].to_string();
                            let key = next_identifier(map);
                            let mut sub_expr = Expr::Terminal(key.clone());
                            map.insert(key, identifier);
                            if is_negated {
                                sub_expr = Expr::not(sub_expr);
                            }
                            expr = current_operator.apply(expr, sub_expr);
                            current_operator = Operator::And;
                        }
                    },
                    '!' => match parse_status {
                        ParseStatus::Operator => {
                            is_negated = true;
                        }
                        ParseStatus::Identifier(_, _) => {
                            unreachable!()
                        }
                    },
                    c if c.is_ascii_whitespace() => {}
                    _ => match parse_status {
                        ParseStatus::Operator => {
                            parse_status = ParseStatus::Identifier(i, i + 1);
                        }
                        ParseStatus::Identifier(x, _) => {
                            parse_status = ParseStatus::Identifier(x, i + 1);
                        }
                    },
                }
                i += 1;
            }
            (expr, line.len())
        }
        // preprocess
        self.dominance_line = self.dominance_line.replace("\\/", "+");
        self.dominance_line = self.dominance_line.replace("/\\", "*");
        self.dominance_line = self.dominance_line.replace("->", "#");
        self.dominance_line = self
            .dominance_line
            .strip_prefix("dominance_relation ")
            .unwrap()
            .to_string();
        self.dominance_line = REG
            .replace_all(&self.dominance_line, "from_solution_${1}")
            .into();
        let mut map = HashMap::new();
        let (dom_no_good, _i) = parse_inner(&self.dominance_line, &mut map);
        // Dominance no good relation is negated to get the actual dominance relation.
        (Expr::not(dom_no_good), map)
    }
}

enum ParseStatus {
    Identifier(usize, usize),
    Operator,
}

enum Operator {
    And,
    Or,
    Imply,
}

impl Operator {
    fn apply(&self, lhs: Expr<String>, rhs: Expr<String>) -> Expr<String> {
        match self {
            Operator::And => Expr::and(lhs, rhs),
            Operator::Or => Expr::or(lhs, rhs),
            Operator::Imply => Expr::or(Expr::not(lhs), rhs),
        }
    }
}
