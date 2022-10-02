# PatternFly quick start for Yew

This is a prototype for a user friendly blockchain explorer which allows for a "lego" like experience of constructing of complex data queries.

## Pre-requisites

* Rust

      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

* Trunk

      cargo install trunk

* NodeJS `npm`

## Initialize

Fetch the PatternFly dependencies:

    npm install

## Run local developer setup

Start a local development server, which re-builds every time you make changes to the code:

    trunk serve

Direct your web browser to: http://localhost:8080

## Perform a release build

To build the Rust components and package up the page, run:

    trunk build

The release is in the `dist/` folder.
