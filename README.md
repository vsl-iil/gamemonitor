Gamemonitor
===========

[![ru](https://img.shields.io/badge/lang-ru-red)](https://github.com/vsl-iil/gamemonitor/blob/master/README.ru.md)
[![en](https://img.shields.io/badge/lang-en-black)](https://github.com/vsl-iil/gamemonitor/blob/master/README.md)


Monitors and notifies about Steam game discounts.

## Usage

```
Usage: gamemonitor [OPTIONS] --appid <APPID> --threshold <THRESHOLD> --delay <DELAY> [FEEDTYPE]

Arguments:
  [FEEDTYPE]  Way to notify about discounts [default: log] [possible values: log, telegram]

Options:
  -a, --appid <APPID>          App ID on Steam. Usually it's the last number contained in Store URL of the game
      --cc <CC>                Country code. Use this to get country-specific price [default: ]
  -t, --threshold <THRESHOLD>  Minimal discount that will trigger the monitor
  -d, --delay <DELAY>          How many seconds to wait between check. Should be no less than 2 secs. You may use `s', `m', `h', `d' suffixes to specify time in seconds, minutes, hours or days. Default unit: seconds
  -h, --help                   Print help
  -V, --version                Print version
```


## Running (Docker)

Build a container:

```bash
docker build . -t gamemonitor
```


Launch it with:

```bash
docker run -d --restart=unless-stopped --env-file=.env gamemonitor
```


`--env-file` argument is optional. If you choose `telegram` notify 
method, then the project directory should also contain an `.env` 
file with the following enviroment variables specified:

```bash
TELEGRAM_API_TOKEN=<Telegram bot token>
TELEGRAM_CHAT_ID=<ID of a chat with the bot>
```


## Building

Build from source:

```bash
cargo build --release
```


Launch it with:
```bash
cargo run
```

