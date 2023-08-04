# Schools API

This is a very simple proof-of-concept API written in Rust, making use of the 
Axum and Diesel libraries. It uses sqlite as a database, which is totally unsuitable 
for a production context but serves the purposes of this demo.

## Requirements
* cargo

## Run
Start a server on port 3000 with:

        cargo run

## Endpoints 

### GET /
Returns a list of available endpoints.

### GET /school/:id
Returns a single school. 
Response schema: [schema/school.schema.json](schema/school.schema.json)

### POST /school
Create a new school.
Request body schema: [schema/createSchool.schema.json](schema/createSchool.schema.json)
Response body schema: [schema/school.schema.json](schema/school.schema.json)

## Test
        cargo test

## TODO
* add PATCH and DELETE endpoints
* add logging
* more fine-grained error handling
* end-to-end tests that actually start the http server and test against it
* non-sqlite database 
* 