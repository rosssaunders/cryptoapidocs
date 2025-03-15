# Binance Options Public REST API Documentation

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

## MARKET DATA

Check Server Time
==========

API Description[​](/docs/derivatives/option/market-data#api-description)
----------

Test connectivity to the Rest API and get the current server time.

HTTP Request[​](/docs/derivatives/option/market-data#http-request)
----------

GET `/eapi/v1/time`

Request Weight[​](/docs/derivatives/option/market-data#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/option/market-data#request-parameters)
----------

NONE

Response Example[​](/docs/derivatives/option/market-data#response-example)
----------

```
{  "serverTime": 1499827319559}
```

## 24HR TICKER PRICE CHANGE STATISTICS

24hr Ticker Price Change Statistics
==========

API Description[​](/docs/derivatives/option/market-data/24hr-Ticker-Price-Change-Statistics#api-description)
----------

24 hour rolling window price change statistics.

HTTP Request[​](/docs/derivatives/option/market-data/24hr-Ticker-Price-Change-Statistics#http-request)
----------

GET `/eapi/v1/ticker`

Request Weight[​](/docs/derivatives/option/market-data/24hr-Ticker-Price-Change-Statistics#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/option/market-data/24hr-Ticker-Price-Change-Statistics#request-parameters)
----------

| Name | Type |Mandatory|               Description                |
|------|------|---------|------------------------------------------|
|symbol|STRING|   NO    |Option trading pair, e.g BTC-200730-9000-C|

Response Example[​](/docs/derivatives/option/market-data/24hr-Ticker-Price-Change-Statistics#response-example)
----------

```
[  {    "symbol": "BTC-200730-9000-C",    "priceChange": "-16.2038",        //24-hour price change    "priceChangePercent": "-0.0162",  //24-hour percent price change    "lastPrice": "1000",              //Last trade price    "lastQty": "1000",                //Last trade amount    "open": "1016.2038",              //24-hour open price    "high": "1016.2038",              //24-hour high    "low": "0",                       //24-hour low    "volume": "5",                    //Trading volume(contracts)    "amount": "1",                    //Trade amount(in quote asset)    "bidPrice":"999.34",              //The best buy price    "askPrice":"1000.23",             //The best sell price    "openTime": 1592317127349,        //Time the first trade occurred within the last 24 hours    "closeTime": 1592380593516,       //Time the last trade occurred within the last 24 hours         "firstTradeId": 1,                //First trade ID    "tradeCount": 5,                  //Number of trades    "strikePrice": "9000",            //Strike price    "exercisePrice": "3000.3356"      //return estimated settlement price one hour before exercise, return index price at other times  }]
```

## EXCHANGE INFORMATION

Exchange Information
==========

API Description[​](/docs/derivatives/option/market-data/Exchange-Information#api-description)
----------

Current exchange trading rules and symbol information

HTTP Request[​](/docs/derivatives/option/market-data/Exchange-Information#http-request)
----------

GET `/eapi/v1/exchangeInfo`

Request Weight[​](/docs/derivatives/option/market-data/Exchange-Information#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/option/market-data/Exchange-Information#request-parameters)
----------

NONE

Response Example[​](/docs/derivatives/option/market-data/Exchange-Information#response-example)
----------

```
{  "timezone": "UTC",                    // Time zone used by the server  "serverTime": 1592387337630,          // Current system time  "optionContracts": [                  // Option contract underlying asset info    {      "baseAsset": "BTC",               // Base currency      "quoteAsset": "USDT",             // Quotation asset      "underlying": "BTCUSDT",          // Name of the underlying asset of the option contract      "settleAsset": "USDT"             // Settlement currency    }  ],  "optionAssets": [                     // Option asset info    {      "name": "USDT"                    // Asset name    }  ],  "optionSymbols": [                    // Option trading pair info    {        "expiryDate": 1660521600000,    // expiry time        "filters": [            {                "filterType": "PRICE_FILTER",                "minPrice": "0.02",                "maxPrice": "80000.01",                "tickSize": "0.01"            },            {                "filterType": "LOT_SIZE",                "minQty": "0.01",                "maxQty": "100",                "stepSize": "0.01"            }        ],        "symbol": "BTC-220815-50000-C",   // Trading pair name        "side": "CALL",                   // Direction: CALL long, PUT short        "strikePrice": "50000",           // Strike price        "underlying": "BTCUSDT",          // Underlying asset of the contract        "unit": 1,                        // Contract unit, the quantity of the underlying asset represented by a single contract.        "makerFeeRate": "0.0002",         // maker commission rate        "takerFeeRate": "0.0002",         // taker commission rate        "minQty": "0.01",                 // Minimum order quantity        "maxQty": "100",                  // Maximum order quantity        "initialMargin": "0.15",          // Initial Magin Ratio        "maintenanceMargin": "0.075",     // Maintenance Margin Ratio        "minInitialMargin": "0.1",        // Min Initial Margin Ratio        "minMaintenanceMargin": "0.05",   // Min Maintenance Margin Ratio        "priceScale": 2,                  // price precision        "quantityScale": 2,               // quantity precision        "quoteAsset": "USDT"              // Quotation asset    }  ],  "rateLimits": [    {        "rateLimitType": "REQUEST_WEIGHT",        "interval": "MINUTE",        "intervalNum": 1,        "limit": 2400    },    {        "rateLimitType": "ORDERS",        "interval": "MINUTE",        "intervalNum": 1,        "limit": 1200    },    {        "rateLimitType": "ORDERS",        "interval": "SECOND",        "intervalNum": 10,        "limit": 300    }  ]}
```

## HISTORICAL EXERCISE RECORDS

Historical Exercise Records
==========

API Description[​](/docs/derivatives/option/market-data/Historical-Exercise-Records#api-description)
----------

Get historical exercise records.

* REALISTIC\_VALUE\_STRICKEN -\> Exercised
* EXTRINSIC\_VALUE\_EXPIRED -\> Expired OTM

HTTP Request[​](/docs/derivatives/option/market-data/Historical-Exercise-Records#http-request)
----------

GET `/eapi/v1/exerciseHistory`

Request Weight[​](/docs/derivatives/option/market-data/Historical-Exercise-Records#request-weight)
----------

**3**

Request Parameters[​](/docs/derivatives/option/market-data/Historical-Exercise-Records#request-parameters)
----------

|   Name   | Type |Mandatory|             Description             |
|----------|------|---------|-------------------------------------|
|underlying|STRING|   NO    |    Underlying index like BTCUSDT    |
|startTime | LONG |   NO    |             Start Time              |
| endTime  | LONG |   NO    |              End Time               |
|  limit   | INT  |   NO    |Number of records Default:100 Max:100|

Response Example[​](/docs/derivatives/option/market-data/Historical-Exercise-Records#response-example)
----------

```
[  {     "symbol": "BTC-220121-60000-P",            // symbol      "strikePrice": "60000",                    // strike price    "realStrikePrice": "38844.69652571",       // real strike price    "expiryDate": 1642752000000,               // Exercise time    "strikeResult": "REALISTIC_VALUE_STRICKEN" // strike result  }]
```

## OPEN INTEREST

Open Interest
==========

API Description[​](/docs/derivatives/option/market-data/Open-Interest#api-description)
----------

Get open interest for specific underlying asset on specific expiration date.

HTTP Request[​](/docs/derivatives/option/market-data/Open-Interest#http-request)
----------

GET `/eapi/v1/openInterest`

Request Weight[​](/docs/derivatives/option/market-data/Open-Interest#request-weight)
----------

**0**

Request Parameters[​](/docs/derivatives/option/market-data/Open-Interest#request-parameters)
----------

|     Name      | Type |Mandatory|         Description         |
|---------------|------|---------|-----------------------------|
|underlyingAsset|STRING|   YES   |underlying asset, e.g ETH/BTC|
|  expiration   |STRING|   YES   | expiration date, e.g 221225 |

Response Example[​](/docs/derivatives/option/market-data/Open-Interest#response-example)
----------

```
[    {        "symbol": "ETH-221119-1175-P",        "sumOpenInterest": "4.01",        "sumOpenInterestUsd": "4880.2985615624",        "timestamp": "1668754020000"    }]
```

## ORDER BOOK

Order Book
==========

API Description[​](/docs/derivatives/option/market-data/Order-Book#api-description)
----------

Check orderbook depth on specific symbol

HTTP Request[​](/docs/derivatives/option/market-data/Order-Book#http-request)
----------

GET `/eapi/v1/depth`

Request Weight[​](/docs/derivatives/option/market-data/Order-Book#request-weight)
----------

|    limit    |weight|
|-------------|------|
|5, 10, 20, 50|  2   |
|     100     |  5   |
|     500     |  10  |
|    1000     |  20  |

Request Parameters[​](/docs/derivatives/option/market-data/Order-Book#request-parameters)
----------

| Name | Type |Mandatory|                          Description                           |
|------|------|---------|----------------------------------------------------------------|
|symbol|STRING|   YES   |           Option trading pair, e.g BTC-200730-9000-C           |
|limit | INT  |   NO    |Default:100 Max:1000.Optional value:[10, 20, 50, 100, 500, 1000]|

Response Example[​](/docs/derivatives/option/market-data/Order-Book#response-example)
----------

```
{  "T": 1589436922972,   // transaction time  "u": 37461            // update id  "bids": [             // Buy order    [      "1000",            // Price      "0.9"              // Quantity    ]  ],  "asks": [              // Sell order    [      "1100",            // Price      "0.1"              // Quantity    ]  ]}  
```

## RECENT TRADES LIST

Recent Trades List
==========

API Description[​](/docs/derivatives/option/market-data/Recent-Trades-List#api-description)
----------

Get recent market trades

HTTP Request[​](/docs/derivatives/option/market-data/Recent-Trades-List#http-request)
----------

GET `/eapi/v1/trades`

Request Weight[​](/docs/derivatives/option/market-data/Recent-Trades-List#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/option/market-data/Recent-Trades-List#request-parameters)
----------

| Name | Type |Mandatory|               Description                |
|------|------|---------|------------------------------------------|
|symbol|STRING|   YES   |Option trading pair, e.g BTC-200730-9000-C|
|limit | INT  |   NO    |  Number of records Default:100 Max:500   |

Response Example[​](/docs/derivatives/option/market-data/Recent-Trades-List#response-example)
----------

```
[  {     "id":"1",             // TradeId    "symbol": "BTC-220722-19000-C",    "price": "1000",      // Completed trade price    "qty": "-0.1",        // Completed trade quantity    "quoteQty": "-100",   // Completed trade amount    "side": -1            // Completed trade direction（-1 Sell，1 Buy）    "time": 1592449455993,// Time   }]  
```

## RECENT BLOCK TRADE LIST

Recent Block Trades List
==========

API Description[​](/docs/derivatives/option/market-data/Recent-Block-Trade-List#api-description)
----------

Get recent block trades

HTTP Request[​](/docs/derivatives/option/market-data/Recent-Block-Trade-List#http-request)
----------

GET `/eapi/v1/blockTrades`

Request Weight[​](/docs/derivatives/option/market-data/Recent-Block-Trade-List#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/option/market-data/Recent-Block-Trade-List#request-parameters)
----------

| Name | Type |Mandatory|                 Description                |
|------|------|---------|--------------------------------------------|
|symbol|STRING|   NO    |Option trading pair, e.g. BTC-200730-9000-C |
|limit | INT  |   NO    |Number of records; Default: 100 and Max: 500|

Response Example[​](/docs/derivatives/option/market-data/Recent-Block-Trade-List#response-example)
----------

```
[	{		"id": 1125899906901081078,		"tradeId": 389,		"symbol": "ETH-250725-1200-P",		"price": "342.40",		"qty": "-2167.20",		"quoteQty": "-4.90",		"side": -1,		"time": 1733950676483	},	{		"id": 1125899906901080972,		"tradeId": 161,		"symbol": "XRP-250904-0.086-P",		"price": "3.0",		"qty": "-6.0",		"quoteQty": "-2.02",		"side": -1,		"time": 1733950488444	}]
```

## SYMBOL PRICE TICKER

Symbol Price Ticker
==========

API Description[​](/docs/derivatives/option/market-data/Symbol-Price-Ticker#api-description)
----------

Get spot index price for option underlying.

HTTP Request[​](/docs/derivatives/option/market-data/Symbol-Price-Ticker#http-request)
----------

GET `/eapi/v1/index`

Request Weight[​](/docs/derivatives/option/market-data/Symbol-Price-Ticker#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/option/market-data/Symbol-Price-Ticker#request-parameters)
----------

|   Name   | Type |Mandatory|                      Description                       |
|----------|------|---------|--------------------------------------------------------|
|underlying|STRING|   YES   |Spot pair（Option contract underlying asset, e.g BTCUSDT)|

Response Example[​](/docs/derivatives/option/market-data/Symbol-Price-Ticker#response-example)
----------

```
{   "time": 1656647305000,   "indexPrice": "9200" // Current spot index price}
```

## KLINE CANDLESTICK DATA

Kline/Candlestick Data
==========

API Description[​](/docs/derivatives/option/market-data/Kline-Candlestick-Data#api-description)
----------

Kline/candlestick bars for an option symbol.
Klines are uniquely identified by their open time.

HTTP Request[​](/docs/derivatives/option/market-data/Kline-Candlestick-Data#http-request)
----------

GET `/eapi/v1/klines`

Request Weight[​](/docs/derivatives/option/market-data/Kline-Candlestick-Data#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/option/market-data/Kline-Candlestick-Data#request-parameters)
----------

|  Name   | Type |Mandatory|               Description                |
|---------|------|---------|------------------------------------------|
| symbol  |STRING|   YES   |Option trading pair, e.g BTC-200730-9000-C|
|interval |STRING|   YES   |              Time interval               |
|startTime| LONG |   NO    |         Start Time 1592317127349         |
| endTime | LONG |   NO    |                 End Time                 |
|  limit  | INT  |   NO    |  Number of records Default:500 Max:1500  |

>
>
> * If startTime and endTime are not sent, the most recent klines are returned.
>
>

Response Example[​](/docs/derivatives/option/market-data/Kline-Candlestick-Data#response-example)
----------

```
[  {      "open": "950",              // Opening price      "high": "1100",             // Highest price      "low": "900",               // Lowest price      "close": "1000",            // Closing price (latest price if the current candle has not closed)      "volume": "100"             // Trading volume(contracts)             "amount": "2",              // Trading amount(in quote asset)      "interval": "5m",           // Candle type      "tradeCount": 10,           // Number of completed trades      "takerVolume": "100",       // Taker trading volume(contracts)            "takerAmount": "10000",     // Taker trade amount(in quote asset)      "openTime": 1499040000000,  // Opening time      "closeTime": 1499644799999, // Closing time  }]
```

## TEST CONNECTIVITY

Test Connectivity
==========

API Description[​](/docs/derivatives/option/market-data/Test-Connectivity#api-description)
----------

Test connectivity to the Rest API.

HTTP Request[​](/docs/derivatives/option/market-data/Test-Connectivity#http-request)
----------

GET `/eapi/v1/ping`

Request Weight[​](/docs/derivatives/option/market-data/Test-Connectivity#request-weight)
----------

**1**

Request Parameters[​](/docs/derivatives/option/market-data/Test-Connectivity#request-parameters)
----------

NONE

Response Example[​](/docs/derivatives/option/market-data/Test-Connectivity#response-example)
----------

```
{}
```

## OLD TRADES LOOKUP

Old Trades Lookup (MARKET\_DATA)
==========

API Description[​](/docs/derivatives/option/market-data/Old-Trades-Lookup#api-description)
----------

Get older market historical trades.

HTTP Request[​](/docs/derivatives/option/market-data/Old-Trades-Lookup#http-request)
----------

GET `/eapi/v1/historicalTrades`

Request Weight[​](/docs/derivatives/option/market-data/Old-Trades-Lookup#request-weight)
----------

20

Request Parameters[​](/docs/derivatives/option/market-data/Old-Trades-Lookup#request-parameters)
----------

| Name | Type |Mandatory|                                    Description                                    |
|------|------|---------|-----------------------------------------------------------------------------------|
|symbol|STRING|   YES   |                    Option trading pair, e.g BTC-200730-9000-C                     |
|fromId| LONG |   NO    |The UniqueId ID from which to return. The latest deal record is returned by default|
|limit | INT  |   NO    |                       Number of records Default:100 Max:500                       |

Response Example[​](/docs/derivatives/option/market-data/Old-Trades-Lookup#response-example)
----------

```
[  {    "id":"1",             // UniqueId    "tradeId": "159244329455993",    // TradeId    "price": "1000",      // Completed trade price    "qty": "-0.1",        // Completed trade Quantity    "quoteQty": "-100",   // Completed trade amount    "side": -1            // Completed trade direction（-1 Sell，1 Buy）    "time": 1592449455993,// Time  }]
```

## OPTION MARK PRICE

Option Mark Price
==========

API Description[​](/docs/derivatives/option/market-data/Option-Mark-Price#api-description)
----------

Option mark price and greek info.

HTTP Request[​](/docs/derivatives/option/market-data/Option-Mark-Price#http-request)
----------

GET `/eapi/v1/mark`

Request Weight[​](/docs/derivatives/option/market-data/Option-Mark-Price#request-weight)
----------

**5**

Request Parameters[​](/docs/derivatives/option/market-data/Option-Mark-Price#request-parameters)
----------

| Name | Type |Mandatory|               Description                |
|------|------|---------|------------------------------------------|
|symbol|STRING|   NO    |Option trading pair, e.g BTC-200730-9000-C|

Response Example[​](/docs/derivatives/option/market-data/Option-Mark-Price#response-example)
----------

```
[  {    "symbol": "BTC-200730-9000-C",    "markPrice": "1343.2883",       // Mark price    "bidIV": "1.40000077",          // Implied volatility Buy    "askIV": "1.50000153",          // Implied volatility Sell    "markIV": "1.45000000"          // Implied volatility mark      "delta": "0.55937056",          // delta    "theta": "3739.82509871",       // theta    "gamma": "0.00010969",          // gamma    "vega": "978.58874732",         // vega    "highPriceLimit": "1618.241",   // Current highest buy price    "lowPriceLimit": "1068.3356"    // Current lowest sell price    "riskFreeInterest": "0.1"       // risk free rate  }]
```

