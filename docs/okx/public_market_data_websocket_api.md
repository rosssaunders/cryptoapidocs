# OKX API Documentation - Public Market Data WebSocket API

### Overview ###

WebSocket is a new HTML5 protocol that achieves full-duplex data transmission between the client and server, allowing data to be transferred effectively in both directions. A connection between the client and server can be established with just one handshake. The server will then be able to push data to the client according to preset rules. Its advantages include:

* The WebSocket request header size for data transmission between client and server is only 2 bytes.
* Either the client or server can initiate data transmission.
* There's no need to repeatedly create and delete TCP connections, saving resources on bandwidth and server.

We recommend developers use WebSocket API to retrieve market data and order book depth.

---

### Connect ###

**Connection limit**: 3 requests per second (based on IP)

When subscribing to a public channel, use the address of the public service. When subscribing to a private channel, use the address of the private service

**Request limit**:

The total number of 'subscribe'/'unsubscribe'/'login' requests per connection is limited to 480 times per hour.

If there’s a network problem, the system will automatically disable the connection.

The connection will break automatically if the subscription is not established or data has not been pushed for more than 30 seconds.

To keep the connection stable:

1. Set a timer of N seconds whenever a response message is received, where N is less than 30.

2. If the timer is triggered, which means that no new message is received within N seconds, send the String 'ping'.

3. Expect a 'pong' as a response. If the response message is not received within N seconds, please raise an error or reconnect.

---

### Connection count limit ###

The limit will be set at 30 WebSocket connections per specific WebSocket channel per sub-account. Each WebSocket connection is identified by the unique `connId`.

The WebSocket channels subject to this limitation are as follows:

