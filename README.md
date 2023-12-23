# This is a small robot notification developed using rust, supporting telegram and lark

# This is a configuration information structure

```json
{
    "web_server_port": 9191,
    "telegram_bot_port": 8383,
    "telegram_bot_token": "your telegram bot token",
    "telegram_bot_uri": "your is telegram bot uri must be https type",
    "lark_cfg":[
        {
            "name":"The request parameter is the name of the key",
            "uri":"lark bot url ",
            "token":"lark bot token"
        }
    ]
}
```

## config is name:cfg.json

## build
```env
cargo build --release
```


## dev use https in ngrok

### http request post

#### lark bot to msg info

> Requests and POST
>> /notice/lark
>> Requests and Parameter Description

| Parameter      |     Type |     Must |   Parameter Description   |
| :-------- | :--------| :------ | :------ |
| message|  String|  yes|  msg |
| key|  String|   yes| is cfg in lark_cfg->name |

>> Response not Parameters

#### telegram bot to msg info

> Requests and POST
>> /notice/telegram
>> Requests and Parameter Description

| Parameter      |     Type |     Must |   Parameter Description   |
| :-------- | :--------| :------ | :------ |
| message|  String|  yes|  msg |
| key|  String|   yes| telegram chat id,can bot get chat id is command /chatid |

>> Response not Parameters


