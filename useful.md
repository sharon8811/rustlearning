Cargo watch is useful for reducing compilation time when working:

`cargo watch -x check -x test -x run`

cargo-watch monitors your source code to trigger commands every time a file changes:

`cargo watch -x check`

and to run tests and the server on every change:
`cargo watch -x check -x test -x run`

