# Binance Derivatives COINM Private REST API Documentation

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

testnet[​](/docs/derivatives/coin-margined-futures/general-info#testnet)
----------

* Most of the endpoints can be also used in the testnet platform.
* The REST baseurl for **testnet** is "[https://testnet.binancefuture.com](https://testnet.binancefuture.com)"
* The Websocket baseurl for **testnet** is "wss://dstream.binancefuture.com"

General API Information[​](/docs/derivatives/coin-margined-futures/general-info#general-api-information)
----------

* The base endpoint is: **[https://dapi.binance.com](https://dapi.binance.com)**
* All endpoints return either a JSON object or array.
* Data is returned in **ascending** order. Oldest first, newest last.
* All time and timestamp related fields are in milliseconds.
* All data types adopt definition in JAVA.

### HTTP Return Codes[​](/docs/derivatives/coin-margined-futures/general-info#http-return-codes) ###

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

### Error Codes and Messages[​](/docs/derivatives/coin-margined-futures/general-info#error-codes-and-messages) ###

* Any endpoint can return an ERROR

>
>
> ***The error payload is as follows:***
>
>

```
{  "code": -1121,  "msg": "Invalid symbol."}
```

* Specific error codes and messages defined in [Error Codes](/docs/derivatives/coin-margined-futures/general-info#error-codes).

### General Information on Endpoints[​](/docs/derivatives/coin-margined-futures/general-info#general-information-on-endpoints) ###

* For `GET` endpoints, parameters must be sent as a `query string`.
* For `POST`, `PUT`, and `DELETE` endpoints, the parameters may be sent as a`query string` or in the `request body` with content type`application/x-www-form-urlencoded`. You may mix parameters between both the`query string` and `request body` if you wish to do so.
* Parameters may be sent in any order.
* If a parameter sent in both the `query string` and `request body`, the`query string` parameter will be used.

LIMITS[​](/docs/derivatives/coin-margined-futures/general-info#limits)
----------

* The `/dapi/v1/exchangeInfo` `rateLimits` array contains objects related to the exchange's `RAW_REQUEST`, `REQUEST_WEIGHT`, and `ORDER` rate limits. These are further defined in the `ENUM definitions` section under `Rate limiters (rateLimitType)`.
* A `429` will be returned when either rate limit is violated.

Binance has the right to further tighten the rate limits on users with intent to attack.

### IP Limits[​](/docs/derivatives/coin-margined-futures/general-info#ip-limits) ###

* Every request will contain `X-MBX-USED-WEIGHT-(intervalNum)(intervalLetter)` in the response headers which has the current used weight for the IP for all request rate limiters defined.
* Each route has a `weight` which determines for the number of requests each endpoint counts for. Heavier endpoints and endpoints that do operations on multiple symbols will have a heavier `weight`.
* When a 429 is received, it's your obligation as an API to back off and not spam the API.
* **Repeatedly violating rate limits and/or failing to back off after receiving 429s will result in an automated IP ban (HTTP status 418).**
* IP bans are tracked and **scale in duration** for repeat offenders, **from 2 minutes to 3 days**.
* **The limits on the API are based on the IPs, not the API keys.**

It is strongly recommended to use websocket stream for getting data as much as possible, which can not only ensure the timeliness of the message, but also reduce the access restriction pressure caused by the request.

### Order Rate Limits[​](/docs/derivatives/coin-margined-futures/general-info#order-rate-limits) ###

* Every order response will contain a `X-MBX-ORDER-COUNT-(intervalNum)(intervalLetter)` header which has the current order count for the account for all order rate limiters defined.
* Rejected/unsuccessful orders are not guaranteed to have `X-MBX-ORDER-COUNT-**` headers in the response.
* **The order rate limit is counted against each account**.

Endpoint Security Type[​](/docs/derivatives/coin-margined-futures/general-info#endpoint-security-type)
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

SIGNED (TRADE and USER\_DATA) Endpoint Security[​](/docs/derivatives/coin-margined-futures/general-info#signed-trade-and-user_data-endpoint-security)
----------

* `SIGNED` endpoints require an additional parameter, `signature`, to be
  sent in the `query string` or `request body`.
* Endpoints use `HMAC SHA256` signatures. The `HMAC SHA256 signature` is a keyed `HMAC SHA256` operation.
  Use your `secretKey` as the key and `totalParams` as the value for the HMAC operation.
* The `signature` is **not case sensitive**.
* Please make sure the `signature` is the end part of your `query string` or `request body`.
* `totalParams` is defined as the `query string` concatenated with the`request body`.

### Timing security[​](/docs/derivatives/coin-margined-futures/general-info#timing-security) ###

* A `SIGNED` endpoint also requires a parameter, `timestamp`, to be sent which
  should be the millisecond timestamp of when the request was created and sent.
* An additional parameter, `recvWindow`, may be sent to specify the number of
  milliseconds after `timestamp` the request is valid for. If `recvWindow`is not sent, **it defaults to 5000**.
* If the server determines that the timestamp sent by the client is more than **one second** in the future of the server time, the request will also be rejected.

>
>
> The logic is as follows:
>
>

```
  if (timestamp < (serverTime + 1000) && (serverTime - timestamp) <= recvWindow){    // process request  }   else {    // reject request  }
```

**Serious trading is about timing.** Networks can be unstable and unreliable,
which can lead to requests taking varying amounts of time to reach the
servers. With `recvWindow`, you can specify that the request must be
processed within a certain number of milliseconds or be rejected by the
server.

It is recommended to use a small recvWindow of 5000 or less!

### SIGNED Endpoint Examples for POST /dapi/v1/order - HMAC Keys[​](/docs/derivatives/coin-margined-futures/general-info#signed-endpoint-examples-for-post-dapiv1order---hmac-keys) ###

Here is a step-by-step example of how to send a vaild signed payload from the
Linux command line using `echo`, `openssl`, and `curl`.

|   Key   |                             Value                              |
|---------|----------------------------------------------------------------|
| apiKey  |dbefbc809e3e83c283a984c3a1459732ea7db1360ca80c5c2c8867408d28cc83|
|secretKey|2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9|

| Parameter |    Value     |
|-----------|--------------|
|  symbol   |BTCUSD\_200925|
|   side    |     BUY      |
|   type    |    LIMIT     |
|timeInForce|     GTC      |
| quantity  |      1       |
|   price   |     9000     |
|recvWindow |     5000     |
| timestamp |1591702613943 |

#### Example 1: As a query string[​](/docs/derivatives/coin-margined-futures/general-info#example-1-as-a-query-string) ####

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
    $ echo -n "symbol=BTCUSD_200925&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943" | openssl dgst -sha256 -hmac "2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9"    (stdin)= 21fd819734bf0e5c68740eed892909414d693635c5f7fffab1313925ae13556a
```

>
>
> **curl command:**
>
>

```
    (HMAC SHA256)    $ curl -H "X-MBX-APIKEY: dbefbc809e3e83c283a984c3a1459732ea7db1360ca80c5c2c8867408d28cc83" -X POST 'https://dapi.binance.com/dapi/v1/order?symbol=BTCUSD_200925&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943&signature= 21fd819734bf0e5c68740eed892909414d693635c5f7fffab1313925ae13556a'
```

* **queryString:**

  symbol=BTCUSD\_200925  
  &side=BUY  
  &type=LIMIT  
  &timeInForce=GTC  
  &quantity=1  
  &price=9000  
  &recvWindow=5000  
  &timestamp=1591702613943

#### Example 2: As a request body[​](/docs/derivatives/coin-margined-futures/general-info#example-2-as-a-request-body) ####

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
    $ echo -n "symbol=BTCUSD_200925&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943" | openssl dgst -sha256 -hmac "2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9"    (stdin)= 21fd819734bf0e5c68740eed892909414d693635c5f7fffab1313925ae13556a
```

>
>
> **curl command:**
>
>

```
    (HMAC SHA256)    $ curl -H "X-MBX-APIKEY: dbefbc809e3e83c283a984c3a1459732ea7db1360ca80c5c2c8867408d28cc83" -X POST 'https://dapi.binance.com/dapi/v1/order' -d 'symbol=BTCUSD_200925&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943&signature= 21fd819734bf0e5c68740eed892909414d693635c5f7fffab1313925ae13556a'
```

* **requestBody:**

  symbol=BTCUSD\_200925  
  &side=BUY  
  &type=LIMIT  
  &timeInForce=GTC  
  &quantity=1  
  &price=9000  
  &recvWindow=5000  
  &timestamp=1591702613943

#### Example 3: Mixed query string and request body[​](/docs/derivatives/coin-margined-futures/general-info#example-3-mixed-query-string-and-request-body) ####

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
    $ echo -n "symbol=BTCUSD_200925&side=BUY&type=LIMIT&timeInForce=GTCquantity=1&price=9000&recvWindow=5000&timestamp= 1591702613943" | openssl dgst -sha256 -hmac "2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9"    (stdin)= f3129e7c72c7727037891ad8a86b76a7dc514ba125a536775c8ba403b2d1b222
```

>
>
> **curl command:**
>
>

```
    (HMAC SHA256)    $ curl -H "X-MBX-APIKEY: dbefbc809e3e83c283a984c3a1459732ea7db1360ca80c5c2c8867408d28cc83" -X POST 'https://dapi.binance.com/dapi/v1/order?symbol=BTCUSD_200925&side=BUY&type=LIMIT&timeInForce=GTC' -d 'quantity=1&price=9000&recvWindow=5000&timestamp= 1591702613943&signature=f3129e7c72c7727037891ad8a86b76a7dc514ba125a536775c8ba403b2d1b222'
```

* **queryString:** symbol=BTCUSD\_200925&side=BUY&type=LIMIT&timeInForce=GTC
* **requestBody:** quantity=1&price=9000&recvWindow=5000&timestamp= 1591702613943

Note that the signature is different in example 3.  
There is no & between "GTC" and "quantity=1".

### SIGNED Endpoint Examples for POST /dapi/v1/order - RSA Keys[​](/docs/derivatives/coin-margined-futures/general-info#signed-endpoint-examples-for-post-dapiv1order---rsa-keys) ###

* This will be a step by step process how to create the signature payload to send a valid signed payload.
* We support `PKCS#8` currently.
* To get your API key, you need to upload your RSA Public Key to your account and a corresponding API key will be provided for you.

For this example, the private key will be referenced as `test-prv-key.pem`

| Key  |                             Value                              |
|------|----------------------------------------------------------------|
|apiKey|vE3BDAL1gP1UaexugRLtteaAHg3UO8Nza20uexEuW1Kh3tVwQfFHdAiyjjY428o2|

|Parameter |    Value    |
|----------|-------------|
|  symbol  |BTCUSD\_PERP |
|   side   |    SELL     |
|   type   |   MARKET    |
| quantity |     100     |
|recvWindow|   9999999   |
|timestamp |1671090801999|

>
>
> **Signature payload (with the listed parameters):**
>
>

```
timestamp=1671090801999&recvWindow=9999999&symbol=BTCUSD_PERP&side=SELL&type=MARKET&quantity=100
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
 $ echo -n 'timestamp=1671090801999&recvWindow=9999999&symbol=BTCUSD_PERP&side=SELL&type=MARKET&quantity=100' | openssl dgst -keyform PEM -sha256 -sign ./test-prv-key.pem
```

2.2 - Sign payload using RSASSA-PKCS1-v1\_5 algorithm with SHA-256 hash function.

>
>
> **Step 2.3**
>
>

```
$ echo -n 'timestamp=1671090801999&recvWindow=9999999&symbol=BTCUSD_PERP&side=SELL&type=MARKET&quantity=100' | openssl dgst -keyform PEM -sha256 -sign ./test-prv-key.pem | openssl enc -base64aap36wD5loVXizxvvPI3wz9Cjqwmb3KVbxoym0XeWG1jZq8umqrnSk8H8dkLQeySjgVY91Ufs%2BBGCW%2B4sZjQEpgAfjM76riNxjlD3coGGEsPsT2lG39R%2F1q72zpDs8pYcQ4A692NgHO1zXcgScTGgdkjp%2Brp2bcddKjyz5XBrBM%3D
```

2.3 - Encode output as base64 string.

>
>
> **Step 2.4**
>
>

```
$  echo -n 'timestamp=1671090801999&recvWindow=9999999&symbol=BTCUSD_PERP&side=SELL&type=MARKET&quantity=100' | openssl dgst -keyform PEM -sha256 -sign ./test-prv-key.pem | openssl enc -base64 | tr -d '\n'aap36wD5loVXizxvvPI3wz9Cjqwmb3KVbxoym0XeWG1jZq8umqrnSk8H8dkLQeySjgVY91Ufs%2BBGCW%2B4sZjQEpgAfjM76riNxjlD3coGGEsPsT2lG39R%2F1q72zpDs8pYcQ4A692NgHO1zXcgScTGgdkjp%2Brp2bcddKjyz5XBrBM%3D
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
 curl -H "X-MBX-APIKEY: vE3BDAL1gP1UaexugRLtteaAHg3UO8Nza20uexEuW1Kh3tVwQfFHdAiyjjY428o2" -X POST 'https://dapi.binance.com/dapi/v1/order?timestamp=1671090801999&recvWindow=9999999&symbol=BTCUSD_PERP&side=SELL&type=MARKET&quantity=100&signature=aap36wD5loVXizxvvPI3wz9Cjqwmb3KVbxoym0XeWG1jZq8umqrnSk8H8dkLQeySjgVY91Ufs%2BBGCW%2B4sZjQEpgAfjM76riNxjlD3coGGEsPsT2lG39R%2F1q72zpDs8pYcQ4A692NgHO1zXcgScTGgdkjp%2Brp2bcddKjyz5XBrBM%3D'
```

2.6 - curl command

>
>
> **Bash script**
>
>

```
#!/usr/bin/env bash# Set up authentication:apiKey="vE3BDAL1gP1UaexugRLtteaAHg3UO8Nza20uexEuW1Kh3tVwQfFHdAiyjjY428o2"   ### REPLACE THIS WITH YOUR API KEY# Set up the request:apiMethod="POST"apiCall="v1/order"apiParams="timestamp=1671090801999&recvWindow=9999999&symbol=BTCUSD_PERP&side=SELL&type=MARKET&quantity=100"function rawurlencode {    local value="$1"    local len=${#value}    local encoded=""    local pos c o    for (( pos=0 ; pos<len ; pos++ ))    do        c=${value:$pos:1}        case "$c" in            [-_.~a-zA-Z0-9] ) o="${c}" ;;            * )   printf -v o '%%%02x' "'$c"        esac        encoded+="$o"    done    echo "$encoded"}ts=$(date +%s000)paramsWithTs="$apiParams&timestamp=$ts"rawSignature=$(echo -n "$paramsWithTs" \               | openssl dgst -keyform PEM -sha256 -sign ./test-prv-key.pem \  ### THIS IS YOUR PRIVATE KEY. DO NOT SHARE THIS FILE WITH ANYONE.               | openssl enc -base64 \               | tr -d '\n')signature=$(rawurlencode "$rawSignature")curl -H "X-MBX-APIKEY: $apiKey" -X $apiMethod \    "https://dapi.binance.com/dapi/$apiCall?$paramsWithTs&signature=$signature"
```

A sample Bash script containing similar steps is available in the right side.

## COMMON DEFINITION

Public Endpoints Info
==========

Terminology[​](/docs/derivatives/coin-margined-futures/common-definition#terminology)
----------

* `symbol` refers to the symbol name of a contract symbol
* `pair` refers to the underlying symbol of a contracrt symbol
* `base asset` refers to the asset that is the `quantity` of a symbol.
* `quote asset` refers to the asset that is the `price` of a symbol.
* `margin asset` refers to the asset that is the `margin` of a symbol

ENUM definitions[​](/docs/derivatives/coin-margined-futures/common-definition#enum-definitions)
----------

**Symbol type:**

* DELIVERY\_CONTRACT
* PERPETUAL\_CONTRACT

**Contract type (contractType):**

* PERPETUAL
* CURRENT\_QUARTER
* NEXT\_QUARTER
* CURRENT\_QUARTER\_DELIVERING // Invalid type, only used for DELIVERING status
* NEXT\_QUARTER\_DELIVERING // Invalid type, only used for DELIVERING status
* PERPETUAL\_DELIVERING

**Contract status (contractStatus, status):**

* PENDING\_TRADING
* TRADING
* PRE\_DELIVERING
* DELIVERING
* DELIVERED

**Order status (status):**

* NEW
* PARTIALLY\_FILLED
* FILLED
* CANCELED
* EXPIRED

**Order types (type):**

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

* GTC - Good Till Cancel
* IOC - Immediate or Cancel
* FOK - Fill or Kill
* GTX - Good Till Crossing (Post Only)

**Working Type (workingType)**

* MARK\_PRICE
* CONTRACT\_PRICE

**New Order Response Type (newOrderRespType)**

* ACK
* RESULT

**Price Match (priceMatch)**

* NONE: no price match
* OPPONENT: counterparty best price
* OPPONENT\_5: counterparty 5th best price
* OPPONENT\_10: counterparty 10th best price
* OPPONENT\_20: counterparty 20th best price
* QUEUE: the best price on the same side of the order book
* QUEUE\_5: the 5th best price on the same side of the order book
* QUEUE\_10: the 10th best price on the same side of the order book
* QUEUE\_20: the 20th best price on the same side of the order book

**Self-Trade Prevention mode (selfTradePreventionMode)**

* NONE: No Self-Trade Prevention
* EXPIRE\_TAKER: expire taker order when STP trigger
* EXPIRE\_BOTH: expire taker and maker order when STP trigger
* EXPIRE\_MAKER: expire maker order when STP trigger

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

**Rate limiters (rateLimitType)**

>
>
> REQUEST\_WEIGHT
>
>

```
  {  	"rateLimitType": "REQUEST_WEIGHT",  	"interval": "MINUTE",  	"intervalNum": 1,  	"limit": 6000  }
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

Symbol filters[​](/docs/derivatives/coin-margined-futures/common-definition#symbol-filters)
----------

### PRICE\_FILTER[​](/docs/derivatives/coin-margined-futures/common-definition#price_filter) ###

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

### LOT\_SIZE[​](/docs/derivatives/coin-margined-futures/common-definition#lot_size) ###

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

### MARKET\_LOT\_SIZE[​](/docs/derivatives/coin-margined-futures/common-definition#market_lot_size) ###

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

### MAX\_NUM\_ORDERS[​](/docs/derivatives/coin-margined-futures/common-definition#max_num_orders) ###

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

### PERCENT\_PRICE[​](/docs/derivatives/coin-margined-futures/common-definition#percent_price) ###

>
>
> **/exchangeInfo format:**
>
>

```
  {    "filterType": "PERCENT_PRICE",    "multiplierUp": "1.0500",    "multiplierDown": "0.9500",    "multiplierDecimal": 4  }
```

The `PERCENT_PRICE` filter defines valid range for a price based on the mark price.

In order to pass the `percent price`, the following must be true for `price`:

* BUY: `price` \<= `markPrice` \* `multiplierUp`
* SELL: `price` \>= `markPrice` \* `multiplierDown`

## REST API

New Order (TRADE)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api#api-description)
----------

Send in a new order.

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api#http-request)
----------

POST `/dapi/v1/order`

Request Weight(IP)[​](/docs/derivatives/coin-margined-futures/trade/rest-api#request-weightip)
----------

1 on 1min order rate limit(X-MBX-ORDER-COUNT-1M)  
0 on IP rate limit(x-mbx-used-weight-1m)

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api#request-parameters)
----------

|         Name          | Type  |Mandatory|                                                                                                  Description                                                                                                  |
|-----------------------|-------|---------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|        symbol         |STRING |   YES   |                                                                                                                                                                                                               |
|         side          | ENUM  |   YES   |                                                                                                                                                                                                               |
|     positionSide      | ENUM  |   NO    |                                                      Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent in Hedge Mode.                                                       |
|         type          | ENUM  |   YES   |                                                                                                                                                                                                               |
|      timeInForce      | ENUM  |   NO    |                                                                                                                                                                                                               |
|       quantity        |DECIMAL|   NO    |                                                               quantity measured by contract number, Cannot be sent with `closePosition`=`true`                                                                |
|      reduceOnly       |STRING |   NO    |                                            "true" or "false". default "false". Cannot be sent in Hedge Mode; cannot be sent with `closePosition`=`true`(Close-All)                                            |
|         price         |DECIMAL|   NO    |                                                                                                                                                                                                               |
|   newClientOrderId    |STRING |   NO    |                                    A unique id among open orders. Automatically generated if not sent. Can only be string following the rule: `^[\.A-Z\:/a-z0-9_-]{1,36}$`                                    |
|       stopPrice       |DECIMAL|   NO    |                                                                   Used with `STOP/STOP_MARKET` or `TAKE_PROFIT/TAKE_PROFIT_MARKET` orders.                                                                    |
|     closePosition     |STRING |   NO    |                                                                  `true`, `false`；Close-All,used with `STOP_MARKET` or `TAKE_PROFIT_MARKET`.                                                                   |
|    activationPrice    |DECIMAL|   NO    |                                                   Used with `TRAILING_STOP_MARKET` orders, default as the latest price(supporting different `workingType`)                                                    |
|     callbackRate      |DECIMAL|   NO    |                                                                    Used with `TRAILING_STOP_MARKET` orders, min 0.1, max 5 where 1 for 1%                                                                     |
|      workingType      | ENUM  |   NO    |                                                              stopPrice triggered by: "MARK\_PRICE", "CONTRACT\_PRICE". Default "CONTRACT\_PRICE"                                                              |
|     priceProtect      |STRING |   NO    |                                                 "TRUE" or "FALSE", default "FALSE". Used with `STOP/STOP_MARKET` or `TAKE_PROFIT/TAKE_PROFIT_MARKET` orders.                                                  |
|   newOrderRespType    | ENUM  |   NO    |                                                                                        "ACK", "RESULT", default "ACK"                                                                                         |
|      priceMatch       | ENUM  |   NO    |only avaliable for `LIMIT`/`STOP`/`TAKE_PROFIT` order; can be set to `OPPONENT`/ `OPPONENT_5`/ `OPPONENT_10`/ `OPPONENT_20`: /`QUEUE`/ `QUEUE_5`/ `QUEUE_10`/ `QUEUE_20`; Can't be passed together with `price`|
|selfTradePreventionMode| ENUM  |   NO    |         `NONE`:No STP / `EXPIRE_TAKER`:expire taker order when STP triggers/ `EXPIRE_MAKER`:expire taker order when STP triggers/ `EXPIRE_BOTH`:expire both orders when STP triggers; default `NONE`          |
|      recvWindow       | LONG  |   NO    |                                                                                                                                                                                                               |
|       timestamp       | LONG  |   YES   |                                                                                                                                                                                                               |

Additional mandatory parameters based on `type`:

|              Type              | Additional mandatory parameters  |
|--------------------------------|----------------------------------|
|            `LIMIT`             |`timeInForce`, `quantity`, `price`|
|            `MARKET`            |            `quantity`            |
|       `STOP/TAKE_PROFIT`       |       `price`, `stopPrice`       |
|`STOP_MARKET/TAKE_PROFIT_MARKET`|           `stopPrice`            |
|     `TRAILING_STOP_MARKET`     |          `callbackRate`          |

>
>
> * Order with type `STOP`, parameter `timeInForce` can be sent ( default `GTC`).
>   
>   
> * Order with type `TAKE_PROFIT`, parameter `timeInForce` can be sent ( default `GTC`).
>   
>   
> * Condition orders will be triggered when:
>   
>   
>   * If parameter`priceProtect`is sent as true:
>     * when price reaches the `stopPrice` ，the difference rate between "MARK\_PRICE" and "CONTRACT\_PRICE" cannot be larger than the "triggerProtect" of the symbol
>     * "triggerProtect" of a symbol can be got from `GET /dapi/v1/exchangeInfo`
>     
>     
>   * `STOP`, `STOP_MARKET`:
>     * BUY: latest price ("MARK\_PRICE" or "CONTRACT\_PRICE") \>= `stopPrice`
>     * SELL: latest price ("MARK\_PRICE" or "CONTRACT\_PRICE") \<= `stopPrice`
>     
>     
>   * `TAKE_PROFIT`, `TAKE_PROFIT_MARKET`:
>     * BUY: latest price ("MARK\_PRICE" or "CONTRACT\_PRICE") \<= `stopPrice`
>     * SELL: latest price ("MARK\_PRICE" or "CONTRACT\_PRICE") \>= `stopPrice`
>     
>     
>   * `TRAILING_STOP_MARKET`:
>     * BUY: the lowest price after order placed `<= `activationPrice`, and the latest price >`= the lowest price \* (1 + `callbackRate`)
>     * SELL: the highest price after order placed \>= `activationPrice`, and the latest price \<= the highest price \* (1 - `callbackRate`)
>     
>     
>   
>   
> * For `TRAILING_STOP_MARKET`, if you got such error code.  
>   `{"code": -2021, "msg": "Order would immediately trigger."}`  
>   means that the parameters you send do not meet the following requirements:
>   
>   
>   * BUY: `activationPrice` should be smaller than latest price.
>   * SELL: `activationPrice` should be larger than latest price.
>   
>   
> * If `newOrderRespType ` is sent as `RESULT` :
>   
>   
>   * `MARKET` order: the final FILLED result of the order will be return directly.
>   * `LIMIT` order with special `timeInForce`: the final status result of the order(FILLED or EXPIRED) will be returned directly.
>   
>   
> * `STOP_MARKET`, `TAKE_PROFIT_MARKET` with `closePosition`=`true`:
>   
>   
>   * Follow the same rules for condition orders.
>   * If triggered,**close all** current long position( if `SELL`) or current short position( if `BUY`).
>   * Cannot be used with `quantity` parameter
>   * Cannot be used with `reduceOnly` parameter
>   * In Hedge Mode,cannot be used with `BUY` orders in `LONG` position side. and cannot be used with `SELL` orders in `SHORT` position side
>   
>   
> * `selfTradePreventionMode` is only effective when `timeInForce` set to `IOC` or `GTC`.
>   
>   
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api#response-example)
----------

```
{ 	"clientOrderId": "testOrder", 	"cumQty": "0", 	"cumBase": "0", 	"executedQty": "0", 	"orderId": 22542179, 	"avgPrice": "0.0", 	"origQty": "10",  	"price": "0",  	"reduceOnly": false,  	"side": "BUY",  	"positionSide": "SHORT",   	"status": "NEW",  	"stopPrice": "9300",			   // please ignore when order type is TRAILING_STOP_MARKET  	"closePosition": false,  		   // if Close-All  	"symbol": "BTCUSD_200925",  	"pair": "BTCUSD",  	"timeInForce": "GTC",  	"type": "TRAILING_STOP_MARKET",  	"origType": "TRAILING_STOP_MARKET",  	"activatePrice": "9020",			// activation price, only return with TRAILING_STOP_MARKET order  	"priceRate": "0.3",					// callback rate, only return with TRAILING_STOP_MARKET order 	"updateTime": 1566818724722, 	"workingType": "CONTRACT_PRICE", 	"priceProtect": false,              // if conditional order trigger is protected	"priceMatch": "NONE",               //price match mode 	"selfTradePreventionMode": "NONE",  //self trading preventation mode}
```

## PLACE MULTIPLE ORDERS

Place Multiple Orders(TRADE)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Place-Multiple-Orders#api-description)
----------

Place multiple orders

* Parameter rules are same with `New Order`
* Batch orders are processed concurrently, and the order of matching is not guaranteed.
* The order of returned contents for batch orders is the same as the order of the order list.

HTTP请求[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Place-Multiple-Orders#http请求)
----------

POST `/dapi/v1/batchOrders`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Place-Multiple-Orders#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Place-Multiple-Orders#request-parameters)
----------

|   Name    |    Type    |Mandatory|      Description       |
|-----------|------------|---------|------------------------|
|batchOrders|LIST\<JSON\>|   YES   |order list. Max 5 orders|
|recvWindow |    LONG    |   NO    |                        |
| timestamp |    LONG    |   YES   |                        |

**Where `batchOrders` is the list of order parameters in JSON**

* **Example:** /dapi/v1/batchOrders?batchOrders=[{"type":"LIMIT","timeInForce":"GTC",  
  "symbol":"BTCUSD\_PERP","side":"BUY","price":"10001","quantity":"1"}]

|         Name          | Type  |Mandatory|                                                                                                  Description                                                                                                  |
|-----------------------|-------|---------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|        symbol         |STRING |   YES   |                                                                                                                                                                                                               |
|         side          | ENUM  |   YES   |                                                                                                                                                                                                               |
|     positionSide      | ENUM  |   NO    |                                                     Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent with Hedge Mode.                                                      |
|         type          | ENUM  |   YES   |                                                                                                                                                                                                               |
|      timeInForce      | ENUM  |   NO    |                                                                                                                                                                                                               |
|       quantity        |DECIMAL|   YES   |                                                                                                                                                                                                               |
|      reduceOnly       |STRING |   NO    |                                                                                      "true" or "false". default "false".                                                                                      |
|         price         |DECIMAL|   NO    |                                                                                                                                                                                                               |
|   newClientOrderId    |STRING |   NO    |                                    A unique id among open orders. Automatically generated if not sent. Can only be string following the rule: `^[\.A-Z\:/a-z0-9_-]{1,36}$`                                    |
|       stopPrice       |DECIMAL|   NO    |                                                                   Used with `STOP/STOP_MARKET` or `TAKE_PROFIT/TAKE_PROFIT_MARKET` orders.                                                                    |
|    activationPrice    |DECIMAL|   NO    |                                                   Used with `TRAILING_STOP_MARKET` orders, default as the latest price(supporting different `workingType`)                                                    |
|     callbackRate      |DECIMAL|   NO    |                                                                    Used with `TRAILING_STOP_MARKET` orders, min 0.1, max 4 where 1 for 1%                                                                     |
|      workingType      | ENUM  |   NO    |                                                              stopPrice triggered by: "MARK\_PRICE", "CONTRACT\_PRICE". Default "CONTRACT\_PRICE"                                                              |
|     priceProtect      |STRING |   NO    |                                                 "TRUE" or "FALSE", default "FALSE". Used with `STOP/STOP_MARKET` or `TAKE_PROFIT/TAKE_PROFIT_MARKET` orders.                                                  |
|   newOrderRespType    | ENUM  |   NO    |                                                                                        "ACK", "RESULT", default "ACK"                                                                                         |
|      priceMatch       | ENUM  |   NO    |only avaliable for `LIMIT`/`STOP`/`TAKE_PROFIT` order; can be set to `OPPONENT`/ `OPPONENT_5`/ `OPPONENT_10`/ `OPPONENT_20`: /`QUEUE`/ `QUEUE_5`/ `QUEUE_10`/ `QUEUE_20`; Can't be passed together with `price`|
|selfTradePreventionMode| ENUM  |   NO    |         `NONE`:No STP / `EXPIRE_TAKER`:expire taker order when STP triggers/ `EXPIRE_MAKER`:expire taker order when STP triggers/ `EXPIRE_BOTH`:expire both orders when STP triggers; default `NONE`          |

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Place-Multiple-Orders#response-example)
----------

```
[	{	 	"clientOrderId": "testOrder",	 	"cumQty": "0",	 	"cumBase": "0",	 	"executedQty": "0",	 	"orderId": 22542179,	 	"avgPrice": "0.0",	 	"origQty": "10",	 	"price": "0",	  	"reduceOnly": false,	  	"side": "BUY",	  	"positionSide": "SHORT",	  	"status": "NEW",	  	"stopPrice": "9300",		     // please ignore when order type is TRAILING_STOP_MARKET	  	"symbol": "BTCUSD_200925",	  	"pair": "BTCUSD",	  	"timeInForce": "GTC",	  	"type": "TRAILING_STOP_MARKET",	  	"origType": "TRAILING_STOP_MARKET",	  	"activatePrice": "9020",	     // activation price, only return with TRAILING_STOP_MARKET order	  	"priceRate": "0.3",			     // callback rate, only return with TRAILING_STOP_MARKET order	 	"updateTime": 1566818724722,	 	"workingType": "CONTRACT_PRICE",	 	"priceProtect": false,           // if conditional order trigger is protected	 	"priceMatch": "NONE",              //price match mode	 	"selfTradePreventionMode": "NONE"  //self trading preventation mode	},	{		"code": -2022, 		"msg": "ReduceOnly Order is rejected."	}]
```

## MODIFY ORDER

Modify Order (TRADE)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Order#api-description)
----------

Order modify function, currently only LIMIT order modification is supported, modified orders will be reordered in the match queue

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Order#http-request)
----------

PUT `/dapi/v1/order`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Order#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Order#request-parameters)
----------

|      Name       | Type  |Mandatory|                                                                                                  Description                                                                                                  |
|-----------------|-------|---------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     orderId     | LONG  |   NO    |                                                                                                                                                                                                               |
|origClientOrderId|STRING |   NO    |                                                                                                                                                                                                               |
|     symbol      |STRING |   YES   |                                                                                                                                                                                                               |
|      side       | ENUM  |   YES   |                                                                                                 `SELL`, `BUY`                                                                                                 |
|    quantity     |DECIMAL|   NO    |                                                                           Order quantity, cannot be sent with `closePosition=true`                                                                            |
|      price      |DECIMAL|   NO    |                                                                                                                                                                                                               |
|   priceMatch    | ENUM  |   NO    |only avaliable for `LIMIT`/`STOP`/`TAKE_PROFIT` order; can be set to `OPPONENT`/ `OPPONENT_5`/ `OPPONENT_10`/ `OPPONENT_20`: /`QUEUE`/ `QUEUE_5`/ `QUEUE_10`/ `QUEUE_20`; Can't be passed together with `price`|
|   recvWindow    | LONG  |   NO    |                                                                                                                                                                                                               |
|    timestamp    | LONG  |   YES   |                                                                                                                                                                                                               |

>
>
> * Either `orderId` or `origClientOrderId` must be sent, and the `orderId` will prevail if both are sent.
> * Either `quantity` or `price` must be sent.
> * When the new `quantity` or `price` doesn't satisfy PRICE\_FILTER / PERCENT\_FILTER / LOT\_SIZE, amendment will be rejected and the order will stay as it is.
> * However the order will be cancelled by the amendment in the following situations:
>   * when the order is in partially filled status and the new `quantity` \<= `executedQty`
>   * When the order is `GTX` and the new price will cause it to be executed immediately
>   
>   
> * One order can only be modfied for less than 10000 times
> * Modify order will set `selfTradePreventionMode` to `NONE`
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Order#response-example)
----------

```
{ 	"orderId": 20072994037, 	"symbol": "BTCUSD_PERP", 	"pair": "BTCUSD", 	"status": "NEW", 	"clientOrderId": "LJ9R4QZDihCaS8UAOOLpgW", 	"price": "30005", 	"avgPrice": "0.0", 	"origQty": "1", 	"executedQty": "0", 	"cumQty": "0", 	"cumBase": "0", 	"timeInForce": "GTC", 	"type": "LIMIT", 	"reduceOnly": false, 	"closePosition": false, 	"side": "BUY", 	"positionSide": "LONG", 	"stopPrice": "0", 	"workingType": "CONTRACT_PRICE", 	"priceProtect": false, 	"origType": "LIMIT", 	"priceMatch": "NONE",               //price match mode 	"selfTradePreventionMode": "NONE",  //self trading preventation mode 	"updateTime": 1629182711600}
```

## MODIFY MULTIPLE ORDERS

Modify Multiple Orders(TRADE)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Multiple-Orders#api-description)
----------

Modify Multiple Orders

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Multiple-Orders#http-request)
----------

PUT `/dapi/v1/batchOrders`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Multiple-Orders#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Multiple-Orders#request-parameters)
----------

|   Name    |    Type    |Mandatory|      Description       |
|-----------|------------|---------|------------------------|
|batchOrders|list\<JSON\>|   YES   |order list. Max 5 orders|
|recvWindow |    LONG    |   NO    |                        |
| timestamp |    LONG    |   YES   |                        |

**Where `batchOrders` is the list of order parameters in JSON**

|      Name       | Type  |Mandatory|                      Description                       |
|-----------------|-------|---------|--------------------------------------------------------|
|     orderId     | LONG  |   NO    |                                                        |
|origClientOrderId|STRING |   NO    |                                                        |
|     symbol      |STRING |   YES   |                                                        |
|      side       | ENUM  |   YES   |                     `SELL`, `BUY`                      |
|    quantity     |DECIMAL|   NO    |Order quantity, cannot be sent with `closePosition=true`|
|      price      |DECIMAL|   NO    |                                                        |
|   recvWindow    | LONG  |   NO    |                                                        |
|    timestamp    | LONG  |   YES   |                                                        |

>
>
> * Parameter rules are same with `Modify Order`
> * Batch modify orders are processed concurrently, and the order of matching is not guaranteed.
> * The order of returned contents for batch modify orders is the same as the order of the order list.
> * One order can only be modfied for less than 10000 times
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Multiple-Orders#response-example)
----------

```
[	{		"orderId": 20072994037,		"symbol": "BTCUSD_PERP",		"pair": "BTCUSD",		"status": "NEW",		"clientOrderId": "LJ9R4QZDihCaS8UAOOLpgW",		"price": "30005",		"avgPrice": "0.0",		"origQty": "1",		"executedQty": "0",		"cumQty": "0",		"cumBase": "0",		"timeInForce": "GTC",		"type": "LIMIT",		"reduceOnly": false,		"closePosition": false,		"side": "BUY",		"positionSide": "LONG",		"stopPrice": "0",		"workingType": "CONTRACT_PRICE",		"priceProtect": false,		"origType": "LIMIT",		"priceMatch": "NONE",               //price match mode		"selfTradePreventionMode": "NONE",  //self trading preventation mode		"updateTime": 1629182711600	},	{		"code": -2022, 		"msg": "ReduceOnly Order is rejected."	}]
```

## GET ORDER MODIFY HISTORY

Get Order Modify History (USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Order-Modify-History#api-description)
----------

Get order modification history

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Order-Modify-History#http-request)
----------

GET `/dapi/v1/orderAmendment`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Order-Modify-History#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Order-Modify-History#request-parameters)
----------

|      Name       | Type |Mandatory|                        Description                        |
|-----------------|------|---------|-----------------------------------------------------------|
|     symbol      |STRING|   YES   |                                                           |
|     orderId     | LONG |   NO    |                                                           |
|origClientOrderId|STRING|   NO    |                                                           |
|    startTime    | LONG |   NO    |Timestamp in ms to get modification history from INCLUSIVE |
|     endTime     | LONG |   NO    |Timestamp in ms to get modification history until INCLUSIVE|
|      limit      | INT  |   NO    |                    Default 50; max 100                    |
|   recvWindow    | LONG |   NO    |                                                           |
|    timestamp    | LONG |   YES   |                                                           |

>
>
> * Either `orderId` or `origClientOrderId` must be sent, and the `orderId` will prevail if both are sent.
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Order-Modify-History#response-example)
----------

```
[    {        "amendmentId": 5363,	// Order modification ID        "symbol": "BTCUSD_PERP",        "pair": "BTCUSD",        "orderId": 20072994037,        "clientOrderId": "LJ9R4QZDihCaS8UAOOLpgW",        "time": 1629184560899,	// Order modification time        "amendment": {            "price": {                "before": "30004",                "after": "30003.2"            },            "origQty": {                "before": "1",                "after": "1"            },            "count": 3	// Order modification count, representing the number of times the order has been modified        }    },    {        "amendmentId": 5361,        "symbol": "BTCUSD_PERP",        "pair": "BTCUSD",        "orderId": 20072994037,        "clientOrderId": "LJ9R4QZDihCaS8UAOOLpgW",        "time": 1629184533946,        "amendment": {            "price": {                "before": "30005",                "after": "30004"            },            "origQty": {                "before": "1",                "after": "1"            },            "count": 2        }    },    {        "amendmentId": 5325,        "symbol": "BTCUSD_PERP",        "pair": "BTCUSD",        "orderId": 20072994037,        "clientOrderId": "LJ9R4QZDihCaS8UAOOLpgW",        "time": 1629182711787,        "amendment": {            "price": {                "before": "30002",                "after": "30005"            },            "origQty": {                "before": "1",                "after": "1"            },            "count": 1        }    }]
```

## CANCEL ORDER

Cancel Order (TRADE)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-Order#api-description)
----------

Cancel an active order.

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-Order#http-request)
----------

DELETE `/dapi/v1/order`

**Weight:****1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-Order#request-parameters)
----------

|      Name       | Type |Mandatory|Description|
|-----------------|------|---------|-----------|
|     symbol      |STRING|   YES   |           |
|     orderId     | LONG |   NO    |           |
|origClientOrderId|STRING|   NO    |           |
|   recvWindow    | LONG |   NO    |           |
|    timestamp    | LONG |   YES   |           |

>
>
> * Either `orderId` or `origClientOrderId` must be sent.
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-Order#response-example)
----------

```
{ 	"avgPrice": "0.0", 	"clientOrderId": "myOrder1", 	"cumQty": "0", 	"cumBase": "0", 	"executedQty": "0", 	"orderId": 283194212, 	"origQty": "11", 	"origType": "TRAILING_STOP_MARKET",  	"price": "0",  	"reduceOnly": false,  	"side": "BUY",  	"positionSide": "SHORT", 			  	"status": "CANCELED",  	"stopPrice": "9300",				// please ignore when order type is TRAILING_STOP_MARKET  	"closePosition": false,   			// if Close-All  	"symbol": "BTCUSD_200925",  	"pair": "BTCUSD",  	"timeInForce": "GTC",  	"type": "TRAILING_STOP_MARKET",  	"activatePrice": "9020",			// activation price, only return with TRAILING_STOP_MARKET order  	"priceRate": "0.3",					// callback rate, only return with TRAILING_STOP_MARKET order 	"updateTime": 1571110484038, 	"workingType": "CONTRACT_PRICE", 	"priceProtect": false,              // if conditional order trigger is protected 	"priceMatch": "NONE",               //price match mode 	"selfTradePreventionMode": "NONE"   //self trading preventation mode}
```

## CANCEL MULTIPLE ORDERS

Cancel Multiple Orders(TRADE)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-Multiple-Orders#api-description)
----------

Cancel Multiple Orders

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-Multiple-Orders#http-request)
----------

DELETE `/dapi/v1/batchOrders`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-Multiple-Orders#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-Multiple-Orders#request-parameters)
----------

|        Name         |     Type     |Mandatory|                                            Description                                             |
|---------------------|--------------|---------|----------------------------------------------------------------------------------------------------|
|       symbol        |    STRING    |   YES   |                                                                                                    |
|     orderIdList     | LIST\<LONG\> |   NO    |                            max length 10   <br/> e.g. [1234567,2345678]                            |
|origClientOrderIdList|LIST\<STRING\>|   NO    |max length 10  <br/> e.g. ["my\_id\_1","my\_id\_2"], encode the double quotes. No space after comma.|
|     recvWindow      |     LONG     |   NO    |                                                                                                    |
|      timestamp      |     LONG     |   YES   |                                                                                                    |

>
>
> * Either `orderIdList` or `origClientOrderIdList ` must be sent.
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-Multiple-Orders#response-example)
----------

```
[	{	 	"avgPrice": "0.0",	 	"clientOrderId": "myOrder1",	 	"cumQty": "0",	 	"cumBase": "0",	 	"executedQty": "0",	 	"orderId": 283194212,	 	"origQty": "11",	 	"origType": "TRAILING_STOP_MARKET",  		"price": "0",  		"reduceOnly": false,  		"side": "BUY",  		"positionSide": "SHORT",  		"status": "CANCELED",  		"stopPrice": "9300",				// please ignore when order type is TRAILING_STOP_MARKET  		"closePosition": false,   			// if Close-All  		"symbol": "BTCUSD_200925",  		"timeInForce": "GTC",  		"type": "TRAILING_STOP_MARKET",  		"activatePrice": "9020",			// activation price, only return with TRAILING_STOP_MARKET order  		"priceRate": "0.3",					// callback rate, only return with TRAILING_STOP_MARKET order  		"workingType": "CONTRACT_PRICE", 		"priceProtect": false,              // if conditional order trigger is protected 		"priceMatch": "NONE",               //price match mode 		"selfTradePreventionMode": "NONE",  //self trading preventation mode	 	"updateTime": 1571110484038	},	{		"code": -2011,		"msg": "Unknown order sent."	}]
```

## CANCEL ALL OPEN ORDERS

Cancel All Open Orders(TRADE)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-All-Open-Orders#api-description)
----------

Cancel All Open Orders

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-All-Open-Orders#http-request)
----------

DELETE `/dapi/v1/allOpenOrders`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-All-Open-Orders#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-All-Open-Orders#request-parameters)
----------

|   Name   | Type |Mandatory|Description|
|----------|------|---------|-----------|
|  symbol  |STRING|   YES   |           |
|recvWindow| LONG |   NO    |           |
|timestamp | LONG |   YES   |           |

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-All-Open-Orders#response-example)
----------

```
{	"code": 200, 	"msg": "The operation of cancel all open order is done."}
```

## AUTO CANCEL ALL OPEN ORDERS

Auto-Cancel All Open Orders (TRADE)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Auto-Cancel-All-Open-Orders#api-description)
----------

Cancel all open orders of the specified symbol at the end of the specified countdown. This rest endpoint means to ensure your open orders are canceled in case of an outage. The endpoint should be called repeatedly as heartbeats so that the existing countdown time can be canceled and repalced by a new one. The system will check all countdowns **approximately every 10 milliseconds**, so please note that sufficient redundancy should be considered when using this function. We do not recommend setting the countdown time to be too precise or too small.

* Example usage:

>
>
> Call this endpoint at 30s intervals with an countdownTime of 120000 (120s).  
> If this endpoint is not called within 120 seconds, all your orders of the specified symbol will be automatically canceled.  
> If this endpoint is called with an countdownTime of 0, the countdown timer will be stopped.
>
>

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Auto-Cancel-All-Open-Orders#http-request)
----------

POST `/dapi/v1/countdownCancelAll`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Auto-Cancel-All-Open-Orders#request-weight)
----------

**10**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Auto-Cancel-All-Open-Orders#request-parameters)
----------

|    Name     | Type |Mandatory|                      Description                       |
|-------------|------|---------|--------------------------------------------------------|
|   symbol    |STRING|   YES   |                                                        |
|countdownTime| LONG |   YES   |countdown time, 1000 for 1 second. 0 to cancel the timer|
| recvWindow  | LONG |   NO    |                                                        |
|  timestamp  | LONG |   YES   |                                                        |

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Auto-Cancel-All-Open-Orders#http-request-1)
----------

```
{	"symbol": "BTCUSD_200925", 	"countdownTime": "100000"}
```

## QUERY ORDER

Query Order (USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Order#api-description)
----------

Check an order's status.

* These orders will not be found:
  * order status is CANCELED or EXPIRED AND order has NO filled trade AND created time + 3 days \< current time
  * order create time + 90 days \< current time

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Order#http-request)
----------

GET `/dapi/v1/order`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Order#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Order#request-parameters)
----------

|      Name       | Type |Mandatory|Description|
|-----------------|------|---------|-----------|
|     symbol      |STRING|   YES   |           |
|     orderId     | LONG |   NO    |           |
|origClientOrderId|STRING|   NO    |           |
|   recvWindow    | LONG |   NO    |           |
|    timestamp    | LONG |   YES   |           |

>
>
> * Either `orderId` or `origClientOrderId` must be sent.
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Order#response-example)
----------

```
{  	"avgPrice": "0.0",  	"clientOrderId": "abc",  	"cumBase": "0",  	"executedQty": "0",  	"orderId": 1917641,  	"origQty": "0.40",  	"origType": "TRAILING_STOP_MARKET",  	"price": "0",  	"reduceOnly": false,  	"side": "BUY",  	"status": "NEW",  	"stopPrice": "9300",				// please ignore when order type is TRAILING_STOP_MARKET  	"closePosition": false,   			// if Close-All  	"symbol": "BTCUSD_200925",  	"pair": "BTCUSD",  	"time": 1579276756075,				// order time  	"timeInForce": "GTC",  	"type": "TRAILING_STOP_MARKET",  	"activatePrice": "9020",			// activation price, only return with TRAILING_STOP_MARKET order  	"priceRate": "0.3",					// callback rate, only return with TRAILING_STOP_MARKET order  	"updateTime": 1579276756075,		// update time  	"workingType": "CONTRACT_PRICE",  	"priceProtect": false,              // if conditional order trigger is protected  	"priceMatch": "NONE",               //price match mode  	"selfTradePreventionMode": "NONE"   //self trading preventation mode}
```

## ALL ORDERS

All Orders (USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/All-Orders#api-description)
----------

Get all account orders; active, canceled, or filled.

* These orders will not be found:
  * order status is CANCELED or EXPIRED AND order has NO filled trade AND created time + 3 days \< current time
  * order create time + 90 days \< current time

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/All-Orders#http-request)
----------

GET `/dapi/v1/allOrders`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/All-Orders#request-weight)
----------

**20** with symbol, **40** with pair

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/All-Orders#request-parameters)
----------

|   Name   | Type |Mandatory|    Description     |
|----------|------|---------|--------------------|
|  symbol  |STRING|   NO    |                    |
|   pair   |STRING|   NO    |                    |
| orderId  | LONG |   NO    |                    |
|startTime | LONG |   NO    |                    |
| endTime  | LONG |   NO    |                    |
|  limit   | INT  |   NO    |Default 50; max 100.|
|recvWindow| LONG |   NO    |                    |
|timestamp | LONG |   YES   |                    |

**Notes:**

>
>
> * Either `symbol` or `pair` must be sent.
> * `pair` can't be sent with `orderId`
> * If `orderId` is set, it will get orders \>= that `orderId`. Otherwise most recent orders are returned.
> * If orderId is set, it will get orders \>= that orderId. Otherwise most recent orders are returned.
> * The query time period must be less then 7 days( default as the recent 7 days).
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/All-Orders#response-example)
----------

```
[  {   	"avgPrice": "0.0",  	"clientOrderId": "abc",  	"cumBase": "0",  	"executedQty": "0",  	"orderId": 1917641,  	"origQty": "0.40",  	"origType": "TRAILING_STOP_MARKET",  	"price": "0",  	"reduceOnly": false,  	"side": "BUY",  	"positionSide": "SHORT",  	"status": "NEW",  	"stopPrice": "9300",				// please ignore when order type is TRAILING_STOP_MARKET  	"closePosition": false,   			// if Close-All  	"symbol": "BTCUSD_200925",  	"pair": "BTCUSD",  	"time": 1579276756075,				// order time  	"timeInForce": "GTC",  	"type": "TRAILING_STOP_MARKET",  	"activatePrice": "9020",			// activation price, only return with TRAILING_STOP_MARKET order  	"priceRate": "0.3",					// callback rate, only return with TRAILING_STOP_MARKET order  	"updateTime": 1579276756075,		// update time  	"workingType": "CONTRACT_PRICE",  	"priceProtect": false,              // if conditional order trigger is protected  	"priceMatch": "NONE",               //price match mode  	"selfTradePreventionMode": "NONE",  //self trading preventation mode  }]
```

## CURRENT ALL OPEN ORDERS

Current All Open Orders (USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Current-All-Open-Orders#api-description)
----------

Get all open orders on a symbol. **Careful** when accessing this with no symbol.

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Current-All-Open-Orders#http-request)
----------

GET `/dapi/v1/openOrders`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Current-All-Open-Orders#request-weight)
----------

**1** for a single symbol, **40** for mutltiple symbols

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Current-All-Open-Orders#request-parameters)
----------

|   Name   | Type |Mandatory|Description|
|----------|------|---------|-----------|
|  symbol  |STRING|   NO    |           |
|   pair   |STRING|   NO    |           |
|recvWindow| LONG |   NO    |           |
|timestamp | LONG |   YES   |           |

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Current-All-Open-Orders#response-example)
----------

```
[  {  	"avgPrice": "0.0",  	"clientOrderId": "abc",  	"cumBase": "0",  	"executedQty": "0",  	"orderId": 1917641,  	"origQty": "0.40",  	"origType": "TRAILING_STOP_MARKET",  	"price": "0",  	"reduceOnly": false,  	"side": "BUY",  	"positionSide": "SHORT",  	"status": "NEW",  	"stopPrice": "9300",				// please ignore when order type is TRAILING_STOP_MARKET  	"closePosition": false,   			// if Close-All  	"symbol": "BTCUSD_200925",  	"time": 1579276756075,				// order time  	"timeInForce": "GTC",  	"type": "TRAILING_STOP_MARKET",  	"activatePrice": "9020",			// activation price, only return with TRAILING_STOP_MARKET order  	"priceRate": "0.3",					// callback rate, only return with TRAILING_STOP_MARKET order  	"updateTime": 1579276756075,		// update time  	"workingType": "CONTRACT_PRICE",  	"priceProtect": false,              // if conditional order trigger is protected  	"priceMatch": "NONE",               //price match mode  	"selfTradePreventionMode": "NONE"   //self trading preventation mode  }]
```

## QUERY CURRENT OPEN ORDER

Query Current Open Order(USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Current-Open-Order#api-description)
----------

Query Current Open Order

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Current-Open-Order#http-request)
----------

GET `/dapi/v1/openOrder`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Current-Open-Order#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Current-Open-Order#request-parameters)
----------

|      Name       | Type |Mandatory|Description|
|-----------------|------|---------|-----------|
|     symbol      |STRING|   YES   |           |
|     orderId     | LONG |   NO    |           |
|origClientOrderId|STRING|   NO    |           |
|   recvWindow    | LONG |   NO    |           |
|    timestamp    | LONG |   YES   |           |

>
>
> * Either`orderId` or `origClientOrderId` must be sent
> * If the queried order has been filled or cancelled, the error message "Order does not exist" will be returned.
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Current-Open-Order#response-example)
----------

```
{  	"avgPrice": "0.0",				  	"clientOrderId": "abc",				  	"cumBase": "0",						  	"executedQty": "0",					  	"orderId": 1917641,					  	"origQty": "0.40",						  	"origType": "TRAILING_STOP_MARKET",  	"price": "0",  	"reduceOnly": false,  	"side": "BUY",  	"positionSide": "SHORT",  	"status": "NEW",  	"stopPrice": "9300",				// please ignore when order type is TRAILING_STOP_MARKET  	"closePosition": false,   			// if Close-All  	"symbol": "BTCUSD_200925",  	"pair": "BTCUSD"  	"time": 1579276756075,				// order time  	"timeInForce": "GTC",  	"type": "TRAILING_STOP_MARKET",  	"activatePrice": "9020",			// activation price, only return with TRAILING_STOP_MARKET order  	"priceRate": "0.3",					// callback rate, only return with TRAILING_STOP_MARKET order						  	"updateTime": 1579276756075,		  	"workingType": "CONTRACT_PRICE",  	"priceProtect": false               // if conditional order trigger is protected	  	"priceMatch": "NONE",               // price match mode  	"selfTradePreventionMode": "NONE"   // self trading preventation mode	}
```

## USERS FORCE ORDERS

User's Force Orders(USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Users-Force-Orders#api-description)
----------

User's Force Orders

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Users-Force-Orders#http-request)
----------

GET `/dapi/v1/forceOrders`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Users-Force-Orders#request-weight)
----------

**20** with symbol, **50** without symbol

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Users-Force-Orders#request-parameters)
----------

```
Name      |  Type  | Mandatory |                         Description
```

\------------- | ------ | --------- | -----------------------------------------------------------
symbol | STRING | NO |
autoCloseType | ENUM | NO | "LIQUIDATION" for liquidation orders, "ADL" for ADL orders.
startTime | LONG | NO |
endTime | LONG | NO |
limit | INT | NO | Default 50; max 100.
recvWindow | LONG | NO |
timestamp | LONG | YES |

>
>
> * If "autoCloseType" is not sent, orders with both of the types will be returned
> * If "startTime" is not sent, data within 200 days before "endTime" can be queried
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Users-Force-Orders#response-example)
----------

```
[  { 	"orderId": 165123080,  	"symbol": "BTCUSD_200925",  	"pair": "BTCUSD",  	"status": "FILLED",  	"clientOrderId": "autoclose-1596542005017000006",  	"price": "11326.9",  	"avgPrice": "11326.9",  	"origQty": "1",  	"executedQty": "1",  	"cumBase": "0.00882854",  	"timeInForce": "IOC",  	"type": "LIMIT",  	"reduceOnly": false,  	"closePosition": false,  	"side": "SELL",  	"positionSide": "BOTH",  	"stopPrice": "0",  	"workingType": "CONTRACT_PRICE",  	"priceProtect": false,  	"origType": "LIMIT",  	"time": 1596542005019,  	"updateTime": 1596542005050  },  {  	"orderId": 207251986,  	"symbol": "BTCUSD_200925",  	"pair": "BTCUSD",  	"status": "FILLED",  	"clientOrderId": "autoclose-1597307316020000006",  	"price": "11619.4",  	"avgPrice": "11661.2",  	"origQty": "1",  	"executedQty": "1",  	"cumBase": "0.00857544",  	"timeInForce": "IOC",  	"type": "LIMIT",  	"reduceOnly": false,  	"closePosition": false,  	"side": "SELL",  	"positionSide": "LONG",  	"stopPrice": "0",  	"workingType": "CONTRACT_PRICE",  	"priceProtect": false,  	"origType": "LIMIT",  	"time": 1597307316022,  	"updateTime": 1597307316035  }]
```

## ACCOUNT TRADE LIST

Account Trade List (USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Account-Trade-List#api-description)
----------

Get trades for a specific account and symbol.

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Account-Trade-List#http-request)
----------

GET `/dapi/v1/userTrades`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Account-Trade-List#request-weight)
----------

**20** with symbol，**40** with pair

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Account-Trade-List#request-parameters)
----------

|   Name   | Type |Mandatory|                      Description                       |
|----------|------|---------|--------------------------------------------------------|
|  symbol  |STRING|   NO    |                                                        |
|   pair   |STRING|   NO    |                                                        |
| orderId  |STRING|   NO    |                                                        |
|startTime | LONG |   NO    |                                                        |
| endTime  | LONG |   NO    |                                                        |
|  fromId  | LONG |   NO    |Trade id to fetch from. Default gets most recent trades.|
|  limit   | INT  |   NO    |                  Default 50; max 1000                  |
|recvWindow| LONG |   NO    |                                                        |
|timestamp | LONG |   YES   |                                                        |

>
>
> * Either symbol or pair must be sent
> * Symbol and pair cannot be sent together
> * Pair and fromId cannot be sent together
> * OrderId can only be sent together with symbol
> * If a pair is sent,tickers for all symbols of the pair will be returned
> * The parameter `fromId` cannot be sent with `startTime` or `endTime`
> * If startTime and endTime are both not sent, then the last 7 days' data will be returned.
> * The time between startTime and endTime cannot be longer than 7 days.
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Account-Trade-List#response-example)
----------

```
[	{		'symbol': 'BTCUSD_200626',	  	'id': 6,	  	'orderId': 28,	  	'pair': 'BTCUSD',	  	'side': 'SELL',	  	'price': '8800',	  	'qty': '1',	  	'realizedPnl': '0',	  	'marginAsset': 'BTC',	  	'baseQty': '0.01136364',	  	'commission': '0.00000454',	  	'commissionAsset': 'BTC',	  	'time': 1590743483586,	  	'positionSide': 'BOTH',	  	'buyer': false,	  	'maker': false	}]
```

## POSITION INFORMATION

Position Information(USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Position-Information#api-description)
----------

Get current account information.

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Position-Information#http-request)
----------

GET `/dapi/v1/positionRisk`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Position-Information#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Position-Information#request-parameters)
----------

|   Name    | Type |Mandatory|Description|
|-----------|------|---------|-----------|
|marginAsset|STRING|   NO    |           |
|   pair    |STRING|   NO    |           |
|recvWindow | LONG |   NO    |           |
| timestamp | LONG |   YES   |           |

>
>
> * If neither `marginAsset` nor `pair` is sent, positions of all symbols with `TRADING` status will be returned.
> * for One-way Mode user, the response will only show the "BOTH" positions
> * for Hedge Mode user, the response will show "BOTH", "LONG", and "SHORT" positions.
>
>

**Note**

>
>
> Please use with user data stream `ACCOUNT_UPDATE` to meet your timeliness and accuracy needs.
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Position-Information#response-example)
----------

```
[    {        "symbol": "BTCUSD_201225",        "positionAmt": "0",        "entryPrice": "0.0",        "breakEvenPrice": "0.0",  // break-even price        "markPrice": "0.00000000",        "unRealizedProfit": "0.00000000",        "liquidationPrice": "0",        "leverage": "125",        "maxQty": "50",  // maximum quantity of base asset        "marginType": "cross",        "isolatedMargin": "0.00000000",        "isAutoAddMargin": "false",        "positionSide": "BOTH",        "updateTime": 0    },    {        "symbol": "BTCUSD_201225",        "positionAmt": "1",        "entryPrice": "11707.70000003",        "breakEvenPrice": "11707.80000005",  // break-even price        "markPrice": "11788.66626667",        "unRealizedProfit": "0.00005866",        "liquidationPrice": "11667.63509587",        "leverage": "125",        "maxQty": "50",        "marginType": "cross",        "isolatedMargin": "0.00000000",        "isAutoAddMargin": "false",        "positionSide": "LONG",        "updateTime": 1627026881327     },    {        "symbol": "BTCUSD_201225",        "positionAmt": "0",        "entryPrice": "0.0",        "breakEvenPrice": "0.0",  // break-even price        "markPrice": "0.00000000",        "unRealizedProfit": "0.00000000",        "liquidationPrice": "0",        "leverage": "125",        "maxQty": "50",        "marginType": "cross",        "isolatedMargin": "0.00000000",        "isAutoAddMargin": "false",        "positionSide": "SHORT",        "updateTime":1627026881327  }]
```

## CHANGE POSITION MODE

Change Position Mode(TRADE)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Position-Mode#api-description)
----------

Change user's position mode (Hedge Mode or One-way Mode ) on ***EVERY symbol***

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Position-Mode#http-request)
----------

POST `/dapi/v1/positionSide/dual`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Position-Mode#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Position-Mode#request-parameters)
----------

|      Name      | Type |Mandatory|               Description               |
|----------------|------|---------|-----------------------------------------|
|dualSidePosition|STRING|   YES   |"true": Hedge Mode; "false": One-way Mode|
|   recvWindow   | LONG |   NO    |                                         |
|   timestamp    | LONG |   YES   |                                         |

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Position-Mode#response-example)
----------

```
{	"code": 200,	"msg": "success"}
```

## CHANGE MARGIN TYPE

Change Margin Type (TRADE)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Margin-Type#api-description)
----------

Change user's margin type in the specific symbol market.For Hedge Mode, LONG and SHORT positions of one symbol use the same margin type.  
With ISOLATED margin type, margins of the LONG and SHORT positions are isolated from each other.

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Margin-Type#http-request)
----------

POST `/dapi/v1/marginType`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Margin-Type#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Margin-Type#request-parameters)
----------

|   Name   | Type |Mandatory|   Description   |
|----------|------|---------|-----------------|
|  symbol  |STRING|   YES   |                 |
|marginType| ENUM |   YES   |ISOLATED, CROSSED|
|recvWindow| LONG |   NO    |                 |
|timestamp | LONG |   YES   |                 |

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Margin-Type#response-example)
----------

```
{	"code": 200,	"msg": "success"}
```

## CHANGE INITIAL LEVERAGE

Change Initial Leverage (TRADE)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Initial-Leverage#api-description)
----------

Change user's initial leverage in the specific symbol market.  
For Hedge Mode, LONG and SHORT positions of one symbol use the same initial leverage and share a total notional value.

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Initial-Leverage#http-request)
----------

POST `/dapi/v1/leverage`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Initial-Leverage#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Initial-Leverage#request-parameters)
----------

|   Name   | Type |Mandatory|               Description                |
|----------|------|---------|------------------------------------------|
|  symbol  |STRING|   YES   |                                          |
| leverage | INT  |   YES   |target initial leverage: int from 1 to 125|
|recvWindow| LONG |   NO    |                                          |
|timestamp | LONG |   YES   |                                          |

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Initial-Leverage#response-example)
----------

```
{ 	"leverage": 21, 	"maxQty": "1000",  // maximum quantity of base asset 	"symbol": "BTCUSD_200925"}
```

## POSITION ADL QUANTILE ESTIMATION

Position ADL Quantile Estimation(USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation#api-description)
----------

Query position ADL quantile estimation

>
>
> * Values update every 30s.
> * Values 0, 1, 2, 3, 4 shows the queue position and possibility of ADL from low to high.
> * For positions of the symbol are in One-way Mode or isolated margined in Hedge Mode, "LONG", "SHORT", and "BOTH" will be returned to show the positions' adl quantiles of different position sides.
> * If the positions of the symbol are crossed margined in Hedge Mode:
>   * "HEDGE" as a sign will be returned instead of "BOTH";
>   * A same value caculated on unrealized pnls on long and short sides' positions will be shown for "LONG" and "SHORT" when there are positions in both of long and short sides.
>   
>   
>
>

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation#http-request)
----------

GET `/dapi/v1/adlQuantile`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation#request-parameters)
----------

|   Name   | Type |Mandatory|Description|
|----------|------|---------|-----------|
|  symbol  |STRING|   NO    |           |
|recvWindow| LONG |   NO    |           |
|timestamp | LONG |   YES   |           |

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation#response-example)
----------

```
[	{		"symbol": "BTCUSD_200925", 		"adlQuantile": 			{				// if the positions of the symbol are crossed margined in Hedge Mode, "LONG" and "SHORT" will be returned a same quantile value, and "HEDGE" will be returned instead of "BOTH".				"LONG": 3,  				"SHORT": 3, 				"HEDGE": 0   // only a sign, ignore the value			}		}, 	{ 		"symbol": "BTCUSD_201225",  		"adlQuantile":  			{ 				// for positions of the symbol are in One-way Mode or isolated margined in Hedge Mode 				"LONG": 1, 	    // adl quantile for "LONG" position in hedge mode 				"SHORT": 2, 	// adl qauntile for "SHORT" position in hedge mode 				"BOTH": 0		// adl qunatile for position in one-way mode 			} 	} ]
```

## MODIFY ISOLATED POSITION MARGIN

Modify Isolated Position Margin(TRADE)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin#api-description)
----------

Modify Isolated Position Margin

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin#http-request)
----------

POST `/dapi/v1/positionMargin`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin#request-parameters)
----------

|    Name    | Type  |Mandatory|                                            Description                                             |
|------------|-------|---------|----------------------------------------------------------------------------------------------------|
|   symbol   |STRING |   YES   |                                                                                                    |
|positionSide| ENUM  |   NO    |Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent with Hedge Mode.|
|   amount   |DECIMAL|   YES   |                                                                                                    |
|    type    |  INT  |   YES   |                          1: Add position margin,2: Reduce position margin                          |
| recvWindow | LONG  |   NO    |                                                                                                    |
| timestamp  | LONG  |   YES   |                                                                                                    |

>
>
> * Only for isolated symbol
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin#response-example)
----------

```
{	"amount": 100.0,  	"code": 200,  	"msg": "Successfully modify position margin.",  	"type": 1}
```

## GET POSITION MARGIN CHANGE HISTORY

Get Position Margin Change History(TRADE)
==========

API Description[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Position-Margin-Change-History#api-description)
----------

Get position margin change history

HTTP Request[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Position-Margin-Change-History#http-request)
----------

GET `/dapi/v1/positionMargin/history`

Request Weight[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Position-Margin-Change-History#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Position-Margin-Change-History#request-parameters)
----------

|   Name   | Type |Mandatory|                  Description                   |
|----------|------|---------|------------------------------------------------|
|  symbol  |STRING|   YES   |                                                |
|   type   | INT  |   NO    |1: Add position margin,2: Reduce position margin|
|startTime | LONG |   NO    |                                                |
| endTime  | LONG |   NO    |                                                |
|  limit   | INT  |   NO    |                  Default: 50                   |
|recvWindow| LONG |   NO    |                                                |
|timestamp | LONG |   YES   |                                                |

Response Example[​](/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Position-Margin-Change-History#response-example)
----------

```
[	{		"amount": "23.36332311",	  	"asset": "BTC",	  	"symbol": "BTCUSD_200925",	  	"time": 1578047897183,	  	"type": 1,	  	"positionSide": "BOTH"	},	{		"amount": "100",	  	"asset": "BTC",	  	"symbol": "BTCUSD_200925",	  	"time": 1578047900425,	  	"type": 1,	  	"positionSide": "LONG"	}]
```

## REST API

New Future Account Transfer
==========

Please find details from [here](https://developers.binance.com/docs/wallet/asset/user-universal-transfer).

## USER UNIVERSAL TRANSFER

User Universal Transfer (USER\_DATA)
==========

API Description[​](/docs/wallet/asset/user-universal-transfer#api-description)
----------

user universal transfer

HTTP Request[​](/docs/wallet/asset/user-universal-transfer#http-request)
----------

POST `/sapi/v1/asset/transfer`

You need to enable `Permits Universal Transfer` option for the API Key which requests this endpoint.

Request Weight(UID)[​](/docs/wallet/asset/user-universal-transfer#request-weightuid)
----------

**900**

Request Parameters[​](/docs/wallet/asset/user-universal-transfer#request-parameters)
----------

|   Name   | Type  |Mandatory|Description|
|----------|-------|---------|-----------|
|   type   | ENUM  |   YES   |           |
|  asset   |STRING |   YES   |           |
|  amount  |DECIMAL|   YES   |           |
|fromSymbol|STRING |   NO    |           |
| toSymbol |STRING |   NO    |           |
|recvWindow| LONG  |   NO    |           |
|timestamp | LONG  |   YES   |           |

* `fromSymbol` must be sent when type are ISOLATEDMARGIN\_MARGIN and ISOLATEDMARGIN\_ISOLATEDMARGIN

* `toSymbol` must be sent when type are MARGIN\_ISOLATEDMARGIN and ISOLATEDMARGIN\_ISOLATEDMARGIN

* ENUM of transfer types:

  * MAIN\_UMFUTURE Spot account transfer to USDⓈ-M Futures account
  * MAIN\_CMFUTURE Spot account transfer to COIN-M Futures account
  * MAIN\_MARGIN Spot account transfer to Margin（cross）account
  * UMFUTURE\_MAIN USDⓈ-M Futures account transfer to Spot account
  * UMFUTURE\_MARGIN USDⓈ-M Futures account transfer to Margin（cross）account
  * CMFUTURE\_MAIN COIN-M Futures account transfer to Spot account
  * CMFUTURE\_MARGIN COIN-M Futures account transfer to Margin(cross) account
  * MARGIN\_MAIN Margin（cross）account transfer to Spot account
  * MARGIN\_UMFUTURE Margin（cross）account transfer to USDⓈ-M Futures
  * MARGIN\_CMFUTURE Margin（cross）account transfer to COIN-M Futures
  * ISOLATEDMARGIN\_MARGIN Isolated margin account transfer to Margin(cross) account
  * MARGIN\_ISOLATEDMARGIN Margin(cross) account transfer to Isolated margin account
  * ISOLATEDMARGIN\_ISOLATEDMARGIN Isolated margin account transfer to Isolated margin account
  * MAIN\_FUNDING Spot account transfer to Funding account
  * FUNDING\_MAIN Funding account transfer to Spot account
  * FUNDING\_UMFUTURE Funding account transfer to UMFUTURE account
  * UMFUTURE\_FUNDING UMFUTURE account transfer to Funding account
  * MARGIN\_FUNDING MARGIN account transfer to Funding account
  * FUNDING\_MARGIN Funding account transfer to Margin account
  * FUNDING\_CMFUTURE Funding account transfer to CMFUTURE account
  * CMFUTURE\_FUNDING CMFUTURE account transfer to Funding account
  * MAIN\_OPTION Spot account transfer to Options account
  * OPTION\_MAIN Options account transfer to Spot account
  * UMFUTURE\_OPTION USDⓈ-M Futures account transfer to Options account
  * OPTION\_UMFUTURE Options account transfer to USDⓈ-M Futures account
  * MARGIN\_OPTION Margin（cross）account transfer to Options account
  * OPTION\_MARGIN Options account transfer to Margin（cross）account
  * FUNDING\_OPTION Funding account transfer to Options account
  * OPTION\_FUNDING Options account transfer to Funding account
  * MAIN\_PORTFOLIO\_MARGIN Spot account transfer to Portfolio Margin account
  * PORTFOLIO\_MARGIN\_MAIN Portfolio Margin account transfer to Spot account

Response Example[​](/docs/wallet/asset/user-universal-transfer#response-example)
----------

```
{    "tranId":13526853623}
```

## FUTURES ACCOUNT BALANCE

Futures Account Balance (USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/account/rest-api/Futures-Account-Balance#api-description)
----------

Check futures account balance

HTTP Request[​](/docs/derivatives/coin-margined-futures/account/rest-api/Futures-Account-Balance#http-request)
----------

GET `/dapi/v1/balance`

Request Weight[​](/docs/derivatives/coin-margined-futures/account/rest-api/Futures-Account-Balance#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/account/rest-api/Futures-Account-Balance#request-parameters)
----------

|   Name   |Type|Mandatory|Description|
|----------|----|---------|-----------|
|recvWindow|LONG|   NO    |           |
|timestamp |LONG|   YES   |           |

Response Example[​](/docs/derivatives/coin-margined-futures/account/rest-api/Futures-Account-Balance#response-example)
----------

```
[ 	{ 		"accountAlias": "SgsR",    // unique account code 		"asset": "BTC", 		"balance": "0.00250000", 		"withdrawAvailable": "0.00250000", 		"crossWalletBalance": "0.00241969",  		"crossUnPnl": "0.00000000",  		"availableBalance": "0.00241969", 		"updateTime": 1592468353979	}]
```

## GET FUTURE ACCOUNT TRANSACTION HISTORY LIST

Get Future Account Transaction History List
==========

Please find details from [here](https://developers.binance.com/docs/wallet/asset/query-user-universal-transfer).

## USER COMMISSION RATE

User Commission Rate (USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/account/rest-api/User-Commission-Rate#api-description)
----------

Query user commission rate

HTTP Request[​](/docs/derivatives/coin-margined-futures/account/rest-api/User-Commission-Rate#http-request)
----------

GET `/dapi/v1/commissionRate`

Request Weight[​](/docs/derivatives/coin-margined-futures/account/rest-api/User-Commission-Rate#request-weight)
----------

**20**

Request Parameters[​](/docs/derivatives/coin-margined-futures/account/rest-api/User-Commission-Rate#request-parameters)
----------

|   Name   | Type |Mandatory|Description|
|----------|------|---------|-----------|
|  symbol  |STRING|   YES   |           |
|recvWindow| LONG |   NO    |           |
|timestamp | LONG |   YES   |           |

Response Example[​](/docs/derivatives/coin-margined-futures/account/rest-api/User-Commission-Rate#response-example)
----------

```
{	"symbol": "BTCUSD_PERP",  	"makerCommissionRate": "0.00015",  // 0.015%  	"takerCommissionRate": "0.00040"   // 0.040%}
```

## ACCOUNT INFORMATION

Account Information (USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/account/rest-api/Account-Information#api-description)
----------

Get current account information.

HTTP Request[​](/docs/derivatives/coin-margined-futures/account/rest-api/Account-Information#http-request)
----------

GET `/dapi/v1/account`

Request Weight[​](/docs/derivatives/coin-margined-futures/account/rest-api/Account-Information#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/coin-margined-futures/account/rest-api/Account-Information#request-parameters)
----------

|   Name   |Type|Mandatory|Description|
|----------|----|---------|-----------|
|recvWindow|LONG|   NO    |           |
|timestamp |LONG|   YES   |           |

>
>
> * for One-way Mode user, the "positions" will only show the "BOTH" positions
> * for Hedge Mode user, the "positions" will show "BOTH", "LONG", and "SHORT" positions.
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/account/rest-api/Account-Information#response-example)
----------

```
{	"assets": [		{			"asset": "BTC",  // asset name    			"walletBalance": "0.00241969",  // total wallet balance   			"unrealizedProfit": "0.00000000",  // unrealized profit or loss   			"marginBalance": "0.00241969",  // margin balance   			"maintMargin": "0.00000000",	// maintenance margin   			"initialMargin": "0.00000000",  // total intial margin required with the latest mark price   			"positionInitialMargin": "0.00000000",  // positions" margin required with the latest mark price   			"openOrderInitialMargin": "0.00000000",  // open orders" intial margin required with the latest mark price   			"maxWithdrawAmount": "0.00241969",  // available amount for transfer out   			"crossWalletBalance": "0.00241969",  // wallet balance for crossed margin   			"crossUnPnl": "0.00000000",  // total unrealized profit or loss of crossed positions   			"availableBalance": "0.00241969", // available margin balance			"updateTime": 1625474304765 //update time   		}	 ],	 "positions": [		 {		 	"symbol": "BTCUSD_201225",		 	"positionAmt":"0",	// position amount   			"initialMargin": "0",   			"maintMargin": "0",   			"unrealizedProfit": "0.00000000",   			"positionInitialMargin": "0",   			"openOrderInitialMargin": "0",   			"leverage": "125",   			"isolated": false,   			"positionSide": "BOTH", // BOTH means that it is the position of One-way Mode     			"entryPrice": "0.0",   			"breakEvenPrice": "0.0",  // break-even price   			"maxQty": "50",  // maximum quantity of base asset   			"updateTime": 0   		},  		{  			"symbol": "BTCUSD_201225",		 	"positionAmt":"0",   			"initialMargin": "0",   			"maintMargin": "0",   			"unrealizedProfit": "0.00000000",   			"positionInitialMargin": "0",   			"openOrderInitialMargin": "0",   			"leverage": "125",   			"isolated": false,   			"positionSide": "LONG",  // LONG or SHORT means that it is the position of Hedge Mode    			"entryPrice": "0.0",   			"breakEvenPrice": "0.0",  // break-even price   			"maxQty": "50",   			"updateTime": 0   		},  		{  			"symbol": "BTCUSD_201225",		 	"positionAmt":"0",   			"initialMargin": "0",   			"maintMargin": "0",   			"unrealizedProfit": "0.00000000",   			"positionInitialMargin": "0",   			"openOrderInitialMargin": "0",   			"leverage": "125",   			"isolated": false,   			"positionSide": "SHORT",  // LONG or SHORT means that it is the position of Hedge Mode    			"entryPrice": "0.0",   			"breakEvenPrice": "0.0",  // break-even price   			"maxQty": "50",			"notionalValue": "0",   			"updateTime":1627026881327   		}	 ],	 "canDeposit": true,	 "canTrade": true,	 "canWithdraw": true,	 "feeTier": 2,	 "updateTime": 0}
```

## NOTIONAL BRACKET FOR SYMBOL

Notional Bracket for Symbol(USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Symbol#api-description)
----------

Get the symbol's notional bracket list.

HTTP Request[​](/docs/derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Symbol#http-request)
----------

GET `/dapi/v2/leverageBracket`

Request Weight[​](/docs/derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Symbol#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Symbol#request-parameters)
----------

|   Name   | Type |Mandatory|Description|
|----------|------|---------|-----------|
|  symbol  |STRING|   NO    |           |
|recvWindow| LONG |   NO    |           |
|timestamp | LONG |   YES   |           |

Response Example[​](/docs/derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Symbol#response-example)
----------

```
[    {        "symbol": "BTCUSD_PERP",        "notionalCoef": 1.50,  //user symbol bracket multiplier, only appears when user's symbol bracket is adjusted         "brackets": [            {                "bracket": 1,   // bracket level                "initialLeverage": 125,  // the maximum leverage                "qtyCap": 50,  // upper edge of base asset quantity                "qtylFloor": 0,  // lower edge of base asset quantity                "maintMarginRatio": 0.004 // maintenance margin rate				"cum": 0.0 // Auxiliary number for quick calculation             },        ]    }]
```

## NOTIONAL BRACKET FOR PAIR

Notional Bracket for Pair(USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Pair#api-description)
----------

**Not recommended to continue using this v1 endpoint**

Get the pair's default notional bracket list, may return ambiguous values when there have been multiple different `symbol` brackets under the `pair`, suggest using the following `GET /dapi/v2/leverageBracket` query instead to get the specific `symbol` notional bracket list.

HTTP Request[​](/docs/derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Pair#http-request)
----------

GET `/dapi/v1/leverageBracket`

Request Weight[​](/docs/derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Pair#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Pair#request-parameters)
----------

|   Name   | Type |Mandatory|Description|
|----------|------|---------|-----------|
|   pair   |STRING|   NO    |           |
|recvWindow| LONG |   NO    |           |
|timestamp | LONG |   YES   |           |

Response Example[​](/docs/derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Pair#response-example)
----------

```
[    {        "pair": "BTCUSD",        "brackets": [            {                "bracket": 1,   // bracket level                "initialLeverage": 125,  // the maximum leverage                "qtyCap": 50,  // upper edge of base asset quantity                "qtylFloor": 0,  // lower edge of base asset quantity                "maintMarginRatio": 0.004 // maintenance margin rate				"cum": 0.0  // Auxiliary number for quick calculation             },        ]    }]
```

## GET CURRENT POSITION MODE

Get Current Position Mode(USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Current-Position-Mode#api-description)
----------

Get user's position mode (Hedge Mode or One-way Mode ) on ***EVERY symbol***

HTTP Request[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Current-Position-Mode#http-request)
----------

GET `/dapi/v1/positionSide/dual`

Request Weight[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Current-Position-Mode#request-weight)
----------

**30**

Request Parameters[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Current-Position-Mode#request-parameters)
----------

|   Name   |Type|Mandatory|Description|
|----------|----|---------|-----------|
|recvWindow|LONG|   NO    |           |
|timestamp |LONG|   YES   |           |

Response Example[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Current-Position-Mode#response-example)
----------

```
{	"dualSidePosition": true // "true": Hedge Mode; "false": One-way Mode}
```

## GET INCOME HISTORY

Get Income History(USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Income-History#api-description)
----------

Get income history

HTTP Request[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Income-History#http-request)
----------

GET `/dapi/v1/income`

Request Weight[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Income-History#request-weight)
----------

**20**

Request Parameters[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Income-History#request-parameters)
----------

|   Name   | Type |Mandatory|                                                        Description                                                        |
|----------|------|---------|---------------------------------------------------------------------------------------------------------------------------|
|  symbol  |STRING|   NO    |                                                                                                                           |
|incomeType|STRING|   NO    |"TRANSFER","WELCOME\_BONUS", "FUNDING\_FEE", "REALIZED\_PNL", "COMMISSION", "INSURANCE\_CLEAR", and "DELIVERED\_SETTELMENT"|
|startTime | LONG |   NO    |                                      Timestamp in ms to get funding from INCLUSIVE.                                       |
| endTime  | LONG |   NO    |                                      Timestamp in ms to get funding until INCLUSIVE.                                      |
|   page   | INT  |   NO    |                                                                                                                           |
|  limit   | INT  |   NO    |                                                   Default 100; max 1000                                                   |
|recvWindow| LONG |   NO    |                                                                                                                           |
|timestamp | LONG |   YES   |                                                                                                                           |

>
>
> * If `incomeType ` is not sent, all kinds of flow will be returned
> * "trandId" is unique in the same "incomeType" for a user
> * The time between `startTime` and `endTime` can not be longer than 1 year
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Income-History#response-example)
----------

```
[	{    	"symbol": "",				// trade symbol, if existing    	"incomeType": "TRANSFER",	// income type    	"income": "-0.37500000",	// income amount    	"asset": "BTC",				// income asset    	"info":"WITHDRAW",			// extra information    	"time": 1570608000000,    	"tranId":"9689322392",		// transaction id    	"tradeId":""				// trade id, if existing	},	{   		"symbol": "BTCUSD_200925",    	"incomeType": "COMMISSION",     	"income": "-0.01000000",    	"asset": "BTC",    	"info":"",    	"time": 1570636800000,    	"tranId":"9689322392",    	"tradeId":"2059192"	}]
```

## GET DOWNLOAD ID FOR FUTURES TRANSACTION HISTORY

Get Download Id For Futures Transaction History(USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Transaction-History#api-description)
----------

Get download id for futures transaction history

HTTP Request[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Transaction-History#http-request)
----------

GET `/dapi/v1/income/asyn`

Request Weight[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Transaction-History#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Transaction-History#request-parameters)
----------

|   Name   |Type|Mandatory|  Description  |
|----------|----|---------|---------------|
|startTime |LONG|   YES   |Timestamp in ms|
| endTime  |LONG|   YES   |Timestamp in ms|
|recvWindow|LONG|   NO    |               |
|timestamp |LONG|   YES   |               |

>
>
> * Request Limitation is 5 times per month, shared by front end download page and rest api
> * The time between `startTime` and `endTime` can not be longer than 1 year
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Transaction-History#response-example)
----------

```
{	"avgCostTimestampOfLast30d":7241837, // Average time taken for data download in the past 30 days  	"downloadId":"546975389218332672",}
```

## GET FUTURES TRANSACTION HISTORY DOWNLOAD LINK BY ID

Get Futures Transaction History Download Link by Id (USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id#api-description)
----------

Get futures transaction history download link by Id

HTTP Request[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id#http-request)
----------

GET `/dapi/v1/income/asyn/id`

Request Weight[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id#request-parameters)
----------

|   Name   | Type |Mandatory|     Description      |
|----------|------|---------|----------------------|
|downloadId|STRING|   YES   |get by download id api|
|recvWindow| LONG |   NO    |                      |
|timestamp | LONG |   YES   |                      |

>
>
> * Download link expiration: 24h
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id#response-example)
----------

>
>
> **Response:**
>
>

```
{	"downloadId":"545923594199212032",  	"status":"completed",     // Enum：completed，processing  	"url":"www.binance.com",  // The link is mapped to download id  	"notified":true,          // ignore  	"expirationTimestamp":1645009771000,  // The link would expire after this timestamp  	"isExpired":null,}
```

>
>
> **OR** (Response when server is processing)
>
>

```
{	"downloadId":"545923594199212032",  	"status":"processing",  	"url":"",   	"notified":false,  	"expirationTimestamp":-1  	"isExpired":null,  	}
```

## GET DOWNLOAD ID FOR FUTURES ORDER HISTORY

Get Download Id For Futures Order History (USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Order-History#api-description)
----------

Get Download Id For Futures Order History

HTTP Request[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Order-History#http-request)
----------

GET `/dapi/v1/order/asyn`

Request Weight[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Order-History#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Order-History#request-parameters)
----------

|   Name   |Type|Mandatory|  Description  |
|----------|----|---------|---------------|
|startTime |LONG|   YES   |Timestamp in ms|
| endTime  |LONG|   YES   |Timestamp in ms|
|recvWindow|LONG|   NO    |               |
|timestamp |LONG|   YES   |               |

>
>
> * Request Limitation is 10 times per month, shared by front end download page and rest api
> * The time between `startTime` and `endTime` can not be longer than 1 year
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Order-History#response-example)
----------

```
{	"avgCostTimestampOfLast30d":7241837, // Average time taken for data download in the past 30 days  	"downloadId":"546975389218332672",}
```

## GET FUTURES ORDER HISTORY DOWNLOAD LINK BY ID

Get Futures Order History Download Link by Id (USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Order-History-Download-Link-by-Id#api-description)
----------

Get futures order history download link by Id

HTTP Request[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Order-History-Download-Link-by-Id#http-request)
----------

GET `/dapi/v1/order/asyn/id`

Request Weight[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Order-History-Download-Link-by-Id#request-weight)
----------

5

Request Parameters[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Order-History-Download-Link-by-Id#request-parameters)
----------

|   Name   | Type |Mandatory|     Description      |
|----------|------|---------|----------------------|
|downloadId|STRING|   YES   |get by download id api|
|recvWindow| LONG |   NO    |                      |
|timestamp | LONG |   YES   |                      |

>
>
> * Download link expiration: 24h
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Order-History-Download-Link-by-Id#response-example)
----------

>
>
> **Response:**
>
>

```
{	"downloadId":"545923594199212032",  	"status":"completed",     // Enum：completed，processing  	"url":"www.binance.com",  // The link is mapped to download id  	"notified":true,          // ignore  	"expirationTimestamp":1645009771000,  // The link would expire after this timestamp  	"isExpired":null,}
```

>
>
> **OR** (Response when server is processing)
>
>

```
{	"downloadId":"545923594199212032",  	"status":"processing",  	"url":"",   	"notified":false,  	"expirationTimestamp":-1  	"isExpired":null,  	}
```

## GET DOWNLOAD ID FOR FUTURES TRADE HISTORY

Get Download Id For Futures Trade History (USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Trade-History#api-description)
----------

Get download id for futures trade history

HTTP Request[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Trade-History#http-request)
----------

GET `/dapi/v1/trade/asyn`

Request Weight[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Trade-History#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Trade-History#request-parameters)
----------

|   Name   |Type|Mandatory|  Description  |
|----------|----|---------|---------------|
|startTime |LONG|   YES   |Timestamp in ms|
| endTime  |LONG|   YES   |Timestamp in ms|
|recvWindow|LONG|   NO    |               |
|timestamp |LONG|   YES   |               |

>
>
> * Request Limitation is 5 times per month, shared by front end download page and rest api
> * The time between `startTime` and `endTime` can not be longer than 1 year
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Trade-History#response-example)
----------

```
{	"avgCostTimestampOfLast30d":7241837, // Average time taken for data download in the past 30 days  	"downloadId":"546975389218332672",}
```

## GET FUTURES TRADE DOWNLOAD LINK BY ID

Get Futures Trade Download Link by Id(USER\_DATA)
==========

API Description[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Trade-Download-Link-by-Id#api-description)
----------

Get futures trade download link by Id

HTTP Request[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Trade-Download-Link-by-Id#http-request)
----------

GET `/dapi/v1/trade/asyn/id`

Request Weight[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Trade-Download-Link-by-Id#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Trade-Download-Link-by-Id#request-parameters)
----------

|   Name   | Type |Mandatory|     Description      |
|----------|------|---------|----------------------|
|downloadId|STRING|   YES   |get by download id api|
|recvWindow| LONG |   NO    |                      |
|timestamp | LONG |   YES   |                      |

>
>
> * Download link expiration: 24h
>
>

Response Example[​](/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Trade-Download-Link-by-Id#response-example)
----------

>
>
> **Response:**
>
>

```
{	"downloadId":"545923594199212032",  	"status":"completed",     // Enum：completed，processing  	"url":"www.binance.com",  // The link is mapped to download id  	"notified":true,          // ignore  	"expirationTimestamp":1645009771000,  // The link would expire after this timestamp  	"isExpired":null,}
```

>
>
> **OR** (Response when server is processing)
>
>

```
{	"downloadId":"545923594199212032",  	"status":"processing",  	"url":"",   	"notified":false,  	"expirationTimestamp":-1  	"isExpired":null,  	}
```

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

10xx - General Server or Network issues[​](/docs/derivatives/coin-margined-futures/error-code#10xx---general-server-or-network-issues)
----------

### -1000 UNKNOWN[​](/docs/derivatives/coin-margined-futures/error-code#-1000-unknown) ###

* An unknown error occured while processing the request.

### -1001 DISCONNECTED[​](/docs/derivatives/coin-margined-futures/error-code#-1001-disconnected) ###

* Internal error; unable to process your request. Please try again.

### -1002 UNAUTHORIZED[​](/docs/derivatives/coin-margined-futures/error-code#-1002-unauthorized) ###

* You are not authorized to execute this request.

### -1003 TOO\_MANY\_REQUESTS[​](/docs/derivatives/coin-margined-futures/error-code#-1003-too_many_requests) ###

* Too many requests; current limit is %s requests per minute. Please use the websocket for live updates to avoid polling the API.
* Way too many requests; IP banned until %s. Please use the websocket for live updates to avoid bans.

### -1004 DUPLICATE\_IP[​](/docs/derivatives/coin-margined-futures/error-code#-1004-duplicate_ip) ###

* This IP is already on the white list

### -1005 NO\_SUCH\_IP[​](/docs/derivatives/coin-margined-futures/error-code#-1005-no_such_ip) ###

* No such IP has been white listed

### -1006 UNEXPECTED\_RESP[​](/docs/derivatives/coin-margined-futures/error-code#-1006-unexpected_resp) ###

* An unexpected response was received from the message bus. Execution status unknown.

### -1007 TIMEOUT[​](/docs/derivatives/coin-margined-futures/error-code#-1007-timeout) ###

* Timeout waiting for response from backend server. Send status unknown; execution status unknown.

### -1010 ERROR\_MSG\_RECEIVED[​](/docs/derivatives/coin-margined-futures/error-code#-1010-error_msg_received) ###

* ERROR\_MSG\_RECEIVED.

### -1011 NON\_WHITE\_LIST[​](/docs/derivatives/coin-margined-futures/error-code#-1011-non_white_list) ###

* This IP cannot access this route.

### -1013 INVALID\_MESSAGE[​](/docs/derivatives/coin-margined-futures/error-code#-1013-invalid_message) ###

* INVALID\_MESSAGE.

### -1014 UNKNOWN\_ORDER\_COMPOSITION[​](/docs/derivatives/coin-margined-futures/error-code#-1014-unknown_order_composition) ###

* Unsupported order combination.

### -1015 TOO\_MANY\_ORDERS[​](/docs/derivatives/coin-margined-futures/error-code#-1015-too_many_orders) ###

* Too many new orders.
* Too many new orders; current limit is %s orders per %s.

### -1016 SERVICE\_SHUTTING\_DOWN[​](/docs/derivatives/coin-margined-futures/error-code#-1016-service_shutting_down) ###

* This service is no longer available.

### -1020 UNSUPPORTED\_OPERATION[​](/docs/derivatives/coin-margined-futures/error-code#-1020-unsupported_operation) ###

* This operation is not supported.

### -1021 INVALID\_TIMESTAMP[​](/docs/derivatives/coin-margined-futures/error-code#-1021-invalid_timestamp) ###

* Timestamp for this request is outside of the recvWindow.
* Timestamp for this request was 1000ms ahead of the server's time.

### -1022 INVALID\_SIGNATURE[​](/docs/derivatives/coin-margined-futures/error-code#-1022-invalid_signature) ###

* Signature for this request is not valid.

### -1023 START\_TIME\_GREATER\_THAN\_END\_TIME[​](/docs/derivatives/coin-margined-futures/error-code#-1023-start_time_greater_than_end_time) ###

* Start time is greater than end time.

11xx - Request issues[​](/docs/derivatives/coin-margined-futures/error-code#11xx---request-issues)
----------

### -1100 ILLEGAL\_CHARS[​](/docs/derivatives/coin-margined-futures/error-code#-1100-illegal_chars) ###

* Illegal characters found in a parameter.
* Illegal characters found in parameter '%s'; legal range is '%s'.

### -1101 TOO\_MANY\_PARAMETERS[​](/docs/derivatives/coin-margined-futures/error-code#-1101-too_many_parameters) ###

* Too many parameters sent for this endpoint.
* Too many parameters; expected '%s' and received '%s'.
* Duplicate values for a parameter detected.

### -1102 MANDATORY\_PARAM\_EMPTY\_OR\_MALFORMED[​](/docs/derivatives/coin-margined-futures/error-code#-1102-mandatory_param_empty_or_malformed) ###

* A mandatory parameter was not sent, was empty/null, or malformed.
* Mandatory parameter '%s' was not sent, was empty/null, or malformed.
* Param '%s' or '%s' must be sent, but both were empty/null!

### -1103 UNKNOWN\_PARAM[​](/docs/derivatives/coin-margined-futures/error-code#-1103-unknown_param) ###

* An unknown parameter was sent.

### -1104 UNREAD\_PARAMETERS[​](/docs/derivatives/coin-margined-futures/error-code#-1104-unread_parameters) ###

* Not all sent parameters were read.
* Not all sent parameters were read; read '%s' parameter(s) but was sent '%s'.

### -1105 PARAM\_EMPTY[​](/docs/derivatives/coin-margined-futures/error-code#-1105-param_empty) ###

* A parameter was empty.
* Parameter '%s' was empty.

### -1106 PARAM\_NOT\_REQUIRED[​](/docs/derivatives/coin-margined-futures/error-code#-1106-param_not_required) ###

* A parameter was sent when not required.
* Parameter '%s' sent when not required.

### -1108 BAD\_ASSET[​](/docs/derivatives/coin-margined-futures/error-code#-1108-bad_asset) ###

* Invalid asset.

### -1109 BAD\_ACCOUNT[​](/docs/derivatives/coin-margined-futures/error-code#-1109-bad_account) ###

* Invalid account.

### -1110 BAD\_INSTRUMENT\_TYPE[​](/docs/derivatives/coin-margined-futures/error-code#-1110-bad_instrument_type) ###

* Invalid symbolType.

### -1111 BAD\_PRECISION[​](/docs/derivatives/coin-margined-futures/error-code#-1111-bad_precision) ###

* Precision is over the maximum defined for this asset.

### -1112 NO\_DEPTH[​](/docs/derivatives/coin-margined-futures/error-code#-1112-no_depth) ###

* No orders on book for symbol.

### -1113 WITHDRAW\_NOT\_NEGATIVE[​](/docs/derivatives/coin-margined-futures/error-code#-1113-withdraw_not_negative) ###

* Withdrawal amount must be negative.

### -1114 TIF\_NOT\_REQUIRED[​](/docs/derivatives/coin-margined-futures/error-code#-1114-tif_not_required) ###

* TimeInForce parameter sent when not required.

### -1115 INVALID\_TIF[​](/docs/derivatives/coin-margined-futures/error-code#-1115-invalid_tif) ###

* Invalid timeInForce.

### -1116 INVALID\_ORDER\_TYPE[​](/docs/derivatives/coin-margined-futures/error-code#-1116-invalid_order_type) ###

* Invalid orderType.

### -1117 INVALID\_SIDE[​](/docs/derivatives/coin-margined-futures/error-code#-1117-invalid_side) ###

* Invalid side.

### -1118 EMPTY\_NEW\_CL\_ORD\_ID[​](/docs/derivatives/coin-margined-futures/error-code#-1118-empty_new_cl_ord_id) ###

* New client order ID was empty.

### -1119 EMPTY\_ORG\_CL\_ORD\_ID[​](/docs/derivatives/coin-margined-futures/error-code#-1119-empty_org_cl_ord_id) ###

* Original client order ID was empty.

### -1120 BAD\_INTERVAL[​](/docs/derivatives/coin-margined-futures/error-code#-1120-bad_interval) ###

* Invalid interval.

### -1121 BAD\_SYMBOL[​](/docs/derivatives/coin-margined-futures/error-code#-1121-bad_symbol) ###

* Invalid symbol.

### -1125 INVALID\_LISTEN\_KEY[​](/docs/derivatives/coin-margined-futures/error-code#-1125-invalid_listen_key) ###

* This listenKey does not exist. Please use `POST /fapi/v1/listenKey` to recreate `listenKey`

### -1127 MORE\_THAN\_XX\_HOURS[​](/docs/derivatives/coin-margined-futures/error-code#-1127-more_than_xx_hours) ###

* Lookup interval is too big.
* More than %s hours between startTime and endTime.

### -1128 OPTIONAL\_PARAMS\_BAD\_COMBO[​](/docs/derivatives/coin-margined-futures/error-code#-1128-optional_params_bad_combo) ###

* Combination of optional parameters invalid.

### -1130 INVALID\_PARAMETER[​](/docs/derivatives/coin-margined-futures/error-code#-1130-invalid_parameter) ###

* Invalid data sent for a parameter.
* Data sent for parameter '%s' is not valid.

### -1136 INVALID\_NEW\_ORDER\_RESP\_TYPE[​](/docs/derivatives/coin-margined-futures/error-code#-1136-invalid_new_order_resp_type) ###

* Invalid newOrderRespType.

20xx - Processing Issues[​](/docs/derivatives/coin-margined-futures/error-code#20xx---processing-issues)
----------

### -2010 NEW\_ORDER\_REJECTED[​](/docs/derivatives/coin-margined-futures/error-code#-2010-new_order_rejected) ###

* NEW\_ORDER\_REJECTED

### -2011 CANCEL\_REJECTED[​](/docs/derivatives/coin-margined-futures/error-code#-2011-cancel_rejected) ###

* CANCEL\_REJECTED

### -2013 NO\_SUCH\_ORDER[​](/docs/derivatives/coin-margined-futures/error-code#-2013-no_such_order) ###

* Order does not exist.

### -2014 BAD\_API\_KEY\_FMT[​](/docs/derivatives/coin-margined-futures/error-code#-2014-bad_api_key_fmt) ###

* API-key format invalid.

### -2015 REJECTED\_MBX\_KEY[​](/docs/derivatives/coin-margined-futures/error-code#-2015-rejected_mbx_key) ###

* Invalid API-key, IP, or permissions for action.

### -2016 NO\_TRADING\_WINDOW[​](/docs/derivatives/coin-margined-futures/error-code#-2016-no_trading_window) ###

* No trading window could be found for the symbol. Try ticker/24hrs instead.

### -2018 BALANCE\_NOT\_SUFFICIENT[​](/docs/derivatives/coin-margined-futures/error-code#-2018-balance_not_sufficient) ###

* Balance is insufficient.

### -2019 MARGIN\_NOT\_SUFFICIEN[​](/docs/derivatives/coin-margined-futures/error-code#-2019-margin_not_sufficien) ###

* Margin is insufficient.

### -2020 UNABLE\_TO\_FILL[​](/docs/derivatives/coin-margined-futures/error-code#-2020-unable_to_fill) ###

* Unable to fill.

### -2021 ORDER\_WOULD\_IMMEDIATELY\_TRIGGER[​](/docs/derivatives/coin-margined-futures/error-code#-2021-order_would_immediately_trigger) ###

* Order would immediately trigger.

### -2022 REDUCE\_ONLY\_REJECT[​](/docs/derivatives/coin-margined-futures/error-code#-2022-reduce_only_reject) ###

* ReduceOnly Order is rejected.

### -2023 USER\_IN\_LIQUIDATION[​](/docs/derivatives/coin-margined-futures/error-code#-2023-user_in_liquidation) ###

* User in liquidation mode now.

### -2024 POSITION\_NOT\_SUFFICIENT[​](/docs/derivatives/coin-margined-futures/error-code#-2024-position_not_sufficient) ###

* Position is not sufficient.

### -2025 MAX\_OPEN\_ORDER\_EXCEEDED[​](/docs/derivatives/coin-margined-futures/error-code#-2025-max_open_order_exceeded) ###

* Reach max open order limit.

### -2026 REDUCE\_ONLY\_ORDER\_TYPE\_NOT\_SUPPORTED[​](/docs/derivatives/coin-margined-futures/error-code#-2026-reduce_only_order_type_not_supported) ###

* This OrderType is not supported when reduceOnly.

### -2027 MAX\_LEVERAGE\_RATIO[​](/docs/derivatives/coin-margined-futures/error-code#-2027-max_leverage_ratio) ###

* Exceeded the maximum allowable position at current leverage.

### -2028 MIN\_LEVERAGE\_RATIO[​](/docs/derivatives/coin-margined-futures/error-code#-2028-min_leverage_ratio) ###

* Leverage is smaller than permitted: insufficient margin balance.

40xx - Filters and other Issues[​](/docs/derivatives/coin-margined-futures/error-code#40xx---filters-and-other-issues)
----------

### -4000 INVALID\_ORDER\_STATUS[​](/docs/derivatives/coin-margined-futures/error-code#-4000-invalid_order_status) ###

* Invalid order status.

### -4001 PRICE\_LESS\_THAN\_ZERO[​](/docs/derivatives/coin-margined-futures/error-code#-4001-price_less_than_zero) ###

* Price less than 0.

### -4002 PRICE\_GREATER\_THAN\_MAX\_PRICE[​](/docs/derivatives/coin-margined-futures/error-code#-4002-price_greater_than_max_price) ###

* Price greater than max price.

### -4003 QTY\_LESS\_THAN\_ZERO[​](/docs/derivatives/coin-margined-futures/error-code#-4003-qty_less_than_zero) ###

* Quantity less than zero.

### -4004 QTY\_LESS\_THAN\_MIN\_QTY[​](/docs/derivatives/coin-margined-futures/error-code#-4004-qty_less_than_min_qty) ###

* Quantity less than min quantity.

### -4005 QTY\_GREATER\_THAN\_MAX\_QTY[​](/docs/derivatives/coin-margined-futures/error-code#-4005-qty_greater_than_max_qty) ###

* Quantity greater than max quantity.

### -4006 STOP\_PRICE\_LESS\_THAN\_ZERO[​](/docs/derivatives/coin-margined-futures/error-code#-4006-stop_price_less_than_zero) ###

* Stop price less than zero.

### -4007 STOP\_PRICE\_GREATER\_THAN\_MAX\_PRICE[​](/docs/derivatives/coin-margined-futures/error-code#-4007-stop_price_greater_than_max_price) ###

* Stop price greater than max price.

### -4008 TICK\_SIZE\_LESS\_THAN\_ZERO[​](/docs/derivatives/coin-margined-futures/error-code#-4008-tick_size_less_than_zero) ###

* Tick size less than zero.

### -4009 MAX\_PRICE\_LESS\_THAN\_MIN\_PRICE[​](/docs/derivatives/coin-margined-futures/error-code#-4009-max_price_less_than_min_price) ###

* Max price less than min price.

### -4010 MAX\_QTY\_LESS\_THAN\_MIN\_QTY[​](/docs/derivatives/coin-margined-futures/error-code#-4010-max_qty_less_than_min_qty) ###

* Max qty less than min qty.

### -4011 STEP\_SIZE\_LESS\_THAN\_ZERO[​](/docs/derivatives/coin-margined-futures/error-code#-4011-step_size_less_than_zero) ###

* Step size less than zero.

### -4012 MAX\_NUM\_ORDERS\_LESS\_THAN\_ZERO[​](/docs/derivatives/coin-margined-futures/error-code#-4012-max_num_orders_less_than_zero) ###

* Max mum orders less than zero.

### -4013 PRICE\_LESS\_THAN\_MIN\_PRICE[​](/docs/derivatives/coin-margined-futures/error-code#-4013-price_less_than_min_price) ###

* Price less than min price.

### -4014 PRICE\_NOT\_INCREASED\_BY\_TICK\_SIZE[​](/docs/derivatives/coin-margined-futures/error-code#-4014-price_not_increased_by_tick_size) ###

* Price not increased by tick size.

### -4015 INVALID\_CL\_ORD\_ID\_LEN[​](/docs/derivatives/coin-margined-futures/error-code#-4015-invalid_cl_ord_id_len) ###

* Client order id is not valid.
* Client order id length should not be more than 36 chars

### -4016 PRICE\_HIGHTER\_THAN\_MULTIPLIER\_UP[​](/docs/derivatives/coin-margined-futures/error-code#-4016-price_highter_than_multiplier_up) ###

* Price is higher than mark price multiplier cap.

### -4017 MULTIPLIER\_UP\_LESS\_THAN\_ZERO[​](/docs/derivatives/coin-margined-futures/error-code#-4017-multiplier_up_less_than_zero) ###

* Multiplier up less than zero.

### -4018 MULTIPLIER\_DOWN\_LESS\_THAN\_ZERO[​](/docs/derivatives/coin-margined-futures/error-code#-4018-multiplier_down_less_than_zero) ###

* Multiplier down less than zero.

### -4019 COMPOSITE\_SCALE\_OVERFLOW[​](/docs/derivatives/coin-margined-futures/error-code#-4019-composite_scale_overflow) ###

* Composite scale too large.

### -4020 TARGET\_STRATEGY\_INVALID[​](/docs/derivatives/coin-margined-futures/error-code#-4020-target_strategy_invalid) ###

* Target strategy invalid for orderType '%s',reduceOnly '%b'.

### -4021 INVALID\_DEPTH\_LIMIT[​](/docs/derivatives/coin-margined-futures/error-code#-4021-invalid_depth_limit) ###

* Invalid depth limit.
* '%s' is not valid depth limit.

### -4022 WRONG\_MARKET\_STATUS[​](/docs/derivatives/coin-margined-futures/error-code#-4022-wrong_market_status) ###

* market status sent is not valid.

### -4023 QTY\_NOT\_INCREASED\_BY\_STEP\_SIZE[​](/docs/derivatives/coin-margined-futures/error-code#-4023-qty_not_increased_by_step_size) ###

* Qty not increased by step size.

### -4024 PRICE\_LOWER\_THAN\_MULTIPLIER\_DOWN[​](/docs/derivatives/coin-margined-futures/error-code#-4024-price_lower_than_multiplier_down) ###

* Price is lower than mark price multiplier floor.

### -4025 MULTIPLIER\_DECIMAL\_LESS\_THAN\_ZERO[​](/docs/derivatives/coin-margined-futures/error-code#-4025-multiplier_decimal_less_than_zero) ###

* Multiplier decimal less than zero.

### -4026 COMMISSION\_INVALID[​](/docs/derivatives/coin-margined-futures/error-code#-4026-commission_invalid) ###

* Commission invalid.
* `%s` less than zero.
* `%s` absolute value greater than `%s`

### -4027 INVALID\_ACCOUNT\_TYPE[​](/docs/derivatives/coin-margined-futures/error-code#-4027-invalid_account_type) ###

* Invalid account type.

### -4028 INVALID\_LEVERAGE[​](/docs/derivatives/coin-margined-futures/error-code#-4028-invalid_leverage) ###

* Invalid leverage
* Leverage `%s` is not valid
* Leverage `%s` already exist with `%s`

### -4029 INVALID\_TICK\_SIZE\_PRECISION[​](/docs/derivatives/coin-margined-futures/error-code#-4029-invalid_tick_size_precision) ###

* Tick size precision is invalid.

### -4030 INVALID\_STEP\_SIZE\_PRECISION[​](/docs/derivatives/coin-margined-futures/error-code#-4030-invalid_step_size_precision) ###

* Step size precision is invalid.

### -4031 INVALID\_WORKING\_TYPE[​](/docs/derivatives/coin-margined-futures/error-code#-4031-invalid_working_type) ###

* Invalid parameter working type
* Invalid parameter working type: `%s`

### -4032 EXCEED\_MAX\_CANCEL\_ORDER\_SIZE[​](/docs/derivatives/coin-margined-futures/error-code#-4032-exceed_max_cancel_order_size) ###

* Exceed maximum cancel order size.
* Invalid parameter working type: `%s`

### -4033 INSURANCE\_ACCOUNT\_NOT\_FOUND[​](/docs/derivatives/coin-margined-futures/error-code#-4033-insurance_account_not_found) ###

* Insurance account not found.

### -4044 INVALID\_BALANCE\_TYPE[​](/docs/derivatives/coin-margined-futures/error-code#-4044-invalid_balance_type) ###

* Balance Type is invalid.

### -4045 MAX\_STOP\_ORDER\_EXCEEDED[​](/docs/derivatives/coin-margined-futures/error-code#-4045-max_stop_order_exceeded) ###

* Reach max stop order limit.

### -4046 NO\_NEED\_TO\_CHANGE\_MARGIN\_TYPE[​](/docs/derivatives/coin-margined-futures/error-code#-4046-no_need_to_change_margin_type) ###

* No need to change margin type.

### -4047 THERE\_EXISTS\_OPEN\_ORDERS[​](/docs/derivatives/coin-margined-futures/error-code#-4047-there_exists_open_orders) ###

* Margin type cannot be changed if there exists open orders.

### -4048 THERE\_EXISTS\_QUANTITY[​](/docs/derivatives/coin-margined-futures/error-code#-4048-there_exists_quantity) ###

* Margin type cannot be changed if there exists position.

### -4049 ADD\_ISOLATED\_MARGIN\_REJECT[​](/docs/derivatives/coin-margined-futures/error-code#-4049-add_isolated_margin_reject) ###

* Add margin only support for isolated position.

### -4050 CROSS\_BALANCE\_INSUFFICIENT[​](/docs/derivatives/coin-margined-futures/error-code#-4050-cross_balance_insufficient) ###

* Cross balance insufficient.

### -4051 ISOLATED\_BALANCE\_INSUFFICIENT[​](/docs/derivatives/coin-margined-futures/error-code#-4051-isolated_balance_insufficient) ###

* Isolated balance insufficient.

### -4052 NO\_NEED\_TO\_CHANGE\_AUTO\_ADD\_MARGIN[​](/docs/derivatives/coin-margined-futures/error-code#-4052-no_need_to_change_auto_add_margin) ###

* No need to change auto add margin.

### -4053 AUTO\_ADD\_CROSSED\_MARGIN\_REJECT[​](/docs/derivatives/coin-margined-futures/error-code#-4053-auto_add_crossed_margin_reject) ###

* Auto add margin only support for isolated position.

### -4054 ADD\_ISOLATED\_MARGIN\_NO\_POSITION\_REJECT[​](/docs/derivatives/coin-margined-futures/error-code#-4054-add_isolated_margin_no_position_reject) ###

* Cannot add position margin: position is 0.

### -4055 AMOUNT\_MUST\_BE\_POSITIVE[​](/docs/derivatives/coin-margined-futures/error-code#-4055-amount_must_be_positive) ###

* Amount must be positive.

### -4056 INVALID\_API\_KEY\_TYPE[​](/docs/derivatives/coin-margined-futures/error-code#-4056-invalid_api_key_type) ###

* Invalid api key type.

### -4057 INVALID\_RSA\_PUBLIC\_KEY[​](/docs/derivatives/coin-margined-futures/error-code#-4057-invalid_rsa_public_key) ###

* Invalid api public key

### -4058 MAX\_PRICE\_TOO\_LARGE[​](/docs/derivatives/coin-margined-futures/error-code#-4058-max_price_too_large) ###

* maxPrice and priceDecimal too large,please check.

### -4059 NO\_NEED\_TO\_CHANGE\_POSITION\_SIDE[​](/docs/derivatives/coin-margined-futures/error-code#-4059-no_need_to_change_position_side) ###

* No need to change position side.

### -4060 INVALID\_POSITION\_SIDE[​](/docs/derivatives/coin-margined-futures/error-code#-4060-invalid_position_side) ###

* Invalid position side.

### -4061 POSITION\_SIDE\_NOT\_MATCH[​](/docs/derivatives/coin-margined-futures/error-code#-4061-position_side_not_match) ###

* Order's position side does not match user's setting.

### -4062 REDUCE\_ONLY\_CONFLICT[​](/docs/derivatives/coin-margined-futures/error-code#-4062-reduce_only_conflict) ###

* Invalid or improper reduceOnly value.

### -4067 POSITION\_SIDE\_CHANGE\_EXISTS\_OPEN\_ORDERS[​](/docs/derivatives/coin-margined-futures/error-code#-4067-position_side_change_exists_open_orders) ###

* Position side cannot be changed if there exists open orders.

### -4068 POSITION\_SIDE\_CHANGE\_EXISTS\_QUANTITY[​](/docs/derivatives/coin-margined-futures/error-code#-4068-position_side_change_exists_quantity) ###

* Position side cannot be changed if there exists position.

### -4082 INVALID\_BATCH\_PLACE\_ORDER\_SIZE[​](/docs/derivatives/coin-margined-futures/error-code#-4082-invalid_batch_place_order_size) ###

* Invalid number of batch place orders.
* Invalid number of batch place orders: %s

### -4083 PLACE\_BATCH\_ORDERS\_FAIL[​](/docs/derivatives/coin-margined-futures/error-code#-4083-place_batch_orders_fail) ###

* Fail to place batch orders.

### -4084 UPCOMING\_METHOD[​](/docs/derivatives/coin-margined-futures/error-code#-4084-upcoming_method) ###

* Method is not allowed currently. Upcoming soon.

### -4086 INVALID\_PRICE\_SPREAD\_THRESHOLD[​](/docs/derivatives/coin-margined-futures/error-code#-4086-invalid_price_spread_threshold) ###

* Invalid price spread threshold.

### -4087 INVALID\_PAIR[​](/docs/derivatives/coin-margined-futures/error-code#-4087-invalid_pair) ###

* Invalid pair.

### -4088 INVALID\_TIME\_INTERVAL[​](/docs/derivatives/coin-margined-futures/error-code#-4088-invalid_time_interval) ###

* Invalid time interval.
* Maximum time interval is %s days.

### -4089 REDUCE\_ONLY\_ORDER\_PERMISSION[​](/docs/derivatives/coin-margined-futures/error-code#-4089-reduce_only_order_permission) ###

* User can only place reduce only order.

### -4090 NO\_PLACE\_ORDER\_PERMISSION[​](/docs/derivatives/coin-margined-futures/error-code#-4090-no_place_order_permission) ###

* User can not place order currently.

### -4104 INVALID\_CONTRACT\_TYPE[​](/docs/derivatives/coin-margined-futures/error-code#-4104-invalid_contract_type) ###

* Invalid contract type.

### -4110 INVALID\_CLIENT\_TRAN\_ID\_LEN[​](/docs/derivatives/coin-margined-futures/error-code#-4110-invalid_client_tran_id_len) ###

* clientTranId is not valid.
* Client tran id length should be less than 64 chars.

### -4111 DUPLICATED\_CLIENT\_TRAN\_ID[​](/docs/derivatives/coin-margined-futures/error-code#-4111-duplicated_client_tran_id) ###

* clientTranId is duplicated.
* Client tran id should be unique within 7 days.

### -4112 REDUCE\_ONLY\_MARGIN\_CHECK\_FAILED[​](/docs/derivatives/coin-margined-futures/error-code#-4112-reduce_only_margin_check_failed) ###

* ReduceOnly Order Failed. Please check your existing position and open orders.

### -4113 MARKET\_ORDER\_REJECT[​](/docs/derivatives/coin-margined-futures/error-code#-4113-market_order_reject) ###

* The counterparty's best price does not meet the PERCENT\_PRICE filter limit.

### -4135 INVALID\_ACTIVATION\_PRICE[​](/docs/derivatives/coin-margined-futures/error-code#-4135-invalid_activation_price) ###

* Invalid activation price.

### -4137 QUANTITY\_EXISTS\_WITH\_CLOSE\_POSITION[​](/docs/derivatives/coin-margined-futures/error-code#-4137-quantity_exists_with_close_position) ###

* Quantity must be zero with closePosition equals true.

### -4138 REDUCE\_ONLY\_MUST\_BE\_TRUE[​](/docs/derivatives/coin-margined-futures/error-code#-4138-reduce_only_must_be_true) ###

* Reduce only must be true with closePosition equals true.

### -4139 ORDER\_TYPE\_CANNOT\_BE\_MKT[​](/docs/derivatives/coin-margined-futures/error-code#-4139-order_type_cannot_be_mkt) ###

* Order type can not be market if it's unable to cancel.

### -4142 STRATEGY\_INVALID\_TRIGGER\_PRICE[​](/docs/derivatives/coin-margined-futures/error-code#-4142-strategy_invalid_trigger_price) ###

* REJECT: take profit or stop order will be triggered immediately.

### -4150 ISOLATED\_LEVERAGE\_REJECT\_WITH\_POSITION[​](/docs/derivatives/coin-margined-futures/error-code#-4150-isolated_leverage_reject_with_position) ###

* Leverage reduction is not supported in Isolated Margin Mode with open positions.

### -4151 PRICE\_HIGHTER\_THAN\_STOP\_MULTIPLIER\_UP[​](/docs/derivatives/coin-margined-futures/error-code#-4151-price_highter_than_stop_multiplier_up) ###

* Price is higher than stop price multiplier cap.
* Limit price can't be higher than %s.

### -4152 PRICE\_LOWER\_THAN\_STOP\_MULTIPLIER\_DOWN[​](/docs/derivatives/coin-margined-futures/error-code#-4152-price_lower_than_stop_multiplier_down) ###

* Price is lower than stop price multiplier floor.
* Limit price can't be lower than %s.

### -4154 STOP\_PRICE\_HIGHER\_THAN\_PRICE\_MULTIPLIER\_LIMIT[​](/docs/derivatives/coin-margined-futures/error-code#-4154-stop_price_higher_than_price_multiplier_limit) ###

* Stop price is higher than price multiplier cap.
* Stop price can't be higher than %s

### -4155 STOP\_PRICE\_LOWER\_THAN\_PRICE\_MULTIPLIER\_LIMIT[​](/docs/derivatives/coin-margined-futures/error-code#-4155-stop_price_lower_than_price_multiplier_limit) ###

* PStop price is lower than price multiplier floor.
* Stop price can't be lower than %s

### -4178 MIN\_NOTIONAL[​](/docs/derivatives/coin-margined-futures/error-code#-4178-min_notional) ###

* Order's notional must be no smaller than one (unless you choose reduce only)
* Order's notional must be no smaller than %s (unless you choose reduce only)

### -4192 COOLING\_OFF\_PERIOD[​](/docs/derivatives/coin-margined-futures/error-code#-4192-cooling_off_period) ###

* Trade forbidden due to Cooling-off Period.

### -4194 ADJUST\_LEVERAGE\_KYC\_FAILED[​](/docs/derivatives/coin-margined-futures/error-code#-4194-adjust_leverage_kyc_failed) ###

* Intermediate Personal Verification is required for adjusting leverage over 20x.

### -4195 ADJUST\_LEVERAGE\_ONE\_MONTH\_FAILED[​](/docs/derivatives/coin-margined-futures/error-code#-4195-adjust_leverage_one_month_failed) ###

* More than 20x leverage is available one month after account registration.

### -4196 LIMIT\_ORDER\_ONLY[​](/docs/derivatives/coin-margined-futures/error-code#-4196-limit_order_only) ###

* Only limit order is supported.

### -4197 SAME\_ORDER[​](/docs/derivatives/coin-margined-futures/error-code#-4197-same_order) ###

* No need to modify the order.

### -4198 EXCEED\_MAX\_MODIFY\_ORDER\_LIMIT[​](/docs/derivatives/coin-margined-futures/error-code#-4198-exceed_max_modify_order_limit) ###

* Exceed maximum modify order limit.

### -4199 MOVE\_ORDER\_NOT\_ALLOWED\_SYMBOL\_REASON[​](/docs/derivatives/coin-margined-futures/error-code#-4199-move_order_not_allowed_symbol_reason) ###

* Symbol is not in trading status. Order amendment is not permitted.

### -4200 ADJUST\_LEVERAGE\_X\_DAYS\_FAILED[​](/docs/derivatives/coin-margined-futures/error-code#-4200-adjust_leverage_x_days_failed) ###

* More than 20x leverage is available 30 days after Futures account registration.
* More than 20x leverage is available %s days after Futures account registration.

### -4201 ADJUST\_LEVERAGE\_KYC\_LIMIT[​](/docs/derivatives/coin-margined-futures/error-code#-4201-adjust_leverage_kyc_limit) ###

* Users in this country has limited adjust leverage.
* Users in your location/country can only access a maximum leverage of %s

### -4202 ADJUST\_LEVERAGE\_ACCOUNT\_SYMBOL\_FAILED[​](/docs/derivatives/coin-margined-futures/error-code#-4202-adjust_leverage_account_symbol_failed) ###

* Current symbol leverage cannot exceed 20 when using position limit adjustment service.

### -4188 ME\_INVALID\_TIMESTAMP[​](/docs/derivatives/coin-margined-futures/error-code#-4188-me_invalid_timestamp) ###

* Timestamp for this request is outside of the ME recvWindow.

