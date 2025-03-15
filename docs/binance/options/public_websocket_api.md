# Binance Options Public Websocket API Documentation

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

General API Information[​](/docs/derivatives/option/general-info#general-api-information)
----------

* Some endpoints will require an API Key. Please refer to [this page](https://www.binance.com/en/support/articles/360002502072)
* The base endpoint is: \*\*[https://eapi.binance.com](https://eapi.binance.com)
* All endpoints return either a JSON object or array.
* Data is returned in ascending order. Oldest first, newest last.
* All time and timestamp related fields are in milliseconds.

### HTTP Return Codes[​](/docs/derivatives/option/general-info#http-return-codes) ###

* HTTP `4XX` return codes are used for for malformed requests;
  the issue is on the sender's side.
* HTTP `403` return code is used when the WAF Limit (Web Application Firewall) has been violated.
* HTTP `429` return code is used when breaking a request rate limit.
* HTTP `418` return code is used when an IP has been auto-banned for continuing to send requests after receiving `429` codes.
* HTTP `5XX` return codes are used for internal errors; the issue is on
  Binance's side.
* HTTP `503` return code is used when:
  1. If there is an error message **"Unknown error, please check your request or try again later."** returned in the response, the API successfully sent the request but not get a response within the timeout period.  
     It is important to **NOT** treat this as a failure operation; the execution status is **UNKNOWN** and could have been a success;
  2. If there is an error message **"Service Unavailable."** returned in the response, it means this is a failure API operation and the service might be unavailable at the moment, you need to retry later.
  3. If there is an error message **"Internal error; unable to process your request. Please try again."** returned in the response, it means this is a failure API operation and you can resend your request if you need.

### Error Codes and Messages[​](/docs/derivatives/option/general-info#error-codes-and-messages) ###

* Any endpoint can return an ERROR

>
>
> ***The error payload is as follows:***
>
>

```
{  "code": -1121,  "msg": "Invalid symbol."}
```

* Specific error codes and messages defined in [Error Codes](/docs/derivatives/option/general-info#error-codes).

### General Information on Endpoints[​](/docs/derivatives/option/general-info#general-information-on-endpoints) ###

* For `GET` endpoints, parameters must be sent as a `query string` without setting content type in the http headers.
* For `POST`, `PUT`, and `DELETE` endpoints, the parameters may be sent as a`query string` or in the `request body` with content type`application/x-www-form-urlencoded`. You may mix parameters between both the`query string` and `request body` if you wish to do so.
* Parameters may be sent in any order.
* If a parameter sent in both the `query string` and `request body`, the`query string` parameter will be used.

LIMITS[​](/docs/derivatives/option/general-info#limits)
----------

* The `/eapi/v1/exchangeInfo` `rateLimits` array contains objects related to the exchange's `RAW_REQUEST`, `REQUEST_WEIGHT`, and `ORDER` rate limits. These are further defined in the `ENUM definitions` section under `Rate limiters (rateLimitType)`.
* A `429` will be returned when either rate limit is violated.

Binance has the right to further tighten the rate limits on users with intent to attack.

### IP Limits[​](/docs/derivatives/option/general-info#ip-limits) ###

* Every request will contain `X-MBX-USED-WEIGHT-(intervalNum)(intervalLetter)` in the response headers which has the current used weight for the IP for all request rate limiters defined.
* Each route has a `weight` which determines for the number of requests each endpoint counts for. Heavier endpoints and endpoints that do operations on multiple symbols will have a heavier `weight`.
* When a 429 is received, it's your obligation as an API to back off and not spam the API.
* **Repeatedly violating rate limits and/or failing to back off after receiving 429s will result in an automated IP ban (HTTP status 418).**
* IP bans are tracked and **scale in duration** for repeat offenders, **from 2 minutes to 3 days**.
* **The limits on the API are based on the IPs, not the API keys.**

It is strongly recommended to use websocket stream for getting data as much as possible, which can not only ensure the timeliness of the message, but also reduce the access restriction pressure caused by the request.

### Order Rate Limits[​](/docs/derivatives/option/general-info#order-rate-limits) ###

* Every order response will contain a `X-MBX-ORDER-COUNT-(intervalNum)(intervalLetter)` header which has the current order count for the account for all order rate limiters defined.
* Rejected/unsuccessful orders are not guaranteed to have `X-MBX-ORDER-COUNT-**` headers in the response.
* **The order rate limit is counted against each account**.

Endpoint Security Type[​](/docs/derivatives/option/general-info#endpoint-security-type)
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

SIGNED (TRADE and USER\_DATA) Endpoint Security[​](/docs/derivatives/option/general-info#signed-trade-and-user_data-endpoint-security)
----------

* `SIGNED` endpoints require an additional parameter, `signature`, to be
  sent in the `query string` or `request body`.
* Endpoints use `HMAC SHA256` signatures. The `HMAC SHA256 signature` is a keyed `HMAC SHA256` operation.
  Use your `secretKey` as the key and `totalParams` as the value for the HMAC operation.
* The `signature` is **not case sensitive**.
* Please make sure the `signature` is the end part of your `query string` or `request body`.
* `totalParams` is defined as the `query string` concatenated with the`request body`.

### Timing Security[​](/docs/derivatives/option/general-info#timing-security) ###

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

### SIGNED Endpoint Examples for POST /eapi/v1/order[​](/docs/derivatives/option/general-info#signed-endpoint-examples-for-post-eapiv1order) ###

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

#### Example 1: As a query string[​](/docs/derivatives/option/general-info#example-1-as-a-query-string) ####

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
    $ echo -n "symbol=BTC-210129-40000-C&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=2000&recvWindow=5000&timestamp=1611825601400" | openssl dgst -sha256 -hmac "YtP1BudNOWZE1ag5uzCkh4hIC7qSmQOu797r5EJBFGhxBYivjj8HIX0iiiPof5yG"    (stdin)= 7c12045972f6140e765e0f2b67d28099718df805732676494238f50be830a7d7
```

>
>
> **curl command:**
>
>

```
    (HMAC SHA256)    $ curl -H "X-MBX-APIKEY: 22BjeOROKiXJ3NxbR3zjh3uoGcaflPu3VMyBXAg8Jj2J1xVSnY0eB4dzacdE9IWn" -X POST 'https://eapi.binance.com/eapi/v1/order' -d 'symbol=BTC-210129-40000-C&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=2000&recvWindow=5000&timestamp=1611825601400&signature=7c12045972f6140e765e0f2b67d28099718df805732676494238f50be830a7d7'
```

* **requestBody:**

symbol=BTC-210129-40000-C  
&side=BUY  
&type=LIMIT  
&timeInForce=GTC  
&quantity=1  
&price=2000  
&recvWindow=5000  
&timestamp=1611825601400

#### Example 2: As a request body[​](/docs/derivatives/option/general-info#example-2-as-a-request-body) ####

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
    $ echo -n "symbol=BTC-210129-40000-C&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=2000&recvWindow=5000&timestamp=1611825601400" | openssl dgst -sha256 -hmac "YtP1BudNOWZE1ag5uzCkh4hIC7qSmQOu797r5EJBFGhxBYivjj8HIX0iiiPof5yG"    (stdin)= 7c12045972f6140e765e0f2b67d28099718df805732676494238f50be830a7d7
```

>
>
> **curl command:**
>
>

```
    (HMAC SHA256)   $ curl -H "X-MBX-APIKEY: 22BjeOROKiXJ3NxbR3zjh3uoGcaflPu3VMyBXAg8Jj2J1xVSnY0eB4dzacdE9IWn" -X POST 'https://eapi.binance.com/eapi/v1/order?symbol=BTC-210129-40000-C&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=2000&recvWindow=5000&timestamp=1611825601400&signature=7c12045972f6140e765e0f2b67d28099718df805732676494238f50be830a7d7'
```

* **queryString:**

symbol=BTC-210129-40000-C  
&side=BUY  
&type=LIMIT  
&timeInForce=GTC  
&quantity=1  
&price=2000  
&recvWindow=5000  
&timestamp=1611825601400

#### Example 3: Mixed query string and request body[​](/docs/derivatives/option/general-info#example-3-mixed-query-string-and-request-body) ####

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
   $ echo -n "symbol=BTC-210129-40000-C&side=BUY&type=LIMIT&timeInForce=GTCquantity=0.01&price=2000&recvWindow=5000&timestamp=1611825601400" | openssl dgst -sha256 -hmac "YtP1BudNOWZE1ag5uzCkh4hIC7qSmQOu797r5EJBFGhxBYivjj8HIX0iiiPof5yG"    (stdin)= fa6045c54fb02912b766442be1f66fab619217e551a4fb4f8a1ee000df914d8e
```

>
>
> **curl command:**
>
>

```
    (HMAC SHA256)    $ curl -H "X-MBX-APIKEY: 22BjeOROKiXJ3NxbR3zjh3uoGcaflPu3VMyBXAg8Jj2J1xVSnY0eB4dzacdE9IWn" -X POST 'https://eapi.binance.com/eapi/v1/order?symbol=BTC-210129-40000-C&side=BUY&type=LIMIT&timeInForce=GTC' -d 'quantity=0.01&price=2000&recvWindow=5000&timestamp=1611825601400&signature=fa6045c54fb02912b766442be1f66fab619217e551a4fb4f8a1ee000df914d8e'
```

* **queryString:**

symbol=BTC-210129-40000-C&side=BUY&type=LIMIT&timeInForce=GTC

* **requestBody:**

quantity=1&price=2000&recvWindow=5000&timestamp=1611825601400

Note that the signature is different in example 3.
There is no & between "GTC" and "quantity=1".

## COMMON DEFINITION

Public Endpoints Info
==========

Terminology[​](/docs/derivatives/option/common-definition#terminology)
----------

* `symbol` refers to the symbol name of a options contract symbol
* `underlying` refers to the underlying symbol of a options contract symbol
* `quoteAsset` refers to the asset that is the price of a symbol.
* `settleAsset` refers to the settlement asset when options are exercised

ENUM definitions[​](/docs/derivatives/option/common-definition#enum-definitions)
----------

**Options contract type**

* CALL
* PUT

**Order side (side)**

* BUY
* SELL

**Position side (positionSide)**

* LONG
* SHORT

**Time in force (timeInForce)**

* GTC - Good Till Cancel
* IOC - Immediate or Cancel
* FOK - Fill or Kill

**Response Type (newOrderRespType)**

* ACK
* RESULT

**Order types (type)**

* LIMIT

**Order status (status)**

* ACCEPTED
* REJECTED
* PARTIALLY\_FILLED
* FILLED
* CANCELLED

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

Symbol filters[​](/docs/derivatives/option/common-definition#symbol-filters)
----------

### PRICE\_FILTER[​](/docs/derivatives/option/common-definition#price_filter) ###

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

* sell order `price` \>= `minPrice`
* buy order `price` \<= `maxPrice`
* (`price`-`minPrice`) % `tickSize` == 0

### LOT\_SIZE[​](/docs/derivatives/option/common-definition#lot_size) ###

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

10xx - General Server or Network issues[​](/docs/derivatives/option/error-code#10xx---general-server-or-network-issues)
----------

### -1000 UNKNOWN[​](/docs/derivatives/option/error-code#-1000-unknown) ###

* An unknown error occurred while processing the request.

### -1001 DISCONNECTED[​](/docs/derivatives/option/error-code#-1001-disconnected) ###

* Internal error; unable to process your request. Please try again.

### -1002 UNAUTHORIZED[​](/docs/derivatives/option/error-code#-1002-unauthorized) ###

* You are not authorized to execute this request.

### -1008 TOO\_MANY\_REQUESTS[​](/docs/derivatives/option/error-code#-1008-too_many_requests) ###

* Too many requests queued.
* Too much request weight used; please use the websocket for live updates to avoid polling the API.
* Too much request weight used; current limit is %s request weight per %s %s. Please use the websocket for live updates to avoid polling the API.
* Way too much request weight used; IP banned until %s. Please use the websocket for live updates to avoid bans.

### -1014 UNKNOWN\_ORDER\_COMPOSITION[​](/docs/derivatives/option/error-code#-1014-unknown_order_composition) ###

* Unsupported order combination.

### -1015 TOO\_MANY\_ORDERS[​](/docs/derivatives/option/error-code#-1015-too_many_orders) ###

* Too many new orders.
* Too many new orders; current limit is %s orders per %s.

### -1016 SERVICE\_SHUTTING\_DOWN[​](/docs/derivatives/option/error-code#-1016-service_shutting_down) ###

* This service is no longer available.

### -1020 UNSUPPORTED\_OPERATION[​](/docs/derivatives/option/error-code#-1020-unsupported_operation) ###

* This operation is not supported.

### -1021 INVALID\_TIMESTAMP[​](/docs/derivatives/option/error-code#-1021-invalid_timestamp) ###

* Timestamp for this request is outside of the recvWindow.
* Timestamp for this request was 1000ms ahead of the server's time.

### -1022 INVALID\_SIGNATURE[​](/docs/derivatives/option/error-code#-1022-invalid_signature) ###

* Signature for this request is not valid.

11xx - 2xxx Request issues[​](/docs/derivatives/option/error-code#11xx---2xxx-request-issues)
----------

### -1100 ILLEGAL\_CHARS[​](/docs/derivatives/option/error-code#-1100-illegal_chars) ###

* Illegal characters found in a parameter.
* Illegal characters found in a parameter. %s
* Illegal characters found in parameter `%s`; legal range is `%s`.

### -1101 TOO\_MANY\_PARAMETERS[​](/docs/derivatives/option/error-code#-1101-too_many_parameters) ###

* Too many parameters sent for this endpoint.
* Too many parameters; expected `%s` and received `%s`.
* Duplicate values for a parameter detected.

### -1102 MANDATORY\_PARAM\_EMPTY\_OR\_MALFORMED[​](/docs/derivatives/option/error-code#-1102-mandatory_param_empty_or_malformed) ###

* A mandatory parameter was not sent, was empty/null, or malformed.
* Mandatory parameter `%s` was not sent, was empty/null, or malformed.
* Param `%s` or `%s` must be sent, but both were empty/null!

### -1103 UNKNOWN\_PARAM[​](/docs/derivatives/option/error-code#-1103-unknown_param) ###

* An unknown parameter was sent.

### -1104 UNREAD\_PARAMETERS[​](/docs/derivatives/option/error-code#-1104-unread_parameters) ###

* Not all sent parameters were read.
* Not all sent parameters were read; read `%s` parameter(s) but was sent `%s`.

### -1105 PARAM\_EMPTY[​](/docs/derivatives/option/error-code#-1105-param_empty) ###

* A parameter was empty.
* Parameter `%s` was empty.

### -1106 PARAM\_NOT\_REQUIRED[​](/docs/derivatives/option/error-code#-1106-param_not_required) ###

* A parameter was sent when not required.
* Parameter `%s` sent when not required.

### -1111 BAD\_PRECISION[​](/docs/derivatives/option/error-code#-1111-bad_precision) ###

* Precision is over the maximum defined for this asset.

### -1115 INVALID\_TIF[​](/docs/derivatives/option/error-code#-1115-invalid_tif) ###

* Invalid timeInForce.

### -1116 INVALID\_ORDER\_TYPE[​](/docs/derivatives/option/error-code#-1116-invalid_order_type) ###

* Invalid orderType.

### -1117 INVALID\_SIDE[​](/docs/derivatives/option/error-code#-1117-invalid_side) ###

* Invalid side.

### -1118 EMPTY\_NEW\_CL\_ORD\_ID[​](/docs/derivatives/option/error-code#-1118-empty_new_cl_ord_id) ###

* New client order ID was empty.

### -1119 EMPTY\_ORG\_CL\_ORD\_ID[​](/docs/derivatives/option/error-code#-1119-empty_org_cl_ord_id) ###

* Original client order ID was empty.

### -1120 BAD\_INTERVAL[​](/docs/derivatives/option/error-code#-1120-bad_interval) ###

* Invalid interval.

### -1121 BAD\_SYMBOL[​](/docs/derivatives/option/error-code#-1121-bad_symbol) ###

* Invalid symbol.

### -1125 INVALID\_LISTEN\_KEY[​](/docs/derivatives/option/error-code#-1125-invalid_listen_key) ###

* This listenKey does not exist.

### -1127 MORE\_THAN\_XX\_HOURS[​](/docs/derivatives/option/error-code#-1127-more_than_xx_hours) ###

* Lookup interval is too big.
* More than %s hours between startTime and endTime.

### -1128 BAD\_CONTRACT[​](/docs/derivatives/option/error-code#-1128-bad_contract) ###

* Invalid underlying

### -1129 BAD\_CURRENCY[​](/docs/derivatives/option/error-code#-1129-bad_currency) ###

* Invalid asset。

### -1130 INVALID\_PARAMETER[​](/docs/derivatives/option/error-code#-1130-invalid_parameter) ###

* Invalid data sent for a parameter.
* Data sent for paramter `%s` is not valid.

### -1131 BAD\_RECV\_WINDOW[​](/docs/derivatives/option/error-code#-1131-bad_recv_window) ###

* recvWindow must be less than 60000

### -2010 NEW\_ORDER\_REJECTED[​](/docs/derivatives/option/error-code#-2010-new_order_rejected) ###

* NEW\_ORDER\_REJECTED

### -2013 NO\_SUCH\_ORDER[​](/docs/derivatives/option/error-code#-2013-no_such_order) ###

* Order does not exist.

### -2014 BAD\_API\_KEY\_FMT[​](/docs/derivatives/option/error-code#-2014-bad_api_key_fmt) ###

* API-key format invalid.

### -2015 INVALID\_API\_KEY[​](/docs/derivatives/option/error-code#-2015-invalid_api_key) ###

* Invalid API-key, IP, or permissions for action.

### -2018 BALANCE\_NOT\_SUFFICIENT[​](/docs/derivatives/option/error-code#-2018-balance_not_sufficient) ###

* Balance is insufficient.

### -2027 OPTION\_MARGIN\_NOT\_SUFFICIENT[​](/docs/derivatives/option/error-code#-2027-option_margin_not_sufficient) ###

* Option margin is insufficient.

3xxx-5xxx Filters and other issues[​](/docs/derivatives/option/error-code#3xxx-5xxx-filters-and-other-issues)
----------

### -3029 TRANSFER\_FAILED[​](/docs/derivatives/option/error-code#-3029-transfer_failed) ###

* Asset transfer fail.

### -4001 PRICE\_LESS\_THAN\_ZERO[​](/docs/derivatives/option/error-code#-4001-price_less_than_zero) ###

* Price less than 0.

### -4002 PRICE\_GREATER\_THAN\_MAX\_PRICE[​](/docs/derivatives/option/error-code#-4002-price_greater_than_max_price) ###

* Price greater than max price.

### -4003 QTY\_LESS\_THAN\_ZERO[​](/docs/derivatives/option/error-code#-4003-qty_less_than_zero) ###

* Quantity less than zero.

### -4004 QTY\_LESS\_THAN\_MIN\_QTY[​](/docs/derivatives/option/error-code#-4004-qty_less_than_min_qty) ###

* Quantity less than min quantity.

### -4005 QTY\_GREATER\_THAN\_MAX\_QTY[​](/docs/derivatives/option/error-code#-4005-qty_greater_than_max_qty) ###

* Quantity greater than max quantity.

### -4013 PRICE\_LESS\_THAN\_MIN\_PRICE[​](/docs/derivatives/option/error-code#-4013-price_less_than_min_price) ###

* Price less than min price.

### -4029 INVALID\_TICK\_SIZE\_PRECISION[​](/docs/derivatives/option/error-code#-4029-invalid_tick_size_precision) ###

* Tick size precision is invalid.

### -4030 INVALID\_QTY\_PRECISION[​](/docs/derivatives/option/error-code#-4030-invalid_qty_precision) ###

* Step size precision is invalid.

### -4055 AMOUNT\_MUST\_BE\_POSITIVE[​](/docs/derivatives/option/error-code#-4055-amount_must_be_positive) ###

* Amount must be positive.

## WEBSOCKET MARKET STREAMS

Connect
==========

* The baseurl of the websocket interface is: ****wss://nbstream.binance.com/eoptions/****

* Streams can be access either in a single raw stream or a combined stream

* Raw streams are accessed at **/ws/\<streamName\>**

* Combined streams are accessed at **/stream?streams=\<streamName1\>/\<streamName2\>/\<streamName3\>**

* Example:

* `wss://nbstream.binance.com/eoptions/ws/BTC-210630-9000-P@ticker`

* `wss://nbstream.binance.com/eoptions/stream?streams=BTC-210630-9000-P@trade/BTC-210630-9000-P@ticker`

* A single connection is only valid for 24 hours; expect to be disconnected at the 24 hour mark

* The websocket server will send a `ping frame` every 5 minutes. If the websocket server does not receive a `pong frame` back from the connection within a 15 minute period, the connection will be disconnected. Unsolicited `pong frames` are allowed.

* WebSocket connections have a limit of 10 incoming messages per second.

* A connection that goes beyond the limit will be disconnected; IPs that are repeatedly disconnected may be banned.

* A single connection can listen to a maximum of **200** streams.

* Considering the possible data latency from RESTful endpoints during an extremely volatile market, it is highly recommended to get the order status, position, etc from the Websocket user data stream.

* Combined stream events are wrapped as follows: **{"stream":"\<streamName\>","data":\<rawPayload\>}**

* All symbols for streams are **uppercase**

* A single connection is only valid for 24 hours; expect to be disconnected at the 24 hour mark

* The websocket server will send a `ping frame` every 5 minutes. If the websocket server does not receive a `pong frame` back from the connection within a 15 minute period, the connection will be disconnected. Unsolicited `pong frames` are allowed.

* WebSocket connections have a limit of 10 incoming messages per second.

* A connection that goes beyond the limit will be disconnected; IPs that are repeatedly disconnected may be banned.

* A single connection can listen to a maximum of **200** streams.

* Considering the possible data latency from RESTful endpoints during an extremely volatile market, it is highly recommended to get the order status, position, etc from the Websocket user data stream.

## LIVE SUBSCRIBING UNSUBSCRIBING TO STREAMS

Live Subscribing/Unsubscribing to streams
==========

* The following data can be sent through the websocket instance in order to subscribe/unsubscribe from streams. Examples can be seen below.
* The `id` used in the JSON payloads is an unsigned INT used as an identifier to uniquely identify the messages going back and forth.

Subscribe to a stream[​](/docs/derivatives/option/websocket-market-streams/Live-Subscribing-Unsubscribing-to-streams#subscribe-to-a-stream)
----------

>
>
> **Response**
>
>

```
{  "result": null,  "id": 1}
```

* **Request**

{  
"method": "SUBSCRIBE",  
"params":  
[  
"BTC-210630-9000-P@ticker",  
"BTC-210630-9000-P@depth"  
],  
"id": 1  
}

Unsubscribe to a stream[​](/docs/derivatives/option/websocket-market-streams/Live-Subscribing-Unsubscribing-to-streams#unsubscribe-to-a-stream)
----------

>
>
> **Response**
>
>

```
{  "result": null,  "id": 312}
```

* **Request**

  {  
  "method": "UNSUBSCRIBE",  
  "params":  
  [  
  "BTC-210630-9000-P@ticker"  
  ],  
  "id": 312  
  }

Listing Subscriptions[​](/docs/derivatives/option/websocket-market-streams/Live-Subscribing-Unsubscribing-to-streams#listing-subscriptions)
----------

>
>
> **Response**
>
>

```
{  "result": [    "BTC-210630-9000-P@ticker"  ],  "id": 3}
```

* **Request**

  {  
  "method": "LIST\_SUBSCRIPTIONS",  
  "id": 3  
  }

Setting Properties[​](/docs/derivatives/option/websocket-market-streams/Live-Subscribing-Unsubscribing-to-streams#setting-properties)
----------

Currently, the only property can be set is to set whether `combined` stream payloads are enabled are not.
The combined property is set to `false` when connecting using `/ws/` ("raw streams") and `true` when connecting using `/stream/`.

>
>
> **Response**
>
>

```
{  "result": null,  "id": 5}
```

* **Request**

  {  
  "method": "SET\_PROPERTY",  
  "params":  
  [  
  "combined",  
  true  
  ],  
  "id": 5  
  }

Retrieving Properties[​](/docs/derivatives/option/websocket-market-streams/Live-Subscribing-Unsubscribing-to-streams#retrieving-properties)
----------

>
>
> **Response**
>
>

```
{  "result": true, // Indicates that combined is set to true.  "id": 2}
```

* **Request**

  {  
  "method": "GET\_PROPERTY",  
  "params":  
  [  
  "combined"  
  ],  
  "id": 2  
  }

Error Messages[​](/docs/derivatives/option/websocket-market-streams/Live-Subscribing-Unsubscribing-to-streams#error-messages)
----------

|                                                                                 Error Message                                                                                 |                                            Description                                             |
|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|
|                                                                    {"code": 0, "msg": "Unknown property"}                                                                     |                 Parameter used in the `SET_PROPERTY` or `GET_PROPERTY` was invalid                 |
|                                                          {"code": 1, "msg": "Invalid value type: expected Boolean"}                                                           |                               Value should only be `true` or `false`                               |
|                                                     {"code": 2, "msg": "Invalid request: property name must be a string"}                                                     |                                 Property name provided was invalid                                 |
|                                                 {"code": 2, "msg": "Invalid request: request ID must be an unsigned integer"}                                                 |Parameter `id` had to be provided or the value provided in the `id` parameter is an unsupported type|
|{"code": 2, "msg": "Invalid request: unknown variant %s, expected one of `SUBSCRIBE`, `UNSUBSCRIBE`, `LIST_SUBSCRIPTIONS`, `SET_PROPERTY`, `GET_PROPERTY` at line 1 column 28"}|     Possible typo in the provided method or provided method was neither of the expected values     |
|                                                          {"code": 2, "msg": "Invalid request: too many parameters"}                                                           |                            Unnecessary parameters provided in the data                             |
|                                                     {"code": 2, "msg": "Invalid request: property name must be a string"}                                                     |                                   Property name was not provided                                   |
|                                               {"code": 2, "msg": "Invalid request: missing field `method` at line 1 column 73"}                                               |                               `method` was not provided in the data                                |
|                                                     {"code":3,"msg":"Invalid JSON: expected value at line %s column %s"}                                                      |                       JSON data sent has incorrect syntax. ## Trade Streams                        |

## NEW SYMBOL INFO

New Symbol Info
==========

Stream Description[​](/docs/derivatives/option/websocket-market-streams/New-Symbol-Info#stream-description)
----------

New symbol listing stream.

Stream Name[​](/docs/derivatives/option/websocket-market-streams/New-Symbol-Info#stream-name)
----------

`option_pair`

Update Speed[​](/docs/derivatives/option/websocket-market-streams/New-Symbol-Info#update-speed)
----------

**50ms**

Response Example[​](/docs/derivatives/option/websocket-market-streams/New-Symbol-Info#response-example)
----------

```
{    "e":"OPTION_PAIR",         //eventType       "E":1668573571842,         //eventTime       "u":"BTCUSDT",             //Underlying index of the contract    "qa":"USDT",               //Quotation asset     "s":"BTC-221116-21000-C",  //Trading pair name     "unit":1,                  //Conversion ratio, the quantity of the underlying asset represented by a single contract.     "mq":"0.01",               //Minimum trade volume of the underlying asset      "d":"CALL",                //Option type    "sp":"21000",              //Strike price      "ed":1668585600000         //expiration time  }
```

## OPEN INTEREST

Open Interest
==========

Stream Description[​](/docs/derivatives/option/websocket-market-streams/Open-Interest#stream-description)
----------

Option open interest for specific underlying asset on specific expiration date. E.g.[ETH@openInterest@221125](wss://nbstream.binance.com/eoptions/stream?streams=ETH@openInterest@221125)

Stream Name[​](/docs/derivatives/option/websocket-market-streams/Open-Interest#stream-name)
----------

`<underlyingAsset>@openInterest@<expirationDate>`

Update Speed[​](/docs/derivatives/option/websocket-market-streams/Open-Interest#update-speed)
----------

**60s**

Response Example[​](/docs/derivatives/option/websocket-market-streams/Open-Interest#response-example)
----------

```
[    {      "e":"openInterest",         // Event type      "E":1668759300045,          // Event time      "s":"ETH-221125-2700-C",    // option symbol      "o":"1580.87",              // Open interest in contracts      "h":"1912992.178168204"     // Open interest in USDT    }]
```

## MARK PRICE

Mark Price
==========

Stream Description[​](/docs/derivatives/option/websocket-market-streams/Mark-Price#stream-description)
----------

The mark price for all option symbols on specific underlying asset. E.g.[ETH@markPrice](wss://nbstream.binance.com/eoptions/stream?streams=ETH@markPrice)

Stream Name[​](/docs/derivatives/option/websocket-market-streams/Mark-Price#stream-name)
----------

`<underlyingAsset>@markPrice`

Update Speed[​](/docs/derivatives/option/websocket-market-streams/Mark-Price#update-speed)
----------

**1000ms**

Response Example[​](/docs/derivatives/option/websocket-market-streams/Mark-Price#response-example)
----------

```
[    {      "e":"markPrice",         // Event Type      "E":1663684594227,       // Event time      "s":"ETH-220930-1500-C", // Symbol      "mp":"30.3"              // Option mark price    },    {      "e":"markPrice",      "E":1663684594228,      "s":"ETH-220923-1000-C",      "mp":"341.5"    }]
```

## KLINE CANDLESTICK STREAMS

Kline/Candlestick Streams
==========

Stream Description[​](/docs/derivatives/option/websocket-market-streams/Kline-Candlestick-Streams#stream-description)
----------

The Kline/Candlestick Stream push updates to the current klines/candlestick every 1000 milliseconds (if existing).

**Kline/Candlestick chart intervals:**

m -\> minutes; h -\> hours; d -\> days; w -\> weeks; M -\> months

"1m",
"3m",
"5m",
"15m"
"30m"
"1h",
"2h",
"4h",
"6h",
"12h",
"1d",
"3d",
"1w",

Stream Name[​](/docs/derivatives/option/websocket-market-streams/Kline-Candlestick-Streams#stream-name)
----------

`<symbol>@kline_<interval>`

Update Speed[​](/docs/derivatives/option/websocket-market-streams/Kline-Candlestick-Streams#update-speed)
----------

**1000ms**

Response Example[​](/docs/derivatives/option/websocket-market-streams/Kline-Candlestick-Streams#response-example)
----------

```
{    "e":"kline",                        // event type       "E":1638747660000,                  // event time       "s":"BTC-200630-9000-P",            // Option trading symbol       "k":{                                     "t":1638747660000,              // kline start time           "T":1638747719999,              // kline end time          "s":"BTC-200630-9000-P",        // Option trading symbol           "i":"1m",                       // candle period           "F":0,                          // first trade ID          "L":0,                          // last trade ID           "o":"1000",                     // open           "c":"1000",                     // close           "h":"1000",                     // high            "l":"1000",                     // low           "v":"0",                        // volume(in contracts)           "n":0,                          // number of trades           "x":false,                      // current candle has been completed Y/N           "q":"0",                        // completed trade amount   (in quote asset)                    "V":"0",                        // taker completed trade volume (in contracts)                     "Q":"0"                         // taker trade amount(in quote asset)       }}
```

## 24 HOUR TICKER BY UNDERLYING ASSET AND EXPIRATION DATA

24-hour TICKER by underlying asset and expiration data
==========

Stream Description[​](/docs/derivatives/option/websocket-market-streams/24-hour-TICKER-by-underlying-asset-and-expiration-data#stream-description)
----------

24hr ticker info by underlying asset and expiration date. E.g.[ETH@ticker@220930](wss://nbstream.binance.com/eoptions/stream?streams=ETH@ticker@220930)

Stream Name[​](/docs/derivatives/option/websocket-market-streams/24-hour-TICKER-by-underlying-asset-and-expiration-data#stream-name)
----------

`<underlyingAsset>@ticker@<expirationDate>`

Update Speed[​](/docs/derivatives/option/websocket-market-streams/24-hour-TICKER-by-underlying-asset-and-expiration-data#update-speed)
----------

**1000ms** 

Response Example[​](/docs/derivatives/option/websocket-market-streams/24-hour-TICKER-by-underlying-asset-and-expiration-data#response-example)
----------

```
[{      "e":"24hrTicker",           // event type      "E":1657706425200,          // event time       "T":1657706425220,          // transaction time        "s":"ETH-220930-1600-C",    // Option symbol          "o":"2000",                 // 24-hour opening price      "h":"2020",                 // Highest price      "l":"2000",                 // Lowest price      "c":"2020",                 // latest price      "V":"1.42",                 // Trading volume(in contracts)        "A":"2841",                 // trade amount(in quote asset)         "P":"0.01",                 // price change percent      "p":"20",                   // price change       "Q":"0.01",                 // volume of last completed trade(in contracts)        "F":"27",                   // first trade ID      "L":"48",                   // last trade ID        "n":22,                     // number of trades      "bo":"2012",                // The best buy price      "ao":"2020",                // The best sell price      "bq":"4.9",                 // The best buy quantity      "aq":"0.03",                // The best sell quantity      "b":"0.1202",               // BuyImplied volatility         "a":"0.1318",               // SellImplied volatility      "d":"0.98911",              // delta      "t":"-0.16961",             // theta       "g":"0.00004",              // gamma        "v":"2.66584",              // vega      "vo":"0.10001",             // Implied volatility       "mp":"2003.5102",           // Mark price        "hl":"2023.511",            // Buy Maximum price      "ll":"1983.511",            // Sell Minimum price       "eep":"0"                   // Estimated strike price (return estimated strike price half hour before exercise)    },    {      "e":"24hrTicker",      "E":1663685112123,      "s":"ETH-220930-1500-C",      "o":"34.9",      "h":"44.6",      "l":"26.8",      "c":"26.8",      "V":"11.84",      "A":"444.37",      "P":"-0.232",      "p":"-8.1",      "Q":"0",      "F":"91",      "L":"129",      "n":39,      "bo":"26.8",      "ao":"33.9",      "bq":"0.65",      "aq":"0.01",      "b":"0.88790536",      "a":"0.98729014",      "d":"0.2621153",      "t":"-3.44806807",      "g":"0.00158298",      "v":"0.7148147",      "vo":"0.93759775",      "mp":"30.3",      "hl":"228.7",      "ll":"0.1",      "eep":"0"    } ]
```

## INDEX PRICE STREAMS

Index Price Streams
==========

Stream Description[​](/docs/derivatives/option/websocket-market-streams/Index-Price-Streams#stream-description)
----------

Underlying(e.g ETHUSDT) index stream.

**Stream Name:**  
`<symbol>@index`

Update Speed[​](/docs/derivatives/option/websocket-market-streams/Index-Price-Streams#update-speed)
----------

**1000ms**

Response Example[​](/docs/derivatives/option/websocket-market-streams/Index-Price-Streams#response-example)
----------

```
{    "e":"index",         // event type    "E":1661415480351,   // time    "s":"ETHUSDT",       // underlying symbol    "p":"1707.89008607"  // index price}
```

## 24 HOUR TICKER

24-hour TICKER
==========

Stream Description[​](/docs/derivatives/option/websocket-market-streams/24-hour-TICKER#stream-description)
----------

24hr ticker info for all symbols. Only symbols whose ticker info changed will be sent.

Stream Name[​](/docs/derivatives/option/websocket-market-streams/24-hour-TICKER#stream-name)
----------

`<symbol>@ticker`

Update Speed[​](/docs/derivatives/option/websocket-market-streams/24-hour-TICKER#update-speed)
----------

**1000ms**

Response Example[​](/docs/derivatives/option/websocket-market-streams/24-hour-TICKER#response-example)
----------

```
{    "e":"24hrTicker",           // event type    "E":1657706425200,          // event time      "T":1657706425220,          // transaction time      "s":"BTC-220930-18000-C",   // Option symbol        "o":"2000",                 // 24-hour opening price    "h":"2020",                 // Highest price    "l":"2000",                 // Lowest price    "c":"2020",                 // latest price    "V":"1.42",                 // Trading volume(in contracts)      "A":"2841",                 // trade amount(in quote asset)       "P":"0.01",                 // price change percent    "p":"20",                   // price change     "Q":"0.01",                 // volume of last completed trade(in contracts)      "F":"27",                   // first trade ID    "L":"48",                   // last trade ID      "n":22,                     // number of trades    "bo":"2012",                // The best buy price    "ao":"2020",                // The best sell price    "bq":"4.9",                 // The best buy quantity    "aq":"0.03",                // The best sell quantity    "b":"0.1202",               // BuyImplied volatility       "a":"0.1318",               // SellImplied volatility    "d":"0.98911",              // delta    "t":"-0.16961",             // theta     "g":"0.00004",              // gamma      "v":"2.66584",              // vega    "vo":"0.10001",             // Implied volatility     "mp":"2003.5102",           // Mark price      "hl":"2023.511",            // Buy Maximum price    "ll":"1983.511",            // Sell Minimum price     "eep":"0"                   // Estimated strike price (return estimated strike price half hour before exercise)  }
```

## TRADE STREAMS

Trade Streams
==========

Stream Description[​](/docs/derivatives/option/websocket-market-streams/Trade-Streams#stream-description)
----------

The Trade Streams push raw trade information for specific symbol or underlying asset. E.g.[ETH@trade](wss://nbstream.binance.com/eoptions/stream?streams=ETH@trade)

Stream Name[​](/docs/derivatives/option/websocket-market-streams/Trade-Streams#stream-name)
----------

`<symbol>@trade` or `<underlyingAsset>@trade`

Update Speed[​](/docs/derivatives/option/websocket-market-streams/Trade-Streams#update-speed)
----------

**50ms**

Response Example[​](/docs/derivatives/option/websocket-market-streams/Trade-Streams#response-example)
----------

```
{    "e":"trade",                        // event type       "E":1591677941092,                  // event time       "s":"BTC-200630-9000-P",            // Option trading symbol       "t":1,                              // trade ID       "p":"1000",                         // price       "q":"-2",                           // quantity       "b":4611781675939004417,            // buy order ID       "a":4611781675939004418,            // sell order ID       "T":1591677567872,                  // trade completed time      "S":"-1"                            // direction       "X": "MARKET"                       // trade type enum, "MARKET" for Orderbook trading, "BLOCK" for Block trade	}
```

## PARTIAL BOOK DEPTH STREAMS

Partial Book Depth Streams
==========

Stream Description[​](/docs/derivatives/option/websocket-market-streams/Partial-Book-Depth-Streams#stream-description)
----------

Top **\<levels\>** bids and asks, Valid levels are **\<levels\>** are 10, 20, 50, 100.

Stream Name[​](/docs/derivatives/option/websocket-market-streams/Partial-Book-Depth-Streams#stream-name)
----------

`<symbol>@depth<levels>` OR `<symbol>@depth<levels>@100ms` OR `<symbol>@depth<levels>@1000ms`.

Update Speed[​](/docs/derivatives/option/websocket-market-streams/Partial-Book-Depth-Streams#update-speed)
----------

**100ms** or **1000ms**, **500ms**(default when update speed isn't used)

Response Example[​](/docs/derivatives/option/websocket-market-streams/Partial-Book-Depth-Streams#response-example)
----------

```
{    "e":"depth",                    // event type     "E":1591695934010,              // event time    "T":1591695934000,              // transaction time     "s":"BTC-200630-9000-P",        // Option symbol      "u":162,					 	            // update id in event    "pu":162,				 		            // same as update id in event        "b":[                           // Buy order         [          "200",                    // Price          "3",                      // quantity      ],      [          "101",          "1"      ],      [          "100",          "2"      ]    ],    "a":[                           // Sell order           [            "1000",            "89"        ]    ]}
```

