// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

pub mod breakermodule;
pub mod capbankmodule;
pub mod commonmodule;
pub mod coordinationservicemodule;
pub mod essmodule;
pub mod generationmodule;
pub mod interconnectionmodule;
pub mod loadmodule;
pub mod metermodule;
pub mod reclosermodule;
pub mod regulatormodule;
pub mod reservemodule;
pub mod resourcemodule;
pub mod solarmodule;
pub mod solarforecastmodule;
pub mod switchmodule;

#[derive(Debug, Clone, PartialEq)]
pub enum Module {
    BreakerModule,
    CapBankModule,
    CoordinationServiceModule,
    EssModule,
    GenerationModule,
    InterconnectionModule,
    LoadModule,
    MeterModule,
    RecloserModule,
    RegulatorModule,
    ReserveModule,
    ResourceModule,
    SolarModule,
    SolarForecastModule,
    SwitchModule,
}

impl Module {
    pub fn as_str(&self) -> &str {
        match self {
            Module::BreakerModule => "breakermodule",
            Module::CapBankModule => "capbankmodule",
            Module::CoordinationServiceModule => "coordationservicemodule",
            Module::EssModule => "essmodule",
            Module::GenerationModule => "generationmodule",
            Module::LoadModule => "loadmodule",
            Module::MeterModule => "metermodule",
            Module::InterconnectionModule => "interconnectionmodule",
            Module::RecloserModule => "reclosermodule",
            Module::RegulatorModule => "regulatormodule",
            Module::ReserveModule => "reservemodule",
            Module::ResourceModule => "resourcemodule",
            Module::SolarModule => "solarmodule",
            Module::SolarForecastModule => "solarforecastmodule",
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
            "breakmodule" => Ok(Module::BreakerModule),
            "capbankmodule" => Ok(Module::CapBankModule),
            "coordationservicemodule" => Ok(Module::CoordinationServiceModule),
            "essmodule" => Ok(Module::EssModule),
            "generationmodule" => Ok(Module::GenerationModule),
            "loadmodule" => Ok(Module::LoadModule),
            "metermodule" => Ok(Module::MeterModule),
            "interconnectionmodule" => Ok(Module::InterconnectionModule),
            "reclosermodule" => Ok(Module::RecloserModule),
            "regulatormodule" => Ok(Module::RegulatorModule),
            "reservemodule" => Ok(Module::ReserveModule),
            "resourcemodule" => Ok(Module::ReserveModule),
            "solarmodule" => Ok(Module::SolarModule),
            "solarforecastmodule" => Ok(Module::SolarForecastModule),
            "switchmodule" => Ok(Module::SwitchModule),
            _ => Err(()),
        }
    }
}

mod profiles;
pub use profiles::Profile;
mod variant;
pub use variant::ProfileMessage;
