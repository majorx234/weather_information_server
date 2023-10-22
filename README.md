# Info
- small server to scrap weather forcast data
- render data as png file
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
  - `SCRAP_FREQ` - time between scraper calls to url
  - `WEATHER_URL` - irl to get data from
  - `SELECTOR` - DOM selector to get datastring from url
  - `DATA_INDEX` - Index in slector return list
# Test REST Interface
- `curl http://127.0.0.1:8080/weather_information.png`

# Reference
- Idea:
  - thx to Harry Stern and his inspiring
    - blogpost: https://harrystern.net/halldisplay.html
    - github repo: https://github.com/boustrophedon/eink-esp-weather-display
