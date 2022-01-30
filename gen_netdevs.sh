#!/bin/bash
shopt -u expand_aliases
ip a | grep 'inet ' | awk '{print $2}' | sed 's/\/.*$//g' > net_devices
echo '0.0.0.0' >> net_devices