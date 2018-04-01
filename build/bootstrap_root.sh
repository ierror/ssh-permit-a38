#!/usr/bin/env bash
apt-get update --assume-yes
apt-get dist-upgrade --assume-yes
apt-get install build-essential pkg-config libssl-dev cmake --assume-yes
