use weather_util_rust::{
    weather_opts::*,
    config::Config,
};
use anyhow::Error;
use assert_cmd::{cargo::cargo_bin, Command};
use log::info;
use std::{
    path::Path,
    ops::Deref,
};
use envy::*;
use dotenvy::{
    vars,
    *,
};
#[tokio::main]

async fn main() {
    //init_config function takes type Option<Path>
    let path = Path::new("C:\\Users\\thomp\\USERPROFILE\\projects\\weather_util_test_1_0\\src\\config.env");\
    let test_pkg = Config::init_config(Some(path)).unwrap();
    //type config inner, holds env data
    let inner = test_pkg.deref();
    //type weather opts holds weather data
    let opts = WeatherOpts::parse_opts(&test_pkg);
    println!("Out put is {:?}", opts.await.unwrap());
    println!("Output is {:?}", inner);
}
