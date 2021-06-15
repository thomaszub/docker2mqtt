use std::collections::HashMap;

use tokio::{sync::mpsc, task::JoinHandle};

use crate::{
    configuration::Configuration,
    docker::client::DockerHandle,
    events::{ContainerEvent, Event, EventType},
};

use super::{stream, validate};

pub async fn event(
    event: Event,
    tasks: &mut HashMap<String, JoinHandle<()>>,
    client: &DockerHandle,
    event_sender: &mpsc::Sender<Event>,
    conf: &Configuration,
) {
    match &event.event {
        EventType::State(ContainerEvent::Start) => {
            if !validate::target(&event, client, conf).await {
                return;
            }

            tasks.insert(
                event.container_name.to_owned(),
                stream::start(client.clone(), event.clone(), event_sender.clone()).await,
            );
        }
        EventType::State(ContainerEvent::Stop) => {
            tasks
                .remove(&event.container_name)
                .and_then(|handle| Some(handle.abort()));
        }
        EventType::State(ContainerEvent::Die) => {
            tasks
                .remove(&event.container_name)
                .and_then(|handle| Some(handle.abort()));
        }
        _ => {}
    }
}
