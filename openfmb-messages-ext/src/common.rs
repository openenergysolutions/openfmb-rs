// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::error::*;
use openfmb_messages::commonmodule::{
    BehaviourModeKind, ConductingEquipment, EnergyConsumer, Ess, HealthKind, IdentifiedObject, Ied,
    LogicalNode, LogicalNodeForEventAndStatus, MessageInfo, Meter, NamedObject, ReadingMessageInfo,
    StatusMessageInfo,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

/// Has IdentifiedObject
pub trait HasIdentifiedObject {
    /// Get the identified object from the parent object
    fn get_identified_object(&self) -> OpenFMBResult<&IdentifiedObject>;

    /// Get the mrid of the identified object
    fn get_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::parse_str(
            self.get_identified_object()?
                .m_rid
                .as_ref()
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }

    /// Get the name of the identified object
    fn get_name(&self) -> OpenFMBResult<&str> {
        Ok(self
            .get_identified_object()?
            .name
            .as_ref()
            .context(NoName)?)
    }

    /// Get the description of the identifed object
    fn get_description(&self) -> OpenFMBResult<&str> {
        Ok(self
            .get_identified_object()?
            .description
            .as_ref()
            .context(NoName)?)
    }
}

/// Has LogicalNode
pub trait HasLogicalNode {
    fn get_logical_node(&self) -> OpenFMBResult<&LogicalNode>;
}

/// Has LogicalNodeForEventAndStatus
pub trait HasLogicalNodeForEventAndStatus {
    fn get_logical_node_for_event_and_status(&self)
        -> OpenFMBResult<&LogicalNodeForEventAndStatus>;
}
/// Has BehaviourModeKind
pub trait HasBehaviourModeKind {
    fn get_behaviour_mode_kind(&self) -> OpenFMBResult<BehaviourModeKind>;
}

/// Has HealthKind
pub trait HasHealthKind {
    fn get_health_kind(&self) -> OpenFMBResult<HealthKind>;
}

/// Has ConductingEquipment
pub trait HasConductingEquipment {
    fn get_conducting_equipment(&self) -> OpenFMBResult<&ConductingEquipment>;
}

/// Has NamedObject
pub trait HasNamedObject {
    fn get_named_object(&self) -> OpenFMBResult<&NamedObject>;
}

/// Has a device name
pub trait HasDeviceName {
    fn get_device_name(&self) -> OpenFMBResult<&str>;
}

/// Has a device MRID
pub trait HasDeviceMRID {
    fn get_device_mrid(&self) -> OpenFMBResult<Uuid>;
}

/// Has ReadingMessageInfo
pub trait HasReadingMessageInfo {
    fn get_reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo>;
}

/// Has StatusMessageInfo
pub trait HasStatusMessageInfo {
    fn get_status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo>;
}

/// Has Ied
pub trait HasIED {
    fn get_ied(&self) -> OpenFMBResult<&Ied>;
}

/// Has MessageInfo
pub trait HasMessageInfo {
    fn get_message_info(&self) -> OpenFMBResult<&MessageInfo>;

    fn get_message_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(self.get_message_info()?.get_mrid()?)
    }
}

/// Default implementation for types that implement HasStatusMessageInfo
impl HasMessageInfo for dyn HasStatusMessageInfo {
    fn get_message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .get_status_message_info()?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }
}

/// Default implementation for types that implement HasReadingMessageInfo
impl HasMessageInfo for dyn HasReadingMessageInfo {
    fn get_message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .get_reading_message_info()?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }
}

/// Default implementation for types that implement HasConductingEquipment
impl<T: HasConductingEquipment> HasNamedObject for T {
    fn get_named_object(&self) -> OpenFMBResult<&NamedObject> {
        Ok(self
            .get_conducting_equipment()?
            .named_object
            .as_ref()
            .context(NoNamedObject)?)
    }
}

/// Default implementation for types that implement HasConductingEquipment
impl<T: HasConductingEquipment> HasDeviceMRID for T {
    fn get_device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::parse_str(self.get_conducting_equipment()?.m_rid.as_ref()).context(UuidError)?)
    }
}

/// Default implementation for types that implement HasNamedObject
impl<T: HasNamedObject> HasDeviceName for T {
    fn get_device_name(&self) -> OpenFMBResult<&str> {
        Ok(self.get_named_object()?.name.as_ref().context(NoName)?)
    }
}

/// Default implementation for types that implements HasLogicalNodeForEventAndStatus
impl<T: HasLogicalNodeForEventAndStatus> HasHealthKind for T {
    fn get_health_kind(&self) -> OpenFMBResult<HealthKind> {
        Ok(self
            .get_logical_node_for_event_and_status()?
            .ee_health
            .as_ref()
            .context(NoEeHealth)?
            .st_val())
    }
}
/// Default implementation for types that implements HasLogicalNodeForEventAndStatus
impl<T: HasLogicalNodeForEventAndStatus> HasBehaviourModeKind for T {
    fn get_behaviour_mode_kind(&self) -> OpenFMBResult<BehaviourModeKind> {
        Ok(self
            .get_logical_node_for_event_and_status()?
            .beh
            .as_ref()
            .context(NoBeh)?
            .st_val())
    }
}

/// Default implementation for types that implements HasLogicalNodeForEventAndStatus
impl<T: HasLogicalNodeForEventAndStatus> HasLogicalNode for T {
    fn get_logical_node(&self) -> OpenFMBResult<&LogicalNode> {
        self.get_logical_node_for_event_and_status()?
            .logical_node
            .as_ref()
            .context(NoLogicalNode)
    }
}

/// Default implementation of HasIdentifiedObject for types that implements HasLogicalNode
impl<T: HasLogicalNode> HasIdentifiedObject for T {
    fn get_identified_object(&self) -> OpenFMBResult<&IdentifiedObject> {
        self.get_logical_node()?
            .identified_object
            .as_ref()
            .context(NoIdentifiedObject)
    }
}

/// MessageInfo has IdentifiedObject
impl HasIdentifiedObject for MessageInfo {
    fn get_identified_object(&self) -> OpenFMBResult<&IdentifiedObject> {
        self.identified_object.as_ref().context(NoIdentifiedObject)
    }
}

/// Meter has ConductingEquipment
impl HasConductingEquipment for Meter {
    fn get_conducting_equipment(&self) -> OpenFMBResult<&ConductingEquipment> {
        Ok(self
            .conducting_equipment
            .as_ref()
            .context(NoConductingEquipment)?)
    }
}

/// EnergyConsumer has ConductingEquipment
impl HasConductingEquipment for EnergyConsumer {
    fn get_conducting_equipment(&self) -> OpenFMBResult<&ConductingEquipment> {
        Ok(self
            .conducting_equipment
            .as_ref()
            .context(NoConductingEquipment)?)
    }
}

/// Ess has ConductingEquipment
impl HasConductingEquipment for Ess {
    fn get_conducting_equipment(&self) -> OpenFMBResult<&ConductingEquipment> {
        Ok(self
            .conducting_equipment
            .as_ref()
            .context(NoConductingEquipment)?)
    }
}

/// Ied has IdentifiedObject
impl HasIdentifiedObject for Ied {
    fn get_identified_object(&self) -> OpenFMBResult<&IdentifiedObject> {
        Ok(self
            .identified_object
            .as_ref()
            .context(NoIdentifiedObject)?)
    }
}
