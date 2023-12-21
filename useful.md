Useful stuff
============

cargo watch
-----------

Cargo watch is useful for reducing compilation time when working:

`cargo watch -x check -x test -x run`

cargo-watch monitors your source code to trigger commands every time a file changes:

`cargo watch -x check`

and to run tests and the server on every change:
`cargo watch -x check -x test -x run`


cargo testing
-------------

to run tests

`cargo test`

to run tests of specific module

`cargo test domain` replace 'domain' in the module name

for checking coverage for test it's only possible in linux currently with

`cargo install cargo-tarpaulin`

will compute code coverage for your application code, ignoring your test functions

`cargo tarpaulin --ingore-tests`

Linting
-------

The Rust team maintains clippy, the official Rust linter

`rustup component add clippy`

We can run manually clippy on the project with 

`cargo clippy`

To add it to the pipeline we can run

`cargo clippy -- -D warnings`


Formatting
----------

rustfmt is the official Rust formatter, 
If missing, you can easily install it with

`rustup component add rustfmt`

You can format your whole project with

`cargo fmt`

To add it to the CI pipeline we will add formatting step:

`cargo fmt -- --check`

Security Vulnerabilities
------------------------

They also provide cargo-audit11, a convenient cargo sub-command to check if vulnerabilities have been
reported for any of the crates in the dependency tree of your project.
You can install it with

`cargo install cargo-audit`

once installed run:

`cargo audit`


rust nightly
------------

Rust nightly is a build of rust compiler built from the main branch every night (thus the name),
it contains many tools that was not published yet.\
to install it we can run

`rustup toolchain install nightly --allow-downgrade`

Some components of the bundle installed by rustup might be broken/missing on the latest nightly release:
--allow-downgrade tells rustup to find and install the latest nightly where all the needed components are
available.


Dependencies
------------

To add a new dependency into the project run:

`cargo add <name>`

we can add the '@' in the end to choose the version and --features, for example:

`cargo add tokio@1 --features macros,rt-multi-thread`

Package to find unused dependencies:

`cargo +nightly udeps`
we might need to install nightly with `rustup install nightly`


Testing
-------

Integration tests
-----------------

Integration tests are located in a new folder something like

    src/
    tests/
        ...
    Cargo.toml
    Cargo.lock

Anything under the tests folder and your documentation tests, are compiled in
their own separate binaries.

Embedded/unit tests
-------------------

Are located in each file with special macro

`#[cfg(test)]`

An embedded test module is part of the project, just hidden behind a configuration conditional check.
