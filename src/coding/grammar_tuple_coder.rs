use std::io::Read;

use bitstream_io::{BitWrite, BitWriter, BitReader, BitRead, BigEndian};

use crate::grammar::{self, Grammar};

use super::grammar_coder::{GrammarDecoder, GrammarEncoder};

#[derive(Default, Debug)]
pub struct GrammarTupleCoder;

// TODO create number encoders and make the grammar tuple coder take them

impl GrammarEncoder for GrammarTupleCoder {
    type EncodeErr = std::io::Error;
    fn encode<Out: std::io::Write>(mut grammar: Grammar, out: Out) -> Result<(), Self::EncodeErr> {
        let mut bit_writer = BitWriter::endian(out, BigEndian);
        grammar.renumber();

        let (rules, _) = grammar.consume();

        // We write this to the output so we know when to stop reading, in case there are
        // additional padding bits
        let rule_count = rules.len() as u32;
        bit_writer.write_bytes(&u32::to_be_bytes(rule_count))?;

        let (min_len, max_len) = rules.iter()
            .map(|v| v.len() as u32)
            .fold((u32::MAX, 0), |(old_min, old_max), v| (u32::min(old_min, v), u32::max(old_max, v)));

        bit_writer.write_bytes(&u32::to_be_bytes(min_len))?;
        bit_writer.write_bytes(&u32::to_be_bytes(max_len))?;

        for rule in rules {
            // Write the rule length to the output
            let encoded_rule_size = rule.len() as u32 - min_len;
            bit_writer.write_bytes(&u32::to_be_bytes(encoded_rule_size))?;

            for symbol in rule {
                if Grammar::is_terminal(symbol) {
                    // If the symbol is a terminal we write a 0 bit and then the terminal itself
                    let symbol = symbol as u8;
                    bit_writer.write_bit(false)?;
                    bit_writer.write_bytes(&[symbol])?;
                } else {
                    // If the symbol is a non-terminal we write a 1 bit and then the symbol
                    let symbol_bytes = ((symbol - grammar::RULE_OFFSET) as u32).to_be_bytes();
                    bit_writer.write_bit(true)?;
                    bit_writer.write_bytes(&symbol_bytes)?;
                }
            }
        }

        // Make sure any remaining bits are also written out
        bit_writer.byte_align()?;
        bit_writer.flush()?;

        Ok(())
    }
}

impl<I> GrammarDecoder<I> for GrammarTupleCoder
where
    I: Read,
{
    type DecodeErr = std::io::Error;

    fn decode(input: I) -> Result<Grammar, Self::DecodeErr> {
        let mut bit_reader = BitReader::endian(input, BigEndian);
    
        let mut buf32 = [0u8; 4];
        let mut buf8 = [0u8];

        macro_rules! rd {
            (u32) => {{ 
                bit_reader.read_bytes(&mut buf32)?;
                u32::from_be_bytes(buf32) as usize
            }};
            (u8) => {{ 
                bit_reader.read_bytes(&mut buf8)?;
                buf8[0] as usize
            }};
        }

        let rule_count = rd!(u32);
        let min_len = rd!(u32);
        let _max_len = rd!(u32);

        let mut rules = Vec::with_capacity(rule_count); 

        for _ in 0..rule_count {
            let rule_size = rd!(u32) + min_len;
            let mut rule = Vec::with_capacity(rule_size);
            for _ in 0..rule_size {
                let is_nonterminal = bit_reader.read_bit()?;
                let symbol = if is_nonterminal {
                    rd!(u32) + grammar::RULE_OFFSET
                } else {
                    rd!(u8)
                };
                rule.push(symbol);
            } 
            rule.shrink_to_fit();
            rules.push(rule);
        }

        Ok(Grammar::from_parts(rules, rule_count - 1))
    }
}


#[cfg(test)]
mod test {
    use crate::{grammar::Grammar, coding::grammar_coder::{GrammarEncoder, GrammarDecoder}};

    use super::GrammarTupleCoder;

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
    fn coding_decoding_test() {
        let mut gr = setup();
        let mut buf = vec![];

        let encoded = GrammarTupleCoder::encode(gr.clone(), &mut buf);  
        assert!(encoded.is_ok(), "Error during encoding: {:?}", encoded);

        let decoded = GrammarTupleCoder::decode(buf.as_slice());
        assert!(decoded.is_ok(), "Error during decoding: {:?}", encoded);

        let decoded = decoded.unwrap();

        // The read grammar will be renumbered as it is required by the coder
        gr.renumber();
        assert_eq!(gr, decoded, "Resulting grammar differs from original grammar");
    }
}
