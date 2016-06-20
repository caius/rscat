# rscat

`cat(1)` implemented in rust as a learning exercise.

Passes [cat_linter][]:

```
$ cargo build --release
$ cat_linter target/release/rscat
Linting cat against ../rscat/target/release/rscat

Checking `cat /tmp/cat_linter_file1`:
  Output matches
  Exit matches

Checking `cat /tmp/cat_linter_file1 /tmp/cat_linter_file2`
  Output matches
  Exit matches

Checking `cat /tmp/cat_linter_file1 non-existent-file `
  Output matches
  Exit matches

Checking `echo "hi\nnow" | cat`
  Output matches
  Exit matches

Checking `echo "how\nnow\nbrown\ncow" | cat /tmp/cat_linter_file1 - /tmp/cat_linter_file2`
  Output matches
  Exit matches

Checking `echo "how\nnow\nbrown\ncow" | cat /tmp/cat_linter_file1 - /tmp/non-existent-file - /tmp/cat_linter_file2`
  Output matches
  Exit matches
```

[cat_linter]: https://github.com/caius/cat_linter

## License

`cargo run -q LICENSE`
