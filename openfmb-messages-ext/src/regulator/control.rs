use openfmb_messages::{
    commonmodule::*,
    regulatormodule::*,
};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
use uuid::Uuid;

use crate::{error::*, ControlProfileExt, OpenFMBExt};

impl OpenFMBExt for RegulatorDiscreteControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        unimplemented!() 
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        unimplemented!()       
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("RegulatorDiscreteControlProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .regulator_system
                .clone()
                .context(NoProtectedSwitch)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok("No Name Specified".to_string())
    }
}

pub trait RegulatorControlExt: ControlProfileExt {
    fn regulator_tap_lower_phs3_msg(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_lower_phs3_profile(m_rid, ctVal)
    }
    fn regulator_tap_raise_phs3_msg(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_raise_phs3_profile(m_rid, ctVal)
    }  

    fn regulator_tap_lower_phsA_msg(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_lower_phsA_profile(m_rid, ctVal)
    }
    fn regulator_tap_raise_phsA_msg(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_raise_phsA_profile(m_rid, ctVal)
    }

    fn regulator_tap_lower_phsB_msg(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_lower_phsB_profile(m_rid, ctVal)
    }
    fn regulator_tap_raise_phsB_msg(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_raise_phsB_profile(m_rid, ctVal)
    }

    fn regulator_tap_lower_phsC_msg(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_lower_phsC_profile(m_rid, ctVal)
    }
    fn regulator_tap_raise_phsC_msg(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_raise_phsC_profile(m_rid, ctVal)
    }     

    fn build_tap_lower_phs3_profile(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile;
    fn build_tap_raise_phs3_profile(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile;

    fn build_tap_lower_phsA_profile(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile;
    fn build_tap_raise_phsA_profile(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile;

    fn build_tap_lower_phsB_profile(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile;
    fn build_tap_raise_phsB_profile(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile;

    fn build_tap_lower_phsC_profile(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile;
    fn build_tap_raise_phsC_profile(m_rid: &str, ctVal: bool) -> RegulatorDiscreteControlProfile;
}

impl RegulatorControlExt for RegulatorDiscreteControlProfile {
    fn build_tap_lower_phs3_profile(
        m_rid: &str,        
        ctVal: bool,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo = RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),                
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_l: Some(PhaseSpc {
                        phs3: Some(ControlSpc {
                            ctl_val: ctVal
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    } 
    
    fn build_tap_raise_phs3_profile(
        m_rid: &str,        
        ctVal: bool,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo = RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),                
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_r: Some(PhaseSpc {
                        phs3: Some(ControlSpc {
                            ctl_val: ctVal
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    } 

    fn build_tap_lower_phsA_profile(
        m_rid: &str,        
        ctVal: bool,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo = RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),                
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_l: Some(PhaseSpc {
                        phs_a: Some(ControlSpc {
                            ctl_val: ctVal
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    } 
    
    fn build_tap_raise_phsA_profile(
        m_rid: &str,        
        ctVal: bool,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo = RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),                
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_r: Some(PhaseSpc {
                        phs_a: Some(ControlSpc {
                            ctl_val: ctVal
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    }

    fn build_tap_lower_phsB_profile(
        m_rid: &str,        
        ctVal: bool,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo = RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),                
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_l: Some(PhaseSpc {
                        phs_b: Some(ControlSpc {
                            ctl_val: ctVal
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    } 
    
    fn build_tap_raise_phsB_profile(
        m_rid: &str,        
        ctVal: bool,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo = RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),                
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_r: Some(PhaseSpc {
                        phs_b: Some(ControlSpc {
                            ctl_val: ctVal
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    }

    fn build_tap_lower_phsC_profile(
        m_rid: &str,        
        ctVal: bool,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo = RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),                
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_l: Some(PhaseSpc {
                        phs_c: Some(ControlSpc {
                            ctl_val: ctVal
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    } 
    
    fn build_tap_raise_phsC_profile(
        m_rid: &str,        
        ctVal: bool,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo = RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),                
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_r: Some(PhaseSpc {
                        phs_c: Some(ControlSpc {
                            ctl_val: ctVal
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    }
}

impl ControlProfileExt for RegulatorDiscreteControlProfile {}
