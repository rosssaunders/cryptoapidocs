# ByBit V5 REST API Documentation

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

## TIME

Get Bybit Server Time
==========

### HTTP Request[​](#http-request) ###

GET `/v5/market/time`

### Request Parameters[​](#request-parameters) ###

None

### Response Parameters[​](#response-parameters) ###

|Parameter | Type |          Comments           |
|:---------|:-----|-----------------------------|
|timeSecond|string|Bybit server timestamp (sec) |
| timeNano |string|Bybit server timestamp (nano)|
[RUN \>\>](/docs/api-explorer/v5/market/time)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Go
* Node.js

```
GET /v5/market/time HTTP/1.1Host: api.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_server_time())
```

```
import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();client.getServerTime(System.out::println);
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))client.NewUtaBybitServiceNoParams().GetServerTime(context.Background())
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,});client  .getServerTime()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "timeSecond": "1688639403",        "timeNano": "1688639403423213947"    },    "retExtInfo": {},    "time": 1688639403423}
```

## KLINE

Get Kline
==========

Query for historical klines (also known as candles/candlesticks). Charts are returned in groups based on the requested interval.

>
>
> **Covers: Spot / USDT contract / USDC contract / Inverse contract**
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/market/kline`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                           Comments                                           |
|:---------------------------------|:-------|:------|----------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)| false  |string |Product type. `spot`,`linear`,`inverse` When `category` is not passed, use `linear` by default|
|  [symbol](/docs/v5/enum#symbol)  |**true**|string |                         Symbol name, like `BTCUSDT`, uppercase only                          |
|[interval](/docs/v5/enum#interval)|**true**|string |        Kline interval. `1`,`3`,`5`,`15`,`30`,`60`,`120`,`240`,`360`,`720`,`D`,`W`,`M`        |
|              start               | false  |integer|                                   The start timestamp (ms)                                   |
|               end                | false  |integer|                                    The end timestamp (ms)                                    |
|              limit               | false  |integer|                 Limit for data size per page. [`1`, `1000`]. Default: `200`                  |

### Response Parameters[​](#response-parameters) ###

|      Parameter       | Type |                                                    Comments                                                     |
|:---------------------|:-----|-----------------------------------------------------------------------------------------------------------------|
|       category       |string|                                                  Product type                                                   |
|        symbol        |string|                                                   Symbol name                                                   |
|         list         |array |                   * An string array of individual candle<br/>* Sort in reverse by `startTime`                   |
|\> list[0]: startTime |string|                                          Start time of the candle (ms)                                          |
|\> list[1]: openPrice |string|                                                   Open price                                                    |
|\> list[2]: highPrice |string|                                                  Highest price                                                  |
| \> list[3]: lowPrice |string|                                                  Lowest price                                                   |
|\> list[4]: closePrice|string|                      Close price. *Is the last traded price when the candle is not closed*                      |
|  \> list[5]: volume  |string|Trade volume USDT or USDC contract: unit is base coin (e.g., BTC)Inverse contract: unit is quote coin (e.g., USD)|
| \> list[6]: turnover |string| Turnover. USDT or USDC contract: unit is quote coin (e.g., USDT)Inverse contract: unit is base coin (e.g., BTC) |
[RUN \>\>](/docs/api-explorer/v5/market/kline)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Go
* Java
* Node.js

```
GET /v5/market/kline?category=inverse&symbol=BTCUSD&interval=60&start=1670601600000&end=1670608800000 HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_kline(    category="inverse",    symbol="BTCUSD",    interval=60,    start=1670601600000,    end=1670608800000,))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT", "interval": "1"}client.NewUtaBybitServiceWithParams(params).GetMarketKline(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var marketKLineRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").marketInterval(MarketInterval.WEEKLY).build();client.getMarketLinesData(marketKLineRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getKline({        category: 'inverse',        symbol: 'BTCUSD',        interval: '60',        start: 1670601600000,        end: 1670608800000,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "symbol": "BTCUSD",        "category": "inverse",        "list": [            [                "1670608800000",                "17071",                "17073",                "17027",                "17055.5",                "268611",                "15.74462667"            ],            [                "1670605200000",                "17071.5",                "17071.5",                "17061",                "17071",                "4177",                "0.24469757"            ],            [                "1670601600000",                "17086.5",                "17088",                "16978",                "17071.5",                "6356",                "0.37288112"            ]        ]    },    "retExtInfo": {},    "time": 1672025956592}
```

## MARK KLINE

Get Mark Price Kline
==========

Query for historical [mark price](https://www.bybit.com/en-US/help-center/s/article/Glossary-Bybit-Trading-Terms) klines. Charts are returned in groups based on the requested interval.

>
>
> **Covers: USDT contract / USDC contract / Inverse contract**
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/market/mark-price-kline`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                       Comments                                        |
|:---------------------------------|:-------|:------|---------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)| false  |string |Product type. `linear`,`inverse` When `category` is not passed, use `linear` by default|
|              symbol              |**true**|string |                      Symbol name, like `BTCUSDT`, uppercase only                      |
|[interval](/docs/v5/enum#interval)|**true**|string |    Kline interval. `1`,`3`,`5`,`15`,`30`,`60`,`120`,`240`,`360`,`720`,`D`,`M`,`W`     |
|              start               | false  |integer|                               The start timestamp (ms)                                |
|               end                | false  |integer|                                The end timestamp (ms)                                 |
|              limit               | false  |integer|              Limit for data size per page. [`1`, `1000`]. Default: `200`              |

### Response Parameters[​](#response-parameters) ###

|      Parameter       | Type |                                 Comments                                  |
|:---------------------|:-----|---------------------------------------------------------------------------|
|       category       |string|                               Product type                                |
|        symbol        |string|                                Symbol name                                |
|         list         |array |* An string array of individual candle<br/>* Sort in reverse by `startTime`|
|\> list[0]: startTime |string|                       Start time of the candle (ms)                       |
|\> list[1]: openPrice |string|                                Open price                                 |
|\> list[2]: highPrice |string|                               Highest price                               |
| \> list[3]: lowPrice |string|                               Lowest price                                |
|\> list[4]: closePrice|string|   Close price. *Is the last traded price when the candle is not closed*   |
[RUN \>\>](/docs/api-explorer/v5/market/mark-kline)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Go
* Java
* Node.js

```
GET /v5/market/mark-price-kline?category=linear&symbol=BTCUSDT&interval=15&start=1670601600000&end=1670608800000&limit=1 HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_mark_price_kline(    category="linear",    symbol="BTCUSDT",    interval=15,    start=1670601600000,    end=1670608800000,    limit=1,))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT", "interval": "1"}client.NewUtaBybitServiceWithParams(params).GetMarkPriceKline(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var marketKLineRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").marketInterval(MarketInterval.WEEKLY).build();client.getMarketPriceLinesData(marketKLineRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getMarkPriceKline({        category: 'linear',        symbol: 'BTCUSD',        interval: '15',        start: 1670601600000,        end: 1670608800000,        limit: 1,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "symbol": "BTCUSDT",        "category": "linear",        "list": [            [            "1670608800000",            "17164.16",            "17164.16",            "17121.5",            "17131.64"            ]        ]    },    "retExtInfo": {},    "time": 1672026361839}
```

## INDEX KLINE

Get Index Price Kline
==========

Query for historical [index price](https://www.bybit.com/en-US/help-center/s/article/Glossary-Bybit-Trading-Terms) klines. Charts are returned in groups based on the requested interval.

>
>
> **Covers: USDT contract / USDC contract / Inverse contract**
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/market/index-price-kline`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                       Comments                                        |
|:---------------------------------|:-------|:------|---------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)| false  |string |Product type. `linear`,`inverse` When `category` is not passed, use `linear` by default|
|              symbol              |**true**|string |                      Symbol name, like `BTCUSDT`, uppercase only                      |
|[interval](/docs/v5/enum#interval)|**true**|string |    Kline interval. `1`,`3`,`5`,`15`,`30`,`60`,`120`,`240`,`360`,`720`,`D`,`W`,`M`     |
|              start               | false  |integer|                               The start timestamp (ms)                                |
|               end                | false  |integer|                                The end timestamp (ms)                                 |
|              limit               | false  |integer|              Limit for data size per page. [`1`, `1000`]. Default: `200`              |

### Response Parameters[​](#response-parameters) ###

|      Parameter       | Type |                                 Comments                                  |
|:---------------------|:-----|---------------------------------------------------------------------------|
|       category       |string|                               Product type                                |
|        symbol        |string|                                Symbol name                                |
|         list         |array |* An string array of individual candle<br/>* Sort in reverse by `startTime`|
|\> list[0]: startTime |string|                       Start time of the candle (ms)                       |
|\> list[1]: openPrice |string|                                Open price                                 |
|\> list[2]: highPrice |string|                               Highest price                               |
| \> list[3]: lowPrice |string|                               Lowest price                                |
|\> list[4]: closePrice|string|   Close price. *Is the last traded price when the candle is not closed*   |
[RUN \>\>](/docs/api-explorer/v5/market/index-kline)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Go
* Java
* Node.js

```
GET /v5/market/index-price-kline?category=inverse&symbol=BTCUSDZ22&interval=1&start=1670601600000&end=1670608800000&limit=2 HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_index_price_kline(    category="inverse",    symbol="BTCUSDZ22",    interval=1,    start=1670601600000,    end=1670608800000,    limit=2,))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT", "interval": "1"}client.NewUtaBybitServiceWithParams(params).GetIndexPriceKline(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var marketKLineRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").marketInterval(MarketInterval.WEEKLY).build();client.getIndexPriceLinesData(marketKLineRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getIndexPriceKline({        category: 'inverse',        symbol: 'BTCUSDZ22',        interval: '1',        start: 1670601600000,        end: 1670608800000,        limit: 2,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "symbol": "BTCUSDZ22",        "category": "inverse",        "list": [            [                "1670608800000",                "17167.00",                "17167.00",                "17161.90",                "17163.07"            ],            [                "1670608740000",                "17166.54",                "17167.69",                "17165.42",                "17167.00"            ]        ]    },    "retExtInfo": {},    "time": 1672026471128}
```

## PREMIUM INDEX KLINE

Get Premium Index Price Kline
==========

Query for historical [premium index](https://www.bybit.com/data/basic/linear/index-price/premium-index?symbol=BTCUSDT) klines. Charts are returned in groups based on the requested interval.

>
>
> **Covers: USDT and USDC perpetual**
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/market/premium-index-price-kline`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                   Comments                                   |
|:---------------------------------|:-------|:------|------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)| false  |string |                            Product type. `linear`                            |
|              symbol              |**true**|string |                 Symbol name, like `BTCUSDT`, uppercase only                  |
|[interval](/docs/v5/enum#interval)|**true**|string |Kline interval. `1`,`3`,`5`,`15`,`30`,`60`,`120`,`240`,`360`,`720`,`D`,`W`,`M`|
|              start               | false  |integer|                           The start timestamp (ms)                           |
|               end                | false  |integer|                            The end timestamp (ms)                            |
|              limit               | false  |integer|         Limit for data size per page. [`1`, `1000`]. Default: `200`          |

### Response Parameters[​](#response-parameters) ###

|            Parameter             | Type |                               Comments                                |
|:---------------------------------|:-----|-----------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|string|                             Product type                              |
|              symbol              |string|                              Symbol name                              |
|               list               |array |* An string array of individual candle<br/>* Sort in reverse by `start`|
|            \> list[0]            |string|                     Start time of the candle (ms)                     |
|            \> list[1]            |string|                              Open price                               |
|            \> list[2]            |string|                             Highest price                             |
|            \> list[3]            |string|                             Lowest price                              |
|            \> list[4]            |string| Close price. *Is the last traded price when the candle is not closed* |
[RUN \>\>](/docs/api-explorer/v5/market/premium-index-kline)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Go
* Java
* Node.js

```
GET /v5/market/premium-index-price-kline?category=linear&symbol=BTCUSDT&interval=D&start=1652112000000&end=1652544000000 HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP()print(session.get_premium_index_price_kline(    category="linear",    symbol="BTCUSDT",    inverval="D",    start=1652112000000,    end=1652544000000,))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT", "interval": "1"}client.NewUtaBybitServiceWithParams(params).GetPremiumIndexPriceKline(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var marketKLineRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").marketInterval(MarketInterval.WEEKLY).build();client.getPremiumIndexPriceLinesData(marketKLineRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getPremiumIndexPriceKline({        category: 'linear',        symbol: 'BTCUSDT',        interval: 'D',        start: 1652112000000,        end: 1652544000000,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "symbol": "BTCUSDT",        "category": "linear",        "list": [            [                "1652486400000",                "-0.000587",                "-0.000344",                "-0.000480",                "-0.000344"            ],            [                "1652400000000",                "-0.000989",                "-0.000561",                "-0.000587",                "-0.000587"            ]        ]    },    "retExtInfo": {},    "time": 1672765216291}
```

## INSTRUMENT

Get Instruments Info
==========

Query for the instrument specification of online trading pairs.

>
>
> **Covers: Spot / USDT contract / USDC contract / Inverse contract / Option**
>
>

info

* Spot does not support pagination, so `limit`, `cursor` are invalid.
* When query by `baseCoin`, regardless of category=`linear` or `inverse`, the result will have USDT contract, USDC contract and Inverse contract symbols.

### HTTP Request[​](#http-request) ###

GET `/v5/market/instruments-info`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                                                                        Comments                                                                                        |
|:---------------------------------|:-------|:------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string |                                                                    Product type. `spot`,`linear`,`inverse`,`option`                                                                    |
|  [symbol](/docs/v5/enum#symbol)  | false  |string |                                                                      Symbol name, like `BTCUSDT`, uppercase only                                                                       |
|  [status](/docs/v5/enum#status)  | false  |string |Symbol status filter<br/><br/>* By defualt return only `Trading` symbols<br/>* spot has `Trading` only<br/>* `linear` & `inverse`: when status=PreLaunch, it returns pre-market contract|
|             baseCoin             | false  |string |                                       Base coin, uppercase only Apply to`linear`,`inverse`,`option` **only**`option`: it returns BTC by default                                        |
|              limit               | false  |integer|                                                              Limit for data size per page. [`1`, `1000`]. Default: `500`                                                               |
|              cursor              | false  |string |                                          Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set                                          |

### Response Parameters[​](#response-parameters) ###

* Linear/Inverse
* Option
* Spot

|                      Parameter                      |     Type      |                                                                                           Comments                                                                                            |
|-----------------------------------------------------|---------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                      category                       |    string     |                                                                                         Product type                                                                                          |
|                   nextPageCursor                    |    string     |                                                                                  Cursor. Used to pagination                                                                                   |
|                        list                         |     array     |                                                                                            Object                                                                                             |
|                      \> symbol                      |    string     |                                                                                          Symbol name                                                                                          |
|    \> [contractType](/docs/v5/enum#contracttype)    |    string     |                                                                                         Contract type                                                                                         |
|          \> [status](/docs/v5/enum#status)          |    string     |                                                                                       Instrument status                                                                                       |
|                     \> baseCoin                     |    string     |                                                                                           Base coin                                                                                           |
|                    \> quoteCoin                     |    string     |                                                                                          Quote coin                                                                                           |
|                    \> launchTime                    |    string     |                                                                                     Launch timestamp (ms)                                                                                     |
|                   \> deliveryTime                   |    string     |                                                                                    Delivery timestamp (ms)                                                                                    |
|                 \> deliveryFeeRate                  |    string     |                                                                                       Delivery fee rate                                                                                       |
|                    \> priceScale                    |    string     |                                                                                          Price scale                                                                                          |
|                  \> leverageFilter                  |    Object     |                                                                                      Leverage attributes                                                                                      |
|                  \>\> minLeverage                   |    string     |                                                                                       Minimum leverage                                                                                        |
|                  \>\> maxLeverage                   |    string     |                                                                                       Maximum leverage                                                                                        |
|                  \>\> leverageStep                  |    string     |                                                                             The step to increase/reduce leverage                                                                              |
|                   \> priceFilter                    |    Object     |                                                                                       Price attributes                                                                                        |
|                    \>\> minPrice                    |    string     |                                                                                      Minimum order price                                                                                      |
|                    \>\> maxPrice                    |    string     |                                                                                      Maximum order price                                                                                      |
|                    \>\> tickSize                    |    string     |                                                                            The step to increase/reduce order price                                                                            |
|                  \> lotSizeFilter                   |    Object     |                                                                                        Size attributes                                                                                        |
|                \>\> minNotionalValue                |    string     |                                                                                    Minimum notional value                                                                                     |
|                  \>\> maxOrderQty                   |    string     |                                                                         Maximum quantity for Limit and PostOnly order                                                                         |
|                 \>\> maxMktOrderQty                 |    string     |                                                                               Maximum quantity for Market order                                                                               |
|                  \>\> minOrderQty                   |    string     |                                                                                    Minimum order quantity                                                                                     |
|                    \>\> qtyStep                     |    string     |                                                                          The step to increase/reduce order quantity                                                                           |
|              \>\> postOnlyMaxOrderQty               |    string     |                                                                             deprecated, please use `maxOrderQty`                                                                              |
|                \> unifiedMarginTrade                |    boolean    |                                                                            Whether to support unified margin trade                                                                            |
|                 \> fundingInterval                  |    integer    |                                                                                   Funding interval (minute)                                                                                   |
|                    \> settleCoin                    |    string     |                                                                                          Settle coin                                                                                          |
|     \> [copyTrading](/docs/v5/enum#copytrading)     |    string     |                                                                                   Copy trade symbol or not                                                                                    |
|                 \> upperFundingRate                 |    string     |                                                                                  Upper limit of funding date                                                                                  |
|                 \> lowerFundingRate                 |    string     |                                                                                  Lower limit of funding date                                                                                  |
|                  \> riskParameters                  |    object     |Risk parameters for limit order price, refer to [announcement](https://announcements.bybit.com/en/article/adjustments-to-bybit-s-derivative-trading-limit-order-mechanism-blt469228de1902fff6/)|
|                \>\> priceLimitRatioX                |    string     |                                                                                            Ratio X                                                                                            |
|                \>\> priceLimitRatioY                |    string     |                                                                                            Ratio Y                                                                                            |
|                   \> isPreListing                   |    boolean    |                                 Whether the contract is a pre-market contractWhen the pre-market contract is converted to official contract, it will be false                                 |
|                  \> preListingInfo                  |    object     |                                                  If isPreListing=false, preListingInfo=nullIf isPreListing=true, preListingInfo is an object                                                  |
|\>\> [curAuctionPhase](/docs/v5/enum#curauctionphase)|    string     |                                                                                   The current auction phase                                                                                   |
|                     \>\> phases                     |array\<object\>|                                                                                     Each phase time info                                                                                      |
|    \>\>\> [phase](/docs/v5/enum#curauctionphase)    |    string     |                                                                                   pre-market trading phase                                                                                    |
|                  \>\>\> startTime                   |    string     |                                                                          The start time of the phase, timestamp(ms)                                                                           |
|                   \>\>\> endTime                    |    string     |                                                                           The end time of the phase, timestamp(ms)                                                                            |
|                 \>\> auctionFeeInfo                 |    object     |                                                                                        Action fee info                                                                                        |
|                \>\>\> auctionFeeRate                |    string     |                                           The trading fee rate during auction phase There is no trading fee until entering continues trading phase                                            |
|                 \>\>\> takerFeeRate                 |    string     |                                                                       The taker fee rate during continues trading phase                                                                       |
|                 \>\>\> makerFeeRate                 |    string     |                                                                       The maker fee rate during continues trading phase                                                                       |

|            Parameter            | Type |                 Comments                 |
|---------------------------------|------|------------------------------------------|
|            category             |string|               Product type               |
|         nextPageCursor          |string|        Cursor. Used to pagination        |
|              list               |array |                  Object                  |
|            \> symbol            |string|               Symbol name                |
|         \> optionsType          |string|        Option type. `Call`, `Put`        |
|\> [status](/docs/v5/enum#status)|string|            Instrument status             |
|           \> baseCoin           |string|                Base coin                 |
|          \> quoteCoin           |string|                Quote coin                |
|          \> settleCoin          |string|               Settle coin                |
|          \> launchTime          |string|          Launch timestamp (ms)           |
|         \> deliveryTime         |string|         Delivery timestamp (ms)          |
|       \> deliveryFeeRate        |string|            Delivery fee rate             |
|         \> priceFilter          |Object|             Price attributes             |
|          \>\> minPrice          |string|           Minimum order price            |
|          \>\> maxPrice          |string|           Maximum order price            |
|          \>\> tickSize          |string| The step to increase/reduce order price  |
|        \> lotSizeFilter         |Object|             Size attributes              |
|        \>\> maxOrderQty         |string|          Maximum order quantity          |
|        \>\> minOrderQty         |string|          Minimum order quantity          |
|          \>\> qtyStep           |string|The step to increase/reduce order quantity|

|                   Parameter                   | Type |                                                                                                                                                        Comments                                                                                                                                                         |
|-----------------------------------------------|------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                   category                    |string|                                                                                                                                                      Product type                                                                                                                                                       |
|                     list                      |array |                                                                                                                                                         Object                                                                                                                                                          |
|                   \> symbol                   |string|                                                                                                                                                       Symbol name                                                                                                                                                       |
|                  \> baseCoin                  |string|                                                                                                                                                        Base coin                                                                                                                                                        |
|                 \> quoteCoin                  |string|                                                                                                                                                       Quote coin                                                                                                                                                        |
|                 \> innovation                 |string|                                                                                 Whether or not this is an [innovation zone token](https://blog.bybit.com/en-us/post/bybit-innovation-zone-blt055db8d22a445fa6/). `0`: false, `1`: true                                                                                  |
|       \> [status](/docs/v5/enum#status)       |string|                                                                                                                                                    Instrument status                                                                                                                                                    |
|\> [marginTrading](/docs/v5/enum#margintrading)|string|Margin trade symbol or not<br/><br/>* This is to identify if the symbol support margin trading under different account modes<br/>* You may find some symbols not supporting margin buy or margin sell, so you need to go to [Collateral Info (UTA)](/docs/v5/account/collateral-info) to check if that coin is borrowable|
|                   \> stTag                    |string|                                                                        Whether or not it has an [special treatment label](https://www.bybit.com/en/help-center/article/Bybit-Special-Treatment-ST-Label-Management-Rules). `0`: false, `1`: true                                                                        |
|               \> lotSizeFilter                |Object|                                                                                                                                                     Size attributes                                                                                                                                                     |
|              \>\> basePrecision               |string|                                                                                                                                               The precision of base coin                                                                                                                                                |
|              \>\> quotePrecision              |string|                                                                                                                                               The precision of quote coin                                                                                                                                               |
|               \>\> minOrderQty                |string|                                                                                                                                                 Minimum order quantity                                                                                                                                                  |
|               \>\> maxOrderQty                |string|                                                                                                                                                 Maximum order quantity                                                                                                                                                  |
|               \>\> minOrderAmt                |string|                                                                                                                                                  Minimum order amount                                                                                                                                                   |
|               \>\> maxOrderAmt                |string|                                                                                                                                                  Maximum order amount                                                                                                                                                   |
|                \> priceFilter                 |Object|                                                                                                                                                    Price attributes                                                                                                                                                     |
|                 \>\> tickSize                 |string|                                                                                                                                         The step to increase/reduce order price                                                                                                                                         |
|               \> riskParameters               |Object|                                                             Risk parameters for limit order price, refer to [announcement](https://announcements.bybit.com/en/article/title-adjustments-to-bybit-s-spot-trading-limit-order-mechanism-blt786c0c5abf865983/)                                                             |
|             \>\> priceLimitRatioX             |string|                                                                                                                                                         Ratio X                                                                                                                                                         |
|             \>\> priceLimitRatioY             |string|                                                                                                                                                         Ratio Y                                                                                                                                                         |

[RUN \>\>](/docs/api-explorer/v5/market/instrument)
---

### Request Example[​](#request-example) ###

* Linear
* Option
* Spot

* HTTP
* Python
* Go
* Java
* Node.js

```
GET /v5/market/instruments-info?category=linear&symbol=BTCUSDT HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_instruments_info(    category="linear",    symbol="BTCUSDT",))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetInstrumentInfo(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var instrumentInfoRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").instrumentStatus(InstrumentStatus.TRADING).limit(500).build();client.getInstrumentsInfo(instrumentInfoRequest,System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getInstrumentsInfo({        category: 'linear',        symbol: 'BTCUSDT',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

* HTTP
* Python
* Go
* Java
* Node.js

```
GET /v5/market/instruments-info?category=option&symbol=ETH-3JAN23-1250-P HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_instruments_info(    category="option",    symbol="ETH-3JAN23-1250-P",))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "option", "symbol": "ETH-3JAN23-1250-P"}client.NewUtaBybitServiceWithParams(params).GetInstrumentInfo(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var instrumentInfoRequest = MarketDataRequest.builder().category(CategoryType.OPTION).symbol("ETH-3JAN23-1250-P").instrumentStatus(InstrumentStatus.TRADING).limit(500).build();client.getInstrumentsInfo(instrumentInfoRequest,System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client  .getInstrumentsInfo({    category: 'option',    symbol: 'ETH-3JAN23-1250-P',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

* HTTP
* Python
* Go
* Java
* Node.js

```
GET /v5/market/instruments-info?category=spot&symbol=BTCUSDT HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_instruments_info(    category="spot",    symbol="BTCUSDT",))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetInstrumentInfo(context.Background())
```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var instrumentInfoRequest = MarketDataRequest.builder().category(CategoryType.SPOT).symbol("BTCUSDT").instrumentStatus(InstrumentStatus.TRADING).limit(500).build();client.getInstrumentsInfo(instrumentInfoRequest,System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client  .getInstrumentsInfo({    category: 'spot',    symbol: 'BTCUSDT',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

* Linear
* Option
* Spot

```
// official USDT Perpetual instrument structure{    "retCode": 0,    "retMsg": "OK",    "result": {        "category": "linear",        "list": [            {                "symbol": "BTCUSDT",                "contractType": "LinearPerpetual",                "status": "Trading",                "baseCoin": "BTC",                "quoteCoin": "USDT",                "launchTime": "1585526400000",                "deliveryTime": "0",                "deliveryFeeRate": "",                "priceScale": "2",                "leverageFilter": {                    "minLeverage": "1",                    "maxLeverage": "100.00",                    "leverageStep": "0.01"                },                "priceFilter": {                    "minPrice": "0.10",                    "maxPrice": "1999999.80",                    "tickSize": "0.10"                },                "lotSizeFilter": {                    "maxOrderQty": "1190.000",                    "minOrderQty": "0.001",                    "qtyStep": "0.001",                    "postOnlyMaxOrderQty": "1190.000",                    "maxMktOrderQty": "500.000",                    "minNotionalValue": "5"                },                "unifiedMarginTrade": true,                "fundingInterval": 480,                "settleCoin": "USDT",                "copyTrading": "both",                "upperFundingRate": "0.00375",                "lowerFundingRate": "-0.00375",                "isPreListing": false,                "preListingInfo": null,                "riskParameters": {                    "priceLimitRatioX": "0.01",                    "priceLimitRatioY": "0.02"                }            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1735809771618}// Pre-market Perpetual instrument structure{    "retCode": 0,    "retMsg": "OK",    "result": {        "category": "linear",        "list": [            {                "symbol": "BIOUSDT",                "contractType": "LinearPerpetual",                "status": "PreLaunch",                "baseCoin": "BIO",                "quoteCoin": "USDT",                "launchTime": "1735032510000",                "deliveryTime": "0",                "deliveryFeeRate": "",                "priceScale": "4",                "leverageFilter": {                    "minLeverage": "1",                    "maxLeverage": "5.00",                    "leverageStep": "0.01"                },                "priceFilter": {                    "minPrice": "0.0001",                    "maxPrice": "1999.9998",                    "tickSize": "0.0001"                },                "lotSizeFilter": {                    "maxOrderQty": "70000",                    "minOrderQty": "1",                    "qtyStep": "1",                    "postOnlyMaxOrderQty": "70000",                    "maxMktOrderQty": "14000",                    "minNotionalValue": "5"                },                "unifiedMarginTrade": true,                "fundingInterval": 480,                "settleCoin": "USDT",                "copyTrading": "none",                "upperFundingRate": "0.05",                "lowerFundingRate": "-0.05",                "isPreListing": true,                "preListingInfo": {                    "curAuctionPhase": "ContinuousTrading",                    "phases": [                        {                            "phase": "CallAuction",                            "startTime": "1735113600000",                            "endTime": "1735116600000"                        },                        {                            "phase": "CallAuctionNoCancel",                            "startTime": "1735116600000",                            "endTime": "1735116900000"                        },                        {                            "phase": "CrossMatching",                            "startTime": "1735116900000",                            "endTime": "1735117200000"                        },                        {                            "phase": "ContinuousTrading",                            "startTime": "1735117200000",                            "endTime": ""                        }                    ],                    "auctionFeeInfo": {                        "auctionFeeRate": "0",                        "takerFeeRate": "0.001",                        "makerFeeRate": "0.0004"                    }                },                "riskParameters": {                    "priceLimitRatioX": "0.05",                    "priceLimitRatioY": "0.1"                }            }        ],        "nextPageCursor": "first%3DBIOUSDT%26last%3DBIOUSDT"    },    "retExtInfo": {},    "time": 1735810114435}
```

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "category": "option",        "nextPageCursor": "",        "list": [            {                "symbol": "ETH-3JAN23-1250-P",                "status": "Trading",                "baseCoin": "ETH",                "quoteCoin": "USD",                "settleCoin": "USDC",                "optionsType": "Put",                "launchTime": "1672560000000",                "deliveryTime": "1672732800000",                "deliveryFeeRate": "0.00015",                "priceFilter": {                    "minPrice": "0.1",                    "maxPrice": "10000000",                    "tickSize": "0.1"                },                "lotSizeFilter": {                    "maxOrderQty": "1500",                    "minOrderQty": "0.1",                    "qtyStep": "0.1"                }            }        ]    },    "retExtInfo": {},    "time": 1672712537130}
```

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "category": "spot",        "list": [            {                "symbol": "BTCUSDT",                "baseCoin": "BTC",                "quoteCoin": "USDT",                "innovation": "0",                "status": "Trading",                "marginTrading": "utaOnly",                "stTag": "0",                "lotSizeFilter": {                    "basePrecision": "0.000001",                    "quotePrecision": "0.00000001",                    "minOrderQty": "0.000048",                    "maxOrderQty": "71.73956243",                    "minOrderAmt": "1",                    "maxOrderAmt": "2000000"                },                "priceFilter": {                    "tickSize": "0.01"                }                "riskParameters": {                    "priceLimitRatioX": "0.01",                    "priceLimitRatioY": "0.02"                }            }        ]    },    "retExtInfo": {},    "time": 1672712468011}
```

## ORDERBOOK

Get Orderbook
==========

Query for orderbook depth data.

>
>
> **Covers: Spot / USDT contract / USDC contract / Inverse contract / Option**
>
>

* Contract: 500-level of orderbook data
* Spot: 200-level of orderbook data
* Option: 25-level of orderbook data

info

The response is in the snapshot format.

### HTTP Request[​](#http-request) ###

GET `/v5/market/orderbook`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                                                                    Comments                                                                                    |
|:---------------------------------|:-------|:------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string |                                                              Product type. `spot`, `linear`, `inverse`, `option`                                                               |
|              symbol              |**true**|string |                                                                  Symbol name, like `BTCUSDT`, uppercase only                                                                   |
|              limit               | false  |integer|Limit size for each bid and ask<br/><br/>* `spot`: [`1`, `200`]. Default: `1`.<br/>* `linear`&`inverse`: [`1`, `500`]. Default: `25`.<br/>* `option`: [`1`, `25`]. Default: `1`.|

### Response Parameters[​](#response-parameters) ###

|Parameter| Type  |                                                                                          Comments                                                                                           |
|:--------|:------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    s    |string |                                                                                         Symbol name                                                                                         |
|    b    | array |                                                                               Bid, buyer. Sort by price desc                                                                                |
| \> b[0] |string |                                                                                          Bid price                                                                                          |
| \> b[1] |string |                                                                                          Bid size                                                                                           |
|    a    | array |                                                                               Ask, seller. Order by price asc                                                                               |
| \> a[0] |string |                                                                                          Ask price                                                                                          |
| \> a[1] |string |                                                                                          Ask size                                                                                           |
|   ts    |integer|                                                                    The timestamp (ms) that the system generates the data                                                                    |
|    u    |integer|Update ID, is always in sequence<br/><br/>* For contract, it is corresponding to `u` in the wss 500-level orderbook<br/>* For spot, it is corresponding to `u` in the wss 200-level orderbook|
|   seq   |integer|                   Cross sequence You can use this field to compare different levels orderbook data, and for the smaller seq, then it means the data is generated earlier.                   |
|   cts   |integer|            The timestamp from the match engine when this orderbook data is produced. It can be correlated with `T` from [public trade channel](/docs/v5/websocket/public/trade)             |
[RUN \>\>](/docs/api-explorer/v5/market/orderbook)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Go
* Java
* Node.js

```
GET /v5/market/orderbook?category=spot&symbol=BTCUSDT HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_orderbook(    category="linear",    symbol="BTCUSDT",))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetOrderBookInfo(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var orderbookRequest = MarketDataRequest.builder().category(CategoryType.SPOT).symbol("BTCUSDT").build();client.getMarketOrderBook(orderbookRequest,System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getOrderbook({        category: 'linear',        symbol: 'BTCUSDT',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "s": "BTCUSDT",        "a": [            [                "65557.7",                "16.606555"            ]        ],        "b": [            [                "65485.47",                "47.081829"            ]        ],        "ts": 1716863719031,        "u": 230704,        "seq": 1432604333,        "cts": 1716863718905    },    "retExtInfo": {},    "time": 1716863719382}
```

## TICKERS

Get Tickers
==========

Query for the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.

>
>
> **Covers: Spot / USDT contract / USDC contract / Inverse contract / Option**
>
>

info

If category=*option*, `symbol` or `baseCoin` must be passed.

### HTTP Request[​](#http-request) ###

GET `/v5/market/tickers`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type |                       Comments                       |
|:---------------------------------|:-------|:-----|------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string|   Product type. `spot`,`linear`,`inverse`,`option`   |
|  [symbol](/docs/v5/enum#symbol)  | false  |string|     Symbol name, like `BTCUSDT`, uppercase only      |
|             baseCoin             | false  |string|Base coin, uppercase only. Apply to `option` **only** |
|             expDate              | false  |string|Expiry date. e.g., 25DEC22. Apply to `option` **only**|

### Response Parameters[​](#response-parameters) ###

* Linear/Inverse
* Option
* Spot

|                      Parameter                       | Type |                                      Comments                                       |
|------------------------------------------------------|------|-------------------------------------------------------------------------------------|
|                       category                       |string|                                    Product type                                     |
|                         list                         |array |                                       Object                                        |
|                      \> symbol                       |string|                                     Symbol name                                     |
|                     \> lastPrice                     |string|                                     Last price                                      |
|                    \> indexPrice                     |string|                                     Index price                                     |
|                     \> markPrice                     |string|                                     Mark price                                      |
|                   \> prevPrice24h                    |string|                              Market price 24 hours ago                              |
|                   \> price24hPcnt                    |string|                  Percentage change of market price relative to 24h                  |
|                   \> highPrice24h                    |string|                       The highest price in the last 24 hours                        |
|                    \> lowPrice24h                    |string|                        The lowest price in the last 24 hours                        |
|                    \> prevPrice1h                    |string|                              Market price an hour ago                               |
|                   \> openInterest                    |string|                                 Open interest size                                  |
|                 \> openInterestValue                 |string|                                 Open interest value                                 |
|                    \> turnover24h                    |string|                                  Turnover for 24h                                   |
|                     \> volume24h                     |string|                                   Volume for 24h                                    |
|                    \> fundingRate                    |string|                                    Funding rate                                     |
|                  \> nextFundingTime                  |string|                               Next funding time (ms)                                |
|              \> predictedDeliveryPrice               |string|          Predicated delivery price. It has a value 30 mins before delivery          |
|                     \> basisRate                     |string|                                     Basis rate                                      |
|                       \> basis                       |string|                                        Basis                                        |
|                  \> deliveryFeeRate                  |string|                                  Delivery fee rate                                  |
|                   \> deliveryTime                    |string|                               Delivery timestamp (ms)                               |
|                     \> ask1Size                      |string|                                    Best ask size                                    |
|                     \> bid1Price                     |string|                                   Best bid price                                    |
|                     \> ask1Price                     |string|                                   Best ask price                                    |
|                     \> bid1Size                      |string|                                    Best bid size                                    |
|                   \> preOpenPrice                    |string|Estimated pre-market contract open price<br/><br/>* Meaningless once the market opens|
|                      \> preQty                       |string|Estimated pre-market contract open qty The value is meaningless once the market opens|
|\> [curPreListingPhase](/docs/v5/enum#curauctionphase)|string|                        The current pre-market contract phase                        |

|        Parameter        | Type |                            Comments                             |
|-------------------------|------|-----------------------------------------------------------------|
|        category         |string|                          Product type                           |
|          list           |array |                             Object                              |
|        \> symbol        |string|                           Symbol name                           |
|      \> bid1Price       |string|                         Best bid price                          |
|       \> bid1Size       |string|                          Best bid size                          |
|        \> bid1Iv        |string|                           Best bid iv                           |
|      \> ask1Price       |string|                         Best ask price                          |
|       \> ask1Size       |string|                          Best ask size                          |
|        \> ask1Iv        |string|                           Best ask iv                           |
|      \> lastPrice       |string|                           Last price                            |
|     \> highPrice24h     |string|             The highest price in the last 24 hours              |
|     \> lowPrice24h      |string|              The lowest price in the last 24 hours              |
|      \> markPrice       |string|                           Mark price                            |
|      \> indexPrice      |string|                           Index price                           |
|        \> markIv        |string|                          Mark price iv                          |
|   \> underlyingPrice    |string|                        Underlying price                         |
|     \> openInterest     |string|                       Open interest size                        |
|     \> turnover24h      |string|                        Turnover for 24h                         |
|      \> volume24h       |string|                         Volume for 24h                          |
|     \> totalVolume      |string|                          Total volume                           |
|    \> totalTurnover     |string|                         Total turnover                          |
|        \> delta         |string|                              Delta                              |
|        \> gamma         |string|                              Gamma                              |
|         \> vega         |string|                              Vega                               |
|        \> theta         |string|                              Theta                              |
|\> predictedDeliveryPrice|string|Predicated delivery price. It has a value 30 mins before delivery|
|      \> change24h       |string|                 The change in the last 24 hous                  |

|   Parameter    | Type |                                                                                                   Comments                                                                                                    |
|----------------|------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    category    |string|                                                                                                 Product type                                                                                                  |
|      list      |array |                                                                                                    Object                                                                                                     |
|   \> symbol    |string|                                                                                                  Symbol name                                                                                                  |
|  \> bid1Price  |string|                                                                                                Best bid price                                                                                                 |
|  \> bid1Size   |string|                                                                                                 Best bid size                                                                                                 |
|  \> ask1Price  |string|                                                                                                Best ask price                                                                                                 |
|  \> ask1Size   |string|                                                                                                 Best ask size                                                                                                 |
|  \> lastPrice  |string|                                                                                                  Last price                                                                                                   |
|\> prevPrice24h |string|                                                                                           Market price 24 hours ago                                                                                           |
|\> price24hPcnt |string|                                                                               Percentage change of market price relative to 24h                                                                               |
|\> highPrice24h |string|                                                                                    The highest price in the last 24 hours                                                                                     |
| \> lowPrice24h |string|                                                                                     The lowest price in the last 24 hours                                                                                     |
| \> turnover24h |string|                                                                                               Turnover for 24h                                                                                                |
|  \> volume24h  |string|                                                                                                Volume for 24h                                                                                                 |
|\> usdIndexPrice|string|USD index price<br/><br/>* used to calculate USD value of the assets in Unified account<br/>* non-collateral margin coin returns ""<br/>* Only those trading pairs like "XXX/USDT" or "XXX/USDC" have the value|

[RUN \>\>](/docs/api-explorer/v5/market/tickers)
---

### Request Example[​](#request-example) ###

* Inverse
* Option
* Spot

* HTTP
* Python
* Go
* Java
* Node.js

```
GET /v5/market/tickers?category=inverse&symbol=BTCUSD HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_tickers(    category="inverse",    symbol="BTCUSD",))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "inverse", "symbol": "BTCUSD"}client.NewUtaBybitServiceWithParams(params).GetMarketTickers(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var tickerReueqt = MarketDataRequest.builder().category(CategoryType.INVERSE).symbol("BTCUSD").build();client.getMarketTickers(tickerReueqt, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getTickers({        category: 'inverse',        symbol: 'BTCUSDT',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

* HTTP
* Python
* Go
* Java
* Node.js

```
GET /v5/market/tickers?category=option&symbol=BTC-30DEC22-18000-C HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_tickers(    category="option",    symbol="BTC-30DEC22-18000-C",))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "option", "symbol": "BTC-30DEC22-18000-C"}client.NewUtaBybitServiceWithParams(params).GetMarketTickers(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var tickerReueqt = MarketDataRequest.builder().category(CategoryType.OPTION).symbol("BTC-30DEC22-18000-C").build();client.getMarketTickers(tickerReueqt, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client  .getTickers({    category: 'option',    symbol: 'BTC-30DEC22-18000-C',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

* HTTP
* Python
* GO
* Java
* Node.js

```
GET /v5/market/tickers?category=spot&symbol=BTCUSDT HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_tickers(    category="spot",    symbol="BTCUSDT",))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetMarketTickers(context.Background())
```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var tickerReueqt = MarketDataRequest.builder().category(CategoryType.SPOT).symbol("BTCUSDT").build();client.getMarketTickers(tickerReueqt, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client  .getTickers({    category: 'spot',    symbol: 'BTCUSDT',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

* Inverse
* Option
* Spot

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "category": "inverse",        "list": [            {                "symbol": "BTCUSD",                "lastPrice": "16597.00",                "indexPrice": "16598.54",                "markPrice": "16596.00",                "prevPrice24h": "16464.50",                "price24hPcnt": "0.008047",                "highPrice24h": "30912.50",                "lowPrice24h": "15700.00",                "prevPrice1h": "16595.50",                "openInterest": "373504107",                "openInterestValue": "22505.67",                "turnover24h": "2352.94950046",                "volume24h": "49337318",                "fundingRate": "-0.001034",                "nextFundingTime": "1672387200000",                "predictedDeliveryPrice": "",                "basisRate": "",                "deliveryFeeRate": "",                "deliveryTime": "0",                "ask1Size": "1",                "bid1Price": "16596.00",                "ask1Price": "16597.50",                "bid1Size": "1",                "basis": ""            }        ]    },    "retExtInfo": {},    "time": 1672376496682}
```

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "category": "option",        "list": [            {                "symbol": "BTC-30DEC22-18000-C",                "bid1Price": "0",                "bid1Size": "0",                "bid1Iv": "0",                "ask1Price": "435",                "ask1Size": "0.66",                "ask1Iv": "5",                "lastPrice": "435",                "highPrice24h": "435",                "lowPrice24h": "165",                "markPrice": "0.00000009",                "indexPrice": "16600.55",                "markIv": "0.7567",                "underlyingPrice": "16590.42",                "openInterest": "6.3",                "turnover24h": "2482.73",                "volume24h": "0.15",                "totalVolume": "99",                "totalTurnover": "1967653",                "delta": "0.00000001",                "gamma": "0.00000001",                "vega": "0.00000004",                "theta": "-0.00000152",                "predictedDeliveryPrice": "0",                "change24h": "86"            }        ]    },    "retExtInfo": {},    "time": 1672376592395}
```

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "category": "spot",        "list": [            {                "symbol": "BTCUSDT",                "bid1Price": "20517.96",                "bid1Size": "2",                "ask1Price": "20527.77",                "ask1Size": "1.862172",                "lastPrice": "20533.13",                "prevPrice24h": "20393.48",                "price24hPcnt": "0.0068",                "highPrice24h": "21128.12",                "lowPrice24h": "20318.89",                "turnover24h": "243765620.65899866",                "volume24h": "11801.27771",                "usdIndexPrice": "20784.12009279"            }        ]    },    "retExtInfo": {},    "time": 1673859087947}
```

## HISTORY FUND RATE

Get Funding Rate History
==========

Query for historical funding rates. Each symbol has a different funding interval. For example, if the interval is 8 hours and the current time is UTC 12, then it returns the last funding rate, which settled at UTC 8.

To query the funding rate interval, please refer to the [instruments-info](/docs/v5/market/instrument) endpoint.

>
>
> **Covers: USDT and USDC perpetual / Inverse perpetual**
>
>

info

* Passing only `startTime` returns an error.
* Passing only `endTime` returns 200 records up till `endTime`.
* Passing neither returns 200 records up till the current time.

### HTTP Request[​](#http-request) ###

GET `/v5/market/funding/history`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                         Comments                         |
|:---------------------------------|:-------|:------|----------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string |             Product type. `linear`,`inverse`             |
|              symbol              |**true**|string |       Symbol name, like `BTCUSDT`, uppercase only        |
|            startTime             | false  |integer|                 The start timestamp (ms)                 |
|             endTime              | false  |integer|                  The end timestamp (ms)                  |
|              limit               | false  |integer|Limit for data size per page. [`1`, `200`]. Default: `200`|

### Response Parameters[​](#response-parameters) ###

|       Parameter       | Type |         Comments          |
|:----------------------|:-----|---------------------------|
|       category        |string|       Product type        |
|         list          |array |          Object           |
|       \> symbol       |string|        Symbol name        |
|    \> fundingRate     |string|       Funding rate        |
|\> fundingRateTimestamp|string|Funding rate timestamp (ms)|
[RUN \>\>](/docs/api-explorer/v5/market/history-fund-rate)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* GO
* Java
* Node.js

```
GET /v5/market/funding/history?category=linear&symbol=ETHPERP&limit=1 HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP()print(session.get_funding_rate_history(    category="linear",    symbol="ETHPERP",    limit=1,))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetFundingRateHistory(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var fundingHistoryRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSD).startTime(1632046800000L).endTime(1632133200000L).limit(150).build();client.getFundingHistory(fundingHistoryRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getFundingRateHistory({        category: 'linear',        symbol: 'ETHPERP',        limit: 1,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "category": "linear",        "list": [            {                "symbol": "ETHPERP",                "fundingRate": "0.0001",                "fundingRateTimestamp": "1672041600000"            }        ]    },    "retExtInfo": {},    "time": 1672051897447}
```

## RECENT TRADE

Get Public Recent Trading History
==========

Query recent public trading data in Bybit.

>
>
> **Covers: Spot / USDT contract / USDC contract / Inverse contract / Option**
>
>

You can download archived historical trades from the [website](https://www.bybit.com/derivatives/en/history-data)

### HTTP Request[​](#http-request) ###

GET `/v5/market/recent-trade`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                                      Comments                                                       |
|:---------------------------------|:-------|:------|---------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string |                                  Product type. `spot`,`linear`,`inverse`,`option`                                   |
|  [symbol](/docs/v5/enum#symbol)  | false  |string |Symbol name, like `BTCUSDT`, uppercase only<br/><br/>* **required** for spot/linear/inverse<br/>* optional for option|
|             baseCoin             | false  |string |   Base coin, uppercase only Apply to `option` **only**If the field is not passed, return **BTC** data by default    |
|            optionType            | false  |string |                              Option type. `Call` or `Put`. Apply to `option` **only**                               |
|              limit               | false  |integer|    Limit for data size per page<br/><br/>* `spot`: [1,60], default: `60`<br/>* others: [1,1000], default: `500`     |

### Response Parameters[​](#response-parameters) ###

|   Parameter   | Type  |               Comments               |
|:--------------|:------|--------------------------------------|
|   category    |string |          Products category           |
|     list      | array |                Object                |
|   \> execId   |string |             Execution ID             |
|   \> symbol   |string |             Symbol name              |
|   \> price    |string |             Trade price              |
|    \> size    |string |              Trade size              |
|    \> side    |string |     Side of taker `Buy`, `Sell`      |
|    \> time    |string |           Trade time (ms)            |
|\> isBlockTrade|boolean|   Whether the trade is block trade   |
| \> isRPITrade |boolean|    Whether the trade is RPI trade    |
|     \> mP     |string |Mark price, unique field for `option` |
|     \> iP     |string |Index price, unique field for `option`|
|    \> mIv     |string |  Mark iv, unique field for `option`  |
|     \> iv     |string |    iv, unique field for `option`     |
[RUN \>\>](/docs/api-explorer/v5/market/recent-trade)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* GO
* Java
* Node.js

```
GET /v5/market/recent-trade?category=spot&symbol=BTCUSDT&limit=1 HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_public_trade_history(    category="spot",    symbol="BTCUSDT",    limit=1,))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetPublicRecentTrades(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var recentTrade = MarketDataRequest.builder().category(CategoryType.OPTION).symbol("ETH-30JUN23-2050-C").build();client.getRecentTradeData(recentTrade, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getPublicTradingHistory({        category: 'spot',        symbol: 'BTCUSDT',        limit: 1,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "category": "spot",        "list": [            {                "execId": "2100000000007764263",                "symbol": "BTCUSDT",                "price": "16618.49",                "size": "0.00012",                "side": "Buy",                "time": "1672052955758",                "isBlockTrade": false,                "isRPITrade": true            }        ]    },    "retExtInfo": {},    "time": 1672053054358}
```

## OPEN INTEREST

Get Open Interest
==========

Get the [open interest](https://www.bybit.com/en-US/help-center/s/article/Glossary-Bybit-Trading-Terms) of each symbol.

>
>
> **Covers: USDT contract / USDC contract / Inverse contract**
>
>

info

* The upper limit time you can query is the launch time of the symbol.

### HTTP Request[​](#http-request) ###

GET `/v5/market/open-interest`

### Request Parameters[​](#request-parameters) ###

|                Parameter                 |Required| Type  |                        Comments                         |
|:-----------------------------------------|:-------|:------|---------------------------------------------------------|
|    [category](/docs/v5/enum#category)    |**true**|string |            Product type. `linear`,`inverse`             |
|                  symbol                  |**true**|string |       Symbol name, like `BTCUSDT`, uppercase only       |
|[intervalTime](/docs/v5/enum#intervaltime)|**true**|string |  Interval time. `5min`,`15min`,`30min`,`1h`,`4h`,`1d`   |
|                startTime                 | false  |integer|                The start timestamp (ms)                 |
|                 endTime                  | false  |integer|                 The end timestamp (ms)                  |
|                  limit                   | false  |integer|Limit for data size per page. [`1`, `200`]. Default: `50`|
|                  cursor                  | false  |string |                Cursor. Used to paginate                 |

### Response Parameters[​](#response-parameters) ###

|   Parameter   | Type |                                                            Comments                                                             |
|:--------------|:-----|---------------------------------------------------------------------------------------------------------------------------------|
|   category    |string|                                                          Product type                                                           |
|    symbol     |string|                                                           Symbol name                                                           |
|     list      |array |                                                             Object                                                              |
|\> openInterest|string|Open interest. The value is the sum of both sides.   <br/>The unit of value, e.g., BTCUSD(inverse) is USD, BTCUSDT(linear) is BTC|
| \> timestamp  |string|                                                       The timestamp (ms)                                                        |
|nextPageCursor |string|                                                        Used to paginate                                                         |
[RUN \>\>](/docs/api-explorer/v5/market/open-interest)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* GO
* Java
* Node.js

```
GET /v5/market/open-interest?category=inverse&symbol=BTCUSD&intervalTime=5min&startTime=1669571100000&endTime=1669571400000 HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_open_interest(    category="inverse",    symbol="BTCUSD",    intervalTime="5min",    startTime=1669571100000,    endTime=1669571400000,))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetOpenInterests(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var openInterest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").marketInterval(MarketInterval.FIVE_MINUTES).build();client.getOpenInterest(openInterest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getOpenInterest({        category: 'inverse',        symbol: 'BTCUSD',        intervalTime: '5min',        startTime: 1669571100000,        endTime: 1669571400000,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "symbol": "BTCUSD",        "category": "inverse",        "list": [            {                "openInterest": "461134384.00000000",                "timestamp": "1669571400000"            },            {                "openInterest": "461134292.00000000",                "timestamp": "1669571100000"            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1672053548579}
```

## IV

Get Historical Volatility
==========

Query option historical volatility

>
>
> **Covers: Option**
>
>

info

* The data is hourly.
* If both `startTime` and `endTime` are not specified, it will return the most recent 1 hours worth of data.
* `startTime` and `endTime` are a pair of params. Either both are passed or they are not passed at all.
* This endpoint can query the last 2 years worth of data, but make sure [`endTime` - `startTime`] \<= 30 days.

### HTTP Request[​](#http-request) ###

GET `/v5/market/historical-volatility`

### Request Parameters[​](#request-parameters) ###

|             Parameter              |Required| Type  |                                  Comments                                   |
|:-----------------------------------|:-------|:------|-----------------------------------------------------------------------------|
|              category              |**true**|string |                           Product type. `option`                            |
|              baseCoin              | false  |string |             Base coin, uppercase only. Default: return BTC data             |
|             quoteCoin              | false  |string |         Quote coin, `USD` or `USDT`. Default: return quoteCoin=USD          |
|[period](/docs/v5/enum#optionperiod)| false  |integer|Period. If not specified, it will return data with a 7-day average by default|
|             startTime              | false  |integer|                          The start timestamp (ms)                           |
|              endTime               | false  |integer|                           The end timestamp (ms)                            |

### Response Parameters[​](#response-parameters) ###

|Parameter| Type  |   Comments   |
|:--------|:------|--------------|
|category |string | Product type |
|  list   | array |    Object    |
|\> period|integer|    Period    |
|\> value |string |  Volatility  |
| \> time |string |Timestamp (ms)|
[RUN \>\>](/docs/api-explorer/v5/market/iv)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
GET /v5/market/historical-volatility?category=option&baseCoin=ETH&period=30 HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_historical_volatility(    category="option",    baseCoin="ETH",    period=30,))
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var historicalVolatilityRequest = MarketDataRequest.builder().category(CategoryType.OPTION).optionPeriod(7).build();client.getHistoricalVolatility(historicalVolatilityRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getHistoricalVolatility({        category: 'option',        baseCoin: 'ETH',        period: 30,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "SUCCESS",    "category": "option",    "result": [        {            "period": 30,            "value": "0.45024716",            "time": "1672052400000"        }    ]}
```

## INSURANCE

Get Insurance
==========

Query for Bybit [insurance pool](https://www.bybit.com/en/announcement-info/insurance-fund/) data (BTC/USDT/USDC etc). The data is updated every 24 hours.

info

Since the insurance pool data is updated every 24 hours, it is possible that you get ADL trade but the insruance pool still has sufficient funds.

### HTTP Request[​](#http-request) ###

GET `/v5/market/insurance`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                        Comments                         |
|:--------|:-------|:-----|---------------------------------------------------------|
|  coin   | false  |string|coin, uppercase only. Default: return all insurance coins|

### Response Parameters[​](#response-parameters) ###

| Parameter | Type |                                                          Comments                                                           |
|:----------|:-----|-----------------------------------------------------------------------------------------------------------------------------|
|updatedTime|string|                                                   Data updated time (ms)                                                    |
|   list    |array |                                                           Object                                                            |
|  \> coin  |string|                                                            Coin                                                             |
|\> symbols |string|For an independent insurance pool, you may see `"BTCUSDT,ETHUSDT,SOLUSDT"`For non-independent insurance pool, it returns `""`|
|\> balance |string|                                                           Balance                                                           |
| \> value  |string|                                                          USD value                                                          |
[RUN \>\>](/docs/api-explorer/v5/market/insurance)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* GO
* Java
* Node.js

```
GET /v5/market/insurance?coin=ETH HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_insurance(    coin="ETH",))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetMarketInsurance(context.Background())
```

```
import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var insuranceRequest = MarketDataRequest.builder().coin("BTC").build();var insuranceData = client.getInsurance(insuranceRequest);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getInsurance({        coin: 'ETH',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "updatedTime": "1714003200000",        "list": [            {                "coin": "USDT",                "symbols": "MERLUSDT,10000000AIDOGEUSDT,ZEUSUSDT",                "balance": "902178.57602476",                "value": "901898.0963091522"            },            {                "coin": "USDT",                "symbols": "SOLUSDT,OMNIUSDT,ALGOUSDT",                "balance": "14454.51626125",                "value": "14449.515598975464"            },            {                "coin": "USDT",                "symbols": "XLMUSDT,WUSDT",                "balance": "23.45018235",                "value": "22.992864174376344"            },            {                "coin": "USDT",                "symbols": "AGIUSDT,WIFUSDT",                "balance": "10002",                "value": "9998.896846613574"            },            {                "coin": "USDT",                "symbols": "",                "balance": "10148045273.618073",                "value": "10144896808.587431"            }        ]    },    "retExtInfo": {},    "time": 1714028451228}
```

## RISK LIMIT

Get Risk Limit
==========

Query for the [risk limit](https://www.bybit.com/en/help-center/article/Risk-Limit-Perpetual-and-Futures).

>
>
> **Covers: USDT contract / USDC contract / Inverse contract**
>
>

info

category=`linear` returns a data set of 15 symbols in each response. Please use the `cursor` param to get the next data set.

### HTTP Request[​](#http-request) ###

GET `/v5/market/risk-limit`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type |                                             Comments                                             |
|:---------------------------------|:-------|:-----|--------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string|                                 Product type. `linear`,`inverse`                                 |
|              symbol              | false  |string|                           Symbol name, like `BTCUSDT`, uppercase only                            |
|              cursor              | false  |string|Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the data set|

### Response Parameters[​](#response-parameters) ###

|     Parameter      | Type  |                             Comments                              |
|:-------------------|:------|-------------------------------------------------------------------|
|      category      |string |                           Product type                            |
|        list        | array |                              Object                               |
|       \> id        |integer|                              Risk ID                              |
|     \> symbol      |string |                            Symbol name                            |
| \> riskLimitValue  |string |                          Position limit                           |
|\> maintenanceMargin|number |                       Maintain margin rate                        |
|  \> initialMargin  |number |                        Initial margin rate                        |
|  \> isLowestRisk   |integer|                       `1`: true, `0`: false                       |
|   \> maxLeverage   |string |                       Allowed max leverage                        |
|   \> mmDeduction   |string |The maintenance margin deduction value when risk limit tier changed|
|   nextPageCursor   |string |              Refer to the `cursor` request parameter              |
[RUN \>\>](/docs/api-explorer/v5/market/risk-limit)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* GO
* Java
* Node.js

```
GET /v5/market/risk-limit?category=inverse&symbol=BTCUSD HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_risk_limit(    category="inverse",    symbol="BTCUSD",))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetMarketRiskLimits(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var riskMimitRequest = MarketDataRequest.builder().category(CategoryType.INVERSE).symbol("ADAUSD").build();client.getRiskLimit(riskMimitRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getRiskLimit({        category: 'inverse',        symbol: 'BTCUSD',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "category": "inverse",        "list": [            {                "id": 1,                "symbol": "BTCUSD",                "riskLimitValue": "150",                "maintenanceMargin": "0.5",                "initialMargin": "1",                "isLowestRisk": 1,                "maxLeverage": "100.00",                "mmDeduction": ""            },        ....        ]    },    "retExtInfo": {},    "time": 1672054488010}
```

## DELIVERY PRICE

Get Delivery Price
==========

Get the delivery price.

>
>
> **Covers: USDT futures / USDC futures / Inverse futures / Option**
>
>

info

* Option: only returns those symbols which are `DELIVERING` (UTC 8 - UTC 12) when `symbol` is not specified.

### HTTP Request[​](#http-request) ###

GET `/v5/market/delivery-price`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                              Comments                                              |
|:---------------------------------|:-------|:------|----------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string |                            Product type. `linear`, `inverse`, `option`                             |
|              symbol              | false  |string |                            Symbol name, like `BTCUSDT`, uppercase only                             |
|             baseCoin             | false  |string |                Base coin, uppercase only. Default: `BTC`. *Valid for `option` only*                |
|              limit               | false  |integer|                     Limit for data size per page. [`1`, `200`]. Default: `50`                      |
|              cursor              | false  |string |Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set|

### Response Parameters[​](#response-parameters) ###

|   Parameter    | Type |               Comments                |
|:---------------|:-----|---------------------------------------|
|    category    |string|             Product type              |
|      list      |array |                Object                 |
|   \> symbol    |string|              Symbol name              |
|\> deliveryPrice|string|            Delivery price             |
|\> deliveryTime |string|        Delivery timestamp (ms)        |
| nextPageCursor |string|Refer to the `cursor` request parameter|
[RUN \>\>](/docs/api-explorer/v5/market/delivery-price)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* GO
* Java
* Node.js

```
GET /v5/market/delivery-price?category=option&symbol=ETH-26DEC22-1400-C HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP()print(session.get_option_delivery_price(    category="option",    symbol="ETH-26DEC22-1400-C",))
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "ETH-26DEC22-1400-C"}client.NewUtaBybitServiceWithParams(params).GetDeliveryPrice(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var deliveryPriceRequest = MarketDataRequest.builder().category(CategoryType.OPTION).baseCoin("BTC").limit(10).build();client.getDeliveryPrice(deliveryPriceRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getDeliveryPrice({ category: 'option', symbol: 'ETH-26DEC22-1400-C' })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "category": "option",        "nextPageCursor": "",        "list": [            {                "symbol": "ETH-26DEC22-1400-C",                "deliveryPrice": "1220.728594450",                "deliveryTime": "1672041600000"            }        ]    },    "retExtInfo": {},    "time": 1672055336993}
```

## LONG SHORT RATIO

Get Long Short Ratio
==========

### HTTP Request[​](#http-request) ###

GET `/v5/market/account-ratio`

### Request Parameters[​](#request-parameters) ###

|                 Parameter                 |Required| Type  |                                              Comments                                              |
|:------------------------------------------|:-------|:------|----------------------------------------------------------------------------------------------------|
|    [category](/docs/v5/enum#category)     |**true**|string |                          Product type. `linear`(USDT Contract),`inverse`                           |
|      [symbol](/docs/v5/enum#symbol)       |**true**|string |                            Symbol name, like `BTCUSDT`, uppercase only                             |
|[period](/docs/v5/enum#datarecordingperiod)|**true**|string |                 Data recording period. `5min`, `15min`, `30min`, `1h`, `4h`, `1d`                  |
|                 startTime                 | false  |string |                                      The start timestamp (ms)                                      |
|                  endTime                  | false  |string |                                       The end timestamp (ms)                                       |
|                   limit                   | false  |integer|                     Limit for data size per page. [`1`, `500`]. Default: `50`                      |
|                  cursor                   | false  |string |Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set|

### Response Parameters[​](#response-parameters) ###

|  Parameter   | Type |                Comments                 |
|:-------------|:-----|-----------------------------------------|
|     list     |array |                 Object                  |
|  \> symbol   |string|               Symbol name               |
| \> buyRatio  |string|The ratio of the number of long position |
| \> sellRatio |string|The ratio of the number of short position|
| \> timestamp |string|             Timestamp (ms)              |
|nextPageCursor|string| Refer to the `cursor` request parameter |
[RUN \>\>](/docs/api-explorer/v5/market/long-short-ratio)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* GO
* Java
* Node.js

```
GET /v5/market/account-ratio?category=linear&symbol=BTCUSDT&period=1h&limit=2&startTime=1696089600000&endTime=1696262400000 HTTP/1.1Host: api-testnet.bybit.com
```

```

```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "BTCUSDT", "period": "5min"}client.NewUtaBybitServiceWithParams(params).GetLongShortRatio(context.Background())
```

```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var marketAccountRatioRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").dataRecordingPeriod(DataRecordingPeriod.FIFTEEN_MINUTES).limit(10).build();client.getMarketAccountRatio(marketAccountRatioRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,});client  .getLongShortRatio({    category: 'linear',    symbol: 'BTCUSDT',    period: '1h',    limit: 100,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "symbol": "BTCUSDT",                "buyRatio": "0.49",                "sellRatio": "0.51",                "timestamp": "1696262400000"            },            {                "symbol": "BTCUSDT",                "buyRatio": "0.4927",                "sellRatio": "0.5073",                "timestamp": "1696258800000"            }        ],        "nextPageCursor": "lastid%3D0%26lasttime%3D1696258800"    },    "retExtInfo": {},    "time": 1731567491688}
```

## VIP MARGIN

Get VIP Margin Data
==========

This margin data is for **Unified account** in particular.

info

Does not need authentication.

### HTTP Request[​](#http-request) ###

GET `/v5/spot-margin-trade/data`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type |        Comments         |
|:---------------------------------|:-------|:-----|-------------------------|
|[vipLevel](/docs/v5/enum#viplevel)| false  |string|        Vip level        |
|             currency             | false  |string|Coin name, uppercase only|

### Response Parameters[​](#response-parameters) ###

|       Parameter       | Type  |                                                                                                    Comments                                                                                                    |
|:----------------------|:------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      vipCoinList      | array |                                                                                                     Object                                                                                                     |
|        \> list        | array |                                                                                                     Object                                                                                                     |
|    \>\> borrowable    |boolean|                                                                                      Whether it is allowed to be borrowed                                                                                      |
| \>\> collateralRatio  |string |Due to the new Tiered Collateral value logic, this field will no longer be accurate starting on February 19, 2025. Please refer to [Get Tiered Collateral Ratio](/docs/v5/spot-margin-uta/tier-collateral-ratio)|
|     \>\> currency     |string |                                                                                                   Coin name                                                                                                    |
| \>\> hourlyBorrowRate |string |                                                                                         Borrow interest rate per hour                                                                                          |
| \>\> liquidationOrder |string |                                                                                               Liquidation order                                                                                                |
| \>\> marginCollateral |boolean|                                                                             Whether it can be used as a margin collateral currency                                                                             |
|\>\> maxBorrowingAmount|string |                                                                                               Max borrow amount                                                                                                |
|      \> vipLevel      |string |                                                                                                   Vip level                                                                                                    |
[RUN \>\>](/docs/api-explorer/v5/spot-margin-uta/vip-margin)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/spot-margin-trade/data?vipLevel=No VIP&currency=BTC HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.spot_margin_trade_get_vip_margin_data())
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getVIPMarginData({    vipLevel: 'No VIP',    currency: 'BTC',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "vipCoinList": [            {                "list": [                    {                        "borrowable": true,                        "collateralRatio": "0.95",                        "currency": "BTC",                        "hourlyBorrowRate": "0.0000015021220000",                        "liquidationOrder": "11",                        "marginCollateral": true,                        "maxBorrowingAmount": "3"                    }                ],                "vipLevel": "No VIP"            }        ]    }}
```

## TIER COLLATERAL RATIO

Get Tiered Collateral Ratio
==========

UTA loan tiered collateral ratio

info

Does not need authentication.

### HTTP Request[​](#http-request) ###

GET `/v5/spot-margin-trade/collateral`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |        Comments         |
|:--------|:-------|:-----|-------------------------|
|currency | false  |string|Coin name, uppercase only|

### Response Parameters[​](#response-parameters) ###

|      Parameter       | Type |                               Comments                               |
|:---------------------|:-----|----------------------------------------------------------------------|
|         list         |array |                                Object                                |
|     \> currency      |string|                              Coin name                               |
|\> collateralRatioList|array |                                Object                                |
|     \>\> maxQty      |string|Upper limit(in coin) of the tiered range, `""` means positive infinity|
|     \>\> minQty      |string|               lower limit(in coin) of the tiered range               |
| \>\> collateralRatio |string|                           Collateral ratio                           |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/spot-margin-trade/collateral?currency=BTC HTTP/1.1Host: api-testnet.bybit.com
```

```

```

```

```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "currency": "BTC",                "collateralRatioList": [                    {                        "minQty": "0",                        "maxQty": "1000000",                        "collateralRatio": "0.85"                    },                    {                        "minQty": "1000000",                        "maxQty": "",                        "collateralRatio": "0"                    }                ]            }        ]    },    "retExtInfo": "{}",    "time": 1739848984945}
```

## COLLATERAL COIN

Get Collateral Coins
==========

info

Does not need authentication.

### HTTP Request[​](#http-request) ###

GET `/v5/crypto-loan/collateral-data`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                                          Comments                                                          |
|:--------|:-------|:-----|----------------------------------------------------------------------------------------------------------------------------|
|vipLevel | false  |string|Vip level `VIP0`, `VIP1`, `VIP2`, `VIP3`, `VIP4`, `VIP5`, `VIP99`(supreme VIP)`PRO1`, `PRO2`, `PRO3`, `PRO4`, `PRO5`, `PRO6`|
|currency | false  |string|                                                 Coin name, uppercase only                                                  |

### Response Parameters[​](#response-parameters) ###

|       Parameter       | Type  |                                                                         Comments                                                                          |
|:----------------------|:------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|
|      vipCoinList      | array |                                                                          Object                                                                           |
|        \> list        | array |                                                                          Object                                                                           |
|\>\> collateralAccuracy|integer|                                                              Valid collateral coin precision                                                              |
|    \>\> initialLTV    |string |        The Initial LTV ratio determines the initial amount of coins that can be borrowed. The initial LTV ratio may vary for different collateral         |
|  \>\> marginCallLTV   |string |             If the LTV ratio (Loan Amount/Collateral Amount) reaches the threshold, you will be required to add more collateral to your loan              |
|  \>\> liquidationLTV  |string |If the LTV ratio (Loan Amount/Collateral Amount) reaches the threshold, Bybit will liquidate your collateral assets to repay your loan and interest in full|
|     \>\> maxLimit     |string |                                                                     Collateral limit                                                                      |
|      \> vipLevel      |string |                                                                         Vip level                                                                         |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/crypto-loan/collateral-data?currency=ETH&vipLevel=PRO1 HTTP/1.1Host: api.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,)print(session.get_collateral_coins(    currency="ETH",    vipLevel="PRO1",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getCollateralCoins({    currency: 'ETH',    vipLevel: 'PRO1',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "vipCoinList": [            {                "list": [                    {                        "collateralAccuracy": 8,                        "currency": "ETH",                        "initialLTV": "0.8",                        "liquidationLTV": "0.95",                        "marginCallLTV": "0.87",                        "maxLimit": "32000"                    }                ],                "vipLevel": "PRO1"            }        ]    },    "retExtInfo": {},    "time": 1728618590498}
```

## LOAN COIN

Get Borrowable Coins
==========

info

Does not need authentication.

danger

Borrowed coins can be returned at any time before the due date. You'll be charged 3 times the hourly interest during the overdue period. Your collateral will be liquidated to repay a loan and the interest if you fail to make the repayment 48 hours after the due time.

### HTTP Request[​](#http-request) ###

GET `/v5/crypto-loan/loanable-data`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                                          Comments                                                          |
|:--------|:-------|:-----|----------------------------------------------------------------------------------------------------------------------------|
|vipLevel | false  |string|Vip level `VIP0`, `VIP1`, `VIP2`, `VIP3`, `VIP4`, `VIP5`, `VIP99`(supreme VIP)`PRO1`, `PRO2`, `PRO3`, `PRO4`, `PRO5`, `PRO6`|
|currency | false  |string|                                                 Coin name, uppercase only                                                  |

### Response Parameters[​](#response-parameters) ###

|           Parameter           | Type  |                                                                                                                              Comments                                                                                                                              |
|:------------------------------|:------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|          vipCoinList          | array |                                                                                                                               Object                                                                                                                               |
|            \> list            | array |                                                                                                                               Object                                                                                                                               |
|    \>\> borrowingAccuracy     |integer|                                                                                                       The number of decimal places (precision) of this coin                                                                                                        |
|         \>\> currency         |string |                                                                                                                             Coin name                                                                                                                              |
|\>\> flexibleHourlyInterestRate|string |Flexible hourly floating interest rate<br/><br/>* Flexible Crypto Loans offer an hourly floating interest rate, calculated based on the actual borrowing time per hour, with the option for early repayment<br/>* Is `""` if the coin does not support flexible loan|
|   \>\> hourlyInterestRate7D   |string |                                                                                       Hourly interest rate for 7 days loan. Is `""` if the coin does not support 7 days loan                                                                                       |
|  \>\> hourlyInterestRate14D   |string |                                                                                      Hourly interest rate for 14 days loan. Is `""` if the coin does not support 14 days loan                                                                                      |
|  \>\> hourlyInterestRate30D   |string |                                                                                      Hourly interest rate for 30 days loan. Is `""` if the coin does not support 30 days loan                                                                                      |
|  \>\> hourlyInterestRate90D   |string |                                                                                      Hourly interest rate for 90 days loan. Is `""` if the coin does not support 90 days loan                                                                                      |
|  \>\> hourlyInterestRate180D  |string |                                                                                     Hourly interest rate for 180 days loan. Is `""` if the coin does not support 180 days loan                                                                                     |
|    \>\> maxBorrowingAmount    |string |                                                                                                                       Max. amount to borrow                                                                                                                        |
|    \>\> minBorrowingAmount    |string |                                                                                                                       Min. amount to borrow                                                                                                                        |
|          \> vipLevel          |string |                                                                                                                             Vip level                                                                                                                              |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/crypto-loan/loanable-data?currency=USDT&vipLevel=VIP0 HTTP/1.1Host: api.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,)print(session.get_borrowable_coins(    currency="USDT",    vipLevel="VIP0",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getBorrowableCoins({    currency: 'USDT',    vipLevel: 'VIP0',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "vipCoinList": [            {                "list": [                    {                        "borrowingAccuracy": 4,                        "currency": "USDT",                        "flexibleHourlyInterestRate": "0.0000090346",                        "hourlyInterestRate14D": "0.0000207796",                        "hourlyInterestRate180D": "",                        "hourlyInterestRate30D": "0.00002349",                        "hourlyInterestRate7D": "0.0000180692",                        "hourlyInterestRate90D": "",                        "maxBorrowingAmount": "8000000",                        "minBorrowingAmount": "20"                    }                ],                "vipLevel": "VIP0"            }        ]    },    "retExtInfo": {},    "time": 1728619315868}
```

## MARGIN PRODUCT INFO

Get Product Info
==========

tip

* When queried without an API key, this endpoint returns public product data
* If your UID is bound with an OTC loan, then you can get your private product data by calling with your API key
* If your UID is not bound with an OTC loan but you passed your API key, this endpoint returns public product data

### HTTP Request[​](#http-request) ###

GET `/v5/ins-loan/product-infos`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                   Comments                    |
|:--------|:-------|:-----|-----------------------------------------------|
|productId| false  |string|Product ID. If not passed, returns all products|

### Response Parameters[​](#response-parameters) ###

|        Parameter         | Type  |                                                                                                            Comments                                                                                                             |
|:-------------------------|:------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    marginProductInfo     | array |                                                                                                             Object                                                                                                              |
|       \> productId       |string |                                                                                                           Product ID                                                                                                            |
|       \> leverage        |string |                                                                                           The maximum leverage for this loan product                                                                                            |
|      \> supportSpot      |integer|                                                                                        Whether or not Spot is supported. 0:false; 1:true                                                                                        |
|    \> supportContract    |integer|                                                                                     Whether USDT Perpetuals are supported. 0:false; 1:true                                                                                      |
| \> supportMarginTrading  |integer|                                                                                Whether or not Spot margin trading is supported. 0:false; 1:true                                                                                 |
|\> deferredLiquidationLine|string |                                                                                                  Line for deferred liquidation                                                                                                  |
|\> deferredLiquidationTime|string |                                                                                                  Time for deferred liquidation                                                                                                  |
|     \> withdrawLine      |string |                                                                                                  Restrict line for withdrawal                                                                                                   |
|     \> transferLine      |string |                                                                                                   Restrict line for transfer                                                                                                    |
|      \> spotBuyLine      |string |                                                                                                   Restrict line for Spot buy                                                                                                    |
|     \> spotSellLine      |string |                                                                                                 Restrict line for Spot trading                                                                                                  |
|   \> contractOpenLine    |string |                                                                                         Restrict line for USDT Perpetual open position                                                                                          |
|    \> liquidationLine    |string |                                                                                                      Line for liquidation                                                                                                       |
|  \> stopLiquidationLine  |string |                                                                                                    Line for stop liquidation                                                                                                    |
|   \> contractLeverage    |string |                                                                                         The allowed default leverage for USDT Perpetual                                                                                         |
|     \> transferRatio     |string |                                                                        The transfer ratio for loan funds to transfer from Spot wallet to Contract wallet                                                                        |
|      \> spotSymbols      | array |            The whitelist of spot trading pairs<br/><br/>* If `supportSpot`="0", then it returns "[]"<br/>* If empty array, then you can trade any symbols<br/>* If not empty, then you can only trade listed symbols            |
|    \> contractSymbols    | array |        The whitelist of contract trading pairs<br/><br/>* If `supportContract`="0", then it returns "[]"<br/>* If empty array, then you can trade any symbols<br/>* If not empty, then you can only trade listed symbols        |
|  \> supportUSDCContract  |integer|                                                                              Whether or not USDC contracts are supported. `'0'`:false; `'1'`:true                                                                               |
|  \> supportUSDCOptions   |integer|                                                                                  Whether or not Options are supported. `'0'`:false; `'1'`:true                                                                                  |
| \> USDTPerpetualOpenLine |string |                                                                                          Restrict line to open USDT Perpetual position                                                                                          |
| \> USDCContractOpenLine  |string |                                                                                          Restrict line to open USDC Contract position                                                                                           |
|  \> USDCOptionsOpenLine  |string |                                                                                              Restrict line to open Option position                                                                                              |
|\> USDTPerpetualCloseLine |string |                                                                                              Restrict line to trade USDT Perpetual                                                                                              |
| \> USDCContractCloseLine |string |                                                                                              Restrict line to trade USDC Contract                                                                                               |
| \> USDCOptionsCloseLine  |string |                                                                                                  Restrict line to trade Option                                                                                                  |
|  \> USDCContractSymbols  | array |      The whitelist of USDC contract trading pairs<br/><br/>* If `supportContract`="0", then it returns "[]"<br/>* If no whitelist symbols, it is `[]`, and you can trade any<br/>* If supportUSDCContract="0", it is `[]`       |
|  \> USDCOptionsSymbols   | array |                The whitelist of Option symbols<br/><br/>* If `supportContract`="0", then it returns "[]"<br/>* If no whitelisted, it is `[]`, and you can trade any<br/>* If supportUSDCOptions="0", it is `[]`                 |
|    \> marginLeverage     |string |                                                             The allowable maximum leverage for Spot margin trading. If `supportMarginTrading`=0, then it returns ""                                                             |
| \> USDTPerpetualLeverage | array |    Object<br/><br/>* If supportContract="0", it is `[]`<br/>* If no whitelist USDT perp symbols, it returns all trading symbols and leverage by default<br/>* If there are whitelist symbols, it return those whitelist data    |
|       \>\> symbol        |string |                                                                                                           Symbol name                                                                                                           |
|      \>\> leverage       |string |                                                                                                        Maximum leverage                                                                                                         |
| \> USDCContractLeverage  | array |Object<br/><br/>* If supportUSDCContract="0", it is `[]`<br/>* If no whitelist USDC contract symbols, it returns all trading symbols and leverage by default<br/>* If there are whitelist symbols, it return those whitelist data|
|       \>\> symbol        |string |                                                                                                           Symbol name                                                                                                           |
|      \>\> leverage       |string |                                                                                                        Maximum leverage                                                                                                         |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/ins-loan/product-infos?productId=91 HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_product_info(productId="91"))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getInstitutionalLendingProductInfo({    productId: '91',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "marginProductInfo": [            {                "productId": "91",                "leverage": "4.00000000",                "supportSpot": 1,                "supportContract": 0,                "withdrawLine": "",                "transferLine": "",                "spotBuyLine": "",                "spotSellLine": "",                "contractOpenLine": "",                "liquidationLine": "0.75",                "stopLiquidationLine": "0.35000000",                "contractLeverage": "0",                "transferRatio": "0",                "spotSymbols": [],                "contractSymbols": [],                "supportUSDCContract": 0,                "supportUSDCOptions": 0,                "USDTPerpetualOpenLine": "",                "USDCContractOpenLine": "",                "USDCOptionsOpenLine": "",                "USDTPerpetualCloseLine": "",                "USDCContractCloseLine": "",                "USDCOptionsCloseLine": "",                "USDCContractSymbols": [],                "USDCOptionsSymbols": [],                "marginLeverage": "0",                "USDTPerpetualLeverage": [],                "USDCContractLeverage": [],                "deferredLiquidationLine":"",                "deferredLiquidationTime":"",            }        ]    },    "retExtInfo": {},    "time": 1689747746332}
```

## MARGIN COIN CONVERT INFO

Get Margin Coin Info
==========

tip

* When queried without an API key, this endpoint returns public margin data
* If your UID is bound with an OTC loan, then you can get your private margin data by calling with your API key
* If your UID is not bound with an OTC loan but you passed your API key, this endpoint returns public margin data

### HTTP Request[​](#http-request) ###

GET `/v5/ins-loan/ensure-tokens-convert`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                                        Comments                                                        |
|:--------|:-------|:-----|------------------------------------------------------------------------------------------------------------------------|
|productId| false  |string|Product ID. If not passed, returns all margin products. For spot, it returns coins with a `convertRatio` greater than 0.|

### Response Parameters[​](#response-parameters) ###

|      Parameter      | Type |           Comments           |
|:--------------------|:-----|------------------------------|
|     marginToken     |array |            Object            |
|    \> productId     |string|          Product Id          |
|    \> tokenInfo     |array |       Spot margin coin       |
|     \>\> token      |string|         Margin coin          |
|\>\> convertRatioList|array |Margin coin convert ratio List|
|    \>\>\> ladder    |string|            ladder            |
| \>\>\> convertRatio |string|  Margin coin convert ratio   |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/ins-loan/ensure-tokens-convert HTTP/1.1Host: api-testnet.bybit.com
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_margin_coin_info())
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getInstitutionalLendingMarginCoinInfoWithConversionRate({    productId: '81',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "marginToken": [            {                "productId": "81",                "tokenInfo": [                    {                        "token": "USDT",                        "convertRatioList": [                            {                                "ladder": "0-500",                                "convertRatio": "0.95"                            },                            {                                "ladder": "500-1000",                                "convertRatio": "0.9"                            },                            {                                "ladder": "1000-2000",                                "convertRatio": "0.8"                            },                            {                                "ladder": "2000-4000",                                "convertRatio": "0.7"                            },                            {                                "ladder": "4000-99999999999",                                "convertRatio": "0.6"                            }                        ]                    }                  ...                ]            },            {                "productId": "82",                "tokenInfo": [                    ...                    {                        "token": "USDT",                        "convertRatioList": [                            {                                "ladder": "0-1000",                                "convertRatio": "0.7"                            },                            {                                "ladder": "1000-2000",                                "convertRatio": "0.65"                            },                            {                                "ladder": "2000-99999999999",                                "convertRatio": "0.6"                            }                        ]                    }                ]            },            {                "productId": "84",                "tokenInfo": [                    ...                    {                        "token": "BTC",                        "convertRatioList": [                            {                                "ladder": "0-1000",                                "convertRatio": "1"                            },                            {                                "ladder": "1000-5000",                                "convertRatio": "0.9"                            },                            {                                "ladder": "5000-99999999999",                                "convertRatio": "0.55"                            }                        ]                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1683276016497}
```

