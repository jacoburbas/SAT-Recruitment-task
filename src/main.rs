#![allow(non_snake_case)]
#![allow(unused)]
#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::status::{self, BadRequest};
use rand::Rng;

//calculateDisselUsageForDistance

#[get("/?<distance>&<fuelUsagePer100KM>&<yearOfProduction>")]
fn calculateUsage(distance: String, fuelUsagePer100KM: String, yearOfProduction: String) ->  status::Custom<String> {

    if !distance.chars().all(char::is_numeric){
        status::Custom(Status::BadRequest, format!("ERR: wrong type of 'distance' parameter: '{}', expected positive integer", distance))
    }
    else if !fuelUsagePer100KM.chars().all(char::is_numeric){
        status::Custom(Status::BadRequest, format!("ERR: wrong type of 'yearOfProduction' parameter: '{}', expected positive integer", fuelUsagePer100KM))
    }
    else if !yearOfProduction.chars().all(char::is_numeric) || yearOfProduction.len() != 4{
        status::Custom(Status::BadRequest, format!("ERR: wrong type or length of 'fuelUsagePer100KM' parameter: '{}', expected positive integer, 4 characters long", yearOfProduction))
    }
    else {
        let distance_float: f32 = distance.parse::<u32>().unwrap() as f32;
        let fuelUsageper100KM_float: f32 = fuelUsagePer100KM.parse::<u32>().unwrap() as f32;
        let fuelUsage = fuelUsageper100KM_float/100.0 * distance_float; 
        status::Custom(Status::Ok, format!("{}", fuelUsage))
    }
}

#[get("/", rank = 2)]
fn err( ) ->  status::BadRequest<String> {
    status::BadRequest(Some(format!("ERR: all three parameters must be given, try: /calculateDisselUsageForDistance?distance=<distance>&yearOfProduction=<yearOfProduction>&fuelUsagePer100KM=<fuelUsagePer100KM>")))
}

//probabilityOfUnitInjectorFail

#[get("/?<VIN>")]
fn probability( VIN: String) ->  status::Custom<String> {
    if  VIN.bytes().all(|b| matches!(b, b'0'..=b'Z')) && VIN.len() == 17{
    let percentage: f32 = rand::thread_rng().gen_range(0.0..1.0);
    status::Custom(Status::Ok, format!("{:.2}", percentage))
    }
    else {
    status::Custom(Status::BadRequest, format!("VIN number: '{}' should be 17 characters long, made of only uppercase letters and numbers", VIN))
    }
}

#[get("/", rank = 2)]
fn no_VIN() ->  status::BadRequest<String> {
    status::BadRequest(Some(format!("ERR: no VIN, try: /probabilityOfUnitInjectorFail?VIN=<VIN>")))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/calculateDisselUsageForDistance", routes![calculateUsage,err])
    .mount("/probabilityOfUnitInjectorFail", routes![probability,no_VIN])
}