1. [Orders channel](/docs-v5/en/#order-book-trading-trade-ws-order-channel)
2. [Account channel](/docs-v5/en/#trading-account-websocket-account-channel)
3. [Positions channel](/docs-v5/en/#trading-account-websocket-positions-channel)
4. [Balance and positions channel](/docs-v5/en/#trading-account-websocket-balance-and-position-channel)
5. [Position risk warning channel](/docs-v5/en/#trading-account-websocket-position-risk-warning)
6. [Account greeks channel](/docs-v5/en/#trading-account-websocket-account-greeks-channel)

If users subscribe to the same channel through the same WebSocket connection through multiple arguments, for example, by using `{"channel": "orders", "instType": "ANY"}` and `{"channel": "orders", "instType": "SWAP"}`, it will be counted once only. If users subscribe to the listed channels (such as orders and accounts) using either the same or different connections, it will not affect the counting, as these are considered as two different channels. The system calculates the number of WebSocket connections per channel.

The platform will send the number of active connections to clients through the `channel-conn-count` event message **to new channel subscriptions**.

>
>
> Connection count update
>
>

```
{
    "event":"channel-conn-count",
    "channel":"orders",
    "connCount": "2",
    "connId":"abcd1234"
}

```

When the limit is breached, generally the latest connection that sends the subscription request will be rejected. Client will receive the usual subscription acknowledgement followed by the `channel-conn-count-error` from the connection that the subscription has been terminated. In exceptional circumstances the platform may unsubscribe existing connections.

>
>
> Connection limit error
>
>

```
{
    "event": "channel-conn-count-error",
    "channel": "orders",
    "connCount": "20",
    "connId":"a4d3ae55"
}

```

Order operations through WebSocket, including place, amend and cancel orders, are not impacted through this change.

---

### Login ###

>
>
> Request Example
>
>

```
{
  "op": "login",
  "args": [
    {
      "apiKey": "985d5b66-57ce-40fb-b714-afc0b9787083",
      "passphrase": "123456",
      "timestamp": "1538054050",
      "sign": "7L+zFQ+CEgGu5rzCj4+BdV2/uUHGqddA9pI6ztsRRPs="
    }
  ]
}

```

#### Request Parameters ####

|  Parameter  |      Type      |Required|            Description             |
|-------------|----------------|--------|------------------------------------|
|     op      |     String     |  Yes   |      Operation  <br/>`login`       |
|    args     |Array of objects|  Yes   |      List of account to login      |
|  \> apiKey  |     String     |  Yes   |              API Key               |
|\> passphrase|     String     |  Yes   |          API Key password          |
|\> timestamp |     String     |  Yes   |Unix Epoch time, the unit is seconds|
|   \> sign   |     String     |  Yes   |          Signature string          |

>
>
> Successful Response Example
>
>

```
{
  "event": "login",
  "code": "0",
  "msg": "",
  "connId": "a4d3ae55"
}

```

>
>
> Failure Response Example
>
>

```
{
  "event": "error",
  "code": "60009",
  "msg": "Login failed.",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter| Type |Required|             Description             |
|---------|------|--------|-------------------------------------|
|  event  |String|  Yes   |Operation  <br/>`login`  <br/>`error`|
|  code   |String|   No   |             Error code              |
|   msg   |String|   No   |            Error message            |
| connId  |String|  Yes   |       WebSocket connection ID       |

**apiKey**: Unique identification for invoking API. Requires user to apply one manually.

**passphrase**: API Key password

**timestamp**: the Unix Epoch time, the unit is seconds, e.g. 1704876947

**sign**: signature string, the signature algorithm is as follows:

First concatenate `timestamp`, `method`, `requestPath`, strings, then use HMAC SHA256 method to encrypt the concatenated string with SecretKey, and then perform Base64 encoding.

**secretKey**: The security key generated when the user applies for APIKey, e.g. : 22582BD0CFF14C41EDBF1AB98506286D

**Example of timestamp**: const timestamp = '' + Date.now() / 1,000

**Among sign example**: sign=CryptoJS.enc.Base64.stringify(CryptoJS.HmacSHA256(timestamp +'GET'+'/users/self/verify', secretKey))

**method**: always 'GET'.

**requestPath** : always '/users/self/verify'

The request will expire 30 seconds after the timestamp. If your server time differs from the API server time, we recommended using the REST API to query the API server time and then set the timestamp.

---

### Subscribe ###

**Subscription Instructions**

>
>
> Request format description
>
>

```
{
  "op": "subscribe",
  "args": ["<SubscriptionTopic>"]
}

```

WebSocket channels are divided into two categories: `public` and `private` channels.

`Public channels` -- No authentication is required, include tickers channel, K-Line channel, limit price channel, order book channel, and mark price channel etc.

`Private channels` -- including account channel, order channel, and position channel, etc -- require log in.

Users can choose to subscribe to one or more channels, and the total length of multiple channels cannot exceed 64 KB.

Below is an example of subscription parameters. The requirement of subscription parameters for each channel is different. For details please refer to the specification of each channels.

>
>
> Request Example
>
>

```
{
    "op":"subscribe",
    "args":[
        {
            "channel":"tickers",
            "instId":"BTC-USDT"
        }
    ]
}

```

**Request parameters**

|  Parameter  |      Type      |Required|                                            Description                                            |
|-------------|----------------|--------|---------------------------------------------------------------------------------------------------|
|     op      |     String     |  Yes   |                                    Operation  <br/>`subscribe`                                    |
|    args     |Array of objects|  Yes   |                                    List of subscribed channels                                    |
| \> channel  |     String     |  Yes   |                                           Channel name                                            |
| \> instType |     String     |   No   |Instrument type  <br/>`SPOT`  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`  <br/>`ANY`|
|\> instFamily|     String     |   No   |                  Instrument family  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`                  |
|  \> instId  |     String     |   No   |                                           Instrument ID                                           |

>
>
> Response Example
>
>

```
{
    "event": "subscribe",
    "arg": {
        "channel": "tickers",
        "instId": "BTC-USDT"
    },
    "connId": "accb8e21"
}

```

**Return parameters**

|  Parameter  | Type |Required|                                            Description                                            |
|-------------|------|--------|---------------------------------------------------------------------------------------------------|
|    event    |String|  Yes   |                               Event  <br/>`subscribe`  <br/>`error`                               |
|     arg     |Object|   No   |                                        Subscribed channel                                         |
| \> channel  |String|  Yes   |                                           Channel name                                            |
| \> instType |String|   No   |Instrument type  <br/>`SPOT`  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`  <br/>`ANY`|
|\> instFamily|String|   No   |                  Instrument family  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`                  |
|  \> instId  |String|   No   |                                           Instrument ID                                           |
|    code     |String|   No   |                                            Error code                                             |
|     msg     |String|   No   |                                           Error message                                           |
|   connId    |String|  Yes   |                                      WebSocket connection ID                                      |

---

### Unsubscribe ###

Unsubscribe from one or more channels.

>
>
> Request format description
>
>

```
{
  "op": "unsubscribe",
  "args": ["< SubscriptionTopic> "]
}

```

>
>
> Request Example
>
>

```
{
  "op": "unsubscribe",
  "args": [
    {
      "channel": "tickers",
      "instId": "BTC-USDT"
    }
  ]
}

```

**Request parameters**

|  Parameter  |      Type      |Required|                                            Description                                            |
|-------------|----------------|--------|---------------------------------------------------------------------------------------------------|
|     op      |     String     |  Yes   |                                   Operation  <br/>`unsubscribe`                                   |
|    args     |Array of objects|  Yes   |                               List of channels to unsubscribe from                                |
| \> channel  |     String     |  Yes   |                                           Channel name                                            |
| \> instType |     String     |   No   |Instrument type  <br/>`SPOT`  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`  <br/>`ANY`|
|\> instFamily|     String     |   No   |                  Instrument family  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`                  |
|  \> instId  |     String     |   No   |                                           Instrument ID                                           |

>
>
> Response Example
>
>

```
{
    "event": "unsubscribe",
    "arg": {
        "channel": "tickers",
        "instId": "BTC-USDT"
    },
    "connId": "d0b44253"
}

```

**Response parameters**

|  Parameter  | Type |Required|                                      Description                                      |
|-------------|------|--------|---------------------------------------------------------------------------------------|
|    event    |String|  Yes   |                        Event  <br/>`unsubscribe`  <br/>`error`                        |
|     arg     |Object|   No   |                                 Unsubscribed channel                                  |
| \> channel  |String|  Yes   |                                     Channel name                                      |
| \> instType |String|   No   |Instrument type  <br/>`SPOT`  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`|
|\> instFamily|String|   No   |            Instrument family  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`            |
|  \> instId  |String|   No   |                                     Instrument ID                                     |
|    code     |String|   No   |                                      Error code                                       |
|     msg     |String|   No   |                                     Error message                                     |

---

### Notification ###

WebSocket has introduced a new message type (event = `notice`).   

Client will receive the information in the following scenarios:

* Websocket disconnect for service upgrade  

30 seconds prior to the upgrade of the WebSocket service, the notification message will be sent to users indicating that the connection will soon be disconnected. Users are encouraged to establish a new connection to prevent any disruptions caused by disconnection.

>
>
> Response Example
>
>

```
{
    "event": "notice",
    "code": "64008",
    "msg": "The connection will soon be closed for a service upgrade. Please reconnect.",
    "connId": "a4d3ae55"
}

```

The feature is supported by WebSocket Public (/ws/v5/public) and Private (/ws/v5/private) for now.

---

### Instruments channel ###

The instruments will be pushed if there is any change to the instrument’s state (such as delivery of FUTURES, exercise of OPTION, listing of new contracts / trading pairs, trading suspension, etc.).  
(The full instrument list is not pushed since December 28, 2022, [you can click here to view details](/docs-v5/log_en/#2022-12-06))

#### URL Path ####

/ws/v5/public

>
>
> Request Example
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "instruments",
      "instType": "SPOT"
    }
  ]
}

```

#### Request Parameters ####

| Parameter |      Type      |Required|                                      Description                                      |
|-----------|----------------|--------|---------------------------------------------------------------------------------------|
|    op     |     String     |  Yes   |                    Operation  <br/>`subscribe`  <br/>`unsubscribe`                    |
|   args    |Array of objects|  Yes   |                              List of subscribed channels                              |
|\> channel |     String     |  Yes   |                           Channel name  <br/>`instruments`                            |
|\> instType|     String     |  Yes   |Instrument type  <br/>`SPOT`  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`|

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "instruments",
    "instType": "SPOT"
  },
  "connId": "a4d3ae55"
}

```

>
>
> Failure Response Example
>
>

```
{
  "event": "error",
  "code": "60012",
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"instruments\", \"instType\" : \"FUTURES\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

| Parameter | Type |Required|                                      Description                                      |
|-----------|------|--------|---------------------------------------------------------------------------------------|
|   event   |String|  Yes   |               Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`               |
|    arg    |Object|   No   |                                  Subscribed channel                                   |
|\> channel |String|  Yes   |                                     Channel name                                      |
|\> instType|String|  Yes   |Instrument type  <br/>`SPOT`  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`|
|   code    |String|   No   |                                      Error code                                       |
|    msg    |String|   No   |                                     Error message                                     |
|  connId   |String|  Yes   |                                WebSocket connection ID                                |

>
>
> Push Data Example
>
>

```
{
  "arg": {
    "channel": "instruments",
    "instType": "SPOT"
  },
  "data": [
    {
        "alias": "",
        "auctionEndTime": "",
        "baseCcy": "BTC",
        "category": "1",
        "ctMult": "",
        "ctType": "",
        "ctVal": "",
        "ctValCcy": "",
        "expTime": "",
        "futureSettlement": false,
        "instFamily": "",
        "instId": "BTC-USDT",
        "instType": "SPOT",
        "lever": "10",
        "listTime": "1606468572000",
        "lotSz": "0.00000001",
        "maxIcebergSz": "9999999999.0000000000000000",
        "maxLmtAmt": "1000000",
        "maxLmtSz": "9999999999",
        "maxMktAmt": "1000000",
        "maxMktSz": "",
        "maxStopSz": "",
        "maxTriggerSz": "9999999999.0000000000000000",
        "maxTwapSz": "9999999999.0000000000000000",
        "minSz": "0.00001",
        "optType": "",
        "quoteCcy": "USDT",
        "settleCcy": "",
        "state": "live",
        "ruleType": "normal",
        "stk": "",
        "tickSz": "0.1",
        "uly": ""
    }
  ]
}

```

#### Push data parameters ####

|     Parameter     |      Type      |                                                                                                                                        Description                                                                                                                                         |
|-------------------|----------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|        arg        |     Object     |                                                                                                                                     Subscribed channel                                                                                                                                     |
|    \> channel     |     String     |                                                                                                                                        Channel name                                                                                                                                        |
|    \> instType    |     String     |                                                                                                                                      Instrument type                                                                                                                                       |
|       data        |Array of objects|                                                                                                                                      Subscribed data                                                                                                                                       |
|    \> instType    |     String     |                                                                                                                                      Instrument type                                                                                                                                       |
|     \> instId     |     String     |                                                                                                                               Instrument ID, e.g. `BTC-UST`                                                                                                                                |
|      \> uly       |     String     |                                                                                                       Underlying, e.g. `BTC-USD`   <br/>Only applicable to `FUTURES`/`SWAP`/`OPTION`                                                                                                       |
|   \> instFamily   |     String     |                                                                                                   Instrument family, e.g. `BTC-USD`   <br/>Only applicable to `FUTURES`/`SWAP`/`OPTION`                                                                                                    |
|    \> category    |     String     |                                                                                                               Currency category. Note: this parameter is already deprecated                                                                                                                |
|    \> baseCcy     |     String     |                                                                                                     Base currency, e.g. `BTC` in `BTC-USDT`   <br/>Only applicable to `SPOT`/`MARGIN`                                                                                                      |
|    \> quoteCcy    |     String     |                                                                                                    Quote currency, e.g. `USDT` in `BTC-USDT`   <br/>Only applicable to `SPOT`/`MARGIN`                                                                                                     |
|   \> settleCcy    |     String     |                                                                                               Settlement and margin currency, e.g. `BTC`   <br/>Only applicable to `FUTURES`/`SWAP`/`OPTION`                                                                                               |
|     \> ctVal      |     String     |                                                                                                                                       Contract value                                                                                                                                       |
|     \> ctMult     |     String     |                                                                                                                                    Contract multiplier                                                                                                                                     |
|    \> ctValCcy    |     String     |                                                                                                                                  Contract value currency                                                                                                                                   |
|    \> optType     |     String     |                                                                                                        Option type  <br/>`C`: Call  <br/>`P`: Put  <br/>Only applicable to `OPTION`                                                                                                        |
|      \> stk       |     String     |                                                                                                                       Strike price  <br/>Only applicable to `OPTION`                                                                                                                       |
|    \> listTime    |     String     |                                                                                                              Listing time  <br/>Only applicable to `FUTURES`/`SWAP`/`OPTION`                                                                                                               |
| \> auctionEndTime |     String     |                                                 The end time of call auction, Unix timestamp format in milliseconds, e.g. `1597026383085`   <br/>Only applicable to `SPOT` that are listed through call auctions, return "" in other cases                                                 |
|    \> expTime     |     String     |                                   Expiry time  <br/>Applicable to `SPOT`/`MARGIN`/`FUTURES`/`SWAP`/`OPTION`. For `FUTURES`/`OPTION`, it is the delivery/exercise time. It can also be the delisting time of the trading instrument. Update once change.                                    |
|     \> lever      |     String     |                                                                                           Max Leverage  <br/>Not applicable to `SPOT`/`OPTION`, used to distinguish between `MARGIN` and `SPOT`.                                                                                           |
|     \> tickSz     |     String     |                                                                                                      Tick size, e.g. `0.0001`  <br/>For Option, it is minimum tickSz among tick band.                                                                                                      |
|     \> lotSz      |     String     |                                                             Lot size  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `base currency`                                                             |
|     \> minSz      |     String     |                                                        Minimum order size  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `base currency`                                                        |
|     \> ctType     |     String     |                                                                                 Contract type  <br/>`linear`: linear contract  <br/>`inverse`: inverse contract  <br/>Only applicable to `FUTURES`/`SWAP`                                                                                  |
|     \> alias      |     String     |Alias  <br/>`this_week`  <br/>`next_week`  <br/>`this_month`  <br/>`next_month`  <br/>`quarter`  <br/>`next_quarter`  <br/>Only applicable to `FUTURES`   <br/>**Not recommended for use, users are encouraged to rely on the expTime field to determine the delivery time of the contract**|
|     \> state      |     String     |                                     Instrument status  <br/>`live`  <br/>`suspend`  <br/>`expired`  <br/>`preopen`. e.g. There will be preopen before the Futures and Options new contracts state is live.   <br/>`test`: Test pairs, can't be traded                                      |
|     \> state      |     String     |                         Instrument status  <br/>`live`  <br/>`suspend`  <br/>`expired`  <br/>`preopen` e.g. Futures and options contracts rollover from generation to trading start; certain symbols before they go live  <br/>`test`: Test pairs, can't be traded                         |
|    \> ruleType    |     String     |                                                                                                  Trading rule types  <br/>`normal`: normal trading  <br/>`pre_market`: pre-market trading                                                                                                  |
|    \> maxLmtSz    |     String     |                                       The maximum order quantity of a single limit order.  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `base currency`.                                       |
|    \> maxMktSz    |     String     |                                           The maximum order quantity of a single market order.  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `USDT`.                                           |
|   \> maxTwapSz    |     String     |                                       The maximum order quantity of a single TWAP order.  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `base currency`.                                        |
|  \> maxIcebergSz  |     String     |                                      The maximum order quantity of a single iceBerg order.  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `base currency`.                                      |
|  \> maxTriggerSz  |     String     |                                      The maximum order quantity of a single trigger order.  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `base currency`.                                      |
|   \> maxStopSz    |     String     |                                        The maximum order quantity of a single stop market order.  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `USDT`.                                         |
|\> futureSettlement|    Boolean     |                                                                                                Whether daily settlement for expiry feature is enabled  <br/>Applicable to `FUTURES` `cross`                                                                                                |
 Instrument status will trigger pushing of incremental data from instruments channel.
When a new contract is going to be listed, the instrument data of the new contract will be available with status preopen.
When a product is going to be delisted (e.g. when a FUTURES contract is settled or OPTION contract is exercised), the instrument status will be changed to expired. listTime and auctionEndTime  
For spot symbols listed through a call auction, listTime represents the start time of the auction, and auctionEndTime indicates the end of the auction and the start of continuous trading. For other scenarios, listTime will mark the beginning of continuous trading, and auctionEndTime will return an empty value "". state  
The state will always change from `preopen` to `live` when the listTime is reached. Certain symbols will now have `state:preopen` before they go live. Before going live, the instruments channel will push data for pre-listing symbols with `state:preopen`. If the listing is cancelled, the channel will send full data excluding the cancelled symbol, without additional notification. When the symbol goes live (reaching listTime), the channel will push data with `state:live`. Users can also query the corresponding data via the REST endpoint.  
When a product is going to be delisted (e.g. when a FUTURES contract is settled or OPTION contract is exercised), the instrument will not be available.

---

### Open interest channel ###

Retrieve the open interest. Data will be pushed every 3 seconds when there are updates.

#### URL Path ####

/ws/v5/public

>
>
> Request Example
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "open-interest",
      "instId": "LTC-USD-SWAP"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |      Type      |Required|                  Description                  |
|----------|----------------|--------|-----------------------------------------------|
|    op    |     String     |  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`|
|   args   |Array of objects|  Yes   |          List of subscribed channels          |
|\> channel|     String     |  Yes   |      Channel name  <br/>`open-interest`       |
|\> instId |     String     |  Yes   |                 Instrument ID                 |

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
      "channel": "open-interest",
      "instId": "LTC-USD-SWAP"
  },
  "connId": "a4d3ae55"
}

```

>
>
> Failure Response Example
>
>

```
{
  "event": "error",
  "code": "60012",
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"open-interest\", \"instId\" : \"LTC-USD-SWAP\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|                       Description                       |
|----------|------|--------|---------------------------------------------------------|
|  event   |String|  Yes   |Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|   arg    |Object|   No   |                   Subscribed channel                    |
|\> channel|String|  Yes   |                      Channel name                       |
|\> instId |String|  Yes   |                      Instrument ID                      |
|   code   |String|   No   |                       Error code                        |
|   msg    |String|   No   |                      Error message                      |
|  connId  |String|  Yes   |                 WebSocket connection ID                 |

>
>
> Push Data Example
>
>

```
{
  "arg": {
    "channel": "open-interest",
    "instId": "LTC-USD-SWAP"
  },
  "data": [
    {
      "instType": "SWAP",
      "instId": "LTC-USD-SWAP",
      "oi": "5000",
      "oiCcy": "555.55",
      "ts": "1597026383085"
    }
  ]
}

```

#### Push data parameters ####

|**Parameter**|    **Type**    |                                        **Description**                                        |
|-------------|----------------|-----------------------------------------------------------------------------------------------|
|     arg     |     Object     |                                Successfully subscribed channel                                |
| \> channel  |     String     |                                         Channel name                                          |
|  \> instId  |     String     |                                         Instrument ID                                         |
|    data     |Array of objects|                                        Subscribed data                                        |
| \> instType |     String     |                                        Instrument type                                        |
|  \> instId  |     String     |                              Instrument ID, e.g. `BTC-USDT-SWAP`                              |
|    \> oi    |     String     |                             Open interest, in units of contracts.                             |
|  \> oiCcy   |     String     |                          Open interest, in currency units, like BTC.                          |
|    \> ts    |     String     |The time when the data was updated, Unix timestamp format in milliseconds, e.g. `1597026383085`|

---

### Funding rate channel ###

Retrieve funding rate. Data will be pushed in 30s to 90s.

#### URL Path ####

/ws/v5/public

>
>
> Request Example
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "funding-rate",
      "instId": "BTC-USD-SWAP"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |      Type      |Required|                  Description                  |
|----------|----------------|--------|-----------------------------------------------|
|    op    |     String     |  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`|
|   args   |Array of objects|  Yes   |          List of subscribed channels          |
|\> channel|     String     |  Yes   |       Channel name  <br/>`funding-rate`       |
|\> instId |     String     |  Yes   |                 Instrument ID                 |

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "funding-rate",
    "instId": "BTC-USD-SWAP"
  },
  "connId": "a4d3ae55"
}

```

>
>
> Failure Response Example
>
>

```
{
  "event": "error",
  "code": "60012",
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"funding-rate\", \"instId\" : \"BTC-USD-SWAP\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|                       Description                       |
|----------|------|--------|---------------------------------------------------------|
|  event   |String|  Yes   |Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|   arg    |Object|   No   |                   Subscribed channel                    |
|\> channel|String|  yes   |                      Channel name                       |
|\> instId |String|   No   |                      Instrument ID                      |
|   code   |String|   No   |                       Error code                        |
|   msg    |String|   No   |                      Error message                      |
|  connId  |String|  Yes   |                 WebSocket connection ID                 |

>
>
> Push Data Example
>
>

```
{
   "arg":{
      "channel":"funding-rate",
      "instId":"BTC-USD-SWAP"
   },
   "data":[
      {
         "fundingRate":"0.0001875391284828",
         "fundingTime":"1700726400000",
         "instId":"BTC-USD-SWAP",
         "instType":"SWAP",
         "method": "current_period",
         "maxFundingRate":"0.00375",
         "minFundingRate":"-0.00375",
         "nextFundingRate":"",
         "nextFundingTime":"1700755200000",
         "premium": "0.0001233824646391",
         "settFundingRate":"0.0001699799259033",
         "settState":"settled",
         "ts":"1700724675402"
      }
   ]
}

```

#### Push data parameters ####

|  **Parameter**   |    **Type**    |                                                                                               **Description**                                                                                               |
|------------------|----------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|       arg        |     Object     |                                                                                       Successfully subscribed channel                                                                                       |
|    \> channel    |     String     |                                                                                                Channel name                                                                                                 |
|    \> instId     |     String     |                                                                                                Instrument ID                                                                                                |
|       data       |Array of objects|                                                                                               Subscribed data                                                                                               |
|   \> instType    |     String     |                                                                                           Instrument type, `SWAP`                                                                                           |
|    \> instId     |     String     |                                                                                     Instrument ID, e.g. `BTC-USD-SWAP`                                                                                      |
|    \> method     |     String     |                                                        Funding rate mechanism   <br/>`current_period`   <br/>~~`next_period`~~(no longer supported)                                                         |
|  \> fundingRate  |     String     |                                                                                            Current funding rate                                                                                             |
|  \> fundingTime  |     String     |                                       ~~Funding time of the upcoming settlement, Unix timestamp format in milliseconds, e.g. `1597026383085`.~~(no longer supported)                                        |
|\> nextFundingRate|     String     |                                                                                 Forecasted funding rate for the next period                                                                                 |
|\> nextFundingTime|     String     |                                                  Forecasted funding time for the next period, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                   |
|\> minFundingRate |     String     |                                                                       The lower limit of the predicted funding rate of the next cycle                                                                       |
|\> maxFundingRate |     String     |                                                                       The upper limit of the predicted funding rate of the next cycle                                                                       |
|   \> settState   |     String     |                                                                    Settlement state of funding rate   <br/>`processing`   <br/>`settled`                                                                    |
|\> settFundingRate|     String     |If settState = `processing`, it is the funding rate that is being used for current settlement cycle.   <br/>If settState = `settled`, it is the funding rate that is being used for previous settlement cycle|
|    \> premium    |     String     |                                                                      Premium between the mid price of perps market and the index price                                                                      |
|      \> ts       |     String     |                                                                Data return time, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                |
For some altcoins perpetual swaps with significant fluctuations in funding rates, OKX will closely monitor market changes. When necessary, the funding rate collection frequency, currently set at 8 hours, may be adjusted to higher frequencies such as 6 hours, 4 hours, 2 hours, or 1 hour. Thus, users should focus on the difference between `fundingTime` and `nextFundingTime` fields to determine the funding fee interval of a contract.

---

### Price limit channel ###

Retrieve the maximum buy price and minimum sell price of instruments. Data will be pushed every 200ms when there are changes in limits, and will not be pushed when there is no changes on limit.

#### URL Path ####

/ws/v5/public

>
>
> Request Example
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "price-limit",
      "instId": "LTC-USD-190628"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |      Type      |Required|                  Description                  |
|----------|----------------|--------|-----------------------------------------------|
|    op    |     String     |  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`|
|   args   |Array of objects|  Yes   |          List of subscribed channels          |
|\> channel|     String     |  Yes   |       Channel name  <br/>`price-limit`        |
|\> instId |     String     |  Yes   |                 Instrument ID                 |

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "price-limit",
    "instId": "LTC-USD-190628"
  },
  "connId": "a4d3ae55"
}

```

>
>
> Failure Response Example
>
>

```
{
  "event": "error",
  "code": "60012",
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"price-limit\", \"instId\" : \"LTC-USD-190628\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|                       Description                       |
|----------|------|--------|---------------------------------------------------------|
|  event   |String|  Yes   |Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|   arg    |Object|   No   |                   Subscribed channel                    |
|\> channel|String|  Yes   |                      Channel name                       |
|\> instId |String|  Yes   |                      Instrument ID                      |
|   code   |String|   No   |                       Error code                        |
|   msg    |String|   No   |                      Error message                      |
|  connId  |String|  Yes   |                 WebSocket connection ID                 |

>
>
> Push Data Example
>
>

```
{
    "arg": {
        "channel": "price-limit",
        "instId": "LTC-USD-190628"
    },
    "data": [{
        "instId": "LTC-USD-190628",
        "buyLmt": "200",
        "sellLmt": "300",
        "ts": "1597026383085",
        "enabled": true
    }]
}

```

#### Push data parameters ####

|**Parameter**|    **Type**    |                                                       **Description**                                                       |
|-------------|----------------|-----------------------------------------------------------------------------------------------------------------------------|
|     arg     |     Object     |                                               Successfully subscribed channel                                               |
| \> channel  |     String     |                                                        Channel name                                                         |
|  \> instId  |     String     |                                                        Instrument ID                                                        |
|    data     |Array of objects|                                                       Subscribed data                                                       |
| \> instType |     String     |                                                       Instrument type                                                       |
|  \> instId  |     String     |                                               Instrument ID, e.g. `BTC-USDT`                                                |
|  \> buyLmt  |     String     |                                  Maximum buy price   <br/>Return "" when enabled is false                                   |
| \> sellLmt  |     String     |                                  Minimum sell price   <br/>Return "" when enabled is false                                  |
|    \> ts    |     String     |                       Price update time, Unix timestamp format in milliseconds, e.g. `1597026383085`                        |
| \> enabled  |    Boolean     |Whether price limit is effective   <br/>`true`: the price limit is effective   <br/>`false`: the price limit is not effective|

---

### Option summary channel ###

Retrieve detailed pricing information of all OPTION contracts. Data will be pushed at once.

#### URL Path ####

/ws/v5/public

>
>
> Request Example
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "opt-summary",
      "instFamily": "BTC-USD"
    }
  ]
}

```

#### Request Parameters ####

|  Parameter  |      Type      |Required|                  Description                  |
|-------------|----------------|--------|-----------------------------------------------|
|     op      |     String     |  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`|
|    args     |Array of objects|  Yes   |          List of subscribed channels          |
| \> channel  |     String     |  Yes   |       Channel name  <br/>`opt-summary`        |
|\> instFamily|     String     |  Yes   |               Instrument family               |

>
>
> Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "opt-summary",
    "instFamily": "BTC-USD"
  },
  "connId": "a4d3ae55"
}

