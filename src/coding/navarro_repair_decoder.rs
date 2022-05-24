use crate::{
    error::RReaderError,
    grammar::{Grammar, RULE_OFFSET},
};

use super::grammar_coder::GrammarDecoder;

macro_rules! read_int {
    ($chars:ident) => {{
        let a = $chars
            .next()
            .ok_or(RReaderError::MissingInput("Alphabet size"))?;
        let b = $chars
            .next()
            .ok_or(RReaderError::MissingInput("Alphabet size"))?;
        let c = $chars
            .next()
            .ok_or(RReaderError::MissingInput("Alphabet size"))?;
        let d = $chars
            .next()
            .ok_or(RReaderError::MissingInput("Alphabet size"))?;
        u32::from_le_bytes([a, b, c, d]) as usize
    }};
}

#[derive(Debug, Default, Clone, Copy)]
pub struct NavarroRepairDecoder;

impl GrammarDecoder<(String, String)> for NavarroRepairDecoder {
    type DecodeErr = RReaderError;

    fn decode((file_r, file_c): (String, String)) -> Result<Grammar, Self::DecodeErr> {
        let mut chars = file_r.bytes().peekable();

        // Read alphabet size
        let alph_n = read_int!(chars);
        // Read the alphabet
        let alph = alphabet(alph_n, &mut chars)?;

        // The rule vector
        let mut rules = vec![];

        while let Some(_) = chars.peek() {
            let mut l = read_int!(chars);
            let mut r = read_int!(chars);

            l = if l < alph_n {
                alph[l] as usize
            } else {
                l - alph_n + RULE_OFFSET
            };

            r = if r < alph_n {
                alph[r] as usize
            } else {
                r - alph_n + RULE_OFFSET
            };
            rules.push(vec![l, r]);
        }

        let mut chars = file_c.bytes().peekable();
        let mut rule_vec = vec![];
        while let Some(_) = chars.peek() {
            let symb = read_int!(chars);
            rule_vec.push(if symb < alph_n {
                alph[symb] as usize
            } else {
                symb - alph_n + RULE_OFFSET
            });
        }
        rules.push(rule_vec);

        let n_rules = rules.len();
        Ok(Grammar::from_parts(rules, n_rules - 1))
    }
}

fn alphabet(
    alph_n: usize,
    chars: &mut impl Iterator<Item = u8>,
) -> Result<Vec<char>, RReaderError> {
    let mut vec = Vec::with_capacity(alph_n);
    for _ in 0..alph_n {
        vec.push(
            chars
                .next()
                .ok_or(RReaderError::MissingInput("Alphabet character"))? as char,
        );
    }
    Ok(vec)
}
