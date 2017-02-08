#[macro_use]
extern crate clap;
//extern crate rayon;
// extern crate regex;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;

use clap::App;
//use rayon::prelude::*;
// use regex::Regex;


fn count_occurences<P>(filename: P, search_string: &str) -> usize
where P: AsRef<Path>,
{
  let mut count = 0;
  let file = File::open(filename).expect("Cannot find IDs file!");
  let reader = io::BufReader::new(file);

  for some_line in reader.lines() {
    let line = some_line.unwrap();
    if line.contains(search_string) { count += 1; }
  }

  count
}


fn main() {
  let args = App::new(crate_name!())
      .version(crate_version!())
      .author(crate_authors!("\n"))
      .about(crate_description!())
      .args_from_usage("-s --search <search_string> 'search for this string'
                        [files]... 'read through these files'")
      .get_matches();

  let search_string = args.value_of("search").unwrap();
  let files: Vec<&str> = args.values_of("files").unwrap().collect();

  let total: usize = files.iter().map(|path| count_occurences(path, search_string)).sum();

  println!("Found {} occurences of {} in {} files.", total, search_string, files.len());
}
