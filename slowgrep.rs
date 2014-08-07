use std::io::File;
use std::io::BufferedReader;
use std::os;

fn main() {
  let args: Vec<String> = os::args();
  match args.as_slice() {
    [ref _prog, ref pattern, ..paths] => run_grep(pattern.as_slice(), paths),
    _                         => print_usage()
  }
}

fn print_usage() {
  println!("Usage: slowgrep <pattern> <files ...>");
}

fn run_grep(pattern: &str, paths: &[String]) {
  for path in paths.iter() {
    grep(path.as_slice(), pattern);
  }
}

fn grep(pathStr: &str, pattern: &str) {
  let path = Path::new(pathStr);
  let mut reader = BufferedReader::new(File::open(&path)); 
  for line in reader.lines() {
    let haystack = line.unwrap();
    if haystack.as_slice().contains(pattern) {
      print!("{}: {}", pathStr, haystack);
    }
  }
}
