/*
Author: Pavel Boukine
Date: 2023-10-19
For: Noibu Technologies.

Application Info:
-Rust-based GraphQL server
-Provides a GraphQL API for user data
-Defines a GraphQL schema with a User type
-Includes a QueryRoot resolver for fetching user information by ID
-Utilizes async-graphql and warp libraries
-Contains integration tests for GraphQL schema and resolver functions
-Serves as a template for Rust GraphQL server projects.

*/

// Import necessary libraries and modules
use async_graphql::{EmptyMutation, EmptySubscription, Object, Result, Schema};
use async_graphql_warp::graphql;
use warp::{Filter, Rejection};
use async_graphql::http::playground_source;
use async_graphql::http::GraphQLPlaygroundConfig;

// Define a User struct to represent a user with id, name, and email fields
#[derive(Clone)]
struct User {
    id: String,
    name: String,
    email: String,
}

// Implement GraphQL Object for the User struct
#[Object]
impl User {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn email(&self) -> &str {
        &self.email
    }
}

// Define a QueryRoot struct for handling GraphQL queries
struct QueryRoot;

// Implement GraphQL Object for the QueryRoot struct
#[Object]
impl QueryRoot {
    async fn user_by_id(&self, id: String) -> Result<Option<User>> {
        // Simulate data retrieval 
        let user1 = User {
            id: "1".to_string(),
            name: "Pavel".to_string(),
            email: "Pavelboukine@gmail.com".to_string(),
        };
        let user2 = User {
            id: "2".to_string(),
            name: "Charlie".to_string(),
            email: "charlie.gracie@noibu.com".to_string(),
        };

        // Return a user based on the provided ID
        match id.as_str() {
            "1" => Ok(Some(user1)),
            "2" => Ok(Some(user2)),
            _ => Ok(None),
        }
    }
}

#[tokio::main]
async fn main() {
    // Build the GraphQL schema with QueryRoot, EmptyMutation, and EmptySubscription
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

// Create a GraphQL endpoint using Warp
let graphql_endpoint = warp::path("graphql")
    .and(warp::post())
    .and(graphql(schema).and_then(|(schema, request): (Schema<QueryRoot, EmptyMutation, EmptySubscription>, async_graphql::Request)| async move {
        let response = schema.execute(request).await;  // Execute the GraphQL request
        Ok::<_, Rejection>(warp::reply::json(&response))  // Convert the response to JSON
    }));

// Create a GraphQL Playground route
let playground = warp::path("graphql")
    .and(warp::get())
    .map(|| warp::reply::html(playground_source(GraphQLPlaygroundConfig::new("/graphql"))));

 // Combine GraphQL endpoint and Playground routes into a single Warp filter
    let routes = warp::any().and(graphql_endpoint.or(playground));

    // Serve the routes on the specified address and port
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Integration tests

// Define a test for GraphQL queries
use async_graphql::Request;

#[tokio::test]
async fn test_graphql_query() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    // Create a request for the "user_by_id" query
    let request = Request::new(r#"{ "query": "{ user_by_id(id: \"1\") { id, name, email } }" }"#);

    // Simulate a GraphQL query by executing the request against the schema
    let response = schema.execute(request).await;

    // Assert that the response is successful
    assert_eq!(response.is_ok(), true);

    // Convert the async_graphql::Value to serde_json::Value
    let response_data = serde_json::to_value(response.data).expect("Failed to convert response to JSON");

    // Assert that the response data matches the expected JSON
    let expected_response = r#"{
        "data": {
            "user_by_id": {
                "id": "1",
                "name": "Pavel",
                "email": "Pavelboukine@gmail.com"
            }
        }
    }"#;
    assert_eq!(response_data, serde_json::json!(expected_response));
}

// Define a test for the GraphQL Playground route
#[tokio::test]
async fn test_graphql_playground() {
    // Create a Warp filter for the playground route
    let playground_filter = warp::path("graphql")
        .and(warp::get())
        .map(|| warp::reply::html(playground_source(GraphQLPlaygroundConfig::new("/graphql"))));

    // Simulate a GET request to the playground route
    let response = warp::test::request()
        .method("GET")
        .path("/graphql")
        .reply(&playground_filter)
        .await;

    // Assert that the response contains the expected HTML content
    let expected_content = r#"<!DOCTYPE html>
    <!-- ... Include the expected HTML content of the playground ... -->
</html>"#;
    assert_eq!(response.body(), expected_content.as_bytes());
}