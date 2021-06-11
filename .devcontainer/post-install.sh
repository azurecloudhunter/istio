#!/bin/sh

# install rust
sudo apt-get instally -y build-essential
curl https://sh.rustup.rs > rust.sh
chmod +x rust.sh
./rust.sh -y

# copy grafana.db to /grafana
sudo mkdir -p /grafana
sudo  cp deploy/grafanadata/grafana.db /grafana
sudo  chown -R 472:472 /grafana

# install webv
dotnet tool install -g webvalidate --version 2.0.0-beta2
