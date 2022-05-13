//use futures::stream::StreamExt;
use log::info;
use openfmb::encoding::ProtobufEncoding;
use openfmb_messages::{commonmodule::*, loadforecastmodule::*};

use std::env;
use std::time::SystemTime;
use tokio::time;
//use rand::Rng;


pub fn setForecastPoint(watt: f32, q: f32) -> LoadForecastPoint {
  let fp = LoadForecastPoint {
    watt: Some(watt.into()),
    vars: Some(q),
    ..Default::default()
  };
  fp
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
  pretty_env_logger::init();

  let mrid = uuid::Uuid::parse_str("91ccf218-4728-4957-8d0b-63e797247762")?;
  let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
  let nc = nats::asynk::connect(&nats_url).await?;
  let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
  let load = openfmb::device::LoadForecast::new(bus, mrid);
  info!(
    "Connected to loadforecast {:?} using nats at {:?}",
    mrid, nats_url
  );



  let load_watts = vec![84.47727706963761,92.95981200000000,92.78102800000001,92.60140800000001,92.42047900000000,92.23723200000001,92.05021300000000,91.85782000000000,91.65836600000000,91.45030100000000,91.23218799999999,91.00271499999999,90.76082200000000];
  let load_q: Vec<f32> = vec![0.0; 13];
  let mut lfp: Vec<LoadForecastPoint> = Vec::with_capacity(13);
  if load_watts.len() == load_q.len() {
    for (pos, e) in load_watts.iter().enumerate() {
      let fp = setForecastPoint(*e, load_q[pos]);
      lfp.push(fp);
    }
  }


  let mut loadForecastSch: LoadForecastSch = LoadForecastSch { 
    crv_pts: lfp 
  };

  let mut loadForecast: LoadForecast = LoadForecast {
    load_forecast_sch: Some(loadForecastSch),
    ..Default::default()
  };

  let mut poll_interval = time::interval(time::Duration::from_secs(15));
  let mut identified_object: IdentifiedObject = Default::default();
  identified_object.description = Some(format!("OpenFMB Loadforecast Device {}", mrid));
  identified_object.m_rid = Some(format!("{}", mrid));
  let message_info = MessageInfo {
    identified_object: Some(identified_object),
    message_time_stamp: None,
  };

  let mut forecast: LoadForecastProfile  = LoadForecastProfile {
    load_forecast:Some(loadForecast),
    message_info: Some(message_info),    
  };


  loop {
    tokio::select! {
      _ = poll_interval.tick() => {
        info!("Tick, publishing reading");
        let forecast = forecast.clone();
        let mut load = load.clone();
        tokio::spawn(async move {
            let time = SystemTime::now();
            let mut forecast = forecast.clone();
            forecast.message_info.as_mut().unwrap().message_time_stamp = Some(Timestamp {
                seconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                nanoseconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                tq: None,
            });
            load.forecast(forecast).await.unwrap();
        });
      },
    } // select
  } // loop dloop


}