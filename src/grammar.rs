use std::ops::Index;

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

        self.print();

        self.renumber_internal(self.start_rule, &mut renumbering, &mut cnt);

        let rule_count = self.rule_count();

        // Take out the old rules and replace them with a new vector
        let old_rules = std::mem::replace(&mut self.rules, vec![vec![]; rule_count]);

        println!("{}", self.start_rule);
        println!("{:?}", renumbering);

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
}

impl Index<usize> for Grammar {
    type Output = [usize];

    fn index(&self, index: usize) -> &Self::Output {
        &self.rules[index]
    }
}
