use std::io::Cursor;

use serde::{Deserialize, Serialize};

use crate::errors::Error;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Aircraft {
    pub icao24: String,
    pub timestamp: String,
    pub acars: u8,
    pub adsb: u8,
    pub built: Option<String>,
    #[serde(rename = "categoryDescription")]
    pub category_description: String,
    pub country: Option<String>,
    pub engines: Option<String>,
    pub first_flight_date: Option<String>,
    #[serde(rename = "firstSeen")]
    pub first_seen: Option<u64>,
    #[serde(rename = "icaoAircraftClass")]
    pub icao_aircraft_class: Option<String>,
    #[serde(rename = "lineNumber")]
    pub line_number: Option<String>,
    #[serde(rename = "manufacturerIcao")]
    pub manufacturer_icao: Option<String>,
    #[serde(rename = "manufacturerName")]
    pub manufacturer_name: Option<String>,
    pub model: Option<String>,
    pub modes: u8,
    #[serde(rename = "nextReg")]
    pub next_register: Option<String>,
    pub operator: Option<String>,
    #[serde(rename = "operatorCallsign")]
    pub operator_callsign: Option<String>,
    #[serde(rename = "operatorIata")]
    pub operator_iata: Option<String>,
    #[serde(rename = "operatorIcao")]
    pub operator_icao: Option<String>,
    pub owner: Option<String>,
    #[serde(rename = "prevReg")]
    pub previous_register: Option<String>,
    #[serde(rename = "regUntil")]
    pub reg_until: Option<String>,
    pub registration: Option<String>,
    #[serde(rename = "selCal")]
    pub sel_cal: Option<String>,
    pub serial_number: Option<u64>,
    pub status: Option<String>,
    pub typecode: Option<String>,
    pub vdl: u8,
}

pub async fn get_aircraft_data(month: u8, year: u16) -> Result<Vec<Aircraft>, Error> {
    let filename = format!("aircraft-database-complete-{year}-{month:02}.csv");
    let url = format!("https://s3.opensky-network.org/data-samples/metadata/{filename}");
    debug!("url: {url}");
    let data = load_data_normalize(&filename, &url).await?;

    let mut reader = csv::Reader::from_reader(Cursor::new(data));
    debug!("Parsing CSV data");
    let aircrafts: Vec<Aircraft> = reader.deserialize::<Aircraft>().collect::<Result<_, _>>()?;

    Ok(aircrafts)
}
