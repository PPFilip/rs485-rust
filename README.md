# Intro
This program reads information from a Finder 7M.24 power meter connected to a RS485/modbus <-> TCP gateway and saves the 
output in PostgreSQL database. 

# Why?
I did not want to share my power meter data to the cloud. At the same time, I am maxing my raspi usage already, so using 
IoT frameworks such as Node-RED was not an option. I needed a program to query my power meter that can be compiled and 
not be a memory hog. I also took the opportunity to learn some Rust with it.

Here is the setup for illustrating the context this program runs in:
```
     220 V
      |                                                                   grafana <> timescale <> converter
      |                                                                         \      |          /   
      v                                                                            \   |       /
Finder 7M.24.8.230.0010 <--- RS485 ---> Waveshare RS485 to ETH (B) <--- TCP/IP ---> RaspberryPI
      |                                          ^                                       ^
      |--------> 12V AC/DC transformer ----------|----------> DC/DC converter -----------|
      v                                          
  Appliances
```

# Usage
1. Set up your power meter and modbus gateway (Power meter should be installed by a certified professional!)
2. Set up psql database (timescale support highly recommended)
3. Checkout the repo
4. `cargo build`
5. Edit config based on Settings.toml.example
6. Run via cron in interval you like
    
# Reference material
- https://www.findernet.com/en/canada/series/7m-series-smart-energy-meters/
- https://www.waveshare.com/wiki/RS485_TO_ETH_(B)#MODBUS_TCP_Test
- https://www.waveshare.com/w/upload/4/4d/RS485-to-eth-b-user-manual-EN-v1.33.pdf

# TODO
Listed in no particular order or priority
- Make the code prettier
- Add production targets for cargo
- Add support for multiple sites and meters (this is mostly db+grafana stuff)
- Publish related grafana dashboards
- Add support for direct serial connection (via RS485 hat on RaspberryPI or USB to serial adapter)
