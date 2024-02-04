use axum::{
    extract::Query, // Query params
    Json, // JSON Extraction & Response
    http::StatusCode // Status codes
};
use super::db; // Database functions
use serde_derive::Deserialize; // Deserialize marco


#[derive(Deserialize)]
pub struct CarQuery { // struct to handle query params for details handler
    reg_num: String,
}

// Query to import the query params
pub async fn details(Query(query): Query<CarQuery>) -> Result<Json<String>, StatusCode> {

    let car = match db::get(query.reg_num)? {
        Some(val) => val,
        None => {
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let car_json = match serde_json::to_string(&car) { // Convert to JSON
        Ok(val) => val,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(car_json)) // Return the JSON string
}

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

pub async fn create(Json(input): Json<db::Car>) -> Result<StatusCode, StatusCode> {
    Ok(db::create(input)?)
}

pub async fn update(Json(new_car): Json<db::UpdateCar>) -> Result<StatusCode, StatusCode> {
    let old_car = db::get(new_car.reg_num.clone())?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(db::update(old_car, new_car)?)
}

pub async fn delete(Query(query): Query<CarQuery>) -> Result<StatusCode, StatusCode> {
    Ok(db::delete(query.reg_num)?)
}