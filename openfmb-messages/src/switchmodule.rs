/// OpenFMB specialization for switch control:  LN: Circuit switch   Name: XSWI
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchDiscreteControlXswi {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::std::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub pos: ::std::option::Option<super::commonmodule::ControlDpc>,
}
/// Switch discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchDiscreteControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub switch_discrete_control_xswi: ::std::option::Option<SwitchDiscreteControlXswi>,
}
/// A ProtectedSwitch is a switching device that can be operated by ProtectionEquipment.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProtectedSwitch {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
}
/// Switch control profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchDiscreteControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub switch_discrete_control: ::std::option::Option<SwitchDiscreteControl>,
}
/// OpenFMB specialization for SwitchEventProfile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchEventXswi {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// Dynamic test status
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub pos: ::std::option::Option<super::commonmodule::StatusDps>,
}
/// Switch event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub switch_event_xswi: ::std::option::Option<SwitchEventXswi>,
}
/// Switch event profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub switch_event: ::std::option::Option<SwitchEvent>,
}
/// Switch reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchReading {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment_terminal_reading: ::std::option::Option<super::commonmodule::ConductingEquipmentTerminalReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub diff_reading_mmxu: ::std::option::Option<super::commonmodule::ReadingMmxu>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub phase_mmtn: ::std::option::Option<super::commonmodule::PhaseMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub reading_mmtr: ::std::option::Option<super::commonmodule::ReadingMmtr>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub reading_mmxu: ::std::option::Option<super::commonmodule::ReadingMmxu>,
}
/// Switch reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::std::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="4")]
    pub switch_reading: ::std::vec::Vec<SwitchReading>,
}
/// OpenFMB specialization for SwitchStatusProfile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchStatusXswi {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub pos: ::std::option::Option<super::commonmodule::StatusDps>,
}
/// Switch status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::std::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub switch_status_xswi: ::std::option::Option<SwitchStatusXswi>,
}
/// Switch status profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::std::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub switch_status: ::std::option::Option<SwitchStatus>,
}
/// Specialized 61850 FSCC class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::std::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub switch_control_schedule_fsch: ::std::option::Option<super::commonmodule::SwitchControlScheduleFsch>,
}
/// Switch discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub switch_control_fscc: ::std::option::Option<SwitchControlFscc>,
}
/// Switch control profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub switch_control: ::std::option::Option<SwitchControl>,
}
