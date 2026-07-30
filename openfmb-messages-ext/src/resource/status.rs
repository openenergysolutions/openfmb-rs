// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{commonmodule::*, *};
use resourcemodule::{ResourceStatus, ResourceStatusProfile};

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

    /// Set the value of the named entry, updating it in place if already
    /// present (preserving its position) or appending a new entry otherwise.
    /// Lazily initializes `resource_status` if unset. Unlike the getters,
    /// this cannot fail, so it does not return a `Result`.
    fn set_string_value_by_key(&mut self, key: &str, value: String);
    fn set_analog_value_by_key(&mut self, key: &str, value: f64);
    fn set_integer_value_by_key(&mut self, key: &str, value: i32);
    fn set_boolean_value_by_key(&mut self, key: &str, value: bool);
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
            .clone()
            .unwrap_or("".to_string()))
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
            .clone()
            .unwrap_or("".to_string()))
    }

    fn string_ggio(&self) -> OpenFMBResult<Vec<StringEventAndStatusGgio>> {
        Ok(self
            .resource_status
            .as_ref()
            .context(NoResourceStatus)?
            .string_event_and_status_ggio
            .clone())
    }

    fn analog_ggio(&self) -> OpenFMBResult<Vec<AnalogEventAndStatusGgio>> {
        Ok(self
            .resource_status
            .as_ref()
            .context(NoResourceStatus)?
            .analog_event_and_status_ggio
            .clone())
    }

    fn integer_ggio(&self) -> OpenFMBResult<Vec<IntegerEventAndStatusGgio>> {
        Ok(self
            .resource_status
            .as_ref()
            .context(NoResourceStatus)?
            .integer_event_and_status_ggio
            .clone())
    }

    fn boolean_ggio(&self) -> OpenFMBResult<Vec<BooleanEventAndStatusGgio>> {
        Ok(self
            .resource_status
            .as_ref()
            .context(NoResourceStatus)?
            .boolean_event_and_status_ggio
            .clone())
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
                .context(NoName)
            {
                if key == name.to_string() {
                    return Ok(item.str_in.as_ref().context(NoVss)?.st_val.clone());
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
                .context(NoName)
            {
                if key == name.to_string() {
                    return Ok(item.an_in.as_ref().context(NoMv)?.mag);
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
                .context(NoName)
            {
                if key == name.to_string() {
                    return Ok(item.int_in.as_ref().context(NoStatusIns)?.st_val);
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
                .context(NoName)
            {
                if key == name.to_string() {
                    return Ok(item.ind.as_ref().context(NoMv)?.st_val);
                }
            }
        }

        Err(OpenFMBError::NoValue)
    }

    fn set_string_value_by_key(&mut self, key: &str, value: String) {
        let status = self
            .resource_status
            .get_or_insert_with(ResourceStatus::default);

        for item in status.string_event_and_status_ggio.iter_mut() {
            if named(&item.logical_node, key) {
                item.str_in = Some(Vss {
                    q: None,
                    t: None,
                    st_val: value,
                });
                return;
            }
        }

        status
            .string_event_and_status_ggio
            .push(StringEventAndStatusGgio {
                phase: None,
                logical_node: named_logical_node(key),
                str_in: Some(Vss {
                    q: None,
                    t: None,
                    st_val: value,
                }),
            });
    }

    fn set_analog_value_by_key(&mut self, key: &str, value: f64) {
        let status = self
            .resource_status
            .get_or_insert_with(ResourceStatus::default);

        for item in status.analog_event_and_status_ggio.iter_mut() {
            if named(&item.logical_node, key) {
                item.an_in = Some(Mv {
                    mag: value,
                    ..Default::default()
                });
                return;
            }
        }

        status
            .analog_event_and_status_ggio
            .push(AnalogEventAndStatusGgio {
                phase: None,
                logical_node: named_logical_node(key),
                an_in: Some(Mv {
                    mag: value,
                    ..Default::default()
                }),
            });
    }

    fn set_integer_value_by_key(&mut self, key: &str, value: i32) {
        let status = self
            .resource_status
            .get_or_insert_with(ResourceStatus::default);

        for item in status.integer_event_and_status_ggio.iter_mut() {
            if named(&item.logical_node, key) {
                item.int_in = Some(StatusIns {
                    q: None,
                    t: None,
                    st_val: value,
                });
                return;
            }
        }

        status
            .integer_event_and_status_ggio
            .push(IntegerEventAndStatusGgio {
                phase: None,
                logical_node: named_logical_node(key),
                int_in: Some(StatusIns {
                    q: None,
                    t: None,
                    st_val: value,
                }),
            });
    }

    fn set_boolean_value_by_key(&mut self, key: &str, value: bool) {
        let status = self
            .resource_status
            .get_or_insert_with(ResourceStatus::default);

        for item in status.boolean_event_and_status_ggio.iter_mut() {
            if named(&item.logical_node, key) {
                item.ind = Some(StatusSps {
                    q: None,
                    t: None,
                    st_val: value,
                });
                return;
            }
        }

        status
            .boolean_event_and_status_ggio
            .push(BooleanEventAndStatusGgio {
                phase: None,
                logical_node: named_logical_node(key),
                ind: Some(StatusSps {
                    q: None,
                    t: None,
                    st_val: value,
                }),
            });
    }
}

/// True if `node`'s `identifiedObject.name` matches `key`.
fn named(node: &Option<LogicalNode>, key: &str) -> bool {
    node.as_ref()
        .and_then(|n| n.identified_object.as_ref())
        .and_then(|io| io.name.as_ref())
        .is_some_and(|name| name == key)
}

fn named_logical_node(key: &str) -> Option<LogicalNode> {
    Some(LogicalNode {
        identified_object: Some(IdentifiedObject {
            description: None,
            m_rid: None,
            name: Some(key.to_string()),
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::ResourceStatusExt;
    use openfmb_messages::resourcemodule::ResourceStatusProfile;

    #[test]
    fn set_string_inserts_when_absent() {
        let mut profile = ResourceStatusProfile::default();
        profile.set_string_value_by_key("SESSION_START_TS", "2026-07-30T12:00:00Z".to_string());
        assert_eq!(
            profile.string_value_by_key("SESSION_START_TS").unwrap(),
            "2026-07-30T12:00:00Z"
        );
        assert_eq!(
            profile
                .resource_status
                .unwrap()
                .string_event_and_status_ggio
                .len(),
            1
        );
    }

    #[test]
    fn set_string_updates_in_place_without_duplicating() {
        let mut profile = ResourceStatusProfile::default();
        profile.set_string_value_by_key("A", "first".to_string());
        profile.set_string_value_by_key("B", "unrelated".to_string());
        profile.set_string_value_by_key("A", "second".to_string());

        let ggio = &profile
            .resource_status
            .as_ref()
            .unwrap()
            .string_event_and_status_ggio;
        assert_eq!(
            ggio.len(),
            2,
            "updating A must not append a duplicate entry"
        );
        assert_eq!(profile.string_value_by_key("A").unwrap(), "second");
        assert_eq!(profile.string_value_by_key("B").unwrap(), "unrelated");
    }

    #[test]
    fn set_analog_roundtrips_through_getter() {
        let mut profile = ResourceStatusProfile::default();
        profile.set_analog_value_by_key("CHARGER_POWER_KW", 7.2);
        assert_eq!(
            profile.analog_value_by_key("CHARGER_POWER_KW").unwrap(),
            7.2
        );
    }

    #[test]
    fn set_integer_and_boolean_roundtrip() {
        let mut profile = ResourceStatusProfile::default();
        profile.set_integer_value_by_key("COUNT", 42);
        profile.set_boolean_value_by_key("ENABLED", true);
        assert_eq!(profile.integer_value_by_key("COUNT").unwrap(), 42);
        assert!(profile.boolean_value_by_key("ENABLED").unwrap());
    }

    #[test]
    fn get_on_missing_key_still_errors() {
        let mut profile = ResourceStatusProfile::default();
        profile.set_string_value_by_key("A", "value".to_string());
        assert!(profile.string_value_by_key("NOT_PRESENT").is_err());
    }
}
