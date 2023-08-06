# Schools API

This is a very simple proof-of-concept API written in Rust, making use of the 
Axum and Diesel libraries. It uses sqlite as a database, which is totally unsuitable 
for a production context but serves the purposes of this demo.

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
Returns a list of schools.
Response schema: [schema/List.schema.json](schema/List.schema.json)

### GET /school/:id
Returns a single school. 
Response schema: [schema/School.schema.json](schema/School.schema.json)

### POST /school/
Create a new school.
Request body schema: [schema/CreateSchool.schema.json](schema/CreateSchool.schema.json)
Response body schema: [schema/School.schema.json](schema/School.schema.json)

## Test
        cargo test

## TODO
* add PATCH and DELETE endpoints
* add logging
* more fine-grained error handling
* integration tests that actually start the http server and test against it, including JSON schema validation
* non-sqlite database
* remove trailing slash sensitivity
* generate index route response automatically
* ...