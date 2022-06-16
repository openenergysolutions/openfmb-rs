// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    resourcemodule::{
        ResourceDiscreteControlProfile, ResourceEventProfile, ResourceReadingProfile,
        ResourceStatusProfile,
    },
    Module, Profile,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Resource<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<ResourceStatusProfile>
        + Publisher<ResourceEventProfile>
        + Publisher<ResourceReadingProfile>
        + Subscriber<ResourceDiscreteControlProfile>,
{
    bus: MB,
    _mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    discrete_control_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::ResourceModule, profile, mrid.clone())
}

impl<MB> Resource<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<ResourceStatusProfile>
        + Publisher<ResourceEventProfile>
        + Publisher<ResourceReadingProfile>
        + Subscriber<ResourceDiscreteControlProfile>,
{
    /// Create a new resource client instance
    pub fn new(bus: MB, mrid: Uuid) -> Resource<MB> {
        Resource {
            bus,
            _mrid: mrid,
            status_topic: topic(Profile::ResourceStatusProfile, &mrid),
            event_topic: topic(Profile::ResourceEventProfile, &mrid),
            reading_topic: topic(Profile::ResourceReadingProfile, &mrid),
            discrete_control_topic: topic(Profile::ResourceDiscreteControlProfile, &mrid),
        }
    }

    /// publish resource status messages
    pub async fn status(&mut self, msg: ResourceStatusProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.status_topic.iter(), msg).await?)
    }

    /// publish resource event messages
    pub async fn event(&mut self, msg: ResourceEventProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.event_topic.iter(), msg).await?)
    }

    /// publish resource reading messages
    pub async fn reading(&mut self, msg: ResourceReadingProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.reading_topic.iter(), msg).await?)
    }

    /// subscribe to discrete control messages
    pub async fn discrete_control(&mut self) -> SubscribeResult<ResourceDiscreteControlProfile> {
        self.bus.subscribe(self.discrete_control_topic.iter()).await
    }
}
