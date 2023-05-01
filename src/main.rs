#![feature(try_find)]

mod request;

use std::error::Error;
use std::{time::{Duration, Instant}, thread::sleep};

use request::Light;
use rumqttc::{Client, MqttOptions, QoS, Event, ConnectionError, mqttbytes::v4::{Packet, Publish}};

fn main() -> Result<(), Box<dyn Error>> {
    let mqtt_name = Box::leak(std::env::var("MQTT_NAME").unwrap().into_boxed_str()) as &'static str;
    let mqtt_host = Box::leak(std::env::var("MQTT_HOST").unwrap().into_boxed_str()) as &'static str;
    let mqtt_port = Box::leak(std::env::var("MQTT_PORT").unwrap().into_boxed_str()) as &'static str;
    let mqtt_port = mqtt_port.parse::<u16>().unwrap();
    let mqtt_subcribe = Box::leak(std::env::var("MQTT_SUBSCRIBE").unwrap().into_boxed_str()) as &'static str;
    let mqtt_keep_alive = Duration::from_secs(5);
    let mqtt_interval = Duration::from_millis(100);
    let http_rest_host = Box::leak(std::env::var("HTTP_REST_HOST").unwrap().into_boxed_str()) as &'static str;
    let ref http_rest_host = url::Url::parse(http_rest_host)?;
    let http_rest_pass = Box::leak(std::env::var("HTTP_REST_PASS").unwrap().into_boxed_str()) as &'static str;
    let switch_exposure = Duration::from_secs(3);
    let mut switch_lead = None;

    let mut mqtt_options = MqttOptions::new(mqtt_name, mqtt_host, mqtt_port);
    mqtt_options.set_keep_alive(mqtt_keep_alive);
    let (mut client, mut eventloop) = Client::new(mqtt_options, 10);
    
    client.subscribe(mqtt_subcribe, QoS::AtMostOnce).unwrap();

    loop {
        println!("lead: {:?}", switch_lead);
        let message: Option<Event> = eventloop.iter().try_find(|event: &Result<Event, ConnectionError>| {
            match event {
                Ok(Event::Incoming(Packet::Publish(Publish {topic, ..}))) if topic == mqtt_subcribe => Some(true),
                _ => None,
            }
        }).unwrap_or_default().and_then(|event| event.ok());

        if let Some(Event::Incoming(Packet::Publish(Publish {..}))) = message {
            if switch_lead.is_some() {
                switch_lead = Some(Instant::now() + switch_exposure);
            } else if Light::is_powered_off(http_rest_host, http_rest_pass)? {
                Light::power_on(http_rest_host, http_rest_pass)?;
                switch_lead = Some(Instant::now() + switch_exposure);
            } 
        }

        match switch_lead {
            Some(exposition) if exposition < Instant::now() => {
                Light::power_off(http_rest_host, http_rest_pass)?;
                switch_lead = None;
            },
            _ => {},
        }

        sleep(mqtt_interval);
    }
}
