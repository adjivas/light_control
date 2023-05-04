mod request;

use std::error::Error;
use std::iter;
use std::{time::{Duration, Instant}, thread::sleep};

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
use request::Light;
use rumqttc::v5::{Client, MqttOptions, Event};
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::mqttbytes::v5::{Publish, Packet};

fn main() -> Result<(), Box<dyn Error>> {
    let mqtt_name = Box::leak(std::env::var("MQTT_NAME").unwrap().into_boxed_str()) as &'static str;
    let mqtt_host = Box::leak(std::env::var("MQTT_HOST").unwrap().into_boxed_str()) as &'static str;
    let mqtt_port = Box::leak(std::env::var("MQTT_PORT").unwrap().into_boxed_str()) as &'static str;
    let mqtt_port = mqtt_port.parse::<u16>().unwrap();
    let mqtt_subcribe = Box::leak(std::env::var("MQTT_SUBSCRIBE").unwrap().into_boxed_str()) as &'static str;
    let mqtt_keep_alive = Duration::from_secs(10);
    let mqtt_interval = Duration::from_millis(100);
    let http_rest_host = Box::leak(std::env::var("HTTP_REST_HOST").unwrap().into_boxed_str()) as &'static str;
    let http_rest_pass = Box::leak(std::env::var("HTTP_REST_PASS").unwrap().into_boxed_str()) as &'static str;
    let switch_exposure = Duration::from_secs(5);
    let switch_post_exposure = 30;

    let mut mqtt_options = MqttOptions::new(mqtt_name, mqtt_host, mqtt_port);
    mqtt_options.set_keep_alive(mqtt_keep_alive);
    let (client, mut eventloop) = Client::new(mqtt_options, 10);
    
    client.subscribe(mqtt_subcribe, QoS::AtMostOnce).unwrap();

    let ref mut it_events = iter::repeat_with(|| {
        sleep(mqtt_interval);
        match eventloop.recv_timeout(mqtt_interval) {
            Ok(Ok(Event::Incoming(Packet::Publish(Publish {topic, ..})))) if topic == mqtt_subcribe => Some(Instant::now()),
            _ => None,
        }
    });
    loop {
        if let Some(Some(was_time)) = it_events.next() {
            if Light::power_on(&http_rest_host, http_rest_pass).is_some() {
                println!("PowerOn!{:?}", was_time);

                it_events.fold_while(was_time, |was_time, event| match event {
                    Some(time) => Continue(time),
                    None if was_time + switch_exposure > Instant::now() => Continue(was_time),
                    None => Done(was_time),
                }).into_inner();

                println!("PowerOff!{:?}", Instant::now());

                if Light::power_off(&http_rest_host, http_rest_pass).is_some() {
                    it_events.dropping(switch_post_exposure);

                    println!("PowerPostOff!{:?}", Instant::now());
                }
            }
        }
    }
}
