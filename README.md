# Rust GraphQL Server 

Author: Pavel Boukine
Date: 2023-10-19
For: Noibu Technologies.

This is a Rust-based GraphQL server template that provides a starting point for building GraphQL APIs for user data. It utilizes the async-graphql and warp libraries, making it suitable for asynchronous operations and serving GraphQL queries over HTTP.

## Features

- Defines a GraphQL schema with a `User` type.
- Implements a `QueryRoot` resolver for fetching user information by ID.
- Exposes a GraphQL endpoint for querying user data.
- Includes integration tests for GraphQL queries and the GraphQL Playground.
- Serves as a template for Rust GraphQL server projects.

## Getting Started

### Prerequisites

To run this program, you need to have the following installed:

- Rust (https://www.rust-lang.org/tools/install)
- Cargo (Rust's package manager)
- Git (for version control)

### Installation

1. Clone the repository to your local machine:

   ```bash
   git clone <https://github.com/pavelboukine/Rust_Takehome>

2. Change into the project directory: cd rust-graphql-server
3. Build the project: cargo build

### Usage

1. Start the GraphQL server: cargo run
2. Access the GraphQL Playground:
Open a web browser and navigate to http://localhost:3030/graphql to explore and test the GraphQL API interactively.
Example query to return user by ID "1":

{
  userById(id: "1") {
    id
    name
    email
  }
}

### Integration Tests
Please note: The tests WILL fail due to errors in the code.

You can run the included integration tests with the following command: cargo test

### GraphQL Schema

The GraphQL schema includes the following type:

User: Represents a user with fields like id, name, and email.

The schema also includes a single query:

user_by_id(id: String): Fetches user information by providing a user ID.

### Acknowledgments

Thanks to the Rust community for creating and maintaining the async-graphql and warp libraries, which make building Rust-based GraphQL servers easier.