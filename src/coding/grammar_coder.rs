use std::io::Write;

use crate::grammar::Grammar;

pub trait GrammarEncoder {
    type EncodeErr;
    fn encode<Out: Write>(grammar: Grammar, out: Out) -> Result<(), Self::EncodeErr>;
}

pub trait GrammarDecoder<I> {
    type DecodeErr;
    fn decode(input: I) -> Result<Grammar, Self::DecodeErr>;
}
