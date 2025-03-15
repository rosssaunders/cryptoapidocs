# ByBit V5 Private Websocket API Documentation

## RATE LIMIT

Rate Limit
==========

IP Rate Limit[​](#ip-rate-limit)
----------

caution

If you receive an **HTTP 403** (Access Denied) response, your IP has been either temporarily or permanently banned.**You should immediately review the below guidelines to ensure your application does not continue to violate the limit.**If you are still banned after 30 minutes, you likely have a permanent ban.

We do not recommend running your application at the very edge of these limits in case abnormal network activity results
in an unexpected violation.

* `GET`/`POST` method (shared):
  * No more than 600 requests are allowed in any 5-second window.

note

All traffic to `api.bybit.com` or `api.bytick.com` share this limit regardless of if it accesses Spot, Derivatives or Options.

After violating the limit your IP will be banned for a set period of time (usually 30 minutes). Continually violating
the limit will result in a permanent ban. We cannot undo permanent bans or shorten temporary bans.

API Rate Limit[​](#api-rate-limit)
----------

caution

If you receive `"ret_msg": "Too many visits!"` in the JSON response, you have hit the API rate limit.

The API rate limit is based on the **rolling time window per second and UID**. In other words, it is per second per UID.
Every request to the API returns response header shown in the code panel:

* `X-Bapi-Limit-Status` - your remaining requests for current endpoint
* `X-Bapi-Limit` - your current limit for current endpoint
* `X-Bapi-Limit-Reset-Timestamp` - the timestamp indicating when your request limit resets if you have exceeded your rate\_limit. Otherwise, this is just the current timestamp (it may not exactly match `timeNow`).

>
>
> Http Response Header Example
>
>

```
▶Response HeadersContent-Type: application/json; charset=utf-8Content-Length: 141X-Bapi-Limit: 100X-Bapi-Limit-Status: 99X-Bapi-Limit-Reset-Timestamp: 1672738134824
```

### API Rate Limit Table[​](#api-rate-limit-table) ###

#### Trade[​](#trade) ####

* Classic account
* UTA1.0 Pro
* UTA2.0 Pro

|       Method       |       Path       |Classic account|upgradable|   |
|--------------------|------------------|:-------------:|----------|---|
|      inverse       |      linear      |     spot      |          |   |
|        POST        | /v5/order/create |     10/s      |   20/s   | Y |
|  /v5/order/amend   |       10/s       |     10/s      |    Y     |   |
|  /v5/order/cancel  |       10/s       |     20/s      |    Y     |   |
|/v5/order/cancel-all|       10/s       |     20/s      |    N     |   |
|        GET         |/v5/order/realtime|     10/s      |   20/s   | N |
| /v5/order/history  |       10/s       |     20/s      |    N     |   |
| /v5/execution/list |       10/s       |     20/s      |    N     |   |

|             Method              |       Path       |UTA1.0 Pro|upgradable|    |    |   |
|---------------------------------|------------------|:--------:|----------|----|----|---|
|             inverse             |      linear      |  option  |   spot   |    |    |   |
|              POST               | /v5/order/create |   10/s   |   10/s   |10/s|20/s| Y |
|         /v5/order/amend         |       10/s       |   10/s   |   10/s   |20/s| Y  |   |
|        /v5/order/cancel         |       10/s       |   10/s   |   10/s   |20/s| Y  |   |
|      /v5/order/cancel-all       |       10/s       |   10/s   |   1/s    |20/s| N  |   |
|     /v5/order/create-batch      |        \-        |   10/s   |   10/s   |20/s| Y  |   |
|      /v5/order/amend-batch      |        \-        |   10/s   |   10/s   |20/s| Y  |   |
|     /v5/order/cancel-batch      |        \-        |   10/s   |   10/s   |20/s| Y  |   |
|/v5/order/disconnected-cancel-all|        \-        |   5/s    |    N     |    |    |   |
|               GET               |/v5/order/realtime|   10/s   |   50/s   | N  |    |   |
|        /v5/order/history        |       10/s       |   50/s   |    N     |    |    |   |
|       /v5/execution/list        |       10/s       |   50/s   |    N     |    |    |   |
|   /v5/order/spot-borrow-check   |        \-        |   50/s   |    N     |    |    |   |

|             Method              |       Path       |UTA2.0 Pro|upgradable|    |   |
|---------------------------------|------------------|:--------:|----------|----|---|
|             inverse             |      linear      |  option  |   spot   |    |   |
|              POST               | /v5/order/create |   10/s   |   10/s   |20/s| Y |
|         /v5/order/amend         |       10/s       |   10/s   |   10/s   | Y  |   |
|        /v5/order/cancel         |       10/s       |   10/s   |   20/s   | Y  |   |
|      /v5/order/cancel-all       |       10/s       |   1/s    |   20/s   | N  |   |
|     /v5/order/create-batch      |       10/s       |   10/s   |   20/s   | Y  |   |
|      /v5/order/amend-batch      |       10/s       |   10/s   |   20/s   | Y  |   |
|     /v5/order/cancel-batch      |       10/s       |   10/s   |   20/s   | Y  |   |
|/v5/order/disconnected-cancel-all|       5/s        |    N     |          |    |   |
|               GET               |/v5/order/realtime|   50/s   |    N     |    |   |
|        /v5/order/history        |       50/s       |    N     |          |    |   |
|       /v5/execution/list        |       50/s       |    N     |          |    |   |
|   /v5/order/spot-borrow-check   |        \-        |   50/s   |    N     |    |   |

#### Position[​](#position) ####

* Classic account
* UTA1.0 Pro
* UTA2.0 Pro

|        Method         |          Path           |Classic account|upgradable|   |
|-----------------------|-------------------------|:-------------:|----------|---|
|        inverse        |         linear          |     spot      |          |   |
|          GET          |    /v5/position/list    |     10/s      |    \-    | N |
|/v5/position/closed-pnl|          10/s           |      \-       |    N     |   |
|         POST          |/v5/position/set-leverage|     10/s      |    \-    | N |

|        Method         |          Path           |UTA1.0 Pro|upgradable|   |   |   |
|-----------------------|-------------------------|:--------:|----------|---|---|---|
|        inverse        |         linear          |  option  |   spot   |   |   |   |
|          GET          |    /v5/position/list    |   10/s   |   50/s   |\- | N |   |
|/v5/position/closed-pnl|          10/s           |   50/s   |    \-    |\- | N |   |
|         POST          |/v5/position/set-leverage|   10/s   |   10/s   |\- |\- | N |

|        Method         |          Path           |UTA2.0 Pro|upgradable|   |   |   |
|-----------------------|-------------------------|:--------:|----------|---|---|---|
|        inverse        |         linear          |  option  |   spot   |   |   |   |
|          GET          |    /v5/position/list    |   50/s   |    \-    | N |   |   |
|/v5/position/closed-pnl|          50/s           |    \-    |    \-    | N |   |   |
|         POST          |/v5/position/set-leverage|   10/s   |   10/s   |\- |\- | N |

#### Account[​](#account) ####

* Classic account
* UTA1.0 Pro
* UTA2.0 Pro

|          Method          |                Path                |    |Limit|upgradable|
|--------------------------|------------------------------------|----|-----|----------|
|           GET            |/v5/account/contract-transaction-log|    |10/s |    N     |
|/v5/account/wallet-balance|          accountType=SPOT          |20/s|  N  |          |
|   accountType=CONTRACT   |                10/s                | N  |     |          |
|   /v5/account/fee-rate   |          category=linear           |10/s|  N  |          |
|      category=spot       |                5/s                 | N  |     |          |
|     category=option      |                5/s                 | N  |     |          |

|          Method           |           Path           |                    |Limit|upgradable|
|---------------------------|--------------------------|--------------------|-----|----------|
|            GET            |/v5/account/wallet-balance|accountType=CONTRACT|50/s |    N     |
|    accountType=UNIFIED    |                          |                    |     |          |
|  /v5/account/withdrawal   |                          |        50/s        |  N  |          |
|/v5/account/borrow-history |                          |        50/s        |  N  |          |
|/v5/account/collateral-info|                          |        50/s        |  N  |          |
|   /v5/asset/coin-greeks   |                          |        50/s        |  N  |          |
|/v5/account/transaction-log|   accountType=UNIFIED    |        50/s        |  N  |          |
|   /v5/account/fee-rate    |     category=linear      |        10/s        |  N  |          |
|       category=spot       |           5/s            |         N          |     |          |
|      category=option      |           5/s            |         N          |     |          |

|          Method           |           Path           |                   |Limit|upgradable|
|---------------------------|--------------------------|-------------------|-----|----------|
|            GET            |/v5/account/wallet-balance|accountType=UNIFIED|50/s |    N     |
|  /v5/account/withdrawal   |                          |       50/s        |  N  |          |
|/v5/account/borrow-history |                          |       50/s        |  N  |          |
|/v5/account/collateral-info|                          |       50/s        |  N  |          |
|   /v5/asset/coin-greeks   |                          |       50/s        |  N  |          |
|/v5/account/transaction-log|   accountType=UNIFIED    |       50/s        |  N  |          |
|   /v5/account/fee-rate    |     category=linear      |       10/s        |  N  |          |
|       category=spot       |           5/s            |         N         |     |          |
|      category=option      |           5/s            |         N         |     |          |

#### Asset[​](#asset) ####

|                     Method                     |               Path                |  Limit   |upgradable|
|------------------------------------------------|-----------------------------------|----------|----------|
|                      GET                       |/v5/asset/transfer/query-asset-info|60 req/min|    N     |
|  /v5/asset/transfer/query-transfer-coin-list   |            60 req/min             |    N     |          |
|  /v5/asset/transfer/query-inter-transfer-list  |            60 req/min             |    N     |          |
|    /v5/asset/transfer/query-sub-member-list    |            60 req/min             |    N     |          |
|/v5/asset/transfer/query-universal-transfer-list|              5 req/s              |    N     |          |
| /v5/asset/transfer/query-account-coins-balance |              5 req/s              |    N     |          |
|         /v5/asset/deposit/query-record         |            100 req/min            |    N     |          |
|   /v5/asset/deposit/query-sub-member-record    |            300 req/min            |    N     |          |
|        /v5/asset/deposit/query-address         |            300 req/min            |    N     |          |
|   /v5/asset/deposit/query-sub-member-address   |            300 req/min            |    N     |          |
|        /v5/asset/withdraw/query-record         |            300 req/min            |    N     |          |
|           /v5/asset/coin/query-info            |              5 req/s              |    N     |          |
|        /v5/asset/exchange/order-record         |            600 req/min            |    N     |          |
|                      POST                      | /v5/asset/transfer/inter-transfer |60 req/min|    N     |
|  /v5/asset/transfer/save-transfer-sub-member   |             20 req/s              |    N     |          |
|     /v5/asset/transfer/universal-transfer      |              5 req/s              |    N     |          |
|           /v5/asset/withdraw/create            |              5 req/s              |    N     |          |
|           /v5/asset/withdraw/cancel            |            60 req/min             |    N     |          |

#### User[​](#user) ####

|          Method          |           Path           | Limit  |upgradable|
|--------------------------|--------------------------|--------|----------|
|           POST           |v5/user/create-sub-member |5 req/s |    N     |
| /v5/user/create-sub-api  |         5 req/s          |   N    |          |
|/v5/user/frozen-sub-member|         5 req/s          |   N    |          |
|   /v5/user/update-api    |         5 req/s          |   N    |          |
| /v5/user/update-sub-api  |         5 req/s          |   N    |          |
|   /v5/user/delete-api    |         5 req/s          |   N    |          |
| /v5/user/delete-sub-api  |         5 req/s          |   N    |          |
|           GET            |/v5/user/query-sub-members|10 req/s|    N     |
|    /v5/user/query-api    |         10 req/s         |   N    |          |
|/v5/user/aff-customer-info|         10 req/s         |   N    |          |

#### Spot Leverage Token[​](#spot-leverage-token) ####

|Method|              Path               | Limit  |Upgradable|
|:-----|:-------------------------------:|--------|----------|
| GET  |/v5/spot-lever-token/order-record|50 req/s|    N     |
| POST |  /v5/spot-lever-token/purchase  |20 req/s|    N     |
| POST |   /v5/spot-lever-token/redeem   |20 req/s|    N     |

#### Spot Margin Trade (UTA)[​](#spot-margin-trade-uta) ####

|For now, there is no limit for endpoints under this category|
|------------------------------------------------------------|

API Rate Limit Rules For VIPs[​](#api-rate-limit-rules-for-vips)
----------

info

The values in the table represent the application upper limit of the corresponding level, and do not mean that users at
this level will automatically enjoy the corresponding API Rate Limit by default.

instructions for batch endpoints

The batch order endpoint, which includes operations for creating, amending, and canceling, has its own rate limit and
does not share it with single requests, *e.g., let's say the rate limit of single create order endpoint is 100/s, and batch create order endpoint
is 100/s, so in this case, I can place 200 linear orders in one second if I use both endpoints to place orders*

#### When category = linear spot or inverse[​](#when-category--linear-spot-or-inverse) ####

* API for batch create/amend/cancel order, the frequency of the API will be consistent with the current configuration,
  but the counting consumption will be consumed according to the actual number of orders. (Number of consumption = number
  of requests \* number of orders included in a single request), and the configuration of business lines is independent of each other.

* The batch APIs allows 1-10 orders/request. For example, if a batch order request is made once and contains 5 orders,
  then the request limit will consume 5.

* If part of the last batch of orders requested within 1s exceeds the limit, the part that exceeds the limit will fail, and
  the part that does not exceed the limit will succeed. For example, in the 1 second, the remaining limit is 5, but a batch request
  containing 8 orders is placed at this time, then the first 5 orders will be successfully placed, and the 6-8th orders
  will report an error exceeding the limit, and these orders will fail.

|              |Classic account & UTA| UTA Pro  |        |           |          |        |
|--------------|:-------------------:|----------|--------|-----------|----------|--------|
|Level\\Product|     **Futures**     |**Option**|**Spot**|**Futures**|**Option**|**Spot**|
|   Default    |        10/s         |   10/s   |  20/s  |   10/s    |   10/s   |  20/s  |
|    VIP 1     |        20/s         |   20/s   |  25/s  |   20/s    |   20/s   |  25/s  |
|    VIP 2     |        40/s         |   40/s   |  30/s  |   40/s    |   40/s   |  30/s  |
|    VIP 3     |        60/s         |   60/s   |  40/s  |   60/s    |   60/s   |  40/s  |
|    VIP 4     |        60/s         |   60/s   |  40/s  |   60/s    |   60/s   |  40/s  |
|    VIP 5     |        60/s         |   60/s   |  40/s  |   60/s    |   60/s   |  40/s  |
| VIP Supreme  |        60/s         |   60/s   |  40/s  |   60/s    |   60/s   |  40/s  |
|     PRO1     |        100/s        |  100/s   |  50/s  |   150/s   |  150/s   | 150/s  |
|     PRO2     |        150/s        |  150/s   |  75/s  |   200/s   |  200/s   | 200/s  |
|     PRO3     |        200/s        |  200/s   | 100/s  |   250/s   |  250/s   | 250/s  |
|     PRO4     |        200/s        |  200/s   | 100/s  |   300/s   |  300/s   | 300/s  |
|     PRO5     |        200/s        |  200/s   | 100/s  |   300/s   |  300/s   | 300/s  |
|     PRO6     |        200/s        |  200/s   | 100/s  |   300/s   |  300/s   | 300/s  |

How to increase API Limit[​](#how-to-increase-api-limit)
----------

* Institutional account will be automatically upgraded or downgraded according to the trading volume
* VIP account needs to contact your VIP relational manager to change the api rate limit

## ENUM

Enums Definitions
==========

### locale[​](#locale) ###

* `de-DE`
* `en-US`
* `es-AR`
* `es-ES`
* `es-MX`
* `fr-FR`
* `kk-KZ`
* `id-ID`
* `uk-UA`
* `ja-JP`
* `ru-RU`
* `th-TH`
* `pt-BR`
* `tr-TR`
* `vi-VN`
* `zh-TW`
* `ar-SA`
* `hi-IN`
* `fil-PH`

### announcementType[​](#announcementtype) ###

* `new_crypto`
* `latest_bybit_news`
* `delistings`
* `latest_activities`
* `product_updates`
* `maintenance_updates`
* `new_fiat_listings`
* `other`

### announcementTag[​](#announcementtag) ###

* `Spot`
* `Derivatives`
* `Spot Listings`
* `BTC`
* `ETH`
* `Trading Bots`
* `USDC`
* `Leveraged Tokens`
* `USDT`
* `Margin Trading`
* `Partnerships`
* `Launchpad`
* `Upgrades`
* `ByVotes`
* `Delistings`
* `VIP`
* `Futures`
* `Institutions`
* `Options`
* `WEB3`
* `Copy Trading`
* `Earn`
* `Bybit Savings`
* `Dual Asset`
* `Liquidity Mining`
* `Shark Fin`
* `Launchpool`
* `NFT GrabPic`
* `Buy Crypto`
* `P2P Trading`
* `Fiat Deposit`
* `Crypto Deposit`
* `Спот`
* `Спот лістинги`
* `Торгові боти`
* `Токени з кредитним плечем`
* `Маржинальна торгівля`
* `Партнерство`
* `Оновлення`
* `Делістинги`
* `Ф'ючерси`
* `Опціони`
* `Копітрейдинг`
* `Bybit Накопичення`
* `Бівалютні інвестиції`
* `Майнінг ліквідності`
* `Купівля криптовалюти`
* `P2P торгівля`
* `Фіатні депозити`
* `Криптодепозити`
* `Копитрейдинг`
* `Торговые боты`
* `Деривативы`
* `P2P`
* `Спот листинги`
* `Деривативи`
* `MT4`
* `Lucky Draw`
* `Unified Trading Account`
* `Єдиний торговий акаунт`
* `Единый торговый аккаунт`
* `Институциональный трейдинг`
* `Інституціональний трейдинг`
* `Делистинг`

### category[​](#category) ###

*Unified Account*

* `spot`
* `linear` USDT perpetual, USDT Futures and USDC contract, including USDC perp, USDC futures
* `inverse` Inverse contract, including Inverse perp, Inverse futures
* `option`

*Classic Account*

* `linear` USDT perp
* `inverse` Inverse contract, including Inverse perp, Inverse futures
* `spot`

### orderStatus[​](#orderstatus) ###

*open status*

* `New` order has been placed successfully
* `PartiallyFilled`
* `Untriggered` Conditional orders are created

*closed status*

* `Rejected`
* `PartiallyFilledCanceled` Only spot has this order status
* `Filled`
* `Cancelled` In derivatives, orders with this status may have an executed qty
* `Triggered` instantaneous state for conditional orders from Untriggered to New
* `Deactivated` UTA: Spot tp/sl order, conditional order, OCO order are cancelled before they are triggered

### timeInForce[​](#timeinforce) ###

* `GTC` GoodTillCancel
* `IOC` ImmediateOrCancel
* `FOK` FillOrKill
* [PostOnly](https://www.bybit.com/en-US/help-center/bybitHC_Article?language=en_US&id=000001051)
* [RPI](https://www.bybit.com/en/help-center/article/Retail-Price-Improvement-RPI-Order) features:
  * **Exclusive Matching**: Only match non-algorithmic users; no execution against orders from Open API.
  * **Post-Only Mechanism**: Act as maker orders, adding liquidity
  * **Lower Priority**: Execute after non-RPI orders at the same price level.
  * **Limited Access**: Initially for select market makers across multiple spot pairs.
  * **Order Book Updates**: Excluded from API but displayed on the GUI.

### createType[​](#createtype) ###

* `CreateByUser`
* `CreateByAdminClosing`
* `CreateBySettle` USDC Futures delivery; Position closed by contract delisted
* `CreateByStopOrder` Futures conditional order
* `CreateByTakeProfit` Futures take profit order
* `CreateByPartialTakeProfit` Futures partial take profit order
* `CreateByStopLoss` Futures stop loss order
* `CreateByPartialStopLoss` Futures partial stop loss order
* `CreateByTrailingStop` Futures trailing stop order
* `CreateByLiq` Laddered liquidation to reduce the required maintenance margin
* `CreateByTakeOver_PassThrough`If the position is still subject to liquidation (i.e., does not meet the required maintenance margin level), the position shall be taken over by the liquidation engine and closed at the bankruptcy price.
* `CreateByAdl_PassThrough` [Auto-Deleveraging(ADL)](https://www.bybit.com/en/help-center/article/Auto-Deleveraging-ADL)
* `CreateByBlock_PassThrough` Order placed via Paradigm
* `CreateByBlockTradeMovePosition_PassThrough` Order created by move position
* `CreateByClosing` The close order placed via web or app position area - web/app
* `CreateByFGridBot` Order created via grid bot - web/app
* `CloseByFGridBot` Order closed via grid bot - web/app
* `CreateByTWAP` Order created by TWAP - web/app
* `CreateByTVSignal` Order created by TV webhook - web/app
* `CreateByMmRateClose` Order created by Mm rate close function - web/app
* `CreateByMartingaleBot` Order created by Martingale bot - web/app
* `CloseByMartingaleBot` Order closed by Martingale bot - web/app
* `CreateByIceBerg` Order created by Ice berg strategy - web/app
* `CreateByArbitrage` Order created by arbitrage - web/app
* `CreateByDdh` Option dynamic delta hedge order - web/app

### execType[​](#exectype) ###

* `Trade`
* `AdlTrade` [Auto-Deleveraging](https://www.bybit.com/en-US/help-center/bybitHC_Article?language=en_US&id=000001124)
* `Funding` [Funding fee](https://www.bybit.com/en-US/help-center/HelpCenterKnowledge/bybitHC_Article?id=000001123&language=en_US)
* `BustTrade` Takeover liquidation
* `Delivery` USDC futures delivery; Position closed by contract delisted
* `Settle` Inverse futures settlement; Position closed due to delisting
* `BlockTrade`
* `MovePosition`
* `UNKNOWN` May be returned by a classic account. Cannot query by this type

### orderType[​](#ordertype) ###

* `Market`
* `Limit`
* `UNKNOWN` is not a valid request parameter value. Is only used in some responses. Mainly, it is used when `execType` is `Funding`.

### stopOrderType[​](#stopordertype) ###

* `TakeProfit`
* `StopLoss`
* `TrailingStop`
* `Stop`
* `PartialTakeProfit`
* `PartialStopLoss`
* `tpslOrder` spot TP/SL order
* `OcoOrder` spot Oco order
* `MmRateClose` On web or app can set MMR to close position
* `BidirectionalTpslOrder` Spot bidirectional tpsl order

### tickDirection[​](#tickdirection) ###

* `PlusTick` price rise
* `ZeroPlusTick` trade occurs at the same price as the previous trade, which occurred at a price higher than that for the trade preceding it
* `MinusTick` price drop
* `ZeroMinusTick` trade occurs at the same price as the previous trade, which occurred at a price lower than that for the trade preceding it

### interval[​](#interval) ###

* `1` `3` `5` `15` `30` `60` `120` `240` `360` `720` minute
* `D` day
* `W` week
* `M` month

### intervalTime[​](#intervaltime) ###

* `5min` `15min` `30min` minute
* `1h` `4h` hour
* `1d` day

### positionIdx[​](#positionidx) ###

* `0` one-way mode position
* `1` Buy side of hedge-mode position
* `2` Sell side of hedge-mode position

### positionStatus[​](#positionstatus) ###

* `Normal`
* `Liq` in the liquidation progress
* `Adl` in the auto-deleverage progress

### rejectReason[​](#rejectreason) ###

* `EC_NoError`
* `EC_Others`
* `EC_UnknownMessageType`
* `EC_MissingClOrdID`
* `EC_MissingOrigClOrdID`
* `EC_ClOrdIDOrigClOrdIDAreTheSame`
* `EC_DuplicatedClOrdID`
* `EC_OrigClOrdIDDoesNotExist`
* `EC_TooLateToCancel`
* `EC_UnknownOrderType`
* `EC_UnknownSide`
* `EC_UnknownTimeInForce`
* `EC_WronglyRouted`
* `EC_MarketOrderPriceIsNotZero`
* `EC_LimitOrderInvalidPrice`
* `EC_NoEnoughQtyToFill`
* `EC_NoImmediateQtyToFill`
* `EC_PerCancelRequest`
* `EC_MarketOrderCannotBePostOnly`
* `EC_PostOnlyWillTakeLiquidity`
* `EC_CancelReplaceOrder`
* `EC_InvalidSymbolStatus`
* `EC_CancelForNoFullFill`
* `EC_BySelfMatch`
* `EC_InCallAuctionStatus` used for pre-market order operation, e.g., during 2nd phase of call auction, cancel order is not allowed, when the cancel request is failed to be rejected by trading server, the request will be rejected by matching box finally
* `EC_QtyCannotBeZero`
* `EC_MarketOrderNoSupportTIF`
* `EC_ReachMaxTradeNum`
* `EC_InvalidPriceScale`
* `EC_BitIndexInvalid`
* `EC_StopBySelfMatch`
* `EC_InvalidSmpType`
* `EC_CancelByMMP`
* `EC_InvalidUserType`
* `EC_InvalidMirrorOid`
* `EC_InvalidMirrorUid`
* `EC_EcInvalidQty`
* `EC_InvalidAmount`
* `EC_LoadOrderCancel`
* `EC_MarketQuoteNoSuppSell`
* `EC_DisorderOrderID`
* `EC_InvalidBaseValue`
* `EC_LoadOrderCanMatch`
* `EC_SecurityStatusFail`
* `EC_ReachRiskPriceLimit`
* `EC_OrderNotExist`
* `EC_CancelByOrderValueZero`
* `EC_CancelByMatchValueZero`
* `EC_ReachMarketPriceLimit`

### accountType[​](#accounttype) ###

#### [UTA2.0](/docs/v5/acct-mode#uta20)[​](#uta20) ####

* `UNIFIED` Unified Trading Account
* `FUND` Funding Account

#### [UTA1.0](/docs/v5/acct-mode#uta10)[​](#uta10) ####

* `CONTRACT` Inverse Derivatives Account (no UDST in this wallet))
* `UNIFIED` Unified Trading Account
* `FUND` Funding Account

#### Classic account[​](#classic-account) ####

Also known as the "standard account".

* `SPOT` Spot Account
* `CONTRACT` Derivatives Account (contain USDT in this wallet)
* `FUND` Funding Account

### transferStatus[​](#transferstatus) ###

* `SUCCESS`
* `PENDING`
* `FAILED`

### depositStatus[​](#depositstatus) ###

* `0` unknown
* `1` toBeConfirmed
* `2` processing
* `3` success (finalised status of a success deposit)
* `4` deposit failed
* `10011` pending to be credited to funding pool
* `10012` Credited to funding pool successfully

### withdrawStatus[​](#withdrawstatus) ###

* `SecurityCheck`
* `Pending`
* `success`
* `CancelByUser`
* `Reject`
* `Fail`
* `BlockchainConfirmed`
* `MoreInformationRequired`
* `Unknown` a rare status

### triggerBy[​](#triggerby) ###

* `LastPrice`
* `IndexPrice`
* `MarkPrice`

### cancelType[​](#canceltype) ###

* `CancelByUser`
* `CancelByReduceOnly` cancelled by [reduceOnly](https://bybit-exchange.github.io/docs/v5/order/create-order)
* `CancelByPrepareLiq` `CancelAllBeforeLiq` cancelled in order to attempt [liquidation prevention](https://www.bybit.com/en/help-center/article/Liquidation-Process-Derivatives-Standard-Account) by freeing up margin
* `CancelByPrepareAdl` `CancelAllBeforeAdl` cancelled due to [ADL](https://www.bybit.com/en/help-center/article/Auto-Deleveraging-ADL)
* `CancelByAdmin`
* `CancelBySettle` cancelled due to delisting contract
* `CancelByTpSlTsClear` TP/SL order cancelled when the position is cleared
* `CancelBySmp` cancelled by [SMP](https://bybit-exchange.github.io/docs/v5/smp)

*Options:*

* `CancelByUser`
* `CancelByReduceOnly`
* `CancelAllBeforeLiq` cancelled due to liquidation
* `CancelAllBeforeAdl` cancelled due to ADL
* `CancelBySettle`
* `CancelByCannotAffordOrderCost`
* `CancelByPmTrialMmOverEquity`
* `CancelByAccountBlocking`
* `CancelByDelivery`
* `CancelByMmpTriggered`
* `CancelByCrossSelfMuch`
* `CancelByCrossReachMaxTradeNum`
* `CancelByDCP`
* `CancelBySmp`

### optionPeriod[​](#optionperiod) ###

* BTC: `7`,`14`,`21`,`30`,`60`,`90`,`180`,`270`days
* ETH: `7`,`14`,`21`,`30`,`60`,`90`,`180`,`270`days
* SOL: `7`,`14`,`21`,`30`,`60`,`90`days

### dataRecordingPeriod[​](#datarecordingperiod) ###

* `5min` `15min` `30min` minute
* `1h` `4h` hour
* `4d` day

### contractType[​](#contracttype) ###

* `InversePerpetual`
* `LinearPerpetual`
* `LinearFutures` USDT/USDC Futures
* `InverseFutures`

### status[​](#status) ###

* `PreLaunch`
* `Trading`
* `Delivering`
* `Closed`

### curAuctionPhase[​](#curauctionphase) ###

* `NotStarted` Pre-market trading is not started
* `Finished` Pre-market trading is finished
  * After the auction, if the pre-market contract fails to enter continues trading phase, it will be delisted and phase="Finished"
  * After the continuous trading, if the pre-market contract fails to be converted to official contract, it will be delisted and phase="Finished"

* `CallAuction` Auction phase of pre-market trading
  * only timeInForce=GTC, orderType=Limit order is allowed to submit
  * TP/SL are not supported; Conditional orders are not supported
  * cannot **modify** the order at this stage
  * order price range: [[preOpenPrice](/docs/v5/market/tickers) x 0.5, [maxPrice](/docs/v5/market/instrument)]

* `CallAuctionNoCancel` Auction no cancel phase of pre-market trading
  * only timeInForce=GTC, orderType=Limit order is allowed to submit
  * TP/SL are not supported; Conditional orders are not supported
  * cannot **modify and cancel** the order at this stage
  * order price range: Buy [[lastPrice](/docs/v5/market/tickers) x 0.5, [markPrice](/docs/v5/market/tickers) x 1.1], Sell [[markPrice](/docs/v5/market/tickers) x 0.9, [maxPrice](/docs/v5/market/instrument)]

* `CrossMatching` cross matching phase
  * cannot **create, modify and cancel** the order at this stage
  * Candle data is released from this stage

* `ContinuousTrading` Continuous trading phase
  * There is no restriction to create, amend, cancel orders
  * orderbook, public trade data is released from this stage

### marginTrading[​](#margintrading) ###

* `none` Regardless of normal account or UTA account, this trading pair does not support margin trading
* `both` For both normal account and UTA account, this trading pair supports margin trading
* `utaOnly` Only for UTA account,this trading pair supports margin trading
* `normalSpotOnly` Only for normal account, this trading pair supports margin trading

### copyTrading[​](#copytrading) ###

* `none` Regardless of normal account or UTA account, this trading pair does not support copy trading
* `both` For both normal account and UTA account, this trading pair supports copy trading
* `utaOnly` Only for UTA account,this trading pair supports copy trading
* `normalOnly` Only for normal account, this trading pair supports copy trading

### type(uta-translog)[​](#typeuta-translog) ###

* `TRANSFER_IN` Assets that transferred into Unified wallet
* `TRANSFER_OUT` Assets that transferred out from Unified wallet
* `TRADE`
* `SETTLEMENT` USDT Perp funding settlement, and USDC Perp funding settlement + USDC 8-hour session settlement
* `DELIVERY` USDC Futures, Option delivery
* `LIQUIDATION`
* `ADL` Auto-Deleveraging
* `AIRDROP`
* `BONUS` Bonus claimed
* `BONUS_RECOLLECT` Bonus expired
* `FEE_REFUND` Trading fee refunded
* `INTEREST` Interest occurred due to borrowing
* `CURRENCY_BUY` Currency convert, and the liquidation for borrowing asset(UTA loan)
* `CURRENCY_SELL` Currency convert, and the liquidation for borrowing asset(UTA loan)
* `BORROWED_AMOUNT_INS_LOAN`
* `PRINCIPLE_REPAYMENT_INS_LOAN`
* `INTEREST_REPAYMENT_INS_LOAN`
* `AUTO_SOLD_COLLATERAL_INS_LOAN` the liquidation for borrowing asset(INS loan)
* `AUTO_BUY_LIABILITY_INS_LOAN` the liquidation for borrowing asset(INS loan)
* `AUTO_PRINCIPLE_REPAYMENT_INS_LOAN`
* `AUTO_INTEREST_REPAYMENT_INS_LOAN`
* `TRANSFER_IN_INS_LOAN` Transfer In when in the liquidation of OTC loan
* `TRANSFER_OUT_INS_LOAN` Transfer Out when in the liquidation of OTC loan
* `SPOT_REPAYMENT_SELL` One-click repayment currency sell
* `SPOT_REPAYMENT_BUY` One-click repayment currency buy
* `TOKENS_SUBSCRIPTION` Spot leverage token subscription
* `TOKENS_REDEMPTION` Spot leverage token redemption
* `AUTO_DEDUCTION` Asset auto deducted by system (roll back)
* `FLEXIBLE_STAKING_SUBSCRIPTION` Byfi flexible stake subscription
* `FLEXIBLE_STAKING_REDEMPTION` Byfi flexible stake redemption
* `FIXED_STAKING_SUBSCRIPTION` Byfi fixed stake subscription
* `PREMARKET_TRANSFER_OUT`
* `PREMARKET_DELIVERY_SELL_NEW_COIN`
* `PREMARKET_DELIVERY_BUY_NEW_COIN`
* `PREMARKET_DELIVERY_PLEDGE_PAY_SELLER`
* `PREMARKET_DELIVERY_PLEDGE_BACK`
* `PREMARKET_ROLLBACK_PLEDGE_BACK`
* `PREMARKET_ROLLBACK_PLEDGE_PENALTY_TO_BUYER`
* `CUSTODY_NETWORK_FEE` fireblocks business
* `CUSTODY_SETTLE_FEE` fireblocks business
* `CUSTODY_LOCK` fireblocks / copper business
* `CUSTODY_UNLOCK` fireblocks business
* `CUSTODY_UNLOCK_REFUND` fireblocks business
* `LOANS_BORROW_FUNDS` crypto loan
* `LOANS_PLEDGE_ASSET` crypto loan repayment
* `BONUS_TRANSFER_IN`
* `BONUS_TRANSFER_OUT`
* `PEF_TRANSFER_IN`
* `PEF_TRANSFER_OUT`
* `PEF_PROFIT_SHARE`

### type(contract-translog)[​](#typecontract-translog) ###

* `TRANSFER_IN` Assets that transferred into (inverse) derivatives wallet
* `TRANSFER_OUT` Assets that transferred out from (inverse) derivatives wallet
* `TRADE`
* `SETTLEMENT` USDT / Inverse Perp funding settlement
* `DELIVERY` Inverse Futures delivery
* `LIQUIDATION`
* `ADL` Auto-Deleveraging
* `AIRDROP`
* `BONUS` Bonus claimed
* `BONUS_RECOLLECT` Bonus expired
* `FEE_REFUND` Trading fee refunded
* `CURRENCY_BUY` Currency convert
* `CURRENCY_SELL` Currency convert
* `AUTO_DEDUCTION` Asset auto deducted by system (roll back)
* `Others`

### unifiedMarginStatus[​](#unifiedmarginstatus) ###

* `1` Classic account
* `3` Unified trading account 1.0
* `4` Unified trading account 1.0 (pro version)
* `5` Unified trading account 2.0
* `6` Unified trading account 2.0 (pro version)

### ltStatus[​](#ltstatus) ###

* `1` LT can be purchased and redeemed
* `2` LT can be purchased, but not redeemed
* `3` LT can be redeemed, but not purchased
* `4` LT cannot be purchased nor redeemed
* `5` Adjusting position

### convertAccountType[​](#convertaccounttype) ###

Check the value of [`unifiedMarginStatus`](#unifiedmarginstatus)

#### [UTA2.0](/docs/v5/acct-mode#uta20)[​](#uta20-1) ####

* `eb_convert_uta` Unified Trading Account
* `eb_convert_funding` Funding Account

#### [UTA1.0](/docs/v5/acct-mode#uta10)[​](#uta10-1) ####

* `eb_convert_inverse` Inverse Derivatives Account (no USDT in this wallet))
* `eb_convert_uta` Unified Trading Account
* `eb_convert_funding` Funding Account

#### Classic account[​](#classic-account-1) ####

Also known as the "standard account"

* `eb_convert_spot` Spot Account
* `eb_convert_contract` Derivatives Account (contain USDT in this wallet)
* `eb_convert_funding` Funding Account

### symbol[​](#symbol) ###

*USDT Perpetual*:

* `BTCUSDT`
* `ETHUSDT`

*USDT Futures*:

* `BTCUSDT-21FEB25`
* `ETHUSDT-14FEB25`

*USDC Perpetual*:

* `BTCPERP`
* `ETHPERP`

*USDC Futures*:

* `BTC-24MAR23`

*Inverse Perpetual*:

* `BTCUSD`
* `ETHUSD`

*Inverse Futures*:

* `BTCUSDH23` H: First quarter; 23: 2023
* `BTCUSDM23` M: Second quarter; 23: 2023
* `BTCUSDU23` U: Third quarter; 23: 2023
* `BTCUSDZ23` Z: Fourth quarter; 23: 2023

*Spot*:

* `BTCUSDT`
* `ETHUSDC`

*Option*:

* `BTC-13FEB25-89000-P-USDT` USDT Option
* `ETH-28FEB25-2800-C` USDC Option

### vipLevel[​](#viplevel) ###

* No VIP
* VIP-1
* VIP-2
* VIP-3
* VIP-4
* VIP-5
* VIP-Supreme
* PRO-1
* PRO-2
* PRO-3
* PRO-4
* PRO-5

### adlRankIndicator[​](#adlrankindicator) ###

* `0` default value of empty position
* `1`
* `2`
* `3`
* `4`
* `5`

### smpType[​](#smptype) ###

* default: `None`
* `CancelMaker`
* `CancelTaker`
* `CancelBoth`

### Spot Fee Currency Instruction[​](#spot-fee-currency-instruction) ###

with the example of BTCUSDT:

* Is makerFeeRate positive?
  * TRUE
    * Side = Buy -\> base currency (BTC)
    * Side = Sell -\> quote currency (USDT)

  * FALSE
    * IsMakerOrder = TRUE
      * Side = Buy -\> quote currency (USDT)
      * Side = Sell -\> base currency (BTC)

    * IsMakerOrder = FALSE
      * Side = Buy -\> base currency (BTC)
      * Side = Sell -\> quote currency (USDT)

## ERROR

Error Codes
==========

HTTP Code[​](#http-code)
----------

|                              Code                              |                                                              Description                                                              |
|:--------------------------------------------------------------:|:--------------------------------------------------------------------------------------------------------------------------------------|
|                              400                               |                          Bad request. Need to send the request with **GET** / **POST** (must be capitalized)                          |
|[401](/docs/pilot-feature#normal-account-is-supported-by-v5-api)|         Invalid request. 1. Need to use the correct key to access; 2. Need to put authentication params in the request header         |
|                              403                               |Forbidden request. Possible causes: 1. IP rate limit breached; 2. You send GET request with an empty json body; 3. You are using U.S IP|
|[404](/docs/pilot-feature#normal-account-is-supported-by-v5-api)|                    Cannot find path. Possible causes: 1. Wrong path; 2. Category value does not match account mode                    |
|                              429                               |                                  System level frequency protection. Please retry when encounter this                                  |

WS OE General code[​](#ws-oe-general-code)
----------

|Code |                                                                             Description                                                                              |
|:---:|:---------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|10404|                                                   1. op type is not found; 2. `category` is not correct/supported                                                    |
|10429|                                                                  System level frequency protection                                                                   |
|10003|                                                                 Too many sessions under the same UID                                                                 |
|10016|                                                          1. internal server error; 2. Service is restarting                                                          |
|10019|ws trade service is restarting, do not accept new request, but the request in the process is not affected. You can build new connection to be routed to normal service|
|20003|                                                             Too frequent requests under the same session                                                             |
|20006|                                                                         reqId is duplicated                                                                          |

UTA[​](#uta)
----------

|      Code       |                                                                                       Description                                                                                        |
|:---------------:|:-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|        0        |                                                                                            OK                                                                                            |
|       429       |                                            The trading service is experiencing a high server load. Please retry if you encounter this issue.                                             |
|     \-2015      |                                                                             (Spot) Your api key has expired                                                                              |
|      33004      |                                                                          (Derivatives) Your api key has expired                                                                          |
|      10000      |                                                                                      Server Timeout                                                                                      |
|      10001      |                                                                                 Request parameter error                                                                                  |
|      10002      |                                                                     The request time exceeds the time window range.                                                                      |
|      10003      |                             API key is invalid. Check whether the key and domain are matched, there are 4 env: mainnet, testnet, mainnet-demo, testnet-demo                              |
|      10004      |                                                              Error sign, please check your signature generation algorithm.                                                               |
|      10005      |                                                                Permission denied, please check your API key permissions.                                                                 |
|      10006      |                                                                      Too many visits. Exceeded the API Rate Limit.                                                                       |
|      10007      |                                                                               User authentication failed.                                                                                |
|      10008      |                                                                      Common banned, please check your account mode                                                                       |
|      10009      |                                                                                   IP has been banned.                                                                                    |
|      10010      |                                                              Unmatched IP, please check your API key's bound IP addresses.                                                               |
|      10014      |                                                                                Invalid duplicate request.                                                                                |
|      10016      |                                                                                      Server error.                                                                                       |
|      10017      |                                                                                     Route not found.                                                                                     |
|    ~~10018~~    |                                                                             ~~Exceeded the IP Rate Limit.~~                                                                              |
|      10024      |                                                                                Compliance rules triggered                                                                                |
|      10027      |                                                                                 Transactions are banned.                                                                                 |
|      10029      |                                                              The requested symbol is invalid, please check symbol whitelist                                                              |
|      10028      |                                                                  The API can only be accessed by unified account users.                                                                  |
|      30133      |                                                OTC loan: The symbol you select for USDT Perpetual is not allowed by Institutional Lending                                                |
|      30134      |                                                OTC loan: The symbol you select for USDC Contract is not allowed by Institutional Lending                                                 |
|      30135      |                                 The leverage you select for USDT Perpetual trading cannot exceed the maximum leverage allowed by Institutional Lending.                                  |
|      30136      |                            The leverage you select for USDC Perpetual or Futures trading cannot exceed the maximum leverage allowed by Institutional Lending.                            |
|      40004      |                                               the order is modified during the process of replacing , please check the order status again                                                |
|     100028      |                                                                   The API cannot be accessed by unified account users.                                                                   |
|     110001      |                                                                                   Order does not exist                                                                                   |
|     110003      |                                       Order price exceeds the [allowable range](https://www.bybithelp.com/en-US/s/article/Contract-Price-Limits).                                        |
|     110004      |                                                                              Wallet balance is insufficient                                                                              |
|     110005      |                                                                                     position status                                                                                      |
|     110006      |                                                            The assets are estimated to be unable to cover the position margin                                                            |
|     110007      |                                                                            Available balance is insufficient                                                                             |
|     110008      |                                                                        The order has been completed or cancelled.                                                                        |
|     110009      |                                          The number of stop orders exceeds the maximum allowable limit. You can find references in our API doc.                                          |
|     110010      |                                                                               The order has been cancelled                                                                               |
|     110011      |                                                               Liquidation will be triggered immediately by this adjustment                                                               |
|     110012      |                                                                             Insufficient available balance.                                                                              |
|     110013      |                                                                       Cannot set leverage due to risk limit level.                                                                       |
|     110014      |                                                                 Insufficient available balance to add additional margin.                                                                 |
|     110015      |                                                                          The position is in cross margin mode.                                                                           |
|   ~~110016~~    |                                 ~~The quantity of contracts requested exceeds the risk limit, please adjust your risk limit level before trying again~~                                  |
|     110017      |                                                                              Reduce-only rule not satisfied                                                                              |
|     110018      |                                                                                   User ID is illegal.                                                                                    |
|     110019      |                                                                                   Order ID is illegal.                                                                                   |
|     110020      |                                                                     Not allowed to have more than 500 active orders.                                                                     |
|     110021      |                                                              Not allowed to exceeded position limits due to Open Interest.                                                               |
|     110022      |                                                   Quantity has been restricted and orders cannot be modified to increase the quantity.                                                   |
|     110023      |                           Currently you can only reduce your position on this contract. please check our announcement or contact customer service for details.                           |
|     110024      |                                                         You have an existing position, so the position mode cannot be switched.                                                          |
|     110025      |                                                                           Position mode has not been modified.                                                                           |
|     110026      |                                                                    Cross/isolated margin mode has not been modified.                                                                     |
|     110027      |                                                                              Margin has not been modified.                                                                               |
|     110028      |                                                         You have existing open orders, so the position mode cannot be switched.                                                          |
|     110029      |                                                                       Hedge mode is not supported for this symbol.                                                                       |
|   ~~110030~~    |                                                                                  ~~Duplicate orderId~~                                                                                   |
|     110031      |                                                             Non-existing risk limit info, please check the risk limit rules.                                                             |
|     110032      |                                                                                     Order is illegal                                                                                     |
|     110033      |                                                                      You can't set margin without an open position                                                                       |
|     110034      |                                                                                 There is no net position                                                                                 |
|     110035      |                                                               Cancellation of orders was not completed before liquidation                                                                |
|     110036      |                                                             You are not allowed to change leverage due to cross margin mode.                                                             |
|     110037      |                                                                       User setting list does not have this symbol                                                                        |
|     110038      |                                                           You are not allowed to change leverage due to portfolio margin mode.                                                           |
|     110039      |                                                            Maintenance margin rate is too high. This may trigger liquidation.                                                            |
|     110040      |                                                         The order will trigger a forced liquidation, please re-submit the order.                                                         |
|     110041      |                                                          Skip liquidation is not allowed when a position or maker order exists                                                           |
|     110042      |                                                Currently,due to pre-delivery status, you can only reduce your position on this contract.                                                 |
|     110043      |                                                                           Set leverage has not been modified.                                                                            |
|     110044      |                                                                            Available margin is insufficient.                                                                             |
|     110045      |                                                                             Wallet balance is insufficient.                                                                              |
|     110046      |                                                              Liquidation will be triggered immediately by this adjustment.                                                               |
|     110047      |                                                           Risk limit cannot be adjusted due to insufficient available margin.                                                            |
|     110048      |                                           Risk limit cannot be adjusted as the current/expected position value exceeds the revised risk limit.                                           |
|     110049      |                                                                              Tick notes can only be numbers                                                                              |
|     110050      |                                                                                       Invalid coin                                                                                       |
|     110051      |                                                     The user's available balance cannot cover the lowest price of the current market                                                     |
|     110052      |                                                                 Your available balance is insufficient to set the price                                                                  |
|     110053      |                                                 The user's available balance cannot cover the current market price and upper limit price                                                 |
|     110054      |                                     This position has at least one take profit link order, so the take profit and stop loss mode cannot be switched                                      |
|     110055      |                                      This position has at least one stop loss link order, so the take profit and stop loss mode cannot be switched                                       |
|     110056      |                                    This position has at least one trailing stop link order, so the take profit and stop loss mode cannot be switched                                     |
|     110057      |                                                              Conditional order or limit order contains TP/SL related params                                                              |
|     110058      |                                               You can't set take profit and stop loss due to insufficient size of remaining position size.                                               |
|     110059      |                                                             Not allowed to modify the TP/SL of a partially filled open order                                                             |
|     110060      |                                                                 Under full TP/SL mode, it is not allowed to modify TP/SL                                                                 |
|     110061      |                                                              Not allowed to have more than 20 TP/SLs under Partial tpSlMode                                                              |
|     110062      |                                                                  There is no MMP information of the institution found.                                                                   |
|     110063      |                                                               Settlement in progress! {{key0}} not available for trading.                                                                |
|     110064      |                                                   The modified contract quantity cannot be less than or equal to the filled quantity.                                                    |
|     110065      |                                                      MMP hasn't yet been enabled for your account. Please contact your BD manager.                                                       |
|     110066      |                                                                            Trading is currently not allowed.                                                                             |
|     110067      |                                                                            Unified account is not supported.                                                                             |
|     110068      |                                                                            Leveraged trading is not allowed.                                                                             |
|     110069      |                                                                      Ins lending customer is not allowed to trade.                                                                       |
|     110070      |                                                                              ETP symbols cannot be traded.                                                                               |
|     110071      |            Sorry, we're revamping the Unified Margin Account! Currently, new upgrades are not supported. If you have any questions, please contact our 24/7 customer support.            |
|     110072      |                                                                                OrderLinkedID is duplicate                                                                                |
|     110073      |                                                                                  Set margin mode failed                                                                                  |
|     110075      |                                                                                   RiskId not modified                                                                                    |
|~~110075~~ 182021|                     Cannot enable spot margin while in isolated margin mode. Please switch to cross margin mode or portfolio margin mode to trade spot with margin.                      |
|     110076      |                                                                        Only isolated mode can set auto-add-margin                                                                        |
|     110077      |                                                                                  Pm mode cannot support                                                                                  |
|     110078      |                                                                       Added margin more than max can reduce margin                                                                       |
|     110079      |                                                         The order is processing and can not be operated, please try again later                                                          |
|     110080      |  Operations Restriction: The current LTV ratio of your Institutional Lending has hit the liquidation threshold. Assets in your account are being liquidated (trade/risk limit/leverage)  |
|     110082      |                                          You cannot lift Reduce-Only restrictions, as no Reduce-Only restrictions are applied to your position                                           |
|     110083      |                                                Reduce-Only restrictions must be lifted for both Long and Short positions at the same time                                                |
|     110085      |                           The risk limit and margin ratio for this contract has been updated, please select a supported risk limit and place your order again                            |
|     110086      |                           Current order leverage exceeds the maximum available for your current Risk Limit tier. Please lower leverage before placing an order                           |
|     110087      |                                        Leverage for Perpetual or Futures contracts cannot exceed the maximum allowed for your Institutional loan                                         |
|     110088      |                                                                              Please Upgrade to UTA to trade                                                                              |
|     110089      |                                                                           Exceeds the maximum risk limit level                                                                           |
|     110090      |                Order placement failed as your position may exceed the max limit. Please adjust your leverage to {{leverage}} or below to increase the max. position limit                |
|     110092      |                                                            expect Rising, but trigger\_price[XXXXX] \<= current[XXXXX]??laste                                                            |
|     110093      |                                                            expect Falling, but trigger\_price[XXXXX] \>= current[XXXXX]??last                                                            |
|     110094      |                                                                        Order notional value below the lower limit                                                                        |
|     110095      |                                                 You cannot create, modify or cancel Pre-Market Perpetual orders during the Call Auction.                                                 |
|     110096      |                                                           Pre-Market Perpetual Trading does not support Portfolio Margin mode.                                                           |
|     110097      |              Non-UTA users cannot access Pre-Market Perpetual Trading. To place, modify or cancel Pre-Market Perpetual orders, please upgrade your Standard Account to UTA.              |
|     110098      |                                                         Only Good-Till-Canceled (GTC) orders are supported during Call Auction.                                                          |
|     110099      |                                                    You cannot create TP/SL orders during the Call Auction for Pre-Market Perpetuals.                                                     |
|     110100      |                                              You cannot place, modify, or cancel Pre-Market Perpetual orders when you are in Demo Trading.                                               |
|     110101      |                                     Trading inverse contracts under Cross and Portfolio modes requires enabling the settlement asset as collateral.                                      |
|     110102      |                                        The user does not support trading Inverse contracts - copy trading pro, Ins loan account are not supported                                        |
|     110103      |                                                                    Only Post-Only orders are available at this stage                                                                     |
|     110104      |                                                 The LTV for ins Loan has exceeded the limit, and opening inverse contracts is prohibited                                                 |
|     110105      |                                                 The LTV for ins Loan has exceeded the limit, and trading inverse contracts is prohibited                                                 |
|     110106      |                                           Restrictions on Ins Loan; inverse contracts are not on the whitelist and are not allowed for trading                                           |
|     110107      |                                               Restrictions on ins Loan; leverage exceeding the limit for inverse contracts is not allowed.                                               |
|     170346      |                                                                    Settle coin is not a collateral coin, cannot trade                                                                    |
|     181017      |                                                                             OrderStatus must be final status                                                                             |
|     182100      |                                                                  Compulsory closing of positions, no repayment allowed                                                                   |
|     182101      |                                                                    Failed repayment, insufficient collateral balance                                                                     |
|     182102      |                                                            Failed repayment, there are no liabilities in the current currency                                                            |
|     182103      |                                                                      Institutional lending users are not supported                                                                       |
|     182108      |                                                    Switching failed, margin verification failed, please re-adjust the currency status                                                    |
|     182110      |                                                                                     Failed to switch                                                                                     |
|     182111      |                                        The requested currency has a non guaranteed gold currency or does not support switching status currencies                                         |
|     182112      |                                                                           Duplicate currency, please re-adjust                                                                           |
|     3100181     |                                                                                   UID can not be null                                                                                    |
|     3100197     |                                                                        Temporary banned due to the upgrade to UTA                                                                        |
|     3200316     |                USDC Options Trading Restriction: The current LTV ratio for your Institutional Lending has reached the maximum allowable amount for USDC Options trading.                 |
|     3200317     |        USDC Options Open Position Restriction: The current LTV ratio for your Institutional Lending has reached the maximum allowable amount for opening USDC Options positions.         |
|     3100326     |                                                                                   BaseCoin is required                                                                                   |
|     3200403     |                                                                           isolated margin can not create order                                                                           |
|     3200419     |                                               Unable to switch to Portfolio margin due to active pre-market Perpetual orders and positions                                               |
|     3200320     |Operations Restriction: The current LTV ratio of your Institutional Lending has hit the liquidation threshold. Assets in your account are being liquidated. (margin mode or spot leverage)|
|     3400208     |                                                          You have unclosed hedge mode or isolated mode USDT perpetual positions                                                          |
|     3400209     |                                    You have USDT perpetual positions, so upgrading is prohibited for 10 minutes before and after the hour every hour                                     |
|     3400210     |                                                                  The risk rate of your Derivatives account is too high                                                                   |
|     3400211     |                                                                 Once upgraded, the estimated risk rate will be too high                                                                  |
|     3400212     |                          You have USDC perpetual positions or Options positions, so upgrading is prohibited for 10 minutes before and after the hour every hour                          |
|     3400213     |                                                                The risk rate of your USDC Derivatives account is too high                                                                |
|     3400052     |                                                                        You have uncancelled USDC perpetual orders                                                                        |
|     3400053     |                                                                           You have uncancelled Options orders                                                                            |
|     3400054     |                                                                        You have uncancelled USDT perpetual orders                                                                        |
|     3400214     |                                                                           Server error, please try again later                                                                           |
|     3400071     |                                                                              The net asset is not satisfied                                                                              |
|     3401010     |                                                                Cannot switch to PM mode (for copy trading master trader)                                                                 |
|     3400139     |                                       The total value of your positions and orders has exceeded the risk limit for a Perpetual or Futures contract                                       |
|     500010      |                                                             The sub-account specified does not belong to the parent account                                                              |
|     500011      |                                                         The Uid 592334 provided is not associated with a Unified Trading Account                                                         |

Spot Trade[​](#spot-trade)
----------

| Code |                                                                                               Description                                                                                               |
|:----:|:--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|170001|                                                                                             Internal error.                                                                                             |
|170005|                                                                         Too many new orders; current limit is %s orders per %s.                                                                         |
|170007|                                                                            Timeout waiting for response from backend server.                                                                            |
|170010|                                             Purchase failed: Exceed the maximum position limit of leveraged tokens, the current available limit is %s USDT                                              |
|170011|                                                                "Purchase failed: Exceed the maximum position limit of innovation tokens,                                                                |
|170019|                                                                        the current available limit is ''{{.replaceKey0}}'' USDT"                                                                        |
|170031|                                                                                     The feature has been suspended                                                                                      |
|170032|                                                                                  Network error. Please try again later                                                                                  |
|170033|                                                                                   margin Insufficient account balance                                                                                   |
|170034|                                                                               Liability over flow in spot leverage trade!                                                                               |
|170035|                                                                                 Submitted to the system for processing!                                                                                 |
|170036|                                               You haven't enabled Cross Margin Trading yet. To do so, please head to the PC trading site or the Bybit app                                               |
|170037|                                                                       Cross Margin Trading not yet supported by the selected coin                                                                       |
|170105|                                                                                        Parameter '%s' was empty.                                                                                        |
|170115|                                                                                          Invalid timeInForce.                                                                                           |
|170116|                                                                                           Invalid orderType.                                                                                            |
|170117|                                                                                              Invalid side.                                                                                              |
|170121|                                                                                             Invalid symbol.                                                                                             |
|170124|                                                                                         Order amount too large.                                                                                         |
|170130|                                                                                Data sent for paramter '%s' is not valid.                                                                                |
|170131|                                                                                          Balance insufficient                                                                                           |
|170132|                                                                                          Order price too high.                                                                                          |
|170133|                                                                                   Order price lower than the minimum.                                                                                   |
|170134|                                                                                      Order price decimal too long.                                                                                      |
|170135|                                                                                        Order quantity too large.                                                                                        |
|170136|                                                                                 Order quantity lower than the minimum.                                                                                  |
|170137|                                                                                      Order volume decimal too long                                                                                      |
|170139|                                                                                         Order has been filled.                                                                                          |
|170140|                                                                               Transaction amount lower than the minimum.                                                                                |
|170141|                                                                                         Duplicate clientOrderId                                                                                         |
|170142|                                                                                         Order has been canceled                                                                                         |
|170143|                                                                                      Cannot be found on order book                                                                                      |
|170144|                                                                                          Order has been locked                                                                                          |
|170145|                                                                              This order type does not support cancellation                                                                              |
|170146|                                                                                         Order creation timeout                                                                                          |
|170147|                                                                                       Order cancellation timeout                                                                                        |
|170148|                                                                                  Market order amount decimal too long                                                                                   |
|170149|                                                                                           Create order failed                                                                                           |
|170150|                                                                                           Cancel order failed                                                                                           |
|170151|                                                                                    The trading pair is not open yet                                                                                     |
|170157|                                                                            The trading pair is not available for api trading                                                                            |
|170159|                                                 Market Order is not supported within the first %s minutes of newly launched pairs due to risk control.                                                  |
|170190|                                                                                     Cancel order has been finished                                                                                      |
|170191|                                                                              Can not cancel order, please try again later                                                                               |
|170192|                                                                                 Order price cannot be higher than %s .                                                                                  |
|170193|                                                                                Buy order price cannot be higher than %s.                                                                                |
|170194|                                                                                Sell order price cannot be lower than %s.                                                                                |
|170195|                                                      Please note that your order may not be filled. ETP buy order price deviates from risk control                                                      |
|170196|                                                     Please note that your order may not be filled. ETP sell order price deviates from risk control                                                      |
|170197|                                       Your order quantity to buy is too large. The filled price may deviate significantly from the market price. Please try again                                       |
|170198|                                      Your order quantity to sell is too large. The filled price may deviate significantly from the market price. Please try again                                       |
|170199|                                           Your order quantity to buy is too large. The filled price may deviate significantly from the nav. Please try again.                                           |
|170200|                                          Your order quantity to sell is too large. The filled price may deviate significantly from the nav. Please try again.                                           |
|170201|                                                                                      Invalid orderFilter parameter                                                                                      |
|170202|                                                                                      Please enter the TP/SL price.                                                                                      |
|170203|                                                                             trigger price cannot be higher than 110% price.                                                                             |
|170204|                                                                             trigger price cannot be lower than 90% of qty.                                                                              |
|170206|                                                          Stop\_limit Order is not supported within the first 5 minutes of newly launched pairs                                                          |
|170207|                                                                             The loan amount of the platform is not enough.                                                                              |
|170210|                                                                                           New order rejected.                                                                                           |
|170212|                                                                                     Cancel order request processing                                                                                     |
|170213|                                                                                          Order does not exist.                                                                                          |
|170215|                               Spot Trading (Buy) Restriction: The current LTV ratio of your institutional lending has reached the maximum allowable amount for buy orders                               |
|170216|                                              The leverage you select for Spot Trading cannot exceed the maximum leverage allowed by Institutional Lending                                               |
|170217|                                                                        Only LIMIT-MAKER order is supported for the current pair.                                                                        |
|170218|                                                                         The LIMIT-MAKER order is rejected due to invalid price.                                                                         |
|170219|                                                                              UID {{xxx}} is not available to this feature                                                                               |
|170220|                                 Spot Trading Restriction: The current LTV ratio of your institutional lending has reached the maximum allowable amount for Spot trading                                 |
|170221|                                                                                        This coin does not exist.                                                                                        |
|170222|                                                                                  Too many requests in this time frame.                                                                                  |
|170223|                                                             Your Spot Account with Institutional Lending triggers an alert or liquidation.                                                              |
|170224|                                                                                You're not a user of the Innovation Zone.                                                                                |
|170226|                                                                        Your Spot Account for Margin Trading is being liquidated.                                                                        |
|170227|                                                                                     This feature is not supported.                                                                                      |
|170228|                                                            The purchase amount of each order exceeds the estimated maximum purchase amount.                                                             |
|170229|                                                                The sell quantity per order exceeds the estimated maximum sell quantity.                                                                 |
|170230|                                                        Operations Restriction: Due to the deactivation of Margin Trading for institutional loan                                                         |
|170234|                                                                                              System Error                                                                                               |
|170241|To proceed with trading, users must read through and confirm that they fully understand the project's risk disclosure document. For App users, please update your Bybit App to version 4.16.0 to process.|
|170310|                                                                                       Order modification timeout                                                                                        |
|170311|                                                                                        Order modification failed                                                                                        |
|170312|                                                                             The current order does not support modification                                                                             |
|170313|                                                                The modified contract quantity cannot be less than to the filled quantity                                                                |
|170341|                                                                              Request order quantity exceeds maximum limit                                                                               |
|170344|                                                                                Symbol is not supported on Margin Trading                                                                                |
|170355|                                                                        RPI orders are restricted to approved market makers only                                                                         |
|170709|                                                                     OTC loan: The select trading pair is not in the whitelist pair                                                                      |
|170810|                                                                   Cannot exceed maximum of 500 conditional, TP/SL and active orders.                                                                    |

Spot Leverage Token[​](#spot-leverage-token)
----------

| Code |                                                     Description                                                     |
|:----:|:--------------------------------------------------------------------------------------------------------------------|
|175000|                                          The serialNum is already in use.                                           |
|175001|                           Daily purchase limit has been exceeded. Please try again later.                           |
|175002|                         There's a large number of purchase orders. Please try again later.                          |
|175003|                        Insufficient available balance. Please make a deposit and try again.                         |
|175004|                          Daily redemption limit has been exceeded. Please try again later.                          |
|175005|                        There's a large number of redemption orders. Please try again later.                         |
|175006|                        Insufficient available balance. Please make a deposit and try again.                         |
|175007|                                                  Order not found.                                                   |
|175008|                                         Purchase period hasn't started yet.                                         |
|175009|                                    Purchase amount has exceeded the upper limit.                                    |
|175010|          You haven't passed the quiz yet! To purchase and/or redeem an LT, please complete the quiz first.          |
|175012|                                        Redemption period hasn't started yet.                                        |
|175013|                                   Redemption amount has exceeded the upper limit.                                   |
|175014|                                 Purchase of the LT has been temporarily suspended.                                  |
|175015|                                Redemption of the LT has been temporarily suspended.                                 |
|175016|                           Invalid format. Please check the length and numeric precision.                            |
|175017|Failed to place order：Exceed the maximum position limit of leveraged tokens, the current available limit is XXXX USDT|
|175027|           Subscriptions and redemptions are temporarily unavailable while account upgrade is in progress            |

Spot Margin Trade[​](#spot-margin-trade)
----------

| Code |                                                     Description                                                     |
|:----:|:--------------------------------------------------------------------------------------------------------------------|
|176002|                    Query user account info error. Confirm that if you have completed quiz in GUI                    |
|176003|                                            Query user loan history error                                            |
|176004|                                   Query order history start time exceeds end time                                   |
|176005|                                                  Failed to borrow                                                   |
|176006|                                                  Repayment Failed                                                   |
|176007|                                                   User not found                                                    |
|176008|             You haven't enabled Cross Margin Trading yet. To do so, please head to the PC trading site              |
|176009|            You haven't enabled Cross Margin Trading yet. Confirm that if you have turned on margin trade            |
|176010|                                        Failed to locate the coins to borrow                                         |
|176011|                             Cross Margin Trading not yet supported by the selected coin                             |
|176012|                                                 Pair not available                                                  |
|176013|                             Cross Margin Trading not yet supported by the selected pair                             |
|176014|                                             Repeated repayment requests                                             |
|176015|                                           Insufficient available balance                                            |
|176016|                                                No repayment required                                                |
|176017|                                  Repayment amount has exceeded the total liability                                  |
|176018|                                               Settlement in progress                                                |
|176019|                                               Liquidation in progress                                               |
|176020|                                         Failed to locate repayment history                                          |
|176021|                                             Repeated borrowing requests                                             |
|176022|                                     Coins to borrow not generally available yet                                     |
|176023|                                     Pair to borrow not generally available yet                                      |
|176024|                                                 Invalid user status                                                 |
|176025|                  Amount to borrow cannot be lower than the min. amount to borrow (per transaction)                  |
|176026|                 Amount to borrow cannot be larger than the max. amount to borrow (per transaction)                  |
|176027|                      Amount to borrow cannot be higher than the max. amount to borrow per user                      |
|176028|                             Amount to borrow has exceeded Bybit's max. amount to borrow                             |
|176029|                      Amount to borrow has exceeded the user's estimated max. amount to borrow                       |
|176030|                                             Query user loan info error                                              |
|176031|                       Number of decimals for borrow amount has exceeded the maximum precision                       |
|176034|                                         The leverage ratio is out of range                                          |
|176035|                               Failed to close the leverage switch during liquidation                                |
|176036|                             Failed to adjust leverage switch during forced liquidation                              |
|176037|                               For non-unified transaction users, the operation failed                               |
|176038|                        The spot leverage is closed and the current operation is not allowed                         |
|176039|                                     Borrowing, current operation is not allowed                                     |
|176040|                  There is a spot leverage order, and the adjustment of the leverage switch failed!                  |
|176132|                       Number of decimals for repay amount has exceeded the maximum precision                        |
|176133|                  Liquidation may be triggered! Please adjust your transaction amount and try again                  |
|176134|                                    Account has been upgraded (upgrading) to UTA                                     |
|176135|                                               Failed to get bond data                                               |
|176136|                                              Failed to get borrow data                                              |
|176137|                                            Failed to switch user status                                             |
|176138|                 You need to repay all your debts before closing your disabling cross margin account                 |
|176139|             Sorry, you are not eligible to enable cross margin, as you have already enabled OTC lending             |
|176201|                        Account exception. Check if the UID is bound to an institutional loan                        |
|182104|This action could not be completed as your Unified Margin Account's IM/MM utilization rate has exceeded the threshold|
|182105|                                        Adjustment failed, user is upgrading                                         |
|182106|                               Adjustment failed, user forced liquidation in progress.                               |
|182107|                                 Adjustment failed, Maintenance Margin Rate too high                                 |

Asset[​](#asset)
----------

|   Code   |                                                                      Description                                                                       |
|:--------:|:-------------------------------------------------------------------------------------------------------------------------------------------------------|
|  131001  |                                                                   openapi svc error                                                                    |
|  131002  |                                                                    Parameter error                                                                     |
|  131002  |                                                Withdraw address chain or destination tag are not equal                                                 |
|  131003  |                                                                     Internal error                                                                     |
|  131004  |                                                                       KYC needed                                                                       |
|  131066  |                     This address does not support withdrawals for the time being. Please switch to another address for withdrawing                     |
|  131067  |                              Travel rule verification failed, please contact the target exchange. Travel rule for KR user                              |
|  131068  |                          Travel rule information is insufficient, please provide additional details. Travel rule for KR user                           |
|  131069  |                           Unable to withdraw to the receipt, please contact the target the exchange. Travel rule for KR user                           |
|  131070  |                                 The recipient's name is mismatched with the targeted exchange. Travel rule for KR user                                 |
|  131071  |                                       The recipient has not undergone KYC verification. Travel rule for KR user                                        |
|  131072  |                               Your withdrawal currency is not supported by the target exchange. Travel rule for KR user                                |
|  131073  |                             Your withdrawal address has not been included in the target exchange. Travel rule for KR user                              |
|  131074  |                             Beneficiary info is required, please refer to the latest api document. Travel rule for KR user                             |
|  131075  |                                                            InternalAddressCannotBeYourself                                                             |
|  131076  |                                                       internal transfer not support sub-accounts                                                       |
|  131077  |                                                                 receive user not exist                                                                 |
|  131078  |                                                          receive user deposit has been banned                                                          |
|  131079  |                                                                 receive user need kyc                                                                  |
|  131080  |                                                             User left retry times is zero                                                              |
|  131081  |                                                             Do not input memo/tag,please.                                                              |
|  131082  |                                                               Do not repeat the request                                                                |
|  131083  |                                                        Withdraw only allowed from address book                                                         |
|  131084  |                                                        Withdraw failed because of Uta Upgrading                                                        |
|  131085  |                             Withdrawal amount is greater than your availale balance (the deplayed withdrawal is triggered)                             |
|  131086  |                                   Withdrawal amount exceeds risk limit (the risk limit of margin trade is triggered)                                   |
|  131087  |                        your current account spot risk level is too high, withdrawal is prohibited, please adjust and try again                         |
|  131088  |   The withdrawal amount exceeds the remaining withdrawal limit of your identity verification level. The current available amount for withdrawal : %s   |
|  131089  |                                           User sensitive operation, withdrawal is prohibited within 24 hours                                           |
|  131090  |                                                             User withdraw has been banned                                                              |
|  131091  |                                                    Blocked login status does not allow withdrawals                                                     |
|  131092  |                                                                User status is abnormal                                                                 |
|  131093  |                                                     The withdrawal address is not in the whitelist                                                     |
|  131094  |                                                             UserId is not in the whitelist                                                             |
|  131095  |                                                  Withdrawl amount exceeds the 24 hour platform limit                                                   |
|  131096  |                                             Withdraw amount does not satify the lower limit or upper limit                                             |
|  131097  |                                                      Withdrawal of this currency has been closed                                                       |
|  131098  |                                                 Withdrawal currently is not availble from new address                                                  |
|  131099  |                                                       Hot wallet status can cancel the withdraw                                                        |
|  131200  |                                                                     Service error                                                                      |
|  131201  |                                                                     Internal error                                                                     |
|  131202  |                                                                    Invalid memberId                                                                    |
|  131203  |                                                                Request parameter error                                                                 |
|  131204  |                                                                   Account info error                                                                   |
|  131205  |                                                                  Query transfer error                                                                  |
|  131206  |                                                                   cannot be transfer                                                                   |
|  131207  |                                                                   Account not exist                                                                    |
|  131208  |                                                                    Forbid transfer                                                                     |
|  131209  |                                                              Get subMember relation error                                                              |
|  131210  |                                                                 Amount accuracy error                                                                  |
|  131211  |                                                   fromAccountType can't be the same as toAccountType                                                   |
|  131212  |                                                                  Insufficient balance                                                                  |
|  131213  |                                                                TransferLTV check error                                                                 |
|  131214  |                                                                    TransferId exist                                                                    |
|  131215  |                                                                      Amount error                                                                      |
|  131216  |                                                                  Query balance error                                                                   |
|  131217  |                                                                    Risk check error                                                                    |
|  131227  |                                                 Sub-account do not have universal transfer permission                                                  |
|  131228  |                                             your balance is not enough. Please check transfer safe amount                                              |
|  131229  |                                    Due to compliance requirements, the current currency is not allowed to transfer                                     |
|  131230  |                                                       The system is busy, please try again later                                                       |
|  131231  |                                                     Transfers into this account are not supported                                                      |
|  131232  |                                                      Transfers out this account are not supported                                                      |
|  131233  |                                            can not transfer the coin that not supported for islamic account                                            |
|  140001  |                                           Switching the PM spot hedging switch is not allowed in non PM mode                                           |
|~~140002~~|                                             ~~Institutional lending users do not support PM spot hedging~~                                             |
|  140003  |                                             You have position(s) being liquidated, please try again later.                                             |
|  140004  |Operations Restriction: The current LTV ratio of your Institutional Loan has hit the liquidation threshold. Assets in your account are being liquidated.|
|  140005  |                                                   Risk level after switching modes exceeds threshold                                                   |
|  141004  |                                                                sub member is not normal                                                                |
|  141025  |                                                   This sub-account has assets and cannot be deleted                                                    |
|  181000  |                                                                    category is null                                                                    |
|  181001  |                                                    category only support linear or option or spot.                                                     |
|  181002  |                                                                    symbol is null.                                                                     |
|  181003  |                                                                     side is null.                                                                      |
|  181004  |                                                             side only support Buy or Sell.                                                             |
|  181005  |                                                                  orderStatus is wrong                                                                  |
|  181006  |                                                                startTime is not number                                                                 |
|  181007  |                                                                 endTime is not number                                                                  |
|  181008  |                                                    Parameter startTime and endTime are both needed                                                     |
|  181009  |                                                  Parameter startTime needs to be smaller than endTime                                                  |
|  181010  |                                           The time range between startTime and endTime cannot exceed 7 days                                            |
|  181011  |                                                                 limit is not a number                                                                  |
|  181012  |                                                                    symbol not exist                                                                    |
|  181013  |                                                             Only support settleCoin: usdc                                                              |
|  181014  |                                                            Classic account is not supported                                                            |
|  181018  |                                                                    Invalid expDate.                                                                    |
|  181019  |                                                    Parameter expDate can't be earlier than 2 years                                                     |
|  182000  |                                                           symbol related quote price is null                                                           |
|  182200  |                                                               Please upgrade UTA first.                                                                |
|  182201  |                                                           You must enter 2 time parameters.                                                            |
|  182202  |                                                     The start time must be less than the end time                                                      |
|  182203  |                                                             Please enter valid characters                                                              |
|  182204  |                                                                  Coin does not exist                                                                   |
|  182205  |                                                               User level does not exist                                                                |
|  700000  |                                                          accountType/quoteTxId cannot be null                                                          |
|  700001  |                                                             quote fail:no dealer can used                                                              |
|  700004  |                                                                  order does not exist                                                                  |
|  700007  |                                                                   Large Amount Limit                                                                   |
|  700012  |                                                     UTA upgrading, don't allow to apply for quote                                                      |

Crypto Loan[​](#crypto-loan)
----------

| Code |                                                Description                                                |
|:----:|:----------------------------------------------------------------------------------------------------------|
|177002|                                 Server is busy, please wait and try again                                 |
|177003|                                  Illegal characters found in a parameter                                  |
|177004|                           Precision is over the maximum defined for this asset                            |
|177005|                                           Order does not exist                                            |
|177006|                                         We don't have this asset                                          |
|177007|                            Your borrow amount has exceed maximum borrow amount                            |
|177008|                                      Borrow is banned for this asset                                      |
|177009|                             Borrow amount is less than minimum borrow amount                              |
|177010|                                    Repay amount exceeds borrow amount                                     |
|177011|                                           Balance is not enough                                           |
|177012|                                 The system doesn't have enough asset now                                  |
|177013|                            adjustment amount exceeds minimum collateral amount                            |
|177014|                                       Individual loan quota reached                                       |
|177015|Collateral amount has reached the limit. Please reduce your collateral amount or try with other collaterals|
|177016|                                  Minimum collateral amount is not enough                                  |
|177017|                                  This coin cannot be used as collateral                                   |
|177018|                                             duplicate request                                             |
|177019|                                        Your input param is invalid                                        |
|177020|                                  The account does not support the asset                                   |
|177021|                                             Repayment failed                                              |

Institutional Loan[​](#institutional-loan)
----------

| Code  |                               Description                                |
|:-----:|:-------------------------------------------------------------------------|
|3777002|                     UID cannot be bound repeatedly.                      |
|3777003| UID cannot be unbound because the UID has not been bound to a risk unit. |
|3777004|             The main UID of the risk unit cannot be unbound.             |
|3777005| You have unsettled lending or borrowing orders. Please try again later.  |
|3777006|       UID cannot be bound, please try again with a different UID."       |
|3777007|             UID cannot be bound, please upgrade to UTA Pro."             |
|3777012|Your request is currently being processed. Please wait and try again later|
|3777027|          UID cannot be bound, leveraged trading closure failed.          |
|3777029| You currently have orders for pre-market trading that can’t be bind UIDs |
|3777030|          This account has activated copyPro and cannot bind uid          |

Exchange Broker[​](#exchange-broker)
----------

|   Code    |                         Description                          |
|:---------:|:-------------------------------------------------------------|
|  3500402  |          Parameter verification failed for 'limit'.          |
|  3500403  |        Only available to exchange broker main-account        |
|  3500404  |                        Invalid Cursor                        |
|~~3500405~~|Parameter "startTime" and "endTime" need to be input in pairs.|
|  3500406  |                   Out of query time range.                   |
|  3500407  |      Parameter begin and end need to be input in pairs.      |

### Reward[​](#reward) ###

| Code |                                   Description                                    |
|:----:|:---------------------------------------------------------------------------------|
|400001|                                invalid parameter                                 |
|400101|                             The voucher was recycled                             |
|400102|              The voucher has exceeded the redemption date (expired)              |
|400103|                   The voucher is not available for redemption                    |
|400105|                                 Budget exceeded                                  |
|403001|Account rejected, check if the input accountId valid, account banned, or kyc issue|
|404001|                                resource not found                                |
|404011|                              Insufficient inventory                              |
|409011|                                 VIP level limit                                  |
|500001|                              Internal server error                               |

Earn[​](#earn)
----------

| Code |                         Description                          |
|:----:|:-------------------------------------------------------------|
|180001|                      Invalid parameter                       |
|180002|                         Invalid coin                         |
|180003|                         User banned                          |
|180004|Site not allowed. Only users from Bybit global site can access|
|180005|                 Compliance wallet not reach                  |
|180006|                      Validation failed                       |
|180007|                    Product not available                     |
|180008|                       Invalid Product                        |
|180009|                     product is forbidden                     |
|180010|                       User not allowed                       |
|180011|                         User not VIP                         |
|180012|                  Purchase share is invalid                   |
|180013|                   Stake over maximum share                   |
|180014|                     Redeem share invlaid                     |
|180015|                  Products share not enough                   |
|180016|                      Balance not enough                      |
|180017|                      Invalid risk user                       |
|180018|                        internal error                        |
|180019|                     empty order link id                      |

## CONNECT

Connect
==========

**[WebSocket public stream](/docs/v5/websocket/public/orderbook):**

* **Mainnet:**  
  Spot: `wss://stream.bybit.com/v5/public/spot`  
  USDT, USDC perpetual & USDC Futures: `wss://stream.bybit.com/v5/public/linear`  
  Inverse contract: `wss://stream.bybit.com/v5/public/inverse`  
  USDC Option: `wss://stream.bybit.com/v5/public/option`

* **Testnet:**  
  Spot: `wss://stream-testnet.bybit.com/v5/public/spot`  
  USDT and USDC perpetual: `wss://stream-testnet.bybit.com/v5/public/linear`  
  Inverse contract: `wss://stream-testnet.bybit.com/v5/public/inverse`  
  USDC Option: `wss://stream-testnet.bybit.com/v5/public/option`

**[WebSocket private stream](/docs/v5/websocket/private/order):**

* **Mainnet:**  
  `wss://stream.bybit.com/v5/private`

* **Testnet:**  
  `wss://stream-testnet.bybit.com/v5/private`

**[WebSocket Order Entry](/docs/v5/websocket/trade/guideline):**

* **Mainnet:**  
  `wss://stream.bybit.com/v5/trade`

* **Testnet:**  
  `wss://stream-testnet.bybit.com/v5/trade`

info

* If your account is registered from [www.bybit-tr.com](http://www.bybit-tr.com), please use `stream.bybit-tr.com` for mainnet access
* If your account is registered from [www.bybit.kz](http://www.bybit.kz), please use `stream.bybit.kz` for mainnet access

Customise Private Connection Alive Time

For private stream and order entry, you can customise alive duration by adding a param `max_active_time`, the lowest value is `30s` (30 seconds), the highest value is `600s` (10 minutes). You can also pass `1m`, `2m` etc when you try to configure by minute level. e.g., *wss://stream-testnet.bybit.com/v5/private?max\_active\_time=1m*.

In general, if there is no "ping-pong" and no stream data sent from server end, the connection will be cut off after 10 minutes. When you have a particular need, you can configure connection alive time by `max_active_time`.

Since ticker scans every 30s, so it is not fully exact, i.e., if you configure 45s, and your last update or ping-pong is occurred on `2023-08-15 17:27:23`, your disconnection time maybe happened on `2023-08-15 17:28:15`

Authentication[​](#authentication)
----------

info

**Public** topics do not require authentication. The following section applies to **private** topics only.

Apply for authentication when establishing a connection.

Note: if you're using [pybit](https://github.com/bybit-exchange/pybit), [bybit-api](https://www.npmjs.com/package/bybit-api), or another high-level library, you can ignore this code - as authentication is handled for you.

```
{    "req_id": "10001", // optional    "op": "auth",    "args": [        "api_key",        1662350400000, // expires; is greater than your current timestamp        "signature"    ]}
```

```
# based on: https://github.com/bybit-exchange/pybit/blob/master/pybit/_http_manager.pyimport hmacimport jsonimport timeimport websocketapi_key = ""api_secret = ""# Generate expires.expires = int((time.time() + 1) * 1000)# Generate signature.signature = str(hmac.new(    bytes(api_secret, "utf-8"),    bytes(f"GET/realtime{expires}", "utf-8"), digestmod="sha256").hexdigest())ws = websocket.WebSocketApp(    url=url,    ...)# Authenticate with API.ws.send(    json.dumps({        "op": "auth",        "args": [api_key, expires, signature]    }))
```

>
>
> Successful authentication sample response
>
>

```
{    "success": true,    "ret_msg": "",    "op": "auth",    "conn_id": "cejreaspqfh3sjdnldmg-p"}
```

note

Example signature algorithms can be found [here](https://github.com/bybit-exchange/api-usage-examples).

caution

Due to network complexity, your may get disconnected at any time. Please follow the instructions below to ensure that you receive WebSocket messages on time:

1. Keep connection alive by [sending the heartbeat packet](/docs/v5/ws/connect#how-to-send-the-heartbeat-packet)
2. Reconnect as soon as possible if disconnected

IP Limits[​](#ip-limits)
----------

* Do not frequently connect and disconnect the connection.
* Do not build over 500 connections in 5 minutes. This is counted per WebSocket domain.

Public channel - Args limits[​](#public-channel---args-limits)
----------

Regardless of Perpetual, Futures, Options or Spot, for one public connection, you cannot have length of "args" array over 21,000 characters.

* Spot can input up to 10 args for each subscription request sent to one connection
* Options can input up to 2000 args for a single connection
* No args limit for Futures for now

How to Send the Heartbeat Packet[​](#how-to-send-the-heartbeat-packet)
----------

>
>
> How to Send
>
>

```
// req_id is a customised ID, which is optionalws.send(JSON.stringify({"req_id": "100001", "op": "ping"}));
```

>
>
> Pong message example of public channels
>
>

* Spot
* Linear/Inverse
* Option

```
{    "success": true,    "ret_msg": "pong",    "conn_id": "0970e817-426e-429a-a679-ff7f55e0b16a",    "op": "ping"}
```

```
{    "success": true,    "ret_msg": "pong",    "conn_id": "465772b1-7630-4fdc-a492-e003e6f0f260",    "req_id": "",    "op": "ping"}
```

```
{    "args": [        "1672916271846"    ],    "op": "pong"}
```

>
>
> Pong message example of private channels
>
>

```
{    "req_id": "test",    "op": "pong",    "args": [        "1675418560633"    ],    "conn_id": "cfcb4ocsvfriu23r3er0-1b"}
```

caution

To avoid network or program issues, we recommend that you send the `ping` heartbeat packet every **20** seconds to maintain the WebSocket connection.

How to Subscribe to Topics[​](#how-to-subscribe-to-topics)
----------

### Understanding WebSocket Filters[​](#understanding-websocket-filters) ###

How to subscribe with a filter

```
// Subscribing level 1 orderbook{    "req_id": "test", // optional    "op": "subscribe",    "args": [        "orderbook.1.BTCUSDT"    ]}
```

Subscribing with multiple symbols and topics is supported.

```
{    "req_id": "test", // optional    "op": "subscribe",    "args": [        "orderbook.1.BTCUSDT",        "publicTrade.BTCUSDT",        "orderbook.1.ETHUSDT"    ]}
```

### Understanding WebSocket Filters: Unsubscription[​](#understanding-websocket-filters-unsubscription) ###

You can dynamically subscribe and unsubscribe from topics without unsubscribing from the WebSocket like so:

```
{    "op": "unsubscribe",    "args": [        "publicTrade.ETHUSD"    ],    "req_id": "customised_id"}
```

Understanding the Subscription Response[​](#understanding-the-subscription-response)
----------

>
>
> Topic subscription response message example
>
>

* Private
* Public Spot
* Linear/Inverse
* Option

```
{    "success": true,    "ret_msg": "",    "op": "subscribe",    "conn_id": "cejreassvfrsfvb9v1a0-2m"}
```

```
{    "success": true,    "ret_msg": "subscribe",    "conn_id": "2324d924-aa4d-45b0-a858-7b8be29ab52b",    "req_id": "10001",    "op": "subscribe"}
```

```
{    "success": true,    "ret_msg": "",    "conn_id": "3cd84cb1-4d06-4a05-930a-2efe5fc70f0f",    "req_id": "",    "op": "subscribe"}
```

```
{    "success": true,    "conn_id": "aa01fbfffe80af37-00000001-000b37b9-7147f432704fd28c-00e1c172",    "data": {    "failTopics": [],    "successTopics": [        "orderbook.100.BTC-6JAN23-18000-C"    ]},    "type": "COMMAND_RESP"}
```

## POSITION

Position
==========

Subscribe to the position stream to see changes to your position data in **real-time**.

**All-In-One Topic:** `position`  
**Categorised Topic:** `position.linear`, `position.inverse`, `position.option`

info

* All-In-One topic and Categorised topic **cannot** be in the same subscription request
* All-In-One topic: Allow you to listen to all categories (linear, inverse, option) websocket updates
* Categorised Topic: Allow you to listen only to specific category websocket updates

tip

Every time when you create/amend/cancel an order, the position topic will generate a new message (regardless if there's any actual change)

### Response Parameters[​](#response-parameters) ###

|                      Parameter                      | Type  |                                                                                                                                                                                                                                                                                                                                                Comments                                                                                                                                                                                                                                                                                                                                                 |
|:----------------------------------------------------|:------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                         id                          |string |                                                                                                                                                                                                                                                                                                                                               Message ID                                                                                                                                                                                                                                                                                                                                                |
|                        topic                        |string |                                                                                                                                                                                                                                                                                                                                               Topic name                                                                                                                                                                                                                                                                                                                                                |
|                    creationTime                     |number |                                                                                                                                                                                                                                                                                                                                       Data created timestamp (ms)                                                                                                                                                                                                                                                                                                                                       |
|                        data                         | array |                                                                                                                                                                                                                                                                                                                                                 Object                                                                                                                                                                                                                                                                                                                                                  |
|        \> [category](/docs/v5/enum#category)        |string |                                                                                                                                                                                                                                                                                                                                              Product type                                                                                                                                                                                                                                                                                                                                               |
|                      \> symbol                      |string |                                                                                                                                                                                                                                                                                                                                               Symbol name                                                                                                                                                                                                                                                                                                                                               |
|                       \> side                       |string |                                                                                                                                                                         Position side. `Buy`: long, `Sell`: short<br/><br/>* one-way mode: classic & [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse), an empty position returns `None`.<br/>* [UTA2.0](/docs/v5/acct-mode#uta-20)(linear, inverse) & [UTA1.0](/docs/v5/acct-mode#uta-10)(linear): either one-way or hedge mode returns an empty string `""` for an empty position.                                                                                                                                                                         |
|                       \> size                       |string |                                                                                                                                                                                                                                                                                                                                              Position size                                                                                                                                                                                                                                                                                                                                              |
|     \> [positionIdx](/docs/v5/enum#positionidx)     |integer|                                                                                                                                                                                                                                                                                                                         Used to identify positions in different position modes                                                                                                                                                                                                                                                                                                                          |
|                    \> tradeMode                     |integer|                                                                                                                                                                                      Trade mode<br/><br/>* Classic & [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse): `0`: cross-margin, `1`: isolated margin<br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10)(execpt inverse): deprecated, always `0`, check [Get Account Info](/docs/v5/account/account-info) to know the margin mode                                                                                                                                                                                      |
|                  \> positionValue                   |string |                                                                                                                                                                                                                                                                                                                                             Position value                                                                                                                                                                                                                                                                                                                                              |
|                      \> riskId                      |integer|                                                                                                                                                                                                                                                                                             Risk tier ID  <br/>*for portfolio margin mode, this field returns 0, which means risk limit rules are invalid*                                                                                                                                                                                                                                                                                              |
|                  \> riskLimitValue                  |string |                                                                                                                                                                                                                                                                                           Risk limit value  <br/>*for portfolio margin mode, this field returns 0, which means risk limit rules are invalid*                                                                                                                                                                                                                                                                                            |
|                    \> entryPrice                    |string |                                                                                                                                                                                                                                                                                                                                               Entry price                                                                                                                                                                                                                                                                                                                                               |
|                    \> markPrice                     |string |                                                                                                                                                                                                                                                                                                                                               Mark price                                                                                                                                                                                                                                                                                                                                                |
|                     \> leverage                     |string |                                                                                                                                                                                                                                                                                           Position leverage  <br/>*for portfolio margin mode, this field returns "", which means leverage rules are invalid*                                                                                                                                                                                                                                                                                            |
|                 \> positionBalance                  |string |                                                                                                                                                                                                                                                                                    Position margin Classic & [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse) can refer to this field to get the position initial margin                                                                                                                                                                                                                                                                                    |
|                  \> autoAddMargin                   |integer|                                                                                                                                                                                                                                                                                      Whether to add margin automatically. `0`: false, `1`: true. For UTA, it is meaningful only when UTA enables `ISOLATED_MARGIN`                                                                                                                                                                                                                                                                                      |
|                    \> positionIM                    |string |                                                                                                                                                                                                                                                                                     Initial marginClassic & [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse): ignore this fieldUTA portfolio margin mode, it returns ""                                                                                                                                                                                                                                                                                     |
|                    \> positionMM                    |string |                                                                                                                                                                                                                                                                                   Maintenance marginClassic & [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse): ignore this fieldUTA portfolio margin mode, it returns ""                                                                                                                                                                                                                                                                                   |
|                     \> liqPrice                     |string |                                                   Position liquidation price<br/><br/>* [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse) & UTA(isolated margin enabled) & Classic account: it is the real price for isolated and cross positions, and keeps `""` when liqPrice \<= minPrice or liqPrice \>= maxPrice<br/>* UTA (Cross margin mode): it is an estimated price for cross positions(because the unified mode controls the risk rate according to the account), and keeps `""` when liqPrice \<= minPrice or liqPrice \>= maxPrice<br/><br/>However, this field is empty for Portfolio Margin Mode, and no liquidation price will be provided                                                   |
|                    \> bustPrice                     |string |                                                                                                                                                                                                                                                                                 Bankruptcy price  <br/>*Unified mode returns `""`, no position bankruptcy price (except [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse))*                                                                                                                                                                                                                                                                                  |
|                     \> tpslMode                     |string |                                                                                                                                                                                                                                                                                                                               deprecated, meaningless here, always "Full"                                                                                                                                                                                                                                                                                                                               |
|                    \> takeProfit                    |string |                                                                                                                                                                                                                                                                                                                                            Take profit price                                                                                                                                                                                                                                                                                                                                            |
|                     \> stopLoss                     |string |                                                                                                                                                                                                                                                                                                                                             Stop loss price                                                                                                                                                                                                                                                                                                                                             |
|                   \> trailingStop                   |string |                                                                                                                                                                                                                                                                                                                                              Trailing stop                                                                                                                                                                                                                                                                                                                                              |
|                  \> unrealisedPnl                   |string |                                                                                                                                                                                                                                                                                                                                       Unrealised profit and loss                                                                                                                                                                                                                                                                                                                                        |
|                  \> curRealisedPnl                  |string |                                                                                                                                                                                                                                                                                                                            The realised PnL for the current holding position                                                                                                                                                                                                                                                                                                                            |
|                 \> sessionAvgPrice                  |string |                                                                                                                                                                                                                                                                                                      USDC contract session avg price, it is the same figure as avg entry price shown in the web UI                                                                                                                                                                                                                                                                                                      |
|                      \> delta                       |string |                                                                                                                                                                                                                                                                                                                                                  Delta                                                                                                                                                                                                                                                                                                                                                  |
|                      \> gamma                       |string |                                                                                                                                                                                                                                                                                                                                                  Gamma                                                                                                                                                                                                                                                                                                                                                  |
|                       \> vega                       |string |                                                                                                                                                                                                                                                                                                                                                  Vega                                                                                                                                                                                                                                                                                                                                                   |
|                      \> theta                       |string |                                                                                                                                                                                                                                                                                                                                                  Theta                                                                                                                                                                                                                                                                                                                                                  |
|                  \> cumRealisedPnl                  |string |                                                                                                                                                                                                                                                                     Cumulative realised pnl<br/><br/>* Futures & Perp: it is the all time cumulative realised P&L<br/>* Option: it is the realised P&L when you hold that position                                                                                                                                                                                                                                                                      |
|  \> [positionStatus](/docs/v5/enum#positionstatus)  |string |                                                                                                                                                                                                                                                                                                                                 Position status. `Normal`, `Liq`, `Adl`                                                                                                                                                                                                                                                                                                                                 |
|\> [adlRankIndicator](/docs/v5/enum#adlrankindicator)|integer|                                                                                                                                                                                                                                                                              Auto-deleverage rank indicator. [What is Auto-Deleveraging?](https://www.bybit.com/en-US/help-center/s/article/What-is-Auto-Deleveraging-ADL)                                                                                                                                                                                                                                                                              |
|                   \> isReduceOnly                   |boolean|Useful when Bybit lower the risk limit<br/><br/>* `true`: Only allowed to reduce the position. You can consider a series of measures, e.g., lower the risk limit, decrease leverage or reduce the position, add margin, or cancel orders, after these operations, you can call [confirm new risk limit](/docs/v5/position/confirm-mmr) endpoint to check if your position can be removed the reduceOnly mark<br/>* `false`: There is no restriction, and it means your position is under the risk when the risk limit is systematically adjusted<br/>* Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others|
|                \> mmrSysUpdatedTime                 |string |                              Useful when Bybit lower the risk limit<br/><br/>* When isReduceOnly=`true`: the timestamp (ms) when the MMR will be forcibly adjusted by the systemWhen isReduceOnly=`false`: the timestamp when the MMR had been adjusted by system<br/>* It returns the timestamp when the system operates, and if you manually operate, there is no timestamp<br/>* Keeps `""` by default, if there was a lower risk limit system adjustment previously, it shows that system operation timestamp<br/>* Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others                              |
|              \> leverageSysUpdatedTime              |string |                         Useful when Bybit lower the risk limit<br/><br/>* When isReduceOnly=`true`: the timestamp (ms) when the leverage will be forcibly adjusted by the systemWhen isReduceOnly=`false`: the timestamp when the leverage had been adjusted by system<br/>* It returns the timestamp when the system operates, and if you manually operate, there is no timestamp<br/>* Keeps `""` by default, if there was a lower risk limit system adjustment previously, it shows that system operation timestamp<br/>* Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others                         |
|                   \> createdTime                    |string |                                                                                                                                                                                                                                                                                                                 Timestamp of the first time a position was created on this symbol (ms)                                                                                                                                                                                                                                                                                                                  |
|                   \> updatedTime                    |string |                                                                                                                                                                                                                                                                                                                                  Position data updated timestamp (ms)                                                                                                                                                                                                                                                                                                                                   |
|                       \> seq                        | long  |                                                                                                                                                                                    Cross sequence, used to associate each fill and each position update<br/><br/>* Different symbols may have the same seq, please use seq + symbol to check unique<br/>* Returns `"-1"` if the symbol has never been traded<br/>* Returns the seq updated by the last transaction when there are setting like leverage, risk limit                                                                                                                                                                                     |

### Subscribe Example[​](#subscribe-example) ###

```
{    "op": "subscribe",    "args": [        "position"    ]}
```

```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="private",    api_key="XXXXX",    api_secret="XXXXX",)def handle_message(message):    print(message)ws.position_stream(callback=handle_message)while True:    sleep(1)
```

### Stream Example[​](#stream-example) ###

```
{    "id": "1003076014fb7eedb-c7e6-45d6-a8c1-270f0169171a",    "topic": "position",    "creationTime": 1697682317044,    "data": [        {            "positionIdx": 2,            "tradeMode": 0,            "riskId": 1,            "riskLimitValue": "2000000",            "symbol": "BTCUSDT",            "side": "",            "size": "0",            "entryPrice": "0",            "leverage": "10",            "positionValue": "0",            "positionBalance": "0",            "markPrice": "28184.5",            "positionIM": "0",            "positionMM": "0",            "takeProfit": "0",            "stopLoss": "0",            "trailingStop": "0",            "unrealisedPnl": "0",            "curRealisedPnl": "1.26",            "cumRealisedPnl": "-25.06579337",            "sessionAvgPrice": "0",            "createdTime": "1694402496913",            "updatedTime": "1697682317038",            "tpslMode": "Full",            "liqPrice": "0",            "bustPrice": "",            "category": "linear",            "positionStatus": "Normal",            "adlRankIndicator": 0,            "autoAddMargin": 0,            "leverageSysUpdatedTime": "",            "mmrSysUpdatedTime": "",            "seq": 8327597863,            "isReduceOnly": false        }    ]}
```

## EXECUTION

Execution
==========

Subscribe to the execution stream to see your executions in **real-time**.

tip

You may have multiple executions for one order in a single message.

**All-In-One Topic:** `execution`  
**Categorised Topic:** `execution.spot`, `execution.linear`, `execution.inverse`, `execution.option`

info

* All-In-One topic and Categorised topic **cannot** be in the same subscription request
* All-In-One topic: Allow you to listen to all categories (spot, linear, inverse, option) websocket updates
* Categorised Topic: Allow you to listen only to specific category websocket updates

### Response Parameters[​](#response-parameters) ###

|                   Parameter                   | Type  |                                                                                                                      Comments                                                                                                                      |
|:----------------------------------------------|:------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                      id                       |string |                                                                                                                     Message ID                                                                                                                     |
|                     topic                     |string |                                                                                                                     Topic name                                                                                                                     |
|                 creationTime                  |number |                                                                                                            Data created timestamp (ms)                                                                                                             |
|                     data                      | array |                                                                                                                       Object                                                                                                                       |
|     \> [category](/docs/v5/enum#category)     |string |                            Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `spot`, `linear`, `inverse`, `option`<br/>* Classic account: `spot`, `linear`, `inverse`.                             |
|                   \> symbol                   |string |                                                                                                                    Symbol name                                                                                                                     |
|                 \> isLeverage                 |string |                                                               Whether to borrow. **Unified `spot`** only. `0`: false, `1`: true  <br/>*Classic `spot` is not supported, always `0`*                                                                |
|                  \> orderId                   |string |                                                                                                                      Order ID                                                                                                                      |
|                \> orderLinkId                 |string |                                                                                                              User customized order ID                                                                                                              |
|                    \> side                    |string |                                                                                                                 Side. `Buy`,`Sell`                                                                                                                 |
|                 \> orderPrice                 |string |                                                                                                   Order price. *Classic `spot` is not supported*                                                                                                   |
|                  \> orderQty                  |string |                                                                                                    Order qty. *Classic `spot` is not supported*                                                                                                    |
|                 \> leavesQty                  |string |                                                                                         The remaining qty not executed. *Classic `spot` is not supported*                                                                                          |
|   \> [createType](/docs/v5/enum#createtype)   |string |                                                      Order create type Classic account & [UTA1.0](/docs/v5/acct-mode#uta-10)(category=inverse): always `""`Spot, Option do not have this key                                                       |
|    \> [orderType](/docs/v5/enum#ordertype)    |string |                                                                                          Order type. `Market`,`Limit`. *Classic `spot` is not supported*                                                                                           |
|\> [stopOrderType](/docs/v5/enum#stopordertype)|string |                                                                    Stop order type. If the order is not stop order, any type is not returned. *Classic `spot` is not supported*                                                                    |
|                  \> execFee                   |string |                                            Executed trading fee. You can get spot fee currency instruction [here](/docs/v5/enum#spot-fee-currency-instruction)  <br/>*Classic `spot` is not supported*                                             |
|                   \> execId                   |string |                                                                                                                    Execution ID                                                                                                                    |
|                 \> execPrice                  |string |                                                                                                                  Execution price                                                                                                                   |
|                  \> execQty                   |string |                                                                                                                   Execution qty                                                                                                                    |
|                  \> execPnl                   |string |                                       Profit and Loss for each close position execution. The value keeps consistent with the field "cashFlow" in the [Get Transaction Log](/docs/v5/account/transaction-log)                                       |
|     \> [execType](/docs/v5/enum#exectype)     |string |                                                                                                  Executed type. *Classic `spot` is not supported*                                                                                                  |
|                 \> execValue                  |string |                                                                                              Executed order value. *Classic `spot` is not supported*                                                                                               |
|                  \> execTime                  |string |                                                                                                               Executed timestamp（ms）                                                                                                               |
|                  \> isMaker                   |boolean|                                                                                                   Is maker order. `true`: maker, `false`: taker                                                                                                    |
|                  \> feeRate                   |string |                                                                                                Trading fee rate. *Classic `spot` is not supported*                                                                                                 |
|                  \> tradeIv                   |string |                                                                                                       Implied volatility. valid for `option`                                                                                                       |
|                   \> markIv                   |string |                                                                                                Implied volatility of mark price. valid for `option`                                                                                                |
|                 \> markPrice                  |string |                                                                                          The mark price of the symbol when executing. valid for `option`                                                                                           |
|                 \> indexPrice                 |string |                                                                                          The index price of the symbol when executing. valid for `option`                                                                                          |
|              \> underlyingPrice               |string |                                                                                       The underlying price of the symbol when executing. valid for `option`                                                                                        |
|                \> blockTradeId                |string |                                                                                                              Paradigm block trade ID                                                                                                               |
|                 \> closedSize                 |string |                                                                                                                Closed position size                                                                                                                |
|                    \> seq                     | long  |Cross sequence, used to associate each fill and each position update<br/><br/>* The seq will be the same when conclude multiple transactions at the same time<br/>* Different symbols may have the same seq, please use seq + symbol to check unique|

### Subscribe Example[​](#subscribe-example) ###

```
{    "op": "subscribe",    "args": [        "execution"    ]}
```

```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="private",    api_key="XXXXX",    api_secret="XXXXX",)def handle_message(message):    print(message)ws.execution_stream(callback=handle_message)while True:    sleep(1)
```

### Stream Example[​](#stream-example) ###

```
{    "id": "592324803b2785-26fa-4214-9963-bdd4727f07be",    "topic": "execution",    "creationTime": 1672364174455,    "data": [        {            "category": "linear",            "symbol": "XRPUSDT",            "execFee": "0.005061",            "execId": "7e2ae69c-4edf-5800-a352-893d52b446aa",            "execPrice": "0.3374",            "execQty": "25",            "execType": "Trade",            "execValue": "8.435",            "execValue": "0",            "isMaker": false,            "feeRate": "0.0006",            "tradeIv": "",            "markIv": "",            "blockTradeId": "",            "markPrice": "0.3391",            "indexPrice": "",            "underlyingPrice": "",            "leavesQty": "0",            "orderId": "f6e324ff-99c2-4e89-9739-3086e47f9381",            "orderLinkId": "",            "orderPrice": "0.3207",            "orderQty": "25",            "orderType": "Market",            "stopOrderType": "UNKNOWN",            "side": "Sell",            "execTime": "1672364174443",            "isLeverage": "0",            "closedSize": "",            "seq": 4688002127        }    ]}
```

## FAST EXECUTION

Fast Execution
==========

Fast execution stream significantly reduces data latency compared original "execution" stream. However, it pushes limited
execution type of trades, and fewer data fields.

**All-In-One Topic:** `execution.fast`  
**Categorised Topic:** `execution.fast.linear`, `execution.fast.inverse`, `execution.fast.spot`  

info

* Supports all Perps, Futures and Spot exceution, and do not support Options for now
* You can only receive [execType](/docs/v5/enum#exectype)=Trade update

### Response Parameters[​](#response-parameters) ###

|              Parameter              | Type  |                                                                                                                      Comments                                                                                                                      |
|:------------------------------------|:------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                topic                |string |                                                                                                                     Topic name                                                                                                                     |
|            creationTime             |number |                                                                                                            Data created timestamp (ms)                                                                                                             |
|                data                 | array |                                                                                                                       Object                                                                                                                       |
|\> [category](/docs/v5/enum#category)|string |                                 Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `inverse`, `spot`<br/>* Classic account: `linear`, `inverse`, `spot`.                                  |
|              \> symbol              |string |                                                                                                                    Symbol name                                                                                                                     |
|             \> orderId              |string |                                                                                                                      Order ID                                                                                                                      |
|             \> isMaker              |boolean|                                                                                                           `true`: Maker, `false`: Taker                                                                                                            |
|           \> orderLinkId            |string |                                                User customized order ID maker trade is always `""`If a maker order in the orderbook is converted to taker (by price amend), orderLinkId is also ""                                                 |
|              \> execId              |string |                                                                                                                    Execution ID                                                                                                                    |
|            \> execPrice             |string |                                                                                                                  Execution price                                                                                                                   |
|             \> execQty              |string |                                                                                                                   Execution qty                                                                                                                    |
|               \> side               |string |                                                                                                                 Side. `Buy`,`Sell`                                                                                                                 |
|             \> execTime             |string |                                                                                                               Executed timestamp（ms）                                                                                                               |
|               \> seq                | long  |Cross sequence, used to associate each fill and each position update<br/><br/>* The seq will be the same when conclude multiple transactions at the same time<br/>* Different symbols may have the same seq, please use seq + symbol to check unique|

### Subscribe Example[​](#subscribe-example) ###

```
{    "op": "subscribe",    "args": [        "execution.fast"    ]}
```

### Stream Example[​](#stream-example) ###

```
{    "topic": "execution.fast",    "creationTime": 1716800399338,    "data": [        {            "category": "linear",            "symbol": "ICPUSDT",            "execId": "3510f361-0add-5c7b-a2e7-9679810944fc",            "execPrice": "12.015",            "execQty": "3000",            "orderId": "443d63fa-b4c3-4297-b7b1-23bca88b04dc",            "isMaker": false,            "orderLinkId": "test-00001",            "side": "Sell",            "execTime": "1716800399334",            "seq": 34771365464        }    ]}
```

## ORDER

Order
==========

Subscribe to the order stream to see changes to your orders in **real-time**.

**All-In-One Topic:** `order`  
**Categorised Topic:** `order.spot`, `order.linear`, `order.inverse`, `order.option`

info

* All-In-One topic and Categorised topic **cannot** be in the same subscription request
* All-In-One topic: Allow you to listen to all categories (spot, linear, inverse, option) websocket updates
* Categorised Topic: Allow you to listen only to specific category websocket updates

tip

You may receive two orderStatus=`Filled` messages when the cancel request is accepted but the order is executed at the same time. Generally, one
message contains "orderStatus=Filled, rejectReason=EC\_NoError", and another message contains "orderStatus=Filled, cancelType=CancelByUser, rejectReason=EC\_OrigClOrdIDDoesNotExist".
The first message tells you the order is executed, and the second message tells you the followed cancel request is rejected due to order is executed.

### Response Parameters[​](#response-parameters) ###

|                   Parameter                   | Type  |                                                                                                                    Comments                                                                                                                     |
|:----------------------------------------------|:------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                      id                       |string |                                                                                                                   Message ID                                                                                                                    |
|                     topic                     |string |                                                                                                                   Topic name                                                                                                                    |
|                 creationTime                  |number |                                                                                                           Data created timestamp (ms)                                                                                                           |
|                     data                      | array |                                                                                                                     Object                                                                                                                      |
|                  \> category                  |string |                           Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `spot`, `linear`, `inverse`, `option`<br/>* Classic account: `spot`, `linear`, `inverse`.                           |
|                  \> orderId                   |string |                                                                                                                    Order ID                                                                                                                     |
|                \> orderLinkId                 |string |                                                                                                            User customised order ID                                                                                                             |
|                 \> isLeverage                 |string |                                                              Whether to borrow. **Unified `spot`** only. `0`: false, `1`: true  <br/>*Classic `spot` is not supported, always `0`*                                                              |
|                \> blockTradeId                |string |                                                                                                                 Block trade ID                                                                                                                  |
|                   \> symbol                   |string |                                                                                                                   Symbol name                                                                                                                   |
|                   \> price                    |string |                                                                                                                   Order price                                                                                                                   |
|                    \> qty                     |string |                                                                                                                    Order qty                                                                                                                    |
|                    \> side                    |string |                                                                                                               Side. `Buy`,`Sell`                                                                                                                |
|  \> [positionIdx](/docs/v5/enum#positionidx)  |integer|                                                                                     Position index. Used to identify positions in different position modes                                                                                      |
|  \> [orderStatus](/docs/v5/enum#orderstatus)  |string |                                                                                                                  Order status                                                                                                                   |
|   \> [createType](/docs/v5/enum#createtype)   |string |                                                                             Order create type Only for category=linear or inverseSpot, Option do not have this key                                                                              |
|   \> [cancelType](/docs/v5/enum#canceltype)   |string |                                                                                                                   Cancel type                                                                                                                   |
| \> [rejectReason](/docs/v5/enum#rejectreason) |string |                                                                                                Reject reason. *Classic `spot` is not supported*                                                                                                 |
|                  \> avgPrice                  |string |                      Average filled price returns `""` for those orders without avg price, and also for those classic account orders have partilly filled but cancelled at the endClassic Spot: not supported, always `""`                      |
|                 \> leavesQty                  |string |                                                                                        The remaining qty not executed. *Classic `spot` is not supported*                                                                                        |
|                \> leavesValue                 |string |                                                                                       The remaining value not executed. *Classic `spot` is not supported*                                                                                       |
|                 \> cumExecQty                 |string |                                                                                                          Cumulative executed order qty                                                                                                          |
|                \> cumExecValue                |string |                                                                                                         Cumulative executed order value                                                                                                         |
|                 \> cumExecFee                 |string |Cumulative executed trading fee.<br/><br/>* Classic `spot`: it is the latest execution fee for order.<br/>* After upgraded to the Unified account, you can use `execFee` for each fill in [Execution](/docs/v5/websocket/private/execution) topic|
|                 \> closedPnl                  |string |                                                 Closed profit and loss for each close position order. The figure is the same as "closedPnl" from [Get Closed PnL](/docs/v5/position/close-pnl)                                                  |
|                \> feeCurrency                 |string |                                                       Trading fee currency for Spot only. Please understand Spot trading fee currency [here](/docs/v5/enum#spot-fee-currency-instruction)                                                       |
|  \> [timeInForce](/docs/v5/enum#timeinforce)  |string |                                                                                                                  Time in force                                                                                                                  |
|    \> [orderType](/docs/v5/enum#ordertype)    |string |                                                                             Order type. `Market`,`Limit`. For TP/SL order, it means the order type after triggered                                                                              |
|\> [stopOrderType](/docs/v5/enum#stopordertype)|string |                                                                                                                 Stop order type                                                                                                                 |
|                \> ocoTriggerBy                |string |                                                         The trigger type of Spot OCO order.`OcoTriggerByUnknown`, `OcoTriggerByTp`, `OcoTriggerBySl`. *Classic `spot` is not supported*                                                         |
|                  \> orderIv                   |string |                                                                                                               Implied volatility                                                                                                                |
|                 \> marketUnit                 |string |                                                                       The unit for `qty` when create **Spot market** orders for **UTA account**. `baseCoin`, `quoteCoin`                                                                        |
|           \> slippageToleranceType            |string |                                                                         Spot and Futures market order slippage tolerance type `TickSize`, `Percent`, `UNKNOWN`(default)                                                                         |
|             \> slippageTolerance              |string |                                                                                                            Slippage tolerance value                                                                                                             |
|                \> triggerPrice                |string |                                                                     Trigger price. If `stopOrderType`=*TrailingStop*, it is activate price. Otherwise, it is trigger price                                                                      |
|                 \> takeProfit                 |string |                                                                                                                Take profit price                                                                                                                |
|                  \> stopLoss                  |string |                                                                                                                 Stop loss price                                                                                                                 |
|                  \> tpslMode                  |string |                                                  TP/SL mode, `Full`: entire position for TP/SL. `Partial`: partial position tp/sl. Spot does not have this field, and Option returns always ""                                                  |
|                \> tpLimitPrice                |string |                                                                                            The limit order price when take profit price is triggered                                                                                            |
|                \> slLimitPrice                |string |                                                                                             The limit order price when stop loss price is triggered                                                                                             |
|   \> [tpTriggerBy](/docs/v5/enum#triggerby)   |string |                                                                                                      The price type to trigger take profit                                                                                                      |
|   \> [slTriggerBy](/docs/v5/enum#triggerby)   |string |                                                                                                       The price type to trigger stop loss                                                                                                       |
|              \> triggerDirection              |integer|                                                                                                     Trigger direction. `1`: rise, `2`: fall                                                                                                     |
|    \> [triggerBy](/docs/v5/enum#triggerby)    |string |                                                                                                         The price type of trigger price                                                                                                         |
|             \> lastPriceOnCreated             |string |                                                                                                         Last price when place the order                                                                                                         |
|                 \> reduceOnly                 |boolean|                                                                                                 Reduce only. `true` means reduce position size                                                                                                  |
|               \> closeOnTrigger               |boolean|                                                   Close on trigger. [What is a close on trigger order?](https://www.bybit.com/en-US/help-center/bybitHC_Article?language=en_US&id=000001050)                                                    |
|                 \> placeType                  |string |                                                                                                    Place type, `option` used. `iv`, `price`                                                                                                     |
|      \> [smpType](/docs/v5/enum#smptype)      |string |                                                                                                               SMP execution type                                                                                                                |
|                  \> smpGroup                  |integer|                                                                                           Smp group ID. If the UID has no group, it is `0` by default                                                                                           |
|                 \> smpOrderId                 |string |                                                                                          The counterparty's orderID which triggers this SMP execution                                                                                           |
|                \> createdTime                 |string |                                                                                                          Order created timestamp (ms)                                                                                                           |
|                \> updatedTime                 |string |                                                                                                          Order updated timestamp (ms)                                                                                                           |

### Subscribe Example[​](#subscribe-example) ###

```
{    "op": "subscribe",    "args": [        "order"    ]}
```

```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="private",    api_key="XXXXX",    api_secret="XXXXX",)def handle_message(message):    print(message)ws.order_stream(callback=handle_message)while True:    sleep(1)
```

### Stream Example[​](#stream-example) ###

```
{    "id": "5923240c6880ab-c59f-420b-9adb-3639adc9dd90",    "topic": "order",    "creationTime": 1672364262474,    "data": [        {            "symbol": "ETH-30DEC22-1400-C",            "orderId": "5cf98598-39a7-459e-97bf-76ca765ee020",            "side": "Sell",            "orderType": "Market",            "cancelType": "UNKNOWN",            "price": "72.5",            "qty": "1",            "orderIv": "",            "timeInForce": "IOC",            "orderStatus": "Filled",            "orderLinkId": "",            "lastPriceOnCreated": "",            "reduceOnly": false,            "leavesQty": "",            "leavesValue": "",            "cumExecQty": "1",            "cumExecValue": "75",            "avgPrice": "75",            "blockTradeId": "",            "positionIdx": 0,            "cumExecFee": "0.358635",            "closedPnl": "0",            "createdTime": "1672364262444",            "updatedTime": "1672364262457",            "rejectReason": "EC_NoError",            "stopOrderType": "",            "tpslMode": "",            "triggerPrice": "",            "takeProfit": "",            "stopLoss": "",            "tpTriggerBy": "",            "slTriggerBy": "",            "tpLimitPrice": "",            "slLimitPrice": "",            "triggerDirection": 0,            "triggerBy": "",            "closeOnTrigger": false,            "category": "option",            "placeType": "price",            "smpType": "None",            "smpGroup": 0,            "smpOrderId": "",            "feeCurrency": ""        }    ]}
```

## WALLET

Wallet
==========

Subscribe to the wallet stream to see changes to your wallet in **real-time**.

info

* There is no snapshot event given at the time when the subscription is successful
* The unrealised PnL change does not trigger an event

**Topic:** `wallet`

### Response Parameters[​](#response-parameters) ###

|        Parameter        | Type  |                                                                                                                                                                                                                                 Comments                                                                                                                                                                                                                                 |
|:------------------------|:------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|           id            |string |                                                                                                                                                                                                                                Message ID                                                                                                                                                                                                                                |
|          topic          |string |                                                                                                                                                                                                                                Topic name                                                                                                                                                                                                                                |
|      creationTime       |number |                                                                                                                                                                                                                       Data created timestamp (ms)                                                                                                                                                                                                                        |
|          data           | array |                                                                                                                                                                                                                                  Object                                                                                                                                                                                                                                  |
|     \> accountType      |string |                                                                                                                                Account type.<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20): `UNIFIED`<br/>* [UTA1.0](/docs/v5/acct-mode#uta-10): `UNIFIED` (spot/linear/options), `CONTRACT`(inverse)<br/>* Classic: `CONTRACT`, `SPOT`                                                                                                                                 |
|      \> accountLTV      |string |                                                                                                                                                                                                                             deprecated field                                                                                                                                                                                                                             |
|    \> accountIMRate     |string |Account IM rate You can refer to this [Glossary](https://www.bybit.com/en/help-center/article/Glossary-Unified-Trading-Account) to understand the below fields calculation and mearningAll below account wide fields are **not** applicable to   <br/>    [UTA2.0](/docs/v5/acct-mode#uta-20)(isolated margin),   <br/>    [UTA1.0](/docs/v5/acct-mode#uta-10)(isolated margin), [UTA1.0](/docs/v5/acct-mode#uta-10)(CONTRACT),   <br/>    classic account(SPOT, CONTRACT)|
|    \> accountMMRate     |string |                                                                                                                                                                                                                             Account MM rate                                                                                                                                                                                                                              |
|     \> totalEquity      |string |                                                                                                                                                                                                                        Account total equity (USD)                                                                                                                                                                                                                        |
|  \> totalWalletBalance  |string |                                                                                                                                                                                              Account wallet balance (USD): ∑Asset Wallet Balance By USD value of each asset                                                                                                                                                                                              |
|  \> totalMarginBalance  |string |                                                                                                                                                                                                     Account margin balance (USD): totalWalletBalance + totalPerpUPL                                                                                                                                                                                                      |
|\> totalAvailableBalance |string |                                                                                                                                                                                          Account available balance (USD), Cross Margin: totalMarginBalance - totalInitialMargin                                                                                                                                                                                          |
|     \> totalPerpUPL     |string |                                                                                                                                                                                       Account Perps and Futures unrealised p&l (USD): ∑Each Perp and USDC Futures upl by base coin                                                                                                                                                                                       |
|  \> totalInitialMargin  |string |                                                                                                                                                                                                   Account initial margin (USD): ∑Asset Total Initial Margin Base Coin                                                                                                                                                                                                    |
|\> totalMaintenanceMargin|string |                                                                                                                                                                                               Account maintenance margin (USD): ∑ Asset Total Maintenance Margin Base Coin                                                                                                                                                                                               |
|         \> coin         | array |                                                                                                                                                                                                                                  Object                                                                                                                                                                                                                                  |
|        \>\> coin        |string |                                                                                                                                                                                                                 Coin name, such as BTC, ETH, USDT, USDC                                                                                                                                                                                                                  |
|       \>\> equity       |string |                                                                                                                                                                                                                              Equity of coin                                                                                                                                                                                                                              |
|      \>\> usdValue      |string |                                                                                                                                                                                                    USD value of coin. If this coin cannot be collateral, then it is 0                                                                                                                                                                                                    |
|   \>\> walletBalance    |string |                                                                                                                                                                                                                          Wallet balance of coin                                                                                                                                                                                                                          |
|        \>\> free        |string |                                                                                                                                                                                              Available balance for Spot wallet. *This is a unique field for Classic `SPOT`*                                                                                                                                                                                              |
|       \>\> locked       |string |                                                                                                                                                                                                                Locked balance due to the Spot open order                                                                                                                                                                                                                 |
|   \>\> spotHedgingQty   |string |                                                                                                                                                         The spot asset qty that is used to hedge in the portfolio margin, truncate to 8 decimals and "0" by default *This is a unique field for Unified account*                                                                                                                                                         |
|    \>\> borrowAmount    |string |                                                                                                                                                                                                                          Borrow amount of coin                                                                                                                                                                                                                           |
| \>\> availableToBorrow  |string |                                                                                                                                       deprecated field, always return `""` due to feature of main-sub UID sharing borrow quota. Please refer to `availableToBorrow` in the [Get Collateral Info](/docs/v5/account/collateral-info)                                                                                                                                       |
|\>\> availableToWithdraw |string |                                                                                                                                   Available amount to withdraw of coin   <br/>**Note:** this field is deprecated for accountType=UNIFIED, you can use [Get Transferable Amount (Unified)](/docs/v5/account/unified-trans-amnt) instead                                                                                                                                   |
|  \>\> accruedInterest   |string |                                                                                                                                                                                                                             Accrued interest                                                                                                                                                                                                                             |
|    \>\> totalOrderIM    |string |                                                                                                                                                                                                 Pre-occupied margin for order. For portfolio margin mode, it returns ""                                                                                                                                                                                                  |
|  \>\> totalPositionIM   |string |                                                                                                                                                                             Sum of initial margin of all positions + Pre-occupied liquidation fee. For portfolio margin mode, it returns ""                                                                                                                                                                              |
|  \>\> totalPositionMM   |string |                                                                                                                                                                                          Sum of maintenance margin for all positions. For portfolio margin mode, it returns ""                                                                                                                                                                                           |
|   \>\> unrealisedPnl    |string |                                                                                                                                                                                                                              Unrealised P&L                                                                                                                                                                                                                              |
|   \>\> cumRealisedPnl   |string |                                                                                                                                                                                                                         Cumulative Realised P&L                                                                                                                                                                                                                          |
|       \>\> bonus        |string |                                                                                                                                                                                                           Bonus. *This is a unique field for UNIFIED account*                                                                                                                                                                                                            |
|  \>\> collateralSwitch  |boolean|                                                                                                                                     Whether it can be used as a margin collateral currency (platform)<br/><br/>* When marginCollateral=false, then collateralSwitch is meaningless<br/>* This is a unique field for UNIFIED account                                                                                                                                      |
|  \>\> marginCollateral  |boolean|                                                                                                                                              Whether the collateral is turned on by user (user)<br/><br/>* When marginCollateral=true, then collateralSwitch is meaningful<br/>* This is a unique field for UNIFIED account                                                                                                                                              |

### Subscribe Example[​](#subscribe-example) ###

```
{    "op": "subscribe",    "args": [        "wallet"    ]}
```

```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="private",    api_key="XXXXX",    api_secret="XXXXX",)def handle_message(message):    print(message)ws.wallet_stream(callback=handle_message)while True:    sleep(1)
```

### Stream Example[​](#stream-example) ###

```
{    "id": "592324d2bce751-ad38-48eb-8f42-4671d1fb4d4e",    "topic": "wallet",    "creationTime": 1700034722104,    "data": [        {            "accountIMRate": "0",            "accountMMRate": "0",            "totalEquity": "10262.91335023",            "totalWalletBalance": "9684.46297164",            "totalMarginBalance": "9684.46297164",            "totalAvailableBalance": "9556.6056555",            "totalPerpUPL": "0",            "totalInitialMargin": "0",            "totalMaintenanceMargin": "0",            "coin": [                {                    "coin": "BTC",                    "equity": "0.00102964",                    "usdValue": "36.70759517",                    "walletBalance": "0.00102964",                    "availableToWithdraw": "0.00102964",                    "availableToBorrow": "",                    "borrowAmount": "0",                    "accruedInterest": "0",                    "totalOrderIM": "",                    "totalPositionIM": "",                    "totalPositionMM": "",                    "unrealisedPnl": "0",                    "cumRealisedPnl": "-0.00000973",                    "bonus": "0",                    "collateralSwitch": true,                    "marginCollateral": true,                    "locked": "0",                    "spotHedgingQty": "0.01592413"                }            ],            "accountLTV": "0",            "accountType": "UNIFIED"        }    ]}
```

## GREEK

Greek
==========

Subscribe to the greeks stream to see changes to your greeks data in **real-time**. `option` only.

**Topic:** `greeks`

### Response Parameters[​](#response-parameters) ###

|  Parameter  | Type |         Comments          |
|:------------|:-----|---------------------------|
|     id      |string|        Message ID         |
|    topic    |string|        Topic name         |
|creationTime |number|Data created timestamp (ms)|
|    data     |array |          Object           |
| \> baseCoin |string|         Base coin         |
|\> totalDelta|string|        Delta value        |
|\> totalGamma|string|        Gamma value        |
|\> totalVega |string|        Vega value         |
|\> totalTheta|string|        Theta value        |

### Subscribe Example[​](#subscribe-example) ###

```
{    "op": "subscribe",    "args": [        "greeks"    ]}
```

```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="private",    api_key="XXXXX",    api_secret="XXXXX",)def handle_message(message):    print(message)ws.greek_stream(callback=handle_message)while True:    sleep(1)
```

### Stream Example[​](#stream-example) ###

```
{    "id": "592324fa945a30-2603-49a5-b865-21668c29f2a6",    "topic": "greeks",    "creationTime": 1672364262482,    "data": [        {            "baseCoin": "ETH",            "totalDelta": "0.06999986",            "totalGamma": "-0.00000001",            "totalVega": "-0.00000024",            "totalTheta": "0.00001314"        }    ]}
```

## DCP

Dcp
==========

Subscribe to the dcp stream to trigger DCP function.

For example, connection A subscribes "dcp.xxx", connection B does not and connection C subscribes "dcp.xxx".

1. If A is alive, B is dead, C is alive, then this case will not trigger DCP.
2. If A is alive, B is dead, C is dead, then this case will not trigger DCP.
3. If A is dead, B is alive, C is dead, then DCP is triggered when reach the timeWindow threshold

To sum up, for those private connections subscribing "dcp" topic are all dead, then DCP will be triggered.

**Topic:** `dcp.future`, `dcp.spot`, `dcp.option`

### Subscribe Example[​](#subscribe-example) ###

```
{    "op": "subscribe",    "args": [        "dcp.future"    ]}
```

## GUIDELINE

Websocket Trade Guideline
==========

URL[​](#url)
----------

* **Mainnet:**  
  `wss://stream.bybit.com/v5/trade`

info

* Turkey users registered from "[www.bybit-tr.com"](http://www.bybit-tr.com%22), please use `wss://stream.bybit-tr.com/v5/trade`
* Kazakhstan users registered from "[www.bybit.kz"](http://www.bybit.kz%22), please use `wss://stream.bybit.kz/v5/trade`

* **Testnet:**  
  `wss://stream-testnet.bybit.com/v5/trade`

Support[​](#support)
----------

* [UTA2.0](/docs/v5/acct-mode#uta-20): category=linear, spot, option, **inverse**
* [UTA1.0](/docs/v5/acct-mode#uta-10): category=linear, spot, option

Authentication[​](#authentication)
----------

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                                        Comments                                                         |
|:--------|:-------|:-----|-------------------------------------------------------------------------------------------------------------------------|
|  reqId  | false  |string|          Optional field, used to match the response If not passed, this field will not be returned in response          |
|   op    |**true**|string|                                                     Op type. `auth`                                                     |
|  args   |**true**|string|["api key", expiry timestamp, "signature"]. Please click [here](/docs/v5/ws/connect#authentication) to generate signature|

### Response Parameters[​](#response-parameters) ###

|Parameter| Type  |                                                           Comments                                                            |
|:--------|:------|-------------------------------------------------------------------------------------------------------------------------------|
|  reqId  |string |If it is passed on the request, then it is returned in the responseIf it is not passed, then it is not returned in the response|
| retCode |integer|                        `0`: auth success`20001`: repeat auth`10004`: invalid sign`10001`: param error                         |
| retMsg  |string |                                                       `OK`Error message                                                       |
|   op    |string |                                                            Op type                                                            |
| connId  |string |                                        Connection id, the unique id for the connection                                        |

### Request Example[​](#request-example) ###

```
{    "op": "auth",    "args": [        "XXXXXX",        1711010121452,        "ec71040eff72b163a36153d770b69d6637bcb29348fbfbb16c269a76595ececf"    ]}
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "op": "auth",    "connId": "cnt5leec0hvan15eukcg-2t"}
```

Create/Amend/Cancel Order[​](#createamendcancel-order)
----------

### Request Parameters[​](#request-parameters-1) ###

|      Parameter      |Required|     Type      |                                                                                                                                                              Comments                                                                                                                                                              |
|:--------------------|:-------|:--------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|        reqId        | false  |    string     |                                                                 Used to identify the uniqueness of the request, the response will return it when passed. The length cannot exceed 36 characters. If passed, it can't be duplicated, otherwise you will get "20006"                                                                 |
|       header        |**true**|    object     |                                                                                                                                                          Request headers                                                                                                                                                           |
| \> X-BAPI-TIMESTAMP |**true**|    string     |                                                                                                                                                         Current timestamp                                                                                                                                                          |
|\> X-BAPI-RECV-WINDOW| false  |    string     |                                                                              5000(ms) by default. Request will be rejected when not satisfy this rule: *Bybit\_server\_time - X-BAPI-RECV-WINDOW \<= X-BAPI-TIMESTAMP \< Bybit\_server\_time + 1000*                                                                               |
|     \> Referer      | false  |    string     |                                                                                                                                             The referer identifier for API broker user                                                                                                                                             |
|         op          |**true**|    string     |                                                                                                               Op type `order.create`: create an order `order.amend`: amend an order `order.cancel`: cancel an order                                                                                                                |
|        args         |**true**|array\<object\>|Args array, support one item only for now `order.create`: refer to [create order request](/docs/v5/order/create-order#request-parameters)`order.amend`: refer to [amend order request](/docs/v5/order/amend-order#request-parameters)`order.cancel`: refer to [cancel order request](/docs/v5/order/cancel-order#request-parameters)|

### Response Parameters[​](#response-parameters-1) ###

|           Parameter           | Type  |                                                                                                                                                                                                                             Comments                                                                                                                                                                                                                             |
|:------------------------------|:------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|             reqId             |string |                                                                                                                                                                 If it is passed on the request, then it is returned in the responseIf it is not passed, then it is not returned in the response                                                                                                                                                                  |
|            retCode            |integer|`0`: success`10403`: exceed IP rate limit. 3000 requests per second per IP`10404`: 1. op type is not found; 2. `category` is not correct/supported`10429`: System level frequency protection`20006`: reqId is duplicated`10016`: 1. internal server error; 2. Service is restarting`10019`: ws trade service is restarting, do not accept new request, but the request in the process is not affected. You can build new connection to be routed to normal service|
|            retMsg             |string |                                                                                                                                                                                                                      `OK``""`Error message                                                                                                                                                                                                                       |
|              op               |string |                                                                                                                                                                                                                             Op type                                                                                                                                                                                                                              |
|             data              |object |                                                  Business data, keep the same as `result` on rest api response `order.create`: refer to [create order response](/docs/v5/order/create-order#response-parameters)`order.amend`: refer to [amend order response](/docs/v5/order/amend-order#response-parameters)`order.cancel`: refer to [cancel order response](/docs/v5/order/cancel-order#response-parameters)                                                  |
|          retExtInfo           |object |                                                                                                                                                                                                                       Always empty object                                                                                                                                                                                                                        |
|            header             |object |                                                                                                                                                                                                                           Header info                                                                                                                                                                                                                            |
|          \> TraceId           |string |                                                                                                                                                                                                           Trace ID, used to track the trip of request                                                                                                                                                                                                            |
|          \> Timenow           |string |                                                                                                                                                                                                                        Current timestamp                                                                                                                                                                                                                         |
|        \> X-Bapi-Limit        |string |                                                                                                                                                                                                   The total rate limit of the current account for this op type                                                                                                                                                                                                   |
|    \> X-Bapi-Limit-Status     |string |                                                                                                                                                                                                 The remaining rate limit of the current account for this op type                                                                                                                                                                                                 |
|\> X-Bapi-Limit-Reset-Timestamp|string |                                                                                                                                         The timestamp indicates when your request limit resets if you have exceeded your rate limit. Otherwise, this is just the current timestamp (it may not exactly match `timeNow`)                                                                                                                                          |
|            connId             |string |                                                                                                                                                                                                         Connection id, the unique id for the connection                                                                                                                                                                                                          |

info

The ack of create/amend/cancel order request indicates that the request is successfully accepted. Please use websocket order stream to confirm the order status

### Request Example[​](#request-example-1) ###

```
{    "reqId": "test-005",    "header": {        "X-BAPI-TIMESTAMP": "1711001595207",        "X-BAPI-RECV-WINDOW": "8000",        "Referer": "bot-001" // for api broker    },    "op": "order.create",    "args": [        {            "symbol": "ETHUSDT",            "side": "Buy",            "orderType": "Limit",            "qty": "0.2",            "price": "2800",            "category": "linear",            "timeInForce": "PostOnly"        }    ]}
```

### Response Example[​](#response-example-1) ###

```
{    "reqId": "test-005",    "retCode": 0,    "retMsg": "OK",    "op": "order.create",    "data": {        "orderId": "a4c1718e-fe53-4659-a118-1f6ecce04ad9",        "orderLinkId": ""    },    "retExtInfo": {},    "header": {        "X-Bapi-Limit": "10",        "X-Bapi-Limit-Status": "9",        "X-Bapi-Limit-Reset-Timestamp": "1711001595208",        "Traceid": "38b7977b430f9bd228f4b19724794dfd",        "Timenow": "1711001595209"    },    "connId": "cnt5leec0hvan15eukcg-2v"}
```

Batch Create/Amend/Cancel Order[​](#batch-createamendcancel-order)
----------

info

* A maximum of 20 orders (option), 20 orders (inverse), 20 orders (linear), 10 orders (spot) can be placed per request. The returned data list is divided into two lists. The first list indicates whether or not the order creation was successful and the second list details the created order information. The structure of the two lists are completely consistent.

* **Option rate limt** instruction: its rate limit is count based on the actual number of request sent, e.g., by default, option trading rate limit is 10 reqs per sec, so you can send up to 20 \* 10 = 200 orders in one second.
* **Perpetual, Futures, Spot rate limit instruction**, please check [here](/docs/v5/rate-limit#api-rate-limit-rules-for-vips)

* The account rate limit is shared between websocket and http batch orders
* The ack of batch create/amend/cancel order request indicates that the request is successfully accepted. Please use websocket order stream to confirm the order status

### Request Parameters[​](#request-parameters-2) ###

|      Parameter      |Required|     Type      |                                                                                                                                                               Comments                                                                                                                                                                |
|:--------------------|:-------|:--------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|        reqId        | false  |    string     |                                                                  Used to identify the uniqueness of the request, the response will return it when passed. The length cannot exceed 36 characters. If passed, it can't be duplicated, otherwise you will get "20006"                                                                   |
|       header        |**true**|    object     |                                                                                                                                                            Request headers                                                                                                                                                            |
| \> X-BAPI-TIMESTAMP |**true**|    string     |                                                                                                                                                           Current timestamp                                                                                                                                                           |
|\> X-BAPI-RECV-WINDOW| false  |    string     |                                                                                5000(ms) by default. Request will be rejected when not satisfy this rule: *Bybit\_server\_time - X-BAPI-RECV-WINDOW \<= X-BAPI-TIMESTAMP \< Bybit\_server\_time + 1000*                                                                                |
|     \> Referer      | false  |    string     |                                                                                                                                              The referer identifier for API broker user                                                                                                                                               |
|         op          |**true**|    string     |                                                                                                  Op type `order.create-batch`: batch create orders `order.amend-batch`: batch amend orders `order.cancel-batch`: batch cancel orders                                                                                                  |
|        args         |**true**|array\<object\>|Args array `order.create-batch`: refer to [Batch Place Order request](/docs/v5/order/batch-place#request-parameters)`order.amend-batch`: refer to [Batch Amend Order request](/docs/v5/order/batch-amend#request-parameters)`order.cancel-batch`: refer to [Batch Cancel Order request](/docs/v5/order/batch-cancel#request-parameters)|

### Response Parameters[​](#response-parameters-2) ###

|           Parameter           |     Type      |                                                                                                                                                                                                                             Comments                                                                                                                                                                                                                             |
|:------------------------------|:--------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|             reqId             |    string     |                                                                                                                                                                 If it is passed on the request, then it is returned in the responseIf it is not passed, then it is not returned in the response                                                                                                                                                                  |
|            retCode            |    integer    |`0`: success`10403`: exceed IP rate limit. 3000 requests per second per IP`10404`: 1. op type is not found; 2. `category` is not correct/supported`10429`: System level frequency protection`20006`: reqId is duplicated`10016`: 1. internal server error; 2. Service is restarting`10019`: ws trade service is restarting, do not accept new request, but the request in the process is not affected. You can build new connection to be routed to normal service|
|            retMsg             |    string     |                                                                                                                                                                                                                      `OK``""`Error message                                                                                                                                                                                                                       |
|              op               |    string     |                                                                                                                                                                                                                             Op type                                                                                                                                                                                                                              |
|             data              |    object     |                                 Business data, keep the same as `result` on rest api response `order.create-batch`: refer to [Batch Place Order response](/docs/v5/order/batch-place#response-parameters)`order.amend-batch`: refer to [Batch Amend Order response](/docs/v5/order/batch-amend#response-parameters)`order.cancel-batch`: refer to [Batch Cancel Order response](/docs/v5/order/batch-cancel#response-parameters)                                 |
|          retExtInfo           |    object     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
|            \> list            |array\<object\>|                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
|           \>\> code           |    number     |                                                                                                                                                                                                                        Success/error code                                                                                                                                                                                                                        |
|           \>\> msg            |    string     |                                                                                                                                                                                                                      Success/error message                                                                                                                                                                                                                       |
|            header             |    object     |                                                                                                                                                                                                                           Header info                                                                                                                                                                                                                            |
|          \> TraceId           |    string     |                                                                                                                                                                                                           Trace ID, used to track the trip of request                                                                                                                                                                                                            |
|          \> Timenow           |    string     |                                                                                                                                                                                                                        Current timestamp                                                                                                                                                                                                                         |
|        \> X-Bapi-Limit        |    string     |                                                                                                                                                                                                   The total rate limit of the current account for this op type                                                                                                                                                                                                   |
|    \> X-Bapi-Limit-Status     |    string     |                                                                                                                                                                                                 The remaining rate limit of the current account for this op type                                                                                                                                                                                                 |
|\> X-Bapi-Limit-Reset-Timestamp|    string     |                                                                                                                                         The timestamp indicates when your request limit resets if you have exceeded your rate limit. Otherwise, this is just the current timestamp (it may not exactly match `timeNow`)                                                                                                                                          |
|            connId             |    string     |                                                                                                                                                                                                         Connection id, the unique id for the connection                                                                                                                                                                                                          |

### Request Example[​](#request-example-2) ###

```
{    "op": "order.create-batch",    "header": {        "X-BAPI-TIMESTAMP": "1740453381256",        "X-BAPI-RECV-WINDOW": "1000"    },    "args": [        {            "category": "linear",            "request": [                {                    "symbol": "SOLUSDT",                    "qty": "10",                    "price": "500",                    "orderType": "Limit",                    "timeInForce": "GTC",                    "orderLinkId": "-batch-000",                    "side": "Buy"                },                {                    "symbol": "SOLUSDT",                    "qty": "20",                    "price": "1000",                    "orderType": "Limit",                    "timeInForce": "GTC",                    "orderLinkId": "batch-001",                    "side": "Buy"                },                {                    "symbol": "SOLUSDT",                    "qty": "30",                    "price": "1500",                    "orderType": "Limit",                    "timeInForce": "GTC",                    "orderLinkId": "batch-002",                    "side": "Buy"                }            ]        }    ]}
```

### Response Example[​](#response-example-2) ###

```
{    "retCode": 0,    "retMsg": "OK",    "op": "order.create-batch",    "data": {        "list": [            {                "category": "linear",                "symbol": "SOLUSDT",                "orderId": "",                "orderLinkId": "batch-000",                "createAt": ""            },            {                "category": "linear",                "symbol": "SOLUSDT",                "orderId": "",                "orderLinkId": "batch-001",                "createAt": ""            },            {                "category": "linear",                "symbol": "SOLUSDT",                "orderId": "",                "orderLinkId": "batch-002",                "createAt": ""            }        ]    },    "retExtInfo": {        "list": [            {                "code": 10001,                "msg": "position idx not match position mode"            },            {                "code": 10001,                "msg": "position idx not match position mode"            },            {                "code": 10001,                "msg": "position idx not match position mode"            }        ]    },    "header": {        "Timenow": "1740453408556",        "X-Bapi-Limit": "150",        "X-Bapi-Limit-Status": "147",        "X-Bapi-Limit-Reset-Timestamp": "1740453408555",        "Traceid": "0e32b551b3e17aae77651aadf6a5be80"    },    "connId": "cupviqn88smf24t2kpb0-536o"}
```

Ping[​](#ping)
----------

### Request Parameters[​](#request-parameters-3) ###

|Parameter|Required| Type |   Comments    |
|:--------|:-------|:-----|---------------|
|   op    |**true**|string|Op type. `ping`|

### Response Parameters[​](#response-parameters-3) ###

|Parameter| Type  |                    Comments                     |
|:--------|:------|-------------------------------------------------|
| retCode |integer|                   Result code                   |
| retMsg  |string |                 Result message                  |
|   op    |string |                 Op type `pong`                  |
|  data   | array |One item in the array, current timestamp (string)|
| connId  |string | Connection id, the unique id for the connection |

### Request Example[​](#request-example-3) ###

```
{    "op": "ping"}
```

### Response Example[​](#response-example-3) ###

```
{    "retCode": 0,    "retMsg": "OK",    "op": "pong",    "data": [        "1711002002529"    ],    "connId": "cnt5leec0hvan15eukcg-2v"}
```

