// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use futures::{stream, StreamExt};

use openfmb_messages::{
    commonmodule::GridConnectModeKind,
    priceforecastmodule::{PriceForecastProfile, PriceForecastPoint},
};

//use openfmb_messages_ext::PriceForecastControlExt;
use std::time::SystemTime;
use uuid::Uuid;

/// Control and wait on updates from ESS (battery/storage)
///
/// Every function implies a request for the next future state of the ess
/// rather than the last seen state. A variant of this interface could possibly
/// look at the *last* known status and reading instead.
///
/// When writing control algorithms however it is easier determine the next
/// known good value and *then* control it rather than looking at an old status
/// which may be too old to be useful.
pub struct PriceForecast<MB>
where
    MB: Subscriber<PriceForecastProfile>,
{
    bus: MB,
    mrid: Uuid,
    forecast_topic: ProfileTopic,
}

/// Topic string given a message type and mrid
pub fn topic(profile: topic::Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::PriceForecastModule, profile, mrid.clone())
}

impl<MB> PriceForecast<MB>
where
    MB: Subscriber<PriceForecastProfile>,
{
    /// Create a new switch client instance
    pub fn new(bus: MB, mrid: Uuid) -> PriceForecast<MB> {
        PriceForecast { 
            bus, 
            mrid, 
            forecast_topic: topic(Profile::PriceForecastProfile, &mrid),
        }
    }

    /// Get the device MRID as a string
    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.to_hyphenated())
    }

    /// A stream to this devices forecast messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn forecast(&mut self) -> SubscribeResult<PriceForecastProfile> {
        self
            .bus
            .subscribe(self.forecast_topic.iter())
            .await
    }
//    price_forecast.price_forecast_sch.crv_pts.sfp_vec (Vec<PriceForecastPoint>)
    pub async fn forecastpoints(&mut self) -> SubscribeResult<Vec<PriceForecastPoint>> {
        let sf_vec = self.forecast().await?.map(|v| match v {
            Ok(v) => Ok(v
                .price_forecast.unwrap()
                .price_forecast_sch.unwrap()
                .crv_pts),
            Err(err) => Err(err),
        });
        Ok(Box::pin(sf_vec))
    }

    

}
