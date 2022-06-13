// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use uuid::Uuid;

pub use openfmb_messages::{Module, Profile};

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

/// A structured topic for OpenFMB which can be used with a MessageBus
/// as a BusTopic
#[derive(Clone, Debug)]
pub struct ProfileTopic {
    pub(crate) module: Module,
    pub(crate) profile: Profile,
    pub(crate) _mrid: Uuid,
    pub(crate) mrid_str: String,
}

impl ProfileTopic {
    pub fn new(module: Module, profile: Profile, mrid: Uuid) -> ProfileTopic {
        ProfileTopic {
            module,
            profile,
            _mrid: mrid,
            mrid_str: mrid.as_hyphenated().to_string().to_lowercase(),
        }
    }

    pub fn iter<'a>(&'a self) -> ProfileTopicRefIter<'a> {
        ProfileTopicRefIter {
            topic: self,
            pos: 0,
        }
    }
}

/// Iterator for a ProfileTopic that providse string references
/// and does *not* allocate beyond its state to track where it is.
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
            2 => Some(TopicLevel::Exact(self.topic.profile.as_str())),
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
