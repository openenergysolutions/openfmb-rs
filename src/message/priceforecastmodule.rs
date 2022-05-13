use crate::commonmodule::*;
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct PriceForecastPoint {
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
    #[serde(default, rename = "dollarPerKW")]
    pub dollar_per_kw: ::std::option::Option<f32>,
}
mod price_forecast_point {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref FORECAST_TIME: crate::commonmodule::Timestamp = Default::default();
        pub(super) static ref DOLLAR_PER_KW: f32 = Default::default();
    }
}
impl PriceForecastPoint {
}
pub trait IsPriceForecastPoint {
    fn _price_forecast_point(&self) -> &PriceForecastPoint;
    fn _price_forecast_point_mut(&mut self) -> &mut PriceForecastPoint;
    fn forecast_time(&self) -> &super::commonmodule::Timestamp {
        self._price_forecast_point().forecast_time.as_ref().unwrap_or(&price_forecast_point::FORECAST_TIME)
    }
    fn forecast_time_mut(&mut self) -> &mut super::commonmodule::Timestamp {
        self._price_forecast_point_mut().forecast_time.get_or_insert(Default::default())
    }
    fn dollar_per_kw(&self) -> &f32 {
        self._price_forecast_point().dollar_per_kw.as_ref().unwrap_or(&price_forecast_point::DOLLAR_PER_KW)
    }
    fn dollar_per_kw_mut(&mut self) -> &mut f32 {
        self._price_forecast_point_mut().dollar_per_kw.get_or_insert(Default::default())
    }
}
impl IsPriceForecastPoint for PriceForecastPoint {
    fn _price_forecast_point(&self) -> &PriceForecastPoint {
        self
    }
    fn _price_forecast_point_mut(&mut self) -> &mut PriceForecastPoint {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct PriceForecastSch {
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    #[serde(default, rename = "crvPts")]
    pub crv_pts: ::std::vec::Vec<PriceForecastPoint>,
}
mod price_forecast_sch {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
impl PriceForecastSch {
}
pub trait IsPriceForecastSch {
    fn _price_forecast_sch(&self) -> &PriceForecastSch;
    fn _price_forecast_sch_mut(&mut self) -> &mut PriceForecastSch;
    fn crv_pts(&self) -> &::std::vec::Vec<PriceForecastPoint> {
        &self._price_forecast_sch().crv_pts
    }
    fn crv_pts_mut(&mut self) -> &mut ::std::vec::Vec<PriceForecastPoint> {
        &mut self._price_forecast_sch_mut().crv_pts
    }
}
impl IsPriceForecastSch for PriceForecastSch {
    fn _price_forecast_sch(&self) -> &PriceForecastSch {
        self
    }
    fn _price_forecast_sch_mut(&mut self) -> &mut PriceForecastSch {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct PriceForecast {
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
    #[serde(default, rename = "priceForecastSCH")]
    pub price_forecast_sch: ::std::option::Option<PriceForecastSch>,
}
mod price_forecast {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref FORECAST_VALUE_SOURCE: crate::commonmodule::ForecastValueSource = Default::default();
        pub(super) static ref PRICE_FORECAST_SCH: crate::priceforecastmodule::PriceForecastSch = Default::default();
    }
}
impl PriceForecast {
    pub(crate) fn parent(&self) -> &super::commonmodule::ForecastValueSource {
        self.forecast_value_source.as_ref().unwrap_or(&price_forecast::FORECAST_VALUE_SOURCE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ForecastValueSource {
        self.forecast_value_source.get_or_insert(Default::default())
    }
}
pub trait IsPriceForecast {
    fn _price_forecast(&self) -> &PriceForecast;
    fn _price_forecast_mut(&mut self) -> &mut PriceForecast;
    fn forecast_value_source(&self) -> &super::commonmodule::ForecastValueSource {
        self._price_forecast().forecast_value_source.as_ref().unwrap_or(&price_forecast::FORECAST_VALUE_SOURCE)
    }
    fn forecast_value_source_mut(&mut self) -> &mut super::commonmodule::ForecastValueSource {
        self._price_forecast_mut().forecast_value_source.get_or_insert(Default::default())
    }
    fn price_forecast_sch(&self) -> &PriceForecastSch {
        self._price_forecast().price_forecast_sch.as_ref().unwrap_or(&price_forecast::PRICE_FORECAST_SCH)
    }
    fn price_forecast_sch_mut(&mut self) -> &mut PriceForecastSch {
        self._price_forecast_mut().price_forecast_sch.get_or_insert(Default::default())
    }
}
impl IsPriceForecast for PriceForecast {
    fn _price_forecast(&self) -> &PriceForecast {
        self
    }
    fn _price_forecast_mut(&mut self) -> &mut PriceForecast {
        self
    }
}
impl IsForecastValueSource for PriceForecast {
    fn _forecast_value_source(&self) -> &super::commonmodule::ForecastValueSource {
        self.parent()
    }
    fn _forecast_value_source_mut(&mut self) -> &mut ForecastValueSource {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for PriceForecast {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct PriceForecastProfile {
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
    #[serde(default, rename = "priceForecast")]
    pub price_forecast: ::std::option::Option<PriceForecast>,
}
mod price_forecast_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref MESSAGE_INFO: crate::commonmodule::MessageInfo = Default::default();
        pub(super) static ref PRICE_FORECAST: crate::priceforecastmodule::PriceForecast = Default::default();
    }
}
impl PriceForecastProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::MessageInfo {
        self.message_info.as_ref().unwrap_or(&price_forecast_profile::MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::MessageInfo {
        self.message_info.get_or_insert(Default::default())
    }
}
pub trait IsPriceForecastProfile {
    fn _price_forecast_profile(&self) -> &PriceForecastProfile;
    fn _price_forecast_profile_mut(&mut self) -> &mut PriceForecastProfile;
    fn message_info(&self) -> &super::commonmodule::MessageInfo {
        self._price_forecast_profile().message_info.as_ref().unwrap_or(&price_forecast_profile::MESSAGE_INFO)
    }
    fn message_info_mut(&mut self) -> &mut super::commonmodule::MessageInfo {
        self._price_forecast_profile_mut().message_info.get_or_insert(Default::default())
    }
    fn price_forecast(&self) -> &PriceForecast {
        self._price_forecast_profile().price_forecast.as_ref().unwrap_or(&price_forecast_profile::PRICE_FORECAST)
    }
    fn price_forecast_mut(&mut self) -> &mut PriceForecast {
        self._price_forecast_profile_mut().price_forecast.get_or_insert(Default::default())
    }
}
impl IsPriceForecastProfile for PriceForecastProfile {
    fn _price_forecast_profile(&self) -> &PriceForecastProfile {
        self
    }
    fn _price_forecast_profile_mut(&mut self) -> &mut PriceForecastProfile {
        self
    }
}
impl IsMessageInfo for PriceForecastProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for PriceForecastProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
