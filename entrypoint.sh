#!/bin/sh

sudo apt-get update && sudo apt-get install -y firmware-iwlwifi
sudo service dbus start
sudo service bluetooth start

sleep 4

cargo watch --why -x run