```

>
>
> Failure example
>
>

```
{
  "event": "error",
  "code": "60012",
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"opt-summary\", \"uly\" : \"BTC-USD\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|  Parameter  | Type |Required|                       Description                       |
|-------------|------|--------|---------------------------------------------------------|
|    event    |String|  Yes   |Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|     arg     |Object|   No   |                   Subscribed channel                    |
| \> channel  |String|  Yes   |                      Channel name                       |
|\> instFamily|String|  Yes   |                    Instrument family                    |
|    code     |String|   No   |                       Error code                        |
|     msg     |String|   No   |                      Error message                      |
|   connId    |String|  Yes   |                 WebSocket connection ID                 |

>
>
> Push Data Example
>
>

```
{
    "arg": {
        "channel": "opt-summary",
        "instFamily": "BTC-USD"
    },
    "data": [
        {
            "instType": "OPTION",
            "instId": "BTC-USD-241013-70000-P",
            "uly": "BTC-USD",
            "delta": "-1.1180902625",
            "gamma": "2.2361957091",
            "vega": "0.0000000001",
            "theta": "0.0000032334",
            "lever": "8.465747567",
            "markVol": "0.3675503331",
            "bidVol": "0",
            "askVol": "1.1669998535",
            "realVol": "",
            "deltaBS": "-0.9999672034",
            "gammaBS": "0.0000000002",
            "thetaBS": "28.2649858387",
            "vegaBS": "0.0000114332",
            "ts": "1728703155650",
            "fwdPx": "62604.6993093463",
            "volLv": "0.2044711229"
        }
    ]
}

```

#### Push data parameters ####

|**Parameter**|    **Type**    |                               **Description**                                |
|-------------|----------------|------------------------------------------------------------------------------|
|     arg     |     Object     |                       Successfully subscribed channel                        |
| \> channel  |     String     |                                 Channel name                                 |
|\> instFamily|     String     |                              Instrument family                               |
|    data     |Array of objects|                               Subscribed data                                |
| \> instType |     String     |                          Instrument type, `OPTION`                           |
|  \> instId  |     String     |                                Instrument ID                                 |
|   \> uly    |     String     |                                  Underlying                                  |
|  \> delta   |     String     |                  Sensitivity of option price to `uly` price                  |
|  \> gamma   |     String     |                   The delta is sensitivity to `uly` price                    |
|   \> vega   |     String     |              Sensitivity of option price to implied volatility               |
|  \> theta   |     String     |               Sensitivity of option priceo remaining maturity                |
| \> deltaBS  |     String     |            Sensitivity of option price to `uly` price in BS mode             |
| \> gammaBS  |     String     |              The delta is sensitivity to `uly` price in BS mode              |
|  \> vegaBS  |     String     |         Sensitivity of option price to implied volatility in BS mode         |
| \> thetaBS  |     String     |         Sensitivity of option price to remaining maturity in BS mode         |
|  \> lever   |     String     |                                   Leverage                                   |
| \> markVol  |     String     |                               Mark volatility                                |
|  \> bidVol  |     String     |                                Bid volatility                                |
|  \> askVol  |     String     |                                Ask Volatility                                |
| \> realVol  |     String     |                   Realized volatility (not currently used)                   |
|  \> volLv   |     String     |                  Implied volatility of at-the-money options                  |
|  \> fwdPx   |     String     |                                Forward price                                 |
|    \> ts    |     String     |Price update time, Unix timestamp format in milliseconds, e.g. `1597026383085`|

---

### Estimated delivery/exercise/settlement price channel ###

Retrieve the estimated delivery/exercise/settlement price of `FUTURES` and `OPTION` contracts.

Only the estimated price will be pushed in an hour before delivery/exercise/settlement, and will be pushed if there is any price change.

#### URL Path ####

/ws/v5/public

>
>
> Request Example
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "estimated-price",
      "instType": "FUTURES",
      "instFamily": "BTC-USD"
    }
  ]
}

