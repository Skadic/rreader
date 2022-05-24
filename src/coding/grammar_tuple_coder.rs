use std::io::Read;

use bitstream_io::{BitWrite, BitWriter, LittleEndian, Numeric, BitReader, BitRead};

use crate::grammar::{self, Grammar};

use super::grammar_coder::{GrammarDecoder, GrammarEncoder};

#[derive(Default, Debug)]
pub struct GrammarTupleCoder;

// TODO create number encoders and make the grammar tuple coder take them

impl GrammarEncoder for GrammarTupleCoder {
    type EncodeErr = std::io::Error;
    fn encode<Out: std::io::Write>(mut grammar: Grammar, out: Out) -> Result<(), Self::EncodeErr> {
        let mut bit_writer = BitWriter::endian(out, LittleEndian);
        grammar.renumber();

        let (rules, _) = grammar.consume();

        // We write this to the output so we know when to stop reading, in case there are
        // additional padding bits
        let rule_count = rules.len() as u32;
        let rule_count_bytes = rule_count.to_le_bytes();
        bit_writer.write_bytes(&rule_count_bytes)?;

        for rule in rules {
            // Write the rule length to the output
            let rule_size = rule.len() as u32;
            let rule_size_bytes = rule_size.to_le_bytes();
            bit_writer.write_bytes(&rule_size_bytes)?;

            for symbol in rule {
                if Grammar::is_terminal(symbol) {
                    // If the symbol is a terminal we write a 0 bit and then the terminal itself
                    let symbol = symbol as u8;
                    bit_writer.write_bit(false)?;
                    bit_writer.write_bytes(&[symbol])?;
                } else {
                    // If the symbol is a non-terminal we write a 1 bit and then the symbol
                    let symbol_bytes = ((symbol - grammar::RULE_OFFSET) as u32).to_le_bytes();
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
        let mut bit_reader = BitReader::endian(input, LittleEndian);
        macro_rules! rd {
            ($t:ty) => {{ 
                const TYPE_SIZE: usize = std::mem::size_of::<$t>();
                bit_reader.read_to_bytes::<TYPE_SIZE>().map(|bytes| <$t>::from_le_bytes(bytes))? as usize 
            }};
        }

        let rule_count = rd!(u32);

        let mut rules = Vec::with_capacity(rule_count); 

        for _ in 0..rule_count {
            let rule_size = rd!(u32);
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
            rules.push(rule);
        }

        Ok(Grammar::from_parts(rules, rule_count - 1))
    }
}
