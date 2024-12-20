#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::fs::File;
use std::io::{Read, Write};
use std::env;
use zstd::stream::Encoder;

fn compress(inputPath: &str, outputPath: &str) -> std::io::Result<()> 
{
  // Открываем файл с .svg
  let mut inputFile: File = File::open(inputPath)?;
  let mut buffer: Vec<u8> = Vec::new();
  inputFile.read_to_end(&mut buffer)?;

  // Создаем выходной файл
  let outputFile: File = File::create(outputPath)?;

  // Используем zstd для сжатия
  let mut encoder: Encoder<File> = Encoder::new(outputFile, 0)?; // Уровень сжатия 0 (по умолчанию)
  encoder.write_all(&buffer)?;
  encoder.finish()?;

  println!("[Ok] Compressed to [{}]", outputPath);
  Ok(())
}

fn main() 
{
  let args: Vec<String> = env::args().collect();

  match args.len() != 3 
  {
    false => {}
    true => 
    {
      eprintln!("[Help] compsvg <input_file> <output_file_base>");
      return;
    }
  }

  let inputPath: &str = &args[1];
  let outputPath: String = format!("{}.zst", args[2]);

  match compress(inputPath, &outputPath) 
  {
    Ok(_) => (),
    Err(e) => eprintln!("[Error] {}", e),
  }
}
