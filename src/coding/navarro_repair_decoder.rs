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

pub struct RePairResult {
    pub file_c: Vec<u8>,
    pub file_r: Vec<u8>
}

impl GrammarDecoder<RePairResult> for NavarroRepairDecoder {
    type DecodeErr = RReaderError;

    fn decode(res: RePairResult) -> Result<Grammar, Self::DecodeErr> {
        let file_r = res.file_r;
        let file_c = res.file_c;

        let mut chars = file_r.into_iter().peekable();

        // Read alphabet size
        let alph_n = read_int!(chars);
        // Read the alphabet
        let alph = alphabet(alph_n, &mut chars)?;

        // The rule vector
        let mut rules = vec![];

        while chars.peek().is_some() {
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

        let mut chars = file_c.into_iter().peekable();
        let mut rule_vec = vec![];
        while chars.peek().is_some() {
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


#[cfg(test)]
mod test {
    use std::io::Write;

    use crate::{coding::grammar_coder::GrammarDecoder, grammar::Grammar};

    use super::NavarroRepairDecoder;


    #[test]
    fn navarro_decode_test() {
        // alphabet: a c e g
        let mut r_bytes = vec![4u8, 0, 0, 0, 97, 99, 101, 103];
        r_bytes.write(&[0, 0, 0, 0]).unwrap(); // a
        r_bytes.write(&[1, 0, 0, 0]).unwrap(); // c
        
        r_bytes.write(&[4, 0, 0, 0]).unwrap(); // ac 
        r_bytes.write(&[2, 0, 0, 0]).unwrap(); // e
        
        r_bytes.write(&[3, 0, 0, 0]).unwrap(); // g
        r_bytes.write(&[0, 0, 0, 0]).unwrap(); // a
        
        r_bytes.write(&[5, 0, 0, 0]).unwrap(); // ace
        r_bytes.write(&[6, 0, 0, 0]).unwrap(); // ga
        
        let mut c_bytes: Vec<u8> = vec![];
        c_bytes.write(&[5, 0, 0, 0]).unwrap(); // ace
        c_bytes.write(&[7, 0, 0, 0]).unwrap(); // acega
        c_bytes.write(&[1, 0, 0, 0]).unwrap(); // c
        c_bytes.write(&[6, 0, 0, 0]).unwrap(); // ga
        
        let gr = NavarroRepairDecoder::decode(super::RePairResult { file_c: c_bytes, file_r: r_bytes });
        assert!(gr.is_ok(), "Error decoding grammar");
        let gr = gr.unwrap();

        assert_eq!(&Grammar::from_parts(vec![
            vec![97, 99],
            vec![256, 101],
            vec![103, 97],
            vec![257, 258],
            vec![257, 259, 99, 258]
        ], 4), &gr, "Grammar decoded incorrectly");

        let s = gr.produce_source_string();
        assert_eq!(Ok("aceacegacga".to_owned()), s, "Grammar producing the wrong string");
    }
}
