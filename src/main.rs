#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::path::Path;
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

  if args.len() != 2
  {
    eprintln!("[Help] compsvg <input_file>");
    return;
  }

  let inputPath: &str = &args[1];
  
  // Генерация имени для выходного файла с заменой расширения на .zst
  let outputPath: String = format!("{}", Path::new(inputPath).with_extension("zst").display());

  match compress(inputPath, &outputPath) 
  {
    Ok(_) => (),
    Err(e) => eprintln!("[Error] {}", e),
  }
}
