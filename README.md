# Setup
**IMPORTANT** This setup doesn't work for Windows 10 due to missing "libpq" dependecy for diesel client. Possible solution to fix this is yet to be found

## Make sure you have Rust and cargo installed
- Also make sure you are using "stable" toolchain

Installing Rust
https://www.rust-lang.org/tools/install

## Installing Postgres

Ubuntu:
https://www.digitalocean.com/community/tutorials/how-to-install-postgresql-on-ubuntu-20-04-quickstart

Windows 10:
https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql/
[Required administrator]

## Setting up postgres user-schema

Start of by creating a new user

`CREATE USER <name> WITH PASSWORLD '<password>'`

Next create database for the project

`CREATE DATABASE koodi101`

Grant access to the user you just created

`GRANT ALL PRIVILEGES ON DATABASE koodi101 TO <name>`

## Cloning the repository
Run the following command to copy your fork of this repository

`git clone https://github.com/<your_fork>`

Open the project in VS Code with the command

`code ./koodi-101-rust-backend`

## Installing dependencies

If you are on Ubuntu, you most likely don't have libpq installed.

`sudo apt-get install libpq-dev`

*This doesn't sadly work on windows

## Installing and running Diesel client

We are using diesel client to configure our Postgres database.

`cargo install diesel_cli --no-default-features --features postgres`

You can use the commands in "/migrations" with the client. This command will run the "up.sql"

`diesel migration run \
  --database-url postgresql://<user>:<password>@localhost:5432/koodi101`

## Creating the Rocket.toml config

Rocket will read the database url from "/Rocket.toml". Start by creating the file in your project folder

`touch Rocket.toml`

Copy the following inside it with your credentials

```toml
[default.databases.koodi101]
url = "postgres://<user>:<password>@localhost/koodi101"
```

## Finally, running the project

Run the project with

`cargo run`

## TODOS
- validate this setup actually works
- somehow make this available for Windows