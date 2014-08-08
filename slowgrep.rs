use std::io::{File, BufferedReader};
use std::os;
use std::sync::deque::{Stealer, BufferPool, Data, Empty, Abort};

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
  let pool = BufferPool::new();
  let (producer, consumer) = pool.deque();
  for path in paths.iter() {
    producer.push(path.clone());
  }
  for _ in range(0, 3i) {
    let client_consumer = consumer.clone();
    let client_pattern = pattern.clone();
    spawn(proc() {
      consume_grep(client_consumer, &client_pattern);
    });
  }
}

fn consume_grep(consumer:Stealer<String>, pattern:&String) {
  let slice = pattern.as_slice();
  loop {
    match consumer.steal() {
      Data(path) => grep(path.as_slice(), slice),
      Empty => return,
      Abort => ()
    }
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
