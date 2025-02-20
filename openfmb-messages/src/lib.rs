// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

pub mod breakermodule;
pub mod capbankmodule;
pub mod circuitsegmentservicemodule;
pub mod commonmodule;
pub mod environmentmodule;
pub mod essmodule;
pub mod evsemodule;
pub mod generationmodule;
pub mod interconnectionmodule;
pub mod loadmodule;
pub mod metermodule;
pub mod reclosermodule;
pub mod regulatormodule;
pub mod reservemodule;
pub mod resourcemodule;
pub mod solarmodule;
pub mod switchmodule;

#[derive(Debug, Clone, PartialEq)]
pub enum Module {
    BreakerModule,
    CapBankModule,
    CircuitSegmentServiceModule,
    EnvironmentModule,
    EssModule,
    EvseModule,
    GenerationModule,
    InterconnectionModule,
    LoadModule,
    MeterModule,
    RecloserModule,
    RegulatorModule,
    ReserveModule,
    ResourceModule,
    SolarModule,
    SwitchModule,
}

impl Module {
    pub fn as_str(&self) -> &str {
        match self {
            Module::BreakerModule => "breakermodule",
            Module::CapBankModule => "capbankmodule",
            Module::CircuitSegmentServiceModule => "circuitsegmentservicemodule",
            Module::EnvironmentModule => "environmentmodule",
            Module::EssModule => "essmodule",
            Module::EvseModule => "evsemodule",
            Module::GenerationModule => "generationmodule",
            Module::LoadModule => "loadmodule",
            Module::MeterModule => "metermodule",
            Module::InterconnectionModule => "interconnectionmodule",
            Module::RecloserModule => "reclosermodule",
            Module::RegulatorModule => "regulatormodule",
            Module::ReserveModule => "reservemodule",
            Module::ResourceModule => "resourcemodule",
            Module::SolarModule => "solarmodule",
            Module::SwitchModule => "switchmodule",
        }
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Module {
    type Err = ();
    fn from_str(input: &str) -> Result<Module, Self::Err> {
        match input {
            "breakermodule" => Ok(Module::BreakerModule),
            "capbankmodule" => Ok(Module::CapBankModule),
            "circuitsegmentservicemodule" => Ok(Module::CircuitSegmentServiceModule),
            "environmentmodule" => Ok(Module::EnvironmentModule),
            "essmodule" => Ok(Module::EssModule),
            "evsemodule" => Ok(Module::EvseModule),
            "generationmodule" => Ok(Module::GenerationModule),
            "loadmodule" => Ok(Module::LoadModule),
            "metermodule" => Ok(Module::MeterModule),
            "interconnectionmodule" => Ok(Module::InterconnectionModule),
            "reclosermodule" => Ok(Module::RecloserModule),
            "regulatormodule" => Ok(Module::RegulatorModule),
            "reservemodule" => Ok(Module::ReserveModule),
            "resourcemodule" => Ok(Module::ReserveModule),
            "solarmodule" => Ok(Module::SolarModule),
            "switchmodule" => Ok(Module::SwitchModule),
            _ => Err(()),
        }
    }
}

mod profiles;
pub use profiles::Profile;
mod variant;
pub use variant::ProfileMessage;

// encode/decode

use prost::Message;

/// Encode a message to a byte array
pub fn encode_message<T: Message>(message: &T) -> Vec<u8> {
    let mut buf = Vec::new();
    message.encode(&mut buf).unwrap();
    buf
}

/// Decode a message from a byte array
pub fn decode_message<T: Message + std::default::Default>(data: &[u8]) -> T {
    T::decode(data).unwrap()
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let message = breakermodule::BreakerDiscreteControlProfile::default();
        let encoded = encode_message(&message);
        let decoded = decode_message(&encoded);
        assert_eq!(message, decoded);
    }
}
