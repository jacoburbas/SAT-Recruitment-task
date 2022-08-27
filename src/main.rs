#![allow(non_snake_case)]
#![allow(unused)]
#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::status;
use rand::Rng;

#[get("/?<distance>&<fuelUsagePer100KM>&<yearOfProduction>")]
fn calculateUsage(distance: u32, fuelUsagePer100KM: u32, yearOfProduction: u32) ->  String {

    let distance_float = distance as f32;
    let fuelUsage_float = fuelUsagePer100KM as f32;
    let fuelUsage: f32 =  fuelUsage_float/100.0 * distance_float;
     (fuelUsage.to_string())

}

#[get("/?<distance>&<fuelUsagePer100KM>&<yearOfProduction>", rank = 2)]
fn distance_err(distance: String, fuelUsagePer100KM: u32, yearOfProduction: u32) ->  status::BadRequest<String> {
    status::BadRequest(Some(format!("ERR: wrong type of 'distance' parameter: '{}', expected positive integer", distance)))
}

#[get("/?<fuelUsagePer100KM>&<yearOfProduction>", rank = 3)]
fn consumption_err(fuelUsagePer100KM: String, yearOfProduction: u32) ->  status::BadRequest<String> {
    status::BadRequest(Some(format!("ERR: wrong type of 'fuelUsagePer100KM' parameter: '{}', expected positive integer", fuelUsagePer100KM)))
}

#[get("/?<yearOfProduction>", rank = 4)]
fn year_err( yearOfProduction: String) ->  status::BadRequest<String> {
    status::BadRequest(Some(format!("ERR: wrong type of 'yearOfProduction' parameter: '{}', expected positive integer", yearOfProduction)))
}

#[get("/?<VIN>")]
fn probability( VIN: String) ->  String {
    let percentage = rand::thread_rng().gen_range(0..100);
    format!("0,{}", percentage)
}
#[get("/", rank = 2)]
fn no_VIN() ->  status::BadRequest<String> {
    status::BadRequest(Some(format!("ERR: no VIN, try: /probabilityOfUnitInjectorFail?<VIN>")))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/calculateDisselUsageForDistance", routes![calculateUsage,distance_err,consumption_err,year_err])
    .mount("/probabilityOfUnitInjectorFail", routes![probability,no_VIN])

}