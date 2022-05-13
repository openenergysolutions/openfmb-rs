//use futures::stream::StreamExt;
use log::info;
use openfmb::encoding::ProtobufEncoding;
use openfmb_messages::{commonmodule::*, priceforecastmodule::*};

use std::env;
use std::time::SystemTime;
use tokio::time;
//use rand::Rng;


pub fn setForecastPoint(dpk: f32) -> PriceForecastPoint {
  let fp = PriceForecastPoint {
    dollar_per_kw: Some(dpk),    
    ..Default::default()
  };
  fp
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
  pretty_env_logger::init();

  let mrid = uuid::Uuid::parse_str("aa6238a7-009e-42fa-ab9e-9ce6fe0e101b")?;
  let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
  let nc = nats::asynk::connect(&nats_url).await?;
  let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
  let price = openfmb::device::PriceForecast::new(bus, mrid);
  info!(
    "Connected to priceforecast {:?} using nats at {:?}",
    mrid, nats_url
  );


  let real_time_price: Vec<f32> = vec![  3.56400000000000,3.56400000000000,3.56400000000000,3.56400000000000,3.56400000000000,3.56400000000000,3.56400000000000,3.56400000000000,3.63000000000000,3.56400000000000,3.56400000000000,3.56400000000000,3.56400000000000];
  
  let pfp: Vec<PriceForecastPoint> = real_time_price.iter().map( |rtp| setForecastPoint(*rtp)).collect();


  let mut priceForecastSch: PriceForecastSch = PriceForecastSch { 
    crv_pts: pfp 
  };

  let mut priceForecast: PriceForecast = PriceForecast {
    price_forecast_sch: Some(priceForecastSch),
    ..Default::default()
  };

  let mut poll_interval = time::interval(time::Duration::from_secs(15));
  let mut identified_object: IdentifiedObject = Default::default();
  identified_object.description = Some(format!("OpenFMB Priceforecast Device {}", mrid));
  identified_object.m_rid = Some(format!("{}", mrid));
  let message_info = MessageInfo {
    identified_object: Some(identified_object),
    message_time_stamp: None,
  };

  let mut forecast: PriceForecastProfile  = PriceForecastProfile {
    price_forecast:Some(priceForecast),
    message_info: Some(message_info),    
  };


  loop {
    tokio::select! {
      _ = poll_interval.tick() => {
        info!("Tick, publishing forecast");
        let forecast = forecast.clone();
        let mut price = price.clone();
        tokio::spawn(async move {
            let time = SystemTime::now();
            let mut forecast = forecast.clone();
            forecast.message_info.as_mut().unwrap().message_time_stamp = Some(Timestamp {
                seconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                nanoseconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                tq: None,
            });
            price.forecast(forecast).await.unwrap();
        });
      },
    } // select
  } // loop dloop


}