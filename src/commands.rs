use crate::args::*;
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::Result;
use std::fs;
use std::str::FromStr;

fn encode(args: EncodeArgs) -> Result<()> {
    let input_bytes = fs::read(&args.input_file_path)?;
    let output = args.output_file_path.unwrap_or(args.input_file_path);
    let mut png = Png::try_from(input_bytes.as_slice())?;
    let chunk = Chunk::new(ChunkType::from_str(args.chunk_type.as_str())?, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);
    fs::write(output, png.as_bytes())?;
    Ok(())
}

fn decode(args: DecodeArgs) -> Result<()> {
    let input_bytes = fs::read(&args.input_file_path)?;
    let png = Png::try_from(input_bytes.as_slice())?;
    if let Some(c) = png.chunk_by_type(args.chunk_type.as_str()) {
        println!("Decoded: {}", c);
    }
    Ok(())
}
fn remove(args: RemoveArgs) -> Result<()> {
    let input_bytes = fs::read(&args.input_file_path)?;
    let output = args.input_file_path;
    let mut png = Png::try_from(input_bytes.as_slice())?;
    match png.remove_chunk(args.chunk_type.as_str()) {
        Ok(chunk) => {
            fs::write(output, png.as_bytes())?;
            println!("Removed chunk: {}", chunk);
        }
        Err(e) => println!("Error removing chunk: {}", e),
    }
    Ok(())
}
fn print(args: PrintArgs) -> Result<()> {
    let input_bytes = fs::read(&args.input_file_path)?;
    let png = Png::try_from(input_bytes.as_slice())?;
    for chunk in png.chunks() {
        println!("{}", chunk);
    }

    Ok(())
}

pub fn run(command: Commands) -> Result<()> {
    match command {
        Commands::Encode(args) => encode(args),
        Commands::Decode(args) => decode(args),
        Commands::Remove(args) => remove(args),
        Commands::Print(args) => print(args),
    }
}
