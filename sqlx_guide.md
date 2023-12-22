sqlx Guide
==========

installing sqlx cli

`cargo install --version="0.7.3" sqlx-cli --no-default-features 
--features rustls,postgres`

sqlx cli help

`sqlx --help`

The first command we will usually want to run is sqlx database create.
According to the help docs \
we will need it only if we do not have DB in postgres

`sqlx database create`

it relays on env:

`DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/`

To create the first migration

    export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
    sqlx migrate add create_subscriptions_table

A new top-level directory should have now appeared in your project - migrations. This is where all
migrations for our project will be stored by sqlx’s CLI. \
Under migrations you should already have one file called `{timestamp}_create_subscriptions_table.sql`

Running migrations
------------------

`sqlx migrate run`


Building docker images with sqlx
================================

sqlx calls into our database at compile-time to ensure that all queries can be successfully executed considering
the schemas of our tables.\
When running `cargo build` inside our Docker image, though, sqlx fails to establish a connection with the
database that the DATABASE_URL environment variable in the .env file points to.

Offline mode with sqlx
----------------------
`prepare` performs the same work that is usually done when cargo build is invoked but it
saves the outcome of those queries into a directory (.sqlx) which can later be detected by sqlx itself and
used to skip the queries altogether and perform an offline build.

`cargo sqlx prepare --workspace`

We can then set the `SQLX_OFFLINE` environment variable to `true` in our Dockerfile to force sqlx to look at
the saved metadata instead of trying to query a live database

We can use the `--check` flag in CI pipelines to ensure that it stays up-to-date.