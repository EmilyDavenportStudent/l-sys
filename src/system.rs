use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

pub enum BaseSymbols {
    Forward,
    ForwardNoLine,
    AngleIncrement,
    AngleDecrement,
}

impl Display for BaseSymbols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BaseSymbols::Forward => 'F',
                BaseSymbols::ForwardNoLine => 'f',
                BaseSymbols::AngleIncrement => '+',
                BaseSymbols::AngleDecrement => '-',
            }
        )
    }
}

pub struct SystemExpression(pub Vec<BaseSymbols>);

impl Display for SystemExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let exp: Vec<String> = self.0.iter().map(|sym| format!("{}", sym)).collect();
        write!(f, "{}", exp.join(""))
    }
}

impl SystemExpression {
    pub fn total_edges(&self) -> i32 {
        self.0.iter().fold(0, |acc, sym| match sym {
            BaseSymbols::Forward | BaseSymbols::ForwardNoLine => acc + 1,
            _ => acc,
        })
    }
}

#[derive(Debug)]
pub struct System {
    axiom: &'static str,
    rules: HashMap<char, &'static str>,
}

impl System {
    pub fn from_axiom(axiom: &'static str) -> Self {
        Self {
            axiom,
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, rule: &'static str) {
        let pat = Regex::new(r"([A-Za-z])\s*=>\s*([A-Za-z+-]+)").unwrap();
        let captures = pat.captures(rule).expect("Expected to parse system rule");
        let symbol = captures
            .get(1)
            .expect("Expected symbol for rule")
            .as_str()
            .chars()
            .nth(0)
            .unwrap();
        let expression = captures
            .get(2)
            .expect("Expected expression for rule")
            .as_str();
        self.rules.insert(symbol, expression);
    }

    pub fn do_n_iterations(&self, mut n: i32) -> SystemExpression {
        let mut current_buf = self.axiom.clone().to_string();
        while n > 0 {
            current_buf = self.expand_expression(current_buf);
            n -= 1;
        }
        return derive_base_symbols(current_buf);
    }

    fn expand_expression(&self, expression: String) -> String {
        let mut buffer = String::new();
        for c in expression.chars() {
            match self.rules.get(&c) {
                Some(s) => buffer.push_str(s.clone()),
                None => buffer.push(c),
            }
        }
        buffer
    }
}

fn derive_base_symbols(s: String) -> SystemExpression {
    SystemExpression(
        s.chars()
            .map(|c| match c {
                'f' => BaseSymbols::ForwardNoLine,
                '+' => BaseSymbols::AngleIncrement,
                '-' => BaseSymbols::AngleDecrement,
                _ => BaseSymbols::Forward,
            })
            .collect(),
    )
}
