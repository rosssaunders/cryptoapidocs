# OKX API Documentation - Private Orderbook Trading WebSocket API

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

### Public ###

Error Code from 60000 to 64002

#### General Class ####

|Error Code|                                                                                                                         Error Message                                                                                                                          |
|----------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|  60004   |                                                                                                                       Invalid timestamp                                                                                                                        |
|  60005   |                                                                                                                         Invalid apiKey                                                                                                                         |
|  60006   |                                                                                                                   Timestamp request expired                                                                                                                    |
|  60007   |                                                                                                                          Invalid sign                                                                                                                          |
|  60008   |                                                                          The current WebSocket endpoint does not support subscribing to {0} channels. Please check the WebSocket URL                                                                           |
|  60009   |                                                                                                                         Login failure                                                                                                                          |
|  60011   |                                                                                                                         Please log in                                                                                                                          |
|  60012   |                                                                                                                        Invalid request                                                                                                                         |
|  60013   |                                                                                                                          Invalid args                                                                                                                          |
|  60014   |                                                                                                                     Requests too frequent                                                                                                                      |
|  60018   |                                                                         Wrong URL or {0} doesn't exist. Please use the correct URL, channel and parameters referring to API document.                                                                          |
|  60019   |                                                                                                                        Invalid op: {op}                                                                                                                        |
|  60020   |                                                                                                       APIKey subscription amount exceeds the limit {0}.                                                                                                        |
|  60021   |                                                                                                    This operation does not support multiple accounts login.                                                                                                    |
|  60022   |                                                                                                                 Bulk login partially succeeded                                                                                                                 |
|  60023   |                                                                                                                Bulk login requests too frequent                                                                                                                |
|  60024   |                                                                                                                        Wrong passphrase                                                                                                                        |
|  60025   |                                                                                                        token subscription amount exceeds the limit {0}                                                                                                         |
|  60026   |                                                                                                Batch login by APIKey and token simultaneously is not supported.                                                                                                |
|  60027   |                                                                                                                Parameter {0} can not be empty.                                                                                                                 |
|  60028   |                                                                          The current operation is not supported by this URL. Please use the correct WebSocket URL for the operation.                                                                           |
|  60029   |                                                                                Only users who are VIP5 and above in trading fee tier are allowed to subscribe to this channel.                                                                                 |
|  60030   |                                                                           Only users who are VIP4 and above in trading fee tier are allowed to subscribe to books50-l2-tbt channel.                                                                            |
|  60031   |                                                                                               The WebSocket endpoint does not allow multiple or repeated logins.                                                                                               |
|  60032   |                                                                                                                     API key doesn't exist.                                                                                                                     |
|  63999   |                                                                                                  Login failed due to internal error. Please try again later.                                                                                                   |
|  64000   |                         Subscription parameter uly is unavailable anymore, please replace uly with instFamily. More details can refer to: https://www.okx.com/help-center/changes-to-v5-api-websocket-subscription-parameter-and-url.                          |
|  64001   |                       This channel has been migrated to the '/business' URL. Please subscribe using the new URL. More details can refer to: https://www.okx.com/help-center/changes-to-v5-api-websocket-subscription-parameter-and-url.                        |
|  64002   |This channel is not supported by "/business" URL. Please use "/private" URL(for private channels), or "/public" URL(for public channels). More details can refer to: https://www.okx.com/help-center/changes-to-v5-api-websocket-subscription-parameter-and-url.|
|  64003   |                                                                                           Your trading fee tier doesn't meet the requirement to access this channel                                                                                            |

#### Close Frame ####

|Status Code|                               Reason Text                               |
|-----------|-------------------------------------------------------------------------|
|   1009    |            Request message exceeds the maximum frame length             |
|   4001    |                              Login Failed                               |
|   4002    |                             Invalid Request                             |
|   4003    |            APIKey subscription amount exceeds the limit 100             |
|   4004    |                         No data received in 30s                         |
|   4005    |                    Buffer is full, cannot write data                    |
|   4006    |                         Abnormal disconnection                          |
|   4007    |         API key has been updated or deleted. Please reconnect.          |
|   4008    |      The number of subscribed channels exceeds the maximum limit.       |
|   4009    |The number of subscription channels for this connection exceeds the limit|
Disclaimer: The availability of products and services listed on this page will depend on your region. Please see your applicable Terms of Service for more detail.

---

Production Trading Services
----------

The Production Trading URL:

* REST: `https://www.okx.com`  

* Public WebSocket: `wss://ws.okx.com:8443/ws/v5/public`  

* Private WebSocket: `wss://ws.okx.com:8443/ws/v5/private`
* Business WebSocket: `wss://ws.okx.com:8443/ws/v5/business`

---

Demo Trading Services
----------

Currently, the V5 API works for Demo Trading, but some functions are not supported, such as `withdraw`,`deposit`,`purchase/redemption`, etc.

The Demo Trading URL:

* REST: `https://www.okx.com`  

* Public WebSocket: `wss://wspap.okx.com:8443/ws/v5/public`  

* Private WebSocket: `wss://wspap.okx.com:8443/ws/v5/private`  

* Business WebSocket: `wss://wspap.okx.com:8443/ws/v5/business`

OKX account can be used for login on Demo Trading. If you already have an OKX account, you can log in directly.

Start API Demo Trading by the following steps:  
Login OKX —\> Trade —\> Demo Trading —\> Personal Center —\> Demo Trading API -\> Create Demo Trading V5 API Key —\> Start your Demo Trading

Note: `x-simulated-trading: 1` needs to be added to the header of the Demo Trading request.
>
>
> Http Header Example
>
>

```
Content-Type: application/json

OK-ACCESS-KEY: 37c541a1-****-****-****-10fe7a038418

OK-ACCESS-SIGN: leaVRETrtaoEQ3yI9qEtI1CZ82ikZ4xSG5Kj8gnl3uw=

OK-ACCESS-PASSPHRASE: 1****6

OK-ACCESS-TIMESTAMP: 2020-03-28T12:21:41.274Z

x-simulated-trading: 1

```

---

### Demo Trading Explorer ###

You need to sign in to your OKX account before accessing the explorer. The interface only allow access to the demo trading environment.

* Clicking `Try it out` button in Parameters Panel and editing request parameters.

* Clicking `Execute` button to send your request. You can check response in Responses panel.

Try [demo trading explorer](/demo-trading-explorer/v5/en)

---

### Account channel ###

Retrieve account information. Data will be pushed when triggered by events such as placing order, canceling order, transaction execution, etc.
It will also be pushed in regular interval according to subscription granularity.  

Concurrent connection to this channel will be restricted by the following rules: [WebSocket connection count limit](/docs-v5/en/#overview-websocket-connection-count-limit).

#### URL Path ####

/ws/v5/private (required login)

>
>
> Request Example : single
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "account",
      "ccy": "BTC"
    }
  ]
}

```

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
      "channel": "account",
      "extraParams": "
        {
          \"updateInterval\": \"0\"
        }
      "
    }
  ]
}

```

#### Request Parameters ####

|     Parameter     |     Type      |Required|                                                                                                                                                        Description                                                                                                                                                        |
|-------------------|---------------|--------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|        op         |    String     |  Yes   |                                                                                                                                      Operation  <br/>`subscribe`  <br/>`unsubscribe`                                                                                                                                      |
|       args        |Array of object|  Yes   |                                                                                                                                                List of subscribed channels                                                                                                                                                |
|    \> channel     |    String     |  Yes   |                                                                                                                                               Channel name  <br/>`account`                                                                                                                                                |
|      \> ccy       |    String     |   No   |                                                                                                                                                         Currency                                                                                                                                                          |
|  \> extraParams   |    String     |   No   |                                                                                                                                                 Additional configuration                                                                                                                                                  |
|\>\> updateInterval|      int      |   No   |`0`: only push due to account events   <br/>The data will be pushed both by events and regularly if this field is omitted or set to other values than 0.   <br/>The following format should be strictly obeyed when using this field.   <br/>"extraParams": "  <br/>{  <br/> \\"updateInterval\\": \\"0\\"  <br/> }  <br/>"|

>
>
> Successful Response Example : single
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "account",
    "ccy": "BTC"
  },
  "connId": "a4d3ae55"
}

```

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "account"
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
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"account\", \"ccy\" : \"BTC\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|                         Description                         |
|----------|------|--------|-------------------------------------------------------------|
|  event   |String|  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|   arg    |Object|   No   |                     Subscribed channel                      |
|\> channel|String|  Yes   |                Channel name  <br/>`account`                 |
|  \> ccy  |String|   No   |                          Currency                           |
|   code   |String|   No   |                         Error code                          |
|   msg    |String|   No   |                        Error message                        |
|  connId  |String|  Yes   |                   WebSocket connection ID                   |

>
>
> Push Data Example
>
>

```
{
    "arg": {
        "channel": "account",
        "uid": "44*********584"
    },
    "eventType": "snapshot",
    "curPage": 1,
    "lastPage": true,
    "data": [{
        "adjEq": "55444.12216906034",
        "borrowFroz": "0",
        "details": [{
            "availBal": "4734.371190691436",
            "availEq": "4734.371190691435",
            "borrowFroz": "0",
            "cashBal": "4750.426970691436",
            "ccy": "USDT",
            "coinUsdPrice": "0.99927",
            "crossLiab": "0",
            "collateralEnabled": false,
            "disEq": "4889.379316336831",
            "eq": "4892.951170691435",
            "eqUsd": "4889.379316336831",
            "smtSyncEq": "0",
            "spotCopyTradingEq": "0",
            "fixedBal": "0",
            "frozenBal": "158.57998",
            "imr": "",
            "interest": "0",
            "isoEq": "0",
            "isoLiab": "0",
            "isoUpl": "0",
            "liab": "0",
            "maxLoan": "0",
            "mgnRatio": "",
            "mmr": "",
            "notionalLever": "",
            "ordFrozen": "0",
            "rewardBal": "0",
            "spotInUseAmt": "",
            "clSpotInUseAmt": "",
            "maxSpotInUseAmt": "",          
            "spotIsoBal": "0",
            "stgyEq": "150",
            "twap": "0",
            "uTime": "1705564213903",
            "upl": "-7.475800000000003",
            "uplLiab": "0",
            "spotBal": "",
            "openAvgPx": "",
            "accAvgPx": "",
            "spotUpl": "",
            "spotUplRatio": "",
            "totalPnl": "",
            "totalPnlRatio": "",
            "collateralEnabled": ""
        }],
        "imr": "0",
        "isoEq": "0",
        "mgnRatio": "",
        "mmr": "0",
        "notionalUsd": "0",
        "notionalUsdForBorrow": "0",
        "notionalUsdForFutures": "0",
        "notionalUsdForOption": "0",
        "notionalUsdForSwap": "0",
        "ordFroz": "0",
        "totalEq": "",
        "uTime": "1705564223311",
        "upl": "0"
    }]
}

```

#### Push data parameters ####

|     **Parameters**     |   **Types**    |                                                                                                                                                                                                                                                                                             **Description**                                                                                                                                                                                                                                                                                             |
|------------------------|----------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|          arg           |     Object     |                                                                                                                                                                                                                                                                                     Successfully subscribed channel                                                                                                                                                                                                                                                                                     |
|       \> channel       |     String     |                                                                                                                                                                                                                                                                                              Channel name                                                                                                                                                                                                                                                                                               |
|         \> uid         |     String     |                                                                                                                                                                                                                                                                                             User Identifier                                                                                                                                                                                                                                                                                             |
|       eventType        |     String     |                                                                                                                                                                                                                                            Event type:   <br/>`snapshot`: Initial and regular snapshot push   <br/>`event_update`: Event-driven update push                                                                                                                                                                                                                                             |
|        curPage         |    Integer     |                                                                                                                                                                                                                                                Current page number.   <br/>Only applicable for `snapshot` events. Not included in `event_update` events.                                                                                                                                                                                                                                                |
|        lastPage        |    Boolean     |                                                                                                                                                                                                                       Whether this is the last page of pagination:  <br/>`true`  <br/>`false`  <br/>Only applicable for `snapshot` events. Not included in `event_update` events.                                                                                                                                                                                                                       |
|          data          |Array of objects|                                                                                                                                                                                                                                                                                             Subscribed data                                                                                                                                                                                                                                                                                             |
|        \> uTime        |     String     |                                                                                                                                                                                                                                                 The latest time to get account information, millisecond format of Unix timestamp, e.g. `1597026383085`                                                                                                                                                                                                                                                  |
|       \> totalEq       |     String     |                                                                                                                                                                                                                                                                                   The total amount of equity in `USD`                                                                                                                                                                                                                                                                                   |
|        \> isoEq        |     String     |                                                                                                                                                                                                                                         Isolated margin equity in `USD`  <br/>Applicable to `Spot and futures mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                          |
|        \> adjEq        |     String     |Adjusted / Effective equity in `USD`   <br/>The net fiat value of the assets in the account that can provide margins for spot, expiry futures, perpetual futures and options under the cross-margin mode.   <br/>In multi-ccy or PM mode, the asset and margin requirement will all be converted to USD value to process the order check or liquidation.   <br/>Due to the volatility of each currency market, our platform calculates the actual USD value of each currency based on discount rates to balance market risks.   <br/>Applicable to `Spot mode`/`Multi-currency margin`/`Portfolio margin`|
|       \> ordFroz       |     String     |                                                                                                                                                                                                                                              Margin frozen for pending cross orders in `USD`   <br/>Only applicable to `Spot mode`/`Multi-currency margin`                                                                                                                                                                                                                                              |
|         \> imr         |     String     |                                                                                                                                                                                      Initial margin requirement in `USD`   <br/>The sum of initial margins of all open positions and pending orders under cross-margin mode in `USD`.   <br/>Applicable to `Spot mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                       |
|         \> mmr         |     String     |                                                                                                                                                                                  Maintenance margin requirement in `USD`   <br/>The sum of maintenance margins of all open positions and pending orders under cross-margin mode in `USD`.   <br/>Applicable to `Spot mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                   |
|     \> borrowFroz      |     String     |                                                                                                                                                                                                                   Potential borrowing IMR of the account in `USD`   <br/>Only applicable to `Spot mode`/`Multi-currency margin`/`Portfolio margin`. It is "" for other margin modes.                                                                                                                                                                                                                    |
|      \> mgnRatio       |     String     |                                                                                                                                                                                                                                                   Margin ratio in `USD`.   <br/>Applicable to `Spot mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                                    |
|     \> notionalUsd     |     String     |                                                                                                                                                                                                                                            Notional value of positions in `USD`   <br/>Applicable to `Spot mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                             |
|\> notionalUsdForBorrow |     String     |                                                                                                                                                                                                                                              Notional value for `Borrow` in USD  <br/>Applicable to `Spot mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                              |
| \> notionalUsdForSwap  |     String     |                                                                                                                                                                                                                                        Notional value of positions for `Perpetual Futures` in USD  <br/>Applicable to `Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                        |
|\> notionalUsdForFutures|     String     |                                                                                                                                                                                                                                         Notional value of positions for `Expiry Futures` in USD  <br/>Applicable to `Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                          |
|\> notionalUsdForOption |     String     |                                                                                                                                                                                                                                       Notional value of positions for `Option` in USD  <br/>Applicable to `Spot mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                        |
|         \> upl         |     String     |                                                                                                                                                                                                                              Cross-margin info of unrealized profit and loss at the account level in `USD`  <br/>Applicable to `Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                               |
|       \> details       |Array of objects|                                                                                                                                                                                                                                                                              Detailed asset information in all currencies                                                                                                                                                                                                                                                                               |
|        \>\> ccy        |     String     |                                                                                                                                                                                                                                                                                                Currency                                                                                                                                                                                                                                                                                                 |
|        \>\> eq         |     String     |                                                                                                                                                                                                                                                                                           Equity of currency                                                                                                                                                                                                                                                                                            |
|      \>\> cashBal      |     String     |                                                                                                                                                                                                                                                                                              Cash Balance                                                                                                                                                                                                                                                                                               |
|       \>\> uTime       |     String     |                                                                                                                                                                                                                                                                Update time, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                                                                                                                                                                                                                 |
|       \>\> isoEq       |     String     |                                                                                                                                                                                                                                        Isolated margin equity of currency  <br/>Applicable to `Spot and futures mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                        |
|      \>\> availEq      |     String     |                                                                                                                                                                                                                                           Available equity of currency  <br/>Applicable to `Spot and futures mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                           |
|       \>\> disEq       |     String     |                                                                                                                                                                                                                                                                                  Discount equity of currency in `USD`                                                                                                                                                                                                                                                                                   |
|     \>\> fixedBal      |     String     |                                                                                                                                                                                                                                                                            Frozen balance for `Dip Sniper` and `Peak Sniper`                                                                                                                                                                                                                                                                            |
|     \>\> availBal      |     String     |                                                                                                                                                                                                                                                                                      Available balance of currency                                                                                                                                                                                                                                                                                      |
|     \>\> frozenBal     |     String     |                                                                                                                                                                                                                                                                                       Frozen balance of currency                                                                                                                                                                                                                                                                                        |
|     \>\> ordFrozen     |     String     |                                                                                                                                                                                                                                             Margin frozen for open orders   <br/>Applicable to `Spot mode`/`Spot and futures mode`/`Multi-currency margin`                                                                                                                                                                                                                                              |
|       \>\> liab        |     String     |                                                                                                                                                                                                                           Liabilities of currency  <br/>It is a positive value, e.g. `21625.64`.   <br/>Applicable to `Spot mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                            |
|        \>\> upl        |     String     |                                                                                                                                                                                                          The sum of the unrealized profit & loss of all margin and derivatives positions of currency.   <br/>Applicable to `Spot and futures mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                           |
|      \>\> uplLiab      |     String     |                                                                                                                                                                                                                                              Liabilities due to Unrealized loss of currency  <br/>Applicable to `Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                              |
|     \>\> crossLiab     |     String     |                                                                                                                                                                                                                                                Cross Liabilities of currency  <br/>Applicable to `Spot mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                                 |
|      \>\> isoLiab      |     String     |                                                                                                                                                                                                                                                     Isolated Liabilities of currency  <br/>Applicable to `Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                                     |
|     \>\> rewardBal     |     String     |                                                                                                                                                                                                                                                                                           Trial fund balance                                                                                                                                                                                                                                                                                            |
|     \>\> mgnRatio      |     String     |                                                                                                                                                                                                         Cross margin ratio of currency   <br/>The index for measuring the risk of a certain asset in the account.   <br/>Applicable to `Spot and futures mode` and when there is cross position                                                                                                                                                                                                         |
|        \>\> imr        |     String     |                                                                                                                                                                                                                                   Cross initial margin requirement at the currency level  <br/>Applicable to `Spot and futures mode` and when there is cross position                                                                                                                                                                                                                                   |
|        \>\> mmr        |     String     |                                                                                                                                                                                                                                 Cross maintenance margin requirement at the currency level  <br/>Applicable to `Spot and futures mode` and when there is cross position                                                                                                                                                                                                                                 |
|     \>\> interest      |     String     |                                                                                                                                                                                                                                   Interest of currency  <br/>It is a positive value, e.g."9.01". Applicable to `Spot mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                   |
|       \>\> twap        |     String     |                                                                                                                                                                           System is forced repayment(TWAP) indicator  <br/>Divided into multiple levels from 0 to 5, the larger the number, the more likely the auto repayment will be triggered.  <br/>Applicable to `Spot mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                            |
|      \>\> maxLoan      |     String     |                                                                                                                                                                                                                                               Max loan of currency  <br/>Applicable to `cross` of `Spot mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                                |
|       \>\> eqUsd       |     String     |                                                                                                                                                                                                                                                                                        Equity `USD` of currency                                                                                                                                                                                                                                                                                         |
|    \>\> borrowFroz     |     String     |                                                                                                                                                                                                                     Potential borrowing IMR of currency in `USD`   <br/>Only applicable to `Spot mode`/`Multi-currency margin`/`Portfolio margin`. It is "" for other margin modes.                                                                                                                                                                                                                     |
|   \>\> notionalLever   |     String     |                                                                                                                                                                                                                                                                    Leverage of currency  <br/>Applicable to `Spot and futures mode`                                                                                                                                                                                                                                                                     |
|   \>\> coinUsdPrice    |     String     |                                                                                                                                                                                                                                                                                      Price index `USD` of currency                                                                                                                                                                                                                                                                                      |
|      \>\> stgyEq       |     String     |                                                                                                                                                                                                                                                                                             strategy equity                                                                                                                                                                                                                                                                                             |
|      \>\> isoUpl       |     String     |                                                                                                                                                                                                                                 Isolated unrealized profit and loss of currency  <br/>Applicable to `Spot and futures mode`/`Multi-currency margin`/`Portfolio margin`                                                                                                                                                                                                                                  |
|   \>\> spotInUseAmt    |     String     |                                                                                                                                                                                                                                                                        Spot in use amount  <br/>Applicable to `Portfolio margin`                                                                                                                                                                                                                                                                        |
|  \>\> clSpotInUseAmt   |     String     |                                                                                                                                                                                                                                                               User-defined spot risk offset amount  <br/>Applicable to `Portfolio margin`                                                                                                                                                                                                                                                               |
|  \>\> maxSpotInUseAmt  |     String     |                                                                                                                                                                                                                                                               Max possible spot risk offset amount  <br/>Applicable to `Portfolio margin`                                                                                                                                                                                                                                                               |
|    \>\> spotIsoBal     |     String     |                                                                                                                                                                                                                                             Spot isolated balance  <br/>Applicable to copy trading  <br/>Applicable to `Spot mode`/`Spot and futures mode`                                                                                                                                                                                                                                              |
|     \>\> smtSyncEq     |     String     |                                                                                                                                                                                                                                                               Smart sync equity  <br/>The default is "0", only applicable to copy trader.                                                                                                                                                                                                                                                               |
| \>\> spotCopyTradingEq |     String     |                                                                                                                                                                                                                                                           Spot smart sync equity.   <br/>The default is "0", only applicable to copy trader.                                                                                                                                                                                                                                                            |
|      \>\> spotBal      |     String     |                                                                                                                                                                                                                                              Spot balance. The unit is currency, e.g. BTC. [More details](https://www.okx.com/help/i-introduction-of-spot)                                                                                                                                                                                                                                              |
|     \>\> openAvgPx     |     String     |                                                                                                                                                                                                                                                Spot average cost price. The unit is USD. [More details](https://www.okx.com/help/i-introduction-of-spot)                                                                                                                                                                                                                                                |
|     \>\> accAvgPx      |     String     |                                                                                                                                                                                                                                              Spot accumulated cost price. The unit is USD. [More details](https://www.okx.com/help/i-introduction-of-spot)                                                                                                                                                                                                                                              |
|      \>\> spotUpl      |     String     |                                                                                                                                                                                                                                            Spot unrealized profit and loss. The unit is USD. [More details](https://www.okx.com/help/i-introduction-of-spot)                                                                                                                                                                                                                                            |
|   \>\> spotUplRatio    |     String     |                                                                                                                                                                                                                                                 Spot unrealized profit and loss ratio. [More details](https://www.okx.com/help/i-introduction-of-spot)                                                                                                                                                                                                                                                  |
|     \>\> totalPnl      |     String     |                                                                                                                                                                                                                                           Spot accumulated profit and loss. The unit is USD. [More details](https://www.okx.com/help/i-introduction-of-spot)                                                                                                                                                                                                                                            |
|   \>\> totalPnlRatio   |     String     |                                                                                                                                                                                                                                                 Spot accumulated profit and loss ratio. [More details](https://www.okx.com/help/i-introduction-of-spot)                                                                                                                                                                                                                                                 |
| \>\> collateralEnabled |    Boolean     |                                                                                                                                                                                                                                                `true`: Collateral enabled  <br/>`false`: Collateral disabled  <br/>Applicable to `Multi-currency margin`                                                                                                                                                                                                                                                |
"" will be returned for inapplicable fields under the current account level.   
 \- The account data is sent on event basis and regular basis.  
 \- The event push is not pushed in real-time. It is aggregated and pushed at a fixed time interval, around 50ms. For example, if multiple events occur within a fixed time interval, the system will aggregate them into a single message and push it at the end of the fixed time interval. If the data volume is too large, it may be split into multiple messages.  
 \- The regular push sends updates regardless of whether there are activities in the trading account or not.   
 \- Only currencies with non-zero balance will be pushed. Definition of non-zero balance: any value of eq, availEq, availBql parameters is not 0. If the data is too large to be sent in a single push message, it will be split into multiple messages.  
 \- For example, when subscribing to account channel without specifying ccy and there are 5 currencies are with non-zero balance, all 5 currencies data will be pushed in initial snapshot and in regular update. Subsequently when there is change in balance or equity of an token, only the incremental data of that currency will be pushed triggered by this change.

---

### Positions channel ###

Retrieve position information. Initial snapshot will be pushed according to subscription granularity. Data will be pushed when triggered by events such as placing/canceling order, and will also be pushed in regular interval according to subscription granularity.  

Concurrent connection to this channel will be restricted by the following rules: [WebSocket connection count limit](/docs-v5/en/#overview-websocket-connection-count-limit).

#### URL Path ####

/ws/v5/private (required login)

>
>
> Request Example : single
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "positions",
      "instType": "FUTURES",
      "instFamily": "BTC-USD"
    }
  ]
}

```

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
      "channel": "positions",
      "instType": "ANY",
      "extraParams": "
        {
          \"updateInterval\": \"0\"
        }
      "
    }
  ]
}

