# mysqldump-splitter

Split gigantic mysql dumps into smaller one in a human manageable directory structure.

Output files can be optionally be compressed (gzip).

## Installation

This tool is not yet published to crates.io.

````bash
cargo install --git https://github.com/Scoopit/mysqldump-splitter.git
````

## Usage

````
USAGE:
    mysqldump-splitter [OPTIONS] --output <OUTPUT>

OPTIONS:
    -c, --compress
            Compress each output file.

            Output .gz gzipped compressed files instead of plain text .sql files

    -h, --help
            Print help information

    -i, --input <INPUT>
            Read this file instead of the standard input

    -o, --output <OUTPUT>
            Output directory

    -V, --version
            Print version information
````

## Directory structure

The first lines of the dump, before any database creation or table creation is output
to:

````
{output_dir}/00_header.sql
````

Create database & use statements are output in

````
{output_dir}/{db}/00_create_database.sql
````

Each table is output in the following file if a database has been created before:

````
{output_dir}/{db}/{table_name}.sql
````

If no database creation is present in the dump, tables are output to:

````
{output_dir}/{table_name}.sql
````

If the `compress` flag is set, all files are gzipped and  `.gz` extension is appended
to their name.

## Known Issues

- procedures or triggers are appended to the last table of a database
- comments do not match files (eg. the create database file
  contains comments for the first table in the db)

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
