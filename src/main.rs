use rreader::coding::{
    grammar_coder::{GrammarDecoder, GrammarEncoder},
    grammar_tuple_coder::GrammarTupleCoder,
    navarro_repair_decoder::NavarroRepairDecoder,
};
use rreader::error::RReaderError;


fn main() -> Result<(), RReaderError> {
    let file_name = {
        let mut args = std::env::args().skip(1);
        args.next().ok_or(RReaderError::NoInputFile)?
    };

    let res = rreader::repair(&file_name)?;
    let gr = NavarroRepairDecoder::decode(res)?; 
    let file = std::fs::File::create(format!("{file_name}.grm"))?;

    GrammarTupleCoder::encode(gr, file)?;

    Ok(())
}
