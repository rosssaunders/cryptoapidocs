# Binance Spot Public Websocket API Documentation

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

## WEB SOCKET STREAMS

Web Socket Streams for Binance (2025-01-28)
==========

General WSS information[​](/docs/binance-spot-api-docs/web-socket-streams#general-wss-information)
----------

* The base endpoint is: **wss://stream.binance.com:9443** or **wss://stream.binance.com:443**.
* Streams can be accessed either in a single raw stream or in a combined stream.
* Raw streams are accessed at **/ws/\<streamName\>**
* Combined streams are accessed at **/stream?streams=\<streamName1\>/\<streamName2\>/\<streamName3\>**
* Combined stream events are wrapped as follows: **{"stream":"\<streamName\>","data":\<rawPayload\>}**
* All symbols for streams are **lowercase**
* A single connection to **stream.binance.com** is only valid for 24 hours; expect to be disconnected at the 24 hour mark
* The websocket server will send a `ping frame` every 20 seconds.
  * If the websocket server does not receive a `pong frame` back from the connection within a minute the connection will be disconnected.
  * When you receive a ping, you must send a pong with a copy of ping's payload as soon as possible.
  * Unsolicited `pong frames` are allowed, but will not prevent disconnection. **It is recommended that the payload for these pong frames are empty.**

* The base endpoint **wss://data-stream.binance.vision** can be subscribed to receive **only** market data messages.   
   User data stream is **NOT** available from this URL.
* All time and timestamp related fields are **milliseconds by default**. To receive the information in microseconds, please add the parameter `timeUnit=MICROSECOND or timeUnit=microsecond` in the URL.
  * For example: `/stream?streams=btcusdt@trade&timeUnit=MICROSECOND`

Websocket Limits[​](/docs/binance-spot-api-docs/web-socket-streams#websocket-limits)
----------

* WebSocket connections have a limit of 5 incoming messages per second. A message is considered:
  * A PING frame
  * A PONG frame
  * A JSON controlled message (e.g. subscribe, unsubscribe)

* A connection that goes beyond the limit will be disconnected; IPs that are repeatedly disconnected may be banned.
* A single connection can listen to a maximum of 1024 streams.
* There is a limit of **300 connections per attempt every 5 minutes per IP**.

Live Subscribing/Unsubscribing to streams[​](/docs/binance-spot-api-docs/web-socket-streams#live-subscribingunsubscribing-to-streams)
----------

* The following data can be sent through the websocket instance in order to subscribe/unsubscribe from streams. Examples can be seen below.
* The `id` is used as an identifier to uniquely identify the messages going back and forth. The following formats are accepted:
  * 64-bit signed integer
  * alphanumeric strings; max length 36
  * `null`

* In the response, if the `result` received is `null` this means the request sent was a success for non-query requests (e.g. Subscribing/Unsubscribing).

### Subscribe to a stream[​](/docs/binance-spot-api-docs/web-socket-streams#subscribe-to-a-stream) ###

* Request

  ```
  {  "method": "SUBSCRIBE",  "params": [    "btcusdt@aggTrade",    "btcusdt@depth"  ],  "id": 1}
  ```

* Response

  ```
  {  "result": null,  "id": 1}
  ```

### Unsubscribe to a stream[​](/docs/binance-spot-api-docs/web-socket-streams#unsubscribe-to-a-stream) ###

* Request

  ```
  {  "method": "UNSUBSCRIBE",  "params": [    "btcusdt@depth"  ],  "id": 312}
  ```

* Response

  ```
  {  "result": null,  "id": 312}
  ```

### Listing Subscriptions[​](/docs/binance-spot-api-docs/web-socket-streams#listing-subscriptions) ###

* Request

  ```
  {  "method": "LIST_SUBSCRIPTIONS",  "id": 3}
  ```

* Response

  ```
  {  "result": [    "btcusdt@aggTrade"  ],  "id": 3}
  ```

### Setting Properties[​](/docs/binance-spot-api-docs/web-socket-streams#setting-properties) ###

Currently, the only property that can be set is whether `combined` stream payloads are enabled or not.
The combined property is set to `false` when connecting using `/ws/` ("raw streams") and `true` when connecting using `/stream/`.

* Request

  ```
  {  "method": "SET_PROPERTY",  "params": [    "combined",    true  ],  "id": 5}
  ```

* Response

  ```
  {  "result": null,  "id": 5}
  ```

### Retrieving Properties[​](/docs/binance-spot-api-docs/web-socket-streams#retrieving-properties) ###

* Request

  ```
  {  "method": "GET_PROPERTY",  "params": [    "combined"  ],  "id": 2}
  ```

* Response

  ```
  {  "result": true, // Indicates that combined is set to true.  "id": 2}
  ```

### Error Messages[​](/docs/binance-spot-api-docs/web-socket-streams#error-messages) ###

|                                                                                 Error Message                                                                                 |                                            Description                                             |
|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|
|                                                                {"code": 0, "msg": "Unknown property","id": %s}                                                                |                 Parameter used in the `SET_PROPERTY` or `GET_PROPERTY` was invalid                 |
|                                                          {"code": 1, "msg": "Invalid value type: expected Boolean"}                                                           |                               Value should only be `true` or `false`                               |
|                                                     {"code": 2, "msg": "Invalid request: property name must be a string"}                                                     |                                 Property name provided was invalid                                 |
|                                                 {"code": 2, "msg": "Invalid request: request ID must be an unsigned integer"}                                                 |Parameter `id` had to be provided or the value provided in the `id` parameter is an unsupported type|
|{"code": 2, "msg": "Invalid request: unknown variant %s, expected one of `SUBSCRIBE`, `UNSUBSCRIBE`, `LIST_SUBSCRIPTIONS`, `SET_PROPERTY`, `GET_PROPERTY` at line 1 column 28"}|     Possible typo in the provided method or provided method was neither of the expected values     |
|                                                          {"code": 2, "msg": "Invalid request: too many parameters"}                                                           |                            Unnecessary parameters provided in the data                             |
|                                                     {"code": 2, "msg": "Invalid request: property name must be a string"}                                                     |                                   Property name was not provided                                   |
|                                               {"code": 2, "msg": "Invalid request: missing field `method` at line 1 column 73"}                                               |                               `method` was not provided in the data                                |
|                                                     {"code":3,"msg":"Invalid JSON: expected value at line %s column %s"}                                                      |                                JSON data sent has incorrect syntax.                                |

Detailed Stream information
==========

Aggregate Trade Streams[​](/docs/binance-spot-api-docs/web-socket-streams#aggregate-trade-streams)
----------

The Aggregate Trade Streams push trade information that is aggregated for a single taker order.

**Stream Name:** \<symbol\>@aggTrade

**Update Speed:** Real-time

**Payload:**

```
{  "e": "aggTrade",    // Event type  "E": 1672515782136, // Event time  "s": "BNBBTC",      // Symbol  "a": 12345,         // Aggregate trade ID  "p": "0.001",       // Price  "q": "100",         // Quantity  "f": 100,           // First trade ID  "l": 105,           // Last trade ID  "T": 1672515782136, // Trade time  "m": true,          // Is the buyer the market maker?  "M": true           // Ignore}
```

Trade Streams[​](/docs/binance-spot-api-docs/web-socket-streams#trade-streams)
----------

The Trade Streams push raw trade information; each trade has a unique buyer and seller.

**Stream Name:** \<symbol\>@trade

**Update Speed:** Real-time

**Payload:**

```
{  "e": "trade",       // Event type  "E": 1672515782136, // Event time  "s": "BNBBTC",      // Symbol  "t": 12345,         // Trade ID  "p": "0.001",       // Price  "q": "100",         // Quantity  "T": 1672515782136, // Trade time  "m": true,          // Is the buyer the market maker?  "M": true           // Ignore}
```

Kline/Candlestick Streams for UTC[​](/docs/binance-spot-api-docs/web-socket-streams#klinecandlestick-streams-for-utc)
----------

The Kline/Candlestick Stream push updates to the current klines/candlestick every second in `UTC+0` timezone

[]()**Kline/Candlestick chart intervals:**

s-\> seconds; m -\> minutes; h -\> hours; d -\> days; w -\> weeks; M -\> months

* 1s
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

**Stream Name:** \<symbol\>@kline\_\<interval\>

**Update Speed:** 1000ms for `1s`, 2000ms for the other intervals

**Payload:**

```
{  "e": "kline",         // Event type  "E": 1672515782136,   // Event time  "s": "BNBBTC",        // Symbol  "k": {    "t": 1672515780000, // Kline start time    "T": 1672515839999, // Kline close time    "s": "BNBBTC",      // Symbol    "i": "1m",          // Interval    "f": 100,           // First trade ID    "L": 200,           // Last trade ID    "o": "0.0010",      // Open price    "c": "0.0020",      // Close price    "h": "0.0025",      // High price    "l": "0.0015",      // Low price    "v": "1000",        // Base asset volume    "n": 100,           // Number of trades    "x": false,         // Is this kline closed?    "q": "1.0000",      // Quote asset volume    "V": "500",         // Taker buy base asset volume    "Q": "0.500",       // Taker buy quote asset volume    "B": "123456"       // Ignore  }}
```

Kline/Candlestick Streams with timezone offset[​](/docs/binance-spot-api-docs/web-socket-streams#klinecandlestick-streams-with-timezone-offset)
----------

The Kline/Candlestick Stream push updates to the current klines/candlestick every second in `UTC+8` timezone

**Kline/Candlestick chart intervals:**

Supported intervals: See [`Kline/Candlestick chart intervals`](/docs/binance-spot-api-docs/web-socket-streams#kline-intervals)

**UTC+8 timezone offset:**

* Kline intervals open and close in the `UTC+8` timezone. For example the `1d` klines will open at the beginning of the `UTC+8` day, and close at the end of the `UTC+8` day.
* Note that `E` (event time), `t` (start time) and `T` (close time) in the payload are Unix timestamps, which are always interpreted in UTC.

**Stream Name:** \<symbol\>@kline\_\<interval\>@+08:00

**Update Speed:** 1000ms for `1s`, 2000ms for the other intervals

**Payload:**

```
{  "e": "kline",         // Event type  "E": 1672515782136,   // Event time  "s": "BNBBTC",        // Symbol  "k": {    "t": 1672515780000, // Kline start time    "T": 1672515839999, // Kline close time    "s": "BNBBTC",      // Symbol    "i": "1m",          // Interval    "f": 100,           // First trade ID    "L": 200,           // Last trade ID    "o": "0.0010",      // Open price    "c": "0.0020",      // Close price    "h": "0.0025",      // High price    "l": "0.0015",      // Low price    "v": "1000",        // Base asset volume    "n": 100,           // Number of trades    "x": false,         // Is this kline closed?    "q": "1.0000",      // Quote asset volume    "V": "500",         // Taker buy base asset volume    "Q": "0.500",       // Taker buy quote asset volume    "B": "123456"       // Ignore  }}
```

Individual Symbol Mini Ticker Stream[​](/docs/binance-spot-api-docs/web-socket-streams#individual-symbol-mini-ticker-stream)
----------

24hr rolling window mini-ticker statistics. These are NOT the statistics of the UTC day, but a 24hr rolling window for the previous 24hrs.

**Stream Name:** \<symbol\>@miniTicker

**Update Speed:** 1000ms

**Payload:**

```
  {    "e": "24hrMiniTicker",  // Event type    "E": 1672515782136,     // Event time    "s": "BNBBTC",          // Symbol    "c": "0.0025",          // Close price    "o": "0.0010",          // Open price    "h": "0.0025",          // High price    "l": "0.0010",          // Low price    "v": "10000",           // Total traded base asset volume    "q": "18"               // Total traded quote asset volume  }
```

All Market Mini Tickers Stream[​](/docs/binance-spot-api-docs/web-socket-streams#all-market-mini-tickers-stream)
----------

24hr rolling window mini-ticker statistics for all symbols that changed in an array. These are NOT the statistics of the UTC day, but a 24hr rolling window for the previous 24hrs. Note that only tickers that have changed will be present in the array.

**Stream Name:** !miniTicker@arr

**Update Speed:** 1000ms

**Payload:**

```
[  {    // Same as <symbol>@miniTicker payload  }]
```

Individual Symbol Ticker Streams[​](/docs/binance-spot-api-docs/web-socket-streams#individual-symbol-ticker-streams)
----------

24hr rolling window ticker statistics for a single symbol. These are NOT the statistics of the UTC day, but a 24hr rolling window for the previous 24hrs.

**Stream Name:** \<symbol\>@ticker

**Update Speed:** 1000ms

**Payload:**

```
{  "e": "24hrTicker",  // Event type  "E": 1672515782136, // Event time  "s": "BNBBTC",      // Symbol  "p": "0.0015",      // Price change  "P": "250.00",      // Price change percent  "w": "0.0018",      // Weighted average price  "x": "0.0009",      // First trade(F)-1 price (first trade before the 24hr rolling window)  "c": "0.0025",      // Last price  "Q": "10",          // Last quantity  "b": "0.0024",      // Best bid price  "B": "10",          // Best bid quantity  "a": "0.0026",      // Best ask price  "A": "100",         // Best ask quantity  "o": "0.0010",      // Open price  "h": "0.0025",      // High price  "l": "0.0010",      // Low price  "v": "10000",       // Total traded base asset volume  "q": "18",          // Total traded quote asset volume  "O": 0,             // Statistics open time  "C": 86400000,      // Statistics close time  "F": 0,             // First trade ID  "L": 18150,         // Last trade Id  "n": 18151          // Total number of trades}
```

All Market Tickers Stream[​](/docs/binance-spot-api-docs/web-socket-streams#all-market-tickers-stream)
----------

24hr rolling window ticker statistics for all symbols that changed in an array. These are NOT the statistics of the UTC day, but a 24hr rolling window for the previous 24hrs. Note that only tickers that have changed will be present in the array.

**Stream Name:** !ticker@arr

**Update Speed:** 1000ms

**Payload:**

```
[  {    // Same as <symbol>@ticker payload  }]
```

Individual Symbol Rolling Window Statistics Streams[​](/docs/binance-spot-api-docs/web-socket-streams#individual-symbol-rolling-window-statistics-streams)
----------

Rolling window ticker statistics for a single symbol, computed over multiple windows.

**Stream Name:** \<symbol\>@ticker\_\<window\_size\>

**Window Sizes:** 1h,4h,1d

**Update Speed:** 1000ms

**Note**: This stream is different from the \<symbol\>@ticker stream.
The open time `"O"` always starts on a minute, while the closing time `"C"` is the current time of the update.
As such, the effective window might be up to 59999ms wider than \<window\_size\>.

**Payload:**

```
{  "e": "1hTicker",    // Event type  "E": 1672515782136, // Event time  "s": "BNBBTC",      // Symbol  "p": "0.0015",      // Price change  "P": "250.00",      // Price change percent  "o": "0.0010",      // Open price  "h": "0.0025",      // High price  "l": "0.0010",      // Low price  "c": "0.0025",      // Last price  "w": "0.0018",      // Weighted average price  "v": "10000",       // Total traded base asset volume  "q": "18",          // Total traded quote asset volume  "O": 0,             // Statistics open time  "C": 1675216573749, // Statistics close time  "F": 0,             // First trade ID  "L": 18150,         // Last trade Id  "n": 18151          // Total number of trades}
```

All Market Rolling Window Statistics Streams[​](/docs/binance-spot-api-docs/web-socket-streams#all-market-rolling-window-statistics-streams)
----------

Rolling window ticker statistics for all market symbols, computed over multiple windows.
Note that only tickers that have changed will be present in the array.

**Stream Name:** !ticker\_\<window-size\>@arr

**Window Size:** 1h,4h,1d

**Update Speed:** 1000ms

**Payload:**

```
[  {    // Same as <symbol>@ticker_<window_size> payload,    // one for each symbol updated within the interval.  }]
```

Individual Symbol Book Ticker Streams[​](/docs/binance-spot-api-docs/web-socket-streams#individual-symbol-book-ticker-streams)
----------

Pushes any update to the best bid or ask's price or quantity in real-time for a specified symbol.
Multiple `<symbol>@bookTicker` streams can be subscribed to over one connection.

**Stream Name:** \<symbol\>@bookTicker

**Update Speed:** Real-time

**Payload:**

```
{  "u":400900217,     // order book updateId  "s":"BNBUSDT",     // symbol  "b":"25.35190000", // best bid price  "B":"31.21000000", // best bid qty  "a":"25.36520000", // best ask price  "A":"40.66000000"  // best ask qty}
```

Average Price[​](/docs/binance-spot-api-docs/web-socket-streams#average-price)
----------

Average price streams push changes in the average price over a fixed time interval.

**Stream Name:** \<symbol\>@avgPrice

**Update Speed:** 1000ms

**Payload:**

```
{  "e": "avgPrice",          // Event type  "E": 1693907033000,       // Event time  "s": "BTCUSDT",           // Symbol  "i": "5m",                // Average price interval  "w": "25776.86000000",    // Average price  "T": 1693907032213        // Last trade time}
```

Partial Book Depth Streams[​](/docs/binance-spot-api-docs/web-socket-streams#partial-book-depth-streams)
----------

Top **\<levels\>** bids and asks, pushed every second. Valid **\<levels\>** are 5, 10, or 20.

**Stream Names:** \<symbol\>@depth\<levels\> OR \<symbol\>@depth\<levels\>@100ms

**Update Speed:** 1000ms or 100ms

**Payload:**

```
{  "lastUpdateId": 160,  // Last update ID  "bids": [             // Bids to be updated    [      "0.0024",         // Price level to be updated      "10"              // Quantity    ]  ],  "asks": [             // Asks to be updated    [      "0.0026",         // Price level to be updated      "100"             // Quantity    ]  ]}
```

Diff. Depth Stream[​](/docs/binance-spot-api-docs/web-socket-streams#diff-depth-stream)
----------

Order book price and quantity depth updates used to locally manage an order book.

**Stream Name:** \<symbol\>@depth OR \<symbol\>@depth@100ms

**Update Speed:** 1000ms or 100ms

**Payload:**

```
{  "e": "depthUpdate", // Event type  "E": 1672515782136, // Event time  "s": "BNBBTC",      // Symbol  "U": 157,           // First update ID in event  "u": 160,           // Final update ID in event  "b": [              // Bids to be updated    [      "0.0024",       // Price level to be updated      "10"            // Quantity    ]  ],  "a": [              // Asks to be updated    [      "0.0026",       // Price level to be updated      "100"           // Quantity    ]  ]}
```

How to manage a local order book correctly[​](/docs/binance-spot-api-docs/web-socket-streams#how-to-manage-a-local-order-book-correctly)
----------

1. Open a WebSocket connection to `wss://stream.binance.com:9443/ws/bnbbtc@depth`.
2. Buffer the events received from the stream. Note the `U` of the first event you received.
3. Get a depth snapshot from `https://api.binance.com/api/v3/depth?symbol=BNBBTC&limit=5000`.
4. If the `lastUpdateId` from the snapshot is strictly less than the `U` from step 2, go back to step 3.
5. In the buffered events, discard any event where `u` is \<= `lastUpdateId` of the snapshot. The first buffered event should now have `lastUpdateId` within its `[U;u]` range.
6. Set your local order book to the snapshot. Its update ID is `lastUpdateId`.
7. Apply the update procedure below to all buffered events, and then to all subsequent events received.

To apply an event to your local order book, follow this update procedure:

1. If the event `u` (last update ID) is \< the update ID of your local order book, ignore the event.
2. If the event `U` (first update ID) is \> the update ID of your local order book, something went wrong. Discard your local order book and restart the process from the beginning.
3. For each price level in bids (`b`) and asks (`a`), set the new quantity in the order book:
   * If the price level does not exist in the order book, insert it with new quantity.
   * If the quantity is zero, remove the price level from the order book.

4. Set the order book update ID to the last update ID (`u`) in the processed event.

>
>
> [!NOTE]
> Since depth snapshots retrieved from the API have a limit on the number of price levels (5000 on each side maximum), you won't learn the quantities for the levels outside of the initial snapshot unless they change.   
> So be careful when using the information for those levels, since they might not reflect the full view of the order book.   
> However, for most use cases, seeing 5000 levels on each side is enough to understand the market and trade effectively.
>
>

## GENERAL API INFORMATION

General API Information
==========

* The base endpoint is: **`wss://ws-api.binance.com:443/ws-api/v3`**.
  * If you experience issues with the standard 443 port, alternative port 9443 is also available.
  * The base endpoint for [testnet](https://testnet.binance.vision/) is: `wss://testnet.binance.vision/ws-api/v3`

* A single connection to the API is only valid for 24 hours; expect to be disconnected after the 24-hour mark.
* The websocket server will send a `ping frame` every 20 seconds.
  * If the websocket server does not receive a `pong frame` back from the connection within a minute the connection will be disconnected.
  * When you receive a ping, you must send a pong with a copy of ping's payload as soon as possible.
  * Unsolicited `pong frames` are allowed, but will not prevent disconnection. **It is recommended that the payload for these pong frames are empty.**

* Lists are returned in **chronological order**, unless noted otherwise.
* All timestamps in the JSON responses are in **milliseconds in UTC by default**. To receive the information in microseconds, please add the parameter `timeUnit=MICROSECOND` or `timeUnit=microsecond` in the URL.
* Timestamp parameters (e.g. `startTime`, `endTime`, `timestamp)` can be passed in milliseconds or microseconds.
* All field names and values are **case-sensitive**, unless noted otherwise.

## REQUEST FORMAT

Request format
==========

Requests must be sent as JSON in **text frames**, one request per frame.

Example of request:

```
{  "id": "e2a85d9f-07a5-4f94-8d5f-789dc3deb097",  "method": "order.place",  "params": {    "symbol": "BTCUSDT",    "side": "BUY",    "type": "LIMIT",    "price": "0.1",    "quantity": "10",    "timeInForce": "GTC",    "timestamp": 1655716096498,    "apiKey": "T59MTDLWlpRW16JVeZ2Nju5A5C98WkMm8CSzWC4oqynUlTm1zXOxyauT8LmwXEv9",    "signature": "5942ad337e6779f2f4c62cd1c26dba71c91514400a24990a3e7f5edec9323f90"  }}
```

Request fields:

|  Name  |        Type         |Mandatory|                         Description                         |
|--------|---------------------|---------|-------------------------------------------------------------|
|  `id`  |INT / STRING / `null`|   YES   |      Arbitrary ID used to match responses to requests       |
|`method`|       STRING        |   YES   |                     Request method name                     |
|`params`|       OBJECT        |   NO    |Request parameters. May be omitted if there are no parameters|

* Request `id` is truly arbitrary. You can use UUIDs, sequential IDs, current timestamp, etc.
  The server does not interpret `id` in any way, simply echoing it back in the response.

  You can freely reuse IDs within a session.
  However, be careful to not send more than one request at a time with the same ID,
  since otherwise it might be impossible to tell the responses apart.

* Request method names may be prefixed with explicit version: e.g., `"v3/order.place"`.

* The order of `params` is not significant.

## RESPONSE FORMAT

Response format
==========

Responses are returned as JSON in **text frames**, one response per frame.

Example of successful response:

```
{  "id": "e2a85d9f-07a5-4f94-8d5f-789dc3deb097",  "status": 200,  "result": {    "symbol": "BTCUSDT",    "orderId": 12510053279,    "orderListId": -1,    "clientOrderId": "a097fe6304b20a7e4fc436",    "transactTime": 1655716096505,    "price": "0.10000000",    "origQty": "10.00000000",    "executedQty": "0.00000000",    "origQuoteOrderQty": "0.000000",    "cummulativeQuoteQty": "0.00000000",    "status": "NEW",    "timeInForce": "GTC",    "type": "LIMIT",    "side": "BUY",    "workingTime": 1655716096505,    "selfTradePreventionMode": "NONE"  },  "rateLimits": [    {      "rateLimitType": "ORDERS",      "interval": "SECOND",      "intervalNum": 10,      "limit": 50,      "count": 12    },    {      "rateLimitType": "ORDERS",      "interval": "DAY",      "intervalNum": 1,      "limit": 160000,      "count": 4043    },    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 321    }  ]}
```

Example of failed response:

```
{  "id": "e2a85d9f-07a5-4f94-8d5f-789dc3deb097",  "status": 400,  "error": {    "code": -2010,    "msg": "Account has insufficient balance for requested action."  },  "rateLimits": [    {      "rateLimitType": "ORDERS",      "interval": "SECOND",      "intervalNum": 10,      "limit": 50,      "count": 13    },    {      "rateLimitType": "ORDERS",      "interval": "DAY",      "intervalNum": 1,      "limit": 160000,      "count": 4044    },    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 322    }  ]}
```

Response fields:

|    Name    |        Type         |                 Mandatory                  |                                                  Description                                                  |
|------------|---------------------|--------------------------------------------|---------------------------------------------------------------------------------------------------------------|
|    `id`    |INT / STRING / `null`|                    YES                     |                                        Same as in the original request                                        |
|  `status`  |         INT         |                    YES                     |Response status. See [Status codes](/docs/binance-spot-api-docs/web-socket-api/response-format.md#status-codes)|
|  `result`  |   OBJECT / ARRAY    |                    YES                     |                                Response content. Present if request succeeded                                 |
|  `error`   |       OBJECT        |Error description. Present if request failed|                                                                                                               |
|`rateLimits`|        ARRAY        |                     NO                     |Rate limiting status. See [Rate limits](/docs/binance-spot-api-docs/web-socket-api/response-format#rate-limits)|

### Status codes[​](/docs/binance-spot-api-docs/web-socket-api/response-format#status-codes) ###

Status codes in the `status` field are the same as in HTTP.

Here are some common status codes that you might encounter:

* `200` indicates a successful response.
* `4XX` status codes indicate invalid requests; the issue is on your side.
  * `400` – your request failed, see `error` for the reason.
  * `403` – you have been blocked by the Web Application Firewall.
  * `409` – your request partially failed but also partially succeeded, see `error` for details.
  * `418` – you have been auto-banned for repeated violation of rate limits.
  * `429` – you have exceeded API request rate limit, please slow down.

* `5XX` status codes indicate internal errors; the issue is on Binance's side.
  * **Important:** If a response contains 5xx status code, it **does not** necessarily mean that your request has failed.
    Execution status is *unknown* and the request might have actually succeeded.
    Please use query methods to confirm the status.
    You might also want to establish a new WebSocket connection for that.

See [Error codes for Binance](/docs/binance-spot-api-docs/errors) for a list of error codes and messages.

## EVENT FORMAT

Event format
==========

User Data Stream events for non-SBE sessions are sent as JSON in **text frames**, one event per frame.

Events in SBE sessions will be sent as **binary frames.**

Please refer to [`userDataStream.subscribe`](/docs/binance-spot-api-docs/web-socket-api/user-data-stream-requests#user_data_stream_susbcribe) for details on how to subscribe to User Data Stream in WebSocket API.

Example of an event:

```
{  "event": {    "e": "outboundAccountPosition",    "E": 1728972148778,    "u": 1728972148778,    "B": [      {        "a": "ABC",        "f": "11818.00000000",        "l": "182.00000000"      },      {        "a": "DEF",        "f": "10580.00000000",        "l": "70.00000000"      }    ]  }}
```

Event fields:

| Name  | Type |Mandatory|                                    Description                                     |
|-------|------|---------|------------------------------------------------------------------------------------|
|`event`|OBJECT|   YES   |Event payload. See [User Data Streams](/docs/binance-spot-api-docs/user-data-stream)|

## RATE LIMITS

Rate limits
==========

### Connection limits[​](/docs/binance-spot-api-docs/web-socket-api/rate-limits#connection-limits) ###

There is a limit of **300 connections per attempt every 5 minutes**.

The connection is per **IP address**.

### General information on rate limits[​](/docs/binance-spot-api-docs/web-socket-api/rate-limits#general-information-on-rate-limits) ###

* Current API rate limits can be queried using the [`exchangeInfo`](/docs/binance-spot-api-docs/web-socket-api/general-requests#exchange-information) request.
* There are multiple rate limit types across multiple intervals.
* Responses can indicate current rate limit status in the optional `rateLimits` field.
* Requests fail with status `429` when unfilled order count or request rate limits are violated.

#### How to interpret rate limits[​](/docs/binance-spot-api-docs/web-socket-api/rate-limits#how-to-interpret-rate-limits) ####

A response with rate limit status may look like this:

```
{  "id": "7069b743-f477-4ae3-81db-db9b8df085d2",  "status": 200,  "result": {    "serverTime": 1656400526260  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 70    }  ]}
```

The `rateLimits` array describes all currently active rate limits affected by the request.

|     Name      |Type|Mandatory|                     Description                      |
|---------------|----|---------|------------------------------------------------------|
|`rateLimitType`|ENUM|   YES   |     Rate limit type: `REQUEST_WEIGHT`, `ORDERS`      |
|  `interval`   |ENUM|   YES   |Rate limit interval: `SECOND`, `MINUTE`, `HOUR`, `DAY`|
| `intervalNum` |INT |   YES   |            Rate limit interval multiplier            |
|    `limit`    |INT |   YES   |              Request limit per interval              |
|    `count`    |INT |   YES   |              Current usage per interval              |

Rate limits are accounted by intervals.

For example, a `1 MINUTE` interval starts every minute.
Request submitted at 00:01:23.456 counts towards the 00:01:00 minute's limit.
Once the 00:02:00 minute starts, the count will reset to zero again.

Other intervals behave in a similar manner.
For example, `1 DAY` rate limit resets at 00:00 UTC every day,
and `10 SECOND` interval resets at 00, 10, 20... seconds of each minute.

APIs have multiple rate-limiting intervals.
If you exhaust a shorter interval but the longer interval still allows requests,
you will have to wait for the shorter interval to expire and reset.
If you exhaust a longer interval, you will have to wait for that interval to reset,
even if shorter rate limit count is zero.

#### How to show/hide rate limit information[​](/docs/binance-spot-api-docs/web-socket-api/rate-limits#how-to-showhide-rate-limit-information) ####

`rateLimits` field is included with every response by default.

However, rate limit information can be quite bulky.
If you are not interested in detailed rate limit status of every request,
the `rateLimits` field can be omitted from responses to reduce their size.

* Optional `returnRateLimits` boolean parameter in request.

  Use `returnRateLimits` parameter to control whether to include `rateLimits` fields in response to individual requests.

  Default request and response:

  ```
  {"id":1,"method":"time"}
  ```

  ```
  {"id":1,"status":200,"result":{"serverTime":1656400526260},"rateLimits":[{"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":6000,"count":70}]}
  ```

  Request and response without rate limit status:

  ```
  {"id":2,"method":"time","params":{"returnRateLimits":false}}
  ```

  ```
  {"id":2,"status":200,"result":{"serverTime":1656400527891}}
  ```

* Optional `returnRateLimits` boolean parameter in connection URL.

  If you wish to omit `rateLimits` from all responses by default,
  use `returnRateLimits` parameter in the query string instead:

  ```
  wss://ws-api.binance.com:443/ws-api/v3?returnRateLimits=false
  ```

  This will make all requests made through this connection behave as if you have passed `"returnRateLimits": false`.

  If you *want* to see rate limits for a particular request,
  you need to explicitly pass the `"returnRateLimits": true` parameter.

**Note:** Your requests are still rate limited if you hide the `rateLimits` field in responses.

### IP limits[​](/docs/binance-spot-api-docs/web-socket-api/rate-limits#ip-limits) ###

* Every request has a certain **weight**, added to your limit as you perform requests.
  * The heavier the request (e.g. querying data from multiple symbols), the more weight the request will cost.
  * Connecting to WebSocket API costs 2 weight.

* Current weight usage is indicated by the `REQUEST_WEIGHT` rate limit type.
* Use the [`exchangeInfo`](/docs/binance-spot-api-docs/web-socket-api/general-requests#exchange-information) request
  to keep track of the current weight limits.
* Weight is accumulated **per IP address** and is shared by all connections from that address.
* If you go over the weight limit, requests fail with status `429`.
  * This status code indicates you should back off and stop spamming the API.
  * Rate-limited responses include a `retryAfter` field, indicating when you can retry the request.

* **Repeatedly violating rate limits and/or failing to back off after receiving 429s will result in an automated IP ban and you will be disconnected.**
  * Requests from a banned IP address fail with status `418`.
  * `retryAfter` field indicates the timestamp when the ban will be lifted.

* IP bans are tracked and **scale in duration** for repeat offenders, **from 2 minutes to 3 days**.

Successful response indicating that in 1 minute you have used 70 weight out of your 6000 limit:

```
{  "id": "7069b743-f477-4ae3-81db-db9b8df085d2",  "status": 200,  "result": [],  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 70    }  ]}
```

Failed response indicating that you are banned and the ban will last until epoch `1659146400000`:

```
{  "id": "fc93a61a-a192-4cf4-bb2a-a8f0f0c51e06",  "status": 418,  "error": {    "code": -1003,    "msg": "Way too much request weight used; IP banned until 1659146400000. Please use WebSocket Streams for live updates to avoid bans.",    "data": {      "serverTime": 1659142907531,      "retryAfter": 1659146400000    }  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 2411    }  ]}
```

### Unfilled Order Count[​](/docs/binance-spot-api-docs/web-socket-api/rate-limits#unfilled-order-count) ###

* Successfully placed orders update the `ORDERS` rate limit type.
* Rejected or unsuccessful orders might or might not update the `ORDERS` rate limit type.
* **Please note that if your orders are consistently filled by trades, you can continuously place orders on the API**. For more information, please see [Spot Unfilled Order Count Rules](/docs/binance-spot-api-docs/faqs/order_count_decrement).
* Use the [`account.rateLimits.orders`](/docs/binance-spot-api-docs/web-socket-api/account-requests#query-unfilled-order-count) request to keep track of how many orders you have placed within this interval.
* If you exceed this, requests fail with status `429`.
  * This status code indicates you should back off and stop spamming the API.
  * Responses that have a status `429` include a `retryAfter` field, indicating when you can retry the request.

* This is maintained **per account** and is shared by all API keys of the account.

Successful response indicating that you have placed 12 orders in 10 seconds, and 4043 orders in the past 24 hours:

```
{  "id": "e2a85d9f-07a5-4f94-8d5f-789dc3deb097",  "status": 200,  "result": {    "symbol": "BTCUSDT",    "orderId": 12510053279,    "orderListId": -1,    "clientOrderId": "a097fe6304b20a7e4fc436",    "transactTime": 1655716096505,    "price": "0.10000000",    "origQty": "10.00000000",    "executedQty": "0.00000000",    "origQuoteOrderQty": "0.000000",    "cummulativeQuoteQty": "0.00000000",    "status": "NEW",    "timeInForce": "GTC",    "type": "LIMIT",    "side": "BUY",    "workingTime": 1655716096505,    "selfTradePreventionMode": "NONE"  },  "rateLimits": [    {      "rateLimitType": "ORDERS",      "interval": "SECOND",      "intervalNum": 10,      "limit": 50,      "count": 12    },    {      "rateLimitType": "ORDERS",      "interval": "DAY",      "intervalNum": 1,      "limit": 160000,      "count": 4043    },    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 321    }  ]}
```

## REQUEST SECURITY

Request security
==========

* Every method has a security type which determines how to call it.
  * Security type is stated next to the method name.
    For example, [Place new order (TRADE)](/docs/binance-spot-api-docs/web-socket-api/trading-requests#place-new-order-trade).
  * If no security type is stated, the security type is NONE.

|Security type|API key |Signature|                               Description                                |
|-------------|--------|---------|--------------------------------------------------------------------------|
|   `NONE`    |        |         |                            Public market data                            |
|   `TRADE`   |required|required |          Trading on the exchange, placing and canceling orders           |
| `USER_DATA` |required|required |Private account information, such as order status and your trading history|
|`USER_STREAM`|required|         |                 Managing User Data Stream subscriptions                  |

* Secure methods require a valid API key to be specified and authenticated.
  * API keys can be created on the [API Management](https://www.binance.com/en/support/faq/360002502072) page of your Binance account.
  * **Both API key and secret key are sensitive.** Never share them with anyone.
    If you notice unusual activity in your account, immediately revoke all the keys and contact Binance support.

* API keys can be configured to allow access only to certain types of secure methods.
  * For example, you can have an API key with `TRADE` permission for trading,
    while using a separate API key with `USER_DATA` permission to monitor your order status.
  * By default, an API key cannot `TRADE`. You need to enable trading in API Management first.

* `TRADE` and `USER_DATA` requests are also known as `SIGNED` requests.

### SIGNED (TRADE and USER\_DATA) request security[​](/docs/binance-spot-api-docs/web-socket-api/request-security#signed-trade-and-user_data-request-security) ###

* `SIGNED` requests require an additional parameter: `signature`, authorizing the request.
* Please consult [SIGNED request example (HMAC)](/docs/binance-spot-api-docs/web-socket-api/request-security#signed-request-example-hmac), [SIGNED request example (RSA)](/docs/binance-spot-api-docs/web-socket-api/request-security#signed-request-example-rsa), and [SIGNED request example (Ed25519)](/docs/binance-spot-api-docs/web-socket-api/request-security#signed-request-example-ed25519) on how to compute signature, depending on which API key type you are using.

### Timing security[​](/docs/binance-spot-api-docs/web-socket-api/request-security#timing-security) ###

* `SIGNED` requests also require a `timestamp` parameter which should be the current millisecond timestamp.

* An additional optional parameter, `recvWindow`, specifies for how long the request stays valid.

  * If `recvWindow` is not sent, **it defaults to 5000 milliseconds**.
  * Maximum `recvWindow` is 60000 milliseconds.

* Request processing logic is as follows:

  ```
  if (timestamp < (serverTime + 1000) && (serverTime - timestamp) <= recvWindow) {  // process request} else {  // reject request}
  ```

**Serious trading is about timing.** Networks can be unstable and unreliable,
which can lead to requests taking varying amounts of time to reach the
servers. With `recvWindow`, you can specify that the request must be
processed within a certain number of milliseconds or be rejected by the
server.

**It is recommended to use a small `recvWindow` of 5000 or less!**

### SIGNED request example (HMAC)[​](/docs/binance-spot-api-docs/web-socket-api/request-security#signed-request-example-hmac) ###

Here is a step-by-step guide on how to sign requests using HMAC secret key.

Example API key and secret key:

|   Key   |                              Value                               |
|---------|------------------------------------------------------------------|
| apiKey  |`vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A`|
|secretKey|`NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j`|

**WARNING: DO NOT SHARE YOUR API KEY AND SECRET KEY WITH ANYONE.**

The example keys are provided here only for illustrative purposes.

Example of request:

```
{  "id": "4885f793-e5ad-4c3b-8f6c-55d891472b71",  "method": "order.place",  "params": {    "symbol":           "BTCUSDT",    "side":             "SELL",    "type":             "LIMIT",    "timeInForce":      "GTC",    "quantity":         "0.01000000",    "price":            "52000.00",    "newOrderRespType": "ACK",    "recvWindow":       100,    "timestamp":        1645423376532,    "apiKey":           "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",    "signature":        "------ FILL ME ------"  }}
```

As you can see, the `signature` parameter is currently missing.

**Step 1. Construct the signature payload**

Take all request `params` except for the `signature`, sort them by name in alphabetical order:

|   Parameter    |                             Value                              |
|----------------|----------------------------------------------------------------|
|     apiKey     |vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A|
|newOrderRespType|                              ACK                               |
|     price      |                            52000.00                            |
|    quantity    |                           0.01000000                           |
|   recvWindow   |                              100                               |
|      side      |                              SELL                              |
|     symbol     |                            BTCUSDT                             |
|  timeInForce   |                              GTC                               |
|   timestamp    |                         1645423376532                          |
|      type      |                             LIMIT                              |

Format parameters as `parameter=value` pairs separated by `&`.

Resulting signature payload:

```
apiKey=vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A&newOrderRespType=ACK&price=52000.00&quantity=0.01000000&recvWindow=100&side=SELL&symbol=BTCUSDT&timeInForce=GTC&timestamp=1645423376532&type=LIMIT
```

**Step 2. Compute the signature**

1. Interpret `secretKey` as ASCII data, using it as a key for HMAC-SHA-256.
2. Sign signature payload as ASCII data.
3. Encode HMAC-SHA-256 output as a hex string.

Note that `apiKey`, `secretKey`, and the payload are **case-sensitive**, while resulting signature value is case-insensitive.

You can cross-check your signature algorithm implementation with OpenSSL:

```
$ echo -n 'apiKey=vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A&newOrderRespType=ACK&price=52000.00&quantity=0.01000000&recvWindow=100&side=SELL&symbol=BTCUSDT&timeInForce=GTC&timestamp=1645423376532&type=LIMIT' \  | openssl dgst -hex -sha256 -hmac 'NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j'cc15477742bd704c29492d96c7ead9414dfd8e0ec4a00f947bb5bb454ddbd08a
```

**Step 3. Add `signature` to request `params`**

Finally, complete the request by adding the `signature` parameter with the signature string.

```
{  "id": "4885f793-e5ad-4c3b-8f6c-55d891472b71",  "method": "order.place",  "params": {    "symbol":           "BTCUSDT",    "side":             "SELL",    "type":             "LIMIT",    "timeInForce":      "GTC",    "quantity":         "0.01000000",    "price":            "52000.00",    "newOrderRespType": "ACK",    "recvWindow":       100,    "timestamp":        1645423376532,    "apiKey":           "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",    "signature":        "cc15477742bd704c29492d96c7ead9414dfd8e0ec4a00f947bb5bb454ddbd08a"  }}
```

### SIGNED request example (RSA)[​](/docs/binance-spot-api-docs/web-socket-api/request-security#signed-request-example-rsa) ###

Here is a step-by-step guide on how to sign requests using your RSA private key.

| Key  |                              Value                               |
|------|------------------------------------------------------------------|
|apiKey|`CAvIjXy3F44yW6Pou5k8Dy1swsYDWJZLeoK2r8G4cFDnE9nosRppc2eKc1T8TRTQ`|

In this example, we assume the private key is stored in the `test-prv-key.pem` file.

**WARNING: DO NOT SHARE YOUR API KEY AND PRIVATE KEY WITH ANYONE.**

The example keys are provided here only for illustrative purposes.

Example of request:

```
{  "id": "4885f793-e5ad-4c3b-8f6c-55d891472b71",  "method": "order.place",  "params": {    "symbol":           "BTCUSDT",    "side":             "SELL",    "type":             "LIMIT",    "timeInForce":      "GTC",    "quantity":         "0.01000000",    "price":            "52000.00",    "newOrderRespType": "ACK",    "recvWindow":       100,    "timestamp":        1645423376532,    "apiKey":           "CAvIjXy3F44yW6Pou5k8Dy1swsYDWJZLeoK2r8G4cFDnE9nosRppc2eKc1T8TRTQ",    "signature":        "------ FILL ME ------"  }}
```

**Step 1. Construct the signature payload**

Take all request `params` except for the `signature`, sort them by name in alphabetical order:

|   Parameter    |                             Value                              |
|----------------|----------------------------------------------------------------|
|     apiKey     |CAvIjXy3F44yW6Pou5k8Dy1swsYDWJZLeoK2r8G4cFDnE9nosRppc2eKc1T8TRTQ|
|newOrderRespType|                              ACK                               |
|     price      |                            52000.00                            |
|    quantity    |                           0.01000000                           |
|   recvWindow   |                              100                               |
|      side      |                              SELL                              |
|     symbol     |                            BTCUSDT                             |
|  timeInForce   |                              GTC                               |
|   timestamp    |                         1645423376532                          |
|      type      |                             LIMIT                              |

Format parameters as `parameter=value` pairs separated by `&`.

Resulting signature payload:

```
apiKey=CAvIjXy3F44yW6Pou5k8Dy1swsYDWJZLeoK2r8G4cFDnE9nosRppc2eKc1T8TRTQ&newOrderRespType=ACK&price=52000.00&quantity=0.01000000&recvWindow=100&side=SELL&symbol=BTCUSDT&timeInForce=GTC&timestamp=1645423376532&type=LIMIT
```

**Step 2. Compute the signature**

1. Encode signature payload as ASCII data.
2. Sign payload using RSASSA-PKCS1-v1\_5 algorithm with SHA-256 hash function.
3. Encode output as base64 string.

Note that `apiKey`, the payload, and the resulting `signature` are **case-sensitive**.

You can cross-check your signature algorithm implementation with OpenSSL:

```
$ echo -n 'apiKey=CAvIjXy3F44yW6Pou5k8Dy1swsYDWJZLeoK2r8G4cFDnE9nosRppc2eKc1T8TRTQ&newOrderRespType=ACK&price=52000.00&quantity=0.01000000&recvWindow=100&side=SELL&symbol=BTCUSDT&timeInForce=GTC&timestamp=1645423376532&type=LIMIT' \  | openssl dgst -sha256 -sign test-prv-key.pem \  | openssl enc -base64 -AOJJaf8C/3VGrU4ATTR4GiUDqL2FboSE1Qw7UnnoYNfXTXHubIl1iaePGuGyfct4NPu5oVEZCH4Q6ZStfB1w4ssgu0uiB/Bg+fBrRFfVgVaLKBdYHMvT+ljUJzqVaeoThG9oXlduiw8PbS9U8DYAbDvWN3jqZLo4Z2YJbyovyDAvDTr/oC0+vssLqP7NmlNb3fF3Bj7StmOwJvQJTbRAtzxK5PP7OQe+0mbW+D7RqVkUiSswR8qJFWTeSe4nXXNIdZdueYhF/Xf25L+KitJS5IHdIHcKfEw3MQzHFb2ZsGWkjDQwxkwr7Noi0Zaa+gFtxCuatGFm9dFIyx217pmSHtA==
```

**Step 3. Add `signature` to request `params`**

Finally, complete the request by adding the `signature` parameter with the signature string.

```
{  "id": "4885f793-e5ad-4c3b-8f6c-55d891472b71",  "method": "order.place",  "params": {    "symbol":           "BTCUSDT",    "side":             "SELL",    "type":             "LIMIT",    "timeInForce":      "GTC",    "quantity":         "0.01000000",    "price":            "52000.00",    "newOrderRespType": "ACK",    "recvWindow":       100,    "timestamp":        1645423376532,    "apiKey":           "CAvIjXy3F44yW6Pou5k8Dy1swsYDWJZLeoK2r8G4cFDnE9nosRppc2eKc1T8TRTQ",    "signature":        "OJJaf8C/3VGrU4ATTR4GiUDqL2FboSE1Qw7UnnoYNfXTXHubIl1iaePGuGyfct4NPu5oVEZCH4Q6ZStfB1w4ssgu0uiB/Bg+fBrRFfVgVaLKBdYHMvT+ljUJzqVaeoThG9oXlduiw8PbS9U8DYAbDvWN3jqZLo4Z2YJbyovyDAvDTr/oC0+vssLqP7NmlNb3fF3Bj7StmOwJvQJTbRAtzxK5PP7OQe+0mbW+D7RqVkUiSswR8qJFWTeSe4nXXNIdZdueYhF/Xf25L+KitJS5IHdIHcKfEw3MQzHFb2ZsGWkjDQwxkwr7Noi0Zaa+gFtxCuatGFm9dFIyx217pmSHtA=="  }}
```

### SIGNED Request Example (Ed25519)[​](/docs/binance-spot-api-docs/web-socket-api/request-security#signed-request-example-ed25519) ###

**Note: It is highly recommended to use Ed25519 API keys as it should provide the best performance and security out of all supported key types.**

|  Parameter  |    Value    |
|-------------|-------------|
|  `symbol`   |   BTCUSDT   |
|   `side`    |    SELL     |
|   `type`    |    LIMIT    |
|`timeInForce`|     GTC     |
| `quantity`  |      1      |
|   `price`   |     0.2     |
| `timestamp` |1668481559918|

This is a sample code in Python to show how to sign the payload with an Ed25519 key.

```
#!/usr/bin/env python3import base64import timeimport jsonfrom cryptography.hazmat.primitives.serialization import load_pem_private_keyfrom websocket import create_connection# Set up authenticationAPI_KEY='put your own API Key here'PRIVATE_KEY_PATH='test-prv-key.pem'# Load the private key.# In this example the key is expected to be stored without encryption,# but we recommend using a strong password for improved security.with open(PRIVATE_KEY_PATH, 'rb') as f:    private_key = load_pem_private_key(data=f.read(),                                       password=None)# Set up the request parametersparams = {    'apiKey':        API_KEY,    'symbol':       'BTCUSDT',    'side':         'SELL',    'type':         'LIMIT',    'timeInForce':  'GTC',    'quantity':     '1.0000000',    'price':        '0.20'}# Timestamp the requesttimestamp = int(time.time() * 1000) # UNIX timestamp in millisecondsparams['timestamp'] = timestamp# Sign the requestpayload = '&'.join([f'{param}={value}' for param, value in sorted(params.items())])signature = base64.b64encode(private_key.sign(payload.encode('ASCII')))params['signature'] = signature.decode('ASCII')# Send the requestrequest = {    'id': 'my_new_order',    'method': 'order.place',    'params': params}ws = create_connection("wss://ws-api.binance.com:443/ws-api/v3")ws.send(json.dumps(request))result =  ws.recv()ws.close()print(result)
```

## SESSION AUTHENTICATION

Session Authentication
==========

**Note:** Only *Ed25519* keys are supported for this feature.

If you do not want to specify `apiKey` and `signature` in each individual request,
you can authenticate your API key for the active WebSocket session.

Once authenticated, you no longer have to specify `apiKey` and `signature` for those requests that need them.
Requests will be performed on behalf of the account owning the authenticated API key.

**Note:** You still have to specify the `timestamp` parameter for `SIGNED` requests.

### Authenticate after connection[​](/docs/binance-spot-api-docs/web-socket-api/session-authentication#authenticate-after-connection) ###

You can authenticate an already established connection using session authentication requests:

* [`session.logon`](/docs/binance-spot-api-docs/web-socket-api/authentication-requests#log-in-with-api-key-signed) – authenticate, or change the API key associated with the connection
* [`session.status`](/docs/binance-spot-api-docs/web-socket-api/authentication-requests#query-session-status) – check connection status and the current API key
* [`session.logout`](/docs/binance-spot-api-docs/web-socket-api/authentication-requests#log-out-of-the-session) – forget the API key associated with the connection

**Regarding API key revocation:**

If during an active session the API key becomes invalid for *any reason* (e.g. IP address is not whitelisted, API key was deleted, API key doesn't have correct permissions, etc), after the next request the session will be revoked with the following error message:

```
{  "id": null,  "status": 401,  "error": {    "code": -2015,    "msg": "Invalid API-key, IP, or permissions for action."  }}
```

### Authorize *ad hoc* requests[​](/docs/binance-spot-api-docs/web-socket-api/session-authentication#authorize-ad-hoc-requests) ###

Only one API key can be authenticated with the WebSocket connection.
The authenticated API key is used by default for requests that require an `apiKey` parameter.
However, you can always specify the `apiKey` and `signature` explicitly for individual requests,
overriding the authenticated API key and using a different one to authorize a specific request.

For example, you might want to authenticate your `USER_DATA` key to be used by default,
but specify the `TRADE` key with an explicit signature when placing orders.

## DATA SOURCES

Data sources
==========

* The API system is asynchronous. Some delay in the response is normal and expected.

* Each method has a data source indicating where the data is coming from, and thus how up-to-date it is.

|  Data Source  |Latency |                          Description                           |
|---------------|--------|----------------------------------------------------------------|
|Matching Engine| lowest |       The Matching Engine produces the response directly       |
|    Memory     |  low   |Data is fetched from API server's local or external memory cache|
|   Database    |moderate|              Data is retrieved from the database               |

* Some methods have more than one data source (e.g., Memory =\> Database).

  This means that the API will look for the latest data in that order:
  first in the cache, then in the database.

## GENERAL REQUESTS

General requests
==========

### Terminology[​](/docs/binance-spot-api-docs/web-socket-api/general-requests#terminology) ###

These terms will be used throughout the documentation, so it is recommended especially for new users to read to help their understanding of the API.

* `base asset` refers to the asset that is the `quantity` of a symbol. For the symbol BTCUSDT, BTC would be the `base asset`.
* `quote asset` refers to the asset that is the `price` of a symbol. For the symbol BTCUSDT, USDT would be the `quote asset`.

### Test connectivity[​](/docs/binance-spot-api-docs/web-socket-api/general-requests#test-connectivity) ###

```
{  "id": "922bcc6e-9de8-440d-9e84-7c80933a8d0d",  "method": "ping"}
```

Test connectivity to the WebSocket API.

**Note:**You can use regular WebSocket ping frames to test connectivity as well,
WebSocket API will respond with pong frames as soon as possible.`ping` request along with `time` is a safe way to test request-response handling in your application.

**Weight:**1

**Parameters:**NONE

**Data Source:**Memory

**Response:**

```
{  "id": "922bcc6e-9de8-440d-9e84-7c80933a8d0d",  "status": 200,  "result": {},  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 1    }  ]}
```

### Check server time[​](/docs/binance-spot-api-docs/web-socket-api/general-requests#check-server-time) ###

```
{  "id": "187d3cb2-942d-484c-8271-4e2141bbadb1",  "method": "time"}
```

Test connectivity to the WebSocket API and get the current server time.

**Weight:**1

**Parameters:**NONE

**Data Source:**Memory

**Response:**

```
{  "id": "187d3cb2-942d-484c-8271-4e2141bbadb1",  "status": 200,  "result": {    "serverTime": 1656400526260  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 1    }  ]}
```

[]()

### Exchange information[​](/docs/binance-spot-api-docs/web-socket-api/general-requests#exchange-information) ###

```
{  "id": "5494febb-d167-46a2-996d-70533eb4d976",  "method": "exchangeInfo",  "params": {    "symbols": ["BNBBTC"]  }}
```

Query current exchange trading rules, rate limits, and symbol information.

**Weight:**20

**Parameters:**

|        Name        |     Type      |                                                                              Mandatory                                                                              |      Description       |
|--------------------|---------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------|
|      `symbol`      |    STRING     |                                                                                 NO                                                                                  |Describe a single symbol|
|     `symbols`      |ARRAY of STRING|                                                                      Describe multiple symbols                                                                      |                        |
|   `permissions`    |ARRAY of STRING|                                                                    Filter symbols by permissions                                                                    |                        |
|`showPermissionSets`|    BOOLEAN    |                                 Controls whether the content of the `permissionSets` field is populated or not. Defaults to `true`.                                 |                        |
|   `symbolStatus`   |     ENUM      |Filters symbols that have this `tradingStatus`.  <br/><br/> Valid values: `TRADING`, `HALT`, `BREAK`   <br/> Cannot be used in combination with `symbol` or `symbols`|                        |

Notes:

* Only one of `symbol`, `symbols`, `permissions` parameters can be specified.

* Without parameters, `exchangeInfo` displays all symbols with `["SPOT, "MARGIN", "LEVERAGED"]` permissions.

  * In order to list *all* active symbols on the exchange, you need to explicitly request all permissions.

* `permissions` accepts either a list of permissions, or a single permission name. E.g. `"SPOT"`.

* [Available Permissions](/docs/binance-spot-api-docs/web-socket-api/enums.md#account-and-symbol-permissions)

[]()

**Examples of Symbol Permissions Interpretation from the Response:**

* `[["A","B"]]` means you may place an order if your account has either permission "A" **or** permission "B".
* `[["A"],["B"]]` means you can place an order if your account has permission "A" **and** permission "B".
* `[["A"],["B","C"]]` means you can place an order if your account has permission "A" **and** permission "B" or permission "C". (Inclusive or is applied here, not exclusive or, so your account may have both permission "B" and permission "C".)

**Data Source:**Memory

**Response:**

```
{  "id": "5494febb-d167-46a2-996d-70533eb4d976",  "status": 200,  "result": {    "timezone": "UTC",    "serverTime": 1655969291181,    // Global rate limits. See "Rate limits" section.    "rateLimits": [      {        "rateLimitType": "REQUEST_WEIGHT",    // Rate limit type: REQUEST_WEIGHT, ORDERS, CONNECTIONS        "interval": "MINUTE",                 // Rate limit interval: SECOND, MINUTE, DAY        "intervalNum": 1,                     // Rate limit interval multiplier (i.e., "1 minute")        "limit": 6000                         // Rate limit per interval      },      {        "rateLimitType": "ORDERS",        "interval": "SECOND",        "intervalNum": 10,        "limit": 50      },      {        "rateLimitType": "ORDERS",        "interval": "DAY",        "intervalNum": 1,        "limit": 160000      },      {        "rateLimitType": "CONNECTIONS",        "interval": "MINUTE",        "intervalNum": 5,        "limit": 300      }    ],    // Exchange filters are explained on the "Filters" page:    // https://github.com/binance/binance-spot-api-docs/blob/master/filters.md    // All exchange filters are optional.    "exchangeFilters": [],    "symbols": [      {        "symbol": "BNBBTC",        "status": "TRADING",        "baseAsset": "BNB",        "baseAssetPrecision": 8,        "quoteAsset": "BTC",        "quotePrecision": 8,        "quoteAssetPrecision": 8,        "baseCommissionPrecision": 8,        "quoteCommissionPrecision": 8,        "orderTypes": [          "LIMIT",          "LIMIT_MAKER",          "MARKET",          "STOP_LOSS_LIMIT",          "TAKE_PROFIT_LIMIT"        ],        "icebergAllowed": true,        "ocoAllowed": true,        "otoAllowed": true,        "quoteOrderQtyMarketAllowed": true,        "allowTrailingStop": true,        "cancelReplaceAllowed": true,        "isSpotTradingAllowed": true,        "isMarginTradingAllowed": true,        // Symbol filters are explained on the "Filters" page:        // https://github.com/binance/binance-spot-api-docs/blob/master/filters.md        // All symbol filters are optional.        "filters": [          {            "filterType": "PRICE_FILTER",            "minPrice": "0.00000100",            "maxPrice": "100000.00000000",            "tickSize": "0.00000100"          },          {            "filterType": "LOT_SIZE",            "minQty": "0.00100000",            "maxQty": "100000.00000000",            "stepSize": "0.00100000"          }        ],        "permissions": [],        "permissionSets": [          [            "SPOT",            "MARGIN",            "TRD_GRP_004"          ]        ],        "defaultSelfTradePreventionMode": "NONE",        "allowedSelfTradePreventionModes": [          "NONE"        ]      }    ],    // Optional field. Present only when SOR is available.    // https://github.com/binance/binance-spot-api-docs/blob/master/faqs/sor_faq.md    "sors": [      {        "baseAsset": "BTC",        "symbols": [          "BTCUSDT",          "BTCUSDC"        ]      }    ]  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 20    }  ]}
```

## MARKET DATA REQUESTS

Market data requests
==========

### Order book[​](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#order-book) ###

```
{  "id": "51e2affb-0aba-4821-ba75-f2625006eb43",  "method": "depth",  "params": {    "symbol": "BNBBTC",    "limit": 5  }}
```

Get current order book.

Note that this request returns limited market depth.

If you need to continuously monitor order book updates, please consider using WebSocket Streams:

* [`<symbol>@depth<levels>`](/docs/binance-spot-api-docs/web-socket-streams#partial-book-depth-streams)
* [`<symbol>@depth`](/docs/binance-spot-api-docs/web-socket-streams#diff-depth-stream)

You can use `depth` request together with `<symbol>@depth` streams to [maintain a local order book](/docs/binance-spot-api-docs/web-socket-streams#how-to-manage-a-local-order-book-correctly).

**Weight:**Adjusted based on the limit:

|  Limit  |Weight|
|---------|------|
|  1–100  |  5   |
| 101–500 |  25  |
|501–1000 |  50  |
|1001–5000| 250  |

**Parameters:**

|  Name  | Type |Mandatory|     Description     |
|--------|------|---------|---------------------|
|`symbol`|STRING|   YES   |                     |
|`limit` | INT  |   NO    |Default 100; max 5000|

**Data Source:**Memory

**Response:**

```
{  "id": "51e2affb-0aba-4821-ba75-f2625006eb43",  "status": 200,  "result": {    "lastUpdateId": 2731179239,    // Bid levels are sorted from highest to lowest price.    "bids": [      [        "0.01379900",   // Price        "3.43200000"    // Quantity      ],      [        "0.01379800",        "3.24300000"      ],      [        "0.01379700",        "10.45500000"      ],      [        "0.01379600",        "3.82100000"      ],      [        "0.01379500",        "10.26200000"      ]    ],    // Ask levels are sorted from lowest to highest price.    "asks": [      [        "0.01380000",        "5.91700000"      ],      [        "0.01380100",        "6.01400000"      ],      [        "0.01380200",        "0.26800000"      ],      [        "0.01380300",        "0.33800000"      ],      [        "0.01380400",        "0.26800000"      ]    ]  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 2    }  ]}
```

### Recent trades[​](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#recent-trades) ###

```
{  "id": "409a20bd-253d-41db-a6dd-687862a5882f",  "method": "trades.recent",  "params": {    "symbol": "BNBBTC",    "limit": 1  }}
```

Get recent trades.

If you need access to real-time trading activity, please consider using WebSocket Streams:

* [`<symbol>@trade`](/docs/binance-spot-api-docs/web-socket-streams#trade-streams)

**Weight:**25

**Parameters:**

|  Name  | Type |Mandatory|     Description     |
|--------|------|---------|---------------------|
|`symbol`|STRING|   YES   |                     |
|`limit` | INT  |   NO    |Default 500; max 1000|

**Data Source:**Memory

**Response:**

```
{  "id": "409a20bd-253d-41db-a6dd-687862a5882f",  "status": 200,  "result": [    {      "id": 194686783,      "price": "0.01361000",      "qty": "0.01400000",      "quoteQty": "0.00019054",      "time": 1660009530807,      "isBuyerMaker": true,      "isBestMatch": true    }  ],  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 2    }  ]}
```

### Historical trades[​](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#historical-trades) ###

```
{  "id": "cffc9c7d-4efc-4ce0-b587-6b87448f052a",  "method": "trades.historical",  "params": {    "symbol": "BNBBTC",    "fromId": 0,    "limit": 1  }}
```

Get historical trades.

**Weight:**25

**Parameters:**

|  Name  | Type |Mandatory|     Description     |
|--------|------|---------|---------------------|
|`symbol`|STRING|   YES   |                     |
|`fromId`| INT  |   NO    |Trade ID to begin at |
|`limit` | INT  |   NO    |Default 500; max 1000|

Notes:

* If `fromId` is not specified, the most recent trades are returned.

**Data Source:**Database

**Response:**

```
{  "id": "cffc9c7d-4efc-4ce0-b587-6b87448f052a",  "status": 200,  "result": [    {      "id": 0,      "price": "0.00005000",      "qty": "40.00000000",      "quoteQty": "0.00200000",      "time": 1500004800376,      "isBuyerMaker": true,      "isBestMatch": true    }  ],  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 10    }  ]}
```

### Aggregate trades[​](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#aggregate-trades) ###

```
{  "id": "189da436-d4bd-48ca-9f95-9f613d621717",  "method": "trades.aggregate",  "params": {    "symbol": "BNBBTC",    "fromId": 50000000,    "limit": 1  }}
```

Get aggregate trades.

An *aggregate trade* (aggtrade) represents one or more individual trades.
Trades that fill at the same time, from the same taker order, with the same price –
those trades are collected into an aggregate trade with total quantity of the individual trades.

If you need access to real-time trading activity, please consider using WebSocket Streams:

* [`<symbol>@aggTrade`](/docs/binance-spot-api-docs/web-socket-streams#aggregate-trade-streams)

If you need historical aggregate trade data,
please consider using [data.binance.vision](https://github.com/binance/binance-public-data/#aggtrades).

**Weight:**4

**Parameters:**

|   Name    | Type |Mandatory|         Description          |
|-----------|------|---------|------------------------------|
| `symbol`  |STRING|   YES   |                              |
| `fromId`  | INT  |   NO    |Aggregate trade ID to begin at|
|`startTime`| INT  |   NO    |                              |
| `endTime` | INT  |   NO    |                              |
|  `limit`  | INT  |   NO    |    Default 500; max 1000     |

Notes:

* If `fromId` is specified, return aggtrades with aggregate trade ID \>= `fromId`.

  Use `fromId` and `limit` to page through all aggtrades.

* If `startTime` and/or `endTime` are specified, aggtrades are filtered by execution time (`T`).

  `fromId` cannot be used together with `startTime` and `endTime`.

* If no condition is specified, the most recent aggregate trades are returned.

**Data Source:**Database

**Response:**

```
{  "id": "189da436-d4bd-48ca-9f95-9f613d621717",  "status": 200,  "result": [    {      "a": 50000000,        // Aggregate trade ID      "p": "0.00274100",    // Price      "q": "57.19000000",   // Quantity      "f": 59120167,        // First trade ID      "l": 59120170,        // Last trade ID      "T": 1565877971222,   // Timestamp      "m": true,            // Was the buyer the maker?      "M": true             // Was the trade the best price match?    }  ],  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 2    }  ]}
```

### Klines[​](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#klines) ###

```
{  "id": "1dbbeb56-8eea-466a-8f6e-86bdcfa2fc0b",  "method": "klines",  "params": {    "symbol": "BNBBTC",    "interval": "1h",    "startTime": 1655969280000,    "limit": 1  }}
```

Get klines (candlestick bars).

Klines are uniquely identified by their open & close time.

If you need access to real-time kline updates, please consider using WebSocket Streams:

* [`<symbol>@kline_<interval>`](/docs/binance-spot-api-docs/web-socket-streams#klinecandlestick-streams)

If you need historical kline data,
please consider using [data.binance.vision](https://github.com/binance/binance-public-data/./market-data-requests.md#klines).

**Weight:**2

**Parameters:**

|   Name    | Type |Mandatory|     Description     |
|-----------|------|---------|---------------------|
| `symbol`  |STRING|   YES   |                     |
|`interval` | ENUM |   YES   |                     |
|`startTime`| INT  |   NO    |                     |
| `endTime` | INT  |   NO    |                     |
|`timeZone` |STRING|   NO    |  Default: 0 (UTC)   |
|  `limit`  | INT  |   NO    |Default 500; max 1000|

[]()Supported kline intervals (case-sensitive):

|Interval|         `interval` value          |
|--------|-----------------------------------|
|seconds |               `1s`                |
|minutes |  `1m`, `3m`, `5m`, `15m`, `30m`   |
| hours  |`1h`, `2h`, `4h`, `6h`, `8h`, `12h`|
|  days  |            `1d`, `3d`             |
| weeks  |               `1w`                |
| months |               `1M`                |

Notes:

* If `startTime`, `endTime` are not specified, the most recent klines are returned.
* Supported values for `timeZone`:
  * Hours and minutes (e.g. `-1:00`, `05:45`)
  * Only hours (e.g. `0`, `8`, `4`)
  * Accepted range is strictly [-12:00 to +14:00] inclusive

* If `timeZone` provided, kline intervals are interpreted in that timezone instead of UTC.
* Note that `startTime` and `endTime` are always interpreted in UTC, regardless of timeZone.

**Data Source:**Database

**Response:**

```
{  "id": "1dbbeb56-8eea-466a-8f6e-86bdcfa2fc0b",  "status": 200,  "result": [    [      1655971200000,      // Kline open time      "0.01086000",       // Open price      "0.01086600",       // High price      "0.01083600",       // Low price      "0.01083800",       // Close price      "2290.53800000",    // Volume      1655974799999,      // Kline close time      "24.85074442",      // Quote asset volume      2283,               // Number of trades      "1171.64000000",    // Taker buy base asset volume      "12.71225884",      // Taker buy quote asset volume      "0"                 // Unused field, ignore    ]  ],  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 2    }  ]}
```

### UI Klines[​](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#ui-klines) ###

```
{  "id": "b137468a-fb20-4c06-bd6b-625148eec958",  "method": "uiKlines",  "params": {    "symbol": "BNBBTC",    "interval": "1h",    "startTime": 1655969280000,    "limit": 1  }}
```

Get klines (candlestick bars) optimized for presentation.

This request is similar to [`klines`](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#klines), having the same parameters and response.`uiKlines` return modified kline data, optimized for presentation of candlestick charts.

**Weight:**2

**Parameters:**

|   Name    | Type |Mandatory|                                          Description                                          |
|-----------|------|---------|-----------------------------------------------------------------------------------------------|
| `symbol`  |STRING|   YES   |                                                                                               |
|`interval` | ENUM |   YES   |See [`klines`](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#kline-intervals)|
|`startTime`| INT  |   NO    |                                                                                               |
| `endTime` | INT  |   NO    |                                                                                               |
|`timeZone` |STRING|   NO    |                                       Default: 0 (UTC)                                        |
|  `limit`  | INT  |   NO    |                                     Default 500; max 1000                                     |

Notes:

* If `startTime`, `endTime` are not specified, the most recent klines are returned.
* Supported values for `timeZone`:
  * Hours and minutes (e.g. `-1:00`, `05:45`)
  * Only hours (e.g. `0`, `8`, `4`)
  * Accepted range is strictly [-12:00 to +14:00] inclusive

* If `timeZone` provided, kline intervals are interpreted in that timezone instead of UTC.
* Note that `startTime` and `endTime` are always interpreted in UTC, regardless of timeZone.

**Data Source:**Database

**Response:**

```
{  "id": "b137468a-fb20-4c06-bd6b-625148eec958",  "status": 200,  "result": [    [      1655971200000,      // Kline open time      "0.01086000",       // Open price      "0.01086600",       // High price      "0.01083600",       // Low price      "0.01083800",       // Close price      "2290.53800000",    // Volume      1655974799999,      // Kline close time      "24.85074442",      // Quote asset volume      2283,               // Number of trades      "1171.64000000",    // Taker buy base asset volume      "12.71225884",      // Taker buy quote asset volume      "0"                 // Unused field, ignore    ]  ],  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 2    }  ]}
```

### Current average price[​](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#current-average-price) ###

```
{  "id": "ddbfb65f-9ebf-42ec-8240-8f0f91de0867",  "method": "avgPrice",  "params": {    "symbol": "BNBBTC"  }}
```

Get current average price for a symbol.

**Weight:**2

**Parameters:**

|  Name  | Type |Mandatory|Description|
|--------|------|---------|-----------|
|`symbol`|STRING|   YES   |           |

**Data Source:**Memory

**Response:**

```
{  "id": "ddbfb65f-9ebf-42ec-8240-8f0f91de0867",  "status": 200,  "result": {    "mins": 5,                    // Average price interval (in minutes)    "price": "9.35751834",        // Average price    "closeTime": 1694061154503    // Last trade time  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 2    }  ]}
```

### 24hr ticker price change statistics[​](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#24hr-ticker-price-change-statistics) ###

```
{  "id": "93fb61ef-89f8-4d6e-b022-4f035a3fadad",  "method": "ticker.24hr",  "params": {    "symbol": "BNBBTC"  }}
```

Get 24-hour rolling window price change statistics.

If you need to continuously monitor trading statistics, please consider using WebSocket Streams:

* [`<symbol>@ticker`](/docs/binance-spot-api-docs/web-socket-streams#individual-symbol-ticker-streams) or [`!ticker@arr`](/docs/binance-spot-api-docs/web-socket-streams#all-market-tickers-stream)
* [`<symbol>@miniTicker`](/docs/binance-spot-api-docs/web-socket-streams#individual-symbol-mini-ticker-stream) or [`!miniTicker@arr`](/docs/binance-spot-api-docs/web-socket-streams#all-market-mini-tickers-stream)

If you need different window sizes,
use the [`ticker`](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#rolling-window-price-change-statistics) request.

**Weight:**Adjusted based on the number of requested symbols:

|  Symbols  |Weight|
|-----------|------|
|   1–20    |  2   |
|  21–100   |  40  |
|101 or more|  80  |
|all symbols|  80  |

**Parameters:**

|  Name   |     Type      |            Mandatory            |              Description              |
|---------|---------------|---------------------------------|---------------------------------------|
|`symbol` |    STRING     |               NO                |   Query ticker for a single symbol    |
|`symbols`|ARRAY of STRING|Query ticker for multiple symbols|                                       |
| `type`  |     ENUM      |               NO                |Ticker type: `FULL` (default) or `MINI`|

Notes:

* `symbol` and `symbols` cannot be used together.

* If no symbol is specified, returns information about all symbols currently trading on the exchange.

**Data Source:**Memory

**Response:**

`FULL` type, for a single symbol:

```
{  "id": "93fb61ef-89f8-4d6e-b022-4f035a3fadad",  "status": 200,  "result": {    "symbol": "BNBBTC",    "priceChange": "0.00013900",    "priceChangePercent": "1.020",    "weightedAvgPrice": "0.01382453",    "prevClosePrice": "0.01362800",    "lastPrice": "0.01376700",    "lastQty": "1.78800000",    "bidPrice": "0.01376700",    "bidQty": "4.64600000",    "askPrice": "0.01376800",    "askQty": "14.31400000",    "openPrice": "0.01362800",    "highPrice": "0.01414900",    "lowPrice": "0.01346600",    "volume": "69412.40500000",    "quoteVolume": "959.59411487",    "openTime": 1660014164909,    "closeTime": 1660100564909,    "firstId": 194696115,       // First trade ID    "lastId": 194968287,        // Last trade ID    "count": 272173             // Number of trades  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 2    }  ]}
```

`MINI` type, for a single symbol:

```
{  "id": "9fa2a91b-3fca-4ed7-a9ad-58e3b67483de",  "status": 200,  "result": {    "symbol": "BNBBTC",    "openPrice": "0.01362800",    "highPrice": "0.01414900",    "lowPrice": "0.01346600",    "lastPrice": "0.01376700",    "volume": "69412.40500000",    "quoteVolume": "959.59411487",    "openTime": 1660014164909,    "closeTime": 1660100564909,    "firstId": 194696115,       // First trade ID    "lastId": 194968287,        // Last trade ID    "count": 272173             // Number of trades  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 2    }  ]}
```

If more than one symbol is requested, response returns an array:

```
{  "id": "901be0d9-fd3b-45e4-acd6-10c580d03430",  "status": 200,  "result": [    {      "symbol": "BNBBTC",      "priceChange": "0.00016500",      "priceChangePercent": "1.213",      "weightedAvgPrice": "0.01382508",      "prevClosePrice": "0.01360800",      "lastPrice": "0.01377200",      "lastQty": "1.01400000",      "bidPrice": "0.01377100",      "bidQty": "7.55700000",      "askPrice": "0.01377200",      "askQty": "4.37900000",      "openPrice": "0.01360700",      "highPrice": "0.01414900",      "lowPrice": "0.01346600",      "volume": "69376.27900000",      "quoteVolume": "959.13277091",      "openTime": 1660014615517,      "closeTime": 1660101015517,      "firstId": 194697254,      "lastId": 194969483,      "count": 272230    },    {      "symbol": "BTCUSDT",      "priceChange": "-938.06000000",      "priceChangePercent": "-3.938",      "weightedAvgPrice": "23265.34432003",      "prevClosePrice": "23819.17000000",      "lastPrice": "22880.91000000",      "lastQty": "0.00536000",      "bidPrice": "22880.40000000",      "bidQty": "0.00424000",      "askPrice": "22880.91000000",      "askQty": "0.04276000",      "openPrice": "23818.97000000",      "highPrice": "23933.25000000",      "lowPrice": "22664.69000000",      "volume": "153508.37606000",      "quoteVolume": "3571425225.04441220",      "openTime": 1660014615977,      "closeTime": 1660101015977,      "firstId": 1592019902,      "lastId": 1597301762,      "count": 5281861    }  ],  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 2    }  ]}
```

### Trading Day Ticker[​](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#trading-day-ticker) ###

```
{  "id": "f4b3b507-c8f2-442a-81a6-b2f12daa030f",  "method": "ticker.tradingDay",  "params": {    "symbols": [      "BNBBTC",      "BTCUSDT"    ],    "timeZone": "00:00"  }}
```

Price change statistics for a trading day.

**Weight:**

4 for each requested symbol.   

 The weight for this request will cap at 200 once the number of `symbols` in the request is more than 50.

**Parameters:**

|   Name   |     Type      |            Mandatory            |                                Description                                 |
|----------|---------------|---------------------------------|----------------------------------------------------------------------------|
| `symbol` |    STRING     |               YES               |                      Query ticker of a single symbol                       |
|`symbols` |ARRAY of STRING|Query ticker for multiple symbols|                                                                            |
|`timeZone`|    STRING     |               NO                |                              Default: 0 (UTC)                              |
|  `type`  |     ENUM      |               NO                |Supported values: FULL or MINI.   <br/>If none provided, the default is FULL|

**Notes:**

* Supported values for `timeZone`:
  * Hours and minutes (e.g. `-1:00`, `05:45`)
  * Only hours (e.g. `0`, `8`, `4`)

**Data Source:**Database

**Response: - FULL**

With `symbol`:

```
{  "id": "f4b3b507-c8f2-442a-81a6-b2f12daa030f",  "status": 200,  "result": {    "symbol": "BTCUSDT",    "priceChange": "-83.13000000",                // Absolute price change    "priceChangePercent": "-0.317",               // Relative price change in percent    "weightedAvgPrice": "26234.58803036",         // quoteVolume / volume    "openPrice": "26304.80000000",    "highPrice": "26397.46000000",    "lowPrice": "26088.34000000",    "lastPrice": "26221.67000000",    "volume": "18495.35066000",                   // Volume in base asset    "quoteVolume": "485217905.04210480",    "openTime": 1695686400000,    "closeTime": 1695772799999,    "firstId": 3220151555,    "lastId": 3220849281,    "count": 697727  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 4    }  ]}
```

With `symbols`:

```
{  "id": "f4b3b507-c8f2-442a-81a6-b2f12daa030f",  "status": 200,  "result": [    {      "symbol": "BTCUSDT",      "priceChange": "-83.13000000",      "priceChangePercent": "-0.317",      "weightedAvgPrice": "26234.58803036",      "openPrice": "26304.80000000",      "highPrice": "26397.46000000",      "lowPrice": "26088.34000000",      "lastPrice": "26221.67000000",      "volume": "18495.35066000",      "quoteVolume": "485217905.04210480",      "openTime": 1695686400000,      "closeTime": 1695772799999,      "firstId": 3220151555,      "lastId": 3220849281,      "count": 697727    },    {      "symbol": "BNBUSDT",      "priceChange": "2.60000000",      "priceChangePercent": "1.238",      "weightedAvgPrice": "211.92276958",      "openPrice": "210.00000000",      "highPrice": "213.70000000",      "lowPrice": "209.70000000",      "lastPrice": "212.60000000",      "volume": "280709.58900000",      "quoteVolume": "59488753.54750000",      "openTime": 1695686400000,      "closeTime": 1695772799999,      "firstId": 672397461,      "lastId": 672496158,      "count": 98698    }  ],  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 8    }  ]}
```

**Response: - MINI**

With `symbol`:

```
{  "id": "f4b3b507-c8f2-442a-81a6-b2f12daa030f",  "status": 200,  "result": {    "symbol": "BTCUSDT",    "openPrice": "26304.80000000",    "highPrice": "26397.46000000",    "lowPrice": "26088.34000000",    "lastPrice": "26221.67000000",    "volume": "18495.35066000",                  // Volume in base asset    "quoteVolume": "485217905.04210480",         // Volume in quote asset    "openTime": 1695686400000,    "closeTime": 1695772799999,    "firstId": 3220151555,                       // Trade ID of the first trade in the interval    "lastId": 3220849281,                        // Trade ID of the last trade in the interval    "count": 697727                              // Number of trades in the interval  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 4    }  ]}
```

With `symbols`:

```
{  "id": "f4b3b507-c8f2-442a-81a6-b2f12daa030f",  "status": 200,  "result": [    {      "symbol": "BTCUSDT",      "openPrice": "26304.80000000",      "highPrice": "26397.46000000",      "lowPrice": "26088.34000000",      "lastPrice": "26221.67000000",      "volume": "18495.35066000",      "quoteVolume": "485217905.04210480",      "openTime": 1695686400000,      "closeTime": 1695772799999,      "firstId": 3220151555,      "lastId": 3220849281,      "count": 697727    },    {      "symbol": "BNBUSDT",      "openPrice": "210.00000000",      "highPrice": "213.70000000",      "lowPrice": "209.70000000",      "lastPrice": "212.60000000",      "volume": "280709.58900000",      "quoteVolume": "59488753.54750000",      "openTime": 1695686400000,      "closeTime": 1695772799999,      "firstId": 672397461,      "lastId": 672496158,      "count": 98698    }  ],  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 8    }  ]}
```

### Rolling window price change statistics[​](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#rolling-window-price-change-statistics) ###

```
{  "id": "f4b3b507-c8f2-442a-81a6-b2f12daa030f",  "method": "ticker",  "params": {    "symbols": [      "BNBBTC",      "BTCUSDT"    ],    "windowSize": "7d"  }}
```

Get rolling window price change statistics with a custom window.

This request is similar to [`ticker.24hr`](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#24hr-ticker-price-change-statistics),
but statistics are computed on demand using the arbitrary window you specify.

**Note:** Window size precision is limited to 1 minute.
While the `closeTime` is the current time of the request, `openTime` always start on a minute boundary.
As such, the effective window might be up to 59999 ms wider than the requested `windowSize`.

<details class="details_CLKC alert alert--info details_hTlw" data-collapsed="true"><summary>Window computation example</summary>

For example, a request for `"windowSize": "7d"` might result in the following window:

```
"openTime": 1659580020000,"closeTime": 1660184865291,
```

Time of the request – `closeTime` – is 1660184865291 (August 11, 2022 02:27:45.291).
Requested window size should put the `openTime` 7 days before that – August 4, 02:27:45.291 –
but due to limited precision it ends up a bit earlier: 1659580020000 (August 4, 2022 02:27:00),
exactly at the start of a minute.

</details>

If you need to continuously monitor trading statistics, please consider using WebSocket Streams:

* [`<symbol>@ticker_<window_size>`](/docs/binance-spot-api-docs/web-socket-streams#individual-symbol-rolling-window-statistics-streams) or [`!ticker_<window-size>@arr`](/docs/binance-spot-api-docs/web-socket-streams#all-market-rolling-window-statistics-streams)

**Weight:**Adjusted based on the number of requested symbols:

|Symbols|   Weight   |
|-------|------------|
| 1–50  |4 per symbol|
|51–100 |    200     |

**Parameters:**

|    Name    |     Type      |            Mandatory            |              Description              |
|------------|---------------|---------------------------------|---------------------------------------|
|  `symbol`  |    STRING     |               YES               |    Query ticker of a single symbol    |
| `symbols`  |ARRAY of STRING|Query ticker for multiple symbols|                                       |
|   `type`   |     ENUM      |               NO                |Ticker type: `FULL` (default) or `MINI`|
|`windowSize`|     ENUM      |               NO                |             Default `1d`              |

Supported window sizes:

| Unit  | `windowSize` value |
|-------|--------------------|
|minutes|`1m`, `2m` ... `59m`|
| hours |`1h`, `2h` ... `23h`|
| days  |`1d`, `2d` ... `7d` |

Notes:

* Either `symbol` or `symbols` must be specified.

* Maximum number of symbols in one request: 200.

* Window size units cannot be combined.
  E.g., `1d 2h` is not supported.

**Data Source:**Database

**Response:**

`FULL` type, for a single symbol:

```
{  "id": "f4b3b507-c8f2-442a-81a6-b2f12daa030f",  "status": 200,  "result": {    "symbol": "BNBBTC",    "priceChange": "0.00061500",    "priceChangePercent": "4.735",    "weightedAvgPrice": "0.01368242",    "openPrice": "0.01298900",    "highPrice": "0.01418800",    "lowPrice": "0.01296000",    "lastPrice": "0.01360400",    "volume": "587179.23900000",    "quoteVolume": "8034.03382165",    "openTime": 1659580020000,    "closeTime": 1660184865291,    "firstId": 192977765,       // First trade ID    "lastId": 195365758,        // Last trade ID    "count": 2387994            // Number of trades  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 4    }  ]}
```

`MINI` type, for a single symbol:

```
{  "id": "bdb7c503-542c-495c-b797-4d2ee2e91173",  "status": 200,  "result": {    "symbol": "BNBBTC",    "openPrice": "0.01298900",    "highPrice": "0.01418800",    "lowPrice": "0.01296000",    "lastPrice": "0.01360400",    "volume": "587179.23900000",    "quoteVolume": "8034.03382165",    "openTime": 1659580020000,    "closeTime": 1660184865291,    "firstId": 192977765,       // First trade ID    "lastId": 195365758,        // Last trade ID    "count": 2387994            // Number of trades  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 4    }  ]}
```

If more than one symbol is requested, response returns an array:

```
{  "id": "f4b3b507-c8f2-442a-81a6-b2f12daa030f",  "status": 200,  "result": [    {      "symbol": "BNBBTC",      "priceChange": "0.00061500",      "priceChangePercent": "4.735",      "weightedAvgPrice": "0.01368242",      "openPrice": "0.01298900",      "highPrice": "0.01418800",      "lowPrice": "0.01296000",      "lastPrice": "0.01360400",      "volume": "587169.48600000",      "quoteVolume": "8033.90114517",      "openTime": 1659580020000,      "closeTime": 1660184820927,      "firstId": 192977765,      "lastId": 195365700,      "count": 2387936    },    {      "symbol": "BTCUSDT",      "priceChange": "1182.92000000",      "priceChangePercent": "5.113",      "weightedAvgPrice": "23349.27074846",      "openPrice": "23135.33000000",      "highPrice": "24491.22000000",      "lowPrice": "22400.00000000",      "lastPrice": "24318.25000000",      "volume": "1039498.10978000",      "quoteVolume": "24271522807.76838630",      "openTime": 1659580020000,      "closeTime": 1660184820927,      "firstId": 1568787779,      "lastId": 1604337406,      "count": 35549628    }  ],  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 8    }  ]}
```

### Symbol price ticker[​](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#symbol-price-ticker) ###

```
{  "id": "043a7cf2-bde3-4888-9604-c8ac41fcba4d",  "method": "ticker.price",  "params": {    "symbol": "BNBBTC"  }}
```

Get the latest market price for a symbol.

If you need access to real-time price updates, please consider using WebSocket Streams:

* [`<symbol>@aggTrade`](/docs/binance-spot-api-docs/web-socket-streams#aggregate-trade-streams)
* [`<symbol>@trade`](/docs/binance-spot-api-docs/web-socket-streams#trade-streams)

**Weight:**Adjusted based on the number of requested symbols:

|Parameter|Weight|
|---------|------|
|`symbol` |  2   |
|`symbols`|  4   |
|  none   |  4   |

**Parameters:**

|  Name   |     Type      |           Mandatory            |          Description          |
|---------|---------------|--------------------------------|-------------------------------|
|`symbol` |    STRING     |               NO               |Query price for a single symbol|
|`symbols`|ARRAY of STRING|Query price for multiple symbols|                               |

Notes:

* `symbol` and `symbols` cannot be used together.

* If no symbol is specified, returns information about all symbols currently trading on the exchange.

**Data Source:**Memory

**Response:**

```
{  "id": "043a7cf2-bde3-4888-9604-c8ac41fcba4d",  "status": 200,  "result": {    "symbol": "BNBBTC",    "price": "0.01361900"  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 2    }  ]}
```

If more than one symbol is requested, response returns an array:

```
{  "id": "e739e673-24c8-4adf-9cfa-b81f30330b09",  "status": 200,  "result": [    {      "symbol": "BNBBTC",      "price": "0.01363700"    },    {      "symbol": "BTCUSDT",      "price": "24267.15000000"    },    {      "symbol": "BNBBUSD",      "price": "331.10000000"    }  ],  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 4    }  ]}
```

### Symbol order book ticker[​](/docs/binance-spot-api-docs/web-socket-api/market-data-requests#symbol-order-book-ticker) ###

```
{  "id": "057deb3a-2990-41d1-b58b-98ea0f09e1b4",  "method": "ticker.book",  "params": {    "symbols": [      "BNBBTC",      "BTCUSDT"    ]  }}
```

Get the current best price and quantity on the order book.

If you need access to real-time order book ticker updates, please consider using WebSocket Streams:

* [`<symbol>@bookTicker`](/docs/binance-spot-api-docs/web-socket-streams#individual-symbol-book-ticker-streams)

**Weight:**Adjusted based on the number of requested symbols:

|Parameter|Weight|
|---------|------|
|`symbol` |  2   |
|`symbols`|  4   |
|  none   |  4   |

**Parameters:**

|  Name   |     Type      |            Mandatory            |          Description           |
|---------|---------------|---------------------------------|--------------------------------|
|`symbol` |    STRING     |               NO                |Query ticker for a single symbol|
|`symbols`|ARRAY of STRING|Query ticker for multiple symbols|                                |

Notes:

* `symbol` and `symbols` cannot be used together.

* If no symbol is specified, returns information about all symbols currently trading on the exchange.

**Data Source:**Memory

**Response:**

```
{  "id": "9d32157c-a556-4d27-9866-66760a174b57",  "status": 200,  "result": {    "symbol": "BNBBTC",    "bidPrice": "0.01358000",    "bidQty": "12.53400000",    "askPrice": "0.01358100",    "askQty": "17.83700000"  },  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 2    }  ]}
```

If more than one symbol is requested, response returns an array:

```
{  "id": "057deb3a-2990-41d1-b58b-98ea0f09e1b4",  "status": 200,  "result": [    {      "symbol": "BNBBTC",      "bidPrice": "0.01358000",      "bidQty": "12.53400000",      "askPrice": "0.01358100",      "askQty": "17.83700000"    },    {      "symbol": "BTCUSDT",      "bidPrice": "23980.49000000",      "bidQty": "0.01000000",      "askPrice": "23981.31000000",      "askQty": "0.01512000"    }  ],  "rateLimits": [    {      "rateLimitType": "REQUEST_WEIGHT",      "interval": "MINUTE",      "intervalNum": 1,      "limit": 6000,      "count": 4    }  ]}
```

