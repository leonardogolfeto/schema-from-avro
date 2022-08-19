# Struct From Avro

[![Build Status](https://travis-ci.org/joemccann/dillinger.svg?branch=master)](https://travis-ci.org/joemccann/dillinger)

Schema from avro is a project that aims to speed up the development process for rust systems that use avro schema, removing the need to manually map a schema to a struct, thus mitigating probable errors at development time.

## Features

- Generate Rust structs from a avro schema (avsc file or json)

## Usage

```sh
sruct-from-avro your_schema.avsc > your_rust_file.rs
```

## Installation

```sh
cargo install sruct-from-avro
```

## Development

Want to contribute? Great!

it's as simple as coding and opening a pull request!

## License

MIT

**Free Software, Hell Yeah!**
