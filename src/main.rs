use coding::{
    grammar_coder::{GrammarDecoder, GrammarEncoder},
    grammar_tuple_coder::GrammarTupleCoder,
    navarro_repair_decoder::NavarroRepairDecoder,
};
use error::RReaderError;

mod coding;
mod error;
mod grammar;

fn main() -> Result<(), RReaderError> {
    let file_name = {
        let mut args = std::env::args().skip(1);
        args.next().ok_or(RReaderError::NoInputFile)?
    };
    let file_r = std::fs::read_to_string(file_name.clone() + ".R")?;
    let file_c = std::fs::read_to_string(file_name + ".C")?;

    let gr = NavarroRepairDecoder::decode((file_r, file_c))?;

    let mut bytes: Vec<u8> = vec![];
    GrammarTupleCoder::encode(gr, &mut bytes)?;

    let gr2 = GrammarTupleCoder::decode(&bytes[..])?;
    gr2.print();
    Ok(())
}
