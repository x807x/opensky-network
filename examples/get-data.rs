use chrono::{Local, SecondsFormat};
use colored::Colorize;
use log::{error, info, LevelFilter};
use opensky_api::datasets::aircrafts::{get_aircraft_data, Aircraft};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .format(|buf, record| {
            let time = Local::now()
                .to_rfc3339_opts(SecondsFormat::Millis, true)
                .as_str()
                .bright_blue();
            let level = record.level().as_str();
            let colored_level = match record.level().to_level_filter() {
                LevelFilter::Info => level.green(),
                LevelFilter::Warn => level.yellow(),
                LevelFilter::Error => level.red(),
                _ => level.into(),
            };
            writeln!(buf, "{} [{}] - {}", time, colored_level, record.args(),)
        })
        .filter(None, LevelFilter::Debug)
        .init();

    /*
    let pb = ProgressBar::new(total_size).with_message(format!("Downloading: {}", filename));
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} {msg:.blue} ({bytes_per_sec}) [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} [{elapsed_precise}/{eta_precise}]"
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    ); */

    let aircraft_data = get_aircraft_data(9, 2024).await;
    let mut taiwan_aircrafts: Vec<Aircraft> = Vec::new();
    match aircraft_data {
        Ok(aircrafts) => {
            for aircraft in aircrafts {
                if aircraft.registration == Some("B-18917".to_string()) {
                    info!("Found: {:?}", aircraft);
                }
                if aircraft.country == Some("Taiwan".to_string()) {
                    info!("{:?}", aircraft);
                    taiwan_aircrafts.push(aircraft);
                }
            }
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }

    for aircraft in &taiwan_aircrafts {
        info!("{:?}", aircraft);
    }
    info!("Total aircrafts in Taiwan: {}", taiwan_aircrafts.len());
    Ok(())
}
