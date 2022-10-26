# no

Like `yes`, but different.

## Usage

Execute the binary without any arguments to output the letter `n` endlessly, or
use the `--help` flag for help and exit:

```bsah
$ no --help
Like yes, but different

Usage: no [PATTERN]...

Arguments:
  [PATTERN]...  Optional string to output

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## Improvements over `yes`

When killed, `yes` exits with non-zero error. `no`, on the other end, when
terminated by the user exits with `0`.

## License

`no` is maintained under GNU GPLv2. See [LICENSE](LICENSE) file in this
directory for full preamble.
