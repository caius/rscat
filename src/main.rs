use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn copy_file(path: PathBuf) {
  let mut file = File::open(path).unwrap();
  io::copy(&mut file, &mut io::stdout()).unwrap();
}

fn main() {

  let argv = env::args().skip(1);

  if argv.len() == 0 {
    io::copy(&mut io::stdin(), &mut io::stdout()).unwrap();
    return
  }

  for path in argv {
    match Path::new(&path).canonicalize() {
      Err(_) => writeln!(io::stderr(), "cat: {}: No such file or directory", path).unwrap(),
      Ok(filepath) => copy_file(filepath),
    }
  }

}
