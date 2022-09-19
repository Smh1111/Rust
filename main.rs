use std::{io, vec, env, fs};
use std::io::{BufWriter, Write};
use std::time::{Instant, Duration};

#[derive(Debug, Clone, Copy)]
struct FreqTableEntry {
    letter: char,
    count: u64,
    frequency: f64,
}

// This function will update your freq_table
fn unpack_text_line( line: String, freq_table: &mut Vec<FreqTableEntry>, non_burmese_table: &mut Vec<FreqTableEntry> ) -> u64 {
  let mut nc: u64 = 0;
  for i in line.chars() {
    if (i as u32) >= 0x1000 && (i as u32) <= 0x109F{
      check_add_letter( i, freq_table );
      nc += 1;
    }
    else{
      check_add_letter( i, non_burmese_table );
      nc += 1;
    }
  }
  nc
}

fn read_text_line(filepath: &String) -> String{
  print!("Text to be processed ");
  let mut buffer = fs::read_to_string(filepath).expect("Unable to read file");
  buffer = buffer.trim().to_string();
  buffer
}

fn check_add_letter( letter: char, freq_table: &mut Vec<FreqTableEntry> ) {
  let mut found: bool = false;
  for e in freq_table.iter_mut() {
    if e.letter == letter {
      e.count += 1;
      found = true;
      break;
    }
  }
  if !found {
    let new_entry = FreqTableEntry {
      letter: letter,
      count: 1,
      frequency: 0.0,
    };
    freq_table.push( new_entry );
  }
}

fn calc_percentage( freq_table: &mut Vec<FreqTableEntry>, total: u64 ) {
  for e in freq_table.iter_mut() {
    e.frequency = ((e.count as f64) / (total as f64)) * 100.0;
  }
}

fn print_vec( out_file: &String, freq_table: &Vec<FreqTableEntry> , non_burmese_table: &Vec<FreqTableEntry>, text: String, total: u64, duration: Duration ) {
  // create a new file
  let file = fs::File::create(out_file).expect("Unable to create file");
  // write to the file
  let mut file = BufWriter::new(file);

  writeln!(file, "Text to be processed:\n{}\n\n", text).expect("Unable to write data");
  writeln!(file, "========================================================\n/***************burmese Character***************\\\n").expect("Unable to write to file");
  writeln!(file, "Letter\tUnicode\tCount\tFrequency (%)").expect("Unable to write to file");

  for e in freq_table.iter() {
    let unicode = format!("0{:X}", e.letter as u64);
    writeln!(file, "{:9}\t0{:8}\t{}\t{:.3}%\n", e.letter, unicode, e.count, e.frequency ).expect("Unable to write to file");
  }
  writeln!(file, "========================================================\n/***************Non-burmese Character***************\\\n").expect("Unable to write to file");
  writeln!(file, "Letter\tUnicode\tCount\tFrequency (%)").expect("Unable to write to file");

  for e in non_burmese_table.iter() {
    let unicode = format!("0{:X}", e.letter as u64);
    writeln!(file, "{:9}\t0{:8}\t{}\t{:.3}%\n", e.letter, unicode, e.count, e.frequency ).expect("Unable to write to file");
  }
  
  let (max_burmese, max_non_burmese) = add_summary(freq_table, non_burmese_table);
  let time = format!("Time duration to {} miliseconds", duration.as_millis());
  let total = format!("Total number of characters {}", total);

  let sum_head = format!("RESULT SUMMARY: ");
  writeln!(file, "========================================================\n{}\n{}\n{}\n{}\n{}",sum_head, max_burmese, max_non_burmese, time, total).expect("Unable to write to file");

} 

fn add_summary(freq_table: &Vec<FreqTableEntry>, non_burmese_table: &Vec<FreqTableEntry>) -> (String, String) {

  // check most frequent letter in burmese
  let mut max = 0;
  let mut max_letter = ' ';
  for e in freq_table.iter() {
    if e.count > max {
      max = e.count;
      max_letter = e.letter;
    }
  }
  let max_burmese = format!("Most frequent letter in burmese is \"{}\" with {} occurrences", max_letter, max);

  // check most frequent letter that are not burmese
  max = 0;
  max_letter = ' ';
  for e in non_burmese_table.iter() {
    if e.count > max {
      max = e.count;
      max_letter = e.letter;
    }
  }
  let max_non_burmese = format!("Most frequent letter that are not burmese is \"{}\" with {} occurrences", max_letter, max);

  return(max_burmese, max_non_burmese);
}


fn main() {
  // get time duration of the program
  let start = Instant::now();
  let args: Vec<String> = env::args().collect();
  let filepath = &args[1];
  let outputpath = &args[2];

  let mut ix: u64 = 0;
  let mut total = 0;
  let mut nc = 0;
  // Construct and empty table
  let mut cha:Vec<FreqTableEntry> = Vec::new();
  let mut non_burmese:Vec<FreqTableEntry> = Vec::new();
  // loop through the text
  let text = read_text_line(filepath);
  let lines = text.lines();
  for l in lines {
    
 
    nc = unpack_text_line( l.to_string(), &mut cha, &mut non_burmese );
    ix += 1;
    
    total += nc;
  }
  // stop the timer
  let duration = start.elapsed();
  // fn to Generate frequency fractions
  calc_percentage(&mut cha, total);
  calc_percentage(&mut non_burmese, total);
  // fn to Generate report
  print_vec( outputpath ,&cha, &non_burmese, text, total, duration);
}