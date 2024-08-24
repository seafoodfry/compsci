# compsci

# Setup

## Rust

Go to [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

To keep things up-to-date run
```
rustup check
```


## Go

In mac,
```
brew install go
```
See [formulae.brew.sh/formula/go](https://formulae.brew.sh/formula/go).

Then check for updates with
```
brew outdated
```
and
```
brew upgrade
```

In other systems, follow the instructions in [go.dev/doc/install](https://go.dev/doc/install).


## Python

[Poetry](https://python-poetry.org/docs/basic-usage/) seems to be the goto.
We are installing it as

```
python3 -m venv ~/.poetry
~/.poetry/bin/pip install poetry
```