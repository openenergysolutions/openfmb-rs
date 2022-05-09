//use futures::stream::StreamExt;
use log::info;
use openfmb::encoding::ProtobufEncoding;
use openfmb_messages::{commonmodule::*, essmodule::*};

use std::env;
use std::time::SystemTime;
use tokio::time;
//use rand::Rng;

/*  TODO: Pull the values to send for this from a configuration file?  This is just testing data we hope to receive from live devices
    let es_device = ESDevice {
        bus_id:doc.get_first_node("//Cluster/Array/Name[text()='ES_BUS_ID']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        *measurement_soc: doc.get_first_node("//Cluster/Array/Name[text()='ES_Measurement_SOC']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        vf_controller: doc.get_first_node("//Cluster/Array/Name[text()='ES_VF_Controller']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        power_rating: doc.get_first_node("//Cluster/Array/Name[text()='ES_Power_Rating']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        energy_rating: doc.get_first_node("//Cluster/Array/Name[text()='ES_Energy_Rating']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        power_factor:  doc.get_first_node("//Cluster/Array/Name[text()='ES_PowerFactor']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        charge_effic: doc.get_first_node("//Cluster/Array/Name[text()='ES_ChargeEffic']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        discharge_effic: doc.get_first_node("//Cluster/Array/Name[text()='ES_DischargeEffic']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        min_soc: doc.get_first_node("//Cluster/Array/Name[text()='ES_min_SOC']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        max_soc: doc.get_first_node("//Cluster/Array/Name[text()='ES_max_SOC']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        pmeas: doc.get_first_node("//Cluster/Array/Name[text()='ES_Pmeas']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        qmeas: doc.get_first_node("//Cluster/Array/Name[text()='ES_Qmeas']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
    };
*/

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
  pretty_env_logger::init();

  let mrid = uuid::Uuid::parse_str("4f659ee9-5ce8-4ef2-bcb5-0f38f2d31868")?;
  let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
  let nc = nats::asynk::connect(&nats_url).await?;
  let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
  let ess = openfmb::device::Ess::new(bus, mrid);
  info!(
    "Connected to ess {:?} using nats at {:?}",
    mrid, nats_url
  );

  let mut mv: Mv = Default::default();
  let mut ess_status_z: EssStatusZbat = Default::default();

  mv.mag = 50.0;
  ess_status_z.soc = Some(mv);
  
  info!(
    "Set soc f val {:?}", ess_status_z.clone().soc.clone().unwrap()
  );


  let mut pmag: f64 = 90.04681485764624;
  let mut cval: Vector = Vector {
    mag: pmag,
    ..Default::default()
  };

  let mut net: Cmv = Cmv {
    c_val: Some(cval),
    ..Default::default()
  };

  let mut w: Wye = Wye {
    net: Some(net),
    ..Default::default()    
  };


  let mut qmag: f64 = 4.12836753494804;
  let mut qval = Vector { mag: qmag, ..Default::default() };
  let mut qnet: Cmv = Cmv { c_val: Some(qval), ..Default::default() };

  let mut q: Wye = Wye { net: Some(qnet), ..Default::default() };

  // status_message_info (StatusMessageInfo) -> mmxu (ReadingMmxu) -> w (wye) -> net(cmv) -> cval(vec) -> mag (f64) 
// W (WYE) . net/phs? (CMV) . cval (Vector) - ang or mag () 
// VAr (WYE)

  let mut mmxu: ReadingMmxu = ReadingMmxu {
    w: Some(w),
    v_ar: Some(q),
    ..Default::default()
  };

  let mut reading: EssReadingProfile = Default::default();

  
  let mut poll_interval = time::interval(time::Duration::from_secs(20));
  let mut identified_object: IdentifiedObject = Default::default();
  identified_object.description = Some(format!("OpenFMB ESS Device {}", mrid));
  identified_object.m_rid = Some(format!("{}", mrid));
  let message_info = MessageInfo {
    identified_object: Some(identified_object),
    message_time_stamp: None,
  };


  reading.reading_message_info = Some(ReadingMessageInfo {
    message_info: Some(message_info.clone())
  });

  reading.ess_reading = Some(EssReading {
    reading_mmxu: Some(mmxu),
    ..Default::default()
  }); 


  let mut ess_status: EssStatus = Default::default();
  ess_status.ess_status_zbat = Some(ess_status_z);
  
  let mut status: EssStatusProfile = Default::default();
  status.ess_status = Some(ess_status);
  status.status_message_info = Some(StatusMessageInfo {
    message_info: Some(message_info)
  });

  loop {
    tokio::select! {
      _ = poll_interval.tick() => {
        info!("Tick, publishing status");
        let status = status.clone();
        let mut ess = ess.clone();
        let reading = reading.clone();
        let mut ess2 = ess.clone();
        tokio::spawn(async move {
            let time = SystemTime::now();
            let mut status = status.clone();
            status.status_message_info.as_mut().unwrap().message_info.as_mut().unwrap().message_time_stamp = Some(Timestamp {
                seconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                nanoseconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                tq: None,
            });
            ess.status(status).await.unwrap();
        });
        tokio::spawn(async move {
            let time = SystemTime::now();
            let mut reading = reading.clone();
            reading.reading_message_info.as_mut().unwrap().message_info.as_mut().unwrap().message_time_stamp = Some(Timestamp {
                seconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                nanoseconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                tq: None,
            });
            ess2.reading(reading).await.unwrap();
        });
      },
    } // select
  } // loop dloop

}