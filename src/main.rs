use std::env;
use std::fs::File;
use std::io::Write;
use std::io;
use std::path::Path;
use std::process::exit;

fn copy_stdio() {
  io::copy(&mut io::stdin(), &mut io::stdout()).unwrap();
}

fn main() {

  let argv = env::args().skip(1);
  let mut exit_code = 0;

  if argv.len() == 0 {
    copy_stdio();
    return
  }

  for path in argv {
    if path == "-" {
      copy_stdio();
    } else {
      match Path::new(&path).canonicalize() {
        Err(_) => {
          writeln!(io::stderr(), "cat: {}: No such file or directory", path).unwrap();
          exit_code = 1;
        },
        Ok(filepath) => {
          let mut file = File::open(filepath).unwrap();
          io::copy(&mut file, &mut io::stdout()).unwrap();
        },
      }
    }
  }

  exit(exit_code);
}
