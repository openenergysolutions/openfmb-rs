//use futures::stream::StreamExt;
use log::info;
use openfmb::encoding::ProtobufEncoding;
use openfmb_messages::{commonmodule::*, solarforecastmodule::*};

use std::env;
use std::time::SystemTime;
use tokio::time;
//use rand::Rng;

/*

message SolarForecastPoint
{
    commonmodule.Timestamp forecastTime = 1 [(uml.option_required_field) = true, (uml.option_multiplicity_min) = 1];
    google.protobuf.FloatValue output_ACPower = 2;
    google.protobuf.FloatValue irradiance_GlobalHorizontal = 3;
    google.protobuf.FloatValue irradiance_PlaneOfArray = 4;    
    google.protobuf.FloatValue ambientTempC = 5;
    google.protobuf.FloatValue ambientTempF = 6;
}


message SolarForecastSCH
{    
    repeated SolarForecastPoint crvPts = 1 [(uml.option_required_field) = true, (uml.option_multiplicity_min) = 1];
}

message SolarForecast
{
    SolarForecastSCH solarForecastSCH = 1;
}

message SolarForecastProfile
{
    option (uml.option_openfmb_profile) = true;
    commonmodule.MessageInfo messageInfo = 1 [(uml.option_parent_message) = true];
    SolarForecast solarForecast = 2 [(uml.option_required_field) = true, (uml.option_multiplicity_min) = 1];
    
}

*/

pub fn setForecastPoint(point: f32) -> SolarForecastPoint {
  let mut p = SolarForecastPoint {
    output_ac_power: Some(point),
    ..Default::default()
  };
  p
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
  pretty_env_logger::init();

  let mrid = uuid::Uuid::parse_str("51a949c6-ff44-4128-b74b-ae1f11d7453e")?;
  let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
  let nc = nats::asynk::connect(&nats_url).await?;
  let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
  let sf = openfmb::device::SolarForecast::new(bus, mrid);
  info!(
    "Connected to solarforecast {:?} using nats at {:?}",
    mrid, nats_url
  );
  let sfp = vec![5.77730306109351, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

  let sfp_vec: Vec<SolarForecastPoint> = sfp.iter().map( |point| setForecastPoint(*point)).collect();

  let mut solarForecastSch: SolarForecastSch = SolarForecastSch { 
    crv_pts: sfp_vec 
  };

  let mut solarForecast: SolarForecast = SolarForecast {
    solar_forecast_sch: Some(solarForecastSch)
  };

  let mut poll_interval = time::interval(time::Duration::from_secs(1));
  let mut identified_object: IdentifiedObject = Default::default();
  identified_object.description = Some(format!("OpenFMB Solarforecast Device {}", mrid));
  identified_object.m_rid = Some(format!("{}", mrid));
  let message_info = MessageInfo {
    identified_object: Some(identified_object),
    message_time_stamp: None,
  };

  let mut forecast: SolarForecastProfile  = SolarForecastProfile {
    solar_forecast:Some(solarForecast),
    message_info: Some(message_info),    
  };


  loop {
    tokio::select! {
      _ = poll_interval.tick() => {
        info!("Tick, publishing reading");
        let forecast = forecast.clone();
        let mut sf = sf.clone();
        tokio::spawn(async move {
            let time = SystemTime::now();
            let mut forecast = forecast.clone();
            forecast.message_info.as_mut().unwrap().message_time_stamp = Some(Timestamp {
                seconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                nanoseconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                tq: None,
            });
            sf.forecast(forecast).await.unwrap();
        });
      },
    } // select
  } // loop dloop


}