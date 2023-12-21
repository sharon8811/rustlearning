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