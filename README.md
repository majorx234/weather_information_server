# Info
- small server to scrap weather forcast data
- provide data in local Network via REST interface
- WIP

# Build & Run
- `cargo build`
- `source server_conf.sh`
- `cargo run`

# Config
- edit `server_conf.sh`
  - `HOST_IP` - IP address of server
  - `PORT` - Port of server

# Test REST Interface
- `curl http://127.0.0.1:8080/weather_information`