```

#### Request Parameters ####

|     Parameter     |      Type      |Required|                                                                                                                                                                                                                                      Description                                                                                                                                                                                                                                       |
|-------------------|----------------|--------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|        op         |     String     |  Yes   |                                                                                                                                                                                                                    Operation  <br/>`subscribe`  <br/>`unsubscribe`                                                                                                                                                                                                                     |
|       args        |Array of objects|  Yes   |                                                                                                                                                                                                                              List of subscribed channels                                                                                                                                                                                                                               |
|    \> channel     |     String     |  Yes   |                                                                                                                                                                                                                             Channel name  <br/>`positions`                                                                                                                                                                                                                             |
|    \> instType    |     String     |  Yes   |                                                                                                                                                                                                Instrument type  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`   <br/>`ANY`                                                                                                                                                                                                 |
|   \> instFamily   |     String     |   No   |                                                                                                                                                                                                            Instrument family  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`                                                                                                                                                                                                             |
|     \> instId     |     String     |   No   |                                                                                                                                                                                                   Instrument ID  <br/>If instId and instFamily are both passed, instId will be used                                                                                                                                                                                                    |
|  \> extraParams   |     String     |   No   |                                                                                                                                                                                                                                Additional configuration                                                                                                                                                                                                                                |
|\>\> updateInterval|      int       |   No   |`0`: only push due to positions events   <br/>`2000, 3000, 4000`: push by events and regularly according to the time interval setting (ms)   <br/><br/>The data will be pushed both by events and around per 5 seconds regularly if this field is omitted or set to other values than the valid values above.   <br/><br/>The following format should be strictly followed when using this field.   <br/>"extraParams": "   <br/>{   <br/>\\"updateInterval\\": \\"0\\"   <br/>}  <br/>"|

>
>
> Successful Response Example : single
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "positions",
    "instType": "FUTURES",
    "instFamily": "BTC-USD"
  },
  "connId": "a4d3ae55"
}

```

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "positions",
    "instType": "ANY"
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
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"positions\", \"instType\" : \"FUTURES\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|  Parameter  | Type |Required|                                     Description                                      |
|-------------|------|--------|--------------------------------------------------------------------------------------|
|    event    |String|  Yes   |              Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`               |
|     arg     |Object|   No   |                                  Subscribed channel                                  |
| \> channel  |String|  Yes   |                                     Channel name                                     |
| \> instType |String|  Yes   |Instrument type  <br/>`MARGIN`  <br/>`FUTURES`  <br/>`SWAP`  <br/>`OPTION`  <br/>`ANY`|
|\> instFamily|String|   No   |                                  Instrument family                                   |
|  \> instId  |String|   No   |                                    Instrument ID                                     |
|    code     |String|   No   |                                      Error code                                      |
|     msg     |String|   No   |                                    Error message                                     |
|   connId    |String|  Yes   |                               WebSocket connection ID                                |

>
>
> Push Data Example: single
>
>

```
{
  "arg":{
      "channel":"positions",
      "uid": "77982378738415879",
      "instType":"FUTURES"
  },
  "eventType": "snapshot",
  "curPage": 1,
  "lastPage": true,
  "data":[
    {
      "adl":"1",
      "availPos":"1",
      "avgPx":"2566.31",
      "cTime":"1619507758793",
      "ccy":"ETH",
      "deltaBS":"",
      "deltaPA":"",
      "gammaBS":"",
      "gammaPA":"",
      "imr":"",
      "instId":"ETH-USD-210430",
      "instType":"FUTURES",
      "interest":"0",
      "idxPx":"2566.13",
      "last":"2566.22",
      "lever":"10",
      "liab":"",
      "liabCcy":"",
      "liqPx":"2352.8496681818233",
      "markPx":"2353.849",
      "margin":"0.0003896645377994",
      "mgnMode":"isolated",
      "mgnRatio":"11.731726509588816",
      "mmr":"0.0000311811092368",
      "notionalUsd":"2276.2546609009605",
      "optVal":"",
      "pTime":"1619507761462",
      "pendingCloseOrdLiabVal":"0.1",
      "pos":"1",
      "baseBorrowed": "",
      "baseInterest": "",
      "quoteBorrowed": "",
      "quoteInterest": "",
      "posCcy":"",
      "posId":"307173036051017730",
      "posSide":"long",
      "spotInUseAmt": "",
      "clSpotInUseAmt": "",
      "maxSpotInUseAmt": "",
      "bizRefId": "",
      "bizRefType": "",
      "spotInUseCcy": "",
      "thetaBS":"",
      "thetaPA":"",
      "tradeId":"109844",
      "uTime":"1619507761462",
      "upl":"-0.0000009932766034",
      "uplLastPx":"-0.0000009932766034",
      "uplRatio":"-0.0025490556801078",
      "uplRatioLastPx":"-0.0025490556801078",
      "vegaBS":"",
      "vegaPA":"",
      "realizedPnl":"0.001",
      "pnl":"0.0011",
      "fee":"-0.0001",
      "fundingFee":"0",
      "liqPenalty":"0",
      "nonSettleAvgPx":"",
      "settledPnl":"",
      "closeOrderAlgo":[
          {
              "algoId":"123",
              "slTriggerPx":"123",
              "slTriggerPxType":"mark",
              "tpTriggerPx":"123",
              "tpTriggerPxType":"mark",
              "closeFraction":"0.6"
          },
          {
              "algoId":"123",
              "slTriggerPx":"123",
              "slTriggerPxType":"mark",
              "tpTriggerPx":"123",
              "tpTriggerPxType":"mark",
              "closeFraction":"0.4"
          }
      ]
    }
  ]
}

```

>
>
> Push Data Example
>
>

```
{
    "arg": {
        "channel": "positions",
        "uid": "77982378738415879",
        "instType": "ANY"
    },
  "eventType": "snapshot",
    "curPage": 1,
    "lastPage": true,
    "data": [
    {
      "adl":"1",
      "availPos":"1",
      "avgPx":"2566.31",
      "cTime":"1619507758793",
      "ccy":"ETH",
      "deltaBS":"",
      "deltaPA":"",
      "gammaBS":"",
      "gammaPA":"",
      "imr":"",
      "instId":"ETH-USD-210430",
      "instType":"FUTURES",
      "interest":"0",
      "idxPx":"2566.13",
      "last":"2566.22",
      "usdPx":"",
      "bePx":"2353.949",
      "lever":"10",
      "liab":"",
      "liabCcy":"",
      "liqPx":"2352.8496681818233",
      "markPx":"2353.849",
      "margin":"0.0003896645377994",
      "mgnMode":"isolated",
      "mgnRatio":"11.731726509588816",
      "mmr":"0.0000311811092368",
      "notionalUsd":"2276.2546609009605",
      "optVal":"",
      "pTime":"1619507761462",
      "pendingCloseOrdLiabVal":"0.1",
      "pos":"1",
      "baseBorrowed": "",
      "baseInterest": "",
      "quoteBorrowed": "",
      "quoteInterest": "",
      "posCcy":"",
      "posId":"307173036051017730",
      "posSide":"long",
      "spotInUseAmt": "",
      "clSpotInUseAmt": "",
      "maxSpotInUseAmt": "",
      "spotInUseCcy": "",
      "bizRefId": "",
      "bizRefType": "",
      "thetaBS":"",
      "thetaPA":"",
      "tradeId":"109844",
      "uTime":"1619507761462",
      "upl":"-0.0000009932766034",
      "uplLastPx":"-0.0000009932766034",
      "uplRatio":"-0.0025490556801078",
      "uplRatioLastPx":"-0.0025490556801078",
      "vegaBS":"",
      "vegaPA":"",
      "realizedPnl":"0.001",
      "pnl":"0.0011",
      "fee":"-0.0001",
      "fundingFee":"0",
      "liqPenalty":"0",
      "nonSettleAvgPx":"",
      "settledPnl":"",
      "closeOrderAlgo":[
          {
              "algoId":"123",
              "slTriggerPx":"123",
              "slTriggerPxType":"mark",
              "tpTriggerPx":"123",
              "tpTriggerPxType":"mark",
              "closeFraction":"0.6"
          },
          {
              "algoId":"123",
              "slTriggerPx":"123",
              "slTriggerPxType":"mark",
              "tpTriggerPx":"123",
              "tpTriggerPxType":"mark",
              "closeFraction":"0.4"
          }
      ]
    }, {
      "adl":"1",
      "availPos":"1",
      "avgPx":"2566.31",
      "cTime":"1619507758793",
      "ccy":"ETH",
      "deltaBS":"",
      "deltaPA":"",
      "gammaBS":"",
      "gammaPA":"",
      "imr":"",
      "instId":"ETH-USD-SWAP",
      "instType":"SWAP",
      "interest":"0",
      "idxPx":"2566.13",
      "last":"2566.22",
      "usdPx":"",
      "bePx":"2353.949",
      "lever":"10",
      "liab":"",
      "liabCcy":"",
      "liqPx":"2352.8496681818233",
      "markPx":"2353.849",
      "margin":"0.0003896645377994",
      "mgnMode":"isolated",
      "mgnRatio":"11.731726509588816",
      "mmr":"0.0000311811092368",
      "notionalUsd":"2276.2546609009605",
      "optVal":"",
      "pTime":"1619507761462",
      "pendingCloseOrdLiabVal":"0.1",
      "pos":"1",
      "baseBorrowed": "",
      "baseInterest": "",
      "quoteBorrowed": "",
      "quoteInterest": "",
      "posCcy":"",
      "posId":"307173036051017730",
      "posSide":"long",
      "spotInUseAmt": "",
      "clSpotInUseAmt": "",
      "maxSpotInUseAmt": "",
      "spotInUseCcy": "",
      "bizRefId": "",
      "bizRefType": "",
      "thetaBS":"",
      "thetaPA":"",
      "tradeId":"109844",
      "uTime":"1619507761462",
      "upl":"-0.0000009932766034",
      "uplLastPx":"-0.0000009932766034",
      "uplRatio":"-0.0025490556801078",
      "uplRatioLastPx":"-0.0025490556801078",
      "vegaBS":"",
      "vegaPA":"",
      "realizedPnl":"0.001",
      "pnl":"0.0011",
      "fee":"-0.0001",
      "fundingFee":"0",
      "liqPenalty":"0",
      "nonSettleAvgPx":"",
      "settledPnl":"",
      "closeOrderAlgo":[
          {
              "algoId":"123",
              "slTriggerPx":"123",
              "slTriggerPxType":"mark",
              "tpTriggerPx":"123",
              "tpTriggerPxType":"mark",
              "closeFraction":"0.6"
          },
          {
              "algoId":"123",
              "slTriggerPx":"123",
              "slTriggerPxType":"mark",
              "tpTriggerPx":"123",
              "tpTriggerPxType":"mark",
              "closeFraction":"0.4"
          }
      ]
    }
  ]
}

