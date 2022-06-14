// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

include!(concat!(env!("OUT_DIR"), "/openfmb.microgrid.rs"));

use std::str::FromStr;

impl FromStr for microgrid_control::ControlMessage {
    type Err = ();

    fn from_str(input: &str) -> Result<microgrid_control::ControlMessage, Self::Err> {
        match input {
            "ResetDevices" => Ok(microgrid_control::ControlMessage::ResetDevices(
                "".to_string(),
            )),
            "Shutdown" => Ok(microgrid_control::ControlMessage::Shutdown("".to_string())),
            "InitiateIsland" => Ok(microgrid_control::ControlMessage::InitiateIsland(
                "".to_string(),
            )),
            "InitiateGridConnect" => Ok(microgrid_control::ControlMessage::InitiateGridConnect(
                "".to_string(),
            )),
            "EnableNetZero" => Ok(microgrid_control::ControlMessage::EnableNetZero(
                "".to_string(),
            )),
            "DisableNetZero" => Ok(microgrid_control::ControlMessage::DisableNetZero(
                "".to_string(),
            )),
            "PccControl" => Ok(microgrid_control::ControlMessage::PccControl(
                "".to_string(),
            )),
            _ => Err(()),
        }
    }
}

impl FromStr for device_control::DeviceControlMessage {
    type Err = ();

    fn from_str(input: &str) -> Result<device_control::DeviceControlMessage, Self::Err> {
        match input {
            "EnableSolarInverter" => Ok(device_control::DeviceControlMessage::EnableSolarInverter),
            "DisableSolarInverter" => {
                Ok(device_control::DeviceControlMessage::DisableSolarInverter)
            }
            "EnableLoadbank" => Ok(device_control::DeviceControlMessage::EnableLoadbank),
            "DisableLoadbank" => Ok(device_control::DeviceControlMessage::DisableLoadbank),
            "EssStart" => Ok(device_control::DeviceControlMessage::EssStart),
            "EssDischarge" => Ok(device_control::DeviceControlMessage::EssDischarge),
            "EssSocManage" => Ok(device_control::DeviceControlMessage::EssSocManage),
            "EssSocLimits" => Ok(device_control::DeviceControlMessage::EssSocLimits),
            "EssStop" => Ok(device_control::DeviceControlMessage::EssStop),
            "GeneratorOn" => Ok(device_control::DeviceControlMessage::GeneratorOn),
            "GeneratorDisabled" => Ok(device_control::DeviceControlMessage::GeneratorDisabled),
            "GeneratorEnabled" => Ok(device_control::DeviceControlMessage::GeneratorEnabled),
            "GeneratorOff" => Ok(device_control::DeviceControlMessage::GeneratorOff),
            "SwitchOneOpen" => Ok(device_control::DeviceControlMessage::SwitchOneOpen),
            "SwitchOneClosed" => Ok(device_control::DeviceControlMessage::SwitchOneClosed),
            "SwitchTwoOpen" => Ok(device_control::DeviceControlMessage::SwitchTwoOpen),
            "SwitchTwoClosed" => Ok(device_control::DeviceControlMessage::SwitchTwoClosed),
            "BreakerThreeOpen" => Ok(device_control::DeviceControlMessage::BreakerThreeOpen),
            "BreakerThreeClosed" => Ok(device_control::DeviceControlMessage::BreakerThreeClosed),
            "SwitchFourOpen" => Ok(device_control::DeviceControlMessage::SwitchFourOpen),
            "SwitchFourClosed" => Ok(device_control::DeviceControlMessage::SwitchFourClosed),
            "ResetSwitchOne" => Ok(device_control::DeviceControlMessage::ResetSwitchOne),
            "ResetSwitchTwo" => Ok(device_control::DeviceControlMessage::ResetSwitchTwo),
            "ResetSwitchThree" => Ok(device_control::DeviceControlMessage::ResetSwitchThree),
            "ResetSwitchFour" => Ok(device_control::DeviceControlMessage::ResetSwitchFour),
            "ResetBreakerThree" => Ok(device_control::DeviceControlMessage::ResetBreakerThree),
            "ResetEss" => Ok(device_control::DeviceControlMessage::ResetEss),
            "ResetSolar" => Ok(device_control::DeviceControlMessage::ResetSolar),
            "ResetLoadbank" => Ok(device_control::DeviceControlMessage::ResetLoadbank),
            "SolarOn" => Ok(device_control::DeviceControlMessage::SolarOn),
            "SolarOff" => Ok(device_control::DeviceControlMessage::SolarOff),
            _ => Err(()),
        }
    }
}

