// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use async_trait::async_trait;
use futures::StreamExt;
use std::convert::TryInto;
use std::marker::PhantomData;
use zenoh::*;

use lazy_static::lazy_static;

lazy_static! {
    static ref ZENOH: Zenoh = Zenoh::new(net::config::default()).wait().unwrap();
    static ref WORKSPACE: Workspace<'static> = ZENOH.workspace(None).wait().unwrap();
}

/// Zenoh Message Bus
#[derive(Debug)]
pub struct ZenohBus<E: MessageEncoding> {
    encoding: PhantomData<E>,
}

fn topic_to_subject<S: AsRef<str>, T: Topic<S>>(topic: T) -> String {
    let prefix_match = topic.prefix_match();
    let mut subject = String::with_capacity(128);
    topic.for_each(|lvl| match lvl {
        TopicLevel::Exact(val) => {
            subject.push_str(val.as_ref());
            subject.push('/');
        }
        TopicLevel::WildCard => subject.push_str("*/"),
    });

    if prefix_match {
        subject.push_str("**");
    } else {
        subject.pop();
    }
    subject
}

fn topic_level<'a>(level: &'a str) -> TopicLevel<&'a str> {
    match level {
        "*" => TopicLevel::WildCard,
        val => TopicLevel::Exact(val),
    }
}

#[derive(Debug)]
pub struct SubjectIter<'s> {
    iter: std::iter::Map<std::str::Split<'s, char>, for<'a> fn(&'a str) -> TopicLevel<&'a str>>,
}

impl<'s> Iterator for SubjectIter<'s> {
    type Item = TopicLevel<&'s str>;
    fn next(&mut self) -> Option<TopicLevel<&'s str>> {
        self.iter.next()
    }
}

impl<'a> Topic<&'a str> for SubjectIter<'a> {
    fn prefix_match(&self) -> bool {
        false
    }
}

fn subject_to_topic<'a>(subject: &'a zenoh::Path) -> SubjectIter<'a> {
    SubjectIter {
        iter: subject.as_str().split('/').map(topic_level),
    }
}

#[async_trait]
impl<E, M> Subscriber<M> for ZenohBus<E>
where
    E: 'static + MessageEncoding + Send,
    M: 'static + Message<E> + Send,
{
    async fn subscribe<S: AsRef<str>, T: Topic<S>>(
        &mut self,
        topic: T,
    ) -> Result<Subscription<M>, SubscribeError> {
        //debug!("subscribing to {:?}", topic);
        let subject: String = topic_to_subject(topic);
        let stream = WORKSPACE
            .subscribe(&subject.try_into().unwrap())
            .await
            .map_err(|err| SubscribeError::BusError(Box::new(err)))?;
        Ok(Box::pin(stream.map(|change| {
            let topic = subject_to_topic(&change.path);
            let mut raw = vec![];
            let data: &[u8] = match change.value {
                Some(val) => match val {
                    Value::Raw(_c, mut buf) => {
                        raw = buf.read_vec();
                        &raw
                    }
                    _ => &raw,
                },
                _ => &raw,
            };
            M::decode(topic, data).map_err(|err| SubscriptionError::DecodeError(Box::new(err)))
        })))
    }
}

#[async_trait]
impl<E, M> Publisher<M> for ZenohBus<E>
where
    E: 'static + MessageEncoding + Send,
    M: 'static + Message<E> + Send,
{
    async fn publish<S: AsRef<str>, T: Topic<S>>(&mut self, topic: T, msg: M) -> PublishResult<()> {
        // 512 bytes is a reasonable guess, on average we saw about ~400 byte message sizes for openfmb
        // if we had a "max size" message we could statically allocate a buffer and perhaps avoid the heap
        // here
        let mut buf = Vec::with_capacity(512);
        msg.encode(&mut buf)
            .map_err(|err| PublishError::EncodeError(Box::new(err)))?;
        let subject: String = topic_to_subject(topic);
        let workspace = ZENOH.workspace(None).await?;
        Ok(workspace.put(&subject.try_into()?, buf.into()).await?)
    }
}

/// A zenoh connection implements the MessageBus trait
impl<E, M> MessageBus<M> for ZenohBus<E>
where
    E: 'static + MessageEncoding + Send,
    M: 'static + Send + Message<E>,
{
}

impl<E> ZenohBus<E>
where
    E: 'static + MessageEncoding + Send,
{
    pub fn new() -> ZenohBus<E> {
        ZenohBus {
            encoding: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Topic, TopicLevel};

    use super::{subject_to_topic, topic_to_subject};

    #[test]
    fn test_subject_conversion() {
        assert_eq!(
            topic_to_subject(subject_to_topic(&zenoh::Path::new("x/y/z").unwrap())),
            "x/y/z"
        );
        assert_eq!(
            topic_to_subject(subject_to_topic(&zenoh::Path::new("x").unwrap())),
            "x"
        );
        assert_eq!(
            subject_to_topic(&zenoh::Path::new("x/y").unwrap()).prefix_match(),
            false
        );

        let path = zenoh::Path::new("x/y").unwrap();
        let parts0: Vec<TopicLevel<&str>> = subject_to_topic(&path).collect();
        assert_eq!(parts0, vec![TopicLevel::Exact("x"), TopicLevel::Exact("y")]);
    }
}