```

#### Push data parameters ####

|        Parameter        |      Type      |                                                                                                                                                                                Description                                                                                                                                                                                |
|-------------------------|----------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|           arg           |     Object     |                                                                                                                                                                      Successfully subscribed channel                                                                                                                                                                      |
|       \> channel        |     String     |                                                                                                                                                                               Channel name                                                                                                                                                                                |
|         \> uid          |     String     |                                                                                                                                                                              User Identifier                                                                                                                                                                              |
|       \> instType       |     String     |                                                                                                                                                                              Instrument type                                                                                                                                                                              |
|      \> instFamily      |     String     |                                                                                                                                                                             Instrument family                                                                                                                                                                             |
|        \> instId        |     String     |                                                                                                                                                                               Instrument ID                                                                                                                                                                               |
|        eventType        |     String     |                                                                                                                             Event type:   <br/>`snapshot`: Initial and regular snapshot push   <br/>`event_update`: Event-driven update push                                                                                                                              |
|         curPage         |    Integer     |                                                                                                                                 Current page number.   <br/>Only applicable for `snapshot` events. Not included in `event_update` events.                                                                                                                                 |
|        lastPage         |    Boolean     |                                                                                                        Whether this is the last page of pagination:  <br/>`true`  <br/>`false`  <br/>Only applicable for `snapshot` events. Not included in `event_update` events.                                                                                                        |
|          data           |Array of objects|                                                                                                                                                                              Subscribed data                                                                                                                                                                              |
|       \> instType       |     String     |                                                                                                                                                                              Instrument type                                                                                                                                                                              |
|       \> mgnMode        |     String     |                                                                                                                                                                      Margin mode, `cross` `isolated`                                                                                                                                                                      |
|        \> posId         |     String     |                                                                                                                                                                                Position ID                                                                                                                                                                                |
|       \> posSide        |     String     |                                              Position side  <br/>`long`   <br/>`short`   <br/>`net` (`FUTURES`/`SWAP`/`OPTION`: positive `pos` means long position and negative `pos` means short position. `MARGIN`: `posCcy` being base currency means long position, `posCcy` being quote currency means short position.)                                              |
|         \> pos          |     String     |                                                                                                      Quantity of positions. In the isolated margin mode, when doing manual transfers, a position with pos of `0` will be generated after the deposit is transferred                                                                                                       |
|       \> baseBal        |     String     |                                                                                                                                           ~~Base currency balance, only applicable to `MARGIN`（Quick Margin Mode）~~(Deprecated)                                                                                                                                           |
|       \> quoteBal       |     String     |                                                                                                                                          ~~Quote currency balance, only applicable to `MARGIN`（Quick Margin Mode）~~(Deprecated)                                                                                                                                           |
|     \> baseBorrowed     |     String     |                                                                                                                                    ~~Base currency amount already borrowed, only applicable to MARGIN(Quick Margin Mode）~~(Deprecated)                                                                                                                                    |
|     \> baseInterest     |     String     |                                                                                                                          ~~Base Interest, undeducted interest that has been incurred, only applicable to MARGIN(Quick Margin Mode）~~(Deprecated)                                                                                                                          |
|    \> quoteBorrowed     |     String     |                                                                                                                                   ~~Quote currency amount already borrowed, only applicable to MARGIN(Quick Margin Mode）~~(Deprecated)                                                                                                                                    |
|    \> quoteInterest     |     String     |                                                                                                                         ~~Quote Interest, undeducted interest that has been incurred, only applicable to MARGIN(Quick Margin Mode）~~(Deprecated)                                                                                                                          |
|        \> posCcy        |     String     |                                                                                                                                                         Position currency, only applicable to `MARGIN` positions                                                                                                                                                          |
|       \> availPos       |     String     |Position that can be closed   <br/>Only applicable to `MARGIN` and `OPTION`.   <br/>For `Margin` position, the rest of sz will be `SPOT` trading after the liability is repaid while closing the position. Please get the available reduce-only amount from "Get maximum available tradable amount" if you want to reduce the amount of `SPOT` trading as much as possible.|
|        \> avgPx         |     String     |                                                                                                                                                                            Average open price                                                                                                                                                                             |
|         \> upl          |     String     |                                                                                                                                                           Unrealized profit and loss calculated by mark price.                                                                                                                                                            |
|       \> uplRatio       |     String     |                                                                                                                                                        Unrealized profit and loss ratio calculated by mark price.                                                                                                                                                         |
|      \> uplLastPx       |     String     |                                                                                                                                     Unrealized profit and loss calculated by last price. Main usage is showing, actual value is upl.                                                                                                                                      |
|    \> uplRatioLastPx    |     String     |                                                                                                                                                        Unrealized profit and loss ratio calculated by last price.                                                                                                                                                         |
|        \> instId        |     String     |                                                                                                                                                                    Instrument ID, e.g. `BTC-USDT-SWAP`                                                                                                                                                                    |
|        \> lever         |     String     |                                                                                                                                                                Leverage, not applicable to `OPTION` seller                                                                                                                                                                |
|        \> liqPx         |     String     |                                                                                                                                                       Estimated liquidation price   <br/>Not applicable to `OPTION`                                                                                                                                                       |
|        \> markPx        |     String     |                                                                                                                                                                             Latest Mark price                                                                                                                                                                             |
|         \> imr          |     String     |                                                                                                                                                          Initial margin requirement, only applicable to `cross`                                                                                                                                                           |
|        \> margin        |     String     |                                                                                                                                                 Margin, can be added or reduced. Only applicable to `isolated` `Margin`.                                                                                                                                                  |
|       \> mgnRatio       |     String     |                                                                                                                                                                               Margin ratio                                                                                                                                                                                |
|         \> mmr          |     String     |                                                                                                                                                                      Maintenance margin requirement                                                                                                                                                                       |
|         \> liab         |     String     |                                                                                                                                                                 Liabilities, only applicable to `MARGIN`.                                                                                                                                                                 |
|       \> liabCcy        |     String     |                                                                                                                                                            Liabilities currency, only applicable to `MARGIN`.                                                                                                                                                             |
|       \> interest       |     String     |                                                                                                                                                                Interest accrued that has not been settled.                                                                                                                                                                |
|       \> tradeId        |     String     |                                                                                                                                                                               Last trade ID                                                                                                                                                                               |
|     \> notionalUsd      |     String     |                                                                                                                                                                   Notional value of positions in `USD`                                                                                                                                                                    |
|        \> optVal        |     String     |                                                                                                                                                                Option Value, only applicable to `OPTION`.                                                                                                                                                                 |
|\> pendingCloseOrdLiabVal|     String     |                                                                                                                                                         The amount of close orders of isolated margin liability.                                                                                                                                                          |
|         \> adl          |     String     |                                                                                                                      Auto decrease line, signal area  <br/>Divided into 5 levels, from 1 to 5, the smaller the number, the weaker the adl intensity.                                                                                                                      |
|       \> bizRefId       |     String     |                                                                                                                                                              External business id, e.g. experience coupon id                                                                                                                                                              |
|      \> bizRefType      |     String     |                                                                                                                                                                          External business type                                                                                                                                                                           |
|         \> ccy          |     String     |                                                                                                                                                                         Currency used for margin                                                                                                                                                                          |
|         \> last         |     String     |                                                                                                                                                                            Latest traded price                                                                                                                                                                            |
|        \> idxPx         |     String     |                                                                                                                                                                       Latest underlying index price                                                                                                                                                                       |
|        \> usdPx         |     String     |                                                                                                                                                 Latest USD price of the `ccy` on the market, only applicable to `OPTION`                                                                                                                                                  |
|         \> bePx         |     String     |                                                                                                                                                                              Breakeven price                                                                                                                                                                              |
|       \> deltaBS        |     String     |                                                                                                                                                    delta: Black-Scholes Greeks in dollars, only applicable to `OPTION`                                                                                                                                                    |
|       \> deltaPA        |     String     |                                                                                                                                                            delta: Greeks in coins, only applicable to `OPTION`                                                                                                                                                            |
|       \> gammaBS        |     String     |                                                                                                                                                    gamma: Black-Scholes Greeks in dollars, only applicable to `OPTION`                                                                                                                                                    |
|       \> gammaPA        |     String     |                                                                                                                                                            gamma: Greeks in coins, only applicable to `OPTION`                                                                                                                                                            |
|       \> thetaBS        |     String     |                                                                                                                                                    theta: Black-Scholes Greeks in dollars, only applicable to `OPTION`                                                                                                                                                    |
|       \> thetaPA        |     String     |                                                                                                                                                            theta: Greeks in coins, only applicable to `OPTION`                                                                                                                                                            |
|        \> vegaBS        |     String     |                                                                                                                                                    vega: Black-Scholes Greeks in dollars, only applicable to `OPTION`                                                                                                                                                     |
|        \> vegaPA        |     String     |                                                                                                                                                            vega: Greeks in coins, only applicable to `OPTION`                                                                                                                                                             |
|     \> spotInUseAmt     |     String     |                                                                                                                                                         Spot in use amount  <br/>Applicable to `Portfolio margin`                                                                                                                                                         |
|     \> spotInUseCcy     |     String     |                                                                                                                                                    Spot in use unit, e.g. `BTC`  <br/>Applicable to `Portfolio margin`                                                                                                                                                    |
|    \> clSpotInUseAmt    |     String     |                                                                                                                                                User-defined spot risk offset amount  <br/>Applicable to `Portfolio margin`                                                                                                                                                |
|   \> maxSpotInUseAmt    |     String     |                                                                                                                                                Max possible spot risk offset amount  <br/>Applicable to `Portfolio margin`                                                                                                                                                |
|     \> realizedPnl      |     String     |                                                                                                                  Realized profit and loss  <br/>Only applicable to `FUTURES`/`SWAP`/`OPTION`  <br/> realizedPnl=pnl+fee+fundingFee+liqPenalty+settledPnl                                                                                                                  |
|         \> pnl          |     String     |                                                                                                                                                                    Accumulated pnl of closing order(s)                                                                                                                                                                    |
|         \> fee          |     String     |                                                                                                                   Accumulated fee  <br/>Negative number represents the user transaction fee charged by the platform.Positive number represents rebate.                                                                                                                    |
|      \> fundingFee      |     String     |                                                                                                                                                                          Accumulated funding fee                                                                                                                                                                          |
|      \> liqPenalty      |     String     |                                                                                                                                                  Accumulated liquidation penalty. It is negative when there is a value.                                                                                                                                                   |
|    \> closeOrderAlgo    |Array of object |                                                                                                            Close position algo orders attached to the position. This array will have values only after you request "Place algo order" with `closeFraction`=1.                                                                                                             |
|       \>\> algoId       |     String     |                                                                                                                                                                                  Algo ID                                                                                                                                                                                  |
|    \>\> slTriggerPx     |     String     |                                                                                                                                                                         Stop-loss trigger price.                                                                                                                                                                          |
|  \>\> slTriggerPxType   |     String     |                                                                                                                                Stop-loss trigger price type.   <br/>`last`: last price  <br/>`index`: index price  <br/>`mark`: mark price                                                                                                                                |
|    \>\> tpTriggerPx     |     String     |                                                                                                                                                                        Take-profit trigger price.                                                                                                                                                                         |
|  \>\> tpTriggerPxType   |     String     |                                                                                                                               Take-profit trigger price type.   <br/>`last`: last price  <br/>`index`: index price  <br/>`mark`: mark price                                                                                                                               |
|   \>\> closeFraction    |     String     |                                                                                                                                                    Fraction of position to be closed when the algo order is triggered.                                                                                                                                                    |
|        \> cTime         |     String     |                                                                                                                                                Creation time, Unix timestamp format in milliseconds, e.g. `1597026383085`.                                                                                                                                                |
|        \> uTime         |     String     |                                                                                                                                      Latest time position was adjusted, Unix timestamp format in milliseconds, e.g. `1597026383085`.                                                                                                                                      |
|        \> pTime         |     String     |                                                                                                                                     Push time of positions information, Unix timestamp format in milliseconds, e.g. `1597026383085`.                                                                                                                                      |
|    \> nonSettleAvgPx    |     String     |                                                                                            Non-Settlement entry price  <br/>The non-settlement entry price only reflects the average price at which the position is opened or increased.  <br/>Applicable to `FUTURES` `cross`                                                                                            |
|      \> settledPnl      |     String     |                                                                                                                                      Accumulated settled P&L (calculated by settlement price)  <br/>Applicable to `FUTURES` `cross`                                                                                                                                       |

 \- The position data is sent on event basis and regular basis  
 \- The event push is not pushed in real-time. It is aggregated and pushed at a fixed time interval, around 50ms. For example, if multiple events occur within a fixed time interval, the system will aggregate them into a single message and push it at the end of the fixed time interval. If the data volume is too large, it may be split into multiple messages.  
 \- The regular push sends updates regardless of whether there are position activities or not.  
 \- If an event push and a regular push happen at the same time, the system will send the event push first, followed by the regular push. As for portfolio margin account, the IMR and MMR of the position are calculated in risk unit granularity, thus their values of the same risk unit cross positions are the same. In the position-by-position trading setting, it is an autonomous transfer mode. After the margin is transferred, positions with a position of 0 will be pushed   
 \- Only position with non-zero position quantity will be pushed. Definition of non-zero quantity: value of pos parameter is not 0. If the data is too large to be sent in a single push message, it will be split into multiple messages.  
 \- For example, when subscribing to positions channel specifying an underlying and there are 20 positions are with non-zero quantity, all 20 positions data will be pushed in initial snapshot and in regular push. Subsequently when there is change in pos of a position, only the data of that position will be pushed triggered by this change.

---

### Balance and position channel ###

Retrieve account balance and position information. Data will be pushed when triggered by events such as filled order, funding transfer.  
This channel applies to getting the account cash balance and the change of position asset ASAP.   
Concurrent connection to this channel will be restricted by the following rules: [WebSocket connection count limit](/docs-v5/en/#overview-websocket-connection-count-limit).

#### URL Path ####

/ws/v5/private (required login)

>
>
> Request Example
>
>

```
{
    "op": "subscribe",
    "args": [{
        "channel": "balance_and_position"
    }]
}

```

#### Request Parameters ####

|Parameter |      Type      |Required|                  Description                  |
|----------|----------------|--------|-----------------------------------------------|
|    op    |     String     |  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`|
|   args   |Array of objects|  Yes   |          List of subscribed channels          |
|\> channel|     String     |  Yes   |   Channel name  <br/>`balance_and_position`   |

>
>
> Response Example
>
>

```
{
    "event": "subscribe",
    "arg": {
        "channel": "balance_and_position"
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
    "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"balance_and_position\"}]}",
    "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|                         Description                         |
|----------|------|--------|-------------------------------------------------------------|
|  event   |String|  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|   arg    |Object|   No   |                 List of subscribed channels                 |
|\> channel|String|  Yes   |          Channel name  <br/>`balance_and_position`          |
|   code   |String|   No   |                         Error code                          |
|   msg    |String|   No   |                        Error message                        |
|  connId  |String|  Yes   |                   WebSocket connection ID                   |

>
>
> Push Data Example
>
>

```
{
    "arg": {
        "channel": "balance_and_position",
        "uid": "77982378738415879"
    },
    "data": [{
        "pTime": "1597026383085",
        "eventType": "snapshot",
        "balData": [{
            "ccy": "BTC",
            "cashBal": "1",
            "uTime": "1597026383085"
        }],
        "posData": [{
            "posId": "1111111111",
            "tradeId": "2",
            "instId": "BTC-USD-191018",
            "instType": "FUTURES",
            "mgnMode": "cross",
            "posSide": "long",
            "pos": "10",
            "ccy": "BTC",
            "posCcy": "",
            "avgPx": "3320",
            "nonSettleAvgPx": "",
            "settledPnl": "",
            "uTime": "1597026383085"
        }],
        "trades": [{
            "instId": "BTC-USD-191018",
            "tradeId": "2",
        }]
    }]
}

```

#### Push data parameters ####

|   **Parameter**   |    **Type**    |                                                                                                                         **Description**                                                                                                                         |
|-------------------|----------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|        arg        |     Object     |                                                                                                                     Channel to subscribe to                                                                                                                     |
|    \> channel     |     String     |                                                                                                                          Channel name                                                                                                                           |
|      \> uid       |     String     |                                                                                                                         User Identifier                                                                                                                         |
|       data        |Array of objects|                                                                                                                         Subscribed data                                                                                                                         |
|     \> pTime      |     String     |                                                                         Push time of both balance and position information, millisecond format of Unix timestamp, e.g. `1597026383085`                                                                          |
|   \> eventType    |     String     |Event Type  <br/>`snapshot`  <br/>`delivered`  <br/>`exercised`  <br/>`transferred`  <br/>`filled`  <br/>`liquidation`  <br/>`claw_back`  <br/>`adl`  <br/>`funding_fee`  <br/>`adjust_margin`  <br/>`set_leverage`  <br/>`interest_deduction`  <br/>`settlement`|
|    \> balData     |Array of objects|                                                                                                                          Balance data                                                                                                                           |
|     \>\> ccy      |     String     |                                                                                                                            Currency                                                                                                                             |
|   \>\> cashBal    |     String     |                                                                                                                          Cash Balance                                                                                                                           |
|    \>\> uTime     |     String     |                                                                                            Update time, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                                             |
|    \> posData     |Array of objects|                                                                                                                          Position data                                                                                                                          |
|    \>\> posId     |     String     |                                                                                                                           Position ID                                                                                                                           |
|   \>\> tradeId    |     String     |                                                                                                                          Last trade ID                                                                                                                          |
|    \>\> instId    |     String     |                                                                                                               Instrument ID, e.g `BTC-USD-180213`                                                                                                               |
|   \>\> instType   |     String     |                                                                                                                         Instrument type                                                                                                                         |
|   \>\> mgnMode    |     String     |                                                                                                              Margin mode  <br/>`isolated`, `cross`                                                                                                              |
|    \>\> avgPx     |     String     |                                                                                                                       Average open price                                                                                                                        |
|     \>\> ccy      |     String     |                                                                                                                    Currency used for margin                                                                                                                     |
|   \>\> posSide    |     String     |                                                                                                           Position side  <br/>`long`, `short`, `net`                                                                                                            |
|     \>\> pos      |     String     |                                                 Quantity of positions. In the isolated margin mode, when doing manual transfers, a position with pos of `0` will be generated after the deposit is transferred                                                  |
|   \>\> baseBal    |     String     |                                                                                      ~~Base currency balance, only applicable to `MARGIN`（Quick Margin Mode）~~(Deprecated)                                                                                      |
|   \>\> quoteBal   |     String     |                                                                                     ~~Quote currency balance, only applicable to `MARGIN`（Quick Margin Mode）~~(Deprecated)                                                                                      |
|    \>\> posCcy    |     String     |                                                                                                     Position currency, only applicable to MARGIN positions.                                                                                                     |
|\>\> nonSettleAvgPx|     String     |                                       Non-Settlement entry price  <br/>The non-settlement entry price only reflects the average price at which the position is opened or increased.  <br/>Applicable to `FUTURES` `cross`                                       |
|  \>\> settledPnl  |     String     |                                                                                 Accumulated settled P&L (calculated by settlement price)  <br/>Applicable to `FUTURES` `cross`                                                                                  |
|    \>\> uTime     |     String     |                                                                                            Update time, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                                             |
|     \> trades     |Array of object |                                                                                                                        Details of trade                                                                                                                         |
|    \>\> instId    |     String     |                                                                                                                 Instrument ID, e.g. `BTC-USDT`                                                                                                                  |
|   \>\> tradeId    |     String     |                                                                                                                            Trade ID                                                                                                                             |
 Only balData will be pushed if only the account balance changes; only posData will be pushed if only the position changes.   
 \- Initial snapshot: Only either position with non-zero position quantity or cash balance with non-zero quantity will be pushed. If the data is too large to be sent in a single push message, it will be split into multiple messages.  
 \- For example, if you subscribe according to all currencies and the user has 5 currency balances that are not 0 and 20 positions, all 20 positions data and 5 currency balances data will be pushed in initial snapshot; Subsequently when there is change in pos of a position, only the data of that position will be pushed triggered by this change.

---

### Position risk warning ###

This push channel is only used as a risk warning, and is not recommended as a risk judgment for strategic trading   
In the case that the market is volatile, there may be the possibility that the position has been liquidated at the same time that this message is pushed.  
The warning is sent when a position is at risk of liquidation for isolated margin positions. The warning is sent when all the positions are at risk of liquidation for cross-margin positions.  
Concurrent connection to this channel will be restricted by the following rules: [WebSocket connection count limit](/docs-v5/en/#overview-websocket-connection-count-limit).

#### URL Path ####

/ws/v5/private (required login)

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
      "channel": "liquidation-warning",
      "instType": "ANY"
    }
  ]
}

```

#### Request Parameters ####

|  Parameter  |     Type      |Required|                                      Description                                      |
|-------------|---------------|--------|---------------------------------------------------------------------------------------|
|     op      |    String     |  Yes   |                    Operation  <br/>`subscribe`  <br/>`unsubscribe`                    |
|    args     |Array of object|  Yes   |                              List of subscribed channels                              |
| \> channel  |    String     |  Yes   |                       Channel name  <br/>`liquidation-warning`                        |
| \> instType |    String     |  Yes   |Instrument type  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`   <br/>`ANY`|
|\> instFamily|    String     |   No   |            Instrument family  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`            |
|  \> instId  |    String     |   No   |                                     Instrument ID                                     |

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "liquidation-warning",
    "instType": "ANY"
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
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"liquidation-warning\", \"instType\" : \"FUTURES\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|  Parameter  | Type |Required|                                      Description                                      |
|-------------|------|--------|---------------------------------------------------------------------------------------|
|    event    |String|  Yes   |               Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`               |
|     arg     |Object|   No   |                                  Subscribed channel                                   |
| \> channel  |String|  Yes   |                                     Channel name                                      |
| \> instType |String|  Yes   |Instrument type  <br/>`OPTION`  <br/>`FUTURES`  <br/>`SWAP`  <br/>`MARGIN`   <br/>`ANY`|
|\> instFamily|String|   No   |                                   Instrument family                                   |
|  \> instId  |String|   No   |                                     Instrument ID                                     |
|    code     |String|   No   |                                      Error code                                       |
|     msg     |String|   No   |                                     Error message                                     |
|   connId    |String|  Yes   |                                WebSocket connection ID                                |

>
>
> Push Data Example
>
>

```
{
    "arg":{
        "channel":"liquidation-warning",
        "uid": "77982378738415879",
        "instType":"FUTURES"
    },
    "data":[
        {
            "cTime":"1619507758793",
            "ccy":"ETH",
            "instId":"ETH-USD-210430",
            "instType":"FUTURES",
            "lever":"10",
            "markPx":"2353.849",
            "mgnMode":"isolated",
            "mgnRatio":"11.731726509588816",
            "pTime":"1619507761462",
            "pos":"1",
            "posCcy":"",
            "posId":"307173036051017730",
            "posSide":"long",
            "uTime":"1619507761462",
        }
    ]
}

```

#### Push data parameters ####

|  Parameter  |     Type      |                                                                                                                                  Description                                                                                                                                  |
|-------------|---------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     arg     |    Object     |                                                                                                                        Successfully subscribed channel                                                                                                                        |
| \> channel  |    String     |                                                                                                                                 Channel name                                                                                                                                  |
|   \> uid    |    String     |                                                                                                                                User Identifier                                                                                                                                |
| \> instType |    String     |                                                                                                                                Instrument type                                                                                                                                |
|\> instFamily|    String     |                                                                                                                               Instrument family                                                                                                                               |
|  \> instId  |    String     |                                                                                                                                 Instrument ID                                                                                                                                 |
|    data     |Array of object|                                                                                                                                Subscribed data                                                                                                                                |
| \> instType |    String     |                                                                                                                                Instrument type                                                                                                                                |
| \> mgnMode  |    String     |                                                                                                                        Margin mode, `cross` `isolated`                                                                                                                        |
|  \> posId   |    String     |                                                                                                                                  Position ID                                                                                                                                  |
| \> posSide  |    String     |Position side  <br/>`long`   <br/>`short`   <br/>`net` (`FUTURES`/`SWAP`/`OPTION`: positive `pos` means long position and negative `pos` means short position. `MARGIN`: `posCcy` being base currency means long position, `posCcy` being quote currency means short position.)|
|   \> pos    |    String     |                                                                                                                             Quantity of positions                                                                                                                             |
|  \> posCcy  |    String     |                                                                                                           Position currency, only applicable to `MARGIN` positions                                                                                                            |
|  \> instId  |    String     |                                                                                                                      Instrument ID, e.g. `BTC-USDT-SWAP`                                                                                                                      |
|  \> lever   |    String     |                                                                                                                  Leverage, not applicable to `OPTION` seller                                                                                                                  |
|  \> markPx  |    String     |                                                                                                                                  Mark price                                                                                                                                   |
| \> mgnRatio |    String     |                                                                                                                                 Margin ratio                                                                                                                                  |
|   \> ccy    |    String     |                                                                                                                           Currency used for margin                                                                                                                            |
|  \> cTime   |    String     |                                                                                                  Creation time, Unix timestamp format in milliseconds, e.g. `1597026383085`.                                                                                                  |
|  \> uTime   |    String     |                                                                                        Latest time position was adjusted, Unix timestamp format in milliseconds, e.g. `1597026383085`.                                                                                        |
|  \> pTime   |    String     |                                                                                       Push time of positions information, Unix timestamp format in milliseconds, e.g. `1597026383085`.                                                                                        |
 Trigger push logic: the trigger logic of the liquidation warning and the liquidation message is the same

---

### Account greeks channel ###

Retrieve account greeks information. Data will be pushed when triggered by events such as increase/decrease positions or cash balance in account, and will also be pushed in regular interval according to subscription granularity.  
Concurrent connection to this channel will be restricted by the following rules: [WebSocket connection count limit](/docs-v5/en/#overview-websocket-connection-count-limit).

#### URL Path ####

/ws/v5/private (required login)

>
>
> Request Example : single
>
>

```
{
    "op": "subscribe",
    "args": [{
        "channel": "account-greeks",
        "ccy": "BTC"
    }]
}

```

>
>
> Request Example
>
>

```
{
    "op": "subscribe",
    "args": [{
        "channel": "account-greeks"
    }]
}

```

#### Request Parameters ####

|Parameter |     Type      |Required|                                                                                                                                 Description                                                                                                                                 |
|----------|---------------|--------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    op    |    String     |  Yes   |                                                                                                                  Operation  <br/>`subscribe` `unsubscribe`                                                                                                                  |
|   args   |Array of object|  Yes   |                                                                                                                         List of subscribed channels                                                                                                                         |
|\> channel|    String     |  Yes   |                                                                                                                     Channel name  <br/>`account-greeks`                                                                                                                     |
|  \> ccy  |    String     |   No   |Settlement currency  <br/>When the user specifies a settlement currency, event push will only be triggered when the position of the same settlement currency changes. For example, when ccy=BTC, if the position of `BTC-USDT-SWAP` changes, no event push will be triggered.|

>
>
> Successful Response Example : single
>
>

```
{
    "event": "subscribe",
    "arg": {
        "channel": "account-greeks",
        "ccy": "BTC"
    },
  "connId": "a4d3ae55"
}

