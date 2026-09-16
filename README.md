# Temperature and Humidity Sensor

This project uses a Si7021 Temperature + Humidity Sensor connected to a nRF54L15-DK to measure and output temperature and humidity data.

## Quick Start
### Hardware Connections
(Si7021)    -->     (nRF54L15-DK)
VIN         -->     P30.01
GND         -->     P30.02
SCL         -->     P1.12
SDA         -->     P1.11

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
