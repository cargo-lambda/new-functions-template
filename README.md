# Introduction

This repository contains a template for a Rust project that implements an AWS Lambda function.

# Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo Lambda](https://www.cargo-lambda.info/guide/installation.html)

# Usage

This is the default functions template that Cargo Lambda uses starting from version 1.5.0. This template doesn't work with older versions of Cargo Lambda.

To create a new project with this template you can simply run `cargo lambda new PROJECT_NAME`. That will download this template and create a new project with the name `PROJECT_NAME`.

If you're making changes to this template, you can run `cargo lambda new --template ./path/to/template PROJECT_NAME` to create a new project with the changes you made to the template.

If you want to create a new project with a specific branch of this repository, you can run `cargo lambda new --template https://github.com/cargo-lambda/new-functions-template/tree/BRANCH_NAME`.

Read more about Cargo Lambda Templates in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/new.html#templates).