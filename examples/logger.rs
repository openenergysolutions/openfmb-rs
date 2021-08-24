// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use futures::StreamExt;
use log::info;
use openfmb::{encoding::ProtobufVariantEncoding, messages::ProfileMessage, prelude::*};
use std::env;

struct OpenFMBEverything {
    done: bool,
}

impl Iterator for OpenFMBEverything {
    type Item = TopicLevel<&'static str>;

    fn next(&mut self) -> Option<TopicLevel<&'static str>> {
        if !self.done {
            self.done = true;
            Some(TopicLevel::Exact("openfmb"))
        } else {
            None
        }
    }
}

impl Topic<&'static str> for OpenFMBEverything {
    fn prefix_match(&self) -> bool {
        true
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let nats_url = env::var("NATS_URL")?;
    let nc = nats::asynk::connect(&nats_url).await?;
    let mut bus = openfmb::bus::NatsBus::<ProtobufVariantEncoding>::new(nc);
    let openfmb_everything_topic = OpenFMBEverything { done: false };
    let mut openfmb_stream = bus.subscribe(openfmb_everything_topic).await?;
    info!(
        "Connected to OpenFMB Bus and subscribed to all using nats at {:?}",
        nats_url
    );
    while let Some(msg) = openfmb_stream.next().await {
        let msg: ProfileMessage = msg?;
        info!("{:?}", msg);
    }
    Ok(())
}
