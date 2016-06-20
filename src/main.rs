use std::env;
use std::fs::File;
use std::io;
use std::path::Path;

fn main() {

  let argv = env::args().skip(1);

  if argv.len() == 0 {
    io::copy(&mut io::stdin(), &mut io::stdout()).unwrap();
  } else {
    let handlers = argv
      .map (|path| Path::new(&path).canonicalize().unwrap() )
      .map (|path| File::open(&path).unwrap() );

    for mut file in handlers {
      io::copy(&mut file, &mut io::stdout()).unwrap();
    }
  }
}
