#!/bin/bash
set -e

docker compose up -d
sudo chmod 777 sock/*.sock
curl -I --unix-socket sock/freshrss.sock http://localhost
docker compose down