```

#### Request Parameters ####

|  Parameter  |      Type      | Required  |                            Description                             |
|-------------|----------------|-----------|--------------------------------------------------------------------|
|     op      |     String     |    Yes    |          Operation  <br/>`subscribe`  <br/>`unsubscribe`           |
|    args     |Array of objects|    Yes    |                    List of subscribed channels                     |
| \> channel  |     String     |    Yes    |                Channel name  <br/>`estimated-price`                |
| \> instType |     String     |    Yes    |           Instrument type  <br/>`OPTION`  <br/>`FUTURES`           |
|\> instFamily|     String     |Conditional|Instrument family  <br/>Either `instFamily` or `instId` is required.|
|  \> instId  |     String     |Conditional|  Instrument ID  <br/>Either `instFamily` or `instId` is required.  |

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "estimated-price",
    "instType": "FUTURES",
    "instFamily": "BTC-USD"
  },
  "connId": "a4d3ae55"
}

```

>
>
> Failure Response Example
>
>

```
{
  "event": "error",
  "code": "60012",
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"estimated-price\", \"instId\" : \"FUTURES\",\"uly\" :\"BTC-USD\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|  Parameter  | Type | Required  |                       Description                       |
|-------------|------|-----------|---------------------------------------------------------|
|    event    |String|    Yes    |Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|     arg     |Object|    No     |                   Subscribed channel                    |
| \> channel  |String|    Yes    |                      Channel name                       |
| \> instType |String|    Yes    |     Instrument type  <br/>`OPTION`  <br/>`FUTURES`      |
|\> instFamily|String|Conditional|                    Instrument family                    |
|  \> instId  |String|Conditional|                      Instrument ID                      |
|    code     |String|    No     |                       Error code                        |
|     msg     |String|    No     |                      Error message                      |
|   connId    |String|    Yes    |                 WebSocket connection ID                 |

>
>
> Push Data Example
>
>

```
{
    "arg": {
        "channel": "estimated-price",
        "instType": "FUTURES",
        "instFamily": "XRP-USDT"
    },
    "data": [{
        "instId": "XRP-USDT-250307",
        "instType": "FUTURES",
        "settlePx": "2.4230631578947368",
        "settleType": "settlement",
        "ts": "1741244598708"
    }]
}

