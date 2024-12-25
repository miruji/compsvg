#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::path::Path;
use brotli2::write::BrotliEncoder;

fn compress(inputPath: &str, outputPath: &str) -> std::io::Result<()> 
{
  // Открываем исходный SVG-файл
  let inputFile: File = File::open(inputPath)?;
  let mut inputReader = std::io::BufReader::new(inputFile);

  // Создаем выходной файл для сжатых данных
  let outputFile: File = File::create(outputPath)?;
  let mut output_writer = std::io::BufWriter::new(outputFile);

  // Создаем BrotliEncoder с уровнем сжатия 5
  let mut encoder: BrotliEncoder = BrotliEncoder::new(output_writer, 5);

  // Копируем данные из исходного файла в сжимающий поток
  std::io::copy(&mut inputReader, &mut encoder)?;

  // Завершаем сжатие
  encoder.finish()?;

  println!("[Ok] Compressed to [{}]", outputPath);
  Ok(())
}

// for bash:
//for file in 111/*.svg; do 
//  cargo run --release -- "$file"
//done

fn main() 
{
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 
  {
    eprintln!("[Help] compress_svg <inputFile>");
    return;
  }

  let inputPath = &args[1];

  // Генерация имени для выходного файла с заменой расширения на .svg.br
  let outputPath: String = format!("{}.br", inputPath);

  match compress(inputPath, &outputPath) 
  {
    Ok(_) => (),
    Err(e) => eprintln!("[Error] {}", e),
  }
}
