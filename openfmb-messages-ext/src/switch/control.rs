use crate::{error::*, ControlProfileExt, OpenFMBExt};

use openfmb_messages::{
    commonmodule::{
        CheckConditions, ConductingEquipment, ControlDpc, ControlMessageInfo, ControlTimestamp,
        MessageInfo, SwitchControlScheduleFsch, SwitchCsg, SwitchPoint,
    },
    switchmodule::{ProtectedSwitch, SwitchControl, SwitchControlFscc, SwitchControlProfile},
};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
use uuid::Uuid;

impl OpenFMBExt for SwitchControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        let switch_control = if self
            .switch_control
            .clone()
            .unwrap()
            .switch_control_fscc
            .unwrap()
            .switch_control_schedule_fsch
            .unwrap()
            .val_dcsg
            .unwrap()
            .crv_pts
            .first()
            .clone()
            .unwrap()
            .clone()
            .pos
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
    fn switch_open_msg(m_rid: &str) -> SwitchControlProfile {
        Self::build_control_profile(m_rid, SystemTime::now(), false)
    }

    fn switch_close_msg(m_rid: &str) -> SwitchControlProfile {
        Self::build_control_profile(m_rid, SystemTime::now(), true)
    }

    fn switch_synchro_msg(m_rid: &str, synchro_check: bool) -> SwitchControlProfile {
        Self::build_synchro_profile(m_rid, SystemTime::now(), synchro_check)
    }

    fn build_control_profile(
        m_rid: &str,
        start_time: SystemTime,
        pos: bool,
    ) -> SwitchControlProfile;
    fn build_synchro_profile(
        m_rid: &str,
        start_time: SystemTime,
        synchro_check: bool,
    ) -> SwitchControlProfile;
}

impl SwitchControlExt for SwitchControlProfile {
    fn build_synchro_profile(
        m_rid: &str,
        _start_time: SystemTime,
        synchro_check: bool,
    ) -> SwitchControlProfile {
        let msg_info: ControlMessageInfo = SwitchControlProfile::build_control_message_info();
        SwitchControlProfile {
            control_message_info: Some(msg_info),
            protected_switch: Some(ProtectedSwitch {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            switch_control: Some(SwitchControl {
                check: Some(CheckConditions {
                    interlock_check: None,
                    synchro_check: Some(synchro_check),
                }),
                control_value: None,
                switch_control_fscc: None,
            }),
            ied: None,
        }
    }

    fn build_control_profile(
        m_rid: &str,
        start_time: SystemTime,
        pos: bool,
    ) -> SwitchControlProfile {
        let msg_info: ControlMessageInfo = SwitchControlProfile::build_control_message_info();
        SwitchControlProfile {
            control_message_info: Some(msg_info),
            protected_switch: Some(ProtectedSwitch {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            switch_control: Some(SwitchControl {
                check: None,
                control_value: None,
                switch_control_fscc: Some(SwitchControlFscc {
                    logical_node_for_control: None,
                    switch_control_schedule_fsch: Some(SwitchControlScheduleFsch {
                        val_dcsg: Some(SwitchCsg {
                            crv_pts: vec![SwitchPoint {
                                pos: Some(ControlDpc { ctl_val: pos }),
                                start_time: Some(ControlTimestamp {
                                    fraction: 0,
                                    seconds: start_time
                                        .duration_since(SystemTime::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs(),
                                }),
                            }],
                        }),
                    }),
                }),
            }),
            ied: None,
        }
    }
}

impl ControlProfileExt for SwitchControlProfile {}