```

#### Push data parameters ####

|**Parameter**|    **Type**    |                                                **Description**                                                 |
|-------------|----------------|----------------------------------------------------------------------------------------------------------------|
|     arg     |     Object     |                                        Successfully subscribed channel                                         |
| \> channel  |     String     |                                                  Channel name                                                  |
| \> instType |     String     |                                 Instrument type  <br/>`FUTURES`  <br/>`OPTION`                                 |
|\> instFamily|     String     |                                               Instrument family                                                |
|  \> instId  |     String     |                                                 Instrument ID                                                  |
|    data     |Array of objects|                                                Subscribed data                                                 |
| \> instType |     String     |                                                Instrument type                                                 |
|  \> instId  |     String     |                                      Instrument ID, e.g. `BTC-USD-170310`                                      |
|\> settleType|     String     |Type  <br/>`settlement`: Futures settlement  <br/>`delivery`: Futures delivery  <br/>`exercise`: Option exercise|
| \> settlePx |     String     |                                                Estimated price                                                 |
|    \> ts    |     String     |                 Data update time, Unix timestamp format in milliseconds, e.g. `1597026383085`                  |

---

### Mark price channel ###

Retrieve the mark price. Data will be pushed every 200 ms when the mark price changes, and will be pushed every 10 seconds when the mark price does not change.

#### URL Path ####

/ws/v5/public

>
>
> Request Example
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "mark-price",
      "instId": "LTC-USD-190628"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |      Type      |Required|                  Description                  |
|----------|----------------|--------|-----------------------------------------------|
|    op    |     String     |  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`|
|   args   |Array of objects|  Yes   |          List of subscribed channels          |
|\> channel|     String     |  Yes   |        Channel name  <br/>`mark-price`        |
|\> instId |     String     |  Yes   |                 Instrument ID                 |

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "mark-price",
    "instId": "LTC-USD-190628"
  },
  "connId": "a4d3ae55"
}

