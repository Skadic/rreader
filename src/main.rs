use clap::Parser;
use rreader::coding::{
    grammar_coder::{GrammarDecoder, GrammarEncoder},
    grammar_tuple_coder::GrammarTupleCoder,
    navarro_repair_decoder::NavarroRepairDecoder,
};
use rreader::error::RReaderError;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, help = "The input file")]
    file: String,
    #[clap(short, long, requires = "out", help = "Decompress the input file")]
    decompress: bool,
    #[clap(short, long, help = "The output file")]
    out: Option<String>,
}

fn main() -> Result<(), RReaderError> {
    let args = Args::parse();

    if !args.decompress {
        let repair_result = rreader::repair(&args.file)?;
        let grammar = NavarroRepairDecoder::decode(repair_result)?;
        let out_file_name = args.out.unwrap_or(format!("{}.grm", &args.file));
        let out_file = std::fs::File::create(out_file_name)?;

        GrammarTupleCoder::encode(grammar, out_file)?
    } else {
        let file = std::fs::File::open(&args.file)?;
        // out is required when decompressing
        let out_file = std::fs::File::create(args.out.unwrap())?;
        let grammar = GrammarTupleCoder::decode(file)?;
        grammar.write_source_string(out_file)?
    }

    Ok(())
}
