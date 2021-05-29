# OpenController Server

A server for OpenController house specifications.

## Development

1. Clone the repository

        git clone https://github.com/Open-Controller/server.git
        cd ./server

2. Run with an ocbin file

        cargo run ./test/house.ocbin

3. Run tests

        cargo test

## Installation

1. Clone the repository

        git clone https://github.com/Open-Controller/server.git
        cd ./server

2. Install

        cargo install

## Usage

    opencontroller-server [OPTIONS] <input>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -v <verbosity>        Sets the level of verbosity [default: INFO]

    ARGS:
        <input>    Sets the input file to use

### Environment Variables

- PORT: Port to serve on, default 3612
