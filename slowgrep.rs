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
  let (parent_tx, parent_rx) = channel();
  let workers = Vec::from_fn(3, |idx| {
    let (child_tx, child_rx): (Sender<Option<String>>, Receiver<Option<String>>) = channel();
    let child_pattern = pattern.clone();
    let child_parent_tx = parent_tx.clone();
    spawn(proc() { child(idx, child_parent_tx, child_rx, child_pattern) });
    child_tx
  });

  for path in paths.iter() {
    let idx = parent_rx.recv();
    let worker = workers.get(idx);
    worker.send(Some(path.clone()));
  };
  for worker in workers.iter() {
    worker.send(None);
  }
}

fn child(idx: uint, tx:Sender<uint>, rx:Receiver<Option<String>>, pattern: String) {
  let slice = pattern.as_slice();
  loop {
    if tx.send_opt(idx).is_ok() {
      match rx.recv() {
        Some(path) => grep(path.as_slice(), slice),
        None => return // Done
      }
    } else {
      return; // Parent died
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