```

>
>
> Successful Response Example
>
>

```
{
    "event": "subscribe",
    "arg": {
        "channel": "account-greeks"
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
    "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"account-greeks\", \"ccy\" : \"BTC\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|                         Description                         |
|----------|------|--------|-------------------------------------------------------------|
|  event   |String|  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|   arg    |Object|   No   |                     Subscribed channel                      |
|\> channel|String|  Yes   |                Channel name,`account-greeks`                |
|  \> ccy  |String|   No   |                     Settlement currency                     |
|   code   |String|   No   |                         Error code                          |
|   msg    |String|   No   |                        Error message                        |
|  connId  |String|  Yes   |                   WebSocket connection ID                   |

>
>
> Push Data Example: single
>
>

```
{
    "arg": {
        "channel": "account-greeks",
        "ccy": "BTC",
        "uid": "614488474791936"
    },
    "data": [
        {
            "ccy": "BTC",
            "deltaBS": "1.1246665401944310",
            "deltaPA": "-0.0074076183688949",
            "gammaBS": "0.0000000000000000",
            "gammaPA": "0.0148152367377899",
            "thetaBS": "2.0356991946421226",
            "thetaPA": "-0.0000000200174309",
            "ts": "1729179082006",
            "vegaBS": "0.0000000000000000",
            "vegaPA": "0.0000000000000000"
        }
    ]
}

```

>
>
> Push Data Example
>
>

```
{
    "arg": {
        "channel": "account-greeks",
        "uid": "614488474791936"
    },
    "data": [
        {
            "ccy": "BTC",
            "deltaBS": "1.1246665403011684",
            "deltaPA": "-0.0074021163991037",
            "gammaBS": "0.0000000000000000",
            "gammaPA": "0.0148042327982075",
            "thetaBS": "2.1342098201092528",
            "thetaPA": "-0.0000000200876441",
            "ts": "1729179001692",
            "vegaBS": "0.0000000000000000",
            "vegaPA": "0.0000000000000000"
        },
        {
            "ccy": "ETH",
            "deltaBS": "0.3810670161698570",
            "deltaPA": "-0.0688347042402955",
            "gammaBS": "-0.0000000000230396",
            "gammaPA": "0.1376693483440320",
            "thetaBS": "0.3314776517141782",
            "thetaPA": "0.0000000001316008",
            "ts": "1729179001692",
            "vegaBS": "-0.0000000045069794",
            "vegaPA": "-0.0000000000017267"
        }
    ]
}

```

#### Push data parameters ####

|**Parameters**|   **Types**    |                                   **Description**                                    |
|--------------|----------------|--------------------------------------------------------------------------------------|
|     arg      |     Object     |                           Successfully subscribed channel                            |
|  \> channel  |     String     |                                     Channel name                                     |
|    \> uid    |     String     |                                   User Identifier                                    |
|     data     |Array of objects|                                   Subscribed data                                    |
|  \> deltaBS  |     String     |                        delta: Black-Scholes Greeks in dollars                        |
|  \> deltaPA  |     String     |                                delta: Greeks in coins                                |
|  \> gammaBS  |     String     |       gamma: Black-Scholes Greeks in dollars, only applicable to OPTION cross        |
|  \> gammaPA  |     String     |               gamma: Greeks in coins, only applicable to OPTION cross                |
|  \> thetaBS  |     String     |       theta: Black-Scholes Greeks in dollars, only applicable to OPTION cross        |
|  \> thetaPA  |     String     |               theta: Greeks in coins, only applicable to OPTION cross                |
|  \> vegaBS   |     String     |        vega: Black-Scholes Greeks in dollars, only applicable to OPTION cross        |
|  \> vegaPA   |     String     |                vega: Greeks in coins, only applicable to OPTION cross                |
|    \> ccy    |     String     |                                       Currency                                       |
|    \> ts     |     String     |Push time of account greeks, Unix timestamp format in milliseconds, e.g. 1597026383085|
 The account greeks data is sent on event basis and regular basis  
 \- The event push is not pushed in real-time. It is aggregated and pushed at a fixed time interval, around 50ms. For example, if multiple events occur within a fixed time interval, the system will aggregate them into a single message and push it at the end of the fixed time interval. If the data volume is too large, it may be split into multiple messages.  
 \- When the user specifies a settlement currency in the subscribe request, event push will only be triggered when the position of the same settlement currency changes. For example, when subscribe `ccy`=BTC, if the position of `BTC-USDT-SWAP` changes, no event push will be triggered.  
 \- The regular push sends updates regardless of whether there are activities or not.   
 \- Only currencies in the account will be pushed. If the data is too large to be sent in a single push message, it will be split into multiple messages.  
 \- For example, when subscribing to account-greeks channel without specifying ccy and there are 5 currencies are with non-zero balance, all 5 currencies data will be pushed in initial snapshot and in regular interval. Subsequently when there is change in balance or equity of an token, only the incremental data of that currency will be pushed triggered by this change.

---

### WS / Order channel ###

Retrieve order information. Data will not be pushed when first subscribed. Data will only be pushed when there are order updates.  

Concurrent connection to this channel will be restricted by the following rules: [WebSocket connection count limit](/docs-v5/en/#overview-websocket-connection-count-limit).

#### URL Path ####

/ws/v5/private (required login)

>
>
> Request Example : single
>
>

```
{
  "op": "subscribe",
  "args": [
    {
      "channel": "orders",
      "instType": "FUTURES",
      "instId": "BTC-USD-200329"
    }
  ]
}

```

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
      "channel": "orders",
      "instType": "FUTURES",
      "instFamily": "BTC-USD"
    }
  ]
}

```

#### Request Parameters ####

|  Parameter  |      Type      |Required|                                            Description                                            |
|-------------|----------------|--------|---------------------------------------------------------------------------------------------------|
|     op      |     String     |  Yes   |                          Operation  <br/>`subscribe`  <br/>`unsubscribe`                          |
|    args     |Array of objects|  Yes   |                                    List of subscribed channels                                    |
| \> channel  |     String     |  Yes   |                                    Channel name  <br/>`orders`                                    |
| \> instType |     String     |  Yes   |Instrument type  <br/>`SPOT`  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`  <br/>`ANY`|
|\> instFamily|     String     |   No   |                  Instrument family  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`                  |
|  \> instId  |     String     |   No   |                                           Instrument ID                                           |

>
>
> Successful Response Example : single
>
>

```
{
    "event": "subscribe",
    "arg": {
        "channel": "orders",
        "instType": "FUTURES",
        "instId": "BTC-USD-200329"
    },
    "connId": "a4d3ae55"
}

```

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "orders",
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
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"orders\", \"instType\" : \"FUTURES\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|  Parameter  | Type |Required|                                            Description                                            |
|-------------|------|--------|---------------------------------------------------------------------------------------------------|
|    event    |String|  Yes   |                     Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`                     |
|     arg     |Object|   No   |                                        Subscribed channel                                         |
| \> channel  |String|  Yes   |                                           Channel name                                            |
| \> instType |String|  Yes   |Instrument type  <br/>`SPOT`  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`  <br/>`ANY`|
|\> instFamily|String|   No   |                                         Instrument family                                         |
|  \> instId  |String|   No   |                                           Instrument ID                                           |
|    code     |String|   No   |                                            Error code                                             |
|     msg     |String|   No   |                                           Error message                                           |
|   connId    |String|  Yes   |                                      WebSocket connection ID                                      |

>
>
> Push Data Example: single
>
>

```
{
    "arg": {
        "channel": "orders",
        "instType": "SPOT",
        "instId": "BTC-USDT",
        "uid": "614488474791936"
    },
    "data": [
        {
            "accFillSz": "0.001",
            "algoClOrdId": "",
            "algoId": "",
            "amendResult": "",
            "amendSource": "",
            "avgPx": "31527.1",
            "cancelSource": "",
            "category": "normal",
            "ccy": "",
            "clOrdId": "",
            "code": "0",
            "cTime": "1654084334977",
            "execType": "M",
            "fee": "-0.02522168",
            "feeCcy": "USDT",
            "fillFee": "-0.02522168",
            "fillFeeCcy": "USDT",
            "fillNotionalUsd": "31.50818374",
            "fillPx": "31527.1",
            "fillSz": "0.001",
            "fillPnl": "0.01",
            "fillTime": "1654084353263",
            "fillPxVol": "",
            "fillPxUsd": "",
            "fillMarkVol": "",
            "fillFwdPx": "",
            "fillMarkPx": "",
            "instId": "BTC-USDT",
            "instType": "SPOT",
            "lever": "0",
            "msg": "",
            "notionalUsd": "31.50818374",
            "ordId": "452197707845865472",
            "ordType": "limit",
            "pnl": "0",
            "posSide": "",
            "px": "31527.1",
            "pxUsd":"",
            "pxVol":"",
            "pxType":"",
            "quickMgnType": "",
            "rebate": "0",
            "rebateCcy": "BTC",
            "reduceOnly": "false",
            "reqId": "",
            "side": "sell",
            "attachAlgoClOrdId": "",
            "slOrdPx": "",
            "slTriggerPx": "",
            "slTriggerPxType": "last",
            "source": "",
            "state": "filled",
            "stpId": "",
            "stpMode": "",
            "sz": "0.001",
            "tag": "",
            "tdMode": "cash",
            "tgtCcy": "",
            "tpOrdPx": "",
            "tpTriggerPx": "",
            "tpTriggerPxType": "last",
            "attachAlgoOrds": [],
            "tradeId": "242589207",
            "lastPx": "38892.2",
            "uTime": "1654084353264",
            "isTpLimit": "false",
            "linkedAlgoOrd": {
                "algoId": ""
            }
        }
    ]
}

```

#### Push data parameters ####

|        Parameter        |      Type      |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|-------------------------|----------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|           arg           |     Object     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            Successfully subscribed channel                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
|       \> channel        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      Channel name                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|         \> uid          |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    User Identifier                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
|       \> instType       |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    Instrument type                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
|      \> instFamily      |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   Instrument family                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
|        \> instId        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     Instrument ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|          data           |Array of objects|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    Subscribed data                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
|       \> instType       |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    Instrument type                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
|        \> instId        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     Instrument ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|        \> tgtCcy        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      Order quantity unit setting for `sz`  <br/>`base_ccy`: Base currency ,`quote_ccy`: Quote currency   <br/>Only applicable to `SPOT` Market orders.   <br/>Default is `quote_ccy` for buy, `base_ccy` for sell                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|         \> ccy          |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              Margin currency   <br/>Applicable to all `isolated` `MARGIN` orders and `cross` `MARGIN` orders in `Spot and futures mode`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
|        \> ordId         |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        Order ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
|       \> clOrdId        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       Client Order ID as assigned by the client                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
|         \> tag          |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       Order tag                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
|          \> px          |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               Price   <br/>For options, use coin as unit (e.g. BTC, ETH)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
|        \> pxUsd         |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  Options price in USDOnly applicable to options; return "" for other instrument types                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
|        \> pxVol         |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        Implied volatility of the options orderOnly applicable to options; return "" for other instrument types                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
|        \> pxType        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       Price type of options   <br/>`px`: Place an order based on price, in the unit of coin (the unit for the request parameter px is BTC or ETH)   <br/>`pxVol`: Place an order based on pxVol   <br/>`pxUsd`: Place an order based on pxUsd, in the unit of USD (the unit for the request parameter px is USD)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|          \> sz          |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               The original order quantity, `SPOT`/`MARGIN`, in the unit of currency; `FUTURES`/`SWAP`/`OPTION`, in the unit of contract                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
|     \> notionalUsd      |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       Estimated national value in `USD` of order                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|       \> ordType        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    Order type   <br/>`market`: market order   <br/>`limit`: limit order   <br/>`post_only`: Post-only order   <br/>`fok`: Fill-or-kill order   <br/>`ioc`: Immediate-or-cancel order   <br/>`optimal_limit_ioc`: Market order with immediate-or-cancel order (applicable only to Expiry Futures and Perpetual Futures)  <br/>`mmp`: Market Maker Protection (only applicable to Option in Portfolio Margin mode)  <br/>`mmp_and_post_only`: Market Maker Protection and Post-only order(only applicable to Option in Portfolio Margin mode).   <br/>`op_fok`: Simple options (fok)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
|         \> side         |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                Order side, `buy` `sell`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
|       \> posSide        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                Position side   <br/>`net`   <br/>`long` or `short` Only applicable to `FUTURES`/`SWAP`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
|        \> tdMode        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              Trade mode, `cross`: cross `isolated`: isolated `cash`: cash                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
|        \> fillPx        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   Last filled price                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
|       \> tradeId        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     Last trade ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|        \> fillSz        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              Last filled quantity  <br/>The unit is `base_ccy` for SPOT and MARGIN, e.g. BTC-USDT, the unit is BTC; For market orders, the unit both is `base_ccy` when the tgtCcy is `base_ccy` or `quote_ccy`;  <br/>The unit is contract for `FUTURES`/`SWAP`/`OPTION`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
|       \> fillPnl        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           Last filled profit and loss, applicable to orders which have a trade and aim to close position. It always is 0 in other conditions                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
|       \> fillTime       |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    Last filled time                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
|       \> fillFee        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         last filled fee amount or rebate amount:   <br/>Negative number represents the user transaction fee charged by the platform;   <br/>Positive number represents rebate                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
|      \> fillFeeCcy      |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       last filled fee currency or rebate currency.  <br/>It is fee currency when fillFee is less than 0; It is rebate currency when fillFee\>=0.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|      \> fillPxVol       |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         Implied volatility when filled   <br/>Only applicable to options; return "" for other instrument types                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
|      \> fillPxUsd       |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 Options price when filled, in the unit of USD   <br/>Only applicable to options; return "" for other instrument types                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
|     \> fillMarkVol      |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          Mark volatility when filled   <br/>Only applicable to options; return "" for other instrument types                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
|      \> fillFwdPx       |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           Forward price when filled   <br/>Only applicable to options; return "" for other instrument types                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
|      \> fillMarkPx      |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        Mark price when filled   <br/>Applicable to `FUTURES`, `SWAP`, `OPTION`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
|       \> execType       |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             Liquidity taker or maker of the last filled, T: taker M: maker                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
|      \> accFillSz       |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           Accumulated fill quantity  <br/>The unit is `base_ccy` for SPOT and MARGIN, e.g. BTC-USDT, the unit is BTC; For market orders, the unit both is `base_ccy` when the tgtCcy is `base_ccy` or `quote_ccy`;  <br/>The unit is contract for `FUTURES`/`SWAP`/`OPTION`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
|   \> fillNotionalUsd    |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        Filled notional value in `USD` of order                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
|        \> avgPx         |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              Average filled price. If none is filled, it will return `0`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
|        \> state         |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       Order state   <br/>`canceled`  <br/>`live`   <br/>`partially_filled`   <br/>`filled`  <br/>`mmp_canceled`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
|        \> lever         |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    Leverage, from `0.01` to `125`.   <br/>Only applicable to `MARGIN/FUTURES/SWAP`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
|  \> attachAlgoClOrdId   |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              Client-supplied Algo ID when placing order attaching TP/SL.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
|     \> tpTriggerPx      |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             Take-profit trigger price, it                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
|   \> tpTriggerPxType    |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     Take-profit trigger price type.   <br/>`last`: last price  <br/>`index`: index price  <br/>`mark`: mark price                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|       \> tpOrdPx        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              Take-profit order price, it                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
|     \> slTriggerPx      |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              Stop-loss trigger price, it                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
|   \> slTriggerPxType    |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      Stop-loss trigger price type.   <br/>`last`: last price  <br/>`index`: index price  <br/>`mark`: mark price                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|       \> slOrdPx        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               Stop-loss order price, it                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
|    \> attachAlgoOrds    |Array of object |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     TP/SL information attached when placing order                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|    \>\> attachAlgoId    |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         The order ID of attached TP/SL order. It can be used to identity the TP/SL order when amending. It will not be posted to algoId when placing TP/SL order after the general order is filled completely.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| \>\> attachAlgoClOrdId  |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   Client-supplied Algo ID when placing order attaching TP/SL  <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.  <br/>It will be posted to `algoClOrdId` when placing TP/SL order once the general order is filled completely.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
|     \>\> tpOrdKind      |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     TP order kind  <br/>`condition`  <br/>`limit`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|    \>\> tpTriggerPx     |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               Take-profit trigger price.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
|  \>\> tpTriggerPxType   |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     Take-profit trigger price type.   <br/>`last`: last price  <br/>`index`: index price  <br/>`mark`: mark price                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|      \>\> tpOrdPx       |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                Take-profit order price.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
|    \>\> slTriggerPx     |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                Stop-loss trigger price.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
|  \>\> slTriggerPxType   |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      Stop-loss trigger price type.   <br/>`last`: last price  <br/>`index`: index price  <br/>`mark`: mark price                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|      \>\> slOrdPx       |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 Stop-loss order price.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
|         \>\> sz         |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     Size. Only applicable to TP order of split TPs                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
|\>\> amendPxOnTriggerType|     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          Whether to enable Cost-price SL. Only applicable to SL order of split TPs.   <br/>`0`: disable, the default value   <br/>`1`: Enable                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
|    \> linkedAlgoOrd     |     Object     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             Linked SL order detail, only applicable to TP limit order of one-cancels-the-other order(oco)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
|       \>\> algoId       |     Object     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        Algo ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
|        \> stpId         |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          ~~Self trade prevention ID  <br/>Return "" if self trade prevention is not applicable~~ (deprecated)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
|       \> stpMode        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 Self trade prevention mode   <br/>Return "" if self trade prevention is not applicable                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
|        \> feeCcy        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            Fee currency   <br/>`SPOT`/`MARGIN`: If you buy, you will receive `base currency`; if you sell, you will receive `quota currency`   <br/>`FUTURES`/`SWAP`/`OPTION` What is charged is the margin                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
|         \> fee          |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 Fee and rebate  <br/>For spot and margin, it is accumulated fee charged by the platform. It is always negative, e.g. -0.01.   <br/>For Expiry Futures, Perpetual Futures and Options, it is accumulated fee and rebate                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
|      \> rebateCcy       |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               Rebate currency, if there is no rebate, this field is "".                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
|        \> rebate        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               Rebate accumulated amount, only applicable to spot and margin, the reward of placing orders from the platform (rebate) given to user who has reached the specified trading level. If there is no rebate, this field is "".                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
|         \> pnl          |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     Profit and loss, applicable to orders which have a trade and aim to close position. It always is 0 in other conditions.   <br/>For liquidation under cross margin mode, it will include liquidation penalties.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
|        \> source        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               Order source  <br/>`6`: The normal order triggered by the `trigger order`  <br/>`7`:The normal order triggered by the `TP/SL order`   <br/>`13`: The normal order triggered by the algo order  <br/>`25`:The normal order triggered by the `trailing stop order`  <br/>`34`: The normal order triggered by the chase order                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
|     \> cancelSource     |     String     |Source of the order cancellation.  <br/>Valid values and the corresponding meanings are:  <br/>`0`: Order canceled by system  <br/>`1`: Order canceled by user  <br/>`2`: Order canceled: Pre reduce-only order canceled, due to insufficient margin in user position  <br/>`3`: Order canceled: Risk cancellation was triggered. Pending order was canceled due to insufficient margin ratio and forced-liquidation risk.  <br/>`4`: Order canceled: Borrowings of crypto reached hard cap, order was canceled by system.  <br/>`6`: Order canceled: ADL order cancellation was triggered. Pending order was canceled due to a low margin ratio and forced-liquidation risk.   <br/>`7`: Order canceled: Futures contract delivery.   <br/>`9`: Order canceled: Insufficient balance after funding fees deducted.   <br/>`10`: Order canceled: Option contract expiration.  <br/>`13`: Order canceled: FOK order was canceled due to incompletely filled.  <br/>`14`: Order canceled: IOC order was partially canceled due to incompletely filled.  <br/>`15`: Order canceled: The order price is beyond the limit  <br/>`17`: Order canceled: Close order was canceled, due to the position was already closed at market price.  <br/>`20`: Cancel all after triggered  <br/>`21`: Order canceled: The TP/SL order was canceled because the position had been closed  <br/>`22`, `23`: Order canceled: Reduce-only orders only allow reducing your current position. System has already canceled this order.  <br/>`27`: Order canceled: Price limit verification failed because the price difference between counterparties exceeds 5%   <br/>`31`: The post-only order will take liquidity in taker orders   <br/>`32`: Self trade prevention   <br/>`33`: The order exceeds the maximum number of order matches per taker order  <br/>`36`: Your TP limit order was canceled because the corresponding SL order was triggered.   <br/>`37`: Your TP limit order was canceled because the corresponding SL order was canceled.  <br/>`38`: You have canceled market maker protection (MMP) orders.  <br/>`39`: Your order was canceled because market maker protection (MMP) was triggered.   <br/>`42`: Your order was canceled because the difference between the initial and current best bid or ask prices reached the maximum chase difference.|
|     \> amendSource      |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        Source of the order amendation.   <br/>`1`: Order amended by user  <br/>`2`: Order amended by user, but the order quantity is overriden by system due to reduce-only  <br/>`3`: New order placed by user, but the order quantity is overriden by system due to reduce-only  <br/>`4`: Order amended by system due to other pending orders  <br/>`5`: Order modification due to changes in options px, pxVol, or pxUsd as a result of following variations. For example, when iv = 60, USD and px are anchored at iv = 60, the changes in USD or px lead to modification.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
|       \> category       |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             Category   <br/>`normal`   <br/>`twap`   <br/>`adl`   <br/>`full_liquidation`   <br/>`partial_liquidation`  <br/>`delivery`   <br/>`ddh`: Delta dynamic hedge                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
|      \> isTpLimit       |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      Whether it is TP limit order. true or false                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|        \> uTime         |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        Update time, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
|        \> cTime         |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       Creation time, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|        \> reqId         |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  Client Request ID as assigned by the client for order amendment. "" will be returned if there is no order amendment.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
|     \> amendResult      |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       The result of amending the order   <br/>`-1`: failure   <br/>`0`: success   <br/>`1`: Automatic cancel (amendment request returned success but amendment subsequently failed then automatically canceled by the system)   <br/>`2`: Automatic amendation successfully, only applicable to pxVol and pxUsd orders of Option.  <br/>When amending the order through API and `cxlOnFail` is set to `true` in the order amendment request but the amendment is rejected, "" is returned.   <br/> When amending the order through API, the order amendment acknowledgement returns success and the amendment subsequently failed, `-1` will be returned if `cxlOnFail` is set to `false`, `1` will be returned if `cxlOnFail` is set to `true`.   <br/>When amending the order through Web/APP and the amendment failed, `-1` will be returned.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|      \> reduceOnly      |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 Whether the order can only reduce the position size. Valid options: `true` or `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
|     \> quickMgnType     |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 Quick Margin type, Only applicable to Quick Margin Mode of isolated margin  <br/>`manual`, `auto_borrow`, `auto_repay`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
|     \> algoClOrdId      |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 Client-supplied Algo ID. There will be a value when algo order attaching `algoClOrdId` is triggered, or it will be "".                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
|        \> algoId        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     Algo ID. There will be a value when algo order is triggered, or it will be "".                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
|        \> lastPx        |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       Last price                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|         \> code         |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              Error Code, the default is 0                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
|         \> msg          |     String     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            Error Message, The default is ""                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |

 For market orders, it's likely the orders channel will show order state as "filled" while showing the "last filled quantity (fillSz)" as 0.

