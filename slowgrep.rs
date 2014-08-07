use std::io::{File, BufferedReader};
use std::os;

fn main() {
  let args: Vec<String> = os::args();
  match args.as_slice() {
    [ref _prog, ref pattern, ..paths] => run_grep(pattern, paths),
    _                                 => print_usage()
  }
}

fn print_usage() {
  println!("Usage: slowgrep <pattern> <files ...>");
}

fn run_grep(pattern: &String, paths: &[String]) {
  for path in paths.iter() {
    let pattern2 = pattern.clone();
    let path2 = path.clone();
    spawn(proc() {
      grep(path2.as_slice(), pattern2.as_slice());
    });
  }
}

fn grep(pathStr: &str, pattern: &str) {
  let path = Path::new(pathStr);
  let mut reader = BufferedReader::new(File::open(&path)); 
  for line in reader.lines() {
    if line.is_ok() {
      let haystack = line.unwrap();
      if haystack.as_slice().contains(pattern) {
        print!("{}: {}", pathStr, haystack);
      }
    } else {
      std::io::stderr().write_str(format!("slowgrep: Failed to process {}\n", pathStr).as_slice()).unwrap();
      break;
    }
  }
}
