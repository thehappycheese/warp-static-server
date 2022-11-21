# Warp-Static-Server

A single binary .exe server that creates a basic static file server.

## Installation

Check the release page to download precompiled

## Usage

By default if you double click the executable it will try to serve files found
in a folder called static in the same directory

```powershell
warp-static-server.exe
```

```text
                                _        _   _
__      ____ _ _ __ _ __    ___| |_ __ _| |_(_) ___ 
\ \ /\ / / _` | '__| '_ \  / __| __/ _` | __| |/ __|
 \ V  V / (_| | |  | |_) | \__ \ || (_| | |_| | (__ 
  \_/\_/ \__,_|_|  | .__/  |___/\__\__,_|\__|_|\___|
  ___  ___ _ ____  |_|__ _ __
/ __|/ _ \ '__\ \ / / _ \ '__|
\__ \  __/ |   \ V /  __/ |
|___/\___|_|    \_/ \___|_|

By Nick Archer :)

Attempting to start server.
Server started at http://0.0.0.0:8080 serving folder `./static/`
INFO - Server::run; addr=0.0.0.0:8080
INFO - listening on http://0.0.0.0:8080
```

### Arguments

- **--path** specifies a path to a directory containing the static files to serve
  - default `./static/`
  - eg `warp-static-server.exe --path "some path/to/serve/"`
- **--port** specifies the port to serve on
  - default `80`
  - eg `warp-static-server.exe --port 80`
- **--allowlocal** causes the server to work on `0.0.0.0` instead of `127.0.0.1`.
  - `0.0.0.0` will allow incoming traffic from other computers on your local network
    - eg `warp-static-server.exe --allowlocal`
  - The default, `127.0.0.1`, will allow only allow you to access the site from your own machine
    - eg `warp-static-server.exe`