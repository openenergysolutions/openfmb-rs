// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{error::*, ControlProfileExt, OpenFMBExt};
use openfmb_messages::{
    commonmodule::{
        ConductingEquipment, ControlApc, MessageInfo,
    },
    resourcemodule::{
        ResourceDiscreteControlProfile, ResourceDiscreteControl, AnalogControlGgio
    },
};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
use uuid::Uuid;

impl OpenFMBExt for ResourceDiscreteControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        unimplemented!()
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        unimplemented!()
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("ResourceDiscreteControlProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self                
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self                                   
            .conducting_equipment
            .as_ref()
            .context(NoConductingEquipment)?
            .named_object
            .as_ref()
            .context(NoNamedObject)?
            .name
            .clone()
            .context(NoName)?)
    }
}

pub trait ResourceControlExt: ControlProfileExt {
    fn set_analog_msg(m_rid: &str, val: f64) -> ResourceDiscreteControlProfile {
        Self::build_control_profile(m_rid, val, SystemTime::now())
    }

    fn build_control_profile(
        m_rid: &str,
        val: f64,
        start_time: SystemTime
    ) -> ResourceDiscreteControlProfile;
}

impl ControlProfileExt for ResourceDiscreteControlProfile {}

impl ResourceControlExt for ResourceDiscreteControlProfile {
    fn build_control_profile(
        m_rid: &str,
        val: f64,
        _start_time: SystemTime
    ) -> ResourceDiscreteControlProfile
    {
        ResourceDiscreteControlProfile {
            control_message_info: Some(ResourceDiscreteControlProfile::build_control_message_info()),
            conducting_equipment: Some(ConductingEquipment {
                named_object: None,
                m_rid: m_rid.to_string(),
            }),
            resource_discrete_control: Some(ResourceDiscreteControl{
                analog_control_ggio: vec![AnalogControlGgio {
                    logical_node: None,
                    phase: None,
                    an_out: Some(ControlApc {
                        ctl_val: val
                    })
                }],
                check: None,
                identified_object: None,
                boolean_control_ggio: vec![],
                integer_control_ggio: vec![],
                string_control_ggio: vec![]
            })
        }
    }
}