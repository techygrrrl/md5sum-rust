# md5 CLI in Rust

The following is just a quick and dirty MD5 proof of concept CLI tool in Rust.

## Usage

    md5sum 0.1.0  Usage: md5sum --value <VALUE> Options:
      -v, --value <VALUE>  The value you'd like to convert to an MD5 hash
      -h, --help           Print help information
      -V, --version        Print version information`

## Running the app

You can run the proof of concept by doing the following:

    cargo run -- --value techygrrrl

You can put any value in place of `techygrrrl`.

You should see the following output:

    md5("techygrrrl") = 49a7d6fdbc5aa48c3f0b3b674693e2dc

If you do not pass the input value properly, you will see the following error:

    error: The following required arguments were not provided:
      --value <VALUE>

    Usage: md5sum --value <VALUE>

    For more information try '--help'

## Building

If you'd like to build a release binary you can run the following command:

    cargo build --release

Then you can execute the release build:

    ./target/release/md5sum --value techygrrrl

If you'd like to use this utility locally, you can create an alias for it in your `.zshrc` or `.bash_profile` file:

```sh
alias md5sum="<code-directory>/md5sum/target/release/md5sum --value "
```

Source your `.zshrc` or `.bash_profile` and then use it like this:

```sh
md5sum techygrrrl
# md5("techygrrrl") = 49a7d6fdbc5aa48c3f0b3b674693e2dc
```

## Dependencies

| Crate                                     | Purpose                                                               |
| ----------------------------------------- | --------------------------------------------------------------------- |
| [md5](https://docs.rs/md5/latest/md5/)    | Generates the MD5 hex digest from the input                           |
| [clap](https://docs.rs/clap/latest/clap/) | Parses command line arguments, provides logging output for CLI usage. |
