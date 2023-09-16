mod light;
mod request;

use std::error::Error;
use std::iter;
use std::time::{Duration, Instant};

use light::Light;

use itertools::{Itertools, FoldWhile::{Continue, Done}};
use rumqttc::v5::{Client, MqttOptions, Event, mqttbytes::{QoS, v5::{Publish, Packet}}};
use chrono::{Local, Datelike};

fn main() -> Result<(), Box<dyn Error>> {
    let http_rest_host = Box::leak(std::env::var("HTTP_REST_HOST").unwrap().into_boxed_str()) as &'static str;
    let http_rest_pass = Box::leak(std::env::var("HTTP_REST_PASS").unwrap().into_boxed_str()) as &'static str;
    let mqtt_name = Box::leak(std::env::var("MQTT_NAME").unwrap().into_boxed_str()) as &'static str;
    let mqtt_host = Box::leak(std::env::var("MQTT_HOST").unwrap().into_boxed_str()) as &'static str;
    let mqtt_port = Box::leak(std::env::var("MQTT_PORT").unwrap().into_boxed_str()) as &'static str;
    let mqtt_port = mqtt_port.parse::<u16>().unwrap();
    let mqtt_subcribe = Box::leak(std::env::var("MQTT_SUBSCRIBE").unwrap().into_boxed_str()) as &'static str;
    let mqtt_keep_alive = Duration::from_secs(10);
    let mqtt_interval_recv = Duration::from_millis(100);
    let light_exposure = Box::leak(std::env::var("LIGHT_EXPOSURE").unwrap().into_boxed_str()) as &'static str;
    let light_exposure = Duration::from_secs(light_exposure.parse::<u64>().unwrap());
    let light_post_exposure = Box::leak(std::env::var("LIGHT_POST_EXPOSURE").unwrap().into_boxed_str()) as &'static str;
    let light_post_exposure = light_post_exposure.parse::<u64>().unwrap();
    let light_post_exposure = Duration::from_secs(light_post_exposure);
    let light_latitude = Box::leak(std::env::var("LIGHT_LATITUDE").unwrap().into_boxed_str()) as &'static str;
    let light_latitude = light_latitude.parse::<f64>().unwrap();
    let light_longitude = Box::leak(std::env::var("LIGHT_LONGITUDE").unwrap().into_boxed_str()) as &'static str;
    let light_longitude = light_longitude.parse::<f64>().unwrap();

    Light::power_off(&http_rest_host, http_rest_pass);

    let mut mqtt_options = MqttOptions::new(mqtt_name, mqtt_host, mqtt_port);
    mqtt_options.set_keep_alive(mqtt_keep_alive);
    let (client, mut eventloop) = Client::new(mqtt_options, 10);
    
    client.subscribe(mqtt_subcribe, QoS::AtMostOnce).unwrap();

    let ref mut it_events = iter::repeat_with(|| {
        while let Ok(event) = eventloop.recv_timeout(mqtt_interval_recv) {
            match event {
                Ok(Event::Incoming(Packet::Publish(Publish {topic, ..}))) if topic == mqtt_subcribe => {
                    return Some(Instant::now())
                },
                _ => continue ,
            };
        }
        None
    });

    loop {
        if let Some(Some(was_time)) = it_events.next() {
            let local = Local::now();
            let (sunrise, sunset) = sunrise::sunrise_sunset(light_latitude, light_longitude, local.year(), local.month(), local.day());
            let sunshine: bool = sunrise <= local.timestamp() && local.timestamp() <= sunset;
            println!("sunshine {} {:?} {:?}", sunshine, sunrise, sunset);
            let sunshine: bool = false;
            if !sunshine {
                if Light::power_on(&http_rest_host, http_rest_pass).is_some() {
                    println!("PowerOn!{:?}", was_time);

                    it_events.fold_while(was_time, |was_time, event| match event {
                        Some(time) => Continue(time),
                        None if was_time + light_exposure > Instant::now() => Continue(was_time),
                        None => Done(was_time),
                    }).into_inner();

                    println!("PowerOff!{:?}", Instant::now());

                    if Light::power_off(&http_rest_host, http_rest_pass).is_some() {
                        it_events.fold_while(Instant::now() + light_post_exposure, |was_time, _event| {
                            if was_time > Instant::now() {
                                Continue(was_time)
                            } else {
                                Done(was_time)
                            }
                        }).into_inner();
                    }
                }
            }
        }
    }
}
