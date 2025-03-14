# Binance Spot SBE API Documentation

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

## SBE API

Content not found.

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

