mod request;

use std::error::Error;
use std::iter;
use std::{time::{Duration, Instant}, thread::sleep};

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
use request::Light;
use rumqttc::{Client, MqttOptions, QoS, Event, mqttbytes::v4::{Packet, Publish}};

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
    let switch_post_exposure = 10;

    let mut mqtt_options = MqttOptions::new(mqtt_name, mqtt_host, mqtt_port);
    mqtt_options.set_keep_alive(mqtt_keep_alive);
    let (mut client, mut eventloop) = Client::new(mqtt_options, 10);
    
    client.subscribe(mqtt_subcribe, QoS::AtMostOnce).unwrap();

    let ref mut it_event = iter::repeat_with(|| {
        sleep(mqtt_interval);

        match eventloop.recv_timeout(mqtt_interval) {
            Ok(Ok(Event::Incoming(Packet::Publish(Publish {topic, ..})))) if topic == mqtt_subcribe => Some(Instant::now()),
            _ => None,
        }
    });
    loop {
        println!("idle");
        if let Some(Some(was_time)) = it_event.next() {
            println!("PowerOn!{:?}", was_time);

            Light::power_on(&http_rest_host, http_rest_pass)?;

            let was_time: Instant = it_event.fold_while(was_time, |was_time, event| match event {
                Some(time) => Continue(time),
                None if was_time + switch_exposure > Instant::now() => Continue(was_time),
                None => Done(was_time),
            }).into_inner();

            println!("PowerOff!{:?}", was_time);

            Light::power_off(&http_rest_host, http_rest_pass)?;

            it_event.dropping(switch_post_exposure);

            println!("PowerPostOff!");
        }
    }
}
