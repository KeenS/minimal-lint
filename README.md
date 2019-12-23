This is tested against nightly-2019-12-22.
As compiler plugin is deprecated feature, this may not be compile in future rustc.

# Setup

```console
# install nightly
$ rustup update
# install rustc-dev
$ rustup component add rustc-dev
```


# Run

```console
$ cargo test
warning: item is named 'lintme'
 --> tests/lintme.rs:5:1
  |
5 | fn lintme() {}
  | ^^^^^^^^^^^^^^
  |
  = note: `#[warn(test_lint)]` on by default

    Finished test [unoptimized + debuginfo] target(s) in 0.74s
     Running target/debug/deps/minimal_lint-6460443341f748b2

```

