//use futures::stream::StreamExt;
use log::info;
use openfmb::encoding::ProtobufEncoding;
use openfmb_messages::{commonmodule::*, generationmodule::*};

use std::env;
use std::time::SystemTime;
use tokio::time;
//use rand::Rng;

/*  TODO: Pull the values to send for this from a configuration file?  This is just testing data we hope to receive from live devices
        bus_id:doc.get_first_node("//Cluster/Array/Name[text()='GEN_BUS_ID']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        vf_controller: doc.get_first_node("//Cluster/Array/Name[text()='GEN_VF_Controller']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        min_power: doc.get_first_node("//Cluster/Array/Name[text()='GEN_Min_Power']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
GenerationStatusProfile.GeneratingUnit.maxOperatingP        power_rating: doc.get_first_node("//Cluster/Array/Name[text()='GEN_Power_Rating']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        maxpf_lead: doc.get_first_node("//Cluster/Array/Name[text()='GEN_MaxPF_Lead']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        maxpf_lag:  doc.get_first_node("//Cluster/Array/Name[text()='GEN_MaxPF_Lag']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
        marginal_cost: doc.get_nodeset("//Cluster/Array/Name[text()='GEN_Marginal_Cost']/../DBL/Val/text()").unwrap().iter().map(|x| x.to_string().parse().unwrap()).collect(),
        marginal_power: doc.get_nodeset("//Cluster/Array/Name[text()='GEN_Marginal_Power']/../DBL/Val/text()").unwrap().iter().map(|x| x.to_string().parse().unwrap()).collect(),
        start_cost: doc.get_first_node("//Cluster/Array/Name[text()='GEN_Start_Cost']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
GenerationReadingProfile.GenerationReading.ReadingMMXU.W          pmeas: doc.get_first_node("//Cluster/Array/Name[text()='GEN_Pmeas']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(), 
GenerationReadingProfile.GenerationReading.ReadingMMXU.VAr        qmeas: doc.get_first_node("//Cluster/Array/Name[text()='GEN_Qmeas']/../DBL/Val/text()").unwrap().to_string().parse().unwrap(),
*/

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
  pretty_env_logger::init();

  let mrid = uuid::Uuid::parse_str("46f9c003-96c9-4de6-b6a1-110ffca8362a")?;
  let nats_url = env::var("NATS_URL").unwrap_or("nats://127.0.0.1:4222".to_string());
  let nc = nats::asynk::connect(&nats_url).await?;
  let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
  let gen = openfmb::device::Generation::new(bus, mrid);
  info!(
    "Connected to ess {:?} using nats at {:?}",
    mrid, nats_url
  );

  let mut poll_interval = time::interval(time::Duration::from_secs(1));
  let mut identified_object: IdentifiedObject = Default::default();
  identified_object.description = Some(format!("OpenFMB Generation Device {}", mrid));
  identified_object.m_rid = Some(format!("{}", mrid));

  let message_info = MessageInfo {
    identified_object: Some(identified_object),
    message_time_stamp: None,
  };

  let mut pmag: f64 = -0.99593641755326;
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


  let mut qmag: f64 = -1.58428961466532;
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

  /*
  
  message GenerationReading
{
    // UML inherited base object
    commonmodule.ConductingEquipmentTerminalReading conductingEquipmentTerminalReading = 1 [(uml.option_parent_message) = true];
    // MISSING DOCUMENTATION!!!
    commonmodule.PhaseMMTN phaseMMTN = 2;
    // MISSING DOCUMENTATION!!!
    commonmodule.ReadingMMTR readingMMTR = 3;
    // MISSING DOCUMENTATION!!!
    commonmodule.ReadingMMXU readingMMXU = 4;
}

// Generation reading profile
message GenerationReadingProfile
{
    option (uml.option_openfmb_profile) = true;
    // UML inherited base object
    commonmodule.ReadingMessageInfo readingMessageInfo = 1 [(uml.option_parent_message) = true];
    // MISSING DOCUMENTATION!!!
    GeneratingUnit generatingUnit = 2 [(uml.option_required_field) = true, (uml.option_multiplicity_min) = 1];
    // MISSING DOCUMENTATION!!!
    GenerationReading generationReading = 3 [(uml.option_required_field) = true, (uml.option_multiplicity_min) = 1];
}

*/

  let mut reading: GenerationReadingProfile = Default::default();
 
  reading.reading_message_info = Some(ReadingMessageInfo {
    message_info: Some(message_info)
  });

  reading.generation_reading = Some(GenerationReading {
    reading_mmxu: Some(mmxu),
    ..Default::default()
 }); 

  loop {
    tokio::select! {
      _ = poll_interval.tick() => {
        info!("Tick, publishing reading");
        let reading = reading.clone();
        let mut gen = gen.clone();
        tokio::spawn(async move {
            let time = SystemTime::now();
            let mut reading = reading.clone();
            reading.reading_message_info.as_mut().unwrap().message_info.as_mut().unwrap().message_time_stamp = Some(Timestamp {
                seconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                nanoseconds: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                tq: None,
            });
            gen.reading(reading).await.unwrap();
        });
      },
    } // select
  } // loop dloop

}