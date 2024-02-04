use std::fs;
use std::env;
use std::path::Path;
use axum::http::StatusCode;

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Car {
    reg_num: String,
    brand: String,
    model: String,
    year: u16,
    color: String
}

impl Car {
    pub fn update_with(&self, new: UpdateCar) -> Car {
        Car {
            reg_num: self.reg_num.clone(),
            brand: new.brand.unwrap_or_else(|| self.brand.clone()),
            model: new.model.unwrap_or_else(|| self.model.clone()),
            year: new.year.unwrap_or_else(|| self.year),
            color: new.color.unwrap_or_else(|| self.color.clone()),
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateCar {
    pub reg_num: String,
    brand: Option<String>,
    model: Option<String>,
    year: Option<u16>,
    color: Option<String>
}

fn read_file() -> Result<String, StatusCode> {

    let user_profile = match env::var("USERPROFILE") {
        Ok(val) => val,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let file_path = Path::new(&user_profile).join("Documents").join("cars.json");

    let data: String = match fs::read_to_string(file_path) {
        Ok(val) => val,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
    Ok(data)
}

fn save_file(cars: Vec<Car>) -> Result<(), StatusCode> {

    let user_profile = match env::var("USERPROFILE") {
        Ok(val) => val,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let file_path = Path::new(&user_profile).join("Documents").join("cars.json");

    let data_json = serde_json::to_string_pretty(&cars).expect("Failed to parse to JSON");

    match fs::write(file_path, data_json) {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
} 

pub fn get_all() -> Result<Vec<Car>, StatusCode>{

    let data = read_file()?; // Send the status code upwards

    let cars: Vec<Car> =  match serde_json::from_str(&data) {
        Ok(val) => val,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(cars)
}

pub fn get(reg_num: String) -> Result<Option<Car>, StatusCode> {

    let cars = get_all()?;

    Ok(cars.iter().find(|car| car.reg_num == reg_num).cloned())
}

pub fn create(car: Car) -> Result<StatusCode, StatusCode> {

    match get(car.reg_num.clone())? {
        Some(_) => return Err(StatusCode::CONFLICT),
        None => {}
    };

    let cars = match get_all() {
        Ok(mut val) => {
            val.push(car);
            val
        }
        Err(status) => return Err(status)
    };

    save_file(cars)?;
    Ok(StatusCode::CREATED)
}

pub fn update(old_car: Car, new_car: UpdateCar) -> Result<StatusCode, StatusCode>{

    let updated_car: Car = old_car.update_with(new_car);

    let mut cars: Vec<Car> = get_all()?;

    if let Some(index) = cars.iter().position(|car| car.reg_num == updated_car.reg_num) {
        cars[index] = updated_car;
    } else {
        return Err(StatusCode::NOT_FOUND);
    }

    save_file(cars)?;
    Ok(StatusCode::CREATED)
}

pub fn delete(reg_num: String) -> Result<StatusCode, StatusCode> {

    get(reg_num.clone())?.ok_or(StatusCode::NOT_FOUND)?;

    let cars: Vec<Car> = get_all()?;

    save_file(cars.into_iter().filter(|car| car.reg_num != reg_num).collect())?;

    Ok(StatusCode::OK)

}