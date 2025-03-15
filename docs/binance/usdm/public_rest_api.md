# Binance Derivatives USDM REST API Documentation

## QUICK START

Quick Start
==========

API Key Setup[​](/docs/derivatives/quick-start#api-key-setup)
----------

* Some endpoints will require an API Key. Please refer to [this page](https://www.binance.com/en/support/faq/how-to-create-api-keys-on-binance-360002502072) regarding API key creation.
* Once API key is created, it is recommended to set IP restrictions on the key for security reasons.
* **Never share your API key/secret key to ANYONE.**

If the API keys were accidentally shared, please delete them immediately and create a new key.

API Key Restrictions[​](/docs/derivatives/quick-start#api-key-restrictions)
----------

* After creating the API key, the default restrictions is `Enable Reading`.
* To **enable withdrawals via the API**, the API key restriction needs to be modified through the Binance UI.

Enabling Accounts[​](/docs/derivatives/quick-start#enabling-accounts)
----------

### Account[​](/docs/derivatives/quick-start#account) ###

A `SPOT` account is provided by default upon creation of a Binance Account.

### Futures Account[​](/docs/derivatives/quick-start#futures-account) ###

To enable a `FUTURES` account for Futures Trading, please refer to the [Futures Trading Guide](https://www.binance.com/en/support/faq/a-beginner-s-guide-to-futures-trading-website-360039304272)

### Futures Testnet[​](/docs/derivatives/quick-start#futures-testnet) ###

Users can use the Futures Testnet to practice `FUTURES` trading.

Currently, this is only available via the API.

Please refer to the [Futures Testnet page](https://testnet.binancefuture.com/en/futures/BTCUSDT) for more information and how to set up the Testnet API key.

### Option Account[​](/docs/derivatives/quick-start#option-account) ###

To enable a `OPTION` account for Option Trading, please refer to the [Option Trading Guide](https://www.binance.com/en/support/faq/introduction-to-binance-options-374321c9317c473480243365298b8706)

API Library[​](/docs/derivatives/quick-start#api-library)
----------

### Python connector[​](/docs/derivatives/quick-start#python-connector) ###

This is a lightweight library that works as a connector to Binance public API, written in Python.

[https://github.com/binance/binance-futures-connector-python](https://github.com/binance/binance-futures-connector-python)

### Java connector[​](/docs/derivatives/quick-start#java-connector) ###

This is a lightweight library that works as a connector to Binance public API, written for Java users.

[https://github.com/binance/binance-futures-connector-java](https://github.com/binance/binance-futures-connector-java)

## GENERAL INFO

General Info
==========

testnet[​](/docs/derivatives/usds-margined-futures/general-info#testnet)
----------

* Most of the endpoints can be used in the testnet platform.
* The REST baseurl for **testnet** is "[https://testnet.binancefuture.com](https://testnet.binancefuture.com)"
* The Websocket baseurl for **testnet** is "wss://stream.binancefuture.com"

SDK and Code Demonstration[​](/docs/derivatives/usds-margined-futures/general-info#sdk-and-code-demonstration)
----------

**Disclaimer:**

* The following SDKs are provided by partners and users, and are **not officially** produced. They are only used to help users become familiar with the API endpoint. Please use it with caution and expand R&D according to your own situation.
* Binance does not make any commitment to the safety and performance of the SDKs, nor will be liable for the risks or even losses caused by using the SDKs.

### Python3[​](/docs/derivatives/usds-margined-futures/general-info#python3) ###

**SDK:**To get the provided SDK for Binance Futures Connector,  
please visit [https://github.com/binance/binance-futures-connector-python](https://github.com/binance/binance-futures-connector-python),  
or use the command below:  
`pip install binance-futures-connector`

### Java[​](/docs/derivatives/usds-margined-futures/general-info#java) ###

To get the provided SDK for Binance Futures,  
please visit [https://github.com/binance/binance-futures-connector-java](https://github.com/binance/binance-futures-connector-java),  
or use the command below:  
`git clone https://github.com/binance/binance-futures-connector-java.git`

General API Information[​](/docs/derivatives/usds-margined-futures/general-info#general-api-information)
----------

* Some endpoints will require an API Key. Please refer to [this page](https://www.binance.com/en/support/articles/360002502072)
* The base endpoint is: **[https://fapi.binance.com](https://fapi.binance.com)**
* All endpoints return either a JSON object or array.
* Data is returned in **ascending** order. Oldest first, newest last.
* All time and timestamp related fields are in milliseconds.
* All data types adopt definition in JAVA.

### HTTP Return Codes[​](/docs/derivatives/usds-margined-futures/general-info#http-return-codes) ###

* HTTP `4XX` return codes are used for for malformed requests;
  the issue is on the sender's side.
* HTTP `403` return code is used when the WAF Limit (Web Application Firewall) has been violated.
* HTTP `408` return code is used when a timeout has occurred while waiting for a response from the backend server.
* HTTP `429` return code is used when breaking a request rate limit.
* HTTP `418` return code is used when an IP has been auto-banned for continuing to send requests after receiving `429` codes.
* HTTP `5XX` return codes are used for internal errors; the issue is on
  Binance's side.
  1. If there is an error message **"Request occur unknown error."**, please retry later.

* HTTP `503` return code is used when:
  1. If there is an error message **"Unknown error, please check your request or try again later."** returned in the response, the API successfully sent the request but not get a response within the timeout period.  
     It is important to **NOT** treat this as a failure operation; the execution status is **UNKNOWN** and could have been a success;
  2. If there is an error message **"Service Unavailable."** returned in the response, it means this is a failure API operation and the service might be unavailable at the moment, you need to retry later.
  3. If there is an error message **"Internal error; unable to process your request. Please try again."** returned in the response, it means this is a failure API operation and you can resend your request if you need.

### Error Codes and Messages[​](/docs/derivatives/usds-margined-futures/general-info#error-codes-and-messages) ###

* Any endpoint can return an ERROR

>
>
> ***The error payload is as follows:***
>
>

```
{  "code": -1121,  "msg": "Invalid symbol."}
```

* Specific error codes and messages defined in [Error Codes](/docs/derivatives/usds-margined-futures/general-info#error-codes).

### General Information on Endpoints[​](/docs/derivatives/usds-margined-futures/general-info#general-information-on-endpoints) ###

* For `GET` endpoints, parameters must be sent as a `query string`.
* For `POST`, `PUT`, and `DELETE` endpoints, the parameters may be sent as a`query string` or in the `request body` with content type`application/x-www-form-urlencoded`. You may mix parameters between both the`query string` and `request body` if you wish to do so.
* Parameters may be sent in any order.
* If a parameter sent in both the `query string` and `request body`, the`query string` parameter will be used.

LIMITS[​](/docs/derivatives/usds-margined-futures/general-info#limits)
----------

* The `/fapi/v1/exchangeInfo` `rateLimits` array contains objects related to the exchange's `RAW_REQUEST`, `REQUEST_WEIGHT`, and `ORDER` rate limits. These are further defined in the `ENUM definitions` section under `Rate limiters (rateLimitType)`.
* A `429` will be returned when either rate limit is violated.

Binance has the right to further tighten the rate limits on users with intent to attack.

### IP Limits[​](/docs/derivatives/usds-margined-futures/general-info#ip-limits) ###

* Every request will contain `X-MBX-USED-WEIGHT-(intervalNum)(intervalLetter)` in the response headers which has the current used weight for the IP for all request rate limiters defined.
* Each route has a `weight` which determines for the number of requests each endpoint counts for. Heavier endpoints and endpoints that do operations on multiple symbols will have a heavier `weight`.
* When a 429 is received, it's your obligation as an API to back off and not spam the API.
* **Repeatedly violating rate limits and/or failing to back off after receiving 429s will result in an automated IP ban (HTTP status 418).**
* IP bans are tracked and **scale in duration** for repeat offenders, **from 2 minutes to 3 days**.
* **The limits on the API are based on the IPs, not the API keys.**

It is strongly recommended to use websocket stream for getting data as much as possible, which can not only ensure the timeliness of the message, but also reduce the access restriction pressure caused by the request.

### Order Rate Limits[​](/docs/derivatives/usds-margined-futures/general-info#order-rate-limits) ###

* Every order response will contain a `X-MBX-ORDER-COUNT-(intervalNum)(intervalLetter)` header which has the current order count for the account for all order rate limiters defined.
* Rejected/unsuccessful orders are not guaranteed to have `X-MBX-ORDER-COUNT-**` headers in the response.
* **The order rate limit is counted against each account**.

Endpoint Security Type[​](/docs/derivatives/usds-margined-futures/general-info#endpoint-security-type)
----------

* Each endpoint has a security type that determines the how you will
  interact with it.
* API-keys are passed into the Rest API via the `X-MBX-APIKEY`header.
* API-keys and secret-keys **are case sensitive**.
* API-keys can be configured to only access certain types of secure endpoints.
  For example, one API-key could be used for TRADE only, while another API-key
  can access everything except for TRADE routes.
* By default, API-keys can access all secure routes.

|Security Type|                      Description                       |
|-------------|--------------------------------------------------------|
|    NONE     |            Endpoint can be accessed freely.            |
|    TRADE    |Endpoint requires sending a valid API-Key and signature.|
| USER\_DATA  |Endpoint requires sending a valid API-Key and signature.|
|USER\_STREAM |       Endpoint requires sending a valid API-Key.       |
|MARKET\_DATA |       Endpoint requires sending a valid API-Key.       |

* `TRADE` and `USER_DATA` endpoints are `SIGNED` endpoints.

SIGNED (TRADE and USER\_DATA) Endpoint Security[​](/docs/derivatives/usds-margined-futures/general-info#signed-trade-and-user_data-endpoint-security)
----------

* `SIGNED` endpoints require an additional parameter, `signature`, to be
  sent in the `query string` or `request body`.
* Endpoints use `HMAC SHA256` signatures. The `HMAC SHA256 signature` is a keyed `HMAC SHA256` operation.
  Use your `secretKey` as the key and `totalParams` as the value for the HMAC operation.
* The `signature` is **not case sensitive**.
* Please make sure the `signature` is the end part of your `query string` or `request body`.
* `totalParams` is defined as the `query string` concatenated with the`request body`.

### Timing Security[​](/docs/derivatives/usds-margined-futures/general-info#timing-security) ###

* A `SIGNED` endpoint also requires a parameter, `timestamp`, to be sent which
  should be the millisecond timestamp of when the request was created and sent.
* An additional parameter, `recvWindow`, may be sent to specify the number of
  milliseconds after `timestamp` the request is valid for. If `recvWindow`is not sent, **it defaults to 5000**.

>
>
> The logic is as follows:
>
>

```
if (timestamp < serverTime + 1000 && serverTime - timestamp <= recvWindow) {  // process request} else {  // reject request}
```

**Serious trading is about timing.** Networks can be unstable and unreliable,
which can lead to requests taking varying amounts of time to reach the
servers. With `recvWindow`, you can specify that the request must be
processed within a certain number of milliseconds or be rejected by the
server.

It is recommended to use a small recvWindow of 5000 or less!

### SIGNED Endpoint Examples for POST /fapi/v1/order - HMAC Keys[​](/docs/derivatives/usds-margined-futures/general-info#signed-endpoint-examples-for-post-fapiv1order---hmac-keys) ###

Here is a step-by-step example of how to send a vaild signed payload from the
Linux command line using `echo`, `openssl`, and `curl`.

|   Key   |                             Value                              |
|---------|----------------------------------------------------------------|
| apiKey  |dbefbc809e3e83c283a984c3a1459732ea7db1360ca80c5c2c8867408d28cc83|
|secretKey|2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9|

| Parameter |    Value    |
|-----------|-------------|
|  symbol   |   BTCUSDT   |
|   side    |     BUY     |
|   type    |    LIMIT    |
|timeInForce|     GTC     |
| quantity  |      1      |
|   price   |    9000     |
|recvWindow |    5000     |
| timestamp |1591702613943|

#### Example 1: As a query string[​](/docs/derivatives/usds-margined-futures/general-info#example-1-as-a-query-string) ####

>
>
> **Example 1**
>
>

>
>
> **HMAC SHA256 signature:**
>
>

```
    $ echo -n "symbol=BTCUSDT&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943" | openssl dgst -sha256 -hmac "2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9"    (stdin)= 3c661234138461fcc7a7d8746c6558c9842d4e10870d2ecbedf7777cad694af9
```

>
>
> **curl command:**
>
>

```
    (HMAC SHA256)    $ curl -H "X-MBX-APIKEY: dbefbc809e3e83c283a984c3a1459732ea7db1360ca80c5c2c8867408d28cc83" -X POST 'https://fapi/binance.com/fapi/v1/order?symbol=BTCUSDT&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943&signature= 3c661234138461fcc7a7d8746c6558c9842d4e10870d2ecbedf7777cad694af9'
```

* **queryString:**

  symbol=BTCUSDT  
  &side=BUY  
  &type=LIMIT  
  &timeInForce=GTC  
  &quantity=1  
  &price=9000  
  &recvWindow=5000  
  &timestamp=1591702613943

#### Example 2: As a request body[​](/docs/derivatives/usds-margined-futures/general-info#example-2-as-a-request-body) ####

>
>
> **Example 2**
>
>

>
>
> **HMAC SHA256 signature:**
>
>

```
    $ echo -n "symbol=BTCUSDT&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943" | openssl dgst -sha256 -hmac "2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9"    (stdin)= 3c661234138461fcc7a7d8746c6558c9842d4e10870d2ecbedf7777cad694af9
```

>
>
> **curl command:**
>
>

```
    (HMAC SHA256)    $ curl -H "X-MBX-APIKEY: dbefbc809e3e83c283a984c3a1459732ea7db1360ca80c5c2c8867408d28cc83" -X POST 'https://fapi/binance.com/fapi/v1/order' -d 'symbol=BTCUSDT&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943&signature= 3c661234138461fcc7a7d8746c6558c9842d4e10870d2ecbedf7777cad694af9'
```

* **requestBody:**

  symbol=BTCUSDT  
  &side=BUY  
  &type=LIMIT  
  &timeInForce=GTC  
  &quantity=1  
  &price=9000  
  &recvWindow=5000  
  &timestamp=1591702613943

#### Example 3: Mixed query string and request body[​](/docs/derivatives/usds-margined-futures/general-info#example-3-mixed-query-string-and-request-body) ####

>
>
> **Example 3**
>
>

>
>
> **HMAC SHA256 signature:**
>
>

```
    $ echo -n "symbol=BTCUSDT&side=BUY&type=LIMIT&timeInForce=GTCquantity=1&price=9000&recvWindow=5000&timestamp= 1591702613943" | openssl dgst -sha256 -hmac "2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9"    (stdin)= f9d0ae5e813ef6ccf15c2b5a434047a0181cb5a342b903b367ca6d27a66e36f2
```

>
>
> **curl command:**
>
>

```
    (HMAC SHA256)    $ curl -H "X-MBX-APIKEY: dbefbc809e3e83c283a984c3a1459732ea7db1360ca80c5c2c8867408d28cc83" -X POST 'https://fapi.binance.com/fapi/v1/order?symbol=BTCUSDT&side=BUY&type=LIMIT&timeInForce=GTC' -d 'quantity=1&price=9000&recvWindow=5000&timestamp=1591702613943&signature=f9d0ae5e813ef6ccf15c2b5a434047a0181cb5a342b903b367ca6d27a66e36f2'
```

* **queryString:** symbol=BTCUSDT&side=BUY&type=LIMIT&timeInForce=GTC
* **requestBody:** quantity=1&price=9000&recvWindow=5000&timestamp= 1591702613943

Note that the signature is different in example 3.  
There is no & between "GTC" and "quantity=1".

### SIGNED Endpoint Examples for POST /fapi/v1/order - RSA Keys[​](/docs/derivatives/usds-margined-futures/general-info#signed-endpoint-examples-for-post-fapiv1order---rsa-keys) ###

* This will be a step by step process how to create the signature payload to send a valid signed payload.
* We support `PKCS#8` currently.
* To get your API key, you need to upload your RSA Public Key to your account and a corresponding API key will be provided for you.

For this example, the private key will be referenced as `test-prv-key.pem`

| Key  |                             Value                              |
|------|----------------------------------------------------------------|
|apiKey|vE3BDAL1gP1UaexugRLtteaAHg3UO8Nza20uexEuW1Kh3tVwQfFHdAiyjjY428o2|

|Parameter |    Value    |
|----------|-------------|
|  symbol  |   BTCUSDT   |
|   side   |    SELL     |
|   type   |   MARKET    |
| quantity |    1.23     |
|recvWindow|   9999999   |
|timestamp |1671090801999|

>
>
> **Signature payload (with the listed parameters):**
>
>

```
timestamp=1671090801999&recvWindow=9999999&symbol=BTCUSDT&side=SELL&type=MARKET&quantity=1.23
```

**Step 1: Construct the payload**

Arrange the list of parameters into a string. Separate each parameter with a `&`.

**Step 2: Compute the signature:**

2.1 - Encode signature payload as ASCII data.

>
>
> **Step 2.2**
>
>

```
 $ echo -n 'timestamp=1671090801999&recvWindow=9999999&symbol=BTCUSDT&side=SELL&type=MARKET&quantity=1.23' | openssl dgst -keyform PEM -sha256 -sign ./test-prv-key.pem
```

2.2 - Sign payload using RSASSA-PKCS1-v1\_5 algorithm with SHA-256 hash function.

>
>
> **Step 2.3**
>
>

```
$ echo -n 'timestamp=1671090801999&recvWindow=9999999&symbol=BTCUSDT&side=SELL&type=MARKET&quantity=1.23' | openssl dgst -keyform PEM -sha256 -sign ./test-prv-key.pem | openssl enc -base64aap36wD5loVXizxvvPI3wz9Cjqwmb3KVbxoym0XeWG1jZq8umqrnSk8H8dkLQeySjgVY91Ufs%2BBGCW%2B4sZjQEpgAfjM76riNxjlD3coGGEsPsT2lG39R%2F1q72zpDs8pYcQ4A692NgHO1zXcgScTGgdkjp%2Brp2bcddKjyz5XBrBM%3D
```

2.3 - Encode output as base64 string.

>
>
> **Step 2.4**
>
>

```
$  echo -n 'timestamp=1671090801999&recvWindow=9999999&symbol=BTCUSDT&side=SELL&type=MARKET&quantity=1.23' | openssl dgst -keyform PEM -sha256 -sign ./test-prv-key.pem | openssl enc -base64 | tr -d '\n'aap36wD5loVXizxvvPI3wz9Cjqwmb3KVbxoym0XeWG1jZq8umqrnSk8H8dkLQeySjgVY91Ufs%2BBGCW%2B4sZjQEpgAfjM76riNxjlD3coGGEsPsT2lG39R%2F1q72zpDs8pYcQ4A692NgHO1zXcgScTGgdkjp%2Brp2bcddKjyz5XBrBM%3D
```

2.4 - Delete any newlines in the signature.

>
>
> **Step 2.5**
>
>

```
aap36wD5loVXizxvvPI3wz9Cjqwmb3KVbxoym0XeWG1jZq8umqrnSk8H8dkLQeySjgVY91Ufs%2BBGCW%2B4sZjQEpgAfjM76riNxjlD3coGGEsPsT2lG39R%2F1q72zpDs8pYcQ4A692NgHO1zXcgScTGgdkjp%2Brp2bcddKjyz5XBrBM%3D
```

2.5 - Since the signature may contain `/` and `=`, this could cause issues with sending the request. So the signature has to be URL encoded.

>
>
> **Step 2.6**
>
>

```
 curl -H "X-MBX-APIKEY: vE3BDAL1gP1UaexugRLtteaAHg3UO8Nza20uexEuW1Kh3tVwQfFHdAiyjjY428o2" -X POST 'https://fapi.binance.com/fapi/v1/order?timestamp=1671090801999&recvWindow=9999999&symbol=BTCUSDT&side=SELL&type=MARKET&quantity=1.23&signature=aap36wD5loVXizxvvPI3wz9Cjqwmb3KVbxoym0XeWG1jZq8umqrnSk8H8dkLQeySjgVY91Ufs%2BBGCW%2B4sZjQEpgAfjM76riNxjlD3coGGEsPsT2lG39R%2F1q72zpDs8pYcQ4A692NgHO1zXcgScTGgdkjp%2Brp2bcddKjyz5XBrBM%3D'
```

2.6 - curl command

>
>
> **Bash script**
>
>

```
#!/usr/bin/env bash# Set up authentication:apiKey="vE3BDAL1gP1UaexugRLtteaAHg3UO8Nza20uexEuW1Kh3tVwQfFHdAiyjjY428o2"   ### REPLACE THIS WITH YOUR API KEY# Set up the request:apiMethod="POST"apiCall="v1/order"apiParams="timestamp=1671090801999&recvWindow=9999999&symbol=BTCUSDT&side=SELL&type=MARKET&quantity=1.23"function rawurlencode {    local value="$1"    local len=${#value}    local encoded=""    local pos c o    for (( pos=0 ; pos<len ; pos++ ))    do        c=${value:$pos:1}        case "$c" in            [-_.~a-zA-Z0-9] ) o="${c}" ;;            * )   printf -v o '%%%02x' "'$c"        esac        encoded+="$o"    done    echo "$encoded"}ts=$(date +%s000)paramsWithTs="$apiParams&timestamp=$ts"rawSignature=$(echo -n "$paramsWithTs" \               | openssl dgst -keyform PEM -sha256 -sign ./test-prv-key.pem \  ### THIS IS YOUR PRIVATE KEY. DO NOT SHARE THIS FILE WITH ANYONE.               | openssl enc -base64 \               | tr -d '\n')signature=$(rawurlencode "$rawSignature")curl -H "X-MBX-APIKEY: $apiKey" -X $apiMethod \    "https://fapi.binance.com/fapi/$apiCall?$paramsWithTs&signature=$signature"
```

A sample Bash script containing similar steps is available in the right side.

---

Postman Collections[​](/docs/derivatives/usds-margined-futures/general-info#postman-collections)
----------

There is now a Postman collection containing the API endpoints for quick and easy use.

For more information please refer to this page: [Binance API Postman](https://github.com/binance-exchange/binance-api-postman)

## COMMON DEFINITION

Public Endpoints Info
==========

Terminology[​](/docs/derivatives/usds-margined-futures/common-definition#terminology)
----------

* `base asset` refers to the asset that is the `quantity` of a symbol.
* `quote asset` refers to the asset that is the `price` of a symbol.

ENUM definitions[​](/docs/derivatives/usds-margined-futures/common-definition#enum-definitions)
----------

**Symbol type:**

* FUTURE

**Contract type (contractType):**

* PERPETUAL
* CURRENT\_MONTH
* NEXT\_MONTH
* CURRENT\_QUARTER
* NEXT\_QUARTER
* PERPETUAL\_DELIVERING

**Contract status (contractStatus, status):**

* PENDING\_TRADING
* TRADING
* PRE\_DELIVERING
* DELIVERING
* DELIVERED
* PRE\_SETTLE
* SETTLING
* CLOSE

**Order status (status):**

* NEW
* PARTIALLY\_FILLED
* FILLED
* CANCELED
* REJECTED
* EXPIRED
* EXPIRED\_IN\_MATCH

**Order types (orderTypes, type):**

* LIMIT
* MARKET
* STOP
* STOP\_MARKET
* TAKE\_PROFIT
* TAKE\_PROFIT\_MARKET
* TRAILING\_STOP\_MARKET

**Order side (side):**

* BUY
* SELL

**Position side (positionSide):**

* BOTH
* LONG
* SHORT

**Time in force (timeInForce):**

* GTC - Good Till Cancel(GTC order valitidy is 1 year from placement)
* IOC - Immediate or Cancel
* FOK - Fill or Kill
* GTX - Good Till Crossing (Post Only)
* GTD - Good Till Date

**Working Type (workingType)**

* MARK\_PRICE
* CONTRACT\_PRICE

**Response Type (newOrderRespType)**

* ACK
* RESULT

**Kline/Candlestick chart intervals:**

m -\> minutes; h -\> hours; d -\> days; w -\> weeks; M -\> months

* 1m
* 3m
* 5m
* 15m
* 30m
* 1h
* 2h
* 4h
* 6h
* 8h
* 12h
* 1d
* 3d
* 1w
* 1M

**STP MODE (selfTradePreventionMode):**

* EXPIRE\_TAKER
* EXPIRE\_BOTH
* EXPIRE\_MAKER

**Price Match (priceMatch)**

* NONE (No price match)
* OPPONENT (counterparty best price)
* OPPONENT\_5 (the 5th best price from the counterparty)
* OPPONENT\_10 (the 10th best price from the counterparty)
* OPPONENT\_20 (the 20th best price from the counterparty)
* QUEUE (the best price on the same side of the order book)
* QUEUE\_5 (the 5th best price on the same side of the order book)
* QUEUE\_10 (the 10th best price on the same side of the order book)
* QUEUE\_20 (the 20th best price on the same side of the order book)

**Rate limiters (rateLimitType)**

>
>
> REQUEST\_WEIGHT
>
>

```
  {  	"rateLimitType": "REQUEST_WEIGHT",  	"interval": "MINUTE",  	"intervalNum": 1,  	"limit": 2400  }
```

>
>
> ORDERS
>
>

```
  {  	"rateLimitType": "ORDERS",  	"interval": "MINUTE",  	"intervalNum": 1,  	"limit": 1200   }
```

* REQUEST\_WEIGHT

* ORDERS

**Rate limit intervals (interval)**

* MINUTE

Filters
==========

Filters define trading rules on a symbol or an exchange.

Symbol filters[​](/docs/derivatives/usds-margined-futures/common-definition#symbol-filters)
----------

### PRICE\_FILTER[​](/docs/derivatives/usds-margined-futures/common-definition#price_filter) ###

>
>
> **/exchangeInfo format:**
>
>

```
  {    "filterType": "PRICE_FILTER",    "minPrice": "0.00000100",    "maxPrice": "100000.00000000",    "tickSize": "0.00000100"  }
```

The `PRICE_FILTER` defines the `price` rules for a symbol. There are 3 parts:

* `minPrice` defines the minimum `price`/`stopPrice` allowed; disabled on `minPrice` == 0.
* `maxPrice` defines the maximum `price`/`stopPrice` allowed; disabled on `maxPrice` == 0.
* `tickSize` defines the intervals that a `price`/`stopPrice` can be increased/decreased by; disabled on `tickSize` == 0.

Any of the above variables can be set to 0, which disables that rule in the `price filter`. In order to pass the `price filter`, the following must be true for `price`/`stopPrice` of the enabled rules:

* `price` \>= `minPrice`
* `price` \<= `maxPrice`
* (`price`-`minPrice`) % `tickSize` == 0

### LOT\_SIZE[​](/docs/derivatives/usds-margined-futures/common-definition#lot_size) ###

>
>
> **/exchangeInfo format:**
>
>

```
  {    "filterType": "LOT_SIZE",    "minQty": "0.00100000",    "maxQty": "100000.00000000",    "stepSize": "0.00100000"  }
```

The `LOT_SIZE` filter defines the `quantity` (aka "lots" in auction terms) rules for a symbol. There are 3 parts:

* `minQty` defines the minimum `quantity` allowed.
* `maxQty` defines the maximum `quantity` allowed.
* `stepSize` defines the intervals that a `quantity` can be increased/decreased by.

In order to pass the `lot size`, the following must be true for `quantity`:

* `quantity` \>= `minQty`
* `quantity` \<= `maxQty`
* (`quantity`-`minQty`) % `stepSize` == 0

### MARKET\_LOT\_SIZE[​](/docs/derivatives/usds-margined-futures/common-definition#market_lot_size) ###

>
>
> **/exchangeInfo format:**
>
>

```
  {    "filterType": "MARKET_LOT_SIZE",    "minQty": "0.00100000",    "maxQty": "100000.00000000",    "stepSize": "0.00100000"  }
```

The `MARKET_LOT_SIZE` filter defines the `quantity` (aka "lots" in auction terms) rules for `MARKET` orders on a symbol. There are 3 parts:

* `minQty` defines the minimum `quantity` allowed.
* `maxQty` defines the maximum `quantity` allowed.
* `stepSize` defines the intervals that a `quantity` can be increased/decreased by.

In order to pass the `market lot size`, the following must be true for `quantity`:

* `quantity` \>= `minQty`
* `quantity` \<= `maxQty`
* (`quantity`-`minQty`) % `stepSize` == 0

### MAX\_NUM\_ORDERS[​](/docs/derivatives/usds-margined-futures/common-definition#max_num_orders) ###

>
>
> **/exchangeInfo format:**
>
>

```
  {    "filterType": "MAX_NUM_ORDERS",    "limit": 200  }
```

The `MAX_NUM_ORDERS` filter defines the maximum number of orders an account is allowed to have open on a symbol.

Note that both "algo" orders and normal orders are counted for this filter.

### MAX\_NUM\_ALGO\_ORDERS[​](/docs/derivatives/usds-margined-futures/common-definition#max_num_algo_orders) ###

>
>
> **/exchangeInfo format:**
>
>

```
  {    "filterType": "MAX_NUM_ALGO_ORDERS",    "limit": 100  }
```

The `MAX_NUM_ALGO_ORDERS ` filter defines the maximum number of all kinds of algo orders an account is allowed to have open on a symbol.

The algo orders include `STOP`, `STOP_MARKET`, `TAKE_PROFIT`, `TAKE_PROFIT_MARKET`, and `TRAILING_STOP_MARKET` orders.

### PERCENT\_PRICE[​](/docs/derivatives/usds-margined-futures/common-definition#percent_price) ###

>
>
> **/exchangeInfo format:**
>
>

```
  {    "filterType": "PERCENT_PRICE",    "multiplierUp": "1.1500",    "multiplierDown": "0.8500",    "multiplierDecimal": 4  }
```

The `PERCENT_PRICE` filter defines valid range for a price based on the mark price.

In order to pass the `percent price`, the following must be true for `price`:

* BUY: `price` \<= `markPrice` \* `multiplierUp`
* SELL: `price` \>= `markPrice` \* `multiplierDown`

### MIN\_NOTIONAL[​](/docs/derivatives/usds-margined-futures/common-definition#min_notional) ###

>
>
> **/exchangeInfo format:**
>
>

```
  {    "filterType": "MIN_NOTIONAL",    "notional": "5.0"  }
```

The `MIN_NOTIONAL` filter defines the minimum notional value allowed for an order on a symbol.
An order's notional value is the `price` \* `quantity`.
Since `MARKET` orders have no price, the mark price is used.

## ERROR CODE

Error Codes
==========

>
>
> Here is the error JSON payload:
>
>

```
{  "code":-1121,  "msg":"Invalid symbol."}
```

Errors consist of two parts: an error code and a message.  
Codes are universal,but messages can vary.

10xx - General Server or Network issues[​](/docs/derivatives/usds-margined-futures/error-code#10xx---general-server-or-network-issues)
----------

### -1000 UNKNOWN[​](/docs/derivatives/usds-margined-futures/error-code#-1000-unknown) ###

* An unknown error occured while processing the request.

### -1001 DISCONNECTED[​](/docs/derivatives/usds-margined-futures/error-code#-1001-disconnected) ###

* Internal error; unable to process your request. Please try again.

### -1002 UNAUTHORIZED[​](/docs/derivatives/usds-margined-futures/error-code#-1002-unauthorized) ###

* You are not authorized to execute this request.

### -1003 TOO\_MANY\_REQUESTS[​](/docs/derivatives/usds-margined-futures/error-code#-1003-too_many_requests) ###

* Too many requests; current limit is %s requests per minute. Please use the websocket for live updates to avoid polling the API.
* Way too many requests; IP banned until %s. Please use the websocket for live updates to avoid bans.

### -1004 DUPLICATE\_IP[​](/docs/derivatives/usds-margined-futures/error-code#-1004-duplicate_ip) ###

* This IP is already on the white list

### -1005 NO\_SUCH\_IP[​](/docs/derivatives/usds-margined-futures/error-code#-1005-no_such_ip) ###

* No such IP has been white listed

### -1006 UNEXPECTED\_RESP[​](/docs/derivatives/usds-margined-futures/error-code#-1006-unexpected_resp) ###

* An unexpected response was received from the message bus. Execution status unknown.

### -1007 TIMEOUT[​](/docs/derivatives/usds-margined-futures/error-code#-1007-timeout) ###

* Timeout waiting for response from backend server. Send status unknown; execution status unknown.

### -1010 ERROR\_MSG\_RECEIVED[​](/docs/derivatives/usds-margined-futures/error-code#-1010-error_msg_received) ###

* ERROR\_MSG\_RECEIVED.

### -1011 NON\_WHITE\_LIST[​](/docs/derivatives/usds-margined-futures/error-code#-1011-non_white_list) ###

* This IP cannot access this route.

### -1013 INVALID\_MESSAGE[​](/docs/derivatives/usds-margined-futures/error-code#-1013-invalid_message) ###

* INVALID\_MESSAGE.

### -1014 UNKNOWN\_ORDER\_COMPOSITION[​](/docs/derivatives/usds-margined-futures/error-code#-1014-unknown_order_composition) ###

* Unsupported order combination.

### -1015 TOO\_MANY\_ORDERS[​](/docs/derivatives/usds-margined-futures/error-code#-1015-too_many_orders) ###

* Too many new orders.
* Too many new orders; current limit is %s orders per %s.

### -1016 SERVICE\_SHUTTING\_DOWN[​](/docs/derivatives/usds-margined-futures/error-code#-1016-service_shutting_down) ###

* This service is no longer available.

### -1020 UNSUPPORTED\_OPERATION[​](/docs/derivatives/usds-margined-futures/error-code#-1020-unsupported_operation) ###

* This operation is not supported.

### -1021 INVALID\_TIMESTAMP[​](/docs/derivatives/usds-margined-futures/error-code#-1021-invalid_timestamp) ###

* Timestamp for this request is outside of the recvWindow.
* Timestamp for this request was 1000ms ahead of the server's time.

### -1022 INVALID\_SIGNATURE[​](/docs/derivatives/usds-margined-futures/error-code#-1022-invalid_signature) ###

* Signature for this request is not valid.

### -1023 START\_TIME\_GREATER\_THAN\_END\_TIME[​](/docs/derivatives/usds-margined-futures/error-code#-1023-start_time_greater_than_end_time) ###

* Start time is greater than end time.

### -1099 NOT\_FOUND[​](/docs/derivatives/usds-margined-futures/error-code#-1099-not_found) ###

* Not found, unauthenticated, or unauthorized.

11xx - Request issues[​](/docs/derivatives/usds-margined-futures/error-code#11xx---request-issues)
----------

### -1100 ILLEGAL\_CHARS[​](/docs/derivatives/usds-margined-futures/error-code#-1100-illegal_chars) ###

* Illegal characters found in a parameter.
* Illegal characters found in parameter '%s'; legal range is '%s'.

### -1101 TOO\_MANY\_PARAMETERS[​](/docs/derivatives/usds-margined-futures/error-code#-1101-too_many_parameters) ###

* Too many parameters sent for this endpoint.
* Too many parameters; expected '%s' and received '%s'.
* Duplicate values for a parameter detected.

### -1102 MANDATORY\_PARAM\_EMPTY\_OR\_MALFORMED[​](/docs/derivatives/usds-margined-futures/error-code#-1102-mandatory_param_empty_or_malformed) ###

* A mandatory parameter was not sent, was empty/null, or malformed.
* Mandatory parameter '%s' was not sent, was empty/null, or malformed.
* Param '%s' or '%s' must be sent, but both were empty/null!

### -1103 UNKNOWN\_PARAM[​](/docs/derivatives/usds-margined-futures/error-code#-1103-unknown_param) ###

* An unknown parameter was sent.

### -1104 UNREAD\_PARAMETERS[​](/docs/derivatives/usds-margined-futures/error-code#-1104-unread_parameters) ###

* Not all sent parameters were read.
* Not all sent parameters were read; read '%s' parameter(s) but was sent '%s'.

### -1105 PARAM\_EMPTY[​](/docs/derivatives/usds-margined-futures/error-code#-1105-param_empty) ###

* A parameter was empty.
* Parameter '%s' was empty.

### -1106 PARAM\_NOT\_REQUIRED[​](/docs/derivatives/usds-margined-futures/error-code#-1106-param_not_required) ###

* A parameter was sent when not required.
* Parameter '%s' sent when not required.

### -1108 BAD\_ASSET[​](/docs/derivatives/usds-margined-futures/error-code#-1108-bad_asset) ###

* Invalid asset.

### -1109 BAD\_ACCOUNT[​](/docs/derivatives/usds-margined-futures/error-code#-1109-bad_account) ###

* Invalid account.

### -1110 BAD\_INSTRUMENT\_TYPE[​](/docs/derivatives/usds-margined-futures/error-code#-1110-bad_instrument_type) ###

* Invalid symbolType.

### -1111 BAD\_PRECISION[​](/docs/derivatives/usds-margined-futures/error-code#-1111-bad_precision) ###

* Precision is over the maximum defined for this asset.

### -1112 NO\_DEPTH[​](/docs/derivatives/usds-margined-futures/error-code#-1112-no_depth) ###

* No orders on book for symbol.

### -1113 WITHDRAW\_NOT\_NEGATIVE[​](/docs/derivatives/usds-margined-futures/error-code#-1113-withdraw_not_negative) ###

* Withdrawal amount must be negative.

### -1114 TIF\_NOT\_REQUIRED[​](/docs/derivatives/usds-margined-futures/error-code#-1114-tif_not_required) ###

* TimeInForce parameter sent when not required.

### -1115 INVALID\_TIF[​](/docs/derivatives/usds-margined-futures/error-code#-1115-invalid_tif) ###

* Invalid timeInForce.

### -1116 INVALID\_ORDER\_TYPE[​](/docs/derivatives/usds-margined-futures/error-code#-1116-invalid_order_type) ###

* Invalid orderType.

### -1117 INVALID\_SIDE[​](/docs/derivatives/usds-margined-futures/error-code#-1117-invalid_side) ###

* Invalid side.

### -1118 EMPTY\_NEW\_CL\_ORD\_ID[​](/docs/derivatives/usds-margined-futures/error-code#-1118-empty_new_cl_ord_id) ###

* New client order ID was empty.

### -1119 EMPTY\_ORG\_CL\_ORD\_ID[​](/docs/derivatives/usds-margined-futures/error-code#-1119-empty_org_cl_ord_id) ###

* Original client order ID was empty.

### -1120 BAD\_INTERVAL[​](/docs/derivatives/usds-margined-futures/error-code#-1120-bad_interval) ###

* Invalid interval.

### -1121 BAD\_SYMBOL[​](/docs/derivatives/usds-margined-futures/error-code#-1121-bad_symbol) ###

* Invalid symbol.

### -1122 INVALID\_SYMBOL\_STATUS[​](/docs/derivatives/usds-margined-futures/error-code#-1122-invalid_symbol_status) ###

* Invalid symbol status.

### -1125 INVALID\_LISTEN\_KEY[​](/docs/derivatives/usds-margined-futures/error-code#-1125-invalid_listen_key) ###

* This listenKey does not exist. Please use `POST /fapi/v1/listenKey` to recreate `listenKey`

### -1126 ASSET\_NOT\_SUPPORTED[​](/docs/derivatives/usds-margined-futures/error-code#-1126-asset_not_supported) ###

* This asset is not supported.

### -1127 MORE\_THAN\_XX\_HOURS[​](/docs/derivatives/usds-margined-futures/error-code#-1127-more_than_xx_hours) ###

* Lookup interval is too big.
* More than %s hours between startTime and endTime.

### -1128 OPTIONAL\_PARAMS\_BAD\_COMBO[​](/docs/derivatives/usds-margined-futures/error-code#-1128-optional_params_bad_combo) ###

* Combination of optional parameters invalid.

### -1130 INVALID\_PARAMETER[​](/docs/derivatives/usds-margined-futures/error-code#-1130-invalid_parameter) ###

* Invalid data sent for a parameter.
* Data sent for parameter '%s' is not valid.

### -1136 INVALID\_NEW\_ORDER\_RESP\_TYPE[​](/docs/derivatives/usds-margined-futures/error-code#-1136-invalid_new_order_resp_type) ###

* Invalid newOrderRespType.

20xx - Processing Issues[​](/docs/derivatives/usds-margined-futures/error-code#20xx---processing-issues)
----------

### -2010 NEW\_ORDER\_REJECTED[​](/docs/derivatives/usds-margined-futures/error-code#-2010-new_order_rejected) ###

* NEW\_ORDER\_REJECTED

### -2011 CANCEL\_REJECTED[​](/docs/derivatives/usds-margined-futures/error-code#-2011-cancel_rejected) ###

* CANCEL\_REJECTED

### -2012 CANCEL\_ALL\_FAIL[​](/docs/derivatives/usds-margined-futures/error-code#-2012-cancel_all_fail) ###

* Batch cancel failure.

### -2013 NO\_SUCH\_ORDER[​](/docs/derivatives/usds-margined-futures/error-code#-2013-no_such_order) ###

* Order does not exist.

### -2014 BAD\_API\_KEY\_FMT[​](/docs/derivatives/usds-margined-futures/error-code#-2014-bad_api_key_fmt) ###

* API-key format invalid.

### -2015 REJECTED\_MBX\_KEY[​](/docs/derivatives/usds-margined-futures/error-code#-2015-rejected_mbx_key) ###

* Invalid API-key, IP, or permissions for action.

### -2016 NO\_TRADING\_WINDOW[​](/docs/derivatives/usds-margined-futures/error-code#-2016-no_trading_window) ###

* No trading window could be found for the symbol. Try ticker/24hrs instead.

### -2017 API\_KEYS\_LOCKED[​](/docs/derivatives/usds-margined-futures/error-code#-2017-api_keys_locked) ###

* API Keys are locked on this account.

### -2018 BALANCE\_NOT\_SUFFICIENT[​](/docs/derivatives/usds-margined-futures/error-code#-2018-balance_not_sufficient) ###

* Balance is insufficient.

### -2019 MARGIN\_NOT\_SUFFICIEN[​](/docs/derivatives/usds-margined-futures/error-code#-2019-margin_not_sufficien) ###

* Margin is insufficient.

### -2020 UNABLE\_TO\_FILL[​](/docs/derivatives/usds-margined-futures/error-code#-2020-unable_to_fill) ###

* Unable to fill.

### -2021 ORDER\_WOULD\_IMMEDIATELY\_TRIGGER[​](/docs/derivatives/usds-margined-futures/error-code#-2021-order_would_immediately_trigger) ###

* Order would immediately trigger.

### -2022 REDUCE\_ONLY\_REJECT[​](/docs/derivatives/usds-margined-futures/error-code#-2022-reduce_only_reject) ###

* ReduceOnly Order is rejected.

### -2023 USER\_IN\_LIQUIDATION[​](/docs/derivatives/usds-margined-futures/error-code#-2023-user_in_liquidation) ###

* User in liquidation mode now.

### -2024 POSITION\_NOT\_SUFFICIENT[​](/docs/derivatives/usds-margined-futures/error-code#-2024-position_not_sufficient) ###

* Position is not sufficient.

### -2025 MAX\_OPEN\_ORDER\_EXCEEDED[​](/docs/derivatives/usds-margined-futures/error-code#-2025-max_open_order_exceeded) ###

* Reach max open order limit.

### -2026 REDUCE\_ONLY\_ORDER\_TYPE\_NOT\_SUPPORTED[​](/docs/derivatives/usds-margined-futures/error-code#-2026-reduce_only_order_type_not_supported) ###

* This OrderType is not supported when reduceOnly.

### -2027 MAX\_LEVERAGE\_RATIO[​](/docs/derivatives/usds-margined-futures/error-code#-2027-max_leverage_ratio) ###

* Exceeded the maximum allowable position at current leverage.

### -2028 MIN\_LEVERAGE\_RATIO[​](/docs/derivatives/usds-margined-futures/error-code#-2028-min_leverage_ratio) ###

* Leverage is smaller than permitted: insufficient margin balance.

40xx - Filters and other Issues[​](/docs/derivatives/usds-margined-futures/error-code#40xx---filters-and-other-issues)
----------

### -4000 INVALID\_ORDER\_STATUS[​](/docs/derivatives/usds-margined-futures/error-code#-4000-invalid_order_status) ###

* Invalid order status.

### -4001 PRICE\_LESS\_THAN\_ZERO[​](/docs/derivatives/usds-margined-futures/error-code#-4001-price_less_than_zero) ###

* Price less than 0.

### -4002 PRICE\_GREATER\_THAN\_MAX\_PRICE[​](/docs/derivatives/usds-margined-futures/error-code#-4002-price_greater_than_max_price) ###

* Price greater than max price.

### -4003 QTY\_LESS\_THAN\_ZERO[​](/docs/derivatives/usds-margined-futures/error-code#-4003-qty_less_than_zero) ###

* Quantity less than zero.

### -4004 QTY\_LESS\_THAN\_MIN\_QTY[​](/docs/derivatives/usds-margined-futures/error-code#-4004-qty_less_than_min_qty) ###

* Quantity less than min quantity.

### -4005 QTY\_GREATER\_THAN\_MAX\_QTY[​](/docs/derivatives/usds-margined-futures/error-code#-4005-qty_greater_than_max_qty) ###

* Quantity greater than max quantity.

### -4006 STOP\_PRICE\_LESS\_THAN\_ZERO[​](/docs/derivatives/usds-margined-futures/error-code#-4006-stop_price_less_than_zero) ###

* Stop price less than zero.

### -4007 STOP\_PRICE\_GREATER\_THAN\_MAX\_PRICE[​](/docs/derivatives/usds-margined-futures/error-code#-4007-stop_price_greater_than_max_price) ###

* Stop price greater than max price.

### -4008 TICK\_SIZE\_LESS\_THAN\_ZERO[​](/docs/derivatives/usds-margined-futures/error-code#-4008-tick_size_less_than_zero) ###

* Tick size less than zero.

### -4009 MAX\_PRICE\_LESS\_THAN\_MIN\_PRICE[​](/docs/derivatives/usds-margined-futures/error-code#-4009-max_price_less_than_min_price) ###

* Max price less than min price.

### -4010 MAX\_QTY\_LESS\_THAN\_MIN\_QTY[​](/docs/derivatives/usds-margined-futures/error-code#-4010-max_qty_less_than_min_qty) ###

* Max qty less than min qty.

### -4011 STEP\_SIZE\_LESS\_THAN\_ZERO[​](/docs/derivatives/usds-margined-futures/error-code#-4011-step_size_less_than_zero) ###

* Step size less than zero.

### -4012 MAX\_NUM\_ORDERS\_LESS\_THAN\_ZERO[​](/docs/derivatives/usds-margined-futures/error-code#-4012-max_num_orders_less_than_zero) ###

* Max mum orders less than zero.

### -4013 PRICE\_LESS\_THAN\_MIN\_PRICE[​](/docs/derivatives/usds-margined-futures/error-code#-4013-price_less_than_min_price) ###

* Price less than min price.

### -4014 PRICE\_NOT\_INCREASED\_BY\_TICK\_SIZE[​](/docs/derivatives/usds-margined-futures/error-code#-4014-price_not_increased_by_tick_size) ###

* Price not increased by tick size.

### -4015 INVALID\_CL\_ORD\_ID\_LEN[​](/docs/derivatives/usds-margined-futures/error-code#-4015-invalid_cl_ord_id_len) ###

* Client order id is not valid.
* Client order id length should not be more than 36 chars

### -4016 PRICE\_HIGHTER\_THAN\_MULTIPLIER\_UP[​](/docs/derivatives/usds-margined-futures/error-code#-4016-price_highter_than_multiplier_up) ###

* Price is higher than mark price multiplier cap.

### -4017 MULTIPLIER\_UP\_LESS\_THAN\_ZERO[​](/docs/derivatives/usds-margined-futures/error-code#-4017-multiplier_up_less_than_zero) ###

* Multiplier up less than zero.

### -4018 MULTIPLIER\_DOWN\_LESS\_THAN\_ZERO[​](/docs/derivatives/usds-margined-futures/error-code#-4018-multiplier_down_less_than_zero) ###

* Multiplier down less than zero.

### -4019 COMPOSITE\_SCALE\_OVERFLOW[​](/docs/derivatives/usds-margined-futures/error-code#-4019-composite_scale_overflow) ###

* Composite scale too large.

### -4020 TARGET\_STRATEGY\_INVALID[​](/docs/derivatives/usds-margined-futures/error-code#-4020-target_strategy_invalid) ###

* Target strategy invalid for orderType '%s',reduceOnly '%b'.

### -4021 INVALID\_DEPTH\_LIMIT[​](/docs/derivatives/usds-margined-futures/error-code#-4021-invalid_depth_limit) ###

* Invalid depth limit.
* '%s' is not valid depth limit.

### -4022 WRONG\_MARKET\_STATUS[​](/docs/derivatives/usds-margined-futures/error-code#-4022-wrong_market_status) ###

* market status sent is not valid.

### -4023 QTY\_NOT\_INCREASED\_BY\_STEP\_SIZE[​](/docs/derivatives/usds-margined-futures/error-code#-4023-qty_not_increased_by_step_size) ###

* Qty not increased by step size.

### -4024 PRICE\_LOWER\_THAN\_MULTIPLIER\_DOWN[​](/docs/derivatives/usds-margined-futures/error-code#-4024-price_lower_than_multiplier_down) ###

* Price is lower than mark price multiplier floor.

### -4025 MULTIPLIER\_DECIMAL\_LESS\_THAN\_ZERO[​](/docs/derivatives/usds-margined-futures/error-code#-4025-multiplier_decimal_less_than_zero) ###

* Multiplier decimal less than zero.

### -4026 COMMISSION\_INVALID[​](/docs/derivatives/usds-margined-futures/error-code#-4026-commission_invalid) ###

* Commission invalid.
* `%s` less than zero.
* `%s` absolute value greater than `%s`

### -4027 INVALID\_ACCOUNT\_TYPE[​](/docs/derivatives/usds-margined-futures/error-code#-4027-invalid_account_type) ###

* Invalid account type.

### -4028 INVALID\_LEVERAGE[​](/docs/derivatives/usds-margined-futures/error-code#-4028-invalid_leverage) ###

* Invalid leverage
* Leverage `%s` is not valid
* Leverage `%s` already exist with `%s`

### -4029 INVALID\_TICK\_SIZE\_PRECISION[​](/docs/derivatives/usds-margined-futures/error-code#-4029-invalid_tick_size_precision) ###

* Tick size precision is invalid.

### -4030 INVALID\_STEP\_SIZE\_PRECISION[​](/docs/derivatives/usds-margined-futures/error-code#-4030-invalid_step_size_precision) ###

* Step size precision is invalid.

### -4031 INVALID\_WORKING\_TYPE[​](/docs/derivatives/usds-margined-futures/error-code#-4031-invalid_working_type) ###

* Invalid parameter working type
* Invalid parameter working type: `%s`

### -4032 EXCEED\_MAX\_CANCEL\_ORDER\_SIZE[​](/docs/derivatives/usds-margined-futures/error-code#-4032-exceed_max_cancel_order_size) ###

* Exceed maximum cancel order size.
* Invalid parameter working type: `%s`

### -4033 INSURANCE\_ACCOUNT\_NOT\_FOUND[​](/docs/derivatives/usds-margined-futures/error-code#-4033-insurance_account_not_found) ###

* Insurance account not found.

### -4044 INVALID\_BALANCE\_TYPE[​](/docs/derivatives/usds-margined-futures/error-code#-4044-invalid_balance_type) ###

* Balance Type is invalid.

### -4045 MAX\_STOP\_ORDER\_EXCEEDED[​](/docs/derivatives/usds-margined-futures/error-code#-4045-max_stop_order_exceeded) ###

* Reach max stop order limit.

### -4046 NO\_NEED\_TO\_CHANGE\_MARGIN\_TYPE[​](/docs/derivatives/usds-margined-futures/error-code#-4046-no_need_to_change_margin_type) ###

* No need to change margin type.

### -4047 THERE\_EXISTS\_OPEN\_ORDERS[​](/docs/derivatives/usds-margined-futures/error-code#-4047-there_exists_open_orders) ###

* Margin type cannot be changed if there exists open orders.

### -4048 THERE\_EXISTS\_QUANTITY[​](/docs/derivatives/usds-margined-futures/error-code#-4048-there_exists_quantity) ###

* Margin type cannot be changed if there exists position.

### -4049 ADD\_ISOLATED\_MARGIN\_REJECT[​](/docs/derivatives/usds-margined-futures/error-code#-4049-add_isolated_margin_reject) ###

* Add margin only support for isolated position.

### -4050 CROSS\_BALANCE\_INSUFFICIENT[​](/docs/derivatives/usds-margined-futures/error-code#-4050-cross_balance_insufficient) ###

* Cross balance insufficient.

### -4051 ISOLATED\_BALANCE\_INSUFFICIENT[​](/docs/derivatives/usds-margined-futures/error-code#-4051-isolated_balance_insufficient) ###

* Isolated balance insufficient.

### -4052 NO\_NEED\_TO\_CHANGE\_AUTO\_ADD\_MARGIN[​](/docs/derivatives/usds-margined-futures/error-code#-4052-no_need_to_change_auto_add_margin) ###

* No need to change auto add margin.

### -4053 AUTO\_ADD\_CROSSED\_MARGIN\_REJECT[​](/docs/derivatives/usds-margined-futures/error-code#-4053-auto_add_crossed_margin_reject) ###

* Auto add margin only support for isolated position.

### -4054 ADD\_ISOLATED\_MARGIN\_NO\_POSITION\_REJECT[​](/docs/derivatives/usds-margined-futures/error-code#-4054-add_isolated_margin_no_position_reject) ###

* Cannot add position margin: position is 0.

### -4055 AMOUNT\_MUST\_BE\_POSITIVE[​](/docs/derivatives/usds-margined-futures/error-code#-4055-amount_must_be_positive) ###

* Amount must be positive.

### -4056 INVALID\_API\_KEY\_TYPE[​](/docs/derivatives/usds-margined-futures/error-code#-4056-invalid_api_key_type) ###

* Invalid api key type.

### -4057 INVALID\_RSA\_PUBLIC\_KEY[​](/docs/derivatives/usds-margined-futures/error-code#-4057-invalid_rsa_public_key) ###

* Invalid api public key

### -4058 MAX\_PRICE\_TOO\_LARGE[​](/docs/derivatives/usds-margined-futures/error-code#-4058-max_price_too_large) ###

* maxPrice and priceDecimal too large,please check.

### -4059 NO\_NEED\_TO\_CHANGE\_POSITION\_SIDE[​](/docs/derivatives/usds-margined-futures/error-code#-4059-no_need_to_change_position_side) ###

* No need to change position side.

### -4060 INVALID\_POSITION\_SIDE[​](/docs/derivatives/usds-margined-futures/error-code#-4060-invalid_position_side) ###

* Invalid position side.

### -4061 POSITION\_SIDE\_NOT\_MATCH[​](/docs/derivatives/usds-margined-futures/error-code#-4061-position_side_not_match) ###

* Order's position side does not match user's setting.

### -4062 REDUCE\_ONLY\_CONFLICT[​](/docs/derivatives/usds-margined-futures/error-code#-4062-reduce_only_conflict) ###

* Invalid or improper reduceOnly value.

### -4063 INVALID\_OPTIONS\_REQUEST\_TYPE[​](/docs/derivatives/usds-margined-futures/error-code#-4063-invalid_options_request_type) ###

* Invalid options request type

### -4064 INVALID\_OPTIONS\_TIME\_FRAME[​](/docs/derivatives/usds-margined-futures/error-code#-4064-invalid_options_time_frame) ###

* Invalid options time frame

### -4065 INVALID\_OPTIONS\_AMOUNT[​](/docs/derivatives/usds-margined-futures/error-code#-4065-invalid_options_amount) ###

* Invalid options amount

### -4066 INVALID\_OPTIONS\_EVENT\_TYPE[​](/docs/derivatives/usds-margined-futures/error-code#-4066-invalid_options_event_type) ###

* Invalid options event type

### -4067 POSITION\_SIDE\_CHANGE\_EXISTS\_OPEN\_ORDERS[​](/docs/derivatives/usds-margined-futures/error-code#-4067-position_side_change_exists_open_orders) ###

* Position side cannot be changed if there exists open orders.

### -4068 POSITION\_SIDE\_CHANGE\_EXISTS\_QUANTITY[​](/docs/derivatives/usds-margined-futures/error-code#-4068-position_side_change_exists_quantity) ###

* Position side cannot be changed if there exists position.

### -4069 INVALID\_OPTIONS\_PREMIUM\_FEE[​](/docs/derivatives/usds-margined-futures/error-code#-4069-invalid_options_premium_fee) ###

* Invalid options premium fee

### -4070 INVALID\_CL\_OPTIONS\_ID\_LEN[​](/docs/derivatives/usds-margined-futures/error-code#-4070-invalid_cl_options_id_len) ###

* Client options id is not valid.
* Client options id length should be less than 32 chars

### -4071 INVALID\_OPTIONS\_DIRECTION[​](/docs/derivatives/usds-margined-futures/error-code#-4071-invalid_options_direction) ###

* Invalid options direction

### -4072 OPTIONS\_PREMIUM\_NOT\_UPDATE[​](/docs/derivatives/usds-margined-futures/error-code#-4072-options_premium_not_update) ###

* premium fee is not updated, reject order

### -4073 OPTIONS\_PREMIUM\_INPUT\_LESS\_THAN\_ZERO[​](/docs/derivatives/usds-margined-futures/error-code#-4073-options_premium_input_less_than_zero) ###

* input premium fee is less than 0, reject order

### -4074 OPTIONS\_AMOUNT\_BIGGER\_THAN\_UPPER[​](/docs/derivatives/usds-margined-futures/error-code#-4074-options_amount_bigger_than_upper) ###

* Order amount is bigger than upper boundary or less than 0, reject order

### -4075 OPTIONS\_PREMIUM\_OUTPUT\_ZERO[​](/docs/derivatives/usds-margined-futures/error-code#-4075-options_premium_output_zero) ###

* output premium fee is less than 0, reject order

### -4076 OPTIONS\_PREMIUM\_TOO\_DIFF[​](/docs/derivatives/usds-margined-futures/error-code#-4076-options_premium_too_diff) ###

* original fee is too much higher than last fee

### -4077 OPTIONS\_PREMIUM\_REACH\_LIMIT[​](/docs/derivatives/usds-margined-futures/error-code#-4077-options_premium_reach_limit) ###

* place order amount has reached to limit, reject order

### -4078 OPTIONS\_COMMON\_ERROR[​](/docs/derivatives/usds-margined-futures/error-code#-4078-options_common_error) ###

* options internal error

### -4079 INVALID\_OPTIONS\_ID[​](/docs/derivatives/usds-margined-futures/error-code#-4079-invalid_options_id) ###

* invalid options id
* invalid options id: %s
* duplicate options id %d for user %d

### -4080 OPTIONS\_USER\_NOT\_FOUND[​](/docs/derivatives/usds-margined-futures/error-code#-4080-options_user_not_found) ###

* user not found
* user not found with id: %s

### -4081 OPTIONS\_NOT\_FOUND[​](/docs/derivatives/usds-margined-futures/error-code#-4081-options_not_found) ###

* options not found
* options not found with id: %s

### -4082 INVALID\_BATCH\_PLACE\_ORDER\_SIZE[​](/docs/derivatives/usds-margined-futures/error-code#-4082-invalid_batch_place_order_size) ###

* Invalid number of batch place orders.
* Invalid number of batch place orders: %s

### -4083 PLACE\_BATCH\_ORDERS\_FAIL[​](/docs/derivatives/usds-margined-futures/error-code#-4083-place_batch_orders_fail) ###

* Fail to place batch orders.

### -4084 UPCOMING\_METHOD[​](/docs/derivatives/usds-margined-futures/error-code#-4084-upcoming_method) ###

* Method is not allowed currently. Upcoming soon.

### -4085 INVALID\_NOTIONAL\_LIMIT\_COEF[​](/docs/derivatives/usds-margined-futures/error-code#-4085-invalid_notional_limit_coef) ###

* Invalid notional limit coefficient

### -4086 INVALID\_PRICE\_SPREAD\_THRESHOLD[​](/docs/derivatives/usds-margined-futures/error-code#-4086-invalid_price_spread_threshold) ###

* Invalid price spread threshold

### -4087 REDUCE\_ONLY\_ORDER\_PERMISSION[​](/docs/derivatives/usds-margined-futures/error-code#-4087-reduce_only_order_permission) ###

* User can only place reduce only order

### -4088 NO\_PLACE\_ORDER\_PERMISSION[​](/docs/derivatives/usds-margined-futures/error-code#-4088-no_place_order_permission) ###

* User can not place order currently

### -4104 INVALID\_CONTRACT\_TYPE[​](/docs/derivatives/usds-margined-futures/error-code#-4104-invalid_contract_type) ###

* Invalid contract type

### -4114 INVALID\_CLIENT\_TRAN\_ID\_LEN[​](/docs/derivatives/usds-margined-futures/error-code#-4114-invalid_client_tran_id_len) ###

* clientTranId is not valid
* Client tran id length should be less than 64 chars

### -4115 DUPLICATED\_CLIENT\_TRAN\_ID[​](/docs/derivatives/usds-margined-futures/error-code#-4115-duplicated_client_tran_id) ###

* clientTranId is duplicated
* Client tran id should be unique within 7 days

### -4116 DUPLICATED\_CLIENT\_ORDER\_ID[​](/docs/derivatives/usds-margined-futures/error-code#-4116-duplicated_client_order_id) ###

* clientOrderId is duplicated

### -4117 STOP\_ORDER\_TRIGGERING[​](/docs/derivatives/usds-margined-futures/error-code#-4117-stop_order_triggering) ###

* stop order is triggering

### -4118 REDUCE\_ONLY\_MARGIN\_CHECK\_FAILED[​](/docs/derivatives/usds-margined-futures/error-code#-4118-reduce_only_margin_check_failed) ###

* ReduceOnly Order Failed. Please check your existing position and open orders

### -4131 MARKET\_ORDER\_REJECT[​](/docs/derivatives/usds-margined-futures/error-code#-4131-market_order_reject) ###

* The counterparty's best price does not meet the PERCENT\_PRICE filter limit

### -4135 INVALID\_ACTIVATION\_PRICE[​](/docs/derivatives/usds-margined-futures/error-code#-4135-invalid_activation_price) ###

* Invalid activation price

### -4137 QUANTITY\_EXISTS\_WITH\_CLOSE\_POSITION[​](/docs/derivatives/usds-margined-futures/error-code#-4137-quantity_exists_with_close_position) ###

* Quantity must be zero with closePosition equals true

### -4138 REDUCE\_ONLY\_MUST\_BE\_TRUE[​](/docs/derivatives/usds-margined-futures/error-code#-4138-reduce_only_must_be_true) ###

* Reduce only must be true with closePosition equals true

### -4139 ORDER\_TYPE\_CANNOT\_BE\_MKT[​](/docs/derivatives/usds-margined-futures/error-code#-4139-order_type_cannot_be_mkt) ###

* Order type can not be market if it's unable to cancel

### -4140 INVALID\_OPENING\_POSITION\_STATUS[​](/docs/derivatives/usds-margined-futures/error-code#-4140-invalid_opening_position_status) ###

* Invalid symbol status for opening position

### -4141 SYMBOL\_ALREADY\_CLOSED[​](/docs/derivatives/usds-margined-futures/error-code#-4141-symbol_already_closed) ###

* Symbol is closed

### -4142 STRATEGY\_INVALID\_TRIGGER\_PRICE[​](/docs/derivatives/usds-margined-futures/error-code#-4142-strategy_invalid_trigger_price) ###

* REJECT: take profit or stop order will be triggered immediately

### -4144 INVALID\_PAIR[​](/docs/derivatives/usds-margined-futures/error-code#-4144-invalid_pair) ###

* Invalid pair

### -4161 ISOLATED\_LEVERAGE\_REJECT\_WITH\_POSITION[​](/docs/derivatives/usds-margined-futures/error-code#-4161-isolated_leverage_reject_with_position) ###

* Leverage reduction is not supported in Isolated Margin Mode with open positions

### -4164 MIN\_NOTIONAL[​](/docs/derivatives/usds-margined-futures/error-code#-4164-min_notional) ###

* Order's notional must be no smaller than 5.0 (unless you choose reduce only)
* Order's notional must be no smaller than %s (unless you choose reduce only)

### -4165 INVALID\_TIME\_INTERVAL[​](/docs/derivatives/usds-margined-futures/error-code#-4165-invalid_time_interval) ###

* Invalid time interval
* Maximum time interval is %s days

### -4167 ISOLATED\_REJECT\_WITH\_JOINT\_MARGIN[​](/docs/derivatives/usds-margined-futures/error-code#-4167-isolated_reject_with_joint_margin) ###

* Unable to adjust to Multi-Assets mode with symbols of USDⓈ-M Futures under isolated-margin mode.

### -4168 JOINT\_MARGIN\_REJECT\_WITH\_ISOLATED[​](/docs/derivatives/usds-margined-futures/error-code#-4168-joint_margin_reject_with_isolated) ###

* Unable to adjust to isolated-margin mode under the Multi-Assets mode.

### -4169 JOINT\_MARGIN\_REJECT\_WITH\_MB[​](/docs/derivatives/usds-margined-futures/error-code#-4169-joint_margin_reject_with_mb) ###

* Unable to adjust Multi-Assets Mode with insufficient margin balance in USDⓈ-M Futures.

### -4170 JOINT\_MARGIN\_REJECT\_WITH\_OPEN\_ORDER[​](/docs/derivatives/usds-margined-futures/error-code#-4170-joint_margin_reject_with_open_order) ###

* Unable to adjust Multi-Assets Mode with open orders in USDⓈ-M Futures.

### -4171 NO\_NEED\_TO\_CHANGE\_JOINT\_MARGIN[​](/docs/derivatives/usds-margined-futures/error-code#-4171-no_need_to_change_joint_margin) ###

* Adjusted asset mode is currently set and does not need to be adjusted repeatedly.

### -4172 JOINT\_MARGIN\_REJECT\_WITH\_NEGATIVE\_BALANCE[​](/docs/derivatives/usds-margined-futures/error-code#-4172-joint_margin_reject_with_negative_balance) ###

* Unable to adjust Multi-Assets Mode with a negative wallet balance of margin available asset in USDⓈ-M Futures account.

### -4183 ISOLATED\_REJECT\_WITH\_JOINT\_MARGIN[​](/docs/derivatives/usds-margined-futures/error-code#-4183-isolated_reject_with_joint_margin) ###

* Price is higher than stop price multiplier cap.
* Limit price can't be higher than %s.

### -4184 PRICE\_LOWER\_THAN\_STOP\_MULTIPLIER\_DOWN[​](/docs/derivatives/usds-margined-futures/error-code#-4184-price_lower_than_stop_multiplier_down) ###

* Price is lower than stop price multiplier floor.
* Limit price can't be lower than %s.

### -4192 COOLING\_OFF\_PERIOD[​](/docs/derivatives/usds-margined-futures/error-code#-4192-cooling_off_period) ###

* Trade forbidden due to Cooling-off Period.

### -4202 ADJUST\_LEVERAGE\_KYC\_FAILED[​](/docs/derivatives/usds-margined-futures/error-code#-4202-adjust_leverage_kyc_failed) ###

* Intermediate Personal Verification is required for adjusting leverage over 20x

### -4203 ADJUST\_LEVERAGE\_ONE\_MONTH\_FAILED[​](/docs/derivatives/usds-margined-futures/error-code#-4203-adjust_leverage_one_month_failed) ###

* More than 20x leverage is available one month after account registration.

### -4205 ADJUST\_LEVERAGE\_X\_DAYS\_FAILED[​](/docs/derivatives/usds-margined-futures/error-code#-4205-adjust_leverage_x_days_failed) ###

* More than 20x leverage is available %s days after Futures account registration.

### -4206 ADJUST\_LEVERAGE\_KYC\_LIMIT[​](/docs/derivatives/usds-margined-futures/error-code#-4206-adjust_leverage_kyc_limit) ###

* Users in this country has limited adjust leverage.
* Users in your location/country can only access a maximum leverage of %s

### -4208 ADJUST\_LEVERAGE\_ACCOUNT\_SYMBOL\_FAILED[​](/docs/derivatives/usds-margined-futures/error-code#-4208-adjust_leverage_account_symbol_failed) ###

* Current symbol leverage cannot exceed 20 when using position limit adjustment service.

### -4209 ADJUST\_LEVERAGE\_SYMBOL\_FAILED[​](/docs/derivatives/usds-margined-futures/error-code#-4209-adjust_leverage_symbol_failed) ###

* The max leverage of Symbol is 20x
* Leverage adjustment failed. Current symbol max leverage limit is %sx

### -4210 STOP\_PRICE\_HIGHER\_THAN\_PRICE\_MULTIPLIER\_LIMIT[​](/docs/derivatives/usds-margined-futures/error-code#-4210-stop_price_higher_than_price_multiplier_limit) ###

* Stop price is higher than price multiplier cap.
* Stop price can't be higher than %s

### -4211 STOP\_PRICE\_LOWER\_THAN\_PRICE\_MULTIPLIER\_LIMIT[​](/docs/derivatives/usds-margined-futures/error-code#-4211-stop_price_lower_than_price_multiplier_limit) ###

* Stop price is lower than price multiplier floor.
* Stop price can't be lower than %s

### -4400 TRADING\_QUANTITATIVE\_RULE[​](/docs/derivatives/usds-margined-futures/error-code#-4400-trading_quantitative_rule) ###

* Futures Trading Quantitative Rules violated, only reduceOnly order is allowed, please try again later.

### -4401 LARGE\_POSITION\_SYM\_RULE[​](/docs/derivatives/usds-margined-futures/error-code#-4401-large_position_sym_rule) ###

* Futures Trading Risk Control Rules of large position holding violated, only reduceOnly order is allowed, please reduce the position.
  .

### -4402 COMPLIANCE\_BLACK\_SYMBOL\_RESTRICTION[​](/docs/derivatives/usds-margined-futures/error-code#-4402-compliance_black_symbol_restriction) ###

* Dear user, as per our Terms of Use and compliance with local regulations, this feature is currently not available in your region.

### -4403 ADJUST\_LEVERAGE\_COMPLIANCE\_FAILED[​](/docs/derivatives/usds-margined-futures/error-code#-4403-adjust_leverage_compliance_failed) ###

* Dear user, as per our Terms of Use and compliance with local regulations, the leverage can only up to 10x in your region
* Dear user, as per our Terms of Use and compliance with local regulations, the leverage can only up to %sx in your region

50xx - Order Execution Issues[​](/docs/derivatives/usds-margined-futures/error-code#50xx---order-execution-issues)
----------

### -5021 FOK\_ORDER\_REJECT[​](/docs/derivatives/usds-margined-futures/error-code#-5021-fok_order_reject) ###

* Due to the order could not be filled immediately, the FOK order has been rejected.

### -5022 GTX\_ORDER\_REJECT[​](/docs/derivatives/usds-margined-futures/error-code#-5022-gtx_order_reject) ###

* Due to the order could not be executed as maker, the Post Only order will be rejected.

### -5024 MOVE\_ORDER\_NOT\_ALLOWED\_SYMBOL\_REASON[​](/docs/derivatives/usds-margined-futures/error-code#-5024-move_order_not_allowed_symbol_reason) ###

* Symbol is not in trading status. Order amendment is not permitted.

### -5025 LIMIT\_ORDER\_ONLY[​](/docs/derivatives/usds-margined-futures/error-code#-5025-limit_order_only) ###

* Only limit order is supported.

### -5026 Exceed\_Maximum\_Modify\_Order\_Limit[​](/docs/derivatives/usds-margined-futures/error-code#-5026-exceed_maximum_modify_order_limit) ###

* Exceed maximum modify order limit.

### -5027 SAME\_ORDER[​](/docs/derivatives/usds-margined-futures/error-code#-5027-same_order) ###

* No need to modify the order.

### -5028 ME\_RECVWINDOW\_REJECT[​](/docs/derivatives/usds-margined-futures/error-code#-5028-me_recvwindow_reject) ###

* Timestamp for this request is outside of the ME recvWindow.

### -5029 MODIFICATION\_MIN\_NOTIONAL[​](/docs/derivatives/usds-margined-futures/error-code#-5029-modification_min_notional) ###

* Order's notional must be no smaller than %s

### -5037 INVALID\_PRICE\_MATCH[​](/docs/derivatives/usds-margined-futures/error-code#-5037-invalid_price_match) ###

* Invalid price match

### -5038 UNSUPPORTED\_ORDER\_TYPE\_PRICE\_MATCH[​](/docs/derivatives/usds-margined-futures/error-code#-5038-unsupported_order_type_price_match) ###

* Price match only supports order type: LIMIT, STOP AND TAKE\_PROFIT

### -5039 INVALID\_SELF\_TRADE\_PREVENTION\_MODE[​](/docs/derivatives/usds-margined-futures/error-code#-5039-invalid_self_trade_prevention_mode) ###

* Invalid self trade prevention mode

### -5040 FUTURE\_GOOD\_TILL\_DATE[​](/docs/derivatives/usds-margined-futures/error-code#-5040-future_good_till_date) ###

* The goodTillDate timestamp must be greater than the current time plus 600 seconds and smaller than 253402300799000 (UTC 9999-12-31 23:59:59)

### -5041 BBO\_ORDER\_REJECT[​](/docs/derivatives/usds-margined-futures/error-code#-5041-bbo_order_reject) ###

* No depth matches this BBO order

## REST API

Test Connectivity
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api#api-description)
----------

Test connectivity to the Rest API.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api#http-request)
----------

GET `/fapi/v1/ping`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api#request-weight)
----------

1

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api#request-parameters)
----------

NONE

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api#response-example)
----------

```
{}
```

## CHECK SERVER TIME

Check Server Time
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Check-Server-Time#api-description)
----------

Test connectivity to the Rest API and get the current server time.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Check-Server-Time#http-request)
----------

GET `/fapi/v1/time`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Check-Server-Time#request-weight)
----------

1

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Check-Server-Time#request-parameters)
----------

NONE

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Check-Server-Time#response-example)
----------

```
{  "serverTime": 1499827319559}
```

## EXCHANGE INFORMATION

Exchange Information
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Exchange-Information#api-description)
----------

Current exchange trading rules and symbol information

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Exchange-Information#http-request)
----------

GET `/fapi/v1/exchangeInfo`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Exchange-Information#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Exchange-Information#request-parameters)
----------

NONE

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Exchange-Information#response-example)
----------

```
{	"exchangeFilters": [], 	"rateLimits": [ 		{ 			"interval": "MINUTE",   			"intervalNum": 1,   			"limit": 2400,   			"rateLimitType": "REQUEST_WEIGHT"    		},  		{  			"interval": "MINUTE",   			"intervalNum": 1,   			"limit": 1200,   			"rateLimitType": "ORDERS"   		}   	], 	"serverTime": 1565613908500,    // Ignore please. If you want to check current server time, please check via "GET /fapi/v1/time" 	"assets": [ // assets information 		{ 			"asset": "BUSD",   			"marginAvailable": true, // whether the asset can be used as margin in Multi-Assets mode   			"autoAssetExchange": 0 // auto-exchange threshold in Multi-Assets margin mode   		}, 		{ 			"asset": "USDT",   			"marginAvailable": true,   			"autoAssetExchange": 0   		}, 		{ 			"asset": "BNB",   			"marginAvailable": false,   			"autoAssetExchange": null   		}   	], 	"symbols": [ 		{ 			"symbol": "BLZUSDT", 			"pair": "BLZUSDT", 			"contractType": "PERPETUAL", 			"deliveryDate": 4133404800000, 			"onboardDate": 1598252400000, 			"status": "TRADING", 			"maintMarginPercent": "2.5000",   // ignore 			"requiredMarginPercent": "5.0000",  // ignore 			"baseAsset": "BLZ",  			"quoteAsset": "USDT", 			"marginAsset": "USDT", 			"pricePrecision": 5,	// please do not use it as tickSize 			"quantityPrecision": 0, // please do not use it as stepSize 			"baseAssetPrecision": 8, 			"quotePrecision": 8,  			"underlyingType": "COIN", 			"underlyingSubType": ["STORAGE"], 			"settlePlan": 0, 			"triggerProtect": "0.15", // threshold for algo order with "priceProtect" 			"filters": [ 				{ 					"filterType": "PRICE_FILTER",     				"maxPrice": "300",     				"minPrice": "0.0001",      				"tickSize": "0.0001"     			},    			{    				"filterType": "LOT_SIZE",      				"maxQty": "10000000",     				"minQty": "1",     				"stepSize": "1"     			},    			{    				"filterType": "MARKET_LOT_SIZE",     				"maxQty": "590119",     				"minQty": "1",     				"stepSize": "1"     			},     			{    				"filterType": "MAX_NUM_ORDERS",    				"limit": 200  				},  				{    				"filterType": "MAX_NUM_ALGO_ORDERS",    				"limit": 10  				},  				{  					"filterType": "MIN_NOTIONAL",  					"notional": "5.0",   				},  				{    				"filterType": "PERCENT_PRICE",    				"multiplierUp": "1.1500",    				"multiplierDown": "0.8500",    				"multiplierDecimal": 4    			}   			], 			"OrderType": [   				"LIMIT",   				"MARKET",   				"STOP",   				"STOP_MARKET",   				"TAKE_PROFIT",   				"TAKE_PROFIT_MARKET",   				"TRAILING_STOP_MARKET"    			],   			"timeInForce": [   				"GTC",    				"IOC",    				"FOK",    				"GTX"  			], 			"liquidationFee": "0.010000",	// liquidation fee rate   			"marketTakeBound": "0.30",	// the max price difference rate( from mark price) a market order can make 		}   	],	"timezone": "UTC" }
```

## ORDER BOOK

Order Book
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Order-Book#api-description)
----------

Query symbol orderbook

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Order-Book#http-request)
----------

GET `/fapi/v1/depth`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Order-Book#request-weight)
----------

Adjusted based on the limit:

|    Limit    |Weight|
|-------------|------|
|5, 10, 20, 50|  2   |
|     100     |  5   |
|     500     |  10  |
|    1000     |  20  |

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Order-Book#request-parameters)
----------

| Name | Type |Mandatory|                       Description                       |
|------|------|---------|---------------------------------------------------------|
|symbol|STRING|   YES   |                                                         |
|limit | INT  |   NO    |Default 500; Valid limits:[5, 10, 20, 50, 100, 500, 1000]|

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Order-Book#response-example)
----------

```
{  "lastUpdateId": 1027024,  "E": 1589436922972,   // Message output time  "T": 1589436922959,   // Transaction time  "bids": [    [      "4.00000000",     // PRICE      "431.00000000"    // QTY    ]  ],  "asks": [    [      "4.00000200",      "12.00000000"    ]  ]}
```

## RECENT TRADES LIST

Recent Trades List
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Recent-Trades-List#api-description)
----------

Get recent market trades

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Recent-Trades-List#http-request)
----------

GET `/fapi/v1/trades`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Recent-Trades-List#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Recent-Trades-List#request-parameters)
----------

| Name | Type |Mandatory|     Description      |
|------|------|---------|----------------------|
|symbol|STRING|   YES   |                      |
|limit | INT  |   NO    |Default 500; max 1000.|

>
>
> * Market trades means trades filled in the order book. Only market trades will be returned, which means the insurance fund trades and ADL trades won't be returned.
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Recent-Trades-List#response-example)
----------

```
[  {    "id": 28457,    "price": "4.00000100",    "qty": "12.00000000",    "quoteQty": "48.00",    "time": 1499865549590,    "isBuyerMaker": true,  }]
```

## OLD TRADES LOOKUP

Old Trades Lookup (MARKET\_DATA)
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Old-Trades-Lookup#api-description)
----------

Get older market historical trades.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Old-Trades-Lookup#http-request)
----------

GET `/fapi/v1/historicalTrades`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Old-Trades-Lookup#request-weight)
----------

**20**

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Old-Trades-Lookup#request-parameters)
----------

| Name | Type |Mandatory|                      Description                      |
|------|------|---------|-------------------------------------------------------|
|symbol|STRING|   YES   |                                                       |
|limit | INT  |   NO    |                 Default 100; max 500.                 |
|fromId| LONG |   NO    |TradeId to fetch from. Default gets most recent trades.|

>
>
> * Market trades means trades filled in the order book. Only market trades will be returned, which means the insurance fund trades and ADL trades won't be returned.
> * Only supports data from within the last three months
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Old-Trades-Lookup#response-example)
----------

```
[  {    "id": 28457,    "price": "4.00000100",    "qty": "12.00000000",    "quoteQty": "8000.00",    "time": 1499865549590,    "isBuyerMaker": true,  }]
```

## COMPRESSED AGGREGATE TRADES LIST

Compressed/Aggregate Trades List
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List#api-description)
----------

Get compressed, aggregate market trades. Market trades that fill in 100ms with the same price and the same taking side will have the quantity aggregated.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List#http-request)
----------

GET `/fapi/v1/aggTrades`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List#request-weight)
----------

20

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List#request-parameters)
----------

|  Name   | Type |Mandatory|                      Description                       |
|---------|------|---------|--------------------------------------------------------|
| symbol  |STRING|   YES   |                                                        |
| fromId  | LONG |   NO    |       ID to get aggregate trades from INCLUSIVE.       |
|startTime| LONG |   NO    |Timestamp in ms to get aggregate trades from INCLUSIVE. |
| endTime | LONG |   NO    |Timestamp in ms to get aggregate trades until INCLUSIVE.|
|  limit  | INT  |   NO    |                 Default 500; max 1000.                 |

>
>
> * support querying futures trade histories that are not older than one year
> * If both `startTime` and `endTime` are sent, time between `startTime` and `endTime` must be less than 1 hour.
> * If `fromId`, `startTime`, and `endTime` are not sent, the most recent aggregate trades will be returned.
> * Only market trades will be aggregated and returned, which means the insurance fund trades and ADL trades won't be aggregated.
> * Sending both `startTime`/`endTime` and `fromId` might cause response timeout, please send either `fromId` or `startTime`/`endTime`
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List#response-example)
----------

```
[  {    "a": 26129,         // Aggregate tradeId    "p": "0.01633102",  // Price    "q": "4.70443515",  // Quantity    "f": 27781,         // First tradeId    "l": 27781,         // Last tradeId    "T": 1498793709153, // Timestamp    "m": true,          // Was the buyer the maker?  }]
```

## KLINE CANDLESTICK DATA

Kline/Candlestick Data
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data#api-description)
----------

Kline/candlestick bars for a symbol.
Klines are uniquely identified by their open time.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data#http-request)
----------

GET `/fapi/v1/klines`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data#request-weight)
----------

based on parameter `LIMIT`

|   LIMIT   |weight|
|-----------|------|
|  [1,100)  |  1   |
|[100, 500) |  2   |
|[500, 1000]|  5   |
|  \> 1000  |  10  |

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data#request-parameters)
----------

|  Name   | Type |Mandatory|     Description      |
|---------|------|---------|----------------------|
| symbol  |STRING|   YES   |                      |
|interval | ENUM |   YES   |                      |
|startTime| LONG |   NO    |                      |
| endTime | LONG |   NO    |                      |
|  limit  | INT  |   NO    |Default 500; max 1500.|

>
>
> * If startTime and endTime are not sent, the most recent klines are returned.
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data#response-example)
----------

```
[  [    1499040000000,      // Open time    "0.01634790",       // Open    "0.80000000",       // High    "0.01575800",       // Low    "0.01577100",       // Close    "148976.11427815",  // Volume    1499644799999,      // Close time    "2434.19055334",    // Quote asset volume    308,                // Number of trades    "1756.87402397",    // Taker buy base asset volume    "28.46694368",      // Taker buy quote asset volume    "17928899.62484339" // Ignore.  ]]
```

## CONTINUOUS CONTRACT KLINE CANDLESTICK DATA

Continuous Contract Kline/Candlestick Data
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data#api-description)
----------

Kline/candlestick bars for a specific contract type.
Klines are uniquely identified by their open time.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data#http-request)
----------

GET `/fapi/v1/continuousKlines`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data#request-weight)
----------

based on parameter `LIMIT`

|   LIMIT   |weight|
|-----------|------|
|  [1,100)  |  1   |
|[100, 500) |  2   |
|[500, 1000]|  5   |
|  \> 1000  |  10  |

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data#request-parameters)
----------

|    Name    | Type |Mandatory|     Description      |
|------------|------|---------|----------------------|
|    pair    |STRING|   YES   |                      |
|contractType| ENUM |   YES   |                      |
|  interval  | ENUM |   YES   |                      |
| startTime  | LONG |   NO    |                      |
|  endTime   | LONG |   NO    |                      |
|   limit    | INT  |   NO    |Default 500; max 1500.|

>
>
> * If startTime and endTime are not sent, the most recent klines are returned.
>
>

>
>
> * Contract type:
>   * PERPETUAL
>   * CURRENT\_QUARTER
>   * NEXT\_QUARTER
>   
>   
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data#response-example)
----------

```
[  [    1607444700000,      	// Open time    "18879.99",       	 	// Open    "18900.00",       	 	// High    "18878.98",       	 	// Low    "18896.13",      	 	// Close (or latest price)    "492.363", 			 	// Volume    1607444759999,       	// Close time    "9302145.66080",    	// Quote asset volume    1874,             		// Number of trades    "385.983",    			// Taker buy volume    "7292402.33267",      	// Taker buy quote asset volume    "0" 					// Ignore.  ]]
```

## INDEX PRICE KLINE CANDLESTICK DATA

Index Price Kline/Candlestick Data
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data#api-description)
----------

Kline/candlestick bars for the index price of a pair.
Klines are uniquely identified by their open time.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data#http-request)
----------

GET `/fapi/v1/indexPriceKlines`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data#request-weight)
----------

based on parameter `LIMIT`

|   LIMIT   |weight|
|-----------|------|
|  [1,100)  |  1   |
|[100, 500) |  2   |
|[500, 1000]|  5   |
|  \> 1000  |  10  |

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data#request-parameters)
----------

|  Name   | Type |Mandatory|     Description      |
|---------|------|---------|----------------------|
|  pair   |STRING|   YES   |                      |
|interval | ENUM |   YES   |                      |
|startTime| LONG |   NO    |                      |
| endTime | LONG |   NO    |                      |
|  limit  | INT  |   NO    |Default 500; max 1500.|

* If startTime and endTime are not sent, the most recent klines are returned.

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data#response-example)
----------

```
[  [    1591256400000,      	// Open time    "9653.69440000",    	// Open    "9653.69640000",     	// High    "9651.38600000",     	// Low    "9651.55200000",     	// Close (or latest price)    "0	", 					// Ignore    1591256459999,      	// Close time    "0",    				// Ignore    60,                		// Ignore    "0",    				// Ignore    "0",      				// Ignore    "0" 					// Ignore  ]]
```

## MARK PRICE KLINE CANDLESTICK DATA

Mark Price Kline/Candlestick Data
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price-Kline-Candlestick-Data#api-description)
----------

Kline/candlestick bars for the mark price of a symbol.
Klines are uniquely identified by their open time.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price-Kline-Candlestick-Data#http-request)
----------

GET `/fapi/v1/markPriceKlines`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price-Kline-Candlestick-Data#request-weight)
----------

based on parameter `LIMIT`

|   LIMIT   |weight|
|-----------|------|
|  [1,100)  |  1   |
|[100, 500) |  2   |
|[500, 1000]|  5   |
|  \> 1000  |  10  |

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price-Kline-Candlestick-Data#request-parameters)
----------

|  Name   | Type |Mandatory|     Description      |
|---------|------|---------|----------------------|
| symbol  |STRING|   YES   |                      |
|interval | ENUM |   YES   |                      |
|startTime| LONG |   NO    |                      |
| endTime | LONG |   NO    |                      |
|  limit  | INT  |   NO    |Default 500; max 1500.|

>
>
> * If startTime and endTime are not sent, the most recent klines are returned.
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price-Kline-Candlestick-Data#response-example)
----------

```
[  [    1591256460000,     		// Open time    "9653.29201333",    	// Open    "9654.56401333",     	// High    "9653.07367333",     	// Low    "9653.07367333",     	// Close (or latest price)    "0	", 					// Ignore    1591256519999,      	// Close time    "0",    				// Ignore    60,                	 	// Ignore    "0",    				// Ignore    "0",      			 	// Ignore    "0" 					// Ignore  ]]
```

## PREMIUM INDEX KLINE DATA

Premium index Kline Data
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Premium-Index-Kline-Data#api-description)
----------

Premium index kline bars of a symbol. Klines are uniquely identified by their open time.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Premium-Index-Kline-Data#http-request)
----------

GET `/fapi/v1/premiumIndexKlines`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Premium-Index-Kline-Data#request-weight)
----------

based on parameter `LIMIT`

|   LIMIT   |weight|
|-----------|------|
|  [1,100)  |  1   |
|[100, 500) |  2   |
|[500, 1000]|  5   |
|  \> 1000  |  10  |

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Premium-Index-Kline-Data#request-parameters)
----------

|  Name   | Type |Mandatory|     Description      |
|---------|------|---------|----------------------|
| symbol  |STRING|   YES   |                      |
|interval | ENUM |   YES   |                      |
|startTime| LONG |   NO    |                      |
| endTime | LONG |   NO    |                      |
|  limit  | INT  |   NO    |Default 500; max 1500.|

>
>
> * If startTime and endTime are not sent, the most recent klines are returned.
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Premium-Index-Kline-Data#response-example)
----------

```
[  [    1691603820000,          // Open time    "-0.00042931",          // Open    "-0.00023641",          // High    "-0.00059406",          // Low    "-0.00043659",          // Close    "0",                    // Ignore    1691603879999,          // Close time    "0",                    // Ignore    12,                     // Ignore    "0",                    // Ignore    "0",                    // Ignore    "0"                     // Ignore  ]]
```

## MARK PRICE

Mark Price
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price#api-description)
----------

Mark Price and Funding Rate

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price#http-request)
----------

GET `/fapi/v1/premiumIndex`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price#request-parameters)
----------

| Name | Type |Mandatory|Description|
|------|------|---------|-----------|
|symbol|STRING|   NO    |           |

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price#response-example)
----------

>
>
> **Response:**
>
>

```
{	"symbol": "BTCUSDT",	"markPrice": "11793.63104562",	// mark price	"indexPrice": "11781.80495970",	// index price	"estimatedSettlePrice": "11781.16138815", // Estimated Settle Price, only useful in the last hour before the settlement starts.	"lastFundingRate": "0.00038246",  // This is the Latest funding rate	"interestRate": "0.00010000",	"nextFundingTime": 1597392000000,	"time": 1597370495002}
```

>
>
> **OR (when symbol not sent)**
>
>

```
[	{	    "symbol": "BTCUSDT",	    "markPrice": "11793.63104562",	// mark price	    "indexPrice": "11781.80495970",	// index price	    "estimatedSettlePrice": "11781.16138815", // Estimated Settle Price, only useful in the last hour before the settlement starts.	    "lastFundingRate": "0.00038246",  // This is the Latest funding rate	    "interestRate": "0.00010000",	    "nextFundingTime": 1597392000000,	    "time": 1597370495002	}]
```

## GET FUNDING RATE HISTORY

Get Funding Rate History
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-History#api-description)
----------

Get Funding Rate History

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-History#http-request)
----------

GET `/fapi/v1/fundingRate`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-History#request-weight)
----------

share 500/5min/IP rate limit with GET /fapi/v1/fundingInfo

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-History#request-parameters)
----------

|  Name   | Type |Mandatory|                    Description                     |
|---------|------|---------|----------------------------------------------------|
| symbol  |STRING|   NO    |                                                    |
|startTime| LONG |   NO    |Timestamp in ms to get funding rate from INCLUSIVE. |
| endTime | LONG |   NO    |Timestamp in ms to get funding rate until INCLUSIVE.|
|  limit  | INT  |   NO    |               Default 100; max 1000                |

>
>
> * If `startTime` and `endTime` are not sent, the most recent `limit` datas are returned.
> * If the number of data between `startTime` and `endTime` is larger than `limit`, return as `startTime` + `limit`.
> * In ascending order.
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-History#response-example)
----------

```
[	{    	"symbol": "BTCUSDT",    	"fundingRate": "-0.03750000",    	"fundingTime": 1570608000000,		"markPrice": "34287.54619963"   // mark price associated with a particular funding fee charge	},	{   		"symbol": "BTCUSDT",    	"fundingRate": "0.00010000",    	"fundingTime": 1570636800000,		"markPrice": "34287.54619963" 	}]
```

## GET FUNDING RATE INFO

Get Funding Rate Info
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-Info#api-description)
----------

Query funding rate info for symbols that had FundingRateCap/ FundingRateFloor / fundingIntervalHours adjustment

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-Info#http-request)
----------

GET `/fapi/v1/fundingInfo`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-Info#request-weight)
----------

**0**share 500/5min/IP rate limit with `GET /fapi/v1/fundingInfo`

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-Info#request-parameters)
----------

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-Info#response-example)
----------

```
[    {        "symbol": "BLZUSDT",        "adjustedFundingRateCap": "0.02500000",        "adjustedFundingRateFloor": "-0.02500000",        "fundingIntervalHours": 8,        "disclaimer": false   // ingore    }]
```

## 24HR TICKER PRICE CHANGE STATISTICS

24hr Ticker Price Change Statistics
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics#api-description)
----------

24 hour rolling window price change statistics.  
**Careful** when accessing this with no symbol.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics#http-request)
----------

GET `/fapi/v1/ticker/24hr`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics#request-weight)
----------

**1** for a single symbol;  
**40** when the symbol parameter is omitted

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics#request-parameters)
----------

| Name | Type |Mandatory|Description|
|------|------|---------|-----------|
|symbol|STRING|   NO    |           |

>
>
> * If the symbol is not sent, tickers for all symbols will be returned in an array.
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics#response-example)
----------

>
>
> **Response:**
>
>

```
{  "symbol": "BTCUSDT",  "priceChange": "-94.99999800",  "priceChangePercent": "-95.960",  "weightedAvgPrice": "0.29628482",  "lastPrice": "4.00000200",  "lastQty": "200.00000000",  "openPrice": "99.00000000",  "highPrice": "100.00000000",  "lowPrice": "0.10000000",  "volume": "8913.30000000",  "quoteVolume": "15.30000000",  "openTime": 1499783499040,  "closeTime": 1499869899040,  "firstId": 28385,   // First tradeId  "lastId": 28460,    // Last tradeId  "count": 76         // Trade count}
```

>
>
> OR
>
>

```
[	{  		"symbol": "BTCUSDT",  		"priceChange": "-94.99999800",  		"priceChangePercent": "-95.960",  		"weightedAvgPrice": "0.29628482",  		"lastPrice": "4.00000200",  		"lastQty": "200.00000000",  		"openPrice": "99.00000000",  		"highPrice": "100.00000000",  		"lowPrice": "0.10000000",  		"volume": "8913.30000000",  		"quoteVolume": "15.30000000",  		"openTime": 1499783499040,  		"closeTime": 1499869899040,  		"firstId": 28385,   // First tradeId  		"lastId": 28460,    // Last tradeId  		"count": 76         // Trade count	}]
```

## SYMBOL PRICE TICKER

Symbol Price Ticker
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker#api-description)
----------

Latest price for a symbol or symbols.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker#http-request)
----------

GET `/fapi/v1/ticker/price`

**Weight:**

**1** for a single symbol;  
**2** when the symbol parameter is omitted

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker#request-parameters)
----------

| Name | Type |Mandatory|Description|
|------|------|---------|-----------|
|symbol|STRING|   NO    |           |

>
>
> * If the symbol is not sent, prices for all symbols will be returned in an array.
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker#response-example)
----------

```
{  "symbol": "BTCUSDT",  "price": "6000.01",  "time": 1589437530011   // Transaction time}
```

>
>
> OR
>
>

```
[	{  		"symbol": "BTCUSDT",  		"price": "6000.01",  		"time": 1589437530011	}]
```

## SYMBOL PRICE TICKER V2

Symbol Price Ticker V2
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker-v2#api-description)
----------

Latest price for a symbol or symbols.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker-v2#http-request)
----------

GET `/fapi/v2/ticker/price`

**Weight:**

**1** for a single symbol;  
**2** when the symbol parameter is omitted

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker-v2#request-parameters)
----------

| Name | Type |Mandatory|Description|
|------|------|---------|-----------|
|symbol|STRING|   NO    |           |

>
>
> * If the symbol is not sent, prices for all symbols will be returned in an array.
> * The field `X-MBX-USED-WEIGHT-1M` in response header is not accurate from this endpoint, please ignore.
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker-v2#response-example)
----------

```
{  "symbol": "BTCUSDT",  "price": "6000.01",  "time": 1589437530011   // Transaction time}
```

>
>
> OR
>
>

```
[	{  		"symbol": "BTCUSDT",  		"price": "6000.01",  		"time": 1589437530011	}]
```

## SYMBOL ORDER BOOK TICKER

Symbol Order Book Ticker
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Order-Book-Ticker#api-description)
----------

Best price/qty on the order book for a symbol or symbols.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Order-Book-Ticker#http-request)
----------

GET `/fapi/v1/ticker/bookTicker`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Order-Book-Ticker#request-weight)
----------

**2** for a single symbol;  
**5** when the symbol parameter is omitted

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Order-Book-Ticker#request-parameters)
----------

| Name | Type |Mandatory|Description|
|------|------|---------|-----------|
|symbol|STRING|   NO    |           |

>
>
> * If the symbol is not sent, bookTickers for all symbols will be returned in an array.
> * The field `X-MBX-USED-WEIGHT-1M` in response header is not accurate from this endpoint, please ignore.
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Order-Book-Ticker#response-example)
----------

```
{  "symbol": "BTCUSDT",  "bidPrice": "4.00000000",  "bidQty": "431.00000000",  "askPrice": "4.00000200",  "askQty": "9.00000000",  "time": 1589437530011   // Transaction time}
```

>
>
> OR
>
>

```
[	{  		"symbol": "BTCUSDT",  		"bidPrice": "4.00000000",  		"bidQty": "431.00000000",  		"askPrice": "4.00000200",  		"askQty": "9.00000000",  		"time": 1589437530011	}]
```

## DELIVERY PRICE

Quarterly Contract Settlement Price
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Delivery-Price#api-description)
----------

Latest price for a symbol or symbols.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Delivery-Price#http-request)
----------

GET `/futures/data/delivery-price`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Delivery-Price#request-weight)
----------

**0**

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Delivery-Price#request-parameters)
----------

|Name| Type |Mandatory|Description|
|----|------|---------|-----------|
|pair|STRING|   YES   |e.g BTCUSDT|

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Delivery-Price#response-example)
----------

```
[    {        "deliveryTime": 1695945600000,        "deliveryPrice": 27103.00000000    },    {        "deliveryTime": 1688083200000,        "deliveryPrice": 30733.60000000    },    {        "deliveryTime": 1680220800000,        "deliveryPrice": 27814.20000000    },    {        "deliveryTime": 1648166400000,        "deliveryPrice": 44066.30000000    }]
```

## OPEN INTEREST

Open Interest
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest#api-description)
----------

Get present open interest of a specific symbol.

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest#http-request)
----------

GET `/fapi/v1/openInterest`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest#request-parameters)
----------

| Name | Type |Mandatory|Description|
|------|------|---------|-----------|
|symbol|STRING|   YES   |           |

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest#response-example)
----------

```
{	"openInterest": "10659.509", 	"symbol": "BTCUSDT",	"time": 1589437530011   // Transaction time}
```

## OPEN INTEREST STATISTICS

Open Interest Statistics
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest-Statistics#api-description)
----------

Open Interest Statistics

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest-Statistics#http-request)
----------

GET `/futures/data/openInterestHist`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest-Statistics#request-weight)
----------

**0**

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest-Statistics#request-parameters)
----------

|  Name   | Type |Mandatory|                  Description                  |
|---------|------|---------|-----------------------------------------------|
| symbol  |STRING|   YES   |                                               |
| period  | ENUM |   YES   |"5m","15m","30m","1h","2h","4h","6h","12h","1d"|
|  limit  | LONG |   NO    |              default 30, max 500              |
|startTime| LONG |   NO    |                                               |
| endTime | LONG |   NO    |                                               |

>
>
> * If startTime and endTime are not sent, the most recent data is returned.
> * Only the data of the latest 30 days is available.
> * IP rate limit 1000 requests/5min
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest-Statistics#response-example)
----------

```
[    {          "symbol":"BTCUSDT",	      "sumOpenInterest":"20403.63700000",  // total open interest 	      "sumOpenInterestValue": "150570784.07809979",   // total open interest value	      "timestamp":"1583127900000"    },         {          "symbol":"BTCUSDT",         "sumOpenInterest":"20401.36700000",         "sumOpenInterestValue":"149940752.14464448",         "timestamp":"1583128200000"        },   ]
```

## TOP TRADER LONG SHORT RATIO

Top Trader Long/Short Ratio (Positions)
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Trader-Long-Short-Ratio#api-description)
----------

The proportion of net long and net short positions to total open positions of the top 20% users with the highest margin balance.
Long Position % = Long positions of top traders / Total open positions of top traders
Short Position % = Short positions of top traders / Total open positions of top traders
Long/Short Ratio (Positions) = Long Position % / Short Position %

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Trader-Long-Short-Ratio#http-request)
----------

GET `/futures/data/topLongShortPositionRatio`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Trader-Long-Short-Ratio#request-weight)
----------

**0**

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Trader-Long-Short-Ratio#request-parameters)
----------

|  Name   | Type |Mandatory|                  Description                  |
|---------|------|---------|-----------------------------------------------|
| symbol  |STRING|   YES   |                                               |
| period  | ENUM |   YES   |"5m","15m","30m","1h","2h","4h","6h","12h","1d"|
|  limit  | LONG |   NO    |              default 30, max 500              |
|startTime| LONG |   NO    |                                               |
| endTime | LONG |   NO    |                                               |

>
>
> * If startTime and endTime are not sent, the most recent data is returned.
> * Only the data of the latest 30 days is available.
> * IP rate limit 1000 requests/5min
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Trader-Long-Short-Ratio#response-example)
----------

```
[    {          "symbol":"BTCUSDT",	      "longShortRatio":"1.4342",// long/short position ratio of top traders	      "longAccount": "0.5891", // long positions ratio of top traders	      "shortAccount":"0.4108", // short positions ratio of top traders	      "timestamp":"1583139600000"         },          {                  "symbol":"BTCUSDT",	      "longShortRatio":"1.4337",	      "longAccount": "0.3583", 	      "shortAccount":"0.6417", 	                	      "timestamp":"1583139900000"	                       },   	    ]
```

## TOP LONG SHORT ACCOUNT RATIO

Top Trader Long/Short Ratio (Accounts)
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Long-Short-Account-Ratio#api-description)
----------

The proportion of net long and net short accounts to total accounts of the top 20% users with the highest margin balance. Each account is counted once only.
Long Account % = Accounts of top traders with net long positions / Total accounts of top traders with open positions
Short Account % = Accounts of top traders with net short positions / Total accounts of top traders with open positions
Long/Short Ratio (Accounts) = Long Account % / Short Account %

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Long-Short-Account-Ratio#http-request)
----------

GET `/futures/data/topLongShortAccountRatio`

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Long-Short-Account-Ratio#request-parameters)
----------

|  Name   | Type |Mandatory|                  Description                  |
|---------|------|---------|-----------------------------------------------|
| symbol  |STRING|   YES   |                                               |
| period  | ENUM |   YES   |"5m","15m","30m","1h","2h","4h","6h","12h","1d"|
|  limit  | LONG |   NO    |              default 30, max 500              |
|startTime| LONG |   NO    |                                               |
| endTime | LONG |   NO    |                                               |

>
>
> * If startTime and endTime are not sent, the most recent data is returned.
> * Only the data of the latest 30 days is available.
> * IP rate limit 1000 requests/5min
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Long-Short-Account-Ratio#response-example)
----------

```
[    {          "symbol":"BTCUSDT",	      "longShortRatio":"1.8105",  // long/short account num ratio of top traders	      "longAccount": "0.6442",   // long account num ratio of top traders 	      "shortAccount":"0.3558",   // long account num ratio of top traders 	      "timestamp":"1583139600000"    },     {              "symbol":"BTCUSDT",	      "longShortRatio":"0.5576",	      "longAccount": "0.3580", 	      "shortAccount":"0.6420", 	                	      "timestamp":"1583139900000"             }  ]
```

## LONG SHORT RATIO

Long/Short Ratio
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Long-Short-Ratio#api-description)
----------

Query symbol Long/Short Ratio

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Long-Short-Ratio#http-request)
----------

GET `/futures/data/globalLongShortAccountRatio`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Long-Short-Ratio#request-weight)
----------

**0**

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Long-Short-Ratio#request-parameters)
----------

|  Name   | Type |Mandatory|                  Description                  |
|---------|------|---------|-----------------------------------------------|
| symbol  |STRING|   YES   |                                               |
| period  | ENUM |   YES   |"5m","15m","30m","1h","2h","4h","6h","12h","1d"|
|  limit  | LONG |   NO    |              default 30, max 500              |
|startTime| LONG |   NO    |                                               |
| endTime | LONG |   NO    |                                               |

>
>
> * If startTime and endTime are not sent, the most recent data is returned.
> * Only the data of the latest 30 days is available.
> * IP rate limit 1000 requests/5min
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Long-Short-Ratio#response-example)
----------

```
[    {          "symbol":"BTCUSDT",  // long/short account num ratio of all traders	      "longShortRatio":"0.1960",  //long account num ratio of all traders	      "longAccount": "0.6622",   // short account num ratio of all traders	      "shortAccount":"0.3378", 	      "timestamp":"1583139600000"         },          {                  "symbol":"BTCUSDT",	      "longShortRatio":"1.9559",	      "longAccount": "0.6617", 	      "shortAccount":"0.3382", 	                	      "timestamp":"1583139900000"	                       },   	    ]
```

## TAKER BUYSELL VOLUME

Taker Buy/Sell Volume
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Taker-BuySell-Volume#api-description)
----------

Taker Buy/Sell Volume

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Taker-BuySell-Volume#http-request)
----------

GET `/futures/data/takerlongshortRatio`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Taker-BuySell-Volume#request-weight)
----------

**0**

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Taker-BuySell-Volume#request-parameters)
----------

|  Name   | Type |Mandatory|                  Description                  |
|---------|------|---------|-----------------------------------------------|
| symbol  |STRING|   YES   |                                               |
| period  | ENUM |   YES   |"5m","15m","30m","1h","2h","4h","6h","12h","1d"|
|  limit  | LONG |   NO    |              default 30, max 500              |
|startTime| LONG |   NO    |                                               |
| endTime | LONG |   NO    |                                               |

>
>
> * If startTime and endTime are not sent, the most recent data is returned.
> * Only the data of the latest 30 days is available.
> * IP rate limit 1000 requests/5min
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Taker-BuySell-Volume#response-example)
----------

```
[    { 	    "buySellRatio":"1.5586",	    "buyVol": "387.3300", 	    "sellVol":"248.5030", 	    "timestamp":"1585614900000"    },    { 	    "buySellRatio":"1.3104",	    "buyVol": "343.9290", 	    "sellVol":"248.5030", 	                	    "timestamp":"1583139900000"            },    ]
```

## BASIS

Basis
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Basis#api-description)
----------

Query future basis

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Basis#http-request)
----------

GET `/futures/data/basis`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Basis#request-weight)
----------

**0**

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Basis#request-parameters)
----------

|    Name    | Type |Mandatory|                  Description                  |
|------------|------|---------|-----------------------------------------------|
|    pair    |STRING|   YES   |                    BTCUSDT                    |
|contractType| ENUM |   YES   |  CURRENT\_QUARTER, NEXT\_QUARTER, PERPETUAL   |
|   period   | ENUM |   YES   |"5m","15m","30m","1h","2h","4h","6h","12h","1d"|
|   limit    | LONG |   YES   |              Default 30,Max 500               |
| startTime  | LONG |   NO    |                                               |
|  endTime   | LONG |   NO    |                                               |

>
>
> * If startTime and endTime are not sent, the most recent data is returned.
> * Only the data of the latest 30 days is available.
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Basis#response-example)
----------

```
[      {        "indexPrice": "34400.15945055",        "contractType": "PERPETUAL",        "basisRate": "0.0004",        "futuresPrice": "34414.10",        "annualizedBasisRate": "",        "basis": "13.94054945",        "pair": "BTCUSDT",        "timestamp": 1698742800000    }]
```

## COMPOSITE INDEX SYMBOL INFORMATION

Composite Index Symbol Information
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Composite-Index-Symbol-Information#api-description)
----------

Query composite index symbol information

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Composite-Index-Symbol-Information#http-request)
----------

GET `/fapi/v1/indexInfo`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Composite-Index-Symbol-Information#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Composite-Index-Symbol-Information#request-parameters)
----------

| Name | Type |Mandatory|Description|
|------|------|---------|-----------|
|symbol|STRING|   NO    |           |

>
>
> * Only for composite index symbols
>
>

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Composite-Index-Symbol-Information#response-example)
----------

```
[	{ 		"symbol": "DEFIUSDT",		"time": 1589437530011,    // Current time		"component": "baseAsset", //Component asset		"baseAssetList":[			{				"baseAsset":"BAL",				"quoteAsset": "USDT",				"weightInQuantity":"1.04406228",				"weightInPercentage":"0.02783900"			},			{				"baseAsset":"BAND",				"quoteAsset": "USDT",				"weightInQuantity":"3.53782729",				"weightInPercentage":"0.03935200"			}		]	}]
```

## MULTI ASSETS MODE ASSET INDEX

Multi-Assets Mode Asset Index
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Multi-Assets-Mode-Asset-Index#api-description)
----------

asset index for Multi-Assets mode

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Multi-Assets-Mode-Asset-Index#http-request)
----------

GET `/fapi/v1/assetIndex`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Multi-Assets-Mode-Asset-Index#request-weight)
----------

**1** for a single symbol; **10** when the symbol parameter is omitted

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Multi-Assets-Mode-Asset-Index#request-parameters)
----------

| Name | Type |Mandatory|Description|
|------|------|---------|-----------|
|symbol|STRING|   NO    |Asset pair |

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Multi-Assets-Mode-Asset-Index#response-example)
----------

>
>
> **Response:**
>
>

```
{	"symbol": "ADAUSD",	"time": 1635740268004,	"index": "1.92957370",	"bidBuffer": "0.10000000", 	"askBuffer": "0.10000000", 	"bidRate": "1.73661633",	"askRate": "2.12253107",	"autoExchangeBidBuffer": "0.05000000",	"autoExchangeAskBuffer": "0.05000000",	"autoExchangeBidRate": "1.83309501",	"autoExchangeAskRate": "2.02605238"}
```

>
>
> Or(without symbol)
>
>

```
[	{		"symbol": "ADAUSD",		"time": 1635740268004,		"index": "1.92957370",		"bidBuffer": "0.10000000", 		"askBuffer": "0.10000000", 		"bidRate": "1.73661633",		"askRate": "2.12253107",		"autoExchangeBidBuffer": "0.05000000",		"autoExchangeAskBuffer": "0.05000000",		"autoExchangeBidRate": "1.83309501",		"autoExchangeAskRate": "2.02605238"	}]
```

## INDEX CONSTITUENTS

Query Index Price Constituents
==========

API Description[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Constituents#api-description)
----------

Query index price constituents

HTTP Request[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Constituents#http-request)
----------

GET `/fapi/v1/constituents`

Request Weight[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Constituents#request-weight)
----------

**2**

Request Parameters[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Constituents#request-parameters)
----------

| Name | Type |Mandatory|Description|
|------|------|---------|-----------|
|symbol|STRING|   YES   |           |

Response Example[​](/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Constituents#response-example)
----------

```
{    "symbol": "BTCUSDT",    "time": 1697421272043,    "constituents": [        {            "exchange": "binance",            "symbol": "BTCUSDT"        },        {            "exchange": "okex",            "symbol": "BTC-USDT"        },        {            "exchange": "huobi",            "symbol": "btcusdt"        },        {            "exchange": "coinbase",            "symbol": "BTC-USDT"        }    ]}
```

