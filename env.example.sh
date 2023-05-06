#!/bin/sh

export HTTP_REST_HOST="http://{MY IP}/api/light"
export HTTP_REST_PASS="hinotori"
export MQTT_NAME="{MY SERVICE NAME}"
export MQTT_HOST="localhost"
export MQTT_PORT=1883
export MQTT_SUBSCRIBE="home/doorbell/motion"
export LIGHT_INTERVAL=1200
export LIGHT_EXPOSURE=5
export LIGHT_POST_EXPOSURE=30
export LIGHT_LATITUDE="{MY LATITUDE}"
export LIGHT_LONGITUDE="{MY LONGITUDE}"
