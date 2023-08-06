# Schools API
![Test](https://github.com/hillalex/teachertapp/actions/workflows/Test.yml/badge.svg?branch=main)
[![Project Status: Concept – Minimal or no implementation has been done yet, or the repository is only intended to be a limited example, demo, or proof-of-concept.](https://www.repostatus.org/badges/latest/concept.svg)](https://www.repostatus.org/#concept)

This is a very simple proof-of-concept API written in Rust, making use of the 
Axum and Diesel libraries. It uses `sqlite` as a database, which is totally unsuitable 
for a production context but serves the purposes of this demo!

## Requirements
* cargo

## Run
This project contains two binaries; one to generate JSON schema files from Rust structs, 
the other to start the API.
1. To generate JSON schemas:
        
        cargo run --bin api_schema

2. To start the API server on port 8080:

        cargo run --bin api

## Endpoints 
* nb these are sensitive to trailing slashes

### GET /
Returns a list of available endpoints.

### GET /school/
Returns a list of schools.\
Response schema: [schema/List.schema.json](schema/List.schema.json)

### GET /school/:id
Returns a single school.\
Response schema: [schema/School.schema.json](schema/School.schema.json)

### POST /school/
Create a new school.\
Request body schema: [schema/CreateSchool.schema.json](schema/CreateSchool.schema.json)\
Response body schema: [schema/School.schema.json](schema/School.schema.json)

### DELETE /school/:id
Deletes the school with the given id.

## Test
        cargo test

## TODO
To turn this into a production ready app, some starting points:
* add logging
* more fine-grained error handling
* integration tests that actually start the http server and test against it, including JSON schema validation
* non-sqlite database
* remove trailing slash sensitivity
* generate index route response automatically
* ...