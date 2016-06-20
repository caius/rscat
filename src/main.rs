use std::env;
use std::io;

fn main() {

  let argv = env::args().skip(1);

  if argv.len() == 0 {
    io::copy(&mut io::stdin(), &mut io::stdout()).unwrap();
  }
}
