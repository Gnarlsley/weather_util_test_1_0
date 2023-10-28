use weather_util_rust::{
    weather_opts::*,
    config::Config,
    weather_api::WeatherApi,
    weather_api::WeatherLocation,
};
use anyhow::Error;
use assert_cmd::{cargo::cargo_bin, Command};
use log::info;
use std::{
    path::Path,
    ops::Deref,
};
use std::env;
use dotenvy::*;

#[tokio::main]

async fn main() {
    //init_config function takes type Option<Path>
    let path = Path::new("C:\\Users\\thomp\\USERPROFILE\\projects\\weather_util_test_1_0\\src\\config.env");
    let test_pkg = Config::init_config(Some(path)).unwrap();

    dotenv().ok();

    //type config inner, holds env data
    let inner = test_pkg.deref();

    //type weather opts holds weather data
    let w1 = WeatherOpts::parse_opts(&test_pkg);
    //println!("{:?}", w1.await.unwrap());

    //let test = WeatherOpts::get_location(&w1.await);
    //println!("{:?}", test);

    let mut w2 = WeatherOpts::default();
    let test = WeatherOpts::get_location(&w2);

    //w2.api_key = test_pkg.api_key.clone();

    let api = WeatherApi::new(&env::var("API_KEY").unwrap(), &env::var("API_ENDPOINT").unwrap(), &env::var("API_PATH").unwrap(), &env::var("GEO_PATH").unwrap());
    let zipcode_location = WeatherLocation::from_zipcode(73301);
    let city_location = WeatherLocation::from_city_name("Austin, Texas");


    let data = api.get_weather_data(&zipcode_location).await.unwrap();

    let w3 = data.get_current_conditions();

    println!("{}", w3);
}
