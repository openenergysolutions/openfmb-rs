use crate::commonmodule::*;
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadForecastPoint {
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "forecastTime")]
    pub forecast_time: ::std::option::Option<super::commonmodule::Timestamp>,
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub watt: ::std::option::Option<f32>,
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub vars: ::std::option::Option<f32>,
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub watt_hrs: ::std::option::Option<f32>,
}
mod load_forecast_point {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref FORECAST_TIME: crate::commonmodule::Timestamp = Default::default();
        pub(super) static ref WATT: f32 = Default::default();
        pub(super) static ref VARS: f32 = Default::default();
        pub(super) static ref WATT_HRS: f32 = Default::default();
    }
}
impl LoadForecastPoint {
}
pub trait IsLoadForecastPoint {
    fn _load_forecast_point(&self) -> &LoadForecastPoint;
    fn _load_forecast_point_mut(&mut self) -> &mut LoadForecastPoint;
    fn forecast_time(&self) -> &super::commonmodule::Timestamp {
        self._load_forecast_point().forecast_time.as_ref().unwrap_or(&load_forecast_point::FORECAST_TIME)
    }
    fn forecast_time_mut(&mut self) -> &mut super::commonmodule::Timestamp {
        self._load_forecast_point_mut().forecast_time.get_or_insert(Default::default())
    }
    fn watt(&self) -> &f32 {
        self._load_forecast_point().watt.as_ref().unwrap_or(&load_forecast_point::WATT)
    }
    fn watt_mut(&mut self) -> &mut f32 {
        self._load_forecast_point_mut().watt.get_or_insert(Default::default())
    }
    fn vars(&self) -> &f32 {
        self._load_forecast_point().vars.as_ref().unwrap_or(&load_forecast_point::VARS)
    }
    fn vars_mut(&mut self) -> &mut f32 {
        self._load_forecast_point_mut().vars.get_or_insert(Default::default())
    }
    fn watt_hrs(&self) -> &f32 {
        self._load_forecast_point().watt_hrs.as_ref().unwrap_or(&load_forecast_point::WATT_HRS)
    }
    fn watt_hrs_mut(&mut self) -> &mut f32 {
        self._load_forecast_point_mut().watt_hrs.get_or_insert(Default::default())
    }
}
impl IsLoadForecastPoint for LoadForecastPoint {
    fn _load_forecast_point(&self) -> &LoadForecastPoint {
        self
    }
    fn _load_forecast_point_mut(&mut self) -> &mut LoadForecastPoint {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadForecastSch {
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    #[serde(default, rename = "crvPts")]
    pub crv_pts: ::std::vec::Vec<LoadForecastPoint>,
}
mod load_forecast_sch {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
impl LoadForecastSch {
}
pub trait IsLoadForecastSch {
    fn _load_forecast_sch(&self) -> &LoadForecastSch;
    fn _load_forecast_sch_mut(&mut self) -> &mut LoadForecastSch;
    fn crv_pts(&self) -> &::std::vec::Vec<LoadForecastPoint> {
        &self._load_forecast_sch().crv_pts
    }
    fn crv_pts_mut(&mut self) -> &mut ::std::vec::Vec<LoadForecastPoint> {
        &mut self._load_forecast_sch_mut().crv_pts
    }
}
impl IsLoadForecastSch for LoadForecastSch {
    fn _load_forecast_sch(&self) -> &LoadForecastSch {
        self
    }
    fn _load_forecast_sch_mut(&mut self) -> &mut LoadForecastSch {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadForecast {
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "forecastValueSource")]
    pub forecast_value_source: ::std::option::Option<super::commonmodule::ForecastValueSource>,
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "loadForecastSCH")]
    pub load_forecast_sch: ::std::option::Option<LoadForecastSch>,
}
mod load_forecast {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref FORECAST_VALUE_SOURCE: crate::commonmodule::ForecastValueSource = Default::default();
        pub(super) static ref LOAD_FORECAST_SCH: crate::loadforecastmodule::LoadForecastSch = Default::default();
    }
}
impl LoadForecast {
    pub(crate) fn parent(&self) -> &super::commonmodule::ForecastValueSource {
        self.forecast_value_source.as_ref().unwrap_or(&load_forecast::FORECAST_VALUE_SOURCE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ForecastValueSource {
        self.forecast_value_source.get_or_insert(Default::default())
    }
}
pub trait IsLoadForecast {
    fn _load_forecast(&self) -> &LoadForecast;
    fn _load_forecast_mut(&mut self) -> &mut LoadForecast;
    fn forecast_value_source(&self) -> &super::commonmodule::ForecastValueSource {
        self._load_forecast().forecast_value_source.as_ref().unwrap_or(&load_forecast::FORECAST_VALUE_SOURCE)
    }
    fn forecast_value_source_mut(&mut self) -> &mut super::commonmodule::ForecastValueSource {
        self._load_forecast_mut().forecast_value_source.get_or_insert(Default::default())
    }
    fn load_forecast_sch(&self) -> &LoadForecastSch {
        self._load_forecast().load_forecast_sch.as_ref().unwrap_or(&load_forecast::LOAD_FORECAST_SCH)
    }
    fn load_forecast_sch_mut(&mut self) -> &mut LoadForecastSch {
        self._load_forecast_mut().load_forecast_sch.get_or_insert(Default::default())
    }
}
impl IsLoadForecast for LoadForecast {
    fn _load_forecast(&self) -> &LoadForecast {
        self
    }
    fn _load_forecast_mut(&mut self) -> &mut LoadForecast {
        self
    }
}
impl IsForecastValueSource for LoadForecast {
    fn _forecast_value_source(&self) -> &super::commonmodule::ForecastValueSource {
        self.parent()
    }
    fn _forecast_value_source_mut(&mut self) -> &mut ForecastValueSource {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for LoadForecast {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadForecastProfile {
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "messageInfo")]
    pub message_info: ::std::option::Option<super::commonmodule::MessageInfo>,
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "loadForecast")]
    pub load_forecast: ::std::option::Option<LoadForecast>,
}
mod load_forecast_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref MESSAGE_INFO: crate::commonmodule::MessageInfo = Default::default();
        pub(super) static ref LOAD_FORECAST: crate::loadforecastmodule::LoadForecast = Default::default();
    }
}
impl LoadForecastProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::MessageInfo {
        self.message_info.as_ref().unwrap_or(&load_forecast_profile::MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::MessageInfo {
        self.message_info.get_or_insert(Default::default())
    }
}
pub trait IsLoadForecastProfile {
    fn _load_forecast_profile(&self) -> &LoadForecastProfile;
    fn _load_forecast_profile_mut(&mut self) -> &mut LoadForecastProfile;
    fn message_info(&self) -> &super::commonmodule::MessageInfo {
        self._load_forecast_profile().message_info.as_ref().unwrap_or(&load_forecast_profile::MESSAGE_INFO)
    }
    fn message_info_mut(&mut self) -> &mut super::commonmodule::MessageInfo {
        self._load_forecast_profile_mut().message_info.get_or_insert(Default::default())
    }
    fn load_forecast(&self) -> &LoadForecast {
        self._load_forecast_profile().load_forecast.as_ref().unwrap_or(&load_forecast_profile::LOAD_FORECAST)
    }
    fn load_forecast_mut(&mut self) -> &mut LoadForecast {
        self._load_forecast_profile_mut().load_forecast.get_or_insert(Default::default())
    }
}
impl IsLoadForecastProfile for LoadForecastProfile {
    fn _load_forecast_profile(&self) -> &LoadForecastProfile {
        self
    }
    fn _load_forecast_profile_mut(&mut self) -> &mut LoadForecastProfile {
        self
    }
}
impl IsMessageInfo for LoadForecastProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for LoadForecastProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
