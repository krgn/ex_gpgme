# ExGpgme

Simple wrapper around `gpgme` for Elixir/Erlang.

*Note*: this is a work in progress! Use at your own discretion.

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `ex_gpgme` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:ex_gpgme, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/ex_gpgme](https://hexdocs.pm/ex_gpgme).

## Todo

- implement `export_key`
- implement `create_key`
- implement `encrypt_and_sign`
- implement `decrypt_and_verify`
- implement `sign`
- implement `verify`
- asynchronous encryption/decryption
