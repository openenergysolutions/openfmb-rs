// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use uuid::Uuid;

/// Each level in a topic/subject in the pub/sub bus may contain either an
/// exact string value or wildcard. The exact string value is described in generic
/// terms as something that can be referenced as a &str  (impl AsRef<str> for T)
///
/// From the docs this allows using String or &str:
/// https://doc.rust-lang.org/std/convert/trait.AsRef.html#examples
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TopicLevel<S: AsRef<str>> {
    Exact(S),
    WildCard,
}

/// A Topic is a trait describes a topic as something that can be treated as an Iterator
/// with the added function noting if the topic is a prefix matching topic. It must be Send
/// as it needs to potentially be passed across threads (or async bounds)
pub trait Topic<S: AsRef<str>>: Send + Iterator<Item = TopicLevel<S>> {
    fn prefix_match(&self) -> bool;
}

/// OpenFMB Modules
#[derive(Copy, Clone, Debug)]
pub enum Module {
    Breaker,
    CapBank,
    Common,
    CoordinationService,
    ESS,
    Generation,
    Interconnection,
    Load,
    Meter,
    Recloser,
    Regulator,
    Reserve,
    Resource,
    Solar,
    Switch,
}

impl Module {
    fn as_str(&self) -> &'static str {
        match *self {
            Module::Breaker => "breakermodule",
            Module::CapBank => "capbankmodule",
            Module::Common => "commonmodule",
            Module::CoordinationService => "coordinationservicemodule",
            Module::ESS => "essmodule",
            Module::Generation => "generationmodule",
            Module::Interconnection => "interconnectionmodule",
            Module::Load => "loadmodule",
            Module::Meter => "metermodule",
            Module::Recloser => "reclosermodule",
            Module::Regulator => "regulatormodule",
            Module::Reserve => "reservemodule",
            Module::Resource => "resourcemodule",
            Module::Solar => "solarmodule",
            Module::Switch => "switchmodule",
        }
    }

    fn as_camel_str(&self) -> &'static str {
        match *self {
            Module::Breaker => "Breaker",
            Module::CapBank => "CapBank",
            Module::Common => "Common",
            Module::CoordinationService => "CoordinationService",
            Module::ESS => "ESS",
            Module::Generation => "Generation",
            Module::Interconnection => "Interconnection",
            Module::Load => "Load",
            Module::Meter => "Meter",
            Module::Recloser => "Recloser",
            Module::Regulator => "Regulator",
            Module::Reserve => "Reserve",
            Module::Resource => "Resource",
            Module::Solar => "Solar",
            Module::Switch => "Switch",
        }
    }
}

/// OpenFMB Profiles
#[derive(Copy, Clone, Debug)]
pub enum Profile {
    Reading,
    Control,
    DiscreteControl,
    Status,
    Event,
}

impl Profile {
    fn as_camel_str(&self) -> &str {
        match *self {
            Profile::Reading => "Reading",
            Profile::DiscreteControl => "DiscreteControl",
            Profile::Control => "Control",
            Profile::Status => "Status",
            Profile::Event => "Event",
        }
    }
}

/// A structured topic for OpenFMB which can be used with a MessageBus
/// as a BusTopic
#[derive(Clone, Debug)]
pub struct ProfileTopic {
    pub(crate) module: Module,
    pub(crate) profile: Profile,
    pub(crate) mrid: Uuid,
    pub(crate) profile_str: String,
    pub(crate) mrid_str: String,
}

impl ProfileTopic {
    pub fn new(module: Module, profile: Profile, mrid: Uuid) -> ProfileTopic {
        ProfileTopic {
            module,
            profile,
            mrid,
            profile_str: [module.as_camel_str(), profile.as_camel_str()].join(""),
            mrid_str: mrid.to_hyphenated().to_string().to_lowercase(),
        }
    }

    pub fn iter<'a>(&'a self) -> ProfileTopicRefIter<'a> {
        ProfileTopicRefIter {
            topic: self,
            pos: 0,
        }
    }
}

pub struct ProfileTopicRefIter<'a> {
    topic: &'a ProfileTopic,
    pos: u8,
}

impl<'a> Iterator for ProfileTopicRefIter<'a> {
    type Item = TopicLevel<&'a str>;

    fn next(&mut self) -> Option<TopicLevel<&'a str>> {
        let ret = match self.pos {
            0 => Some(TopicLevel::Exact("openfmb")),
            1 => Some(TopicLevel::Exact(self.topic.module.as_str())),
            2 => Some(TopicLevel::Exact(self.topic.profile_str.as_str())),
            3 => Some(TopicLevel::Exact(self.topic.mrid_str.as_str())),
            _ => None,
        };
        self.pos += 1;
        ret
    }
}

impl<'a> Topic<&'a str> for ProfileTopicRefIter<'a> {
    fn prefix_match(&self) -> bool {
        false
    }
}
