# Temperature and Humidity Sensor

This project uses a [Si7021 Temperature + Humidity Sensor](https://learn.adafruit.com/adafruit-si7021-temperature-plus-humidity-sensor/overview) connected to a [nRF54L15-DK](https://docs.nordicsemi.com/r/bundle/ug_nrf54l15_dk/page/ug/nrf54l15_dk/intro/intro.html) to measure and output temperature and humidity data.
The communication between the Si7021 and the nRF54L15 is done through I2C, where the sensor will send raw data to the board. This data is then processed using the expressions provided in the sensor's datasheet and converted to Celcius and percentages for temperature and relative humidity data respectively.
The converted data is then transmitted through UART protocol from the nRF54L15 to the computer, which can be displayed through a serial monitor like minicom.

## Quick Start
### Hardware Connections
```
+-------------------+               +-------------------+
|   Si7021 Sensor   |               |   nRF54L15-DK     |
|                   |               |                   |
|               VIN | ------------> | P30.01 (VDD)      |
|               GND | ------------> | P30.02 (GND)      |
|               SCL | ------------> | P1.12  (I2C SCL)  |
|               SDA | ------------> | P1.11  (I2C SDA)  |
+-------------------+               +-------------------+
```

### Running
1. Enter nix environment:
```
nix develop
```
2. Run the program:
```
just run
```
3. Read the UART outputs:
```
just minicom
```