```

>
>
> Failure Response Example
>
>

```
{
  "event": "error",
  "code": "60012",
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"mark-price\", \"instId\" : \"LTC-USD-190628\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|                       Description                       |
|----------|------|--------|---------------------------------------------------------|
|  event   |String|  Yes   |Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|   arg    |Object|   No   |                   Subscribed channel                    |
|\> channel|String|  Yes   |                      Channel name                       |
|\> instId |String|   No   |                      Instrument ID                      |
|   code   |String|   No   |                       Error code                        |
|   msg    |String|   No   |                      Error message                      |
|  connId  |String|  Yes   |                 WebSocket connection ID                 |

>
>
> Push Data Example
>
>

```
{
  "arg": {
    "channel": "mark-price",
    "instId": "LTC-USD-190628"
  },
  "data": [
    {
      "instType": "FUTURES",
      "instId": "LTC-USD-190628",
      "markPx": "0.1",
      "ts": "1597026383085"
    }
  ]
}

```

#### Push data parameters ####

| Parameter |      Type      |                                 Description                                  |
|-----------|----------------|------------------------------------------------------------------------------|
|    arg    |     Object     |                       Successfully subscribed channel                        |
|\> channel |     String     |                                 Channel name                                 |
| \> instId |     String     |                                Instrument ID                                 |
|   data    |Array of objects|                               Subscribed data                                |
|\> instType|     String     |                               Instrument type                                |
| \> instId |     String     |                                Instrument ID                                 |
| \> markPx |     String     |                                  Mark price                                  |
|   \> ts   |     String     |Price update time, Unix timestamp format in milliseconds, e.g. `1597026383085`|

---

### Index tickers channel ###

Retrieve index tickers data. Push data every 100ms if there are any changes, otherwise push once a minute.

#### URL Path ####

/ws/v5/public

>
>
> Request Example
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "index-tickers",
      "instId": "BTC-USDT"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |      Type      |Required|                             Description                              |
|----------|----------------|--------|----------------------------------------------------------------------|
|    op    |     String     |  Yes   |                      `subscribe` `unsubscribe`                       |
|   args   |Array of objects|  Yes   |                     List of subscribed channels                      |
|\> channel|     String     |  Yes   |                  Channel name  <br/>`index-tickers`                  |
|\> instId |     String     |  Yes   |Index with USD, USDT, BTC, USDC as the quote currency, e.g. `BTC-USDT`|

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "index-tickers",
    "instId": "BTC-USDT"
  },
  "connId": "a4d3ae55"
}

```

>
>
> Failure Response Example
>
>

```
{
  "event": "error",
  "code": "60012",
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"index-tickers\", \"instId\" : \"BTC-USDT\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|                             Description                              |
|----------|------|--------|----------------------------------------------------------------------|
|  event   |String|  Yes   |                  `subscribe` `unsubscribe` `error`                   |
|   arg    |Object|   No   |                          Subscribed channel                          |
|\> channel|String|  Yes   |                  Channel name  <br/>`index-tickers`                  |
|\> instId |String|  Yes   |Index with USD, USDT, BTC, USDC as the quote currency, e.g. `BTC-USDT`|
|   code   |String|   No   |                              Error code                              |
|   msg    |String|   No   |                            Error message                             |
|  connId  |String|  Yes   |                       WebSocket connection ID                        |

>
>
> Push Data Example
>
>

```
{
  "arg": {
    "channel": "index-tickers",
    "instId": "BTC-USDT"
  },
  "data": [
    {
      "instId": "BTC-USDT",
      "idxPx": "0.1",
      "high24h": "0.5",
      "low24h": "0.1",
      "open24h": "0.1",
      "sodUtc0": "0.1",
      "sodUtc8": "0.1",
      "ts": "1597026383085"
    }
  ]
}

```

#### Push data parameters ####

|Parameter |      Type      |                                        Description                                         |
|----------|----------------|--------------------------------------------------------------------------------------------|
|   arg    |     Object     |                              Successfully subscribed channel                               |
|\> channel|     String     |                                        Channel name                                        |
|\> instId |     String     |              Index with USD, USDT, or BTC as quote currency, e.g. `BTC-USDT`.              |
|   data   |Array of objects|                                      Subscribed data                                       |
|\> instId |     String     |                                           Index                                            |
| \> idxPx |     String     |                                     Latest Index Price                                     |
|\> open24h|     String     |                              Open price in the past 24 hours                               |
|\> high24h|     String     |                             Highest price in the past 24 hours                             |
|\> low24h |     String     |                             Lowest price in the past 24 hours                              |
|\> sodUtc0|     String     |                                  Open price in the UTC 0                                   |
|\> sodUtc8|     String     |                                  Open price in the UTC 8                                   |
|  \> ts   |     String     |Update time of the index ticker, Unix timestamp format in milliseconds, e.g. `1597026383085`|

---

### Mark price candlesticks channel ###

Retrieve the candlesticks data of the mark price. The push frequency is the fastest interval 1 second push the data.

#### URL Path ####

/ws/v5/business

>
>
> Request Example
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "mark-price-candle1D",
      "instId": "BTC-USD-190628"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |      Type      |Required|                                                                                                                                                                                                                                                                                                                                                                                                                         Description                                                                                                                                                                                                                                                                                                                                                                                                                         |
|----------|----------------|--------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    op    |     String     |  Yes   |                                                                                                                                                                                                                                                                                                                                                                                                          Operation  <br/>`subscribe` `unsubscribe`                                                                                                                                                                                                                                                                                                                                                                                                          |
|   args   |Array of objects|  Yes   |                                                                                                                                                                                                                                                                                                                                                                                                                 List of subscribed channels                                                                                                                                                                                                                                                                                                                                                                                                                 |
|\> channel|     String     |  Yes   |Channel name   <br/>`mark-price-candle3M`   <br/>`mark-price-candle1M`   <br/>`mark-price-candle1W`   <br/>`mark-price-candle1D`   <br/>`mark-price-candle2D`   <br/>`mark-price-candle3D`   <br/>`mark-price-candle5D`   <br/>`mark-price-candle12H`   <br/>`mark-price-candle6H`   <br/>`mark-price-candle4H`   <br/>`mark-price-candle2H`   <br/>`mark-price-candle1H`   <br/>`mark-price-candle30m`   <br/>`mark-price-candle15m`   <br/>`mark-price-candle5m`   <br/>`mark-price-candle3m`   <br/>`mark-price-candle1m`   <br/>`mark-price-candle1Yutc`   <br/>`mark-price-candle3Mutc`   <br/>`mark-price-candle1Mutc`   <br/>`mark-price-candle1Wutc`   <br/>`mark-price-candle1Dutc`   <br/>`mark-price-candle2Dutc`   <br/>`mark-price-candle3Dutc`   <br/>`mark-price-candle5Dutc`   <br/>`mark-price-candle12Hutc`   <br/>`mark-price-candle6Hutc`|
|\> instId |     String     |  Yes   |                                                                                                                                                                                                                                                                                                                                                                                                                        Instrument ID                                                                                                                                                                                                                                                                                                                                                                                                                        |

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "mark-price-candle1D",
    "instId": "BTC-USD-190628"
  },
  "connId": "a4d3ae55"
}

```

>
>
> Failure Response Example
>
>

```
{
  "event": "error",
  "code": "60012",
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"mark-price-candle1D\", \"instId\" : \"BTC-USD-190628\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|                       Description                       |
|----------|------|--------|---------------------------------------------------------|
|  event   |String|  Yes   |Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|   arg    |Object|   No   |                   Subscribed channel                    |
|\> channel|String|  Yes   |                      Channel name                       |
|\> instId |String|  Yes   |                      Instrument ID                      |
|   code   |String|   No   |                       Error code                        |
|   msg    |String|   No   |                      Error message                      |
|  connId  |String|  Yes   |                 WebSocket connection ID                 |

>
>
> Push Data Example
>
>

```
{
  "arg": {
    "channel": "mark-price-candle1D",
    "instId": "BTC-USD-190628"
  },
  "data": [
    ["1597026383085", "3.721", "3.743", "3.677", "3.708","0"],
    ["1597026383085", "3.731", "3.799", "3.494", "3.72","1"]
  ]
}

```

#### Push data parameters ####

|Parameter |     Type      |                                                Description                                                 |
|----------|---------------|------------------------------------------------------------------------------------------------------------|
|   arg    |    Object     |                                      Successfully subscribed channel                                       |
|\> channel|    String     |                                                Channel name                                                |
|\> instId |    String     |                                               Instrument ID                                                |
|   data   |Array of Arrays|                                              Subscribed data                                               |
|  \> ts   |    String     |        Opening time of the candlestick, Unix timestamp format in milliseconds, e.g. `1597026383085`        |
|   \> o   |    String     |                                                 Open price                                                 |
|   \> h   |    String     |                                               Highest price                                                |
|   \> l   |    String     |                                                Lowest price                                                |
|   \> c   |    String     |                                                Close price                                                 |
|\> confirm|    String     |The state of candlesticks.  <br/>`0` represents that it is uncompleted, `1` represents that it is completed.|

---

### Index candlesticks channel ###

Retrieve the candlesticks data of the index. The push frequency is the fastest interval 1 second push the data. .

#### URL Path ####

/ws/v5/business

>
>
> Request Example
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "index-candle30m",
      "instId": "BTC-USD"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |      Type      |Required|                                                                                                                                                                                                                                                                                                                                        Description                                                                                                                                                                                                                                                                                                                                         |
|----------|----------------|--------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    op    |     String     |  Yes   |                                                                                                                                                                                                                                                                                                                      Operation  <br/>`subscribe`  <br/>`unsubscribe`                                                                                                                                                                                                                                                                                                                       |
|   args   |Array of objects|  Yes   |                                                                                                                                                                                                                                                                                                                                List of subscribed channels                                                                                                                                                                                                                                                                                                                                 |
|\> channel|     String     |  Yes   |Channel name   <br/>`index-candle3M`   <br/>`index-candle1M`   <br/>`index-candle1W`   <br/>`index-candle1D`   <br/>`index-candle2D`   <br/>`index-candle3D`   <br/>`index-candle5D`   <br/>`index-candle12H`   <br/>`index-candle6H`   <br/>`index-candle4H`   <br/>`index -candle2H`   <br/>`index-candle1H`   <br/>`index-candle30m`   <br/>`index-candle15m`   <br/>`index-candle5m`   <br/>`index-candle3m`   <br/>`index-candle1m`   <br/>`index-candle3Mutc`   <br/>`index-candle1Mutc`   <br/>`index-candle1Wutc`   <br/>`index-candle1Dutc`   <br/>`index-candle2Dutc`   <br/>`index-candle3Dutc`   <br/>`index-candle5Dutc`   <br/>`index-candle12Hutc`   <br/>`index-candle6Hutc`|
|\> instId |     String     |  Yes   |                                                                                                                                                                                                                                                                                                                                   Index, e.g. `BTC-USD`                                                                                                                                                                                                                                                                                                                                    |

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "index-candle30m",
    "instId": "BTC-USD"
  },
  "connId": "a4d3ae55"
}

