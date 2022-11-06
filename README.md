# OpenController Server

A server for OpenController house specifications.

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

## [Contributing](CONTRIBUTING.md)

## License

        Copyright (C) 2022 PJTSearch

        This program is free software: you can redistribute it and/or modify
        it under the terms of the GNU Affero General Public License as
        published by the Free Software Foundation, either version 3 of the
        License, or (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU Affero General Public License for more details.

        You should have received a copy of the GNU Affero General Public License
        along with this program.  If not, see <https://www.gnu.org/licenses/>.
