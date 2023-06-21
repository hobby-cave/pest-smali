#[derive(pest_derive::Parser)]
#[grammar = "parser/code.pest"]
pub struct Code;

#[cfg(test)]
mod test {
    use {
        super::*,
        pest::{iterators::Pairs, Parser},
    };

    #[test]
    fn parse() {
        let root = Code::parse(Rule::root, " #with comment\n.class").expect("parse");
        dump_pair(root, "");
    }

    fn dump_pair<S: AsRef<str>>(pairs: Pairs<Rule>, prefix: S) {
        for pair in pairs {
            println!("{}pair {:?}", prefix.as_ref(), pair.as_rule());
            dump_pair(pair.into_inner(), format!("  {}", prefix.as_ref()))
        }
    }
}
