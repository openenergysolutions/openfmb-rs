use openfmb_messages::{
    commonmodule::*,
    regulatormodule::*,
};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr};
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

pub trait RegulatorDiscreteControlExt: ControlProfileExt {
    fn regulator_tap_lower_phs3_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_lower_phs3_profile(m_rid, ct_val)
    }
    fn regulator_tap_raise_phs3_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_raise_phs3_profile(m_rid, ct_val)
    }  

    fn regulator_tap_lower_phs_a_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_lower_phs_a_profile(m_rid, ct_val)
    }
    fn regulator_tap_raise_phs_a_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_raise_phs_a_profile(m_rid, ct_val)
    }

    fn regulator_tap_lower_phs_b_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_lower_phs_b_profile(m_rid, ct_val)
    }
    fn regulator_tap_raise_phs_b_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_raise_phs_b_profile(m_rid, ct_val)
    }

    fn regulator_tap_lower_phs_c_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_lower_phs_c_profile(m_rid, ct_val)
    }
    fn regulator_tap_raise_phs_c_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_raise_phs_c_profile(m_rid, ct_val)
    }     

    fn build_tap_lower_phs3_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;
    fn build_tap_raise_phs3_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;

    fn build_tap_lower_phs_a_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;
    fn build_tap_raise_phs_a_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;

    fn build_tap_lower_phs_b_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;
    fn build_tap_raise_phs_b_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;

    fn build_tap_lower_phs_c_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;
    fn build_tap_raise_phs_c_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;
}

impl RegulatorDiscreteControlExt for RegulatorDiscreteControlProfile {
    fn build_tap_lower_phs3_profile(
        m_rid: &str,        
        ct_val: bool,
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
                            ctl_val: ct_val
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
        ct_val: bool,
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
                            ctl_val: ct_val
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    } 

    fn build_tap_lower_phs_a_profile(
        m_rid: &str,        
        ct_val: bool,
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
                            ctl_val: ct_val
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    } 
    
    fn build_tap_raise_phs_a_profile(
        m_rid: &str,        
        ct_val: bool,
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
                            ctl_val: ct_val
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    }

    fn build_tap_lower_phs_b_profile(
        m_rid: &str,        
        ct_val: bool,
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
                            ctl_val: ct_val
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    } 
    
    fn build_tap_raise_phs_b_profile(
        m_rid: &str,        
        ct_val: bool,
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
                            ctl_val: ct_val
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    }

    fn build_tap_lower_phs_c_profile(
        m_rid: &str,        
        ct_val: bool,
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
                            ctl_val: ct_val
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),            
        }
    } 
    
    fn build_tap_raise_phs_c_profile(
        m_rid: &str,        
        ct_val: bool,
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
                            ctl_val: ct_val
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