---

### WS / Fills channel ###

Retrieve transaction information. Data will not be pushed when first subscribed. Data will only be pushed when there are order book fill events, where tradeId \> 0.  

The channel is exclusively available to users with trading fee tier VIP5 or above. For other users, please use [WS / Order channel](/docs-v5/en/#order-book-trading-trade-ws-order-channel).

#### URL Path ####

/ws/v5/private (required login)

>
>
> Request Example: single
>
>

```
{
    "op": "subscribe",
    "args": [
        {
            "channel": "fills",
            "instId": "BTC-USDT-SWAP"
        }
    ]
}

```

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
            "channel": "fills"
        }
    ]
}

```

#### Request Parameters ####

|Parameter |     Type      |Required|               Description               |
|----------|---------------|--------|-----------------------------------------|
|    op    |    String     |  Yes   |Operation  <br/>`subscribe` `unsubscribe`|
|   args   |Array of object|  Yes   |       List of subscribed channels       |
|\> channel|    String     |  Yes   |          Channel name `fills`           |
|\> instId |    String     |   No   |              Instrument ID              |

>
>
> Successful Response Example: single
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "fills",
    "instId": "BTC-USDT-SWAP"
  },
  "connId": "a4d3ae55"
}

```

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "fills"
  },
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|                 Description                 |
|----------|------|--------|---------------------------------------------|
|  event   |String|  Yes   |Event  <br/>`subscribe` `unsubscribe` `error`|
|   arg    |Object|   No   |             Subscribed channel              |
|\> channel|String|  Yes   |                Channel name                 |
|\> instId |String|   No   |                Instrument ID                |
|   code   |String|   No   |                 Error code                  |
|   msg    |String|   No   |                Error message                |
|  connId  |String|  Yes   |           WebSocket connection ID           |

>
>
> Push Data Example: single
>
>

```
{
    "arg": {
        "channel": "fills",
        "instId": "BTC-USDT-SWAP",
        "uid": "614488474791111"
    },
    "data":[
        {
            "instId": "BTC-USDT-SWAP",
            "fillSz": "100",
            "fillPx": "70000",
            "side": "buy",
            "ts": "1705449605015",
            "ordId": "680800019749904384",
            "tradeId": "12345",
            "execType": "T",
            "count": "10"
        }
    ]
}

```

#### Push data parameters ####

| Parameter |     Type      |                                       Description                                       |
|-----------|---------------|-----------------------------------------------------------------------------------------|
|    arg    |    Object     |                             Successfully subscribed channel                             |
|\> channel |    String     |                                      Channel name                                       |
|  \> uid   |    String     |                                     User Identifier                                     |
| \> instId |    String     |                                      Instrument ID                                      |
|   data    |Array of object|                                     Subscribed data                                     |
| \> instId |    String     |                                      Instrument ID                                      |
| \> fillSz |    String     |Filled quantity. If the trade is aggregated, the filled quantity will also be aggregated.|
| \> fillPx |    String     |                                    Last filled price                                    |
|  \> side  |    String     |                           Trade direction  <br/>`buy` `sell`                            |
|   \> ts   |    String     |        Filled time, Unix timestamp format in milliseconds, e.g. `1597026383085`         |
| \> ordId  |    String     |                                        Order ID                                         |
|\> tradeId |    String     |                       The last trade ID in the trades aggregation                       |
|\> execType|    String     |                     Liquidity taker or maker, `T`: taker `M`: maker                     |
| \> count  |    String     |                             The count of trades aggregated                              |
 \- The channel is exclusively available to users with trading fee tier VIP5 or above. Others will receive error code 60029 when subscribing to it.   
\- The channel only pushes partial information of the orders channel. Fill events of block trading, nitro spread, liquidation, ADL, and some other non order book events will not be pushed through this channel. Users should also subscribe to the orders channel for order confirmation.   
\- When a fill event is received by this channel, the account balance, margin, and position information might not have changed yet.   
\- Taker orders will be aggregated based on different fill prices. When aggregation occurs, the count field indicates the number of orders matched, and the tradeId represents the tradeId of the last trade in the aggregation. Maker orders will not be aggregated.   
\- In the future, connection limits will be imposed on this channel. The maximum number of connections subscribing to this channel per subaccount will be 20. We recommend users always use this channel within this limit to avoid any impact on their strategies when the limit is enforced.

---

### WS / Place order ###

You can place an order only if you have sufficient funds.  

#### URL Path ####

/ws/v5/private (required login)

#### Rate Limit: 60 requests per 2 seconds ####

#### Rate Limit of lead instruments for Copy Trading: 4 requests per 2 seconds ####

#### Rate limit rule (except Options): User ID + Instrument ID ####

#### Rate limit rule (Options only): User ID + Instrument Family ####