impl FromStr for hmi_control::ControlType {
    type Err = ();

    fn from_str(input: &str) -> Result<hmi_control::ControlType, Self::Err> {
        match input {
            "ToggleEnvironment" => Ok(hmi_control::ControlType::ToggleEnvironment),
            _ => Err(()),
        }
    }
}

impl FromStr for generic_control::ControlType {
    type Err = ();

    fn from_str(input: &str) -> Result<generic_control::ControlType, Self::Err> {
        match input {
            "Open" => Ok(generic_control::ControlType::Open),
            "OpenPhsA" => Ok(generic_control::ControlType::OpenPhsA),
            "OpenPhsB" => Ok(generic_control::ControlType::OpenPhsB),
            "OpenPhsC" => Ok(generic_control::ControlType::OpenPhsC),
            "Close" => Ok(generic_control::ControlType::Close),
            "ClosePhsA" => Ok(generic_control::ControlType::ClosePhsA),
            "ClosePhsB" => Ok(generic_control::ControlType::ClosePhsB),
            "ClosePhsC" => Ok(generic_control::ControlType::ClosePhsC),
            "SetModBlkOn" => Ok(generic_control::ControlType::SetModBlkOn),
            "SetModBlkOff" => Ok(generic_control::ControlType::SetModBlkOff),
            "StateOn" => Ok(generic_control::ControlType::StateOn),
            "StateOff" => Ok(generic_control::ControlType::StateOff),
            "SetValue" => Ok(generic_control::ControlType::SetValue),
            "SetWNetMag" => Ok(generic_control::ControlType::SetWNetMag),
            "SetVarNetMag" => Ok(generic_control::ControlType::SetVarNetMag),
            "TapChangeLowerPhs3" => Ok(generic_control::ControlType::TapChangeLowerPhs3),
            "TapChangeRaisePhs3" => Ok(generic_control::ControlType::TapChangeRaisePhs3),
            "TapChangeLowerPhsA" => Ok(generic_control::ControlType::TapChangeLowerPhsA),
            "TapChangeRaisePhsA" => Ok(generic_control::ControlType::TapChangeRaisePhsA),
            "TapChangeLowerPhsB" => Ok(generic_control::ControlType::TapChangeLowerPhsB),
            "TapChangeRaisePhsB" => Ok(generic_control::ControlType::TapChangeRaisePhsB),
            "TapChangeLowerPhsC" => Ok(generic_control::ControlType::TapChangeLowerPhsC),
            "TapChangeRaisePhsC" => Ok(generic_control::ControlType::TapChangeRaisePhsC),
            "BlackStartEnable" => Ok(generic_control::ControlType::BlackStartEnable),
            "BlackStartDisable" => Ok(generic_control::ControlType::BlackStartDisable),
            "FrequencySetPointEnable" => Ok(generic_control::ControlType::FrequencySetPointEnable),
            "FrequencySetPointDisable" => {
                Ok(generic_control::ControlType::FrequencySetPointDisable)
            }
            "ReactivePowerSetPointEnable" => {
                Ok(generic_control::ControlType::ReactivePowerSetPointEnable)
            }
            "ReactivePowerSetPointDisable" => {
                Ok(generic_control::ControlType::ReactivePowerSetPointDisable)
            }
            "RealPowerSetPointEnable" => Ok(generic_control::ControlType::RealPowerSetPointEnable),
            "RealPowerSetPointDisable" => {
                Ok(generic_control::ControlType::RealPowerSetPointDisable)
            }
            "TransToIslandOnGridLossEnable" => {
                Ok(generic_control::ControlType::TransToIslandOnGridLossEnable)
            }
            "TransToIslandOnGridLossDisable" => {
                Ok(generic_control::ControlType::TransToIslandOnGridLossDisable)
            }
            "VoltageSetPointEnable" => Ok(generic_control::ControlType::VoltageSetPointEnable),
            "VoltageSetPointDisable" => Ok(generic_control::ControlType::VoltageSetPointDisable),
            "SetStateKindUndefined" => Ok(generic_control::ControlType::SetStateKindUndefined),
            "SetStateKindOff" => Ok(generic_control::ControlType::SetStateKindOff),
            "SetStateKindOn" => Ok(generic_control::ControlType::SetStateKindOn),
            "SetStateKindStandBy" => Ok(generic_control::ControlType::SetStateKindStandBy),
            "ResetBreaker" => Ok(generic_control::ControlType::ResetBreaker),
            "ResetEss" => Ok(generic_control::ControlType::ResetEss),
            "ResetLoad" => Ok(generic_control::ControlType::ResetLoad),
            "ResetSolar" => Ok(generic_control::ControlType::ResetSolar),
            "ANetMag" => Ok(generic_control::ControlType::ANetMag),
            "ANeutMag" => Ok(generic_control::ControlType::ANeutMag),
            "APhsAMag" => Ok(generic_control::ControlType::APhsAMag),
            "APhsBMag" => Ok(generic_control::ControlType::APhsBMag),
            "APhsCMag" => Ok(generic_control::ControlType::APhsCMag),
            "HzMag" => Ok(generic_control::ControlType::HzMag),
            "PfNetMag" => Ok(generic_control::ControlType::PfNetMag),
            "PfNeutMag" => Ok(generic_control::ControlType::PfNeutMag),
            "PfPhsAMag" => Ok(generic_control::ControlType::PfPhsAMag),
            "PfPhsBMag" => Ok(generic_control::ControlType::PfPhsBMag),
            "PfPhsCMag" => Ok(generic_control::ControlType::PfPhsCMag),
            "PhVNetAng" => Ok(generic_control::ControlType::PhVNetAng),
            "PhVNetMag" => Ok(generic_control::ControlType::PhVNetMag),
            "PhVNeutAng" => Ok(generic_control::ControlType::PhVNeutAng),
            "PhVNeutMag" => Ok(generic_control::ControlType::PhVNeutMag),
            "PhVPhsAAng" => Ok(generic_control::ControlType::PhVPhsAAng),
            "PhVPhsAMag" => Ok(generic_control::ControlType::PhVPhsAMag),
            "PhVPhsBAng" => Ok(generic_control::ControlType::PhVPhsBAng),
            "PhVPhsBMag" => Ok(generic_control::ControlType::PhVPhsBMag),
            "PhVPhsCAng" => Ok(generic_control::ControlType::PhVPhsCAng),
            "PhVPhsCMag" => Ok(generic_control::ControlType::PhVPhsCMag),
            "PpvPhsAbAng" => Ok(generic_control::ControlType::PpvPhsAbAng),
            "PpvPhsAbMag" => Ok(generic_control::ControlType::PpvPhsAbMag),
            "PpvPhsBcAng" => Ok(generic_control::ControlType::PpvPhsBcAng),
            "PpvPhsBcMag" => Ok(generic_control::ControlType::PpvPhsBcMag),
            "PpvPhsCaAng" => Ok(generic_control::ControlType::PpvPhsCaAng),
            "PpvPhsCaMag" => Ok(generic_control::ControlType::PpvPhsCaMag),
            "VaNetMag" => Ok(generic_control::ControlType::VaNetMag),
            "VaNeutMag" => Ok(generic_control::ControlType::VaNeutMag),
            "VaPhsAMag" => Ok(generic_control::ControlType::VaPhsAMag),
            "VaPhsBMag" => Ok(generic_control::ControlType::VaPhsBMag),
            "VaPhsCMag" => Ok(generic_control::ControlType::VaPhsCMag),
            "VArNetMag" => Ok(generic_control::ControlType::VArNetMag),
            "VArNeutMag" => Ok(generic_control::ControlType::VArNeutMag),
            "VArPhsAMag" => Ok(generic_control::ControlType::VArPhsAMag),
            "VArPhsBMag" => Ok(generic_control::ControlType::VArPhsBMag),
            "VArPhsCMag" => Ok(generic_control::ControlType::VArPhsCMag),
            "WNetMag" => Ok(generic_control::ControlType::WNetMag),
            "WNeutMag" => Ok(generic_control::ControlType::WNeutMag),
            "WPhsAMag" => Ok(generic_control::ControlType::WPhsAMag),
            "WPhsBMag" => Ok(generic_control::ControlType::WPhsBMag),
            "WPhsCMag" => Ok(generic_control::ControlType::WPhsCMag),
            _ => {
                println!("Unsupport Generic control {}", input);
                Err(())
            }
        }
    }
}