```

>
>
> Failure Response Example
>
>

```
{
  "event": "error",
  "code": "60012",
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"index-candle30m\", \"instId\" : \"BTC-USD\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|       Description       |
|----------|------|--------|-------------------------|
|  event   |String|  Yes   |`subscribe` `unsubscribe`|
|   arg    |Object|   No   |   Subscribed channel    |
|\> channel|String|  Yes   |      Channel name       |
|\> instId |String|   No   |  Index, e.g. `BTC-USD`  |
|   code   |String|   No   |       Error code        |
|   msg    |String|   No   |      Error message      |
|  connId  |String|  Yes   | WebSocket connection ID |

>
>
> Push Data Example
>
>

```
{
  "arg": {
    "channel": "index-candle30m",
    "instId": "BTC-USD"
  },
  "data": [["1597026383085", "3811.31", "3811.31", "3811.31", "3811.31", "0"]]
}

```

#### Push data parameters ####

|Parameter |     Type      |                                                Description                                                 |
|----------|---------------|------------------------------------------------------------------------------------------------------------|
|   arg    |    Object     |                                      Successfully subscribed channel                                       |
|\> channel|    String     |                                                Channel name                                                |
|\> instId |    String     |                                                   Index                                                    |
|   data   |Array of Arrays|                                              Subscribed data                                               |
|  \> ts   |    String     |        Opening time of the candlestick, Unix timestamp format in milliseconds, e.g. `1597026383085`        |
|   \> o   |    String     |                                                 Open price                                                 |
|   \> h   |    String     |                                               Highest price                                                |
|   \> l   |    String     |                                                Lowest price                                                |
|   \> c   |    String     |                                                Close price                                                 |
|\> confirm|    String     |The state of candlesticks.  <br/>`0` represents that it is uncompleted, `1` represents that it is completed.|
The order of the returned values is: [ts,o,h,l,c,confirm]

---

### Liquidation orders channel ###

Retrieve the recent liquidation orders. For futures and swaps, each contract will only show a maximum of one order per one-second period. This data doesn’t represent the total number of liquidations on OKX.

#### URL Path ####

/ws/v5/public

>
>
> Request Example
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "liquidation-orders",
      "instType": "SWAP"
    }
  ]
}

```

#### Request Parameters ####

| Parameter |     Type      |Required|                               Description                                |
|-----------|---------------|--------|--------------------------------------------------------------------------|
|    op     |    String     |  Yes   |             Operation  <br/>`subscribe`  <br/>`unsubscribe`              |
|   args    |Array of object|  Yes   |                       List of subscribed channels                        |
|\> channel |    String     |  Yes   |                 Channel name  <br/>`liquidation-orders`                  |
|\> instType|    String     |  Yes   |Instrument type  <br/>`SWAP`  <br/>`FUTURES`  <br/>`MARGIN`  <br/>`OPTION`|

>
>
> Response Example
>
>

```
{
    "arg": {
        "channel": "liquidation-orders",
        "instType": "SWAP"
    },
    "data": [
        {
            "details": [
                {
                    "bkLoss": "0",
                    "bkPx": "0.007831",
                    "ccy": "",
                    "posSide": "short",
                    "side": "buy",
                    "sz": "13",
                    "ts": "1692266434010"
                }
            ],
            "instFamily": "IOST-USDT",
            "instId": "IOST-USDT-SWAP",
            "instType": "SWAP",
            "uly": "IOST-USDT"
        }
    ]
}

```

#### Response Parameters ####

|**Parameter**|    **Type**    |                                                                          **Description**                                                                           |
|-------------|----------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     arg     |     Object     |                                                                  Successfully subscribed channel                                                                   |
| \> channel  |     String     |                                                                            Channel name                                                                            |
|  \> instId  |     String     |                                                                           Instrument ID                                                                            |
|    data     |Array of objects|                                                                          Subscribed data                                                                           |
| \> instType |     String     |                                                                          Instrument type                                                                           |
|  \> instId  |     String     |                                                                 Instrument ID, e.g. `BTC-USD-SWAP`                                                                 |
|   \> uly    |     String     |                                                      Underlying, only applicable to `FUTURES`/`SWAP`/`OPTION`                                                      |
| \> details  |Array of objects|                                                                        Liquidation details                                                                         |
|  \>\> side  |     String     |                                                   Order side, `buy` `sell`, only applicable to `FUTURES`/`SWAP`                                                    |
|\>\> posSide |     String     |                               Position mode side  <br/>`long`: Hedge mode long  <br/>`short`: Hedge mode short  <br/>`net`: Net mode                               |
|  \>\> bkPx  |     String     |                     Bankruptcy price. The price of the transaction with the system's liquidation account, only applicable to `FUTURES`/`SWAP`                      |
|   \>\> sz   |     String     |Quantity of liquidation, only applicable to `MARGIN/FUTURES/SWAP`.  <br/> For `MARGIN`, the unit is base currency.   <br/> For `FUTURES/SWAP`, the unit is contract.|
| \>\> bkLoss |     String     |                                                                          Bankruptcy loss                                                                           |
|  \>\> ccy   |     String     |                                                         Liquidation currency, only applicable to `MARGIN`                                                          |
|   \>\> ts   |     String     |                                           Liquidation time, Unix timestamp format in milliseconds, e.g. `1597026383085`                                            |

---

### ADL warning channel ###

Auto-deleveraging warning channel.

In the `normal` state, data will be pushed once every minute to display the balance of insurance fund and etc.

In the warning state or when there is ADL risk (`warning/adl`), data will be pushed every second to display information such as the real-time decline rate of insurance fund.

For more ADL details, please refer to [Introduction to Auto-deleveraging](https://www.okx.com/help/iv-introduction-to-auto-deleveraging-adl)

#### URL Path ####

/ws/v5/public

>
>
> Request Example
>
>

```
{
    "op": "subscribe",
    "args": [{
        "channel": "adl-warning",
        "instType": "FUTURES",
        "instFamily": "BTC-USDT"
    }]
}

```

#### Request Parameters ####

|  Parameter  |     Type      |Required|                        Description                        |
|-------------|---------------|--------|-----------------------------------------------------------|
|     op      |    String     |  Yes   |      Operation  <br/>`subscribe`  <br/>`unsubscribe`      |
|    args     |Array of object|  Yes   |                List of subscribed channels                |
| \> channel  |    String     |  Yes   |             Channel name  <br/>`adl-warning`              |
| \> instType |    String     |  Yes   |Instrument type  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`|
|\> instFamily|    String     |   No   |                     Instrument family                     |

