# Binance Spot FIX API Documentation

## FILTERS

Filters
==========

Filters define trading rules on a symbol or an exchange.
Filters come in two forms: `symbol filters` and `exchange filters`.

Symbol filters[​](/docs/binance-spot-api-docs/filters#symbol-filters)
----------

### PRICE\_FILTER[​](/docs/binance-spot-api-docs/filters#price_filter) ###

The `PRICE_FILTER` defines the `price` rules for a symbol. There are 3 parts:

* `minPrice` defines the minimum `price`/`stopPrice` allowed; disabled on `minPrice` == 0.
* `maxPrice` defines the maximum `price`/`stopPrice` allowed; disabled on `maxPrice` == 0.
* `tickSize` defines the intervals that a `price`/`stopPrice` can be increased/decreased by; disabled on `tickSize` == 0.

Any of the above variables can be set to 0, which disables that rule in the `price filter`. In order to pass the `price filter`, the following must be true for `price`/`stopPrice` of the enabled rules:

* `price` \>= `minPrice`
* `price` \<= `maxPrice`
* `price` % `tickSize` == 0

**/exchangeInfo format:**

```
{  "filterType": "PRICE_FILTER",  "minPrice": "0.00000100",  "maxPrice": "100000.00000000",  "tickSize": "0.00000100"}
```

### PERCENT\_PRICE[​](/docs/binance-spot-api-docs/filters#percent_price) ###

The `PERCENT_PRICE` filter defines the valid range for the price based on the average of the previous trades.`avgPriceMins` is the number of minutes the average price is calculated over. 0 means the last price is used.

In order to pass the `percent price`, the following must be true for `price`:

* `price` \<= `weightedAveragePrice` \* `multiplierUp`
* `price` \>= `weightedAveragePrice` \* `multiplierDown`

**/exchangeInfo format:**

```
{  "filterType": "PERCENT_PRICE",  "multiplierUp": "1.3000",  "multiplierDown": "0.7000",  "avgPriceMins": 5}
```

### PERCENT\_PRICE\_BY\_SIDE[​](/docs/binance-spot-api-docs/filters#percent_price_by_side) ###

The `PERCENT_PRICE_BY_SIDE` filter defines the valid range for the price based on the average of the previous trades.  
`avgPriceMins` is the number of minutes the average price is calculated over. 0 means the last price is used.   
There is a different range depending on whether the order is placed on the `BUY` side or the `SELL` side.

Buy orders will succeed on this filter if:

* `Order price` \<= `weightedAveragePrice` \* `bidMultiplierUp`
* `Order price` \>= `weightedAveragePrice` \* `bidMultiplierDown`

Sell orders will succeed on this filter if:

* `Order Price` \<= `weightedAveragePrice` \* `askMultiplierUp`
* `Order Price` \>= `weightedAveragePrice` \* `askMultiplierDown`

**/exchangeInfo format:**

```
  {    "filterType": "PERCENT_PRICE_BY_SIDE",    "bidMultiplierUp": "1.2",    "bidMultiplierDown": "0.2",    "askMultiplierUp": "5",    "askMultiplierDown": "0.8",    "avgPriceMins": 1  }
```

### LOT\_SIZE[​](/docs/binance-spot-api-docs/filters#lot_size) ###

The `LOT_SIZE` filter defines the `quantity` (aka "lots" in auction terms) rules for a symbol. There are 3 parts:

* `minQty` defines the minimum `quantity`/`icebergQty` allowed.
* `maxQty` defines the maximum `quantity`/`icebergQty` allowed.
* `stepSize` defines the intervals that a `quantity`/`icebergQty` can be increased/decreased by.

In order to pass the `lot size`, the following must be true for `quantity`/`icebergQty`:

* `quantity` \>= `minQty`
* `quantity` \<= `maxQty`
* `quantity` % `stepSize` == 0

**/exchangeInfo format:**

```
{  "filterType": "LOT_SIZE",  "minQty": "0.00100000",  "maxQty": "100000.00000000",  "stepSize": "0.00100000"}
```

### MIN\_NOTIONAL[​](/docs/binance-spot-api-docs/filters#min_notional) ###

The `MIN_NOTIONAL` filter defines the minimum notional value allowed for an order on a symbol.
An order's notional value is the `price` \* `quantity`.`applyToMarket` determines whether or not the `MIN_NOTIONAL` filter will also be applied to `MARKET` orders.
Since `MARKET` orders have no price, the average price is used over the last `avgPriceMins` minutes.`avgPriceMins` is the number of minutes the average price is calculated over. 0 means the last price is used.

**/exchangeInfo format:**

```
{  "filterType": "MIN_NOTIONAL",  "minNotional": "0.00100000",  "applyToMarket": true,  "avgPriceMins": 5}
```

### NOTIONAL[​](/docs/binance-spot-api-docs/filters#notional) ###

The `NOTIONAL` filter defines the acceptable notional range allowed for an order on a symbol.   

`applyMinToMarket` determines whether the `minNotional` will be applied to `MARKET` orders.   
`applyMaxToMarket` determines whether the `maxNotional` will be applied to `MARKET` orders.

In order to pass this filter, the notional (`price * quantity`) has to pass the following conditions:

* `price * quantity` \<= `maxNotional`
* `price * quantity` \>= `minNotional`

For `MARKET` orders, the average price used over the last `avgPriceMins` minutes will be used for calculation.   
If the `avgPriceMins` is 0, then the last price will be used.

**/exchangeInfo format:**

```
{   "filterType": "NOTIONAL",   "minNotional": "10.00000000",   "applyMinToMarket": false,   "maxNotional": "10000.00000000",   "applyMaxToMarket": false,   "avgPriceMins": 5}
```

### ICEBERG\_PARTS[​](/docs/binance-spot-api-docs/filters#iceberg_parts) ###

The `ICEBERG_PARTS` filter defines the maximum parts an iceberg order can have. The number of `ICEBERG_PARTS` is defined as `CEIL(qty / icebergQty)`.

**/exchangeInfo format:**

```
{  "filterType": "ICEBERG_PARTS",  "limit": 10}
```

### MARKET\_LOT\_SIZE[​](/docs/binance-spot-api-docs/filters#market_lot_size) ###

The `MARKET_LOT_SIZE` filter defines the `quantity` (aka "lots" in auction terms) rules for `MARKET` orders on a symbol. There are 3 parts:

* `minQty` defines the minimum `quantity` allowed.
* `maxQty` defines the maximum `quantity` allowed.
* `stepSize` defines the intervals that a `quantity` can be increased/decreased by.

In order to pass the `market lot size`, the following must be true for `quantity`:

* `quantity` \>= `minQty`
* `quantity` \<= `maxQty`
* `quantity` % `stepSize` == 0

**/exchangeInfo format:**

```
{  "filterType": "MARKET_LOT_SIZE",  "minQty": "0.00100000",  "maxQty": "100000.00000000",  "stepSize": "0.00100000"}
```

### MAX\_NUM\_ORDERS[​](/docs/binance-spot-api-docs/filters#max_num_orders) ###

The `MAX_NUM_ORDERS` filter defines the maximum number of orders an account is allowed to have open on a symbol.
Note that both "algo" orders and normal orders are counted for this filter.

**/exchangeInfo format:**

```
{  "filterType": "MAX_NUM_ORDERS",  "maxNumOrders": 25}
```

### MAX\_NUM\_ALGO\_ORDERS[​](/docs/binance-spot-api-docs/filters#max_num_algo_orders) ###

The `MAX_NUM_ALGO_ORDERS` filter defines the maximum number of "algo" orders an account is allowed to have open on a symbol.
"Algo" orders are `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, and `TAKE_PROFIT_LIMIT` orders.

**/exchangeInfo format:**

```
{  "filterType": "MAX_NUM_ALGO_ORDERS",  "maxNumAlgoOrders": 5}
```

### MAX\_NUM\_ICEBERG\_ORDERS[​](/docs/binance-spot-api-docs/filters#max_num_iceberg_orders) ###

The `MAX_NUM_ICEBERG_ORDERS` filter defines the maximum number of `ICEBERG` orders an account is allowed to have open on a symbol.
An `ICEBERG` order is any order where the `icebergQty` is \> 0.

**/exchangeInfo format:**

```
{  "filterType": "MAX_NUM_ICEBERG_ORDERS",  "maxNumIcebergOrders": 5}
```

### MAX\_POSITION[​](/docs/binance-spot-api-docs/filters#max_position) ###

The `MAX_POSITION` filter defines the allowed maximum position an account can have on the base asset of a symbol. An account's position defined as the sum of the account's:

1. free balance of the base asset
2. locked balance of the base asset
3. sum of the qty of all open BUY orders

`BUY` orders will be rejected if the account's position is greater than the maximum position allowed.

If an order's `quantity` can cause the position to overflow, this will also fail the `MAX_POSITION` filter.

**/exchangeInfo format:**

```
{  "filterType":"MAX_POSITION",  "maxPosition":"10.00000000"}
```

### TRAILING\_DELTA[​](/docs/binance-spot-api-docs/filters#trailing_delta) ###

The `TRAILING_DELTA` filter defines the minimum and maximum value for the parameter `trailingDelta`.

In order for a trailing stop order to pass this filter, the following must be true:

For `STOP_LOSS BUY`, `STOP_LOSS_LIMIT_BUY`,`TAKE_PROFIT SELL` and `TAKE_PROFIT_LIMIT SELL` orders:

* `trailingDelta` \>= `minTrailingAboveDelta`
* `trailingDelta` \<= `maxTrailingAboveDelta`

For `STOP_LOSS SELL`, `STOP_LOSS_LIMIT SELL`, `TAKE_PROFIT BUY`, and `TAKE_PROFIT_LIMIT BUY` orders:

* `trailingDelta` \>= `minTrailingBelowDelta`
* `trailingDelta` \<= `maxTrailingBelowDelta`

**/exchangeInfo format:**

```
    {          "filterType": "TRAILING_DELTA",          "minTrailingAboveDelta": 10,          "maxTrailingAboveDelta": 2000,          "minTrailingBelowDelta": 10,          "maxTrailingBelowDelta": 2000   }
```

Exchange Filters[​](/docs/binance-spot-api-docs/filters#exchange-filters)
----------

### EXCHANGE\_MAX\_NUM\_ORDERS[​](/docs/binance-spot-api-docs/filters#exchange_max_num_orders) ###

The `EXCHANGE_MAX_NUM_ORDERS` filter defines the maximum number of orders an account is allowed to have open on the exchange.
Note that both "algo" orders and normal orders are counted for this filter.

**/exchangeInfo format:**

```
{  "filterType": "EXCHANGE_MAX_NUM_ORDERS",  "maxNumOrders": 1000}
```

### EXCHANGE\_MAX\_NUM\_ALGO\_ORDERS[​](/docs/binance-spot-api-docs/filters#exchange_max_num_algo_orders) ###

The `EXCHANGE_MAX_NUM_ALGO_ORDERS` filter defines the maximum number of "algo" orders an account is allowed to have open on the exchange.
"Algo" orders are `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, and `TAKE_PROFIT_LIMIT` orders.

**/exchangeInfo format:**

```
{  "filterType": "EXCHANGE_MAX_NUM_ALGO_ORDERS",  "maxNumAlgoOrders": 200}
```

### EXCHANGE\_MAX\_NUM\_ICEBERG\_ORDERS[​](/docs/binance-spot-api-docs/filters#exchange_max_num_iceberg_orders) ###

The `EXCHANGE_MAX_NUM_ICEBERG_ORDERS` filter defines the maximum number of iceberg orders an account is allowed to have open on the exchange.

**/exchangeInfo format:**

```
{  "filterType": "EXCHANGE_MAX_NUM_ICEBERG_ORDERS",  "maxNumIcebergOrders": 10000}
```

## ENUMS

ENUM Definitions
==========

This will apply for both Rest API and WebSocket API.

Symbol status (status)[​](/docs/binance-spot-api-docs/enums#symbol-status-status)
----------

* `PRE_TRADING`
* `TRADING`
* `POST_TRADING`
* `END_OF_DAY`
* `HALT`
* `AUCTION_MATCH`
* `BREAK`

[]()

Account and Symbol Permissions (permissions)[​](/docs/binance-spot-api-docs/enums#account-and-symbol-permissions-permissions)
----------

* `SPOT`
* `MARGIN`
* `LEVERAGED`
* `TRD_GRP_002`
* `TRD_GRP_003`
* `TRD_GRP_004`
* `TRD_GRP_005`
* `TRD_GRP_006`
* `TRD_GRP_007`
* `TRD_GRP_008`
* `TRD_GRP_009`
* `TRD_GRP_010`
* `TRD_GRP_011`
* `TRD_GRP_012`
* `TRD_GRP_013`
* `TRD_GRP_014`
* `TRD_GRP_015`
* `TRD_GRP_016`
* `TRD_GRP_017`
* `TRD_GRP_018`
* `TRD_GRP_019`
* `TRD_GRP_020`
* `TRD_GRP_021`
* `TRD_GRP_022`
* `TRD_GRP_023`
* `TRD_GRP_024`
* `TRD_GRP_025`

Order status (status)[​](/docs/binance-spot-api-docs/enums#order-status-status)
----------

|      Status      |                                                                                                                       Description                                                                                                                       |
|------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      `NEW`       |                                                                                                       The order has been accepted by the engine.                                                                                                        |
|  `PENDING_NEW`   |                                                                             The order is in a pending phase until the working order of an order list has been fully filled.                                                                             |
|`PARTIALLY_FILLED`|                                                                                                          A part of the order has been filled.                                                                                                           |
|     `FILLED`     |                                                                                                              The order has been completed.                                                                                                              |
|    `CANCELED`    |                                                                                                        The order has been canceled by the user.                                                                                                         |
| `PENDING_CANCEL` |                                                                                                                    Currently unused                                                                                                                     |
|    `REJECTED`    |                                                                                               The order was not accepted by the engine and not processed.                                                                                               |
|    `EXPIRED`     |The order was canceled according to the order type's rules (e.g. LIMIT FOK orders with no fill, LIMIT IOC or MARKET orders that partially fill)   <br/> or by the exchange, (e.g. orders canceled during liquidation, orders canceled during maintenance)|
|`EXPIRED_IN_MATCH`|                                     The order was expired by the exchange due to STP. (e.g. an order with `EXPIRE_TAKER` will match with existing orders on the book with the same account or same `tradeGroupId`)                                      |

Order List Status (listStatusType)[​](/docs/binance-spot-api-docs/enums#order-list-status-liststatustype)
----------

|    Status    |                                                 Description                                                  |
|--------------|--------------------------------------------------------------------------------------------------------------|
|  `RESPONSE`  |This is used when the ListStatus is responding to a failed action. (E.g. order list placement or cancellation)|
|`EXEC_STARTED`|                The order list has been placed or there is an update to the order list status.                |
|  `ALL_DONE`  |                     The order list has finished executing and thus is no longer active.                      |

Order List Order Status (listOrderStatus)[​](/docs/binance-spot-api-docs/enums#order-list-order-status-listorderstatus)
----------

|  Status   |                                           Description                                           |
|-----------|-------------------------------------------------------------------------------------------------|
|`EXECUTING`|      Either an order list has been placed or there is an update to the status of the list.      |
|`ALL_DONE` |                An order list has completed execution and thus no longer active.                 |
| `REJECT`  |The List Status is responding to a failed action either during order placement or order canceled.|

ContingencyType[​](/docs/binance-spot-api-docs/enums#contingencytype)
----------

* `OCO`
* `OTO`

[]()

AllocationType[​](/docs/binance-spot-api-docs/enums#allocationtype)
----------

* `SOR`

[]()

Order types (orderTypes, type)[​](/docs/binance-spot-api-docs/enums#order-types-ordertypes-type)
----------

* `LIMIT`
* `MARKET`
* `STOP_LOSS`
* `STOP_LOSS_LIMIT`
* `TAKE_PROFIT`
* `TAKE_PROFIT_LIMIT`
* `LIMIT_MAKER`

[]()

Order Response Type (newOrderRespType)[​](/docs/binance-spot-api-docs/enums#order-response-type-neworderresptype)
----------

* `ACK`
* `RESULT`
* `FULL`

Working Floor[​](/docs/binance-spot-api-docs/enums#working-floor)
----------

* `EXCHANGE`
* `SOR`

[]()

Order side (side)[​](/docs/binance-spot-api-docs/enums#order-side-side)
----------

* `BUY`
* `SELL`

[]()

Time in force (timeInForce)[​](/docs/binance-spot-api-docs/enums#time-in-force-timeinforce)
----------

This sets how long an order will be active before expiration.

|Status|                                                Description                                                |
|------|-----------------------------------------------------------------------------------------------------------|
|`GTC` |           Good Til Canceled   <br/> An order will be on the book unless the order is canceled.            |
|`IOC` |Immediate Or Cancel   <br/> An order will try to fill the order as much as it can before the order expires.|
|`FOK` |       Fill or Kill   <br/> An order will expire if the full order cannot be filled upon execution.        |

Rate limiters (rateLimitType)[​](/docs/binance-spot-api-docs/enums#rate-limiters-ratelimittype)
----------

* REQUEST\_WEIGHT

```
    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000    }
```

* ORDERS

```
    {      "rateLimitType": "ORDERS",      "interval": "SECOND",      "intervalNum": 1,      "limit": 10    }
```

* RAW\_REQUESTS

```
    {      "rateLimitType": "RAW_REQUESTS",      "interval": "MINUTE",      "intervalNum": 5,      "limit": 61000    }
```

Rate limit intervals (interval)[​](/docs/binance-spot-api-docs/enums#rate-limit-intervals-interval)
----------

* SECOND
* MINUTE
* DAY

[]()

STP Modes[​](/docs/binance-spot-api-docs/enums#stp-modes)
----------

* `NONE`
* `EXPIRE_MAKER`
* `EXPIRE_TAKER`
* `EXPIRE_BOTH`

## FIX API

FIX API
==========

>
>
> [!NOTE]
> This API can only be used with the SPOT Exchange.
>
>

General Information[​](/docs/binance-spot-api-docs/fix-api#general-information)
----------

FIX connections require TLS encryption. Please either use native TCP+TLS connection or set up a local proxy such as [stunnel](https://www.stunnel.org/) to handle TLS encryption.

**FIX sessions only support Ed25519 keys.**   

Please refer to [this tutorial](https://www.binance.com/en/support/faq/how-to-generate-an-ed25519-key-pair-to-send-api-requests-on-binance-6b9a63f1e3384cf48a2eedb82767a69a)on how to set up an Ed25519 key pair.

### FIX API order entry sessions[​](/docs/binance-spot-api-docs/fix-api#fix-api-order-entry-sessions) ###

* Endpoint is: `tcp+tls://fix-oe.binance.com:9000`
* Supports placing orders, canceling orders, and querying current limit usage.
* Supports receiving all of the account's [ExecutionReport`<8>`](/docs/binance-spot-api-docs/fix-api#executionreport) and [List Status`<N>`](/docs/binance-spot-api-docs/fix-api#liststatus).
* Only API keys with `FIX_API` are allowed to connect.
* QuickFIX Schema can be found [here](https://github.com/binance/binance-spot-api-docs/blob/master/fix/schemas/spot-fix-oe.xml).

### FIX API Drop Copy sessions[​](/docs/binance-spot-api-docs/fix-api#fix-api-drop-copy-sessions) ###

* Endpoint is: `tcp+tls://fix-dc.binance.com:9000`
* Supports receiving all of the account's [ExecutionReport`<8>`](/docs/binance-spot-api-docs/fix-api#executionreport) and [List Status`<N>`](/docs/binance-spot-api-docs/fix-api#liststatus).
* Only API keys with `FIX_API` or `FIX_API_READ_ONLY` are allowed to connect.
* QuickFIX Schema can be found [here](https://github.com/binance/binance-spot-api-docs/blob/master/fix/schemas/spot-fix-oe.xml).

### FIX Market Data sessions[​](/docs/binance-spot-api-docs/fix-api#fix-market-data-sessions) ###

* Endpoint is: `tcp+tls://fix-md.binance.com:9000`
* Supports market data streams and active instruments queries.
* Does not support placing or canceling orders.
* Only API keys with `FIX_API` or `FIX_API_READ_ONLY` are allowed to connect.
* QuickFIX Schema can be found [here](https://github.com/binance/binance-spot-api-docs/blob/master/fix/schemas/spot-fix-md.xml).

### FIX Connection Lifecycle[​](/docs/binance-spot-api-docs/fix-api#fix-connection-lifecycle) ###

* All FIX API sessions will remain open for as long as possible, on a best-effort basis.
* There is no minimum connection time guarantee; a server can enter maintenance at any time.
  * When a server enters maintenance, a [News `<B>`](/docs/binance-spot-api-docs/fix-api#news) message will be sent, prompting clients to reconnect. Upon receiving this message, a client is expected to establish a new session and close the old one **within 10 seconds**. If the client does not close the old session within the time frame, the server will proceed to log it out and close the session.

* After connecting, the client must send a Logon `<A>` request. For more information please refer to [How to sign a Logon request](/docs/binance-spot-api-docs/fix-api#signaturecomputation).
* The client should send a Logout `<5>` message to close the session before disconnecting. Failure to send the logout message will result in the session’s `SenderCompID (49)` being unusable for new session establishment for a duration of 2x the `HeartInt (108)` interval.
* The system allows negotiation of the `HeartInt (108)` value during the logon process. Accepted values range between 5 and 60 seconds.
  * If the server has not sent any messages within a `HeartInt (108)` interval, a [HeartBeat `<0>`](/docs/binance-spot-api-docs/fix-api#heartbeat) will be sent.
  * If the server has not received any messages within a `HeartInt (108)` interval, a [TestRequest `<1>`](/docs/binance-spot-api-docs/fix-api#testrequest) will be sent. If the server does not receive a HeartBeat `<0>` containing the expected `TestReqID (112)` from the client within `HeartInt (108)` seconds, the server will send a Logout `<5>` message and close the connection.
  * If the client has not received any messages within a `HeartInt (108)` interval, the client is responsible for sending a TestRequest `<1>` to ensure the connection is healthy. Upon receiving such a TestRequest `<1>`, the server will respond with a Heartbeat `<0>` containing the expected `TestReqID (112)`. If the client does not receive the server’s response within a `HeartInt (108)` interval, the client should close the session and connection and establish new ones.

### API Key Permissions[​](/docs/binance-spot-api-docs/fix-api#api-key-permissions) ###

To access the FIX API order entry sessions, your API key must be configured with the `FIX_API` permission.

To access the FIX Drop Copy sessions, your API key must be configured with either `FIX_API_READ_ONLY` or `FIX_API` permission.

To access the FIX Market Data sessions, your API key must be configured with either `FIX_API` or `FIX_API_READ_ONLY` permission.

**FIX sessions only support Ed25519 keys.**

Please refer to [this tutorial](https://www.binance.com/en/support/faq/how-to-generate-an-ed25519-key-pair-to-send-api-requests-on-binance-6b9a63f1e3384cf48a2eedb82767a69a)on how to set up an Ed25519 key pair.

[]()

### On message processing order[​](/docs/binance-spot-api-docs/fix-api#on-message-processing-order) ###

The `MessageHandling (25035)` field required in the initial [Logon`<A>`](/docs/binance-spot-api-docs/fix-api#logon-request) message controls whether the
messages may get reordered before they are processed by the engine.

* `UNORDERED(1)` messages from client are allowed to be sent to the engine out of order.
* `SEQUENTIAL(2)` messages from client are always sent to the engine in the `MsgSeqNum (34)` order.

>
>
> [!TIP]`UNORDERED(1)` should offer better performance when there are multiple messages in flight from the client to the server.
>
>

[]()

### Response Mode[​](/docs/binance-spot-api-docs/fix-api#response-mode) ###

By default, all concurrent order entry sessions receive all of the account's
successful [ExecutionReport`<8>`](/docs/binance-spot-api-docs/fix-api#executionreport) and [ListStatus`<N>`](/docs/binance-spot-api-docs/fix-api#liststatus) messages,
including those in response to orders placed from other FIX sessions and via non-FIX APIs.

Use the `ResponseMode (25036)` field in the initial [Logon`<A>`](/docs/binance-spot-api-docs/fix-api#logon-request) message
to change this behavior.

* `EVERYTHING(1)`: The default mode.
* `ONLY_ACKS(2)`: Receive only ACK messages whether operation succeeded or failed. Disables ExecutionReport push.

[]()

### How to sign Logon`<A>` request[​](/docs/binance-spot-api-docs/fix-api#how-to-sign-logona-request) ###

The [Logon`<A>`](/docs/binance-spot-api-docs/fix-api#logon-main) message authenticates your connection to the FIX API.
This must be the first message sent by the client.

* The `Username (553)` field is required to contain the API key.
* The `RawData (96)` field is required to contain a valid signature made with the API key.

The signature payload is a text string constructed by concatenating the values of the following fields in this exact order,
separated by the SOH character:

1. `MsgType (35)`
2. `SenderCompId (49)`
3. `TargetCompId (56)`
4. `MsgSeqNum (34)`
5. `SendingTime (52)`

Sign the payload using your private key.
Encode the signature with **base64**.
The resulting text string is the value of the `RawData (96)` field.

Here is a sample Python code implementing the signature algorithm:

```
import base64from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKeyfrom cryptography.hazmat.primitives.serialization import load_pem_private_keydef logon_raw_data(private_key: Ed25519PrivateKey,                   sender_comp_id: str,                   target_comp_id: str,                   msg_seq_num: str,                   sending_time: str):    """    Computes the value of RawData (96) field in Logon<A> message.    """    payload = chr(1).join([        'A',        sender_comp_id,        target_comp_id,        msg_seq_num,        sending_time,    ])    signature = private_key.sign(payload.encode('ASCII'))    return base64.b64encode(signature).decode('ASCII')with open('private_key.pem', 'rb') as f:    private_key = load_pem_private_key(data=f.read(),                                       password=None)raw_data = logon_raw_data(private_key,                          sender_comp_id='5JQmUOsm',                          target_comp_id='SPOT',                          msg_seq_num='1',                          sending_time='20240612-08:52:21.613')
```

The values presented below can be used to validate the correctness of the signature computation implementation:

|      Field      |         Value         |
|-----------------|-----------------------|
|  MsgType (35)   |          `A`          |
|SenderCompID (49)|       `EXAMPLE`       |
|TargetCompID (56)|        `SPOT`         |
| MsgSeqNum (34)  |          `1`          |
|SendingTime (52) |`20240627-11:17:25.223`|

The Ed25519 private key used in the example computation is shown below:

>
>
> [!CAUTION]
> The following secret key is provided solely for illustrative purposes. Do not use this key in any real-world application as it is not secure and may compromise your cryptographic implementation. Always generate your own unique and secure keys for actual use.
>
>

```
-----BEGIN PRIVATE KEY-----MC4CAQAwBQYDK2VwBCIEIIJEYWtGBrhACmb9Dvy+qa8WEf0lQOl1s4CLIAB9m89u-----END PRIVATE KEY-----
```

Computed signature:

```
4MHXelVVcpkdwuLbl6n73HQUXUf1dse2PCgT1DYqW9w8AVZ1RACFGM+5UdlGPrQHrgtS3CvsRURC1oj73j8gCA==
```

Resulting Logon `<A>` message:

```
8=FIX.4.4|9=247|35=A|34=1|49=EXAMPLE|52=20240627-11:17:25.223|56=SPOT|95=88|96=4MHXelVVcpkdwuLbl6n73HQUXUf1dse2PCgT1DYqW9w8AVZ1RACFGM+5UdlGPrQHrgtS3CvsRURC1oj73j8gCA==|98=0|108=30|141=Y|553=sBRXrJx2DsOraMXOaUovEhgVRcjOvCtQwnWj8VxkOh1xqboS02SPGfKi2h8spZJb|25035=2|10=227|
```

Limits[​](/docs/binance-spot-api-docs/fix-api#limits)
----------

### Message Limits[​](/docs/binance-spot-api-docs/fix-api#message-limits) ###

* Each connection has a limit on **how many messages can be sent to the exchange**.
* The message limit **does not count the messages sent in response to the client**.
* Breaching the message limit results in immediate [Logout `<5>`](/docs/binance-spot-api-docs/fix-api#logout) and disconnection.
* To understand current limits and usage, please send a [LimitQuery`<XLQ>`](/docs/binance-spot-api-docs/fix-api#limitquery) message.
  A [LimitResponse`<XLR>`](/docs/binance-spot-api-docs/fix-api#limitresponse) message will be sent in response, containing information about Order Rate
  Limits and Message Limits.
* FIX Order entry sessions have a limit of 10,000 messages every 10 seconds.
* FIX Drop Copy sessions have a limit of 60 messages every 60 seconds.
* FIX Market Data sessions have a limit of 2000 messages every 60 seconds.

[]()

### Connection Limits[​](/docs/binance-spot-api-docs/fix-api#connection-limits) ###

* Each Account has a limit on how many TCP connections can be established at the same time.
* The limit is reduced when the TCP connection is closed. If the reduction of connections is not immediate, please wait up to twice the value of `HeartBtInt (108)` for the change to take effect.
  For example, if the current value of `HeartBtInt` is 5, please wait up to 10 seconds.
* Upon breaching the limit a [Reject `<3>`](/docs/binance-spot-api-docs/fix-api#reject) will be sent containing information about the connection limit
  breach and the current limit.
* The limit is 5 concurrent TCP connections per account for the order entry sessions.
* The limit is 10 concurrent TCP connections per account for the Drop Copy sessions.
* The limit is 100 concurrent TCP connections per account for Market Data sessions.

### Unfilled Order Count[​](/docs/binance-spot-api-docs/fix-api#unfilled-order-count) ###

* To understand how many orders you have placed within a certain time interval, please send a [LimitQuery`<XLQ>`](/docs/binance-spot-api-docs/fix-api#limitquery) message.
  A [LimitResponse`<XLR>`](/docs/binance-spot-api-docs/fix-api#limitresponse) message will be sent in response, containing information about Unfilled Order Count and Message Limits.
* **Please note that if your orders are consistently filled by trades, you can continuously place orders on the API**. For more information, please see [Spot Unfilled Order Count Rules](/docs/binance-spot-api-docs/faqs/order_count_decrement).
* If you exceed the unfilled order count your message will be rejected, and information will be transferred back to you in a reject message specific to that endpoint.
* **The number of unfilled orders is tracked for each account.**

Error Handling[​](/docs/binance-spot-api-docs/fix-api#error-handling)
----------

Client messages that contain syntax errors, missing required fields, or refer to unknown symbols will be rejected by the server with a [Reject `<3>`](/docs/binance-spot-api-docs/fix-api#reject) message.

If a valid message cannot be processed and is rejected, an appropriate reject response will be sent.
Please refer to the individual message documentation for possible responses.

Please refer to the `Text (58)` and `ErrorCode (25016)` fields in responses for the reject reason.

The list of error codes can be found on the [Error codes](/docs/binance-spot-api-docs/errors) page.

Types[​](/docs/binance-spot-api-docs/fix-api#types)
----------

Only printable ASCII characters and SOH are supported.

|     Type     |                          Description                          |
|--------------|---------------------------------------------------------------|
|  `BOOLEAN`   |                       Enum: `Y` or `N`.                       |
|    `CHAR`    |                       Single character.                       |
|    `INT`     |                    Signed 64-bit integer.                     |
|   `LENGTH`   |                   Unsigned 64-bit integer.                    |
| `NUMINGROUP` |                   Unsigned 64-bit integer.                    |
|   `PRICE`    |Fixed-point number. Precision depends on the symbol definition.|
|    `QTY`     |Fixed-point number. Precision depends on the symbol definition.|
|   `SEQNUM`   |                   Unsigned 64-bit integer.                    |
|   `STRING`   |            Sequence of printable ASCII characters.            |
|`UTCTIMESTAMP`|             String representing datetime in UTC.              |

Supported `UTCTIMESTAMP` formats:

* `20011217-09:30:47` - seconds
* `20011217-09:30:47.123` - milliseconds
* `20011217-09:30:47.123456` - microseconds (always used in messages from the exchange)

Client order ID fields must conform to the regex `^[a-zA-Z0-9-_]{1,36}$`:

* `ClOrdID (11)`
* `OrigClOrdID (41)`
* `MDReqID (262)`
* `ClListID (25014)`
* `OrigClListID (25015)`
* `CancelClOrdID (25034)`

Message Components[​](/docs/binance-spot-api-docs/fix-api#message-components)
----------

>
>
> [!NOTE]
> In example messages, the `|` character is used to represent SOH character:
>
>

```
8=FIX.4.4|9=113|35=A|34=1|49=SPOT|52=20240612-08:52:21.636837|56=5JQmUOsm|98=0|108=30|25037=4392a152-3481-4499-921a-6d42c50702e2|10=051|
```

[]()

### Header[​](/docs/binance-spot-api-docs/fix-api#header) ###

Appears at the start of every message.

| Tag |    Name    |    Type    |Required|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
|-----|------------|------------|--------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|  8  |BeginString |   STRING   |   Y    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               Always `FIX.4.4`.   <br/><br/> Must be the first field the message.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
|  9  | BodyLength |   LENGTH   |   Y    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          Message length in bytes.   <br/><br/> Must be the second field in the message.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| 35  |  MsgType   |   STRING   |   Y    |Must be the third field in the message.   <br/><br/> Possible values:   <br/><br/>`0` - [HEARTBEAT](/docs/binance-spot-api-docs/fix-api#heartbeat)   <br/><br/>`1` - [TEST\_REQUEST](/docs/binance-spot-api-docs/fix-api#testrequest)   <br/><br/>`3` - [REJECT](/docs/binance-spot-api-docs/fix-api#reject)   <br/><br/>`5` - [LOGOUT](/docs/binance-spot-api-docs/fix-api#logout)   <br/><br/>`8` - [EXECUTION\_REPORT](/docs/binance-spot-api-docs/fix-api#executionreport)   <br/><br/>`9` - [ORDER\_CANCEL\_REJECT](/docs/binance-spot-api-docs/fix-api#ordercancelreject)   <br/><br/>`A` - [LOGON](/docs/binance-spot-api-docs/fix-api#logon-main)   <br/><br/>`D` - [NEW\_ORDER\_SINGLE](/docs/binance-spot-api-docs/fix-api#newordersingle)   <br/><br/>`E` - [NEW\_ORDER\_LIST](/docs/binance-spot-api-docs/fix-api#neworderlist)   <br/><br/>`F` - [ORDER\_CANCEL\_REQUEST](/docs/binance-spot-api-docs/fix-api#ordercancelrequest)   <br/><br/>`N` - [LIST\_STATUS](/docs/binance-spot-api-docs/fix-api#liststatus)   <br/><br/>`q` - [ORDER\_MASS\_CANCEL\_REQUEST](/docs/binance-spot-api-docs/fix-api#ordermasscancelrequest)   <br/><br/>`r` - [ORDER\_MASS\_CANCEL\_REPORT](/docs/binance-spot-api-docs/fix-api#ordermasscancelreport)   <br/><br/>`XCN` - [ORDER\_CANCEL\_REQUEST\_AND\_NEW\_ORDER\_SINGLE](/docs/binance-spot-api-docs/fix-api#ordercancelrequestandnewordersingle)   <br/><br/>`XLQ` - [LIMIT\_QUERY](/docs/binance-spot-api-docs/fix-api#limitquery)   <br/><br/>`XLR` - [LIMIT\_RESPONSE](/docs/binance-spot-api-docs/fix-api#limitresponse)   <br/><br/>`B` - [NEWS](/docs/binance-spot-api-docs/fix-api#news)   <br/><br/>`x`- [INSTRUMENT\_LIST\_REQUEST](/docs/binance-spot-api-docs/fix-api#instrumentlistrequest)   <br/><br/>`y` - [INSTRUMENT\_LIST](/docs/binance-spot-api-docs/fix-api#instrumentlist)   <br/><br/>`V` - [MARKET\_DATA\_REQUEST](/docs/binance-spot-api-docs/fix-api#marketdatarequest)   <br/><br/>`Y` - [MARKET\_DATA\_REQUEST\_REJECT](/docs/binance-spot-api-docs/fix-api#marketdatarequestreject)   <br/><br/>`W` - [MARKET\_DATA\_SNAPSHOT](/docs/binance-spot-api-docs/fix-api#marketdatasnapshot)   <br/><br/>`X` - [MARKET\_DATA\_INCREMENTAL\_REFRESH](/docs/binance-spot-api-docs/fix-api#marketdataincrementalrefresh)|
| 49  |SenderCompID|   STRING   |   Y    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             Must be unique across an account's active sessions.   <br/><br/> Must obey regex: `^[a-zA-Z0-9-_]{1,8}$`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| 56  |TargetCompID|   STRING   |   Y    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   A string identifying this TCP connection.  <br/><br/>On messages from client required to be set to `SPOT`.   <br/><br/>Must be unique across TCP connections.   <br/><br/> Must conform to the regex: `^[a-zA-Z0-9-_]{1,8}$`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| 34  | MsgSeqNum  |   SEQNUM   |   Y    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   Integer message sequence number.   <br/><br/> Values that will cause a gap will be rejected.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| 52  |SendingTime |UTCTIMESTAMP|   Y    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     Time of message transmission (always expressed in UTC).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|25000| RecvWindow |    INT     |   N    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              Number of milliseconds after `SendingTime (52)` the request is valid for.   <br/><br/> Defaults to `5000` milliseconds in [Logon`<A>`](/docs/binance-spot-api-docs/fix-api#logon-request) and has a max value of `60000` milliseconds.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

[]()

### Trailer[​](/docs/binance-spot-api-docs/fix-api#trailer) ###

Appears at the end of every message.

|Tag|  Name  | Type |Required|                                                                                                                                                                                  Description                                                                                                                                                                                   |
|---|--------|------|--------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|10 |CheckSum|STRING|   Y    |Always three-character numeric string, calculated by summing the ASCII values of each preceding character in the message, including start-of-header (SOH) characters.   <br/><br/> The resultant sum is divided by 256, with the remainder forming the CheckSum value.   <br/><br/> To maintain a fixed length, the CheckSum field is right-justified and zero-padded as needed.|

Administrative Messages[​](/docs/binance-spot-api-docs/fix-api#administrative-messages)
----------

[]()

### Heartbeat`<0>`[​](/docs/binance-spot-api-docs/fix-api#heartbeat0) ###

Sent by the server if there is no outgoing traffic during the heartbeat interval (`HeartBtInt (108)` in [Logon`<A>`](/docs/binance-spot-api-docs/fix-api#logon-main)).

Sent by the client to indicate that the session is healthy.

Sent by the client or the server in response to a [TestRequest`<1>`](/docs/binance-spot-api-docs/fix-api#testrequest) message.

|Tag|  Name   | Type |Required|                                              Description                                               |
|---|---------|------|--------|--------------------------------------------------------------------------------------------------------|
|112|TestReqID|STRING|   N    |When Heartbeat`<35>` is sent in response to TestRequest`<1>`, must mirror the value in TestRequest`<1>`.|

[]()

### TestRequest`<1>`[​](/docs/binance-spot-api-docs/fix-api#testrequest1) ###

Sent by the server if there is no incoming traffic during the heartbeat interval (`HeartBtInt (108)` in [Logon`<A>`](/docs/binance-spot-api-docs/fix-api#logon-main)).

Sent by the client to request a [Heartbeat`<0>`](/docs/binance-spot-api-docs/fix-api#heartbeat) response.

>
>
> [!NOTE]
> If the client does not respond to TestRequest`<1>` with Heartbeat`<0>` with a correct `TestReqID (112)` within timeout, the connection will be dropped.
>
>

|Tag|  Name   | Type |Required|                             Description                              |
|---|---------|------|--------|----------------------------------------------------------------------|
|112|TestReqID|STRING|   Y    |Arbitrary string that must be included in the Heartbeat`<0>` response.|

[]()

### Reject`<3>`[​](/docs/binance-spot-api-docs/fix-api#reject3) ###

Sent by the server in response to an invalid message that cannot be processed.

Sent by the server if a new connection cannot be accepted.
Please refer to [Connection Limits](/docs/binance-spot-api-docs/fix-api#connection-limits).

Please refer to the `Text (58)` and `ErrorCode (25016)` fields for the reject reason.

| Tag |       Name        | Type |Required|                                                                                                                                                                                                                                                                                                                                                                                                                Description                                                                                                                                                                                                                                                                                                                                                                                                                |
|-----|-------------------|------|--------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 45  |     RefSeqNum     | INT  |   N    |                                                                                                                                                                                                                                                                                                                                                                          The `MsgSeqNum (34)` of the rejected message that caused issuance of this Reject`<3>`.                                                                                                                                                                                                                                                                                                                                                                           |
| 371 |     RefTagID      | INT  |   N    |                                                                                                                                                                                                                                                                                                                                                                     When present, identifies the field that directly caused the issuance of this Reject`<3>` message.                                                                                                                                                                                                                                                                                                                                                                     |
| 372 |    RefMsgType     |STRING|   N    |                                                                                                                                                                                                                                                                                                                                                                           The `MsgType (35)` of the rejected message that caused issuance of this Reject`<3>`.                                                                                                                                                                                                                                                                                                                                                                            |
| 373 |SessionRejectReason| INT  |   N    |A reason for the reject, can be one of the values below.   <br/><br/> Usually accompanied by additional Text description   <br/><br/> Possible values:   <br/><br/>`0`- INVALID\_TAG\_NUMBER   <br/><br/>`1` - REQUIRED\_TAG\_MISSING   <br/><br/>`2` - TAG\_NOT\_DEFINED\_FOR\_THIS\_MESSAGE\_TYPE   <br/><br/>`3` - UNDEFINED\_TAG   <br/><br/>`5` - VALUE\_IS\_INCORRECT   <br/><br/>`6` - INCORRECT\_DATA\_FORMAT\_FOR\_VALUE   <br/><br/>`8` - SIGNATURE\_PROBLEM   <br/><br/>`10` - SENDINGTIME\_ACCURACY\_PROBLEM   <br/><br/>`12` - XML\_VALIDATION\_ERROR   <br/><br/>`13` - TAG\_APPEARS\_MORE\_THAN\_ONCE   <br/><br/>`14` - TAG\_SPECIFIED\_OUT\_OF\_REQUIRED\_ORDER   <br/><br/>`15` - REPEATING\_GROUP\_FIELDS\_OUT\_OF\_ORDER   <br/><br/>`16` - INCORRECT\_NUMINGROUP\_COUNT\_FOR\_REPEATING\_GROUP  <br/><br/>`99` - OTHER|
|25016|     ErrorCode     | INT  |   N    |                                                                                                                                                                                                                                                                                                                                                                                  API error code (see [Error Codes](/docs/binance-spot-api-docs/errors)).                                                                                                                                                                                                                                                                                                                                                                                  |
| 58  |       Text        |STRING|   N    |                                                                                                                                                                                                                                                                                                                                                                                                       Human-readable error message.                                                                                                                                                                                                                                                                                                                                                                                                       |

[]()

### Logon`<A>`[​](/docs/binance-spot-api-docs/fix-api#logona) ###

Sent by the client to authenticate the connection.
Logon`<A>` must be the first message sent by the client.

Sent by the server in response to a successful logon.

>
>
> [!NOTE]
> Logon`<A>` can only be sent once for the entirety of the session.
>
>

[]()

#### Logon Request[​](/docs/binance-spot-api-docs/fix-api#logon-request) ####

| Tag |     Name      | Type  |Required|                                                                                                Description                                                                                                |
|-----|---------------|-------|--------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 98  | EncryptMethod |  INT  |   Y    |                                                                                            Required to be `0`.                                                                                            |
| 108 |  HeartBtInt   |  INT  |   Y    |                                                                    Required to be within range [5, 60]. Heartbeat interval in seconds.                                                                    |
| 95  | RawDataLength |LENGTH |   Y    |                                                                 Length of the `RawData (96)` field that comes strictly after this field.                                                                  |
| 96  |    RawData    | DATA  |   Y    |                                                  Signature. [How to sign Logon`<A>` request](/docs/binance-spot-api-docs/fix-api#signaturecomputation).                                                   |
| 141 |ResetSeqNumFlag|BOOLEAN|   Y    |                                                                                            Required to be `Y`.                                                                                            |
| 553 |   Username    |STRING |   Y    |                                                                             API key. **Only Ed25519 API keys are supported.**                                                                             |
|25035|MessageHandling|  INT  |   Y    |Possible values:   <br/><br/>`1` - UNORDERED   <br/><br/>`2` - SEQUENTIAL   <br/><br/> Please refer to [On message order processing](/docs/binance-spot-api-docs/fix-api#orderedmode) for more information.|
|25036| ResponseMode  |  INT  |   N    |                                                            Please refer to [Response Mode](/docs/binance-spot-api-docs/fix-api#responsemode).                                                             |
|9406 | DropCopyFlag  |BOOLEAN|   N    |                                                                         Must be set to 'Y' when logging into Drop Copy sessions.                                                                          |

**Sample message:**

```
8=FIX.4.4|9=248|35=A|34=1|49=5JQmUOsm|52=20240612-08:52:21.613|56=SPOT|95=88|96=KhJLbZqADWknfTAcp0ZjyNz36Kxa4ffvpNf9nTIc+K5l35h+vA1vzDRvLAEQckyl6VDOwJ53NOBnmmRYxQvQBQ==|98=0|108=30|141=Y|553=W5rcOD30c0gT4jHK8oX5d5NbzWoa0k4SFVoTHIFNJVZ3NuRpYb6ZyJznj8THyx5d|25035=1|10=000|
```

[]()

#### Logon Response[​](/docs/binance-spot-api-docs/fix-api#logon-response) ####

| Tag |    Name     | Type |Required|               Description               |
|-----|-------------|------|--------|-----------------------------------------|
| 98  |EncryptMethod| INT  |   Y    |               Always `0`.               |
| 108 | HeartBtInt  | INT  |   Y    |  Mirrors value from the Logon request.  |
|25037|    UUID     |STRING|   Y    |UUID of the FIX API serving the requests.|

**Sample message:**

```
8=FIX.4.4|9=113|35=A|34=1|49=SPOT|52=20240612-08:52:21.636837|56=5JQmUOsm|98=0|108=30|25037=4392a152-3481-4499-921a-6d42c50702e2|10=051|
```

[]()

### Logout`<5>`[​](/docs/binance-spot-api-docs/fix-api#logout5) ###

Sent to initiate the process of closing the connection, and also when responding to Logout.

|Tag|Name| Type |Required|Description|
|---|----|------|--------|-----------|
|58 |Text|STRING|   N    |           |

**Sample messages:**

Logout Request

```
8=FIX.4.4|9=55|35=5|34=3|49=GhQHzrLR|52=20240611-09:44:25.543|56=SPOT|10=249|
```

Logout Response

```
8=FIX.4.4|9=84|35=5|34=4|49=SPOT|52=20240611-09:44:25.544001|56=GhQHzrLR|58=Logout acknowledgment.|10=212|
```

[]()

### News `<B>`[​](/docs/binance-spot-api-docs/fix-api#news-b) ###

Sent by the server when the connection is about to be closed.

Upon receiving this message, a client is expected to establish a new session and close the old one **within 10 seconds**. If the client does not close the old session within the time frame, the server will proceed to log it out and close the session.

|Tag|  Name  | Type |Required|Description|
|---|--------|------|--------|-----------|
|148|Headline|STRING|   Y    |           |

**Sample message:**

```
8=FIX.4.4|9=0000113|35=B|49=SPOT|56=OE|34=4|52=20240924-21:07:35.773537|148=Your connection is about to be closed. Please reconnect.|10=165|
```

### Resend Request `<2>`[​](/docs/binance-spot-api-docs/fix-api#resend-request-2) ###

Resend requests are currently not supported.

Application Messages[​](/docs/binance-spot-api-docs/fix-api#application-messages)
----------

### Order Entry Messages[​](/docs/binance-spot-api-docs/fix-api#order-entry-messages) ###

>
>
> [!NOTE]
> The messages below can only be used for the FIX Order Entry and FIX Drop Copy Sessions.
>
>

[]()

#### NewOrderSingle`<D>`[​](/docs/binance-spot-api-docs/fix-api#newordersingled) ####

Sent by the client to submit a new order for execution.

Please refer to [Supported Order Types](/docs/binance-spot-api-docs/fix-api#ordertype) for supported field combinations.

>
>
> [!NOTE]
> Many fields become required based on the order type.
> Please refer to [Supported Order Types](/docs/binance-spot-api-docs/fix-api#NewOrderSingle-required-fields).
>
>

| Tag |          Name          | Type  |Required|                                                                                                                                                                                                     Description                                                                                                                                                                                                     |
|-----|------------------------|-------|--------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 11  |        ClOrdID         |STRING |   Y    |                                                                                                                                                                                       `ClOrdID` to be assigned to the order.                                                                                                                                                                                        |
| 38  |        OrderQty        |  QTY  |   N    |                                                                                                                                                                                                Quantity of the order                                                                                                                                                                                                |
| 40  |        OrdType         | CHAR  |   Y    |                                                                      See the [table](/docs/binance-spot-api-docs/fix-api#ordertype) to understand supported order types and the required fields to use them.  <br/><br/>Possible values:   <br/><br/>`1` - MARKET   <br/><br/>`2` - LIMIT   <br/><br/>`3` - STOP   <br/><br/>`4` - STOP\_LIMIT                                                                      |
| 18  |        ExecInst        | CHAR  |   N    |                                                                                                                                                                           Possible values:   <br/><br/>`6` - PARTICIPATE\_DONT\_INITIATE                                                                                                                                                                            |
| 44  |         Price          | PRICE |   N    |                                                                                                                                                                                                 Price of the order                                                                                                                                                                                                  |
| 54  |          Side          | CHAR  |   Y    |                                                                                                                                                             Side of the order.  <br/><br/>Possible values:   <br/><br/>`1` - BUY   <br/><br/>`2` - SELL                                                                                                                                                             |
| 55  |         Symbol         |STRING |   Y    |                                                                                                                                                                                            Symbol to place the order on.                                                                                                                                                                                            |
| 59  |      TimeInForce       | CHAR  |   N    |                                                                                                                                           Possible values:   <br/><br/>`1` - GOOD\_TILL\_CANCEL   <br/><br/>`3` - IMMEDIATE\_OR\_CANCEL   <br/><br/>`4` - FILL\_OR\_KILL                                                                                                                                            |
| 111 |        MaxFloor        |  QTY  |   N    |                                                                                                                                                               Used for iceberg orders, this specifies the visible quantity of the order on the book.                                                                                                                                                                |
| 152 |      CashOrderQty      |  QTY  |   N    |                                                                                                                                                                Quantity of the order specified in the quote asset units, for reverse market orders.                                                                                                                                                                 |
| 847 |     TargetStrategy     |  INT  |   N    |                                                                                                                                                                                      The value cannot be less than `1000000`.                                                                                                                                                                                       |
|7940 |       StrategyID       |  INT  |   N    |                                                                                                                                                                                                                                                                                                                                                                                                                     |
|25001|SelfTradePreventionMode | CHAR  |   N    |                                                                                                                                       Possible values:   <br/><br/>`1` - NONE   <br/><br/>`2` - EXPIRE\_TAKER   <br/><br/>`3` - EXPIRE\_MAKER   <br/><br/>`4` - EXPIRE\_BOTH                                                                                                                                        |
|1100 |      TriggerType       | CHAR  |   N    |                                                                                                                                                                                       Possible values: `4` - PRICE\_MOVEMENT                                                                                                                                                                                        |
|1101 |     TriggerAction      | CHAR  |   N    |                                                                                                                                                                                     Possible values:   <br/><br/>`1` - ACTIVATE                                                                                                                                                                                     |
|1102 |      TriggerPrice      | PRICE |   N    |                                                                                                                                                         Activation price for contingent orders. See [table](/docs/binance-spot-api-docs/fix-api#ordertype)                                                                                                                                                          |
|1107 |    TriggerPriceType    | CHAR  |   N    |                                                                                                                                                                                   Possible values:   <br/><br/>`2` - LAST\_TRADE                                                                                                                                                                                    |
|1109 | TriggerPriceDirection  | CHAR  |   N    |Used to differentiate between StopLoss and TakeProfit orders. See [table](/docs/binance-spot-api-docs/fix-api#ordertype).  <br/><br/>Possible values:   <br/><br/>`U` - TRIGGER\_IF\_THE\_PRICE\_OF\_THE\_SPECIFIED\_TYPE\_GOES\_UP\_TO\_OR\_THROUGH\_THE\_SPECIFIED\_TRIGGER\_PRICE   <br/><br/>`D` - TRIGGER\_IF\_THE\_PRICE\_OF\_THE\_SPECIFIED\_TYPE\_GOES\_DOWN\_TO\_OR\_THROUGH\_THE\_SPECIFIED\_TRIGGER\_PRICE|
|25009|TriggerTrailingDeltaBips|  INT  |   N    |                                                                                                                                                                                         Provide to create trailing orders.                                                                                                                                                                                          |
|25032|          SOR           |BOOLEAN|   N    |                                                                                                                                                                                       Whether to activate SOR for this order.                                                                                                                                                                                       |

**Sample message:**

```
8=FIX.4.4|9=114|35=D|34=2|49=qNXO12fH|52=20240611-09:01:46.228|56=SPOT|11=1718096506197867067|38=5|40=2|44=10|54=1|55=LTCBNB|59=4|10=016|
```

**Response:**

* [ExecutionReport`<8>`](/docs/binance-spot-api-docs/fix-api#executionreport) with `ExecType (150)` value `NEW (0)` if the order was accepted.
* [ExecutionReport`<8>`](/docs/binance-spot-api-docs/fix-api#executionreport) with `ExecType (150)` value `REJECTED (8)` if the order was rejected.
* [Reject`<3>`](/docs/binance-spot-api-docs/fix-api#reject) if the message is rejected.

[]()

##### Supported Order Types[​](/docs/binance-spot-api-docs/fix-api#supported-order-types) #####

|             Order name              | Binance OrderType |   Side    |       required field values       |required fields with user values|
|-------------------------------------|-------------------|-----------|-----------------------------------|--------------------------------|
|            Market order             |     `MARKET`      |BUY or SELL|              `40=1|`              |                                |
|             Limit order             |      `LIMIT`      |BUY or SELL|              `40=2|`              |                                |
|          Limit maker order          |   `LIMIT_MAKER`   |BUY or SELL|           `40=2|18=6|`            |                                |
|         Buy stop loss order         |    `STOP_LOSS`    |    BUY    |`40=3|1100=4|1101=1|1107=2|1109=U|`|              1102              |
|    Buy trailing stop loss order     |    `STOP_LOSS`    |    BUY    |`40=3|1100=4|1101=1|1107=2|1109=U|`|           1102,25009           |
|      Buy stop loss limit order      | `STOP_LOSS_LIMIT` |    BUY    |`40=4|1100=4|1101=1|1107=2|1109=U|`|              1102              |
| Buy trailing stop loss limit order  | `STOP_LOSS_LIMIT` |    BUY    |`40=4|1100=4|1101=1|1107=2|1109=U|`|           1102,25009           |
|        Sell stop loss order         |    `STOP_LOSS`    |   SELL    |`40=3|1100=4|1101=1|1107=2|1109=D|`|              1102              |
|    Sell trailing stop loss order    |    `STOP_LOSS`    |   SELL    |`40=3|1100=4|1101=1|1107=2|1109=D|`|           1102,25009           |
|     Sell stop loss limit order      | `STOP_LOSS_LIMIT` |   SELL    |`40=4|1100=4|1101=1|1107=2|1109=D|`|              1102              |
| Sell trailing stop loss limit order | `STOP_LOSS_LIMIT` |   SELL    |`40=4|1100=4|1101=1|1107=2|1109=D|`|           1102,25009           |
|        Buy take profit order        |   `TAKE_PROFIT`   |    BUY    |`40=3|1100=4|1101=1|1107=2|1109=D|`|              1102              |
|   Buy trailing take profit order    |   `TAKE_PROFIT`   |    BUY    |`40=3|1100=4|1101=1|1107=2|1109=D|`|           1102,25009           |
|   Buy trailing take profit order    |   `TAKE_PROFIT`   |    BUY    |   `40=3|1100=4|1101=1|1107=2|`    |             25009              |
|        Buy take profit order        |`TAKE_PROFIT_LIMIT`|    BUY    |`40=4|1100=4|1101=1|1107=2|1109=D|`|              1102              |
|Buy trailing take profit limit order |`TAKE_PROFIT_LIMIT`|    BUY    |`40=4|1100=4|1101=1|1107=2|1109=D|`|           1102,25009           |
|Buy trailing take profit limit order |`TAKE_PROFIT_LIMIT`|    BUY    |   `40=4|1100=4|1101=1|1107=2|`    |             25009              |
|       Sell take profit order        |   `TAKE_PROFIT`   |   SELL    |`40=3|1100=4|1101=1|1107=2|1109=U|`|              1102              |
|   Sell trailing take profit order   |   `TAKE_PROFIT`   |   SELL    |`40=3|1100=4|1101=1|1107=2|1109=U|`|           1102,25009           |
|   Sell trailing take profit order   |   `TAKE_PROFIT`   |   SELL    |   `40=3|1100=4|1101=1|1107=2|`    |             25009              |
|    Sell take profit limit order     |`TAKE_PROFIT_LIMIT`|   SELL    |`40=4|1100=4|1101=1|1107=2|1109=U|`|              1102              |
|Sell trailing take profit limit order|`TAKE_PROFIT_LIMIT`|   SELL    |`40=4|1100=4|1101=1|1107=2|1109=U|`|           1102,25009           |
|Sell trailing take profit limit order|`TAKE_PROFIT_LIMIT`|   SELL    |   `40=4|1100=4|1101=1|1107=2|`    |             25009              |

[]()

Required fields based on Binance OrderType:

| Binance OrderType |Additional mandatory parameters|                                                                                                                                                                                                                                                                                                                                                 Additional Information                                                                                                                                                                                                                                                                                                                                                  |
|-------------------|-------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      `LIMIT`      |          38, 44, 59           |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
|     `MARKET`      |           38 OR 152           |`MARKET` orders using the `OrderQty (38)` field specifies the amount of the `base asset` the user wants to buy or sell at the market price.   <br/> E.g. `MARKET` order on BTCUSDT will specify how much BTC the user is buying or selling.   <br/><br/>`MARKET` orders using `quoteOrderQty` specifies the amount the user wants to spend (when buying) or receive (when selling) the `quote` asset; the correct `quantity` will be determined based on the market liquidity and `quoteOrderQty`.   <br/> E.g. Using the symbol BTCUSDT:   <br/>`BUY` side, the order will buy as many BTC as `quoteOrderQty` USDT can.   <br/>`SELL` side, the order will sell as much BTC needed to receive `CashOrderQty (152)` USDT.|
|    `STOP_LOSS`    |       38, 1102 or 25009       |                                                                                                                                                                                                                                                                                 This will execute a `MARKET` order when the conditions are met. (e.g. `TriggerPrice (1102)` is met or `TriggerTrailingDeltaBips (25009)` is activated)                                                                                                                                                                                                                                                                                  |
| `STOP_LOSS_LIMIT` |   38, 44, 59, 1102 or 25009   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
|   `TAKE_PROFIT`   |       38, 1102 or 25009       |                                                                                                                                                                                                                                                                                 This will execute a `MARKET` order when the conditions are met. (e.g. `TriggerPrice (1102)` is met or `TriggerTrailingDeltaBips (25009)` is activated)                                                                                                                                                                                                                                                                                  |
|`TAKE_PROFIT_LIMIT`|   38, 44, 59, 1102 or 25009   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
|   `LIMIT_MAKER`   |            38, 44             |                                                                                                                                                                                                                                                                                 This is a `LIMIT` order that will be rejected if the order immediately matches and trades as a taker.   <br/> This is also known as a POST-ONLY order.                                                                                                                                                                                                                                                                                  |

[]()

#### ExecutionReport`<8>`[​](/docs/binance-spot-api-docs/fix-api#executionreport8) ####

Sent by the server whenever an order state changes.

>
>
> [!NOTE]
>
>
>
> * By default, ExecutionReport`<8>` is sent for all orders of an account, including those submitted in different connections. Please see [Response Mode](/docs/binance-spot-api-docs/fix-api#responsemode) for other behavior options.
> * FIX API should give better performance for ExecutionReport`<8>` push.
>
>

|  Tag  |          Name          |    Type    |Required|                                                                                                                                                                                                     Description                                                                                                                                                                                                     |
|-------|------------------------|------------|--------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|  17   |         ExecID         |   STRING   |   N    |                                                                                                                                                                                             Omitted on rejected orders.                                                                                                                                                                                             |
|  11   |        ClOrdID         |   STRING   |   N    |                                                                                                                                                                                  `ClOrdID` of the list as assigned on the request.                                                                                                                                                                                  |
|  41   |      OrigClOrdID       |   STRING   |   N    |                                                                                                                                                                                          Original `ClOrdID` of the order.                                                                                                                                                                                           |
|  37   |        OrderID         |    INT     |   N    |                                                                                                                                                                                                Assigned by exchange.                                                                                                                                                                                                |
|  38   |        OrderQty        |    QTY     |   N    |                                                                                                                                                                                               Quantity of the order.                                                                                                                                                                                                |
|  40   |        OrdType         |    CHAR    |   Y    |                                                                                                                                            Possible values:   <br/><br/>`1` - MARKET   <br/><br/>`2` - LIMIT   <br/><br/>`3` - STOP\_LOSS   <br/><br/>`4` - STOP\_LIMIT                                                                                                                                             |
|  54   |          Side          |    CHAR    |   Y    |                                                                                                                                                                            Possible values:   <br/><br/>`1` - BUY   <br/><br/>`2` - SELL                                                                                                                                                                            |
|  55   |         Symbol         |   STRING   |   Y    |                                                                                                                                                                                                Symbol of the order.                                                                                                                                                                                                 |
|  18   |        ExecInst        |    CHAR    |   N    |                                                                                                                                                                           Possible values:   <br/><br/>`6` - PARTICIPATE\_DONT\_INITIATE                                                                                                                                                                            |
|  44   |         Price          |   PRICE    |   N    |                                                                                                                                                                                                 Price of the order.                                                                                                                                                                                                 |
|  59   |      TimeInForce       |    CHAR    |   N    |                                                                                                                                           Possible values:   <br/><br/>`1` - GOOD\_TILL\_CANCEL   <br/><br/>`3` - IMMEDIATE\_OR\_CANCEL   <br/><br/>`4` - FILL\_OR\_KILL                                                                                                                                            |
|  60   |      TransactTime      |UTCTIMESTAMP|   N    |                                                                                                                                                                                         Timestamp when this event occurred.                                                                                                                                                                                         |
| 25018 |   OrderCreationTime    |    INT     |   N    |                                                                                                                                                                                                                                                                                                                                                                                                                     |
|  111  |        MaxFloor        |    QTY     |   N    |                                                                                                                                                                                             Appears on iceberg orders.                                                                                                                                                                                              |
|  66   |         ListID         |   STRING   |   N    |                                                                                                                                                                                               Appears on list orders.                                                                                                                                                                                               |
|  152  |      CashOrderQty      |    QTY     |   N    |                                                                                                                                                                                    OrderQty specified in the quote asset units.                                                                                                                                                                                     |
|  847  |     TargetStrategy     |    INT     |   N    |                                                                                                                                                                              `TargetStrategy (847)` from the order placement request.                                                                                                                                                                               |
| 7940  |       StrategyID       |    INT     |   N    |                                                                                                                                                                                `StrategyID (7940)` from the order placement request.                                                                                                                                                                                |
| 25001 |SelfTradePreventionMode |    CHAR    |   N    |                                                                                                                                       Possible values:   <br/><br/>`1` - NONE   <br/><br/>`2` - EXPIRE\_TAKER   <br/><br/>`3` - EXPIRE\_MAKER   <br/><br/>`4` - EXPIRE\_BOTH                                                                                                                                        |
|  150  |        ExecType        |    CHAR    |   Y    |                                                    **Note:** Field `PreventedMatchID(25024)` will be present if order has expired due to `SelfTradePreventionMode(25013)`   <br/><br/> Possible values:   <br/><br/>`0` - NEW   <br/><br/>`4` - CANCELED   <br/><br/>`5` - REPLACED   <br/><br/>`8` - REJECTED   <br/><br/>`F` - TRADE   <br/><br/>`C` - EXPIRED                                                    |
|  14   |         CumQty         |    QTY     |   Y    |                                                                                                                                                                                  Total number of base asset traded on this order.                                                                                                                                                                                   |
|  151  |       LeavesQty        |    QTY     |   N    |                                                                                                                                                                                      Quantity remaining for further execution.                                                                                                                                                                                      |
| 25017 |      CumQuoteQty       |    QTY     |   N    |                                                                                                                                                                                  Total number of quote asset traded on this order.                                                                                                                                                                                  |
| 1057  |   AggressorIndicator   |  BOOLEAN   |   N    |                                                                                                                                                        Appears on trade execution reports.   <br/><br/>Indicates whether the order was a taker in the trade.                                                                                                                                                        |
| 1003  |        TradeID         |   STRING   |   N    |                                                                                                                                                                                         Appears on trade execution reports.                                                                                                                                                                                         |
|  31   |         LastPx         |   PRICE    |   N    |                                                                                                                                                                                          The price of the last execution.                                                                                                                                                                                           |
|  32   |        LastQty         |    QTY     |   Y    |                                                                                                                                                                                         The quantity of the last execution.                                                                                                                                                                                         |
|  39   |       OrdStatus        |    CHAR    |   Y    |                                Possible values:   <br/><br/>`0` - NEW   <br/><br/>`1` - PARTIALLY\_FILLED   <br/><br/>`2` - FILLED   <br/><br/>`4` - CANCELED `6` - PENDING\_CANCEL  <br/><br/>`8` - REJECTED   <br/><br/>`A` - PENDING\_NEW   <br/><br/>`C` - EXPIRED   <br/><br/> Note that FIX does not support `EXPIRED_IN_MATCH` status, and get converted to `EXPIRED` in FIX.                                |
|  70   |        AllocID         |    INT     |   N    |                                                                                                                                                                                     Allocation ID as assigned by the exchange.                                                                                                                                                                                      |
|  574  |       MatchType        |    INT     |   N    |                                                                                                                                                              Possible values:  <br/><br/>`1` - ONE\_PARTY\_TRADE\_REPORT  <br/><br/>`4` - AUTO\_MATCH                                                                                                                                                               |
| 25021 |      WorkingFloor      |    INT     |   N    |                                                                                                                                                                                Appears for orders that potentially have allocations.                                                                                                                                                                                |
| 25022 |      TrailingTime      |UTCTIMESTAMP|   N    |                                                                                                                                                                                       Appears only for trailing stop orders.                                                                                                                                                                                        |
|  636  |    WorkingIndicator    |  BOOLEAN   |   N    |                                                                                                                                                                                    Set to `Y` when this order enters order book.                                                                                                                                                                                    |
| 25023 |      WorkingTime       |UTCTIMESTAMP|   N    |                                                                                                                                                                                     When this order appeared on the order book.                                                                                                                                                                                     |
| 25024 |    PreventedMatchID    |    INT     |   N    |                                                                                                                                                                                  Appears only for orders that expired due to STP.                                                                                                                                                                                   |
| 25025 |PreventedExecutionPrice |   PRICE    |   N    |                                                                                                                                                                                  Appears only for orders that expired due to STP.                                                                                                                                                                                   |
| 25026 | PreventedExecutionQty  |    QTY     |   N    |                                                                                                                                                                                  Appears only for orders that expired due to STP.                                                                                                                                                                                   |
| 25027 |      TradeGroupID      |    INT     |   N    |                                                                                                                                                                                  Appears only for orders that expired due to STP.                                                                                                                                                                                   |
| 25028 |     CounterSymbol      |   STRING   |   N    |                                                                                                                                                                                  Appears only for orders that expired due to STP.                                                                                                                                                                                   |
| 25029 |     CounterOrderID     |    INT     |   N    |                                                                                                                                                                                  Appears only for orders that expired due to STP.                                                                                                                                                                                   |
| 25030 |      PreventedQty      |    QTY     |   N    |                                                                                                                                                                                  Appears only for orders that expired due to STP.                                                                                                                                                                                   |
| 25031 |    LastPreventedQty    |    QTY     |   N    |                                                                                                                                                                                  Appears only for orders that expired due to STP.                                                                                                                                                                                   |
| 25032 |          SOR           |  BOOLEAN   |   N    |                                                                                                                                                                                          Appears for orders that used SOR.                                                                                                                                                                                          |
| 25016 |       ErrorCode        |    INT     |   N    |                                                                                                                                                                       API error code (see [Error Codes](/docs/binance-spot-api-docs/errors)).                                                                                                                                                                       |
|  58   |          Text          |   STRING   |   N    |                                                                                                                                                                                            Human-readable error message.                                                                                                                                                                                            |
|  136  |       NoMiscFees       | NUMINGROUP |   N    |                                                                                                                                                                                  Number of repeating groups of miscellaneous fees.                                                                                                                                                                                  |
|\=\>137|       MiscFeeAmt       |    QTY     |   Y    |                                                                                                                                                                               Amount of fees denominated in `MiscFeeCurr(138)` asset                                                                                                                                                                                |
|\=\>138|      MiscFeeCurr       |   STRING   |   Y    |                                                                                                                                                                                           Currency of miscellaneous fee.                                                                                                                                                                                            |
|\=\>139|      MiscFeeType       |    INT     |   Y    |                                                                                                                                                                                  Possible values:   <br/><br/>`4` - EXCHANGE\_FEES                                                                                                                                                                                  |
| 1100  |      TriggerType       |    CHAR    |   N    |                                                                                                                                                                                 Possible values:   <br/><br/>`4` - PRICE\_MOVEMENT                                                                                                                                                                                  |
| 1101  |     TriggerAction      |    CHAR    |   N    |                                                                                                                                                                                     Possible values:   <br/><br/>`1` - ACTIVATE                                                                                                                                                                                     |
| 1102  |      TriggerPrice      |   PRICE    |   N    |                                                                                                                                                         Activation price for contingent orders. See [table](/docs/binance-spot-api-docs/fix-api#ordertype)                                                                                                                                                          |
| 1107  |    TriggerPriceType    |    CHAR    |   N    |                                                                                                                                                                                   Possible values:   <br/><br/>`2` - LAST\_TRADE                                                                                                                                                                                    |
| 1109  | TriggerPriceDirection  |    CHAR    |   N    |Used to differentiate between StopLoss and TakeProfit orders. See [table](/docs/binance-spot-api-docs/fix-api#ordertype).  <br/><br/>Possible values:   <br/><br/>`U` - TRIGGER\_IF\_THE\_PRICE\_OF\_THE\_SPECIFIED\_TYPE\_GOES\_UP\_TO\_OR\_THROUGH\_THE\_SPECIFIED\_TRIGGER\_PRICE   <br/><br/>`D` - TRIGGER\_IF\_THE\_PRICE\_OF\_THE\_SPECIFIED\_TYPE\_GOES\_DOWN\_TO\_OR\_THROUGH\_THE\_SPECIFIED\_TRIGGER\_PRICE|
| 25009 |TriggerTrailingDeltaBips|    INT     |   N    |                                                                                                                                                                                       Appears only for trailing stop orders.                                                                                                                                                                                        |

**Sample message:**

```
8=FIX.4.4|9=330|35=8|34=2|49=SPOT|52=20240611-09:01:46.228950|56=qNXO12fH|11=1718096506197867067|14=0.00000000|17=144|32=0.00000000|37=76|38=5.00000000|39=0|40=2|44=10.00000000|54=1|55=LTCBNB|59=4|60=20240611-09:01:46.228000|150=0|151=5.00000000|636=Y|1057=Y|25001=1|25017=0.00000000|25018=20240611-09:01:46.228000|25023=20240611-09:01:46.228000|10=095|
```

[]()

#### OrderCancelRequest`<F>`[​](/docs/binance-spot-api-docs/fix-api#ordercancelrequestf) ####

Sent by the client to cancel an order or an order list.

* To cancel an order either `OrderID (11)` or `OrigClOrdID (41)` are required.
* To cancel an order list either `ListID (66)` or `OrigClListID (25015)` are required.

If the canceled order is part of an order list, the entire list will be canceled.

| Tag |       Name       | Type |Required|                                                   Description                                                    |
|-----|------------------|------|--------|------------------------------------------------------------------------------------------------------------------|
| 11  |     ClOrdID      |STRING|   Y    |                                            `ClOrdID` of this request.                                            |
| 41  |   OrigClOrdID    |STRING|   N    |                                      `ClOrdID (11)` of the order to cancel.                                      |
| 37  |     OrderID      | INT  |   N    |                                      `OrderID (37)` of the order to cancel.                                      |
|25015|   OrigClListID   |STRING|   N    |                                 `ClListID (25014)` of the order list to cancel.                                  |
| 66  |      ListID      |STRING|   N    |                                    `ListID (66)` of the order list to cancel.                                    |
| 55  |      Symbol      |STRING|   Y    |                                         Symbol on which to cancel order.                                         |
|25002|CancelRestrictions| INT  |   N    |Restrictions on the cancel. Possible values:   <br/><br/>`1` - ONLY\_NEW   <br/><br/>`2` - ONLY\_PARTIALLY\_FILLED|

**Sample message:**

```
8=FIX.4.4|9=93|35=F|34=2|49=ieBwvCKy|52=20240613-01:11:13.784|56=SPOT|11=1718241073695674483|37=2|55=LTCBNB|10=210|
```

**Response:**

* [ExecutionReport`<8>`](/docs/binance-spot-api-docs/fix-api#executionreport) with `ExecType (150)` value `CANCELED (4)` for each canceled order.
* [ListStatus`<N>`](/docs/binance-spot-api-docs/fix-api#liststatus) if orders in an order list were canceled.
* [OrderCancelReject`<9>`](/docs/binance-spot-api-docs/fix-api#ordercancelreject) if cancellation was rejected.
* [Reject`<3>`](/docs/binance-spot-api-docs/fix-api#reject) if the message is rejected.

[]()

#### OrderCancelReject`<9>`[​](/docs/binance-spot-api-docs/fix-api#ordercancelreject9) ####

Sent by the server when [OrderCancelRequest`<F>`](/docs/binance-spot-api-docs/fix-api#ordercancelrequest) has failed.

| Tag |       Name       | Type |Required|                                                               Description                                                                |
|-----|------------------|------|--------|------------------------------------------------------------------------------------------------------------------------------------------|
| 11  |     ClOrdID      |STRING|   Y    |                                                  `ClOrdID (11)` of the cancel request.                                                   |
| 41  |   OrigClOrdID    |STRING|   N    |                                               `OrigClOrdID (41)` from the cancel request.                                                |
| 37  |     OrderID      | INT  |   N    |                                                 `OrderID (37)` from the cancel request.                                                  |
|25015|   OrigClListID   |STRING|   N    |                                             `OrigClListID (25015)` from the cancel request.                                              |
| 66  |      ListID      |STRING|   N    |                                                  `ListID (66)` from the cancel request.                                                  |
| 55  |      Symbol      |STRING|   Y    |                                                  `Symbol (55)` from the cancel request.                                                  |
|25002|CancelRestrictions| INT  |   N    |                                          `CancelRestrictions (25002)` from the cancel request.                                           |
| 434 | CxlRejResponseTo | CHAR |   Y    |Type of request that this OrderCancelReject`<9>` is in response to.   <br/><br/> Possible values:   <br/><br/>`1` - ORDER\_CANCEL\_REQUEST|
|25016|    ErrorCode     | INT  |   Y    |                                 API error code (see [Error Codes](/docs/binance-spot-api-docs/errors)).                                  |
| 58  |       Text       |STRING|   Y    |                                                      Human-readable error message.                                                       |

**Sample message:**

```
8=FIX.4.4|9=137|35=9|34=2|49=SPOT|52=20240613-01:12:41.320869|56=OlZb8ht8|11=1718241161272843932|37=2|55=LTCBNB|58=Unknown order sent.|434=1|25016=-1013|10=087|
```

[]()

#### OrderCancelRequestAndNewOrderSingle`<XCN>`[​](/docs/binance-spot-api-docs/fix-api#ordercancelrequestandnewordersinglexcn) ####

Sent by the client to cancel an order and submit a new one for execution.

Please refer to [Supported Order Types](/docs/binance-spot-api-docs/fix-api#ordertype) for supported field combinations when describing the new order.

>
>
> [!NOTE]
> Cancel is always processed first. Then immediately after that the new order is submitted.
>
>

| Tag |                 Name                  | Type |Required|                                                                                                                                                                                                     Description                                                                                                                                                                                                     |
|-----|---------------------------------------|------|--------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|25033|OrderCancelRequestAndNewOrderSingleMode| INT  |   Y    |                                                                                                                                   What action should be taken if cancel fails.   <br/><br/> Possible values:   <br/><br/>`1` - STOP\_ON\_FAILURE   <br/><br/>`2` - ALLOW\_FAILURE                                                                                                                                   |
|25038|      OrderRateLimitExceededMode       | INT  |   N    |                                                                                                                     What should be done to the cancellation request if you exceed the unfilled order rate limit.   <br/><br/>Possible values: `1` - DO\_NOTHING   <br/><br/>`2` - CANCEL\_ONLY                                                                                                                      |
| 37  |                OrderID                | INT  |   N    |                                                                                                                                                                                          `OrderID` of the order to cancel.                                                                                                                                                                                          |
|25034|             CancelClOrdID             |STRING|   N    |                                                                                                                                                                                              `ClOrdID` of the cancel.                                                                                                                                                                                               |
| 41  |              OrigClOrdID              |STRING|   N    |                                                                                                                                                                                          `ClOrdID` of the order to cancel.                                                                                                                                                                                          |
| 11  |                ClOrdID                |STRING|   Y    |                                                                                                                                                                                     `ClOrdID` to be assigned to the new order.                                                                                                                                                                                      |
|25002|          CancelRestrictions           | INT  |   N    |                                                                                                                                                 Restrictions on the cancel. Possible values:   <br/><br/>`1` - ONLY\_NEW   <br/><br/>`2` - ONLY\_PARTIALLY\_FILLED                                                                                                                                                  |
| 38  |               OrderQty                | QTY  |   N    |                                                                                                                                                                                              Quantity of the new order                                                                                                                                                                                              |
| 40  |                OrdType                | CHAR |   Y    |                                                                      See the [table](/docs/binance-spot-api-docs/fix-api#ordertype) to understand supported order types and the required fields to use them.  <br/><br/>Possible values:   <br/><br/>`1` - MARKET   <br/><br/>`2` - LIMIT   <br/><br/>`3` - STOP   <br/><br/>`4` - STOP\_LIMIT                                                                      |
| 18  |               ExecInst                | CHAR |   N    |                                                                                                                                                                           Possible values:   <br/><br/>`6` - PARTICIPATE\_DONT\_INITIATE                                                                                                                                                                            |
| 44  |                 Price                 |PRICE |   N    |                                                                                                                                                                                               Price of the new order                                                                                                                                                                                                |
| 54  |                 Side                  | CHAR |   Y    |                                                                                                                                                             Side of the order.  <br/><br/>Possible values:   <br/><br/>`1` - BUY   <br/><br/>`2` - SELL                                                                                                                                                             |
| 55  |                Symbol                 |STRING|   Y    |                                                                                                                                                                                      Symbol to cancel and place the order on.                                                                                                                                                                                       |
| 59  |              TimeInForce              | CHAR |   N    |                                                                                                                                           Possible values:   <br/><br/>`1` - GOOD\_TILL\_CANCEL   <br/><br/>`3` - IMMEDIATE\_OR\_CANCEL   <br/><br/>`4` - FILL\_OR\_KILL                                                                                                                                            |
| 111 |               MaxFloor                | QTY  |   N    |                                                                                                                                                               Used for iceberg orders, this specifies the visible quantity of the order on the book.                                                                                                                                                                |
| 152 |             CashOrderQty              | QTY  |   N    |                                                                                                                                                                Quantity of the order specified in the quote asset units, for reverse market orders.                                                                                                                                                                 |
| 847 |            TargetStrategy             | INT  |   N    |                                                                                                                                                                                      The value cannot be less than `1000000`.                                                                                                                                                                                       |
|7940 |              StrategyID               | INT  |   N    |                                                                                                                                                                                                                                                                                                                                                                                                                     |
|25001|        SelfTradePreventionMode        | CHAR |   N    |                                                                                                                                       Possible values:   <br/><br/>`1` - NONE   <br/><br/>`2` - EXPIRE\_TAKER   <br/><br/>`3` - EXPIRE\_MAKER   <br/><br/>`4` - EXPIRE\_BOTH                                                                                                                                        |
|1100 |              TriggerType              | CHAR |   N    |                                                                                                                                                                                       Possible values: `4` - PRICE\_MOVEMENT                                                                                                                                                                                        |
|1101 |             TriggerAction             | CHAR |   N    |                                                                                                                                                                                     Possible values:   <br/><br/>`1` - ACTIVATE                                                                                                                                                                                     |
|1102 |             TriggerPrice              |PRICE |   N    |                                                                                                                                                         Activation price for contingent orders. See [table](/docs/binance-spot-api-docs/fix-api#ordertype)                                                                                                                                                          |
|1107 |           TriggerPriceType            | CHAR |   N    |                                                                                                                                                                                   Possible values:   <br/><br/>`2` - LAST\_TRADE                                                                                                                                                                                    |
|1109 |         TriggerPriceDirection         | CHAR |   N    |Used to differentiate between StopLoss and TakeProfit orders. See [table](/docs/binance-spot-api-docs/fix-api#ordertype).  <br/><br/>Possible values:   <br/><br/>`U` - TRIGGER\_IF\_THE\_PRICE\_OF\_THE\_SPECIFIED\_TYPE\_GOES\_UP\_TO\_OR\_THROUGH\_THE\_SPECIFIED\_TRIGGER\_PRICE   <br/><br/>`D` - TRIGGER\_IF\_THE\_PRICE\_OF\_THE\_SPECIFIED\_TYPE\_GOES\_DOWN\_TO\_OR\_THROUGH\_THE\_SPECIFIED\_TRIGGER\_PRICE|
|25009|       TriggerTrailingDeltaBips        | INT  |   N    |                                                                                                                                                                                         Provide to create trailing orders.                                                                                                                                                                                          |

**Sample message:**

```
8=FIX.4.4|9=160|35=XCN|34=2|49=JS8iiXK6|52=20240613-02:31:53.753|56=SPOT|11=1718245913721036458|37=8|38=5|40=2|44=4|54=1|55=LTCBNB|59=1|111=1|25033=1|25034=1718245913721036819|10=229|
```

**Response:**

* [ExecutionReport`<8>`](/docs/binance-spot-api-docs/fix-api#executionreport) with `ExecType (150)` value `CANCELED (4)` for the canceled order.
* [ExecutionReport`<8>`](/docs/binance-spot-api-docs/fix-api#executionreport) with `ExecType (150)` value `NEW (0)` for the new order.
* [ExecutionReport`<8>`](/docs/binance-spot-api-docs/fix-api#executionreport) with `ExecType (150)` value `REJECTED (8)` if the new order was rejected.
* [OrderCancelReject`<9>`](/docs/binance-spot-api-docs/fix-api#ordercancelreject) if the cancellation was rejected.
* [Reject`<3>`](/docs/binance-spot-api-docs/fix-api#reject) if the message is rejected.

[]()

#### OrderMassCancelRequest`<q>`[​](/docs/binance-spot-api-docs/fix-api#ordermasscancelrequestq) ####

Sent by the client to cancel all open orders on a symbol.

>
>
> [!NOTE]
> All orders of the account will be canceled, including those placed in different connections.
>
>

|Tag|        Name         | Type |Required|                       Description                       |
|---|---------------------|------|--------|---------------------------------------------------------|
|11 |       ClOrdID       |STRING|   Y    |         `ClOrdId` of this mass cancel request.          |
|55 |       Symbol        |STRING|   Y    |            Symbol on which to cancel orders.            |
|530|MassCancelRequestType| CHAR |   Y    |Possible values:   <br/><br/>`1` - CANCEL\_SYMBOL\_ORDERS|

**Sample message:**

```
8=FIX.4.4|9=94|35=q|34=2|49=dpYPesqv|52=20240613-01:24:36.948|56=SPOT|11=1718241876901971671|55=ABCDEF|530=1|10=110|
```

**Responses:**

* [ExecutionReport`<8>`](/docs/binance-spot-api-docs/fix-api#executionreport) with `ExecType (150)` value `CANCELED (4)` for the every order canceled.
* [OrderMassCancelReport`<r>`](/docs/binance-spot-api-docs/fix-api#ordermasscancelreport) with `MassCancelResponse (531)` field indicating whether the message is accepted or rejected.
* [Reject`<3>`](/docs/binance-spot-api-docs/fix-api#reject) if the message is rejected.

[]()

#### OrderMassCancelReport`<r>`[​](/docs/binance-spot-api-docs/fix-api#ordermasscancelreportr) ####

Sent by the server in response to [OrderMassCancelRequest`<q>`](/docs/binance-spot-api-docs/fix-api#ordermasscancelrequest).

| Tag |         Name         | Type |Required|                                             Description                                             |
|-----|----------------------|------|--------|-----------------------------------------------------------------------------------------------------|
| 55  |        Symbol        |STRING|   Y    |                               `Symbol (55)` from the cancel request.                                |
| 11  |       ClOrdID        |STRING|   Y    |                                `ClOrdID (11)` of the cancel request.                                |
| 530 |MassCancelRequestType | CHAR |   Y    |                       `MassCancelRequestType (530)` from the cancel request.                        |
| 531 |  MassCancelResponse  | CHAR |   Y    |Possible values:   <br/><br/>`0` - CANCEL\_REQUEST\_REJECTED   <br/><br/>`1` - CANCEL\_SYMBOL\_ORDERS|
| 532 |MassCancelRejectReason| INT  |   N    |                              Possible values:   <br/><br/>`99` - OTHER                              |
| 533 | TotalAffectedOrders  | INT  |   N    |                                   How many orders were canceled.                                    |
|25016|      ErrorCode       | INT  |   N    |               API error code (see [Error Codes](/docs/binance-spot-api-docs/errors)).               |
| 58  |         Text         |STRING|   N    |                                    Human-readable error message.                                    |

**Sample message:**

```
8=FIX.4.4|9=109|35=r|34=2|49=SPOT|52=20240613-01:24:36.949763|56=dpYPesqv|11=1718241876901971671|55=LTCBNB|530=1|531=1|533=5|10=083|
```

[]()

#### NewOrderList`<E>`[​](/docs/binance-spot-api-docs/fix-api#neworderliste) ####

Sent by the client to submit a list of orders for execution.

Orders in an order list are contingent on one another.
Please refer to [Supported Order List Types](/docs/binance-spot-api-docs/fix-api#order-list-types) for supported order types and triggering instructions.

|   Tag    |            Name            |   Type   |Required|                                                                                                                                                                                                     Description                                                                                                                                                                                                     |
|----------|----------------------------|----------|--------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|  25014   |          ClListID          |  STRING  |   Y    |                                                                                                                                                                                    `ClListID` to be assigned to the order list.                                                                                                                                                                                     |
|   1385   |      ContingencyType       |   INT    |   N    |                                                                                                                                                       Possible values:   <br/><br/>`1` - ONE\_CANCELS\_THE\_OTHER   <br/><br/>`2` - ONE\_TRIGGERS\_THE\_OTHER                                                                                                                                                       |
|    73    |          NoOrders          |NUMINGROUP|   N    |                                                                                                                                                                            The length of the array for Orders. Only 2 or 3 are allowed.                                                                                                                                                                             |
|  \=\>11  |          ClOrdID           |  STRING  |   Y    |                                                                                                                                                                                        `ClOrdID` to be assigned to the order                                                                                                                                                                                        |
|  \=\>38  |          OrderQty          |   QTY    |   N    |                                                                                                                                                                                                Quantity of the order                                                                                                                                                                                                |
|  \=\>40  |          OrdType           |   CHAR   |   Y    |                                                                      See the [table](/docs/binance-spot-api-docs/fix-api#ordertype) to understand supported order types and the required fields to use them.  <br/><br/>Possible values:   <br/><br/>`1` - MARKET   <br/><br/>`2` - LIMIT   <br/><br/>`3` - STOP   <br/><br/>`4` - STOP\_LIMIT                                                                      |
|  \=\>18  |          ExecInst          |   CHAR   |   N    |                                                                                                                                                                           Possible values:   <br/><br/>`6` - PARTICIPATE\_DONT\_INITIATE                                                                                                                                                                            |
|  \=\>44  |           Price            |  PRICE   |   N    |                                                                                                                                                                                                 Price of the order                                                                                                                                                                                                  |
|  \=\>54  |            Side            |   CHAR   |   Y    |                                                                                                                                                                  Side of the order. Possible values:   <br/><br/>`1` - BUY   <br/><br/>`2` - SELL                                                                                                                                                                   |
|  \=\>55  |           Symbol           |  STRING  |   Y    |                                                                                                                                                                                            Symbol to place the order on.                                                                                                                                                                                            |
|  \=\>59  |        TimeInForce         |   CHAR   |   N    |                                                                                                                                           Possible values:   <br/><br/>`1` - GOOD\_TILL\_CANCEL   <br/><br/>`3` - IMMEDIATE\_OR\_CANCEL   <br/><br/>`4` - FILL\_OR\_KILL                                                                                                                                            |
| \=\>111  |          MaxFloor          |   QTY    |   N    |                                                                                                                                                               Used for iceberg orders, this specifies the visible quantity of the order on the book.                                                                                                                                                                |
| \=\>152  |        CashOrderQty        |   QTY    |   N    |                                                                                                                                                                Quantity of the order specified in the quote asset units, for reverse market orders.                                                                                                                                                                 |
| \=\>847  |       TargetStrategy       |   INT    |   N    |                                                                                                                                                                                      The value cannot be less than `1000000`.                                                                                                                                                                                       |
| \=\>7940 |         StrategyID         |   INT    |   N    |                                                                                                                                                                                                                                                                                                                                                                                                                     |
|\=\>25001 |  SelfTradePreventionMode   |   CHAR   |   N    |                                                                                                                                       Possible values:   <br/><br/>`1` - NONE   <br/><br/>`2` - EXPIRE\_TAKER   <br/><br/>`3` - EXPIRE\_MAKER   <br/><br/>`4` - EXPIRE\_BOTH                                                                                                                                        |
| \=\>1100 |        TriggerType         |   CHAR   |   N    |                                                                                                                                                                                 Possible values:   <br/><br/>`4` - PRICE\_MOVEMENT                                                                                                                                                                                  |
| \=\>1101 |       TriggerAction        |   CHAR   |   N    |                                                                                                                                                                                     Possible values:   <br/><br/>`1` - ACTIVATE                                                                                                                                                                                     |
| \=\>1102 |        TriggerPrice        |  PRICE   |   N    |                                                                                                                                                         Activation price for contingent orders. See [table](/docs/binance-spot-api-docs/fix-api#ordertype)                                                                                                                                                          |
| \=\>1107 |      TriggerPriceType      |   CHAR   |   N    |                                                                                                                                                                                   Possible values:   <br/><br/>`2` - LAST\_TRADE                                                                                                                                                                                    |
| \=\>1109 |   TriggerPriceDirection    |   CHAR   |   N    |Used to differentiate between StopLoss and TakeProfit orders. See [table](/docs/binance-spot-api-docs/fix-api#ordertype).  <br/><br/>Possible values:   <br/><br/>`U` - TRIGGER\_IF\_THE\_PRICE\_OF\_THE\_SPECIFIED\_TYPE\_GOES\_UP\_TO\_OR\_THROUGH\_THE\_SPECIFIED\_TRIGGER\_PRICE   <br/><br/>`D` - TRIGGER\_IF\_THE\_PRICE\_OF\_THE\_SPECIFIED\_TYPE\_GOES\_DOWN\_TO\_OR\_THROUGH\_THE\_SPECIFIED\_TRIGGER\_PRICE|
|\=\>25009 |  TriggerTrailingDeltaBips  |   INT    |   N    |                                                                                                                                                                                         Provide to create trailing orders.                                                                                                                                                                                          |
|\=\>25010 |NoListTriggeringInstructions|NUMINGROUP|   N    |                                                                                                                                                                               The length of the array for ListTriggeringInstructions.                                                                                                                                                                               |
|\==\>25011|      ListTriggerType       |   CHAR   |   N    |                                                                                        What needs to happen to the order pointed to by ListTriggerTriggerIndex in order for the action to take place.   <br/><br/> Possible values:   <br/><br/>`1` - ACTIVATED   <br/><br/>`2` - PARTIALLY\_FILLED   <br/><br/>`3` - FILLED                                                                                        |
|\==\>25012|  ListTriggerTriggerIndex   |   INT    |   N    |                                                                                                                                                                                       Index of the trigger order: 0-indexed.                                                                                                                                                                                        |
|\==\>25013|     ListTriggerAction      |   CHAR   |   N    |                                                                                                                          Action to take place on this order after the ListTriggerType has been fulfilled.   <br/><br/> Possible values:   <br/><br/>`1` - RELEASE   <br/><br/>`2` - CANCEL                                                                                                                          |

**Sample message:**

```
8=FIX.4.4|9=236|35=E|34=2|49=Eg13pOvN|52=20240607-02:19:07.836|56=SPOT|73=2|11=w1717726747805308656|55=LTCBNB|54=2|38=1|40=2|44=0.25|59=1|11=p1717726747805308656|55=LTCBNB|54=2|38=1|40=1|25010=1|25011=3|25012=0|25013=1|1385=2|25014=1717726747805308656|10=171|
```

[]()

#### Supported Order List Types[​](/docs/binance-spot-api-docs/fix-api#supported-order-list-types) ####

>
>
> [!NOTE]
> Orders must be specified in the sequence indicated in the *Order Names* column in the table below.
>
>

|Order list name|Contingency Type (1385)|                                     Order names                                      |                                                    Order sides                                                     |                                                                  Allowed Binance order types                                                                   |                                                                                                            List Triggering Instructions                                                                                                            |
|---------------|-----------------------|--------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      OCO      |          `1`          |                      1. below order  <br/><br/> 2. above order                       |                              1. below order=`SELL`  <br/><br/> 2. above order=`SELL`                               |                                    1. below order=`STOP_LOSS` or `STOP_LOSS_LIMIT`  <br/><br/> 2. above order=`LIMIT_MAKER`                                    |                                                      1. below order:   <br/><br/>`25010=1|25011=2|25012=1|25013=2|`  <br/><br/>2. above order:   <br/><br/>`25010=1|25011=1|25012=0|25013=2|`                                                      |
|      OCO      |          `1`          |                      1. below order  <br/><br/> 2. above order                       |                               1. below order=`BUY`   <br/><br/> 2. above order=`BUY`                               |                                   1. below order=`LIMIT_MAKER`   <br/><br/> 2. above order=`STOP_LOSS` or `STOP_LOSS_LIMIT`                                    |                                                      1. below order:   <br/><br/>`25010=1|25011=1|25012=1|25013=2|`  <br/><br/>2. above order:   <br/><br/>`25010=1|25011=2|25012=0|25013=2|`                                                      |
|      OCO      |          `1`          |                      1. below order  <br/><br/> 2. above order                       |                              1. below order=`SELL`  <br/><br/> 2. above order=`SELL`                               |                                   1. below order=`STOP_LOSS` or `STOP_LOSS_LIMIT`  <br/><br/> 2. above order= `TAKE_PROFIT`                                    |                                                      1. below order:   <br/><br/>`25010=1|25011=1|25012=1|25013=2|`  <br/><br/>2. above order:   <br/><br/>`25010=1|25011=1|25012=0|25013=2|`                                                      |
|      OCO      |          `1`          |                      1. below order  <br/><br/> 2. above order                       |                               1. below order=`BUY`   <br/><br/> 2. above order=`BUY`                               |                                  1. below order=`TAKE_PROFIT`   <br/><br/> 2. above order = `STOP_LOSS` or `STOP_LOSS_LIMIT`                                   |                                                      1. below order:   <br/><br/>`25010=1|25011=1|25012=1|25013=2|`  <br/><br/>2. above order:   <br/><br/>`25010=1|25011=1|25012=0|25013=2|`                                                      |
|      OTO      |          `2`          |                    1. working order  <br/><br/> 2. pending order                     |                   1. working order=`SELL` or `BUY`  <br/><br/> 2. pending order=`SELL` or `BUY`                    |                                          1. working order=`LIMIT` or `LIMIT_MAKER`   <br/><br/> 2. pending order=ANY                                           |                                                                   1. working order:  <br/><br/>NONE  <br/><br/> 2. pending order:   <br/><br/>`25010=1|25011=3|25012=0|25013=1|`                                                                   |
|     OTOCO     |          `2`          |1. working order  <br/><br/> 2. pending below order  <br/><br/> 3. pending above order|1. working order=`SELL` or `BUY`  <br/><br/> 2. pending below order=`SELL`  <br/><br/> 3. pending above order=`SELL`|1. working order=`LIMIT` or `LIMIT_MAKER`   <br/><br/> 2. pending below order=`STOP_LOSS` or `STOP_LOSS_LIMIT`  <br/><br/> 3. pending above order=`LIMIT_MAKER` |1. working order:  <br/><br/>NONE  <br/><br/> 2. pending below order:  <br/><br/>`25010=2|25011=3|25012=0|25013=2|25011=2|25012=2|25013=2|`  <br/><br/>3. pending above order:  <br/><br/>`25010=2|25011=3|25012=0|25013=2|25011=1|25012=1|25013=2|`|
|     OTOCO     |          `2`          |1. working order  <br/><br/> 2. pending below order  <br/><br/> 3. pending above order| 1. working order=`SELL` or `BUY`  <br/><br/> 2. pending below order=`BUY`  <br/><br/> 3. pending above order=`BUY` |1. working order=`LIMIT` or `LIMIT_MAKER`   <br/><br/> 2. pending below order=`LIMIT_MAKER`   <br/><br/> 3. pending above order=`STOP_LOSS` or `STOP_LOSS_LIMIT`|1. working order:  <br/><br/>NONE  <br/><br/> 2. pending below order:  <br/><br/>`25010=2|25011=3|25012=0|25013=2|25011=1|25012=2|25013=2|`  <br/><br/>3. pending above order:  <br/><br/>`25010=2|25011=3|25012=0|25013=2|25011=2|25012=1|25013=2|`|
|     OTOCO     |          `2`          |1. working order  <br/><br/> 2. pending below order  <br/><br/> 3. pending above order|1. working order=`SELL` or `BUY`  <br/><br/> 2. pending below order=`SELL`  <br/><br/> 3. pending above order=`SELL`|1. working order=`LIMIT` or `LIMIT_MAKER`   <br/><br/> 2. pending below order=`STOP_LOSS` or `STOP_LOSS_LIMIT`  <br/><br/> 3. pending above order=`TAKE_PROFIT` |1. working order:  <br/><br/>NONE  <br/><br/> 2. pending below order:  <br/><br/>`25010=2|25011=3|25012=0|25013=2|25011=1|25012=2|25013=2|`  <br/><br/>3. pending above order:  <br/><br/>`25010=2|25011=3|25012=0|25013=2|25011=1|25012=1|25013=2|`|
|     OTOCO     |          `2`          |1. working order  <br/><br/> 2. pending below order  <br/><br/> 3. pending above order| 1. working order=`SELL` or `BUY`  <br/><br/> 2. pending below order=`BUY`  <br/><br/> 3. pending above order=`BUY` |1. working order=`LIMIT` or `LIMIT_MAKER`   <br/><br/> 2. pending below order=`TAKE_PROFIT`   <br/><br/> 3. pending above order=`STOP_LOSS` or `STOP_LOSS_LIMIT`|1. working order:  <br/><br/>NONE  <br/><br/> 2. pending below order:  <br/><br/>`25010=2|25011=3|25012=0|25013=2|25011=1|25012=2|25013=2|`  <br/><br/>3. pending above order:  <br/><br/>`25010=2|25011=3|25012=0|25013=2|25011=1|25012=1|25013=2|`|

[]()

#### ListStatus`<N>`[​](/docs/binance-spot-api-docs/fix-api#liststatusn) ####

Sent by the server whenever an order list state changes.

>
>
> [!NOTE]
> By default, ListStatus`<N>` is sent for all order lists of an account, including those submitted in different connections.
> Please see [Response Mode](/docs/binance-spot-api-docs/fix-api#responsemode) for other behavior options.
>
>

|   Tag    |            Name            |    Type    |Required|                                               Description                                               |
|----------|----------------------------|------------|--------|---------------------------------------------------------------------------------------------------------|
|    55    |           Symbol           |   STRING   |   Y    |                                        Symbol of the order list.                                        |
|    66    |           ListID           |   STRING   |   N    |                            `ListID` of the list as assigned by the exchange.                            |
|  25014   |          ClListID          |   STRING   |   N    |                           `ClListID` of the list as assigned on the request.                            |
|  25015   |        OrigClListID        |   STRING   |   N    |                                                                                                         |
|   1385   |      ContingencyType       |    INT     |   N    | Possible values:   <br/><br/>`1` - ONE\_CANCELS\_THE\_OTHER   <br/><br/>`2` - ONE\_TRIGGERS\_THE\_OTHER |
|   429    |       ListStatusType       |    INT     |   Y    | Possible values:   <br/><br/>`2` - RESPONSE   <br/><br/>`4` - EXEC\_STARTED   <br/><br/>`5` - ALL\_DONE |
|   431    |      ListOrderStatus       |    INT     |   Y    |    Possible values:   <br/><br/>`3` - EXECUTING   <br/><br/>`6` - ALL\_DONE   <br/><br/>`7` - REJECT    |
|   1386   |      ListRejectReason      |    INT     |   N    |                                Possible values:   <br/><br/>`99` - OTHER                                |
|   103    |        OrdRejReason        |    INT     |   N    |                                Possible values:   <br/><br/>`99` - OTHER                                |
|    60    |        TransactTime        |UTCTIMESTAMP|   N    |                                   Timestamp when this event occurred.                                   |
|  25016   |         ErrorCode          |    INT     |   N    |                 API error code (see [Error Codes](/docs/binance-spot-api-docs/errors)).                 |
|    58    |            Text            |   STRING   |   N    |                                      Human-readable error message.                                      |
|    73    |          NoOrders          | NUMINGROUP |   N    |                                   The length of the array for Orders.                                   |
|  \=\>55  |           Symbol           |   STRING   |   Y    |                                          Symbol of the order.                                           |
|  \=\>37  |          OrderID           |    INT     |   Y    |                           `OrderID` of the order as assigned by the exchange.                           |
|  \=\>11  |          ClOrdID           |   STRING   |   Y    |                           `ClOrdID` of the order as assigned on the request.                            |
|\=\>25010 |NoListTriggeringInstructions| NUMINGROUP |   N    |                         The length of the array for ListTriggeringInstructions.                         |
|\==\>25011|      ListTriggerType       |    CHAR    |   N    |Possible values:   <br/><br/>`1` - ACTIVATED   <br/><br/>`2` - PARTIALLY\_FILLED   <br/><br/>`3` - FILLED|
|\==\>25012|  ListTriggerTriggerIndex   |    INT     |   N    |                                                                                                         |
|\==\>25013|     ListTriggerAction      |    CHAR    |   N    |                   Possible values:   <br/><br/>`1` - RELEASE   <br/><br/>`2` - CANCEL                   |

**Sample message:**

```
8=FIX.4.4|9=290|35=N|34=2|49=SPOT|52=20240607-02:19:07.837191|56=Eg13pOvN|55=ABCDEF|60=20240607-02:19:07.836000|66=25|73=2|55=LTCBNB|37=52|11=w1717726747805308656|55=ABCDEF|37=53|11=p1717726747805308656|25010=1|25011=3|25012=0|25013=1|429=4|431=3|1385=2|25014=1717726747805308656|25015=1717726747805308656|10=019|
```

### Limit Messages[​](/docs/binance-spot-api-docs/fix-api#limit-messages) ###

[]()

#### LimitQuery`<XLQ>`[​](/docs/binance-spot-api-docs/fix-api#limitqueryxlq) ####

Sent by the client to query current limits.

|Tag |Name | Type |Required|   Description    |
|----|-----|------|--------|------------------|
|6136|ReqID|STRING|   Y    |ID of this request|

**Sample message:**

```
8=FIX.4.4|9=82|35=XLQ|34=2|49=7buKHZxZ|52=20240614-05:35:35.357|56=SPOT|6136=1718343335357229749|10=170|
```

[]()

#### LimitResponse`<XLR>`[​](/docs/binance-spot-api-docs/fix-api#limitresponsexlr) ####

Sent by the server in response to [LimitQuery`<XLQ>`](/docs/binance-spot-api-docs/fix-api#limitquery).

|   Tag   |            Name            |   Type   |Required|                                                                   Description                                                                    |
|---------|----------------------------|----------|--------|--------------------------------------------------------------------------------------------------------------------------------------------------|
|  6136   |           ReqID            |  STRING  |   Y    |                                                            `ReqID` from the request.                                                             |
|  25003  |     NoLimitIndicators      |NUMINGROUP|   Y    |                                                   The length of the array for LimitIndicators.                                                   |
|\=\>25004|         LimitType          |   CHAR   |   Y    |                                 Possible values:   <br/><br/>`1` - ORDER\_LIMIT   <br/><br/>`2` - MESSAGE\_LIMIT                                 |
|\=\>25005|         LimitCount         |   INT    |   Y    |                                                          The current use of this limit.                                                          |
|\=\>25006|          LimitMax          |   INT    |   Y    |                                                       The maximum allowed for this limit.                                                        |
|\=\>25007|     LimitResetInterval     |   INT    |   N    |                                                           How often the limit resets.                                                            |
|\=\>25008|LimitResetIntervalResolution|   CHAR   |   N    |Time unit of `LimitResetInterval`. Possible values:   <br/><br/>`s` - SECOND   <br/><br/>`m` - MINUTE   <br/><br/>`h` - HOUR   <br/><br/>`d` - DAY|

**Sample message:**

```
8=FIX.4.4|9=225|35=XLR|34=2|49=SPOT|52=20240614-05:42:42.724057|56=uGnG0ef8|6136=1718343762723730315|25003=3|25004=2|25005=1|25006=1000|25007=10|25008=s|25004=1|25005=0|25006=200|25007=10|25008=s|25004=1|25005=0|25006=200000|25007=1|25008=d|10=241|
```

### Market Data Messages[​](/docs/binance-spot-api-docs/fix-api#market-data-messages) ###

>
>
> [!NOTE]
> The messages below can only be used for the FIX Market Data.
>
>

[]()

#### InstrumentListRequest`<x>`[​](/docs/binance-spot-api-docs/fix-api#instrumentlistrequestx) ####

Sent by the client to query information about active instruments (i.e., those that have the TRADING status). If used for an inactive instrument, it will be responded to with a [Reject`<3>`](/docs/binance-spot-api-docs/fix-api#reject).

|Tag|          Name           | Type |Required|                                      Description                                       |
|---|-------------------------|------|--------|----------------------------------------------------------------------------------------|
|320|     InstrumentReqID     |STRING|   Y    |                                   ID of this request                                   |
|559|InstrumentListRequestType| INT  |   Y    |Possible values:   <br/><br/>`0` - SINGLE\_INSTRUMENT   <br/><br/>`4` - ALL\_INSTRUMENTS|
|55 |         Symbol          |STRING|   N    |     Required when the `InstrumentListRequestType` is set to `SINGLE_INSTRUMENT(0)`     |

**Sample message:**

```
8=FIX.4.4|9=92|35=x|49=BMDWATCH|56=SPOT|34=2|52=20250114-08:46:56.096691|320=BTCUSDT_INFO|559=0|55=BTCUSDT|10=164|
```

[]()

#### InstrumentList`<y>`[​](/docs/binance-spot-api-docs/fix-api#instrumentlisty) ####

Sent by the server in a response to the [InstrumentListRequest`<x>`](/docs/binance-spot-api-docs/fix-api#instrumentlistrequest).

>
>
> [!NOTE]
> More detailed symbol information is available through the [exchangeInfo](https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md#exchange-information) endpoint.
>
>

|   Tag   |        Name         |   Type   |Required|               Description                |
|---------|---------------------|----------|--------|------------------------------------------|
|   320   |   InstrumentReqID   |  STRING  |   Y    |   `InstrumentReqID` from the request.    |
|   146   |    NoRelatedSym     |NUMINGROUP|   Y    |            Number of symbols             |
| \=\>55  |       Symbol        |  STRING  |   Y    |                                          |
| \=\>15  |      Currency       |  STRING  |   Y    |        Quote asset of this symbol        |
| \=\>562 |     MinTradeVol     |   QTY    |   Y    |       The minimum trading quantity       |
|\=\>1140 |     MaxTradeVol     |   QTY    |   Y    |       The maximum trading quantity       |
|\=\>25039|   MinQtyIncrement   |   QTY    |   Y    |      The minimum quantity increase       |
|\=\>25040|  MarketMinTradeVol  |   QTY    |   Y    |The minimum market order trading quantity |
|\=\>25041|  MarketMaxTradeVol  |   QTY    |   Y    |The maximum market order trading quantity |
|\=\>25042|MarketMinQtyIncrement|   QTY    |   Y    |The minimum market order quantity increase|
| \=\>969 |  MinPriceIncrement  |  PRICE   |   Y    |        The minimum price increase        |

**Sample message:**

```
8=FIX.4.4|9=218|35=y|49=SPOT|56=BMDWATCH|34=2|52=20250114-08:46:56.100147|320=BTCUSDT_INFO|146=1|55=BTCUSDT|15=USDT|562=0.00001000|1140=9000.00000000|25039=0.00001000|25040=0.00000001|25041=76.79001236|25042=0.00000001|969=0.01000000|10=093|
```

[]()

#### MarketDataRequest`<V>`[​](/docs/binance-spot-api-docs/fix-api#marketdatarequestv) ####

Sent by the client to subscribe to or unsubscribe from market data stream.

[]()

**Trade Stream**

The Trade Streams push raw trade information; each trade has a unique buyer and seller.

**Fields required to subscribe:**

* `SubscriptionRequestType` present with value `SUBSCRIBE(1)`
* `MDEntryType` present with value `TRADE(2)`

**Update Speed:** Real-time

[]()

**Individual Symbol Book Ticker Stream**

Pushes any update to the best bid or offers price or quantity in real-time for a specified symbol.

**Fields required to subscribe:**

* `SubscriptionRequestType` with value `SUBSCRIBE(1)`
* `MDEntryType` with value `BID(0)`
* `MDEntryType` with value `OFFER(1)`
* `MarketDepth` with value `1`

**Update Speed:** Real-time

>
>
> [!NOTE]
> In the [Individual Symbol Book Ticker Stream](/docs/binance-spot-api-docs/fix-api#symbolbooktickerstream), when `MDUpdateAction` is set to `CHANGE(1)` in a[MarketDataIncrementalRefresh`<X>`](/docs/binance-spot-api-docs/fix-api#marketdataincrementalrefresh) message sent from the server, it replaces the previous best quote.
>
>

[]()

**Diff. Depth Stream**

Order book price and quantity depth updates used to locally manage an order book.

**Fields required to subscribe:**

* `SubscriptionRequestType` with value `SUBSCRIBE(1)`
* `MDEntryType` with value `BID(0)`
* `MDEntryType` with value `OFFER(1)`
* `MarketDepth` with a value between `2` and `5000`, which controls the size of the initial snapshot and has no effect on subsequent [MarketDataIncrementalRefresh`<X>`](/docs/binance-spot-api-docs/fix-api#marketdataincrementalrefresh) messages

**Update Speed:** 100ms

>
>
> [!NOTE]
> Since the [MarketDataSnapshot`<W>`](/docs/binance-spot-api-docs/fix-api#marketdatasnapshot) have a limit on the number of price levels (5000 on each side maximum), you won't learn the quantities for the levels outside of the initial snapshot unless they change.
> So be careful when using the information for those levels, since they might not reflect the full view of the order book.
> However, for most use cases, seeing 5000 levels on each side is enough to understand the market and trade effectively.
>
>

|  Tag  |         Name          |   Type   |Required|                                                              Description                                                               |
|-------|-----------------------|----------|--------|----------------------------------------------------------------------------------------------------------------------------------------|
|  262  |        MDReqID        |  STRING  |   Y    |                                                           ID of this request                                                           |
|  263  |SubscriptionRequestType|   CHAR   |   Y    |                 Subscription Request Type. Possible values:   <br/><br/>`1` - SUBSCRIBE   <br/><br/>`2` - UNSUBSCRIBE                  |
|  264  |      MarketDepth      |   INT    |   N    |Subscription depth.   <br/><br/> Possible values:   <br/><br/>`1` - Book Ticker subscription   <br/><br/>`2`-`5000` - Diff. Depth Stream|
|  266  |    AggregatedBook     |NUMINGROUP|   N    |                                  Possible values:   <br/><br/>`Y` - one book entry per side per price                                  |
|  146  |     NoRelatedSym      |NUMINGROUP|   N    |                                                           Number of symbols                                                            |
|\=\>55 |        Symbol         |  STRING  |   Y    |                                                                                                                                        |
|  267  |    NoMDEntryTypes     |NUMINGROUP|   N    |                                                         Number of entry types                                                          |
|\=\>269|      MDEntryType      |   CHAR   |   Y    |                         Possible values:   <br/><br/>`0` - BID   <br/><br/>`1` - OFFER   <br/><br/>`2` - TRADE                         |

**Sample message:**

```
# Subscriptions# BOOK TICKER Stream8=FIX.4.4|9=132|35=V|49=TRADER1|56=SPOT|34=4|52=20241122-06:17:14.183428|262=BOOK_TICKER_STREAM|263=1|264=1|266=Y|146=1|55=BTCUSDT|267=2|269=0|269=1|10=010|# DEPTH Stream8=FIX.4.4|9=127|35=V|49=TRADER1|56=SPOT|34=7|52=20241122-06:17:14.443822|262=DEPTH_STREAM|263=1|264=10|266=Y|146=1|55=BTCUSDT|267=2|269=0|269=1|10=111|# TRADE Stream8=FIX.4.4|9=120|35=V|49=TRADER1|56=SPOT|34=3|52=20241122-06:34:14.775606|262=TRADE_STREAM|263=1|264=1|266=Y|146=1|55=BTCUSDT|267=1|269=2|10=040|# Unsubscription from TRADE Stream8=FIX.4.4|9=79|35=V|49=TRADER1|56=SPOT|34=7|52=20241122-06:41:56.966969|262=TRADE_STREAM|263=2|264=1|10=113|
```

[]()

### MarketDataRequestReject`<Y>`[​](/docs/binance-spot-api-docs/fix-api#marketdatarequestrejecty) ###

Sent by the server in a response to an invalid MarketDataRequest `<V>`.

| Tag |     Name     | Type |Required|                                           Description                                           |
|-----|--------------|------|--------|-------------------------------------------------------------------------------------------------|
| 262 |   MDReqID    |STRING|   Y    |ID of the invalid [MarketDataRequest`<V>`](/docs/binance-spot-api-docs/fix-api#marketdatarequest)|
| 281 |MDReqRejReason| CHAR |   N    |Possible values:   <br/><br/>`1` - DUPLICATE\_MDREQID   <br/><br/>`2` - TOO\_MANY\_SUBSCRIPTIONS |
|25016|  ErrorCode   | INT  |   N    |                API Error code. See [Errors](/docs/binance-spot-api-docs/errors)                 |
| 58  |     Text     |STRING|   N    |                                  Human-readable error message.                                  |

**Sample message:**

```
8=FIX.4.4|9=0000218|35=Y|49=SPOT|56=EXAMPLE|34=5|52=20241019-05:39:36.688964|262=BOOK_TICKER_2|281=2|25016=-1191|58=Similar subscription is already active on this connection. Symbol='BNBBUSD', active subscription id: 'BOOK_TICKER_1'.|10=137|
```

[]()

### MarketDataSnapshot`<W>`[​](/docs/binance-spot-api-docs/fix-api#marketdatasnapshotw) ###

Sent by the server in response to a [MarketDataRequest`<V>`](/docs/binance-spot-api-docs/fix-api#marketdatarequest), activating [Individual Symbol Book Ticker Stream](/docs/binance-spot-api-docs/fix-api#symbolbooktickerstream) or [Diff. Depth Stream](/docs/binance-spot-api-docs/fix-api#diffdepthstream) subscriptions.

|  Tag  |      Name      |   Type   |Required|                                                       Description                                                        |
|-------|----------------|----------|--------|--------------------------------------------------------------------------------------------------------------------------|
|  262  |    MDReqID     |  STRING  |   Y    |ID of the [MarketDataRequest`<V>`](/docs/binance-spot-api-docs/fix-api#marketdatarequest) that activated this subscription|
|  55   |     Symbol     |  STRING  |   Y    |                                                                                                                          |
| 25044 |LastBookUpdateID|   INT    |   N    |                                                                                                                          |
|  268  |  NoMDEntries   |NUMINGROUP|   Y    |                                                    Number of entries                                                     |
|\=\>269|  MDEntryType   |   CHAR   |   Y    |                  Possible values:   <br/><br/>`0` - BID   <br/><br/>`1` - OFFER   <br/><br/>`2` - TRADE                  |
|\=\>270|   MDEntryPx    |  PRICE   |   Y    |                                                          Price                                                           |
|\=\>271|  MDEntrySize   |   QTY    |   Y    |                                                         Quantity                                                         |

**Sample message:**

```
8=FIX.4.4|9=0000107|35=W|49=SPOT|56=EXAMPLE|34=34|52=20241019-05:41:52.867164|262=BOOK_TICKER_1_2|55=BNBBUSD|25044=0|268=0|10=151|
```

[]()

### MarketDataIncrementalRefresh`<X>`[​](/docs/binance-spot-api-docs/fix-api#marketdataincrementalrefreshx) ###

Sent by the server when there is a change in a subscribed stream.

|   Tag   |      Name       |    Type    |Required|                                                                                                                                                                                                                                                              Description                                                                                                                                                                                                                                                              |
|---------|-----------------|------------|--------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   262   |     MDReqID     |   STRING   |   Y    |                                                                                                                                                                                                      ID of the [MarketDataRequest`<V>`](/docs/binance-spot-api-docs/fix-api#marketdatarequest) that activated this subscription                                                                                                                                                                                                       |
|   893   |  LastFragment   |  BOOLEAN   |   N    |When present, this indicates that the message was fragmented. Fragmentation occurs when `NoMDEntry` would exceed 10000 in a single [MarketDataIncrementalRefresh`<X>`](/docs/binance-spot-api-docs/fix-api#marketdataincrementalrefresh), in order to limit it to 10000. The fragments of a fragmented message are guaranteed to be consecutive in the stream. It can only appear in the [Trade Stream](/docs/binance-spot-api-docs/fix-api#tradestream) and [Diff. Depth Stream](/docs/binance-spot-api-docs/fix-api#diffdepthstream).|
|   268   |   NoMDEntries   | NUMINGROUP |   Y    |                                                                                                                                                                                                                                                           Number of entries                                                                                                                                                                                                                                                           |
| \=\>279 | MDUpdateAction  |    CHAR    |   Y    |                                                                                                                                                                                                                       Possible values:   <br/><br/>`0` - NEW   <br/><br/>`1` - CHANGE   <br/><br/>`2` - DELETE                                                                                                                                                                                                                        |
| \=\>270 |    MDEntryPx    |   PRICE    |   Y    |                                                                                                                                                                                                                                                                 Price                                                                                                                                                                                                                                                                 |
| \=\>271 |   MDEntrySize   |    QTY     |   N    |                                                                                                                                                                                                                                                               Quantity                                                                                                                                                                                                                                                                |
| \=\>269 |   MDEntryType   |    CHAR    |   Y    |                                                                                                                                                                                                                        Possible values:   <br/><br/>`0` - BID   <br/><br/>`1` - OFFER   <br/><br/>`2` - TRADE                                                                                                                                                                                                                         |
| \=\>55  |     Symbol      |   STRING   |   N    |                                                                                                                                                                                          Market Data Entry will default to the same `Symbol` of the previous Market Data Entry in the same Market Data message if `Symbol` is not specified                                                                                                                                                                                           |
| \=\>60  |  TransactTime   |UTCTIMESTAMP|   N    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|\=\>1003 |     TradeID     |    INT     |   N    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|\=\>2446 |  AggressorSide  |    CHAR    |   N    |                                                                                                                                                                                                                                     Possible values:   <br/><br/>`1` - BUY   <br/><br/>`2` - SELL                                                                                                                                                                                                                                     |
|\=\>25043|FirstBookUpdateID|    INT     |   N    |                                                                                                                           Only present in [Diff. Depth Stream](/docs/binance-spot-api-docs/fix-api#diffdepthstream).   <br/><br/> Market Data Entry will default to the same `FirstBookUpdateID` of the previous Market Data Entry in the same Market Data message if `FirstBookUpdateID` is not specified                                                                                                                            |
|\=\>25044|LastBookUpdateID |    INT     |   N    |                                                                         Only present in [Diff. Depth Stream](/docs/binance-spot-api-docs/fix-api#diffdepthstream) and [Individual Symbol Book Ticker Stream](/docs/binance-spot-api-docs/fix-api#symbolbooktickerstream).   <br/><br/> Market Data Entry will default to the same `LastBookUpdateID` of the previous Market Data Entry in the same Market Data message if `LastBookUpdateID` is not specified                                                                         |

**Sample message:**

```
8=FIX.4.4|9=0000313|35=X|49=SPOT|56=EXAMPLE|34=16|52=20241019-05:40:11.466313|262=TRADE_3|893=N|268=3|279=0|269=2|270=10.00000|271=0.01000|55=BNBBUSD|1003=0|60=20241019-05:40:11.464000|279=0|269=2|270=10.00000|271=0.01000|1003=1|60=20241019-05:40:11.464000|279=0|269=2|270=10.00000|271=0.01000|1003=2|60=20241019-05:40:11.464000|10=125|
```

**Sample fragmented messages:**

>
>
> [!NOTE]
> Below are example messages, with `NoMDEntry` limited to *2*, In the real streams, the `NoMDEntry` is limited to *10000*.
>
>

[Trade Stream](/docs/binance-spot-api-docs/fix-api#tradestream)

```
8=FIX.4.4|9=237|35=X|34=114|49=SPOT|52=20250116-19:36:44.544549|56=EXAMPLE|262=id|268=2|279=0|270=240.00|271=3.00000000|269=2|55=BNBBUSD|60=20250116-19:36:44.196569|1003=67|279=0|270=238.00|271=2.00000000|269=2|60=20250116-19:36:44.196569|1003=68|893=N|10=180|8=FIX.4.4|9=163|35=X|34=115|49=SPOT|52=20250116-19:36:44.544659|56=EXAMPLE|262=id|268=1|279=0|270=233.00|271=1.00000000|269=2|55=BNBBUSD|60=20250116-19:36:44.196569|1003=69|893=Y|10=243|
```

[Diff. Depth Stream](/docs/binance-spot-api-docs/fix-api#diffdepthstream)

```
8=FIX.4.4|9=156|35=X|34=12|49=SPOT|52=20250116-19:45:31.774162|56=EXAMPLE|262=id|268=2|279=2|270=362.00|269=0|55=BNBBUSD|25043=1143|25044=1145|279=2|270=313.00|269=0|893=N|10=047|8=FIX.4.4|9=171|35=X|34=13|49=SPOT|52=20250116-19:45:31.774263|56=EXAMPLE|262=id|268=2|279=2|270=284.00|269=0|55=BNBBUSD|25043=1143|25044=1145|279=1|270=264.00|271=3.00000000|269=0|893=N|10=239|8=FIX.4.4|9=149|35=X|34=14|49=SPOT|52=20250116-19:45:31.774281|56=EXAMPLE|262=id|268=1|279=1|270=395.00|271=19.00000000|269=1|55=BNBBUSD|25043=1143|25044=1145|893=Y|10=024|
```

## ERRORS

Error codes for Binance
==========

**Last Updated: 2025-01-09**

Errors consist of two parts: an error code and a message. Codes are universal,
but messages can vary. Here is the error JSON payload:

```
{  "code":-1121,  "msg":"Invalid symbol."}
```

10xx - General Server or Network issues[​](/docs/binance-spot-api-docs/errors#10xx---general-server-or-network-issues)
----------

### -1000 UNKNOWN[​](/docs/binance-spot-api-docs/errors#-1000-unknown) ###

* An unknown error occurred while processing the request.

### -1001 DISCONNECTED[​](/docs/binance-spot-api-docs/errors#-1001-disconnected) ###

* Internal error; unable to process your request. Please try again.

### -1002 UNAUTHORIZED[​](/docs/binance-spot-api-docs/errors#-1002-unauthorized) ###

* You are not authorized to execute this request.

### -1003 TOO\_MANY\_REQUESTS[​](/docs/binance-spot-api-docs/errors#-1003-too_many_requests) ###

* Too many requests queued.
* Too much request weight used; current limit is %s request weight per %s. Please use WebSocket Streams for live updates to avoid polling the API.
* Way too much request weight used; IP banned until %s. Please use WebSocket Streams for live updates to avoid bans.

### -1006 UNEXPECTED\_RESP[​](/docs/binance-spot-api-docs/errors#-1006-unexpected_resp) ###

* An unexpected response was received from the message bus. Execution status unknown.

### -1007 TIMEOUT[​](/docs/binance-spot-api-docs/errors#-1007-timeout) ###

* Timeout waiting for response from backend server. Send status unknown; execution status unknown.

### -1008 SERVER\_BUSY[​](/docs/binance-spot-api-docs/errors#-1008-server_busy) ###

* Server is currently overloaded with other requests. Please try again in a few minutes.

### -1013 INVALID\_MESSAGE[​](/docs/binance-spot-api-docs/errors#-1013-invalid_message) ###

* The request is rejected by the API. (i.e. The request didn't reach the Matching Engine.)
* Potential error messages can be found in [Filter Failures](/docs/binance-spot-api-docs/errors#filter-failures) or [Failures during order placement](/docs/binance-spot-api-docs/errors#other-errors).

### -1014 UNKNOWN\_ORDER\_COMPOSITION[​](/docs/binance-spot-api-docs/errors#-1014-unknown_order_composition) ###

* Unsupported order combination.

### -1015 TOO\_MANY\_ORDERS[​](/docs/binance-spot-api-docs/errors#-1015-too_many_orders) ###

* Too many new orders.
* Too many new orders; current limit is %s orders per %s.

### -1016 SERVICE\_SHUTTING\_DOWN[​](/docs/binance-spot-api-docs/errors#-1016-service_shutting_down) ###

* This service is no longer available.

### -1020 UNSUPPORTED\_OPERATION[​](/docs/binance-spot-api-docs/errors#-1020-unsupported_operation) ###

* This operation is not supported.

### -1021 INVALID\_TIMESTAMP[​](/docs/binance-spot-api-docs/errors#-1021-invalid_timestamp) ###

* Timestamp for this request is outside of the recvWindow.
* Timestamp for this request was 1000ms ahead of the server's time.

### -1022 INVALID\_SIGNATURE[​](/docs/binance-spot-api-docs/errors#-1022-invalid_signature) ###

* Signature for this request is not valid.

11xx - Request issues[​](/docs/binance-spot-api-docs/errors#11xx---request-issues)
----------

### -1100 ILLEGAL\_CHARS[​](/docs/binance-spot-api-docs/errors#-1100-illegal_chars) ###

* Illegal characters found in a parameter.
* Illegal characters found in parameter '%s'; legal range is '%s'.

### -1101 TOO\_MANY\_PARAMETERS[​](/docs/binance-spot-api-docs/errors#-1101-too_many_parameters) ###

* Too many parameters sent for this endpoint.
* Too many parameters; expected '%s' and received '%s'.
* Duplicate values for a parameter detected.

### -1102 MANDATORY\_PARAM\_EMPTY\_OR\_MALFORMED[​](/docs/binance-spot-api-docs/errors#-1102-mandatory_param_empty_or_malformed) ###

* A mandatory parameter was not sent, was empty/null, or malformed.
* Mandatory parameter '%s' was not sent, was empty/null, or malformed.
* Param '%s' or '%s' must be sent, but both were empty/null!
* '%s' contains unexpected value. Cannot be greater than '%s'.
* Required tag '%s' missing.

### -1103 UNKNOWN\_PARAM[​](/docs/binance-spot-api-docs/errors#-1103-unknown_param) ###

* An unknown parameter was sent.

### -1104 UNREAD\_PARAMETERS[​](/docs/binance-spot-api-docs/errors#-1104-unread_parameters) ###

* Not all sent parameters were read.
* Not all sent parameters were read; read '%s' parameter(s) but was sent '%s'.

### -1105 PARAM\_EMPTY[​](/docs/binance-spot-api-docs/errors#-1105-param_empty) ###

* A parameter was empty.
* Parameter '%s' was empty.

### -1106 PARAM\_NOT\_REQUIRED[​](/docs/binance-spot-api-docs/errors#-1106-param_not_required) ###

* A parameter was sent when not required.
* Parameter '%s' sent when not required.

### -1108 PARAM\_OVERFLOW[​](/docs/binance-spot-api-docs/errors#-1108-param_overflow) ###

* Parameter '%s' overflowed.

### -1111 BAD\_PRECISION[​](/docs/binance-spot-api-docs/errors#-1111-bad_precision) ###

* Parameter '%s' has too much precision.

### -1112 NO\_DEPTH[​](/docs/binance-spot-api-docs/errors#-1112-no_depth) ###

* No orders on book for symbol.

### -1114 TIF\_NOT\_REQUIRED[​](/docs/binance-spot-api-docs/errors#-1114-tif_not_required) ###

* TimeInForce parameter sent when not required.

### -1115 INVALID\_TIF[​](/docs/binance-spot-api-docs/errors#-1115-invalid_tif) ###

* Invalid timeInForce.

### -1116 INVALID\_ORDER\_TYPE[​](/docs/binance-spot-api-docs/errors#-1116-invalid_order_type) ###

* Invalid orderType.

### -1117 INVALID\_SIDE[​](/docs/binance-spot-api-docs/errors#-1117-invalid_side) ###

* Invalid side.

### -1118 EMPTY\_NEW\_CL\_ORD\_ID[​](/docs/binance-spot-api-docs/errors#-1118-empty_new_cl_ord_id) ###

* New client order ID was empty.

### -1119 EMPTY\_ORG\_CL\_ORD\_ID[​](/docs/binance-spot-api-docs/errors#-1119-empty_org_cl_ord_id) ###

* Original client order ID was empty.

### -1120 BAD\_INTERVAL[​](/docs/binance-spot-api-docs/errors#-1120-bad_interval) ###

* Invalid interval.

### -1121 BAD\_SYMBOL[​](/docs/binance-spot-api-docs/errors#-1121-bad_symbol) ###

* Invalid symbol.

### -1122 INVALID\_SYMBOLSTATUS[​](/docs/binance-spot-api-docs/errors#-1122-invalid_symbolstatus) ###

* Invalid symbolStatus.

### -1125 INVALID\_LISTEN\_KEY[​](/docs/binance-spot-api-docs/errors#-1125-invalid_listen_key) ###

* This listenKey does not exist.

### -1127 MORE\_THAN\_XX\_HOURS[​](/docs/binance-spot-api-docs/errors#-1127-more_than_xx_hours) ###

* Lookup interval is too big.
* More than %s hours between startTime and endTime.

### -1128 OPTIONAL\_PARAMS\_BAD\_COMBO[​](/docs/binance-spot-api-docs/errors#-1128-optional_params_bad_combo) ###

* Combination of optional parameters invalid.
* Fields [%s] must be sent together or omitted entirely.
* Invalid 'MDEntryType (269)' combination. BID and OFFER must be requested together.

### -1130 INVALID\_PARAMETER[​](/docs/binance-spot-api-docs/errors#-1130-invalid_parameter) ###

* Invalid data sent for a parameter.
* Data sent for parameter '%s' is not valid.

### -1134 BAD\_STRATEGY\_TYPE[​](/docs/binance-spot-api-docs/errors#-1134-bad_strategy_type) ###

* `strategyType` was less than 1000000.

### -1135 INVALID\_JSON[​](/docs/binance-spot-api-docs/errors#-1135-invalid_json) ###

* Invalid JSON Request
* JSON sent for parameter '%s' is not valid

### -1139 INVALID\_TICKER\_TYPE[​](/docs/binance-spot-api-docs/errors#-1139-invalid_ticker_type) ###

* Invalid ticker type.

### -1145 INVALID\_CANCEL\_RESTRICTIONS[​](/docs/binance-spot-api-docs/errors#-1145-invalid_cancel_restrictions) ###

* `cancelRestrictions` has to be either `ONLY_NEW` or `ONLY_PARTIALLY_FILLED`.

### -1151 DUPLICATE\_SYMBOLS[​](/docs/binance-spot-api-docs/errors#-1151-duplicate_symbols) ###

* Symbol is present multiple times in the list.

### -1152 INVALID\_SBE\_HEADER[​](/docs/binance-spot-api-docs/errors#-1152-invalid_sbe_header) ###

* Invalid `X-MBX-SBE` header; expected `<SCHEMA_ID>:<VERSION>`.

### -1153 UNSUPPORTED\_SCHEMA\_ID[​](/docs/binance-spot-api-docs/errors#-1153-unsupported_schema_id) ###

* Unsupported SBE schema ID or version specified in the `X-MBX-SBE` header.

### -1155 SBE\_DISABLED[​](/docs/binance-spot-api-docs/errors#-1155-sbe_disabled) ###

* SBE is not enabled.

### -1158 OCO\_ORDER\_TYPE\_REJECTED[​](/docs/binance-spot-api-docs/errors#-1158-oco_order_type_rejected) ###

* Order type not supported in OCO.
* If the order type provided in the `aboveType` and/or `belowType` is not supported.

### -1160 OCO\_ICEBERGQTY\_TIMEINFORCE[​](/docs/binance-spot-api-docs/errors#-1160-oco_icebergqty_timeinforce) ###

* Parameter '%s' is not supported if `aboveTimeInForce`/`belowTimeInForce` is not GTC.
* If the order type for the above or below leg is `STOP_LOSS_LIMIT`, and `icebergQty` is provided for that leg, the `timeInForce` has to be `GTC` else it will throw an error.

### -1161 DEPRECATED\_SCHEMA[​](/docs/binance-spot-api-docs/errors#-1161-deprecated_schema) ###

* Unable to encode the response in SBE schema 'x'. Please use schema 'y' or higher.

### -1165 BUY\_OCO\_LIMIT\_MUST\_BE\_BELOW[​](/docs/binance-spot-api-docs/errors#-1165-buy_oco_limit_must_be_below) ###

* A limit order in a buy OCO must be below.

### -1166 SELL\_OCO\_LIMIT\_MUST\_BE\_ABOVE[​](/docs/binance-spot-api-docs/errors#-1166-sell_oco_limit_must_be_above) ###

* A limit order in a sell OCO must be above.

### -1168 BOTH\_OCO\_ORDERS\_CANNOT\_BE\_LIMIT[​](/docs/binance-spot-api-docs/errors#-1168-both_oco_orders_cannot_be_limit) ###

* At least one OCO order must be contingent.

### -1169 INVALID\_TAG\_NUMBER[​](/docs/binance-spot-api-docs/errors#-1169-invalid_tag_number) ###

* Invalid tag number.

### -1170 TAG\_NOT\_DEFINED\_IN\_MESSAGE[​](/docs/binance-spot-api-docs/errors#-1170-tag_not_defined_in_message) ###

* Tag '%s' not defined for this message type.

### -1171 TAG\_APPEARS\_MORE\_THAN\_ONCE[​](/docs/binance-spot-api-docs/errors#-1171-tag_appears_more_than_once) ###

* Tag '%s' appears more than once.

### -1172 TAG\_OUT\_OF\_ORDER[​](/docs/binance-spot-api-docs/errors#-1172-tag_out_of_order) ###

* Tag '%s' specified out of required order.

### -1173 GROUP\_FIELDS\_OUT\_OF\_ORDER[​](/docs/binance-spot-api-docs/errors#-1173-group_fields_out_of_order) ###

* Repeating group '%s' fields out of order.

### -1174 INVALID\_COMPONENT[​](/docs/binance-spot-api-docs/errors#-1174-invalid_component) ###

* Component '%s' is incorrectly populated on '%s' order. Recommendation: '%s'

### -1175 RESET\_SEQ\_NUM\_SUPPORT[​](/docs/binance-spot-api-docs/errors#-1175-reset_seq_num_support) ###

* Continuation of sequence numbers to new session is currently unsupported. Sequence numbers must be reset for each new session.

### -1176 ALREADY\_LOGGED\_IN[​](/docs/binance-spot-api-docs/errors#-1176-already_logged_in) ###

* [Logon`<A>`](/docs/binance-spot-api-docs/fix-api#logon-main) should only be sent once.

### -1177 GARBLED\_MESSAGE[​](/docs/binance-spot-api-docs/errors#-1177-garbled_message) ###

* `CheckSum(10)` contains an incorrect value.
* `BeginString (8)` is not the first tag in a message.
* `MsgType (35)` is not the third tag in a message.
* `BodyLength (9)` does not contain the correct byte count.
* Only printable ASCII characters and SOH (Start of Header) are allowed.

### -1178 BAD\_SENDER\_COMPID[​](/docs/binance-spot-api-docs/errors#-1178-bad_sender_compid) ###

* `SenderCompId(49)` contains an incorrect value. The SenderCompID value should not change throughout the lifetime of a session.

### -1179 BAD\_SEQ\_NUM[​](/docs/binance-spot-api-docs/errors#-1179-bad_seq_num) ###

* `MsgSeqNum(34)` contains an unexpected value. Expected: '%d'.

### -1180 EXPECTED\_LOGON[​](/docs/binance-spot-api-docs/errors#-1180-expected_logon) ###

* [Logon`<A>`](/docs/binance-spot-api-docs/fix-api#logon-main) must be the first message in the session.

### -1181 TOO\_MANY\_MESSAGES[​](/docs/binance-spot-api-docs/errors#-1181-too_many_messages) ###

* Too many messages; current limit is '%d' messages per '%s'.

### -1182 PARAMS\_BAD\_COMBO[​](/docs/binance-spot-api-docs/errors#-1182-params_bad_combo) ###

* Conflicting fields: [%s]

### -1183 NOT\_ALLOWED\_IN\_DROP\_COPY\_SESSIONS[​](/docs/binance-spot-api-docs/errors#-1183-not_allowed_in_drop_copy_sessions) ###

* Requested operation is not allowed in DropCopy sessions.

### -1184 DROP\_COPY\_SESSION\_NOT\_ALLOWED[​](/docs/binance-spot-api-docs/errors#-1184-drop_copy_session_not_allowed) ###

* DropCopy sessions are not supported on this server. Please reconnect to a drop copy server.

### -1185 DROP\_COPY\_SESSION\_REQUIRED[​](/docs/binance-spot-api-docs/errors#-1185-drop_copy_session_required) ###

* Only DropCopy sessions are supported on this server. Either reconnect to order entry server or send `DropCopyFlag (9406)` field.

### -1186 NOT\_ALLOWED\_IN\_ORDER\_ENTRY\_SESSIONS[​](/docs/binance-spot-api-docs/errors#-1186-not_allowed_in_order_entry_sessions) ###

* Requested operation is not allowed in order entry sessions.

### -1187 NOT\_ALLOWED\_IN\_MARKET\_DATA\_SESSIONS[​](/docs/binance-spot-api-docs/errors#-1187-not_allowed_in_market_data_sessions) ###

* Requested operation is not allowed in market data sessions.

### -1188 INCORRECT\_NUM\_IN\_GROUP\_COUNT[​](/docs/binance-spot-api-docs/errors#-1188-incorrect_num_in_group_count) ###

* Incorrect NumInGroup count for repeating group '%s'.

### -1189 DUPLICATE\_ENTRIES\_IN\_A\_GROUP[​](/docs/binance-spot-api-docs/errors#-1189-duplicate_entries_in_a_group) ###

* Group '%s' contains duplicate entries.

### -1190 INVALID\_REQUEST\_ID[​](/docs/binance-spot-api-docs/errors#-1190-invalid_request_id) ###

* 'MDReqID (262)' contains a subscription request id that is already in use on this connection.
* 'MDReqID (262)' contains an unsubscription request id that does not match any active subscription.

### -1191 TOO\_MANY\_SUBSCRIPTIONS[​](/docs/binance-spot-api-docs/errors#-1191-too_many_subscriptions) ###

* Too many subscriptions. Connection may create up to '%s' subscriptions at a time.
* Similar subscription is already active on this connection. Symbol='%s', active subscription id: '%s'.

#### -1194 INVALID\_TIME\_UNIT[​](/docs/binance-spot-api-docs/errors#-1194-invalid_time_unit) ####

* Invalid value for time unit; expected either MICROSECOND or MILLISECOND.

### -1196 BUY\_OCO\_STOP\_LOSS\_MUST\_BE\_ABOVE[​](/docs/binance-spot-api-docs/errors#-1196-buy_oco_stop_loss_must_be_above) ###

* A stop loss order in a buy OCO must be above.

### -1197 SELL\_OCO\_STOP\_LOSS\_MUST\_BE\_BELOW[​](/docs/binance-spot-api-docs/errors#-1197-sell_oco_stop_loss_must_be_below) ###

* A stop loss order in a sell OCO must be below.

### -1198 BUY\_OCO\_TAKE\_PROFIT\_MUST\_BE\_BELOW[​](/docs/binance-spot-api-docs/errors#-1198-buy_oco_take_profit_must_be_below) ###

* A take profit order in a buy OCO must be below.

### -1199 SELL\_OCO\_TAKE\_PROFIT\_MUST\_BE\_ABOVE[​](/docs/binance-spot-api-docs/errors#-1199-sell_oco_take_profit_must_be_above) ###

* A take profit order in a sell OCO must be above.

### -2010 NEW\_ORDER\_REJECTED[​](/docs/binance-spot-api-docs/errors#-2010-new_order_rejected) ###

* NEW\_ORDER\_REJECTED

### -2011 CANCEL\_REJECTED[​](/docs/binance-spot-api-docs/errors#-2011-cancel_rejected) ###

* CANCEL\_REJECTED

### -2013 NO\_SUCH\_ORDER[​](/docs/binance-spot-api-docs/errors#-2013-no_such_order) ###

* Order does not exist.

### -2014 BAD\_API\_KEY\_FMT[​](/docs/binance-spot-api-docs/errors#-2014-bad_api_key_fmt) ###

* API-key format invalid.

### -2015 REJECTED\_MBX\_KEY[​](/docs/binance-spot-api-docs/errors#-2015-rejected_mbx_key) ###

* Invalid API-key, IP, or permissions for action.

### -2016 NO\_TRADING\_WINDOW[​](/docs/binance-spot-api-docs/errors#-2016-no_trading_window) ###

* No trading window could be found for the symbol. Try ticker/24hrs instead.

### -2026 ORDER\_ARCHIVED[​](/docs/binance-spot-api-docs/errors#-2026-order_archived) ###

* Order was canceled or expired with no executed qty over 90 days ago and has been archived.

[]()

Messages for -1010 ERROR\_MSG\_RECEIVED, -2010 NEW\_ORDER\_REJECTED, and -2011 CANCEL\_REJECTED[​](/docs/binance-spot-api-docs/errors#messages-for--1010-error_msg_received--2010-new_order_rejected-and--2011-cancel_rejected)
----------

This code is sent when an error has been returned by the matching engine.
The following messages which will indicate the specific error:

|                                Error message                                 |                                                                                                               Description                                                                                                               |
|------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                            "Unknown order sent."                             |                                                                              The order (by either `orderId`, `clOrdId`, `origClOrdId`) could not be found.                                                                              |
|                           "Duplicate order sent."                            |                                                                                                    The `clOrdId` is already in use.                                                                                                     |
|                             "Market is closed."                              |                                                                                                       The symbol is not trading.                                                                                                        |
|           "Account has insufficient balance for requested action."           |                                                                                                Not enough funds to complete the action.                                                                                                 |
|              "Market orders are not supported for this symbol."              |                                                                                                 `MARKET` is not enabled on the symbol.                                                                                                  |
|             "Iceberg orders are not supported for this symbol."              |                                                                                               `icebergQty` is not enabled on the symbol.                                                                                                |
|            "Stop loss orders are not supported for this symbol."             |                                                                                                `STOP_LOSS` is not enabled on the symbol.                                                                                                |
|         "Stop loss limit orders are not supported for this symbol."          |                                                                                             `STOP_LOSS_LIMIT` is not enabled on the symbol.                                                                                             |
|           "Take profit orders are not supported for this symbol."            |                                                                                               `TAKE_PROFIT` is not enabled on the symbol.                                                                                               |
|        "Take profit limit orders are not supported for this symbol."         |                                                                                            `TAKE_PROFIT_LIMIT` is not enabled on the symbol.                                                                                            |
|                       "Price \* QTY is zero or less."                        |                                                                                                    `price` \* `quantity` is too low.                                                                                                    |
|                          "IcebergQty exceeds QTY."                           |                                                                                           `icebergQty` must be less than the order quantity.                                                                                            |
|                  "This action is disabled on this account."                  |                                                                                Contact customer support; some actions have been disabled on the account.                                                                                |
|                "This account may not place or cancel orders."                |                                                                                   Contact customer support; the account has trading ability disabled.                                                                                   |
|                       "Unsupported order combination"                        |                                                                       The `orderType`, `timeInForce`, `stopPrice`, and/or `icebergQty` combination isn't allowed.                                                                       |
|                      "Order would trigger immediately."                      |                                                                               The order's stop price is not valid when compared to the last traded price.                                                                               |
|          "Cancel order is invalid. Check origClOrdId and orderId."           |                                                                                               No `origClOrdId` or `orderId` was sent in.                                                                                                |
|                  "Order would immediately match and take."                   |                                                                       `LIMIT_MAKER` order type would immediately match and trade, and not be a pure maker order.                                                                        |
|       "The relationship of the prices for the orders is not correct."        |The prices set in the `OCO` is breaking the Price restrictions.   <br/> For reference:   <br/>`BUY` : `LIMIT_MAKER` `price` \< Last Traded Price \< `stopPrice`   <br/>`SELL` : `LIMIT_MAKER` `price` \> Last Traded Price \> `stopPrice`|
|                "OCO orders are not supported for this symbol"                |                                                                                                   `OCO` is not enabled on the symbol.                                                                                                   |
|       "Quote order qty market orders are not support for this symbol."       |                                                                           `MARKET` orders using the parameter `quoteOrderQty` are not enabled on the symbol.                                                                            |
|          "Trailing stop orders are not supported for this symbol."           |                                                                                       Orders using `trailingDelta` are not enabled on the symbol.                                                                                       |
|           "Order cancel-replace is not supported for this symbol."           |                                                          `POST /api/v3/order/cancelReplace` (REST API) or `order.cancelReplace` (WebSocket API) is not enabled on the symbol.                                                           |
|               "This symbol is not permitted for this account."               |                                                                            Account and symbol do not have the same permissions. (e.g. `SPOT`, `MARGIN`, etc)                                                                            |
|                "This symbol is restricted for this account."                 |                                                               Account is unable to trade on that symbol. (e.g. An `ISOLATED_MARGIN` account cannot place `SPOT` orders.)                                                                |
|             "Order was not canceled due to cancel restrictions."             |             Either `cancelRestrictions` was set to `ONLY_NEW` but the order status was not `NEW`   <br/> or   <br/>`cancelRestrictions` was set to `ONLY_PARTIALLY_FILLED` but the order status was not `PARTIALLY_FILLED`.             |
| "Rest API trading is not enabled." / "WebSocket API trading is not enabled." |                                                                     Order is being placed or a server that is not configured to allow access to `TRADE` endpoints.                                                                      |
|   "Order book liquidity is less than `LOT_SIZE` filter minimum quantity."    |                                             Quote quantity market orders cannot be placed when the order book liquidity is less than minimum quantity configured for the `LOT_SIZE` filter.                                             |
|"Order book liquidity is less than `MARKET_LOT_SIZE` filter minimum quantity."|                                               Quote quantity market orders cannot be placed when the order book liquidity is less than the minimum quantity for `MARKET_LOT_SIZE` filter.                                               |
|         "Order book liquidity is less than symbol minimum quantity."         |                                                                           Quote quantity market orders cannot be placed when there are no orders on the book.                                                                           |

Errors regarding POST /api/v3/order/cancelReplace[​](/docs/binance-spot-api-docs/errors#errors-regarding-post-apiv3ordercancelreplace)
----------

### -2021 Order cancel-replace partially failed[​](/docs/binance-spot-api-docs/errors#-2021-order-cancel-replace-partially-failed) ###

* This code is sent when either the cancellation of the order failed or the new order placement failed but not both.

### -2022 Order cancel-replace failed.[​](/docs/binance-spot-api-docs/errors#-2022-order-cancel-replace-failed) ###

* This code is sent when both the cancellation of the order failed and the new order placement failed.

[]()

Filter failures[​](/docs/binance-spot-api-docs/errors#filter-failures)
----------

|                    Error message                    |                                                                                       Description                                                                                       |
|-----------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|           "Filter failure: PRICE\_FILTER"           |                                                  `price` is too high, too low, and/or not following the tick size rule for the symbol.                                                  |
|          "Filter failure: PERCENT\_PRICE"           |                                              `price` is X% too high or X% too low from the average weighted price over the last Y minutes.                                              |
|             "Filter failure: LOT\_SIZE"             |                                                `quantity` is too high, too low, and/or not following the step size rule for the symbol.                                                 |
|           "Filter failure: MIN\_NOTIONAL"           |                                                          `price` \* `quantity` is too low to be a valid order for the symbol.                                                           |
|             "Filter failure: NOTIONAL"              |                                                    `price` \* `quantity` is not within range of the `minNotional` and `maxNotional`                                                     |
|          "Filter failure: ICEBERG\_PARTS"           |                                                        `ICEBERG` order would break into too many parts; icebergQty is too small.                                                        |
|         "Filter failure: MARKET\_LOT\_SIZE"         |                                        `MARKET` order's `quantity` is too high, too low, and/or not following the step size rule for the symbol.                                        |
|           "Filter failure: MAX\_POSITION"           |The account's position has reached the maximum defined limit.   <br/> This is composed of the sum of the balance of the base asset, and the sum of the quantity of all open `BUY` orders.|
|         "Filter failure: MAX\_NUM\_ORDERS"          |                                                                     Account has too many open orders on the symbol.                                                                     |
|      "Filter failure: MAX\_NUM\_ALGO\_ORDERS"       |                                                      Account has too many open stop loss and/or take profit orders on the symbol.                                                       |
|     "Filter failure: MAX\_NUM\_ICEBERG\_ORDERS"     |                                                                 Account has too many open iceberg orders on the symbol.                                                                 |
|          "Filter failure: TRAILING\_DELTA"          |                                                   `trailingDelta` is not within the defined range of the filter for that order type.                                                    |
|    "Filter failure: EXCHANGE\_MAX\_NUM\_ORDERS"     |                                                                    Account has too many open orders on the exchange.                                                                    |
| "Filter failure: EXCHANGE\_MAX\_NUM\_ALGO\_ORDERS"  |                                                     Account has too many open stop loss and/or take profit orders on the exchange.                                                      |
|"Filter failure: EXCHANGE\_MAX\_NUM\_ICEBERG\_ORDERS"|                                                                Account has too many open iceberg orders on the exchange.                                                                |

