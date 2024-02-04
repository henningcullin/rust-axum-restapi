# Rust Axum RESTful API

In this project I explore the Axum framework for Rust.

My application is a RESTful API connected to a CRUD app

This application is built in and for Windows 10 but the only difference for other OS's should be where the json file is stored.

## 1. Examples

Here are some examples of how to do things in the axum framework.

The examples are supposed to be a minimal boilerplate solution.

Remember that the examples are only guarranteed to work for the specified crate versions. Changes may have occured to syntax.

### 1.1 Hello World Example

Cargo.toml:
```toml
axum = "0.7.4"
tokio = { version = "1.0", features = ["full"] }
```

```rust
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### 1.2 Routing Example

Here is an example of how I set up routes

Naturally we will move the controllers to a module later

Cargo.toml:
```toml
axum = "0.7.4"
tokio = { version = "1.0", features = ["full"] }
```

```rust

use axum::{
    routing::{ // import the methods using in your routes
        get,
        delete
    }
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/", delete(bye));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> String {
    "Hello".to_string()
}

async fn bye() {
    // some delete operation
}

```


### 1.3 Query String Example

In this example we have a function called hello which will return the query string of the same name's value.

To do this we need to add the serde & serde_derive crates for the Deseralize macro

Cargo.toml:
```toml
axum = "0.7.4"
tokio = { version = "1.0", features = ["full"] }
serde = "1.0.196"
serde_derive = "1.0.196"
```

```rust

use axum::{
    routing::{ // For the methods
        get,
    },
    Router, // For the router
    extract::Query, // To extract query strings
};

use serde_derive::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct QueryStrings {
    name: String // The query string name and type
}

// Here we will take in the querystring struct and return a String
async fn hello(Query(queries): Query<QueryStrings>) -> String {
    // Not having any ; makes the format! value the return value
    format!("Hello, {}!", queries.name)
}

```

You can have multiple query strings in the same struct

```rust

#[derive(Deseralize)]
struct Person {
    first_name: String,
    last_name: String,
    age: usize
}

```

To access them you just get the property of the struct

```rust
async fn register(Query(person): Query<Person>) {
    let first_name = person.first_name;
}
```

### 1.4 Post & Status Code Example

Cargo.Toml:
```toml
axum = "0.7.4"
tokio = { version = "1.0", features = ["full"] }
serde = "1.0.196"
serde_derive = "1.0.196"
```



```rust

use axum::{
    routing::post,
    Router,
    Json, // Allows us to handle json in requests and responses
    http::StatusCode // Lets us return status codes
};

use serde_derive::Deserialize;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", post(register));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct Person {
    first_name: String,
    last_name: String,
    age: usize
}

async fn register(Json(person): Json<Person>) -> StatusCode {
    // create functionality

    // You can access the post data in the person parameter

    // example if you have a module called db which has a create function

    //db::create(person);

    StatusCode::CREATED
}

```



## 2 Documentation


### 2.1 Cargo.toml

```toml
[package]
name = "axum-rest-api"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = "0.7.4"
tokio = { version = "1.0", features = ["full"] }
serde = "1.0.196"
serde_json = "1.0.113"
serde_derive = "1.0.196"
```

- axum is the framework
- tokio is the asynchronous runtime
- serde is the json crate for rust
- serde_json allows us to to_string our structs
- serde_derive allows us to add macros to convert in our handler/controller functions

### 2.2 main.rs

#### Imports

```rust
mod cars; // Car handlers
mod db; // Database functions

use axum::{ // Framework
    routing::{ // HTTP Methods
        get,
        post,
        put,
        delete
    }, Router // The Router
};
```

#### Main

```rust
#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/car", get(cars::details))
        .route("/cars", get(cars::index))
        .route("/car", post(cars::create))
        .route("/car", put(cars::update))
        .route("/car", delete(cars::delete));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### 2.3 cars.rs

#### Imports

```rust
use axum::{
    extract::Query, // Query params
    Json, // Json Extraction & Response
    http::StatusCode // Status codes
};
use super::db; // Database functions
use serde_derive::Deserialize; // Deserialize marco

```

#### details / show / get Function

We want the details of a specific car and ask the api for it like this: 127.0.0.1:80/car?reg_num=ABC123

```rust
#[derive(Deserialize)]
pub struct CarQuery { // struct to handle query params for details handler
    reg_num: String,
}

// Query to import the query params
// We return a json string of the cars details
pub async fn details(Query(query): Query<CarQuery>) -> Result<Json<String>, StatusCode> {

    // Send the StatusCode error to main
    let car = match db::get(query.reg_num)? {
        Some(val) => val, // If there is a car, store it
        None => {
            return Err(StatusCode::NOT_FOUND); // Else return 404 Error code
        }
    };

    let car_json = match serde_json::to_string(&car) { // Convert the retrieved car to a json string
        Ok(val) => val,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(car_json)) // Return the Json string
}
```


#### index / show_all / get_all Function

We ask the database for all cars, if we get something we Serialize it to a JSON String.

```rust
pub async fn index() -> Result<Json<String>, StatusCode>{

    let cars = db::get_all()?;

    let cars_json = match serde_json::to_string(&cars) {
        Ok(val) => val,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    Ok(Json(cars_json))
}
```

#### create / add / new Function

Self explanatory.

```rust
pub async fn create(Json(input): Json<db::Car>) -> Result<StatusCode, StatusCode> {
    Ok(db::create(input)?)
}
```

#### update Function

We send the new car, which only contains the reg_num and the NEW data we want to add / change.

```rust
pub async fn update(Json(new_car): Json<db::UpdateCar>) -> Result<StatusCode, StatusCode> {
    let old_car = db::get(new_car.reg_num.clone())?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(db::update(old_car, new_car)?)
}
```

#### delete Function

Self explanatory.

```rust
pub async fn delete(Query(query): Query<CarQuery>) -> Result<StatusCode, StatusCode> {
    Ok(db::delete(query.reg_num)?)
}
```