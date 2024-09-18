# ulidtools

a simple tool to generate and parse ULID (Universally Unique Lexicographically Sortable Identifier) in Rust.

## Usage

1. Install: `cargo install --git https://github.com/noaione/ulidtools --branch master ulidtools`
2. Run: `ulidtools`

## Example

### Generate

```shell
ulidtools generate
```

### Parse

```shell
ulidtools parse 01J7Z9RSR9E9NBPTNNAFSD3M67
```

Or, with UUIDv7

```shell
ulidtools parse 0191fe9c-6709-726a-bb6a-b553f2d1d0c7
```

## License

MIT License
