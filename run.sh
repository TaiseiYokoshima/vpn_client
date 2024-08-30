#!/bin/bash

cargo b --release
sudo setcap cap_net_admin=eip ./target/release/vpn_client 
./target/release/vpn_client &
pid=$!
sudo ip link set up dev tun01
sudo ip addr add 192.168.0.1/24 dev tun01
trap "kill $pid" INT TERM
wait $pid
