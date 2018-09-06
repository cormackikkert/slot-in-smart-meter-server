# Smart meter server

Backend webserver for the smart meter project

## Architecture

- server (this project) is in rust
- exposes HTTP api
- stores data in influxDB

## Goals

- ingest the data from the IoT device
- query the data from the api
- do some auth
