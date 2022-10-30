# Intro
This program reads information from a Finder 7M.24 power meter connected to a RS485/modbus <-> TCP gateway and saves the output in PostgreSQL database.

# Usage
- Set up power meter and gateway (Out of scope, but make sure power meter is installed by a certified professional!)
- Set up psql database (timescale support recommended)
- Checkout repo
- `cargo build`
- Edit config
- Run with cron in interval you like
    
# Reference material
- https://www.findernet.com/en/canada/series/7m-series-smart-energy-meters/
- https://www.waveshare.com/wiki/RS485_TO_ETH_(B)#MODBUS_TCP_Test
- https://www.waveshare.com/w/upload/a/a6/EN-RS485-TO-ETH-B-MQTT-and-json-user-manual2.pdf

