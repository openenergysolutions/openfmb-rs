// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{commonmodule::*, *};
use resourcemodule::ResourceStatusProfile;

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus};

impl OpenFMBExtStatus for ResourceStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

impl OpenFMBExt for ResourceStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("ResourceStatusProfile".to_string())
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

pub trait ResourceStatusExt {
    fn message_identified_object_name(&self) -> OpenFMBResult<String>;
    fn message_identified_description(&self) -> OpenFMBResult<String>;
    fn string_ggio(&self) -> OpenFMBResult<Vec<StringEventAndStatusGgio>>;
    fn analog_ggio(&self) -> OpenFMBResult<Vec<AnalogEventAndStatusGgio>>;
    fn integer_ggio(&self) -> OpenFMBResult<Vec<IntegerEventAndStatusGgio>>;
    fn boolean_ggio(&self) -> OpenFMBResult<Vec<BooleanEventAndStatusGgio>>;

    fn string_value_by_key(&self, key: &str) -> OpenFMBResult<String>;
    fn analog_value_by_key(&self, key: &str) -> OpenFMBResult<f64>;
    fn integer_value_by_key(&self, key: &str) -> OpenFMBResult<i32>;
    fn boolean_value_by_key(&self, key: &str) -> OpenFMBResult<bool>;
}

impl ResourceStatusExt for ResourceStatusProfile {
    fn message_identified_object_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?
            .identified_object
            .as_ref()
            .context(NoIdentifiedObject)?
            .name
            .clone().unwrap_or("".to_string())        
        )
    }

    fn message_identified_description(&self) -> OpenFMBResult<String> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?
            .identified_object
            .as_ref()
            .context(NoIdentifiedObject)?
            .description
            .clone().unwrap_or("".to_string())        
        )
    }

    fn string_ggio(&self) -> OpenFMBResult<Vec<StringEventAndStatusGgio>> {
        Ok(self
            .resource_status
            .as_ref()
            .context(NoResourceStatus)?
            .string_event_and_status_ggio
            .clone()
        )
    }

    fn analog_ggio(&self) -> OpenFMBResult<Vec<AnalogEventAndStatusGgio>> {
        Ok(self
            .resource_status
            .as_ref()
            .context(NoResourceStatus)?
            .analog_event_and_status_ggio
            .clone()
        )
    }

    fn integer_ggio(&self) -> OpenFMBResult<Vec<IntegerEventAndStatusGgio>> {
        Ok(self
            .resource_status
            .as_ref()
            .context(NoResourceStatus)?
            .integer_event_and_status_ggio
            .clone()
        )
    }

    fn boolean_ggio(&self) -> OpenFMBResult<Vec<BooleanEventAndStatusGgio>> {
        Ok(self
            .resource_status
            .as_ref()
            .context(NoResourceStatus)?
            .boolean_event_and_status_ggio
            .clone()
        )
    }

    fn string_value_by_key(&self, key: &str) -> OpenFMBResult<String> {
        let into_iter = self.string_ggio()?.into_iter();
    
        for item in into_iter {
            if let Ok(name) = item
                .logical_node
                .as_ref()
                .context(NoLogicalNode)?
                .identified_object
                .as_ref()
                .context(NoIdentifiedObject)?
                .name
                .as_ref()
                .context(NoName) {

                if key == name.to_string() {
                    return Ok(
                        item
                            .str_in
                            .as_ref()
                            .context(NoVss)?
                            .st_val
                            .clone()                            
                    );
                }
            }
        }
    
        Err(OpenFMBError::NoValue)
    }

    fn analog_value_by_key(&self, key: &str) -> OpenFMBResult<f64> {
        let into_iter = self.analog_ggio()?.into_iter();
    
        for item in into_iter {
            if let Ok(name) = item
                .logical_node
                .as_ref()
                .context(NoLogicalNode)?
                .identified_object
                .as_ref()
                .context(NoIdentifiedObject)?
                .name
                .as_ref()
                .context(NoName) {

                if key == name.to_string() {
                    return Ok(
                        item
                            .an_in
                            .as_ref()
                            .context(NoMv)?
                            .mag                                                        
                    );
                }
            }
        }
    
        Err(OpenFMBError::NoValue)
    }

    fn integer_value_by_key(&self, key: &str) -> OpenFMBResult<i32> {
        let into_iter = self.integer_ggio()?.into_iter();
    
        for item in into_iter {
            if let Ok(name) = item
                .logical_node
                .as_ref()
                .context(NoLogicalNode)?
                .identified_object
                .as_ref()
                .context(NoIdentifiedObject)?
                .name
                .as_ref()
                .context(NoName) {

                if key == name.to_string() {
                    return Ok(
                        item
                            .int_in
                            .as_ref()
                            .context(NoStatusIns)?
                            .st_val                                                        
                    );
                }
            }
        }
    
        Err(OpenFMBError::NoValue)
    }

    fn boolean_value_by_key(&self, key: &str) -> OpenFMBResult<bool> {
        let into_iter = self.boolean_ggio()?.into_iter();
    
        for item in into_iter {
            if let Ok(name) = item
                .logical_node
                .as_ref()
                .context(NoLogicalNode)?
                .identified_object
                .as_ref()
                .context(NoIdentifiedObject)?
                .name
                .as_ref()
                .context(NoName) {

                if key == name.to_string() {
                    return Ok(
                        item
                            .ind
                            .as_ref()
                            .context(NoMv)?
                            .st_val                                                        
                    );
                }
            }
        }
    
        Err(OpenFMBError::NoValue)
    }
}