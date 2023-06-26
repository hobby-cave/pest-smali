use pest::{iterators::Pair, Parser};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CodeError {
    #[error("parse entry")]
    ParseEntry(#[source] Box<pest::error::Error<Rule>>),
}

#[derive(pest_derive::Parser)]
#[grammar = "parser/code.pest"]
pub struct Code;

#[doc = "return 's_file' rule pair"]
pub fn parse_smali_file(code: &str) -> Result<Pair<Rule>, CodeError> {
    let mut entry =
        Code::parse(Rule::entry, code).map_err(|e| CodeError::ParseEntry(Box::new(e)))?;
    debug_assert_eq!(entry.len(), 2);
    debug_assert_eq!(entry.next_back().map(|p| p.as_rule()), Some(Rule::EOI));

    let entry = entry.next();
    debug_assert_eq!(entry.as_ref().map(|p| p.as_rule()), Some(Rule::s_file));

    Ok(entry.unwrap())
}

#[cfg(test)]
mod test {
    use {
        super::*,
        pest::iterators::Pairs,
        std::fmt::{Display, Formatter},
    };

    #[derive(Copy, Clone)]
    struct Space(usize);

    impl Display for Space {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for _ in 0..self.0 {
                write!(f, " ")?;
            }
            Ok(())
        }
    }

    #[test]
    fn parse() {
        let root =
            Code::parse(Rule::entry, include_str!("part_test.smali")).expect("parse smali file");
        dump_pair(root, 0);
    }

    #[test]
    fn literal() {
        Code::parse(Rule::h_literal, r#""test\nstring""#).unwrap();
    }

    fn dump_pair(pairs: Pairs<Rule>, prefix: usize) {
        for pair in pairs {
            println!("{}pair {:?}", Space(prefix), pair.as_rule());
            dump_pair(pair.into_inner(), prefix + 2);
        }
    }
}
