use crate::{error::*, ControlProfileExt, OpenFMBExt};

use openfmb_messages::{
    commonmodule::{
        CheckConditions, ConductingEquipment, ControlDpc, PhaseDpc,
        ControlMessageInfo, MessageInfo,
    },
    switchmodule::{
        SwitchDiscreteControlProfile, SwitchDiscreteControl, ProtectedSwitch,
        SwitchDiscreteControlXswi,
    },
};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
use uuid::Uuid;

impl OpenFMBExt for SwitchDiscreteControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        let switch_control = if self
            .switch_discrete_control
            .clone()
            .unwrap()
            .switch_discrete_control_xswi
            .unwrap()
            .pos
            .unwrap()
            .phs3
            .unwrap()
            .ctl_val
        {
            "Request Closed"
        } else {
            "Request Open"
        };
        Ok(switch_control.into())
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        unimplemented!()
        //        Ok(self.solar_control.clone().context(NoStatusMessageInfo)?..unwrap())
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("SolarStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .protected_switch
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

pub trait SwitchControlExt: ControlProfileExt {
    fn switch_open_msg(m_rid: &str) -> SwitchDiscreteControlProfile {
        Self::build_control_profile(m_rid, SystemTime::now(), false)
    }

    fn switch_close_msg(m_rid: &str) -> SwitchDiscreteControlProfile {
        Self::build_control_profile(m_rid, SystemTime::now(), true)
    }

    fn switch_synchro_msg(m_rid: &str, synchro_check: bool) -> SwitchDiscreteControlProfile {
        Self::build_synchro_profile(m_rid, SystemTime::now(), synchro_check)
    }

    fn build_control_profile(
        m_rid: &str,
        start_time: SystemTime,
        pos: bool,
    ) -> SwitchDiscreteControlProfile;
    fn build_synchro_profile(
        m_rid: &str,
        start_time: SystemTime,
        synchro_check: bool,
    ) -> SwitchDiscreteControlProfile;
}

impl SwitchControlExt for SwitchDiscreteControlProfile {
    fn build_synchro_profile(
        m_rid: &str,
        _start_time: SystemTime,
        synchro_check: bool,
    ) -> SwitchDiscreteControlProfile {
        let msg_info: ControlMessageInfo = SwitchDiscreteControlProfile::build_control_message_info();
        SwitchDiscreteControlProfile {
            control_message_info: Some(msg_info),
            protected_switch: Some(ProtectedSwitch {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            switch_discrete_control: Some(SwitchDiscreteControl {
                control_value: None,
                check: Some(CheckConditions {
                    interlock_check: None,
                    synchro_check: Some(synchro_check),
                }),
                switch_discrete_control_xswi: None,
            })
        }
    }

    fn build_control_profile(
        m_rid: &str,
        _start_time: SystemTime,
        pos: bool,
    ) -> SwitchDiscreteControlProfile {
        let msg_info: ControlMessageInfo = SwitchDiscreteControlProfile::build_control_message_info();
        SwitchDiscreteControlProfile {
            control_message_info: Some(msg_info),
            protected_switch: Some(ProtectedSwitch {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            switch_discrete_control: Some(
                SwitchDiscreteControl {
                    check: None,
                    control_value: None,
                    switch_discrete_control_xswi: Some(SwitchDiscreteControlXswi {
                        reset_protection_pickup: None,
                        logical_node_for_control: None,
                        pos: Some(PhaseDpc {
                            phs3: Some(ControlDpc {
                                ctl_val: pos,
                            }),
                            phs_a: None,
                            phs_b: None,
                            phs_c: None,
                        }),
                    }),
                }
            ),
        }
    }
}

impl ControlProfileExt for SwitchDiscreteControlProfile {}