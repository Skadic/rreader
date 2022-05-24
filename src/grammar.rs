use std::{io::Write, ops::Index};

pub const RULE_OFFSET: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grammar {
    rules: Vec<Vec<usize>>,
    start_rule: usize,
}

impl Grammar {
    pub fn empty() -> Self {
        Grammar {
            rules: vec![],
            start_rule: 0,
        }
    }

    pub fn is_terminal(symbol: usize) -> bool {
        symbol < RULE_OFFSET
    }

    pub fn is_nonterminal(symbol: usize) -> bool {
        symbol >= RULE_OFFSET
    }

    pub fn from_parts(rules: Vec<Vec<usize>>, start_rule: usize) -> Self {
        Self { rules, start_rule }
    }

    pub fn set_start_rule(&mut self, new_start_rule: usize) {
        self.start_rule = new_start_rule;
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    fn renumber_internal(&self, id: usize, renumbering: &mut [usize], count: &mut usize) {
        self[id]
            .iter()
            .copied()
            .filter(|&symbol| Grammar::is_nonterminal(symbol))
            .map(|symbol| symbol - RULE_OFFSET)
            .for_each(|symbol| self.renumber_internal(symbol, renumbering, count));

        // If the renumbering is not set yet, set it
        if renumbering[id] == usize::MAX {
            renumbering[id] = *count;
            *count += 1;
        }
    }

    pub fn renumber(&mut self) {
        if self.rules.is_empty() {
            return;
        }

        let mut renumbering = vec![usize::MAX; self.rule_count()];
        let mut cnt = 0;

        self.renumber_internal(self.start_rule, &mut renumbering, &mut cnt);

        let rule_count = self.rule_count();

        // Take out the old rules and replace them with a new vector
        let old_rules = std::mem::replace(&mut self.rules, vec![vec![]; rule_count]);

        // Renumber the symbols in each rule and insert them into the appropriate place
        for (i, mut rule) in old_rules.into_iter().enumerate() {
            rule.iter_mut()
                .filter(|&&mut symbol| Grammar::is_nonterminal(symbol))
                .for_each(|symbol| *symbol = renumbering[*symbol - RULE_OFFSET] + RULE_OFFSET);
            self.rules[renumbering[i]] = rule;
        }

        self.start_rule = self.rule_count() - 1;
    }

    pub fn consume(self) -> (Vec<Vec<usize>>, usize) {
        (self.rules, self.start_rule)
    }

    pub fn print(&self) {
        for (i, rule) in self.rules.iter().enumerate() {
            let rule_str = rule
                .iter()
                .map(|&symbol| {
                    if Grammar::is_terminal(symbol) {
                        (symbol as u8 as char).to_string()
                    } else {
                        format!("R{}", symbol - RULE_OFFSET)
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            println!("R{i} -> {rule_str}")
        }
    }

    pub fn produce_source_string(self) -> Result<String, std::string::FromUtf8Error> {
        let mut vec = Vec::<u8>::new();
        // We're writing to a string. This shouldn't fail
        self.write_source_string(&mut vec)
            .expect("Writing grammar expansion to string failed");
        String::from_utf8(vec)
    }

    pub fn write_source_string(mut self, mut out: impl Write) -> std::io::Result<()> {
        if self.rule_count() == 0 {
            return Ok(());
        }

        self.renumber();

        let rule_count = self.rule_count();
        let mut expansions = Vec::<String>::with_capacity(rule_count);

        // There should always be a start rule
        let start_rule = self.rules.pop().unwrap();

        for rule in self.rules.into_iter() {
            let mut rule_exp = String::with_capacity(rule.len());
            for symbol in rule {
                if Grammar::is_terminal(symbol) {
                    rule_exp.push(symbol as u8 as char)
                } else {
                    rule_exp.push_str(expansions[symbol - RULE_OFFSET].as_str())
                }
            }
            expansions.push(rule_exp)
        }

        for symbol in start_rule {
            if Grammar::is_terminal(symbol) {
                out.write_all(&[symbol as u8])?;
            } else {
                out.write_all(expansions[symbol - RULE_OFFSET].as_bytes())?;
            }
        }

        Ok(())
    }
}

impl Index<usize> for Grammar {
    type Output = [usize];

    fn index(&self, index: usize) -> &Self::Output {
        &self.rules[index]
    }
}

#[cfg(test)]
mod test {
    use super::Grammar;

    fn setup() -> Grammar {
        Grammar::from_parts(
            vec![
                vec![257, 258, 100],
                vec![97, 98, 99],
                vec![100, 101, 259],
                vec![102, 103, 104, 257],
            ],
            0,
        )
    }

    #[test]
    fn consume_test() {
        let gr = setup();
        let (rules, start_rule) = gr.consume();

        assert_eq!(
            vec![
                vec![257, 258, 100],
                vec![97, 98, 99],
                vec![100, 101, 259],
                vec![102, 103, 104, 257],
            ],
            rules,
            "Grammar rules changed when consume was called"
        );
        assert_eq!(
            0, start_rule,
            "Grammar start rule changed when consume was called"
        );
    }

    #[test]
    fn rule_count_test() {
        let gr = setup();
        assert_eq!(4, gr.rule_count(), "Grammar rule count incorrect")
    }

    #[test]
    fn index_test() {
        let gr = setup();
        assert_eq!([97usize, 98, 99], gr[1], "Grammar indexing incorrect");
    }

    #[test]
    fn reproduce_test() {
        let gr = setup();

        let src_string = gr.produce_source_string();
        assert_eq!(
            Ok("abcdefghabcd".to_owned()),
            src_string,
            "Source string not correctly reproduced"
        );
    }

    #[test]
    fn renumber_test() {
        let mut gr = setup();

        gr.renumber();

        assert_eq!(
            &Grammar::from_parts(
                vec![
                    vec![97, 98, 99],
                    vec![102, 103, 104, 256],
                    vec![100, 101, 257],
                    vec![256, 258, 100],
                ],
                3
            ),
            &gr,
            "Grammar renumbering incorrect"
        );
    }

    #[test]
    fn print_test() {
        let gr = setup();

        // It's a print method. I want that coverage
        gr.print();
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            Grammar {
                rules: vec![],
                start_rule: 0
            },
            Grammar::empty(),
            "Empty grammar not empty"
        );
        assert_eq!(
            Grammar {
                rules: vec![],
                start_rule: 0
            },
            Grammar::from_parts(vec![], 0),
            "Empty grammar produced by from_parts not empty"
        );
    }


    #[test]
    fn terminal_non_terminal_test() {
        assert_eq!(true, Grammar::is_nonterminal(260), "symbol 260 not classified as non-terminal");
        assert_eq!(true, Grammar::is_nonterminal(256), "symbol 256 not classified as non-terminal");
        assert_eq!(false, Grammar::is_nonterminal(255), "symbol 255 classified as non-terminal");
        assert_eq!(false, Grammar::is_nonterminal(24), "symbol 224 classified as non-terminal");

        assert_eq!(false, Grammar::is_terminal(260), "symbol 260 classified as terminal");
        assert_eq!(false, Grammar::is_terminal(256), "symbol 256 classified as terminal");
        assert_eq!(true, Grammar::is_terminal(255), "symbol 255 not classified as terminal");
        assert_eq!(true, Grammar::is_terminal(24), "symbol 24 not classified as terminal");
    }
}
