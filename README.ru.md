Gamemonitor
===========

[![ru](https://img.shields.io/badge/lang-ru-black)](https://github.com/vsl-iil/gamemonitor/blob/master/README.ru.md)
[![en](https://img.shields.io/badge/lang-en-red)](https://github.com/vsl-iil/gamemonitor/blob/master/README.md)


Отслеживает и сообщает о скидках на игры в Steam.

## Использование

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


## Запуск (Docker)

Сборка контейнера:

```bash
docker build . -t gamemonitor
```


Запуск:

```bash
docker run -d --restart=unless-stopped --env-file=.env gamemonitor
```


При выборе способа уведомления `telegram` в директории проекта 
также должен находится файл `.env` со следующими переменными окружения:

```bash
TELEGRAM_API_TOKEN=<токен Telegram-бота для уведомлений>
TELEGRAM_CHAT_ID=<ID чата с ботом>
```


## Сборка

Сборка из исходников:

```bash
cargo build --release
```

Запуск:
```bash
cargo run
```

