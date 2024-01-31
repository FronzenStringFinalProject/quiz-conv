use std::collections::HashMap;
use std::io::BufReader;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use crate::quiz_structure::{L1Ops, L2Ops, Quiz};

#[derive(Parser)]
#[grammar = "quizs.pest"]
pub struct QuizParser;

impl QuizParser {
    pub fn load_quiz(input: &str) -> Result<HashMap<String, Vec<Quiz>>, pest::error::Error<Rule>> {
        let mut result = HashMap::new();

        let tokens = Self::parse(Rule::stats, input)?;

        for token in tokens {
            println!("{token}");
            let (l1, ans) = if token.as_rule() == Rule::expr {
                let mut inner = token.into_inner();
                (inner.next().unwrap(), inner.next().unwrap().as_str())
            } else { continue; };

            let (l, r, o) = if l1.as_rule() == Rule::L1_calculate {
                Self::handle_l1(l1)
            } else { continue; };

            let (l,r,o)= if l.as_rule()==Rule::L2_calculate {
                let mut inner = l.into_inner();
                let l = inner.next().unwrap();
                let op = inner.next().unwrap().as_rule();
                let r = inner.next().unwrap();
                (l,r,match op{Rule::multiply=>L2Ops::Mul,Rule::divide=>L2Ops::Div,_=>unreachable!()})
            }else { continue };

            println!("{l},{r},{o:?}, {ans:?}")
        }

        Ok(result)
    }

    fn handle_l1(l1:Pair<Rule>)->(Pair<Rule>,Pair<Rule>,L1Ops){
        let mut inner = l1.into_inner();
        let l = inner.next().unwrap();
        let op = inner.next().unwrap().as_rule();
        let r = inner.next().unwrap();
        (l, r, match op {
            Rule::add => L1Ops::Add,
            Rule::subtract => L1Ops::Sub,
            _ => unreachable!()
        })
    }
}

#[cfg(test)]
mod test {
    use crate::quiz_grammer::QuizParser;

    #[test]
    fn test_parse() {
        let quiz = r"8 - 3 = 5
6 - 2 = 4
9 - 4 = 5
7 - 5 = 2
5 - 1 = 4
9 - 7 = 2
4 - 3 = 1
8 - 2 = 6
6 - 1 = 5
7 - 4 = 3
        ";

        QuizParser::load_quiz(quiz).unwrap();
    }
}