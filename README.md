# Breadboard computer utilities

## Setup 

```bash
sudo snap install arduino-cli
sudo snap connect arduino-cli:raw-usb

sudo chmod a+rw /dev/ttyUSB0

arduino-cli core install arduino:avr

sudo apt-get install minicom
```

optional: 
```bash
arduino-cli completion bash > arduino-cli
sudo mv arduino-cli /etc/bash_completion.d/
```

possibly required: 
```bash
sudo adduser $(whoami) dialout
sudo usermod -aG dialout $(whoami)
```