Rate limit of this endpoint will also be affected by the rules [Sub-account rate limit](/docs-v5/en/#overview-rate-limits-sub-account-rate-limit) and [Fill ratio based sub-account rate limit](/docs-v5/en/#overview-rate-limits-fill-ratio-based-sub-account-rate-limit).

Rate limit is shared with the `Place order` REST API endpoints
>
>
> Request Example
>
>

```
{
  "id": "1512",
  "op": "order",
  "args": [
    {
      "side": "buy",
      "instId": "BTC-USDT",
      "tdMode": "isolated",
      "ordType": "market",
      "sz": "100"
    }
  ]
}

```

#### Request Parameters ####

|   Parameter   |      Type      | Required  |                                                                                                                                                                                                                                 Description                                                                                                                                                                                                                                  |
|---------------|----------------|-----------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      id       |     String     |    Yes    |                                                                                                           Unique identifier of the message   <br/>Provided by client. It will be returned in response message for identifying the corresponding request.   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.                                                                                                           |
|      op       |     String     |    Yes    |                                                                                                                                                                                                                           Operation  <br/>`order`                                                                                                                                                                                                                            |
|     args      |Array of objects|    Yes    |                                                                                                                                                                                                                              Request parameters                                                                                                                                                                                                                              |
|   \> instId   |     String     |    Yes    |                                                                                                                                                                                                                        Instrument ID, e.g. `BTC-USDT`                                                                                                                                                                                                                        |
|   \> tdMode   |     String     |    Yes    |                                                                                                                                   Trade mode   <br/>Margin mode `isolated` `cross`   <br/>Non-Margin mode `cash`  <br/>`spot_isolated` (only applicable to SPOT lead trading, `tdMode` should be `spot_isolated` for `SPOT` lead trading.)                                                                                                                                   |
|    \> ccy     |     String     |    No     |                                                                                                                                                                         Margin currency   <br/>Applicable to all `isolated` `MARGIN` orders and `cross` `MARGIN` orders in `Spot and futures mode`.                                                                                                                                                                          |
|  \> clOrdId   |     String     |    No     |                                                                                                                                                             Client Order ID as assigned by the client   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.                                                                                                                                                              |
|    \> tag     |     String     |    No     |                                                                                                                                                                             Order tag   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 16 characters.                                                                                                                                                                              |
|    \> side    |     String     |    Yes    |                                                                                                                                                                                                                           Order side, `buy` `sell`                                                                                                                                                                                                                           |
|  \> posSide   |     String     |Conditional|                                                                                                                                          Position side   <br/>The default is `net` in the `net` mode   <br/>It is required in the `long/short` mode, and can only be `long` or `short`.   <br/>Only applicable to `FUTURES`/`SWAP`.                                                                                                                                          |
|  \> ordType   |     String     |    Yes    |Order type   <br/>`market`: market order   <br/>`limit`: limit order   <br/>`post_only`: Post-only order   <br/>`fok`: Fill-or-kill order   <br/>`ioc`: Immediate-or-cancel order   <br/>`optimal_limit_ioc`: Market order with immediate-or-cancel order  <br/>`mmp`: Market Maker Protection (only applicable to Option in Portfolio Margin mode)  <br/>`mmp_and_post_only`: Market Maker Protection and Post-only order(only applicable to Option in Portfolio Margin mode)|
|     \> sz     |     String     |    Yes    |                                                                                                                                                                                                                           Quantity to buy or sell.                                                                                                                                                                                                                           |
|     \> px     |     String     |Conditional|                                                                                                                                 Order price. Only applicable to `limit`,`post_only`,`fok`,`ioc`,`mmp`,`mmp_and_post_only` order.  <br/>When placing an option order, one of px/pxUsd/pxVol must be filled in, and only one can be filled in                                                                                                                                  |
|   \> pxUsd    |     String     |Conditional|                                                                                                                                                 Place options orders in `USD`   <br/>Only applicable to options   <br/>When placing an option order, one of px/pxUsd/pxVol must be filled in, and only one can be filled in                                                                                                                                                  |
|   \> pxVol    |     String     |Conditional|                                                                                                                           Place options orders based on implied volatility, where 1 represents 100%   <br/>Only applicable to options   <br/>When placing an option order, one of px/pxUsd/pxVol must be filled in, and only one can be filled in                                                                                                                            |
| \> reduceOnly |    Boolean     |    No     |                                                                                        Whether the order can only reduce the position size.   <br/>Valid options: `true` or `false`. The default value is `false`.  <br/>Only applicable to `MARGIN` orders, and `FUTURES`/`SWAP` orders in `net` mode   <br/>Only applicable to `Spot and futures mode` and `Multi-currency margin`                                                                                         |
|   \> tgtCcy   |     String     |    No     |                                                                                                                                  Order quantity unit setting for `sz`  <br/>`base_ccy`: Base currency ,`quote_ccy`: Quote currency   <br/>Only applicable to `SPOT` Market Orders  <br/>Default is `quote_ccy` for buy, `base_ccy` for sell                                                                                                                                  |
|  \> banAmend  |    Boolean     |    No     |                                                                                Whether to disallow the system from amending the size of the SPOT Market Order.  <br/>Valid options: `true` or `false`. The default value is `false`.  <br/>If `true`, system will not amend and reject the market order if user does not have sufficient funds.   <br/>Only applicable to SPOT Market Orders                                                                                 |
|\> quickMgnType|     String     |    No     |                                                                                                                                                 ~~Quick Margin type. Only applicable to Quick Margin Mode of isolated margin   <br/>`manual`, `auto_borrow`, `auto_repay`  <br/>The default value is `manual`~~(Deprecated)                                                                                                                                                  |
|   \> stpId    |     String     |    No     |                                                                                                                               ~~Self trade prevention ID. Orders from the same master account with the same ID will be prevented from self trade.  <br/>Numerical integers defined by user in the range of 1\<= x\<= 999999999~~ (deprecated)                                                                                                                                |
|  \> stpMode   |     String     |    No     |                                                                                                                                                            Self trade prevention mode.   <br/>Default to cancel maker   <br/>`cancel_maker`,`cancel_taker`, `cancel_both`  <br/>Cancel both does not support FOK.                                                                                                                                                            |
|    expTime    |     String     |    No     |                                                                                                                                                                                           Request effective deadline. Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                                                                                                                                            |

>
>
> Successful Response Example
>
>

```
{
  "id": "1512",
  "op": "order",
  "data": [
    {
      "clOrdId": "",
      "ordId": "12345689",
      "tag": "",
      "ts":"1695190491421",
      "sCode": "0",
      "sMsg": ""
    }
  ],
  "code": "0",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Failure Response Example
>
>

```
{
  "id": "1512",
  "op": "order",
  "data": [
    {
      "clOrdId": "",
      "ordId": "",
      "tag": "",
      "ts":"1695190491421",
      "sCode": "5XXXX",
      "sMsg": "not exist"
    }
  ],
  "code": "1",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Response Example When Format Error
>
>

```
{
  "id": "1512",
  "op": "order",
  "data": [],
  "code": "60013",
  "msg": "Invalid args",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

#### Response Parameters ####

|Parameter |      Type      |                                                           Description                                                            |
|----------|----------------|----------------------------------------------------------------------------------------------------------------------------------|
|    id    |     String     |                                                 Unique identifier of the message                                                 |
|    op    |     String     |                                                            Operation                                                             |
|   code   |     String     |                                                            Error Code                                                            |
|   msg    |     String     |                                                          Error message                                                           |
|   data   |Array of objects|                                                               Data                                                               |
| \> ordId |     String     |                                                             Order ID                                                             |
|\> clOrdId|     String     |                                            Client Order ID as assigned by the client                                             |
|  \> tag  |     String     |                                                            Order tag                                                             |
|  \> ts   |     String     |Timestamp when the order request processing is finished by our system, Unix timestamp format in milliseconds, e.g. `1597026383085`|
| \> sCode |     String     |                                               Order status code, `0` means success                                               |
| \> sMsg  |     String     |                                         Rejection or success message of event execution.                                         |
|  inTime  |     String     |   Timestamp at Websocket gateway when the request is received, Unix timestamp format in microseconds, e.g. `1597026383085123`    |
| outTime  |     String     |     Timestamp at Websocket gateway when the response is sent, Unix timestamp format in microseconds, e.g. `1597026383085123`     |
tdMode  
Trade Mode, when placing an order, you need to specify the trade mode.  
**Spot mode:**  
\- SPOT and OPTION buyer: cash  
**Spot and futures mode:**  
\- Isolated MARGIN: isolated  
\- Cross MARGIN: cross  
\- SPOT: cash  
\- Cross FUTURES/SWAP/OPTION: cross  
\- Isolated FUTURES/SWAP/OPTION: isolated  
**Multi-currency margin:**  
\- Isolated MARGIN: isolated  
\- Cross SPOT: cross  
\- Cross FUTURES/SWAP/OPTION: cross  
\- Isolated FUTURES/SWAP/OPTION: isolated  
**Portfolio margin:**  
\- Isolated MARGIN: isolated  
\- Cross SPOT: cross  
\- Cross FUTURES/SWAP/OPTION: cross  
\- Isolated FUTURES/SWAP/OPTION: isolated clOrdId  
clOrdId is a user-defined unique ID used to identify the order. It will be included in the response parameters if you have specified during order submission, and can be used as a request parameter to the endpoints to query, cancel and amend orders.   
clOrdId must be unique among the clOrdIds of all pending orders. posSide  
Position side, this parameter is not mandatory in **net** mode. If you pass it through, the only valid value is **net**.  
In **long/short** mode, it is mandatory. Valid values are **long** or **short**.  
In **long/short** mode, **side** and **posSide** need to be specified in the combinations below:  
Open long: buy and open long (side: fill in buy; posSide: fill in long)  
Open short: sell and open short (side: fill in sell; posSide: fill in short)  
Close long: sell and close long (side: fill in sell; posSide: fill in long)  
Close short: buy and close short (side: fill in buy; posSide: fill in short)  
Portfolio margin mode: Expiry Futures and Perpetual Futures only support net mode ordType   
Order type. When creating a new order, you must specify the order type. The order type you specify will affect: 1) what order parameters are required, and 2) how the matching system executes your order. The following are valid order types:   
limit: Limit order, which requires specified sz and px.   
market: Market order. For SPOT and MARGIN, market order will be filled with market price (by swiping opposite order book). For Expiry Futures and Perpetual Futures, market order will be placed to order book with most aggressive price allowed by Price Limit Mechanism. For OPTION, market order is not supported yet. As the filled price for market orders cannot be determined in advance, OKX reserves/freezes your quote currency by an additional 5% for risk check.   
post\_only: Post-only order, which the order can only provide liquidity to the market and be a maker. If the order would have executed on placement, it will be canceled instead.   
fok: Fill or kill order. If the order cannot be fully filled, the order will be canceled. The order would not be partially filled.   
ioc: Immediate or cancel order. Immediately execute the transaction at the order price, cancel the remaining unfilled quantity of the order, and the order quantity will not be displayed in the order book.   
optimal\_limit\_ioc: Market order with ioc (immediate or cancel). Immediately execute the transaction of this market order, cancel the remaining unfilled quantity of the order, and the order quantity will not be displayed in the order book. Only applicable to Expiry Futures and Perpetual Futures. sz  
Quantity to buy or sell.   
For SPOT/MARGIN Buy and Sell Limit Orders, it refers to the quantity in base currency.   
For MARGIN Buy Market Orders, it refers to the quantity in quote currency.   
For MARGIN Sell Market Orders, it refers to the quantity in base currency.   
For SPOT Market Orders, it is set by tgtCcy.   
For FUTURES/SWAP/OPTION orders, it refers to the number of contracts. reduceOnly  
When placing an order with this parameter set to true, it means that the order will reduce the size of the position only  
For the same MARGIN instrument, the coin quantity of all reverse direction pending orders adds `sz` of new `reduceOnly` order cannot exceed the position assets. After the debt is paid off, if there is a remaining size of orders, the position will not be opened in reverse, but will be traded in SPOT.  
For the same FUTURES/SWAP instrument, the sum of the current order size and all reverse direction reduce-only pending orders which's price-time priority is higher than the current order, cannot exceed the contract quantity of position.  
Only applicable to `Spot and futures mode` and `Multi-currency margin`  
Only applicable to `MARGIN` orders, and `FUTURES`/`SWAP` orders in `net` mode  
Notice: Under long/short mode of Expiry Futures and Perpetual Futures, all closing orders apply the reduce-only feature which is not affected by this parameter. tgtCcy  
This parameter is used to specify the order quantity in the order request is denominated in the quantity of base or quote currency. This is applicable to SPOT Market Orders only.  
Base currency: base\_ccy  
Quote currency: quote\_ccy  
If you use the Base Currency quantity for buy market orders or the Quote Currency for sell market orders, please note:  
1. If the quantity you enter is greater than what you can buy or sell, the system will execute the order according to your maximum buyable or sellable quantity. If you want to trade according to the specified quantity, you should use Limit orders.  
2. When the market price is too volatile, the locked balance may not be sufficient to buy the Base Currency quantity or sell to receive the Quote Currency that you specified. We will change the quantity of the order to execute the order based on best effort principle based on your account balance. In addition, we will try to over lock a fraction of your balance to avoid changing the order quantity.  
2.1 Example of base currency buy market order:  
Taking the market order to buy 10 LTCs as an example, and the user can buy 11 LTC. At this time, if 10 \< 11, the order is accepted. When the LTC-USDT market price is 200, and the locked balance of the user is 3,000 USDT, as 200\*10 \< 3,000, the market order of 10 LTC is fully executed;
If the market is too volatile and the LTC-USDT market price becomes 400, 400\*10 \> 3,000, the user's locked balance is not sufficient to buy using the specified amount of base currency, the user's maximum locked balance of 3,000 USDT will be used to settle the trade. Final transaction quantity becomes 3,000/400 = 7.5 LTC.  
2.2 Example of quote currency sell market order:  
Taking the market order to sell 1,000 USDT as an example, and the user can sell 1,200 USDT, 1,000 \< 1,200, the order is accepted. When the LTC-USDT market price is 200, and the locked balance of the user is 6 LTC, as 1,000/200 \< 6, the market order of 1,000 USDT is fully executed;
If the market is too volatile and the LTC-USDT market price becomes 100, 100\*6 \< 1,000, the user's locked balance is not sufficient to sell using the specified amount of quote currency, the user's maximum locked balance of 6 LTC will be used to settle the trade. Final transaction quantity becomes 6 \* 100 = 600 USDT. px  
The value for px must be a multiple of tickSz for OPTION orders.  
If not, the system will apply the rounding rules below. Using tickSz 0.0005 as an example:  
The px will be rounded up to the nearest 0.0005 when the remainder of px to 0.0005 is more than 0.00025 or `px` is less than 0.0005.  
The px will be rounded down to the nearest 0.0005 when the remainder of px to 0.0005 is less than 0.00025 and `px` is more than 0.0005. Mandatory self trade prevention (STP)  
The trading platform imposes mandatory self trade prevention at master account level, which means the accounts under the same master account, including master account itself and all its affiliated sub-accounts, will be prevented from self trade. The default STP mode is `Cancel Maker`. Users can also utilize the stpMode request parameter of the placing order endpoint to determine the stpMode of a certain order.  
Mandatory self trade prevention will not lead to latency.   
There are three STP modes. The STP mode is always taken based on the configuration in the taker order.  
1. Cancel Maker: This is the default STP mode, which cancels the maker order to prevent self-trading. Then, the taker order continues to match with the next order based on the order book priority.  
2. Cancel Taker: The taker order is canceled to prevent self-trading. If the user's own maker order is lower in the order book priority, the taker order is partially filled and then canceled. FOK orders are always honored and canceled if they would result in self-trading.  
3. Cancel Both: Both taker and maker orders are canceled to prevent self-trading. If the user's own maker order is lower in the order book priority, the taker order is partially filled. Then, the remaining quantity of the taker order and the first maker order are canceled. FOK orders are not supported in this mode.

---

### WS / Place multiple orders ###

Place orders in a batch. Maximum 20 orders can be placed per request  

#### URL Path ####

/ws/v5/private (required login)

#### Rate Limit: 300 orders per 2 seconds ####

#### Rate Limit of lead instruments for Copy Trading: 4 orders per 2 seconds ####

#### Rate limit rule (except Options): User ID + Instrument ID ####

#### Rate limit rule (Options only): User ID + Instrument Family ####

Rate limit of this endpoint will also be affected by the rules [Sub-account rate limit](/docs-v5/en/#overview-rate-limits-sub-account-rate-limit) and [Fill ratio based sub-account rate limit](/docs-v5/en/#overview-rate-limits-fill-ratio-based-sub-account-rate-limit).

Unlike other endpoints, the rate limit of this endpoint is determined by the number of orders. If there is only one order in the request, it will consume the rate limit of `Place order`. Rate limit is shared with the `Place multiple orders` REST API endpoints
>
>
> Request Example
>
>

```
{
  "id": "1513",
  "op": "batch-orders",
  "args": [
    {
      "side": "buy",
      "instId": "BTC-USDT",
      "tdMode": "cash",
      "ordType": "market",
      "sz": "100"
    },
    {
      "side": "buy",
      "instId": "LTC-USDT",
      "tdMode": "cash",
      "ordType": "market",
      "sz": "1"
    }
  ]
}

```

#### Request Parameters ####

|   Parameter   |      Type      | Required  |                                                                                                                                                                                                                                                               Description                                                                                                                                                                                                                                                               |
|---------------|----------------|-----------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      id       |     String     |    Yes    |                                                                                                                                        Unique identifier of the message   <br/>Provided by client. It will be returned in response message for identifying the corresponding request.   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.                                                                                                                                         |
|      op       |     String     |    Yes    |                                                                                                                                                                                                                                                     Operation  <br/>`batch-orders`                                                                                                                                                                                                                                                      |
|     args      |Array of objects|    Yes    |                                                                                                                                                                                                                                                           Request Parameters                                                                                                                                                                                                                                                            |
|   \> instId   |     String     |    Yes    |                                                                                                                                                                                                                                                     Instrument ID, e.g. `BTC-USDT`                                                                                                                                                                                                                                                      |
|   \> tdMode   |     String     |    Yes    |                                                                                                                                                                Trade mode   <br/>Margin mode `isolated` `cross`   <br/>Non-Margin mode `cash`  <br/>`spot_isolated` (only applicable to SPOT lead trading, `tdMode` should be `spot_isolated` for `SPOT` lead trading.)                                                                                                                                                                 |
|    \> ccy     |     String     |    No     |                                                                                                                                                                                                       Margin currency   <br/>Applicable to all `isolated` `MARGIN` orders and `cross` `MARGIN` orders in `Spot and futures mode`.                                                                                                                                                                                                       |
|  \> clOrdId   |     String     |    No     |                                                                                                                                                                                           Client Order ID as assigned by the client   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.                                                                                                                                                                                           |
|    \> tag     |     String     |    No     |                                                                                                                                                                                                           Order tag   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 16 characters.                                                                                                                                                                                                           |
|    \> side    |     String     |    Yes    |                                                                                                                                                                                                                                                        Order side, `buy` `sell`                                                                                                                                                                                                                                                         |
|  \> posSide   |     String     |Conditional|                                                                                                                                                                           Position side   <br/>The default `net` in the `net` mode   <br/>It is required in the `long/short` mode, and only be `long` or `short`.   <br/>Only applicable to `FUTURES`/`SWAP`.                                                                                                                                                                           |
|  \> ordType   |     String     |    Yes    |Order type   <br/>`market`: market order   <br/>`limit`: limit order   <br/>`post_only`: Post-only order   <br/>`fok`: Fill-or-kill order   <br/>`ioc`: Immediate-or-cancel order   <br/>`optimal_limit_ioc`: Market order with immediate-or-cancel order (applicable only to Expiry Futures and Perpetual Futures)  <br/>`mmp`: Market Maker Protection (only applicable to Option in Portfolio Margin mode)  <br/>`mmp_and_post_only`: Market Maker Protection and Post-only order(only applicable to Option in Portfolio Margin mode).|
|     \> sz     |     String     |    Yes    |                                                                                                                                                                                                                                                        Quantity to buy or sell.                                                                                                                                                                                                                                                         |
|     \> px     |     String     |Conditional|                                                                                                                                                               Order price. Only applicable to `limit`,`post_only`,`fok`,`ioc`,`mmp`,`mmp_and_post_only` order.  <br/>When placing an option order, one of px/pxUsd/pxVol must be filled in, and only one can be filled in                                                                                                                                                               |
|   \> pxUsd    |     String     |Conditional|                                                                                                                                                                               Place options orders in `USD`   <br/>Only applicable to options   <br/>When placing an option order, one of px/pxUsd/pxVol must be filled in, and only one can be filled in                                                                                                                                                                               |
|   \> pxVol    |     String     |Conditional|                                                                                                                                                         Place options orders based on implied volatility, where 1 represents 100%   <br/>Only applicable to options   <br/>When placing an option order, one of px/pxUsd/pxVol must be filled in, and only one can be filled in                                                                                                                                                         |
| \> reduceOnly |    Boolean     |    No     |                                                                                                                      Whether the order can only reduce the position size.   <br/>Valid options: `true` or `false`. The default value is `false`.  <br/>Only applicable to `MARGIN` orders, and `FUTURES`/`SWAP` orders in `net` mode   <br/>Only applicable to `Spot and futures mode` and `Multi-currency margin`                                                                                                                      |
|   \> tgtCcy   |     String     |    No     |                                                                                                                                                               Order quantity unit setting for `sz`  <br/>`base_ccy`: Base currency ,`quote_ccy`: Quote currency   <br/>Only applicable to `SPOT` Market Orders  <br/>Default is `quote_ccy` for buy, `base_ccy` for sell                                                                                                                                                                |
|  \> banAmend  |    Boolean     |    No     |                                                                                                              Whether to disallow the system from amending the size of the SPOT Market Order.  <br/>Valid options: `true` or `false`. The default value is `false`.  <br/>If `true`, system will not amend and reject the market order if user does not have sufficient funds.   <br/>Only applicable to SPOT Market Orders                                                                                                              |
|\> quickMgnType|     String     |    No     |                                                                                                                                                                               ~~Quick Margin type. Only applicable to Quick Margin Mode of isolated margin   <br/>`manual`, `auto_borrow`, `auto_repay`  <br/>The default value is `manual`~~(Deprecated)                                                                                                                                                                               |
|   \> stpId    |     String     |    No     |                                                                                                                                                             ~~Self trade prevention ID. Orders from the same master account with the same ID will be prevented from self trade.  <br/>Numerical integers defined by user in the range of 1\<= x\<= 999999999~~ (deprecated)                                                                                                                                                             |
|  \> stpMode   |     String     |    No     |                                                                                                                                                                                         Self trade prevention mode.   <br/>Default to cancel maker   <br/>`cancel_maker`,`cancel_taker`, `cancel_both`  <br/>Cancel both does not support FOK.                                                                                                                                                                                          |
|    expTime    |     String     |    No     |                                                                                                                                                                                                                         Request effective deadline. Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                                                                                                                                                                         |

>
>
> Response Example When All Succeed
>
>

```
{
  "id": "1513",
  "op": "batch-orders",
  "data": [
    {
      "clOrdId": "",
      "ordId": "12345689",
      "tag": "",
      "ts": "1695190491421",
      "sCode": "0",
      "sMsg": ""
    },
    {
      "clOrdId": "",
      "ordId": "12344",
      "tag": "",
      "ts": "1695190491421",
      "sCode": "0",
      "sMsg": ""
    }
  ],
  "code": "0",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Response Example When Partially Successful
>
>

```
{
  "id": "1513",
  "op": "batch-orders",
  "data": [
    {
      "clOrdId": "",
      "ordId": "12345689",
      "tag": "",
      "ts": "1695190491421",
      "sCode": "0",
      "sMsg": ""
    },
    {
      "clOrdId": "",
      "ordId": "",
      "tag": "",
      "ts": "1695190491421",
      "sCode": "5XXXX",
      "sMsg": "Insufficient margin"
    }
  ],
  "code": "2",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Response Example When All Failed
>
>

```
{
  "id": "1513",
  "op": "batch-orders",
  "data": [
    {
      "clOrdId": "oktswap6",
      "ordId": "",
      "tag": "",
      "ts": "1695190491421",
      "sCode": "5XXXX",
      "sMsg": "Insufficient margin"
    },
    {
      "clOrdId": "oktswap7",
      "ordId": "",
      "tag": "",
      "ts": "1695190491421",
      "sCode": "5XXXX",
      "sMsg": "Insufficient margin"
    }
  ],
  "code": "1",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Response Example When Format Error
>
>

```
{
  "id": "1513",
  "op": "batch-orders",
  "data": [],
  "code": "60013",
  "msg": "Invalid args",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

#### Response Parameters ####

|Parameter |      Type      |                                                           Description                                                            |
|----------|----------------|----------------------------------------------------------------------------------------------------------------------------------|
|    id    |     String     |                                                 Unique identifier of the message                                                 |
|    op    |     String     |                                                            Operation                                                             |
|   code   |     String     |                                                            Error Code                                                            |
|   msg    |     String     |                                                          Error message                                                           |
|   data   |Array of objects|                                                               Data                                                               |
| \> ordId |     String     |                                                             Order ID                                                             |
|\> clOrdId|     String     |                                            Client Order ID as assigned by the client                                             |
|  \> tag  |     String     |                                                            Order tag                                                             |
|  \> ts   |     String     |Timestamp when the order request processing is finished by our system, Unix timestamp format in milliseconds, e.g. `1597026383085`|
| \> sCode |     String     |                                               Order status code, `0` means success                                               |
| \> sMsg  |     String     |                                         Rejection or success message of event execution.                                         |
|  inTime  |     String     |   Timestamp at Websocket gateway when the request is received, Unix timestamp format in microseconds, e.g. `1597026383085123`    |
| outTime  |     String     |     Timestamp at Websocket gateway when the response is sent, Unix timestamp format in microseconds, e.g. `1597026383085123`     |
 In the `Portfolio Margin` account mode, either all orders are accepted by the system successfully, or all orders are rejected by the system. clOrdId  
clOrdId is a user-defined unique ID used to identify the order. It will be included in the response parameters if you have specified during order submission, and can be used as a request parameter to the endpoints to query, cancel and amend orders.   
clOrdId must be unique among all pending orders and the current request.

---

### WS / Cancel order ###

Cancel an incomplete order

#### URL Path ####

/ws/v5/private (required login)

#### Rate Limit: 60 requests per 2 seconds ####

#### Rate limit rule (except Options): User ID + Instrument ID ####

#### Rate limit rule (Options only): User ID + Instrument Family ####

Rate limit is shared with the `Cancel order` REST API endpoints
>
>
> Request Example
>
>

```
{
  "id": "1514",
  "op": "cancel-order",
  "args": [
    {
      "instId": "BTC-USDT",
      "ordId": "2510789768709120"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |      Type      | Required  |                                                                                                                      Description                                                                                                                       |
|----------|----------------|-----------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    id    |     String     |    Yes    |Unique identifier of the message   <br/>Provided by client. It will be returned in response message for identifying the corresponding request.   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.|
|    op    |     String     |    Yes    |                                                                                                             Operation  <br/>`cancel-order`                                                                                                             |
|   args   |Array of objects|    Yes    |                                                                                                                   Request Parameters                                                                                                                   |
|\> instId |     String     |    Yes    |                                                                                                                     Instrument ID                                                                                                                      |
| \> ordId |     String     |Conditional|                                                                            Order ID   <br/>Either `ordId` or `clOrdId` is required, if both are passed, ordId will be used                                                                             |
|\> clOrdId|     String     |Conditional|                                                  Client Order ID as assigned by the client   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.                                                   |

>
>
> Successful Response Example
>
>

```
{
  "id": "1514",
  "op": "cancel-order",
  "data": [
    {
      "clOrdId": "",
      "ordId": "2510789768709120",
      "ts": "1695190491421",
      "sCode": "0",
      "sMsg": ""
    }
  ],
  "code": "0",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Failure Response Example
>
>

```
{
  "id": "1514",
  "op": "cancel-order",
  "data": [
    {
      "clOrdId": "",
      "ordId": "2510789768709120",
      "ts": "1695190491421",
      "sCode": "5XXXX",
      "sMsg": "Order not exist"
    }
  ],
  "code": "1",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Response Example When Format Error
>
>

```
{
  "id": "1514",
  "op": "cancel-order",
  "data": [],
  "code": "60013",
  "msg": "Invalid args",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

#### Response Parameters ####

|Parameter |      Type      |                                                           Description                                                            |
|----------|----------------|----------------------------------------------------------------------------------------------------------------------------------|
|    id    |     String     |                                                 Unique identifier of the message                                                 |
|    op    |     String     |                                                            Operation                                                             |
|   code   |     String     |                                                            Error Code                                                            |
|   msg    |     String     |                                                          Error message                                                           |
|   data   |Array of objects|                                                               Data                                                               |
| \> ordId |     String     |                                                             Order ID                                                             |
|\> clOrdId|     String     |                                            Client Order ID as assigned by the client                                             |
|  \> ts   |     String     |Timestamp when the order request processing is finished by our system, Unix timestamp format in milliseconds, e.g. `1597026383085`|
| \> sCode |     String     |                                               Order status code, `0` means success                                               |
| \> sMsg  |     String     |                                                       Order status message                                                       |
|  inTime  |     String     |   Timestamp at Websocket gateway when the request is received, Unix timestamp format in microseconds, e.g. `1597026383085123`    |
| outTime  |     String     |     Timestamp at Websocket gateway when the response is sent, Unix timestamp format in microseconds, e.g. `1597026383085123`     |
Cancel order returns with sCode equal to 0. It is not strictly considered that the order has been canceled. It only means that your cancellation request has been accepted by the system server. The result of the cancellation is subject to the state pushed by the order channel or the get order state.

---

### WS / Cancel multiple orders ###

Cancel incomplete orders in batches. Maximum 20 orders can be canceled per request.

#### URL Path ####

/ws/v5/private (required login)

#### Rate Limit: 300 orders per 2 seconds ####

#### Rate limit rule (except Options): User ID + Instrument ID ####

#### Rate limit rule (Options only): User ID + Instrument Family ####

Unlike other endpoints, the rate limit of this endpoint is determined by the number of orders. If there is only one order in the request, it will consume the rate limit of `Cancel order`. Rate limit is shared with the `Cancel multiple orders` REST API endpoints
>
>
> Request Example
>
>

```
{
  "id": "1515",
  "op": "batch-cancel-orders",
  "args": [
    {
      "instId": "BTC-USDT",
      "ordId": "2517748157541376"
    },
    {
      "instId": "LTC-USDT",
      "ordId": "2517748155771904"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |      Type      | Required  |                                                                                                                      Description                                                                                                                       |
|----------|----------------|-----------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    id    |     String     |    Yes    |Unique identifier of the message   <br/>Provided by client. It will be returned in response message for identifying the corresponding request.   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.|
|    op    |     String     |    Yes    |                                                                                                         Operation  <br/>`batch-cancel-orders`                                                                                                          |
|   args   |Array of objects|    Yes    |                                                                                                                   Request Parameters                                                                                                                   |
|\> instId |     String     |    Yes    |                                                                                                                     Instrument ID                                                                                                                      |
| \> ordId |     String     |Conditional|                                                                            Order ID   <br/>Either `ordId` or `clOrdId` is required, if both are passed, ordId will be used                                                                             |
|\> clOrdId|     String     |Conditional|                                                  Client Order ID as assigned by the client   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.                                                   |

>
>
> Response Example When All Succeed
>
>

```
{
  "id": "1515",
  "op": "batch-cancel-orders",
  "data": [
    {
      "clOrdId": "oktswap6",
      "ordId": "2517748157541376",
      "ts": "1695190491421",
      "sCode": "0",
      "sMsg": ""
    },
    {
      "clOrdId": "oktswap7",
      "ordId": "2517748155771904",
      "ts": "1695190491421",
      "sCode": "0",
      "sMsg": ""
    }
  ],
  "code": "0",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Response Example When partially successfully
>
>

```
{
  "id": "1515",
  "op": "batch-cancel-orders",
  "data": [
    {
      "clOrdId": "oktswap6",
      "ordId": "2517748157541376",
      "ts": "1695190491421",
      "sCode": "0",
      "sMsg": ""
    },
    {
      "clOrdId": "oktswap7",
      "ordId": "2517748155771904",
      "ts": "1695190491421",
      "sCode": "5XXXX",
      "sMsg": "order not exist"
    }
  ],
  "code": "2",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Response Example When All Failed
>
>

```
{
  "id": "1515",
  "op": "batch-cancel-orders",
  "data": [
    {
      "clOrdId": "oktswap6",
      "ordId": "2517748157541376",
      "ts": "1695190491421",
      "sCode": "5XXXX",
      "sMsg": "order not exist"
    },
    {
      "clOrdId": "oktswap7",
      "ordId": "2517748155771904",
      "ts": "1695190491421",
      "sCode": "5XXXX",
      "sMsg": "order not exist"
    }
  ],
  "code": "1",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Response Example When Format Error
>
>

```
{
  "id": "1515",
  "op": "batch-cancel-orders",
  "data": [],
  "code": "60013",
  "msg": "Invalid args",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

#### Response Parameters ####

|Parameter |      Type      |                                                           Description                                                            |
|----------|----------------|----------------------------------------------------------------------------------------------------------------------------------|
|    id    |     String     |                                                 Unique identifier of the message                                                 |
|    op    |     String     |                                                            Operation                                                             |
|   code   |     String     |                                                            Error Code                                                            |
|   msg    |     String     |                                                          Error message                                                           |
|   data   |Array of objects|                                                               Data                                                               |
| \> ordId |     String     |                                                             Order ID                                                             |
|\> clOrdId|     String     |                                            Client Order ID as assigned by the client                                             |
|  \> ts   |     String     |Timestamp when the order request processing is finished by our system, Unix timestamp format in milliseconds, e.g. `1597026383085`|
| \> sCode |     String     |                                               Order status code, `0` means success                                               |
| \> sMsg  |     String     |                                                       Order status message                                                       |
|  inTime  |     String     |   Timestamp at Websocket gateway when the request is received, Unix timestamp format in microseconds, e.g. `1597026383085123`    |
| outTime  |     String     |     Timestamp at Websocket gateway when the response is sent, Unix timestamp format in microseconds, e.g. `1597026383085123`     |

---

### WS / Amend order ###

Amend an incomplete order.

#### URL Path ####

/ws/v5/private (required login)

#### Rate Limit: 60 requests per 2 seconds ####

#### Rate Limit of lead instruments for Copy Trading: 4 requests per 2 seconds ####

#### Rate limit rule (except Options): User ID + Instrument ID ####

#### Rate limit rule (Options only): User ID + Instrument Family ####

Rate limit of this endpoint will also be affected by the rules [Sub-account rate limit](/docs-v5/en/#overview-rate-limits-sub-account-rate-limit) and [Fill ratio based sub-account rate limit](/docs-v5/en/#overview-rate-limits-fill-ratio-based-sub-account-rate-limit).

Rate limit is shared with the `Amend order` REST API endpoints
>
>
> Request Example
>
>

```
{
  "id": "1512",
  "op": "amend-order",
  "args": [
    {
      "instId": "BTC-USDT",
      "ordId": "2510789768709120",
      "newSz": "2"
    }
  ]
}

```

#### Request Parameters ####

| Parameter  |      Type      | Required  |                                                                                                                                              Description                                                                                                                                              |
|------------|----------------|-----------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     id     |     String     |    Yes    |                       Unique identifier of the message   <br/>Provided by client. It will be returned in response message for identifying the corresponding request.   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.                        |
|     op     |     String     |    Yes    |                                                                                                                                     Operation  <br/>`amend-order`                                                                                                                                     |
|    args    |Array of objects|    Yes    |                                                                                                                                          Request Parameters                                                                                                                                           |
| \> instId  |     String     |    Yes    |                                                                                                                                             Instrument ID                                                                                                                                             |
|\> cxlOnFail|    Boolean     |    No     |                                                                         Whether the order needs to be automatically canceled when the order amendment fails   <br/>Valid options: `false` or `true`, the default is `false`.                                                                          |
|  \> ordId  |     String     |Conditional|                                                                                                  Order ID   <br/>Either `ordId` or `clOrdId` is required, if both are passed, `ordId` will be used.                                                                                                   |
| \> clOrdId |     String     |Conditional|                                                                                                                               Client Order ID as assigned by the client                                                                                                                               |
|  \> reqId  |     String     |    No     |                                                               Client Request ID as assigned by the client for order amendment   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.                                                               |
|  \> newSz  |     String     |Conditional|                                                New quantity after amendment and it has to be larger than 0. Either `newSz` or `newPx` is required. When amending a partially-filled order, the `newSz` should include the amount that has been filled.                                                |
|  \> newPx  |     String     |Conditional|New price after amendment.   <br/>When modifying options orders, users can only fill in one of the following: newPx, newPxUsd, or newPxVol. It must be consistent with parameters when placing orders. For example, if users placed the order using px, they should use newPx when modifying the order.|
|\> newPxUsd |     String     |Conditional|                                                      Modify options orders using USD prices   <br/>Only applicable to options.   <br/>When modifying options orders, users can only fill in one of the following: newPx, newPxUsd, or newPxVol.                                                       |
|\> newPxVol |     String     |Conditional|                                    Modify options orders based on implied volatility, where 1 represents 100%   <br/>Only applicable to options.   <br/>When modifying options orders, users can only fill in one of the following: newPx, newPxUsd, or newPxVol.                                     |
|  expTime   |     String     |    No     |                                                                                                        Request effective deadline. Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                                                        |

>
>
> Successful Response Example
>
>

```
{
  "id": "1512",
  "op": "amend-order",
  "data": [
    {
      "clOrdId": "",
      "ordId": "2510789768709120",
      "ts": "1695190491421",
      "reqId": "b12344",
      "sCode": "0",
      "sMsg": ""
    }
  ],
  "code": "0",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Failure Response Example
>
>

```
{
  "id": "1512",
  "op": "amend-order",
  "data": [
    {
      "clOrdId": "",
      "ordId": "2510789768709120",
      "ts": "1695190491421",
      "reqId": "b12344",
      "sCode": "5XXXX",
      "sMsg": "order not exist"
    }
  ],
  "code": "1",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Response Example When Format Error
>
>

```
{
  "id": "1512",
  "op": "amend-order",
  "data": [],
  "code": "60013",
  "msg": "Invalid args",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

#### Response Parameters ####

|Parameter |      Type      |                                                           Description                                                            |
|----------|----------------|----------------------------------------------------------------------------------------------------------------------------------|
|    id    |     String     |                                                 Unique identifier of the message                                                 |
|    op    |     String     |                                                            Operation                                                             |
|   code   |     String     |                                                            Error Code                                                            |
|   msg    |     String     |                                                          Error message                                                           |
|   data   |Array of objects|                                                               Data                                                               |
| \> ordId |     String     |                                                             Order ID                                                             |
|\> clOrdId|     String     |                                            Client Order ID as assigned by the client                                             |
|  \> ts   |     String     |Timestamp when the order request processing is finished by our system, Unix timestamp format in milliseconds, e.g. `1597026383085`|
| \> reqId |     String     |                                 Client Request ID as assigned by the client for order amendment                                  |
| \> sCode |     String     |                                               Order status code, `0` means success                                               |
| \> sMsg  |     String     |                                                       Order status message                                                       |
|  inTime  |     String     |   Timestamp at Websocket gateway when the request is received, Unix timestamp format in microseconds, e.g. `1597026383085123`    |
| outTime  |     String     |     Timestamp at Websocket gateway when the response is sent, Unix timestamp format in microseconds, e.g. `1597026383085123`     |
newSz   
If the new quantity of the order is less than or equal to the filled quantity when you are amending a partially-filled order, the order status will be changed to filled. The amend order returns sCode equal to 0. It is not strictly considered that the order has been amended. It only means that your amend order request has been accepted by the system server. The result of the amend is subject to the status pushed by the order channel or the order status query

---

### WS / Amend multiple orders ###

Amend incomplete orders in batches. Maximum 20 orders can be amended per request.

#### URL Path ####

/ws/v5/private (required login)

#### Rate Limit: 300 orders per 2 seconds ####

#### Rate Limit of lead instruments for Copy Trading: 4 orders per 2 seconds ####

#### Rate limit rule (except Options): User ID + Instrument ID ####

#### Rate limit rule (Options only): User ID + Instrument Family ####

Rate limit of this endpoint will also be affected by the rules [Sub-account rate limit](/docs-v5/en/#overview-rate-limits-sub-account-rate-limit) and [Fill ratio based sub-account rate limit](/docs-v5/en/#overview-rate-limits-fill-ratio-based-sub-account-rate-limit).

Unlike other endpoints, the rate limit of this endpoint is determined by the number of orders. If there is only one order in the request, it will consume the rate limit of `Amend order`. Rate limit is shared with the `Amend multiple orders` REST API endpoints
>
>
> Request Example
>
>

```
{
  "id": "1513",
  "op": "batch-amend-orders",
  "args": [
    {
      "instId": "BTC-USDT",
      "ordId": "12345689",
      "newSz": "2"
    },
    {
      "instId": "BTC-USDT",
      "ordId": "12344",
      "newSz": "2"
    }
  ]
}

```

#### Request Parameters ####

| Parameter  |      Type      | Required  |                                                                                                                                              Description                                                                                                                                              |
|------------|----------------|-----------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     id     |     String     |    Yes    |                       Unique identifier of the message   <br/>Provided by client. It will be returned in response message for identifying the corresponding request.   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.                        |
|     op     |     String     |    Yes    |                                                                                                                                 Operation  <br/>`batch-amend-orders`                                                                                                                                  |
|    args    |Array of objects|    Yes    |                                                                                                                                          Request Parameters                                                                                                                                           |
| \> instId  |     String     |    Yes    |                                                                                                                                             Instrument ID                                                                                                                                             |
|\> cxlOnFail|    Boolean     |    No     |                                                                         Whether the order needs to be automatically canceled when the order amendment fails   <br/>Valid options: `false` or `true`, the default is `false`.                                                                          |
|  \> ordId  |     String     |Conditional|                                                                                                  Order ID   <br/>Either `ordId` or `clOrdId` is required, if both are passed, `ordId` will be used.                                                                                                   |
| \> clOrdId |     String     |Conditional|                                                                                                                               Client Order ID as assigned by the client                                                                                                                               |
|  \> reqId  |     String     |    No     |                                                               Client Request ID as assigned by the client for order amendment   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.                                                               |
|  \> newSz  |     String     |Conditional|                                                New quantity after amendment and it has to be larger than 0. Either `newSz` or `newPx` is required. When amending a partially-filled order, the `newSz` should include the amount that has been filled.                                                |
|  \> newPx  |     String     |Conditional|New price after amendment.   <br/>When modifying options orders, users can only fill in one of the following: newPx, newPxUsd, or newPxVol. It must be consistent with parameters when placing orders. For example, if users placed the order using px, they should use newPx when modifying the order.|
|\> newPxUsd |     String     |Conditional|                                                      Modify options orders using USD prices   <br/>Only applicable to options.   <br/>When modifying options orders, users can only fill in one of the following: newPx, newPxUsd, or newPxVol.                                                       |
|\> newPxVol |     String     |Conditional|                                    Modify options orders based on implied volatility, where 1 represents 100%   <br/>Only applicable to options.   <br/>When modifying options orders, users can only fill in one of the following: newPx, newPxUsd, or newPxVol.                                     |
|  expTime   |     String     |    No     |                                                                                                        Request effective deadline. Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                                                        |

>
>
> Response Example When All Succeed
>
>

```
{
  "id": "1513",
  "op": "batch-amend-orders",
  "data": [
    {
      "clOrdId": "oktswap6",
      "ordId": "12345689",
      "ts": "1695190491421",
      "reqId": "b12344",
      "sCode": "0",
      "sMsg": ""
    },
    {
      "clOrdId": "oktswap7",
      "ordId": "12344",
      "ts": "1695190491421",
      "reqId": "b12344",
      "sCode": "0",
      "sMsg": ""
    }
  ],
  "code": "0",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Response Example When All Failed
>
>

```
{
  "id": "1513",
  "op": "batch-amend-orders",
  "data": [
    {
      "clOrdId": "",
      "ordId": "12345689",
      "ts": "1695190491421",
      "reqId": "b12344",
      "sCode": "5XXXX",
      "sMsg": "order not exist"
    },
    {
      "clOrdId": "oktswap7",
      "ordId": "",
      "ts": "1695190491421",
      "reqId": "b12344",
      "sCode": "5XXXX",
      "sMsg": "order not exist"
    }
  ],
  "code": "1",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Response Example When Partially Successful
>
>

```
{
  "id": "1513",
  "op": "batch-amend-orders",
  "data": [
    {
      "clOrdId": "",
      "ordId": "12345689",
      "ts": "1695190491421",
      "reqId": "b12344",
      "sCode": "0",
      "sMsg": ""
    },
    {
      "clOrdId": "oktswap7",
      "ordId": "",
      "ts": "1695190491421",
      "reqId": "b12344",
      "sCode": "5XXXX",
      "sMsg": "order not exist"
    }
  ],
  "code": "2",
  "msg": "",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

>
>
> Response Example When Format Error
>
>

```
{
  "id": "1513",
  "op": "batch-amend-orders",
  "data": [],
  "code": "60013",
  "msg": "Invalid args",
  "inTime": "1695190491421339",
  "outTime": "1695190491423240"
}

```

#### Response Parameters ####

|Parameter |      Type      |                                                                       Description                                                                        |
|----------|----------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|
|    id    |     String     |                                                             Unique identifier of the message                                                             |
|    op    |     String     |                                                                        Operation                                                                         |
|   code   |     String     |                                                                        Error Code                                                                        |
|   msg    |     String     |                                                                      Error message                                                                       |
|   data   |Array of objects|                                                                           Data                                                                           |
| \> ordId |     String     |                                                                         Order ID                                                                         |
|\> clOrdId|     String     |                                                        Client Order ID as assigned by the client                                                         |
|  \> ts   |     String     |            Timestamp when the order request processing is finished by our system, Unix timestamp format in milliseconds, e.g. `1597026383085`            |
| \> reqId |     String     |Client Request ID as assigned by the client for order amendment   <br/>If the user provides reqId in the request, the corresponding reqId will be returned|
| \> sCode |     String     |                                                           Order status code, `0` means success                                                           |
| \> sMsg  |     String     |                                                                   Order status message                                                                   |
|  inTime  |     String     |               Timestamp at Websocket gateway when the request is received, Unix timestamp format in microseconds, e.g. `1597026383085123`                |
| outTime  |     String     |                 Timestamp at Websocket gateway when the response is sent, Unix timestamp format in microseconds, e.g. `1597026383085123`                 |
newSz   
If the new quantity of the order is less than or equal to the filled quantity when you are amending a partially-filled order, the order status will be changed to filled.

---

### WS / Mass cancel order ###

Cancel all the MMP pending orders of an instrument family.  

Only applicable to Option in Portfolio Margin mode, and MMP privilege is required.

#### URL Path ####

/ws/v5/private (required login)

#### Rate Limit: 5 requests per 2 seconds ####

#### Rate limit rule: User ID ####

Rate limit is shared with the `Mass Cancel Order` REST API endpoints
>
>
> Request Example
>
>

```
{
    "id": "1512",
    "op": "mass-cancel",
    "args": [{
        "instType":"OPTION",
        "instFamily":"BTC-USD"
    }]
}

```

#### Request Parameters ####

|   Parameter   |      Type      |Required|                                                                                                                                       Description                                                                                                                                        |
|---------------|----------------|--------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      id       |     String     |  Yes   |                 Unique identifier of the message   <br/>Provided by client. It will be returned in response message for identifying the corresponding request.   <br/>A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.                 |
|      op       |     String     |  Yes   |                                                                                                                              Operation  <br/>`mass-cancel`                                                                                                                               |
|     args      |Array of objects|  Yes   |                                                                                                                                    Request parameters                                                                                                                                    |
|  \> instType  |     String     |  Yes   |                                                                                                                              Instrument type  <br/>`OPTION`                                                                                                                              |
| \> instFamily |     String     |  Yes   |                                                                                                                                    Instrument family                                                                                                                                     |
|\> lockInterval|     String     |   No   |Lock interval(ms)  <br/> The range should be [0, 10 000]  <br/> The default is 0. You can set it as "0" if you want to unlock it immediately.  <br/> Error 54008 will be thorwn when placing order duiring lock interval, it is different from 51034 which is thrown when MMP is triggered|

>
>
> ##### Successful Response Example #####
>
>

```
{
    "id": "1512",
    "op": "mass-cancel",
    "data": [
        {
            "result": true
        }
    ],
    "code": "0",
    "msg": ""
}

```

>
>
> Response Example When Format Error
>
>

```
{
  "id": "1512",
  "op": "mass-cancel",
  "data": [],
  "code": "60013",
  "msg": "Invalid args"
}

```

#### Response Parameters ####

|Parameter|     Type      |             Description             |
|---------|---------------|-------------------------------------|
|   id    |    String     |  Unique identifier of the message   |
|   op    |    String     |              Operation              |
|  code   |    String     |             Error Code              |
|   msg   |    String     |            Error message            |
|  data   |Array of object|                Data                 |
|\> result|    Boolean    |Result of the request `true`, `false`|

---

### WS / Tickers channel ###

Retrieve the last traded price, bid price, ask price and 24-hour trading volume of instruments.   
The fastest rate is 1 update/100ms. There will be no update if the event is not triggered. The events which can trigger update: trade, the change on best ask/bid.

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
      "channel": "tickers",
      "instId": "BTC-USDT"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |      Type      |Required|                  Description                  |
|----------|----------------|--------|-----------------------------------------------|
|    op    |     String     |  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`|
|   args   |Array of objects|  Yes   |          List of subscribed channels          |
|\> channel|     String     |  Yes   |         Channel name  <br/>`tickers`          |
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
    "channel": "tickers",
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
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"tickers\", \"instId\" : \"LTC-USD-200327\"}]}",
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
    "channel": "tickers",
    "instId": "BTC-USDT"
  },
  "data": [
    {
      "instType": "SPOT",
      "instId": "BTC-USDT",
      "last": "9999.99",
      "lastSz": "0.1",
      "askPx": "9999.99",
      "askSz": "11",
      "bidPx": "8888.88",
      "bidSz": "5",
      "open24h": "9000",
      "high24h": "10000",
      "low24h": "8888.88",
      "volCcy24h": "2222",
      "vol24h": "2222",
      "sodUtc0": "2222",
      "sodUtc8": "2222",
      "ts": "1597026383085"
    }
  ]
}

```

#### Push data parameters ####

|**Parameter**|    **Type**    |                                                                                                **Description**                                                                                                 |
|-------------|----------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     arg     |     Object     |                                                                                        Successfully subscribed channel                                                                                         |
| \> channel  |     String     |                                                                                                  Channel name                                                                                                  |
|  \> instId  |     String     |                                                                                                 Instrument ID                                                                                                  |
|    data     |Array of objects|                                                                                                Subscribed data                                                                                                 |
| \> instType |     String     |                                                                                                Instrument type                                                                                                 |
|  \> instId  |     String     |                                                                                                 Instrument ID                                                                                                  |
|   \> last   |     String     |                                                                                               Last traded price                                                                                                |
|  \> lastSz  |     String     |                                                                           Last traded size. 0 represents there is no trading volume                                                                            |
|  \> askPx   |     String     |                                                                                                 Best ask price                                                                                                 |
|  \> askSz   |     String     |                                                                                                 Best ask size                                                                                                  |
|  \> bidPx   |     String     |                                                                                                 Best bid price                                                                                                 |
|  \> bidSz   |     String     |                                                                                                 Best bid size                                                                                                  |
| \> open24h  |     String     |                                                                                        Open price in the past 24 hours                                                                                         |
| \> high24h  |     String     |                                                                                       Highest price in the past 24 hours                                                                                       |
|  \> low24h  |     String     |                                                                                       Lowest price in the past 24 hours                                                                                        |
|\> volCcy24h |     String     |24h trading volume, with a unit of `currency`.   <br/>If it is a `derivatives` contract, the value is the number of base currency.   <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in quote currency.|
|  \> vol24h  |     String     |  24h trading volume, with a unit of `contract`.   <br/>If it is a `derivatives` contract, the value is the number of contracts.   <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in base currency.   |
| \> sodUtc0  |     String     |                                                                                            Open price in the UTC 0                                                                                             |
| \> sodUtc8  |     String     |                                                                                            Open price in the UTC 8                                                                                             |
|    \> ts    |     String     |                                                            Ticker data generation time, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                            |

---

### WS / Candlesticks channel ###

Retrieve the candlesticks data of an instrument. the push frequency is the fastest interval 1 second push the data.

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
      "channel": "candle1D",
      "instId": "BTC-USDT"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |      Type      |Required|                                                                                                                                                                                                                                                      Description                                                                                                                                                                                                                                                       |
|----------|----------------|--------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    op    |     String     |  Yes   |                                                                                                                                                                                                                                    Operation  <br/>`subscribe`  <br/>`unsubscribe`                                                                                                                                                                                                                                     |
|   args   |Array of objects|  Yes   |                                                                                                                                                                                                                                              List of subscribed channels                                                                                                                                                                                                                                               |
|\> channel|     String     |  Yes   |Channel name   <br/>`candle3M`  <br/>`candle1M`  <br/>`candle1W`   <br/>`candle1D`  <br/>`candle2D`  <br/>`candle3D`  <br/>`candle5D`  <br/>`candle12H`  <br/>`candle6H`  <br/>`candle4H`  <br/>`candle2H`  <br/>`candle1H`  <br/>`candle30m`  <br/>`candle15m`  <br/>`candle5m`  <br/>`candle3m`  <br/>`candle1m`  <br/>`candle1s`  <br/>`candle3Mutc`  <br/>`candle1Mutc`  <br/>`candle1Wutc`  <br/>`candle1Dutc`  <br/>`candle2Dutc`  <br/>`candle3Dutc`  <br/>`candle5Dutc`  <br/>`candle12Hutc`  <br/>`candle6Hutc`|
|\> instId |     String     |  Yes   |                                                                                                                                                                                                                                                     Instrument ID                                                                                                                                                                                                                                                      |

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "candle1D",
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
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"candle1D\", \"instId\" : \"BTC-USD-191227\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|                       Description                       |
|----------|------|--------|---------------------------------------------------------|
|  event   |String|  Yes   |Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|   arg    |Object|   No   |                   Subscribed channel                    |
|\> channel|String|  yes   |                      channel name                       |
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
    "channel": "candle1D",
    "instId": "BTC-USDT"
  },
  "data": [
    [
      "1597026383085",
      "8533.02",
      "8553.74",
      "8527.17",
      "8548.26",
      "45247",
      "529.5858061",
      "5529.5858061",
      "0"
    ]
  ]
}

```

#### Push data parameters ####

|**Parameter** |   **Type**    |                                                                                              **Description**                                                                                               |
|--------------|---------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     arg      |    Object     |                                                                                      Successfully subscribed channel                                                                                       |
|  \> channel  |    String     |                                                                                                Channel name                                                                                                |
|  \> instId   |    String     |                                                                                               Instrument ID                                                                                                |
|     data     |Array of Arrays|                                                                                              Subscribed data                                                                                               |
|    \> ts     |    String     |                                                        Opening time of the candlestick, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                        |
|     \> o     |    String     |                                                                                                 Open price                                                                                                 |
|     \> h     |    String     |                                                                                               highest price                                                                                                |
|     \> l     |    String     |                                                                                                Lowest price                                                                                                |
|     \> c     |    String     |                                                                                                Close price                                                                                                 |
|    \> vol    |    String     |  Trading volume, with a unit of `contract`.   <br/>If it is a `derivatives` contract, the value is the number of contracts.   <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in base currency.   |
|  \> volCcy   |    String     |Trading volume, with a unit of `currency`.   <br/>If it is a `derivatives` contract, the value is the number of base currency.   <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in quote currency.|
|\> volCcyQuote|    String     |                  Trading volume, the value is the quantity in quote currency   <br/>e.g. The unit is `USDT` for `BTC-USDT` and `BTC-USDT-SWAP`  <br/>The unit is `USD` for `BTC-USD-SWAP`                  |
|  \> confirm  |    String     |                                                         The state of candlesticks  <br/>`0`: K line is uncompleted  <br/>`1`: K line is completed                                                          |

---

### WS / Trades channel ###

Retrieve the recent trades data. Data will be pushed whenever there is a trade. Every update may aggregate multiple trades.   

The message is sent only once per taker order, per filled price. The count field is used to represent the number of aggregated matches.

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
      "channel": "trades",
      "instId": "BTC-USDT"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |      Type      |Required|                  Description                  |
|----------|----------------|--------|-----------------------------------------------|
|    op    |     String     |  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`|
|   args   |Array of objects|  Yes   |          List of subscribed channels          |
|\> channel|     String     |  Yes   |          Channel name  <br/>`trades`          |
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
      "channel": "trades",
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
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"trades\", \"instId\" : \"BTC-USD-191227\"}]}",
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
    "channel": "trades",
    "instId": "BTC-USDT"
  },
  "data": [
    {
      "instId": "BTC-USDT",
      "tradeId": "130639474",
      "px": "42219.9",
      "sz": "0.12060306",
      "side": "buy",
      "ts": "1630048897897",
      "count": "3"
    }
  ]
}

```

#### Push data parameters ####

|**Parameter**|    **Type**    |                                                      **Description**                                                       |
|-------------|----------------|----------------------------------------------------------------------------------------------------------------------------|
|     arg     |     Object     |                                              Successfully subscribed channel                                               |
| \> channel  |     String     |                                                        Channel name                                                        |
|  \> instId  |     String     |                                                       Instrument ID                                                        |
|    data     |Array of objects|                                                      Subscribed data                                                       |
|  \> instId  |     String     |                                               Instrument ID, e.g. `BTC-USDT`                                               |
| \> tradeId  |     String     |                                        The last trade ID in the trades aggregation                                         |
|    \> px    |     String     |                                                        Trade price                                                         |
|    \> sz    |     String     |Trade quantity   <br/>For spot trading, the unit is base currency  <br/>For `FUTURES`/`SWAP`/`OPTION`, the unit is contract.|
|   \> side   |     String     |                                          Trade direction  <br/>`buy`  <br/>`sell`                                          |
|    \> ts    |     String     |                          Filled time, Unix timestamp format in milliseconds, e.g. `1597026383085`                          |
|  \> count   |     String     |                                               The count of trades aggregated                                               |
Aggregation function description:  
1. The system will send only one message per taker order, per filled price. The `count` field will be used to represent the number of aggregated matches.  
2. The `tradeId` field in the message becomes the last trade ID in the aggregation.  
3. When the `count` = 1, it means the taker order matches only one maker order with the specific price.  
4. When the `count` \> 1, it means the taker order matches multiple maker orders with the same price. For example, if `tradeId` = 123 and `count` = 3, it means the message aggregates the trades of `tradeId` = 123, 122, and 121. Maker side has filled multiple orders.  
5. Users can use this information to compare with data from the `trades-all` channel.  
6. Order book and the aggregated trades data are still published sequentially.

---

### WS / All trades channel ###

Retrieve the recent trades data. Data will be pushed whenever there is a trade. Every update contain only one trade.

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
      "channel": "trades-all",
      "instId": "BTC-USDT"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |     Type      |Required|                  Description                  |
|----------|---------------|--------|-----------------------------------------------|
|    op    |    String     |  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`|
|   args   |Array of object|  Yes   |          List of subscribed channels          |
|\> channel|    String     |  Yes   |        Channel name  <br/>`trades-all`        |
|\> instId |    String     |  Yes   |                 Instrument ID                 |

>
>
> Successful Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
      "channel": "trades-all",
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
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"trades-all\", \"instId\" : \"BTC-USD-191227\"}]}",
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
    "channel": "trades-all",
    "instId": "BTC-USDT"
  },
  "data": [
    {
      "instId": "BTC-USDT",
      "tradeId": "130639474",
      "px": "42219.9",
      "sz": "0.12060306",
      "side": "buy",
      "ts": "1630048897897"
    }
  ]
}

```

#### Push data parameters ####

|**Parameter**|    **Type**    |                                                      **Description**                                                       |
|-------------|----------------|----------------------------------------------------------------------------------------------------------------------------|
|     arg     |     Object     |                                              Successfully subscribed channel                                               |
| \> channel  |     String     |                                                        Channel name                                                        |
|  \> instId  |     String     |                                                       Instrument ID                                                        |
|    data     |Array of objects|                                                      Subscribed data                                                       |
|  \> instId  |     String     |                                               Instrument ID, e.g. `BTC-USDT`                                               |
| \> tradeId  |     String     |                                                          Trade ID                                                          |
|    \> px    |     String     |                                                        Trade price                                                         |
|    \> sz    |     String     |Trade quantity   <br/>For spot trading, the unit is base currency  <br/>For `FUTURES`/`SWAP`/`OPTION`, the unit is contract.|
|   \> side   |     String     |                                          Trade direction  <br/>`buy`  <br/>`sell`                                          |
|    \> ts    |     String     |                          Filled time, Unix timestamp format in milliseconds, e.g. `1597026383085`                          |

---

### WS / Order book channel ###

Retrieve order book data.  

Use `books` for 400 depth levels, `books5` for 5 depth levels, `bbo-tbt` tick-by-tick 1 depth level, `books50-l2-tbt` tick-by-tick 50 depth levels, and `books-l2-tbt` for tick-by-tick 400 depth levels.   

* `books`: 400 depth levels will be pushed in the initial full snapshot. Incremental data will be pushed every 100 ms for the changes in the order book during that period of time.   

* `books5`: 5 depth levels snapshot will be pushed in the initial push. Snapshot data will be pushed every 100 ms when there are changes in the 5 depth levels snapshot.  

* `bbo-tbt`: 1 depth level snapshot will be pushed in the initial push. Snapshot data will be pushed every 10 ms when there are changes in the 1 depth level snapshot.   

* `books-l2-tbt`: 400 depth levels will be pushed in the initial full snapshot. Incremental data will be pushed every 10 ms for the changes in the order book during that period of time.   

* `books50-l2-tbt`: 50 depth levels will be pushed in the initial full snapshot. Incremental data will be pushed every 10 ms for the changes in the order book during that period of time.
* The push sequence for order book channels within the same connection and trading symbols is fixed as: bbo-tbt -\> books-l2-tbt -\> books50-l2-tbt -\> books -\> books5.
* Users can not simultaneously subscribe to `books-l2-tbt` and `books50-l2-tbt/books` channels for the same trading symbol.
  * For more details, please refer to the changelog [2024-07-17](/docs-v5/log_en/#2024-07-17)

Only API users who are VIP5 and above in trading fee tier are allowed to subscribe to "books-l2-tbt" 400 depth channels   
Only API users who are VIP4 and above in trading fee tier are allowed to subscribe to "books50-l2-tbt" 50 depth channels  

Identity verification refers to [Login](/docs-v5/en/#overview-websocket-login)

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
      "channel": "books",
      "instId": "BTC-USDT"
    }
  ]
}

```

#### Request Parameters ####

|Parameter |      Type      |Required|                                             Description                                             |
|----------|----------------|--------|-----------------------------------------------------------------------------------------------------|
|    op    |     String     |  Yes   |                           Operation  <br/>`subscribe`  <br/>`unsubscribe`                           |
|   args   |Array of objects|  Yes   |                                     List of subscribed channels                                     |
|\> channel|     String     |  Yes   |Channel name  <br/>`books`  <br/>`books5`  <br/>`bbo-tbt`  <br/>`books50-l2-tbt`  <br/>`books-l2-tbt`|
|\> instId |     String     |  Yes   |                                            Instrument ID                                            |

>
>
> Response Example
>
>

```
{
  "event": "subscribe",
  "arg": {
    "channel": "books",
    "instId": "BTC-USDT"
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
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"books\", \"instId\" : \"BTC-USD-191227\"}]}",
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
|   msg    |String|   No   |                      Error message                      |
|   code   |String|   No   |                       Error code                        |
|  connId  |String|  Yes   |                 WebSocket connection ID                 |

>
>
> Push Data Example: Full Snapshot
>
>

```
{
  "arg": {
    "channel": "books",
    "instId": "BTC-USDT"
  },
  "action": "snapshot",
  "data": [
    {
      "asks": [
        ["8476.98", "415", "0", "13"],
        ["8477", "7", "0", "2"],
        ["8477.34", "85", "0", "1"],
        ["8477.56", "1", "0", "1"],
        ["8505.84", "8", "0", "1"],
        ["8506.37", "85", "0", "1"],
        ["8506.49", "2", "0", "1"],
        ["8506.96", "100", "0", "2"]
      ],
      "bids": [
        ["8476.97", "256", "0", "12"],
        ["8475.55", "101", "0", "1"],
        ["8475.54", "100", "0", "1"],
        ["8475.3", "1", "0", "1"],
        ["8447.32", "6", "0", "1"],
        ["8447.02", "246", "0", "1"],
        ["8446.83", "24", "0", "1"],
        ["8446", "95", "0", "3"]
      ],
      "ts": "1597026383085",
      "checksum": -855196043,
      "prevSeqId": -1,
      "seqId": 123456
    }
  ]
}

```

>
>
> Push Data Example: Incremental Data
>
>

```
{
  "arg": {
    "channel": "books",
    "instId": "BTC-USDT"
  },
  "action": "update",
  "data": [
    {
      "asks": [
        ["8476.98", "415", "0", "13"],
        ["8477", "7", "0", "2"],
        ["8477.34", "85", "0", "1"],
        ["8477.56", "1", "0", "1"],
        ["8505.84", "8", "0", "1"],
        ["8506.37", "85", "0", "1"],
        ["8506.49", "2", "0", "1"],
        ["8506.96", "100", "0", "2"]
      ],
      "bids": [
        ["8476.97", "256", "0", "12"],
        ["8475.55", "101", "0", "1"],
        ["8475.54", "100", "0", "1"],
        ["8475.3", "1", "0", "1"],
        ["8447.32", "6", "0", "1"],
        ["8447.02", "246", "0", "1"],
        ["8446.83", "24", "0", "1"],
        ["8446", "95", "0", "3"]
      ],
      "ts": "1597026383085",
      "checksum": -855196043,
      "prevSeqId": 123456,
      "seqId": 123457
    }
  ]
}

```

#### Push data parameters ####

|**Parameter**|    **Type**    |                                             **Description**                                             |
|-------------|----------------|---------------------------------------------------------------------------------------------------------|
|     arg     |     Object     |                                     Successfully subscribed channel                                     |
| \> channel  |     String     |                                              Channel name                                               |
|  \> instId  |     String     |                                              Instrument ID                                              |
|   action    |     String     |Push data action, incremental data or full snapshot.   <br/>`snapshot`: full   <br/>`update`: incremental|
|    data     |Array of objects|                                             Subscribed data                                             |
|   \> asks   |Array of Arrays |                                         Order book on sell side                                         |
|   \> bids   |Array of Arrays |                                         Order book on buy side                                          |
|    \> ts    |     String     |         Order book generation time, Unix timestamp format in milliseconds, e.g. `1597026383085`         |
| \> checksum |    Integer     |                                 Checksum, implementation details below                                  |
|\> prevSeqId |    Integer     |   Sequence ID of the last sent message. Only applicable to `books`, `books-l2-tbt`, `books50-l2-tbt`    |
|  \> seqId   |    Integer     |                    Sequence ID of the current message, implementation details below                     |
An example of the array of asks and bids values: ["411.8", "10", "0", "4"]  
\- "411.8" is the depth price  
\- "10" is the quantity at the price (number of contracts for derivatives, quantity in base currency for Spot and Spot Margin)  
\- "0" is part of a deprecated feature and it is always "0"  
\- "4" is the number of orders at the price. If you need to subscribe to many 50 or 400 depth level channels, it is recommended to subscribe through multiple websocket connections, with each of less than 30 channels. The order book data will be updated around once a second during the call auction.

#### Sequence ID ####

`seqId` is the sequence ID of the market data published. The set of sequence ID received by users is the same if users are connecting to the same channel through multiple websocket connections. Each `instId` has an unique set of sequence ID. Users can use `prevSeqId` and `seqId` to build the message sequencing for incremental order book updates. Generally the value of seqId is larger than prevSeqId. The `prevSeqId` in the new message matches with `seqId` of the previous message. The smallest possible sequence ID value is 0, except in snapshot messages where the prevSeqId is always -1.  

Exceptions:  
1. If there are no updates to the depth for an extended period, OKX will send a message with `'asks': [], 'bids': []` to inform users that the connection is still active. `seqId` is the same as the last sent message and `prevSeqId` equals to `seqId`.
2. The sequence number may be reset due to maintenance, and in this case, users will receive an incremental message with `seqId` smaller than `prevSeqId`. However, subsequent messages will follow the regular sequencing rule.

##### Example #####

1. Snapshot message: prevSeqId = -1, seqId = 10
2. Incremental message 1 (normal update): prevSeqId = 10, seqId = 15
3. Incremental message 2 (no update): prevSeqId = 15, seqId = 15
4. Incremental message 3 (sequence reset): prevSeqId = 15, seqId = 3
5. Incremental message 4 (normal update): prevSeqId = 3, seqId = 5

#### Checksum ####

This mechanism can assist users in checking the accuracy of depth data.

##### Merging incremental data into full data #####

After subscribing to the incremental load push (such as `books` 400 levels) of Order Book Channel, users first receive the initial full load of market depth. After the incremental load is subsequently received, update the local full load.

1. If there is the same price, compare the size. If the size is 0, delete this depth data. If the size changes, replace the original data.
2. If there is no same price, sort by price (bid in descending order, ask in ascending order), and insert the depth information into the full load.

##### Calculate Checksum #####

Use the first 25 bids and asks in the full load to form a string (where a colon connects the price and size in an ask or a bid), and then calculate the CRC32 value (32-bit signed integer).

>
>
> Calculate Checksum
>
>

```
1. More than 25 levels of bid and ask
A full load of market depth (only 2 levels of data are shown here, while 25 levels of data should actually be intercepted):

```

```
{
    "bids": [
        ["3366.1", "7", "0", "3"],
        ["3366", "6", "3", "4"]
    ],
    "asks": [
        ["3366.8", "9", "10", "3"],
        ["3368", "8", "3", "4"]
    ]
}

```

```
Check string:
"3366.1:7:3366.8:9:3366:6:3368:8"

2. Less than 25 levels of bid or ask
A full load of market depth:

```

```
{
    "bids": [
        ["3366.1", "7", "0", "3"]
    ],
    "asks": [
        ["3366.8", "9", "10", "3"],
        ["3368", "8", "3", "4"],
        ["3372", "8", "3", "4"]
    ]
}

```

```
Check string:
"3366.1:7:3366.8:9:3368:8:3372:8"

```

1. When the bid and ask depth data exceeds 25 levels, each of them will intercept 25 levels of data, and the string to be checked is queued in a way that the bid and ask depth data are alternately arranged.   
   Such as: `bid[price:size]`:`ask[price:size]`:`bid[price:size]`:`ask[price:size]`...
2. When the bid or ask depth data is less than 25 levels, the missing depth data will be ignored.  
   Such as: `bid[price:size]`:`ask[price:size]`:`asks[price:size]`:`asks[price:size]`...

>
>
> Push Data Example of bbo-tbt channel
>
>

```
{
  "arg": {
    "channel": "bbo-tbt",
    "instId": "BCH-USDT-SWAP"
  },
  "data": [
    {
      "asks": [
        [
          "111.06","55154","0","2"
        ]
      ],
      "bids": [
        [
          "111.05","57745","0","2"
        ]
      ],
      "ts": "1670324386802",
      "seqId": 363996337
    }
  ]
}

```

>
>
> Push Data Example of books5 channel
>
>

```
{
  "arg": {
    "channel": "books5",
    "instId": "BCH-USDT-SWAP"
  },
  "data": [
    {
      "asks": [
        ["111.06","55154","0","2"],
        ["111.07","53276","0","2"],
        ["111.08","72435","0","2"],
        ["111.09","70312","0","2"],
        ["111.1","67272","0","2"]],
      "bids": [
        ["111.05","57745","0","2"],
        ["111.04","57109","0","2"],
        ["111.03","69563","0","2"],
        ["111.02","71248","0","2"],
        ["111.01","65090","0","2"]],
      "instId": "BCH-USDT-SWAP",
      "ts": "1670324386802",
      "seqId": 363996337
    }
  ]
}

```

---

### WS / Option trades channel ###

Retrieve the recent trades data. Data will be pushed whenever there is a trade. Every update contain only one trade.

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
        "channel": "option-trades",
        "instType": "OPTION",
        "instFamily": "BTC-USD"
    }]
}

```

#### Request Parameters ####

|  Parameter  |     Type      | Required  |                                                           Description                                                            |
|-------------|---------------|-----------|----------------------------------------------------------------------------------------------------------------------------------|
|     op      |    String     |    Yes    |                                                    `subscribe` `unsubscribe`                                                     |
|    args     |Array of object|    Yes    |                                                   List of subscribed channels                                                    |
| \> channel  |    String     |    Yes    |                                                Channel name  <br/>`option-trades`                                                |
| \> instType |    String     |    Yes    |                                                    Instrument type, `OPTION`                                                     |
|  \> instId  |    String     |Conditional|Instrument ID, e.g. BTC-USD-221230-4000-C, Either `instId` or `instFamily` is required. If both are passed, `instId` will be used.|
|\> instFamily|    String     |Conditional|                                                 Instrument family, e.g. BTC-USD                                                  |

>
>
> Successful Response Example
>
>

```
{
    "event": "subscribe",
    "arg": {
        "channel": "option-trades",
        "instType": "OPTION",
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
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"option-trades\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|           Description           |
|----------|------|--------|---------------------------------|
|  event   |String|  Yes   |`subscribe` `unsubscribe` `error`|
|   arg    |Object|   No   |       Subscribed channel        |
|\> channel|String|  Yes   |   Channel name  <br/>`status`   |
|   code   |String|   No   |           Error code            |
|   msg    |String|   No   |          Error message          |
|  connId  |String|  Yes   |     WebSocket connection ID     |

>
>
> Push Data Example
>
>

```
{
    "arg": {
        "channel": "option-trades",
        "instType": "OPTION",
        "instFamily": "BTC-USD"
    },
    "data": [
        {
            "fillVol": "0.5066007836914062",
            "fwdPx": "16469.69928595038",
            "idxPx": "16537.2",
            "instFamily": "BTC-USD",
            "instId": "BTC-USD-230224-18000-C",
            "markPx": "0.04690107010619562",
            "optType": "C",
            "px": "0.045",
            "side": "sell",
            "sz": "2",
            "tradeId": "38",
            "ts": "1672286551080"
        }
    ]
}

```

#### Push data parameters ####

|  Parameter  |     Type      |                              Description                               |
|-------------|---------------|------------------------------------------------------------------------|
|     arg     |    Object     |                    Successfully subscribed channel                     |
| \> channel  |    String     |                              Channel name                              |
|    data     |Array of object|                            Subscribed data                             |
|  \> instId  |    String     |                             Instrument ID                              |
|\> instFamily|    String     |                           Instrument family                            |
| \> tradeId  |    String     |                                Trade ID                                |
|    \> px    |    String     |                              Trade price                               |
|    \> sz    |    String     |                 Trade quantity. The unit is contract.                  |
|   \> side   |    String     |                 Trade side   <br/>`buy`   <br/>`sell`                  |
| \> optType  |    String     |                      Option type, C: Call P: Put                       |
| \> fillVol  |    String     |      Implied volatility while trading (Correspond to trade price)      |
|  \> fwdPx   |    String     |                      Forward price while trading                       |
|  \> idxPx   |    String     |                       Index price while trading                        |
|  \> markPx  |    String     |                        Mark price while trading                        |
|    \> ts    |    String     |Trade time, Unix timestamp format in milliseconds, e.g. `1597026383085`.|

---

### WS / Call auction details channel ###

Retrieve call auction details.

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
        "channel": "call-auction-details",
        "instId": "ONDO-USDC"
    }]
}

```

#### Request Parameters ####

|Parameter |      Type      |Required|                  Description                  |
|----------|----------------|--------|-----------------------------------------------|
|    op    |     String     |  Yes   |Operation  <br/>`subscribe`  <br/>`unsubscribe`|
|   args   |Array of objects|  Yes   |          List of subscribed channels          |
|\> channel|     String     |  Yes   |  Channel name   <br/>`call-auction-details`   |
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
      "channel": "call-auction-details",
      "instId": "ONDO-USDC"
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
  "msg": "Invalid request: {\"op\": \"subscribe\", \"argss\":[{ \"channel\" : \"call-auction-details\", \"instId\" : \"BTC-USD-191227\"}]}",
  "connId": "a4d3ae55"
}

```

#### Response parameters ####

|Parameter | Type |Required|                       Description                       |
|----------|------|--------|---------------------------------------------------------|
|  event   |String|  Yes   |Event  <br/>`subscribe`  <br/>`unsubscribe`  <br/>`error`|
|   arg    |Object|   No   |                   Subscribed channel                    |
|\> channel|String|  yes   |                      channel name                       |
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
    "channel": "call-auction-details",
    "instId": "ONDO-USDC"
  },
  "data": [
        {
            "instId": "ONDO-USDC",
            "unmatchedSz": "9988764",
            "eqPx": "0.6",
            "matchedSz": "44978",
            "state": "continuous_trading",
            "auctionEndTime": "1726542000000",
            "ts": "1726542000007"
        }
  ]
}

```

#### Push data parameters ####

|  **Parameter**  |    **Type**    |                              **Description**                              |
|-----------------|----------------|---------------------------------------------------------------------------|
|       arg       |     Object     |                      Successfully subscribed channel                      |
|   \> channel    |     String     |                               Channel name                                |
|    \> instId    |     String     |                               Instrument ID                               |
|      data       |Array of objects|                              Subscribed data                              |
|    \> instId    |     String     |                               Instrument ID                               |
|     \> eqPx     |     String     |                             Equilibrium price                             |
|  \> matchedSz   |     String     |   Matched size for both buy and sell  <br/>The unit is in base currency   |
| \> unmatchedSz  |     String     |                              Unmatched size                               |
|\> auctionEndTime|     String     |          Call auction end time. Unix timestamp in milliseconds.           |
|    \> state     |     String     |Trading state of the symbol  <br/>`call_auction`  <br/>`continuous_trading`|
|      \> ts      |     String     |          Data generation time. Unix timestamp in millieseconds.           |
During call auction, users can get the updates of equilibrium price, matched size, unmatched size, and auction end time. The data will be updated around once a second. When call auction ends, this channel will push the last message, returning the actual open price, matched size, and unmatched size, with trading state as `continuous\_trading`.

---

