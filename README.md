# Tonetheus

On-chain monitoring for TON validators through Prometheus.

## Installation
### Binary
```
sudo wget https://github.com/v-lgns/tonetheus/releases/latest/download/tonetheus -O /usr/bin/tonetheus
chmod a+x /usr/bin/tonetheus
```

### Systemd Service Unit
```
sudo wget https://raw.githubusercontent.com/v-lgns/tonetheus/master/assets/tonetheus.service -O /etc/systemd/system/tonetheus.service
```
NOTE: Tonetheus **must** be run as the same user who installed the `mytonctrl` binary.

## Usage
```
tonetheus -n $(hostname)
```
The name provided to tonetheus is solely for identifying the current validator machine. It will be visible in the exported metrics under the label `validator_name`.