>
>
> Successful Response Example
>
>

```
{
   "event":"subscribe",
   "arg":{
      "channel":"adl-warning",
      "instType":"FUTURES",
      "instFamily":"BTC-USDT"
   },
   "connId":"48d8960a"
}

```

>
>
> Failure Response Example
>
>

```
{
   "event":"error",
   "msg":"Illegal request: { \"event\": \"subscribe\", \"arg\": { \"channel\": \"adl-warning\", \"instType\": \"FUTURES\", \"instFamily\": \"BTC-USDT\" } }",
   "code":"60012",
   "connId":"48d8960a"
}

```

#### Response parameters ####

|  Parameter  | Type |Required|                       Description                       |
|-------------|------|--------|---------------------------------------------------------|
|    event    |String|  Yes   |Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|     arg     |Object|   No   |                   Subscribed channel                    |
| \> channel  |String|  Yes   |            Channel name  <br/>`adl-warning`             |
| \> instType |String|  Yes   |                     Instrument type                     |
|\> instFamily|String|   No   |                    Instrument family                    |
|    code     |String|   No   |                       Error code                        |
|     msg     |String|   No   |                      Error message                      |
|   connId    |String|  Yes   |                 WebSocket connection ID                 |

>
>
> Push Data Example
>
>

```
{
   "arg":{
      "channel":"adl-warning",
      "instType":"FUTURES",
      "instFamily":"BTC-USDT"
   },
   "data":[
      {
         "decRate":"",
         "maxBal":"",
         "adlRecRate":"0.25",
         "adlRecBal":"8000.0",
         "bal":"280784384.9564228289548144",
         "instType":"FUTURES",
         "adlRate":"0.5",
         "ccy": "USDT",
         "instFamily":"BTC-USDT",
         "maxBalTs":"",
         "adlType":"",
         "state":"normal",
         "adlBal":"0",
         "ts":"1700210763001"
      }
   ]
}

```

#### Push data parameters ####

|  Parameter  |     Type      |                                                                                                                                                                Description                                                                                                                                                                 |
|-------------|---------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     arg     |    Object     |                                                                                                                                                             Subscribed channel                                                                                                                                                             |
| \> channel  |    String     |                                                                                                                                                      Channel name  <br/>`adl-warning`                                                                                                                                                      |
| \> instType |    String     |                                                                                                                                                              Instrument type                                                                                                                                                               |
|\> instFamily|    String     |                                                                                                                                                             Instrument family                                                                                                                                                              |
|    data     |Array of object|                                                                                                                                                              Subscribed data                                                                                                                                                               |
| \> instType |    String     |                                                                                                                                                              Instrument type                                                                                                                                                               |
|\> instFamily|    String     |                                                                                                                                                             Instrument family                                                                                                                                                              |
|  \> state   |    String     |                                                                                                                                            state   <br/>`normal`   <br/>`warning`   <br/>`adl`                                                                                                                                             |
|   \> bal    |    String     |                                                                                                                                                      Real-time insurance fund balance                                                                                                                                                      |
|   \> ccy    |    String     |                                                                                                                                            The corresponding currency of insurance fund balance                                                                                                                                            |
|  \> maxBal  |    String     |                                                                                                               Maximum insurance fund balance in the past eight hours   <br/><br/>Applicable when state is `warning` or `adl`                                                                                                               |
| \> maxBalTs |    String     |                                                                                                 Timestamp when insurance fund balance reached maximum in the past eight hours, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                                                 |
| \> decRate  |    String     |                                                                                                           Real-time insurance fund decline rate (compare bal and maxBal)   <br/><br/>Applicable when state is `warning` or `adl`                                                                                                           |
| \> adlType  |    String     |ADL related events   <br/>`rate_adl_start`: ADL begins due to high insurance fund decline rate   <br/>`bal_adl_start`: ADL begins due to insurance fund balance falling   <br/>`pos_adl_start`：ADL begins due to the volume of liquidation orders falls to a certain level (only applicable to premarket symbols)   <br/>`adl_end`: ADL ends|
|  \> adlBal  |    String     |                                                                                                                                                  Insurance fund balance that triggers ADL                                                                                                                                                  |
| \> adlRate  |    String     |                                                                                                                                               Insurance fund decline rate that triggers ADL                                                                                                                                                |
|\> adlRecBal |    String     |                                                                                                                                                 Insurance fund balance that turns off ADL                                                                                                                                                  |
|\> adlRecRate|    String     |                                                                                                                                               Insurance fund decline rate that turns off ADL                                                                                                                                               |
|    \> ts    |    String     |                                                                                                                                Data push time, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                                                                                 |

---

### Economic calendar channel ###

This endpoint is only supported in production environment.

Retrieve the most up-to-date economic calendar data. This endpoint is only applicable to VIP 1 and above users in the trading fee tier.

#### URL Path ####

/ws/v5/business (required login)

>
>
> Request Example
>
>

```
{
    "op": "subscribe",
    "args": [
      {
          "channel": "economic-calendar"
      }
    ]
}

```

#### Request parameters ####

|Parameter |      Type      |Required|                  Description                  |
|----------|----------------|--------|-----------------------------------------------|
|    op    |     String     |  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`|
|   args   |Array of objects|  Yes   |          List of subscribed channels          |
|\> channel|     String     |  Yes   |    Channel name  <br/>`economic-calendar`     |

>
>
> Successful Response Example
>
>

```
{
    "event": "subscribe",
    "arg": {
        "channel": "economic-calendar"
    }
}

```

>
>
> Failure Response Example
>
>

```
{
  "event": "error",
  "code": "60012",
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"economic-calendar\", \"instId\" : \"LTC-USD-190628\"}]}"
}

```

#### Response parameters ####

|Parameter | Type |Required|                       Description                       |
|----------|------|--------|---------------------------------------------------------|
|  event   |String|  Yes   |Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|   arg    |Object|   No   |                   Subscribed channel                    |
|\> channel|String|  Yes   |                      Channel name                       |
|   code   |String|   No   |                       Error code                        |
|   msg    |String|   No   |                      Error message                      |
|  connId  |String|  Yes   |                 WebSocket connection ID                 |

>
>
> Push Data Example
>
>

```
{
    "arg": {
        "channel": "economic-calendar"
    },
    "data": [
        {
            "calendarId": "319275",
            "date": "1597026383085",
            "region": "United States",
            "category": "Manufacturing PMI",
            "event": "S&P Global Manufacturing PMI Final",
            "refDate": "1597026383085",
            "actual": "49.2",
            "previous": "47.3",
            "forecast": "49.3",
            "importance": "2",
            "prevInitial": "",
            "ccy": "",
            "unit": "",
            "ts": "1698648096590"
        }
    ]
}

```

#### Push data parameters ####

|  Parameter   |      Type      |                                                  Description                                                  |
|--------------|----------------|---------------------------------------------------------------------------------------------------------------|
|     arg      |     Object     |                                        Successfully subscribed channel                                        |
|  \> channel  |     String     |                                                 Channel name                                                  |
|     data     |Array of objects|                                                Subscribed data                                                |
|   \> event   |     string     |                                                  Event name                                                   |
|  \> region   |     string     |                                           Country, region or entity                                           |
| \> category  |     string     |                                                 Category name                                                 |
|  \> actual   |     string     |                                        The actual value of this event                                         |
| \> previous  |     string     |     Latest actual value of the previous period   <br/>The value will be revised if revision is applicable     |
| \> forecast  |     string     |                          Average forecast among a representative group of economists                          |
|\> prevInitial|     string     |             The initial value of the previous period   <br/>Only applicable when revision happens             |
|   \> date    |     string     |Estimated release time of the value of actual field, millisecond format of Unix timestamp, e.g. `1597026383085`|
|  \> refDate  |     string     |                                    Date for which the datapoint refers to                                     |
|\> calendarId |     string     |                                                  Calendar ID                                                  |
|   \> unit    |     string     |                                               Unit of the data                                                |
|    \> ccy    |     string     |                                             Currency of the data                                              |
|\> importance |     string     |                    Level of importance  <br/>`1`: low   <br/>`2`: medium   <br/>`3`: high                     |
|    \> ts     |     string     |                                         The time of the latest update                                         |

---

