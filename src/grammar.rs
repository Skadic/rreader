use std::{
    io::Write,
    ops::Index,
};

pub const RULE_OFFSET: usize = 256;

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

    pub fn reproduce(self) -> Result<String, std::string::FromUtf8Error> {
        let mut vec = Vec::<u8>::new();
        // We're writing to a string. This shouldn't fail
        self.write(&mut vec)
            .expect("Writing grammar expansion to string failed");
        String::from_utf8(vec)
    }

    pub fn write(mut self, mut out: impl Write) -> std::io::Result<()> {
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
                    rule_exp.extend(expansions[symbol - RULE_OFFSET].chars())
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
